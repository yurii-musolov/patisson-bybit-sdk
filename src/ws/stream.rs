use futures_util::{SinkExt, StreamExt};
use std::time::Duration;
use tokio::{
    sync::mpsc,
    time::{Instant, sleep, sleep_until, timeout},
};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tracing::{debug, error, info, warn};

use crate::{
    serde::{deserialize_json, serialize_json},
    ws::IncomingMessage,
};

use super::{
    Command, Config, DisconnectReason, Event, Handle,
    state::{FrameResult, HeartbeatState, Sink, State},
};

pub struct Stream {
    config: Config,
    cmd_rx: mpsc::Receiver<Command>,
    evt_tx: mpsc::Sender<Event>,
}

impl Stream {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(config: Config) -> (Handle, mpsc::Receiver<Event>) {
        let (cmd_tx, cmd_rx) = mpsc::channel::<Command>(config.command_queue_size);
        let (evt_tx, evt_rx) = mpsc::channel::<Event>(config.event_queue_size);

        let stream = Self {
            config,
            cmd_rx,
            evt_tx,
        };

        tokio::spawn(stream.run());

        (Handle::new(cmd_tx), evt_rx)
    }

    async fn run(mut self) {
        info!("stream started");
        let mut state = State::Idle;

        loop {
            state = match state {
                State::Idle => self.step_idle().await,
                State::Connecting { attempt } => self.step_connecting(attempt).await,
                State::Connected {
                    frame_rx,
                    read_task,
                    sink,
                } => self.step_connected(frame_rx, read_task, sink).await,
                State::Reconnecting { attempt, delay_ms } => {
                    self.step_reconnecting(attempt, delay_ms).await
                }
                State::Closing {
                    frame_rx,
                    read_task,
                    sink,
                } => self.step_closing(frame_rx, read_task, sink).await,
                State::Done => break,
            };
        }

        info!("stream shut down");
    }

    fn emit(&self, event: Event) {
        if let Err(e) = self.evt_tx.try_send(event) {
            match e {
                mpsc::error::TrySendError::Full(dropped) => {
                    warn!("event queue full, dropping event: {:?}", dropped);
                }
                mpsc::error::TrySendError::Closed(_) => {
                    debug!("event receiver dropped");
                }
            }
        }
    }

    async fn step_idle(&mut self) -> State {
        loop {
            match self.cmd_rx.recv().await {
                Some(Command::Connect) => {
                    return State::Connecting { attempt: 0 };
                }
                None => return State::Done,
                Some(Command::Disconnect) => {
                    warn!("Command disconnect ignored - not connected");
                }
                Some(Command::Send(_)) => {
                    warn!("Send ignored - not connected");
                }
            }
        }
    }

    async fn step_connecting(&mut self, attempt: u32) -> State {
        debug!(attempt, "connecting");

        let connect = timeout(self.config.connect_timeout, connect_async(&self.config.url));
        tokio::pin!(connect);

        // Keep serving commands while the handshake is in flight so that a
        // `Disconnect` is not blocked behind a slow or hanging connection.
        let result = loop {
            tokio::select! {
                result = &mut connect => break result,
                cmd = self.cmd_rx.recv() => match cmd {
                    None | Some(Command::Disconnect) => {
                        info!("disconnect requested while connecting");
                        self.emit(Event::Disconnected {
                            reason: DisconnectReason::Requested,
                        });
                        return State::Idle;
                    }
                    Some(Command::Connect) => debug!("Connect ignored - already connecting"),
                    Some(Command::Send(_)) => warn!("Send ignored - not connected yet"),
                },
            }
        };

        match result {
            Ok(Ok((ws_stream, _))) => {
                info!("websocket connected");
                self.emit(Event::Connected);

                let (sink, stream) = ws_stream.split();
                let (frame_tx, frame_rx) =
                    mpsc::channel::<FrameResult>(self.config.event_queue_size);

                let read_task = tokio::spawn(async move {
                    let mut stream = stream;
                    while let Some(msg) = stream.next().await {
                        if frame_tx.send(msg).await.is_err() {
                            break;
                        }
                    }
                });

                State::Connected {
                    frame_rx,
                    read_task,
                    sink: Box::new(sink),
                }
            }
            Ok(Err(e)) => {
                error!(error = %e, attempt, "connection failed");
                self.next_reconnect_state(attempt + 1, DisconnectReason::Error(e.to_string()))
            }
            Err(_) => {
                error!(attempt, "connection timed out");
                self.next_reconnect_state(
                    attempt + 1,
                    DisconnectReason::Error(String::from("connect timeout")),
                )
            }
        }
    }

    async fn step_connected(
        &mut self,
        mut frame_rx: mpsc::Receiver<FrameResult>,
        read_task: tokio::task::JoinHandle<()>,
        mut sink: Sink,
    ) -> State {
        let ping_interval = self.config.ping_interval;
        let pong_timeout = self.config.pong_timeout;
        let mut ping_timer = Box::pin(match ping_interval {
            Some(d) => sleep(d),
            None => sleep(FAR_FUTURE),
        });
        let mut pong_timer = Box::pin(sleep(FAR_FUTURE));
        let mut hb = HeartbeatState::Idle;

        loop {
            tokio::select! {
                biased;

                frame = frame_rx.recv() => match frame {
                    None => {
                        info!("remote closed the connection");
                        read_task.abort();
                        return self.next_reconnect_state(1, DisconnectReason::RemoteClosed);
                    }
                    Some(Ok(msg)) => {
                        match msg {
                            Message::Text(json) => {
                                match deserialize_json::<IncomingMessage>(&json){
                                    Ok(msg) => {
                                        // INFO: Bybit heartbeat process.
                                        // - send to server: { "op": "ping" }
                                        // - for public channels receive from server: { "op": "ping", "ret_msg": "pong", "success": true }
                                        // - for private channels receive from server: { "op": "pong" }
                                        if matches!(hb, HeartbeatState::PingSent) && (msg.is_ping() || msg.is_pong()) {
                                            debug!("receiving heartbeat pong");
                                            hb = HeartbeatState::Idle;
                                            pong_timer.as_mut().reset(far_future_instant());
                                            if let Some(d) = ping_interval {
                                                ping_timer.as_mut().reset(Instant::now() + d);
                                            }
                                        }

                                        self.emit(Event::Message(msg));
                                    }
                                    Err(e) => {
                                        warn!(error = %e, "parsing IncomingMessage failed");
                                        self.emit(Event::ParseError(e.to_string()));
                                    }
                                }
                            }
                            Message::Binary(bytes) => debug!("binary message received ({}B)", bytes.len()),
                            Message::Ping(bytes) => debug!("ping received ({}B)", bytes.len()),
                            Message::Pong(bytes) => debug!("pong received ({}B)", bytes.len()),
                            Message::Close(close_frame) => debug!("close frame received [{:?}]", close_frame),
                            Message::Frame(frame) =>debug!("frame received ({}B)", frame.len()),
                        }
                    }
                    Some(Err(e)) => {
                        error!(error = %e, "websocket read error");
                        read_task.abort();
                        return self.next_reconnect_state(1, DisconnectReason::Error(e.to_string()));
                    }
                },

                cmd = self.cmd_rx.recv() => match cmd {
                    None | Some(Command::Disconnect) => {
                        info!("disconnect requested");
                        return State::Closing { frame_rx, read_task, sink };
                    }
                    Some(Command::Send(msg)) => {
                        let json = match serialize_json(&msg) {
                            Ok(json) => json,
                            Err(e) => {
                                warn!(error = %e, "serializing outgoing message failed, message dropped");
                                continue;
                            }
                        };
                        if let Err(e) = sink.send(Message::Text(json.into())).await {
                            error!(error = %e, "send error");
                            read_task.abort();
                            return self.next_reconnect_state(1, DisconnectReason::Error(e.to_string()));
                        }
                    }
                    Some(Command::Connect) => warn!("Connect ignored - already connected")
                },

                () = &mut ping_timer, if ping_interval.is_some() => {
                    debug!("sending heartbeat ping");
                    if let Err(e) = sink.send(ping()).await {
                        error!(error = %e, "ping send error");
                        read_task.abort();
                        return self.next_reconnect_state(1, DisconnectReason::Error(e.to_string()));
                    }

                    hb = HeartbeatState::PingSent;
                    pong_timer.as_mut().reset(Instant::now() + pong_timeout);
                    ping_timer.as_mut().reset(far_future_instant());
                },

                () = &mut pong_timer, if matches!(hb, HeartbeatState::PingSent) => {
                    warn!("pong timeout - connection appears dead");
                    read_task.abort();
                    return self.next_reconnect_state(1, DisconnectReason::PongTimeout);
                },
            }
        }
    }

    async fn step_reconnecting(&mut self, attempt: u32, delay_ms: u64) -> State {
        warn!(attempt, delay_ms, "waiting before reconnect");
        self.emit(Event::Reconnecting { attempt, delay_ms });

        // Commands that arrive during the back-off must neither shorten the
        // delay nor vanish silently, so the deadline is fixed up front.
        let deadline = Instant::now() + Duration::from_millis(delay_ms);
        loop {
            tokio::select! {
                () = sleep_until(deadline) => return State::Connecting { attempt },
                cmd = self.cmd_rx.recv() => match cmd {
                    None | Some(Command::Disconnect) => {
                        self.emit(Event::Disconnected {
                            reason: DisconnectReason::Requested,
                        });
                        return State::Idle;
                    }
                    Some(Command::Connect) => debug!("Connect ignored - already reconnecting"),
                    Some(Command::Send(_)) => warn!("Send ignored - not connected"),
                },
            }
        }
    }

    async fn step_closing(
        &mut self,
        mut frame_rx: mpsc::Receiver<FrameResult>,
        read_task: tokio::task::JoinHandle<()>,
        mut sink: Sink,
    ) -> State {
        match sink.send(Message::Close(None)).await {
            Ok(()) => {
                // Wait for the server to answer with its own Close frame (or to
                // drop the connection). Commands are not read here, so nothing
                // the user sends meanwhile is lost: it is handled in `Idle`.
                let handshake = async {
                    while let Some(frame) = frame_rx.recv().await {
                        if matches!(frame, Ok(Message::Close(_)) | Err(_)) {
                            break;
                        }
                    }
                };
                if timeout(self.config.close_timeout, handshake).await.is_err() {
                    warn!("close handshake timed out");
                }
            }
            Err(e) => debug!(error = %e, "send close message failed"),
        }
        read_task.abort();

        self.emit(Event::Disconnected {
            reason: DisconnectReason::Requested,
        });
        State::Idle
    }

    fn next_reconnect_state(&self, next_attempt: u32, reason: DisconnectReason) -> State {
        match reconnect_delay_ms(&self.config, next_attempt) {
            Some(delay_ms) => {
                debug!(next_attempt, delay_ms, ?reason, "scheduling reconnect");
                State::Reconnecting {
                    attempt: next_attempt,
                    delay_ms,
                }
            }
            None => {
                warn!(?reason, "giving up reconnecting");
                self.emit(Event::Disconnected { reason });
                State::Idle
            }
        }
    }
}

/// Back-off delay before reconnect attempt number `attempt` (1-based), or
/// `None` when the attempt exceeds `max_reconnect_attempts`.
fn reconnect_delay_ms(config: &Config, attempt: u32) -> Option<u64> {
    if attempt == 0 || attempt > config.max_reconnect_attempts {
        return None;
    }
    let base_ms = config.reconnect_base_delay.as_millis() as u64;
    let max_ms = config.reconnect_max_delay.as_millis() as u64;
    Some(
        base_ms
            .saturating_mul(1u64 << (attempt - 1).min(10))
            .min(max_ms),
    )
}

const FAR_FUTURE: Duration = Duration::from_secs(u64::MAX / 4);

#[inline]
fn far_future_instant() -> Instant {
    Instant::now() + FAR_FUTURE
}

/// Bybit application-level heartbeat (`OutgoingMessage::Ping` without `req_id`).
#[inline]
fn ping() -> Message {
    Message::Text(r#"{"op":"ping"}"#.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ws::OutgoingMessage;
    use tokio::net::TcpListener;

    /// Local WebSocket server that accepts any number of connections and
    /// answers the close handshake. Returns its `ws://` URL.
    async fn spawn_server() -> String {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            while let Ok((tcp, _)) = listener.accept().await {
                tokio::spawn(async move {
                    let Ok(mut ws) = tokio_tungstenite::accept_async(tcp).await else {
                        return;
                    };
                    while let Some(Ok(_)) = ws.next().await {}
                });
            }
        });
        format!("ws://{addr}")
    }

    /// An address nothing listens on.
    async fn closed_port_url() -> String {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        drop(listener);
        format!("ws://{addr}")
    }

    async fn next_event(rx: &mut mpsc::Receiver<Event>) -> Event {
        timeout(Duration::from_secs(2), rx.recv())
            .await
            .expect("timed out waiting for event")
            .expect("event channel closed")
    }

    fn test_config(url: String) -> Config {
        Config::new(url)
            .ping_interval(None)
            .reconnect_base_delay(Duration::from_millis(10))
            .reconnect_max_delay(Duration::from_millis(20))
    }

    #[test]
    fn reconnect_delay_grows_exponentially_and_is_capped() {
        let config = Config::new("")
            .max_reconnect_attempts(5)
            .reconnect_base_delay(Duration::from_millis(100))
            .reconnect_max_delay(Duration::from_millis(500));

        let delays: Vec<_> = (0..=6).map(|n| reconnect_delay_ms(&config, n)).collect();

        assert_eq!(
            delays,
            vec![
                None,
                Some(100),
                Some(200),
                Some(400),
                Some(500),
                Some(500),
                None
            ]
        );
    }

    #[test]
    fn reconnect_is_disabled_with_zero_attempts() {
        let config = Config::new("").max_reconnect_attempts(0);

        assert_eq!(reconnect_delay_ms(&config, 1), None);
    }

    #[tokio::test]
    async fn disconnect_completes_without_waiting_for_close_timeout() {
        let url = spawn_server().await;
        let (handle, mut events) =
            Stream::new(test_config(url).close_timeout(Duration::from_secs(30)));

        handle.connect().await.unwrap();
        assert!(matches!(next_event(&mut events).await, Event::Connected));

        handle.disconnect().await.unwrap();
        assert!(matches!(
            next_event(&mut events).await,
            Event::Disconnected {
                reason: DisconnectReason::Requested
            }
        ));
    }

    #[tokio::test]
    async fn connect_right_after_disconnect_is_not_lost() {
        let url = spawn_server().await;
        let (handle, mut events) = Stream::new(test_config(url));

        handle.connect().await.unwrap();
        assert!(matches!(next_event(&mut events).await, Event::Connected));

        handle.disconnect().await.unwrap();
        handle.connect().await.unwrap();
        assert!(matches!(
            next_event(&mut events).await,
            Event::Disconnected { .. }
        ));
        assert!(matches!(next_event(&mut events).await, Event::Connected));
    }

    #[tokio::test]
    async fn initial_connect_failure_retries_max_attempts_then_gives_up() {
        let url = closed_port_url().await;
        let (handle, mut events) = Stream::new(test_config(url).max_reconnect_attempts(2));

        handle.connect().await.unwrap();

        assert!(matches!(
            next_event(&mut events).await,
            Event::Reconnecting { attempt: 1, .. }
        ));
        assert!(matches!(
            next_event(&mut events).await,
            Event::Reconnecting { attempt: 2, .. }
        ));
        assert!(matches!(
            next_event(&mut events).await,
            Event::Disconnected {
                reason: DisconnectReason::Error(_)
            }
        ));
    }

    #[tokio::test]
    async fn commands_during_backoff_do_not_skip_the_delay() {
        let url = closed_port_url().await;
        let config = test_config(url)
            .max_reconnect_attempts(1)
            .reconnect_base_delay(Duration::from_millis(300))
            .reconnect_max_delay(Duration::from_millis(300));
        let (handle, mut events) = Stream::new(config);

        handle.connect().await.unwrap();
        assert!(matches!(
            next_event(&mut events).await,
            Event::Reconnecting { attempt: 1, .. }
        ));
        let started = Instant::now();
        handle.connect().await.unwrap();
        handle
            .send_command(OutgoingMessage::Ping { req_id: None })
            .await
            .unwrap();

        assert!(matches!(
            next_event(&mut events).await,
            Event::Disconnected { .. }
        ));
        assert!(started.elapsed() >= Duration::from_millis(250));
    }
}
