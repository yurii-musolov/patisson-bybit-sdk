use futures_util::{SinkExt, StreamExt};
use std::time::Duration;
use tokio::{
    sync::mpsc,
    time::{Instant, sleep, sleep_until, timeout},
};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tracing::{debug, error, info, warn};

use std::collections::HashMap;

use crate::{
    Topic,
    serde::serialize_json,
    ws::{CommandMsg, IncomingMessage, OutgoingMessage, create_outgoing_message_auth_at},
};

use super::{
    Command, Config, DisconnectReason, Event, Handle,
    state::{FrameResult, HeartbeatState, Sink, State},
};

/// WebSocket connection driver.
///
/// [`Stream::new`] spawns the driver on the current Tokio runtime and returns a
/// [`Handle`] for commands and a receiver of [`Event`]s.
///
/// # Reconnects
/// After an unexpected disconnect the driver reconnects automatically (see
/// [`Config::max_reconnect_attempts`]) and emits [`Event::Connected`] again.
///
/// Topics subscribed with [`Handle::subscribe`] are restored on every
/// connection. With [`Config::credentials`] the driver first sends `auth`
/// (expiry from [`Config::server_clock`]) and subscribes only after
/// [`Event::Authenticated`]; a rejected `auth` is reported as
/// [`Event::AuthFailed`]. Messages sent with [`Handle::send_command`] are not
/// restored.
///
/// # Event delivery
/// Lifecycle events (`Connected`, `Reconnecting`, `Disconnected`) are always
/// delivered: the driver waits for free space in the event queue. Market and
/// account data (`Message`, `ParseError`) are dropped when the queue is full;
/// the number of dropped events is reported by [`Event::Lagged`] as soon as
/// there is room again.
pub struct Stream {
    config: Config,
    cmd_rx: mpsc::Receiver<Command>,
    evt_tx: mpsc::Sender<Event>,
    /// Data events dropped since the last [`Event::Lagged`].
    dropped: u64,
    /// Topics to (re)subscribe on every connection, in subscription order.
    topics: Vec<Topic>,
    /// Counter for the `req_id` of driver-sent `subscribe` messages.
    next_req_id: u64,
}

/// Bybit accepts at most 10 topics in one `subscribe` for some streams
/// (e.g. spot); use that limit everywhere.
const MAX_TOPICS_PER_REQUEST: usize = 10;

/// Per-connection authentication and subscription progress.
#[derive(Default)]
struct Session {
    /// `auth` was sent and its reply has not arrived yet.
    awaiting_auth: bool,
    /// Topics may be subscribed (no credentials, or `auth` succeeded).
    ready: bool,
    /// Topics of driver-sent `subscribe` messages, by `req_id`.
    pending: HashMap<String, Vec<Topic>>,
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
            dropped: 0,
            topics: Vec::new(),
            next_req_id: 0,
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

    /// Deliver a data event without blocking the driver; drop it (and count
    /// it) when the event queue is full.
    fn emit(&mut self, event: Event) {
        if self.dropped > 0 {
            match self.evt_tx.try_send(Event::Lagged {
                dropped: self.dropped,
            }) {
                Ok(()) => self.dropped = 0,
                Err(mpsc::error::TrySendError::Full(_)) => {
                    self.dropped += 1;
                    return;
                }
                Err(mpsc::error::TrySendError::Closed(_)) => return,
            }
        }

        match self.evt_tx.try_send(event) {
            Ok(()) => {}
            Err(mpsc::error::TrySendError::Full(dropped)) => {
                if self.dropped == 0 {
                    warn!("event queue full, dropping events");
                }
                debug!(event = ?dropped, "event dropped");
                self.dropped += 1;
            }
            Err(mpsc::error::TrySendError::Closed(_)) => debug!("event receiver dropped"),
        }
    }

    /// Deliver a lifecycle event, waiting for space in the event queue.
    /// A pending [`Event::Lagged`] is delivered first to keep the order.
    async fn emit_lifecycle(&mut self, event: Event) {
        if self.dropped > 0 {
            let lagged = Event::Lagged {
                dropped: self.dropped,
            };
            if self.evt_tx.send(lagged).await.is_err() {
                debug!("event receiver dropped");
                return;
            }
            self.dropped = 0;
        }
        if self.evt_tx.send(event).await.is_err() {
            debug!("event receiver dropped");
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
                Some(Command::Subscribe(topics)) => {
                    self.add_topics(topics);
                }
                Some(Command::Unsubscribe(topics)) => {
                    self.remove_topics(topics);
                }
            }
        }
    }

    async fn step_connecting(&mut self, attempt: u32) -> State {
        debug!(attempt, "connecting");

        let url = self.config.url.clone();
        let connect = timeout(self.config.connect_timeout, connect_async(url));
        tokio::pin!(connect);

        // Keep serving commands while the handshake is in flight so that a
        // `Disconnect` is not blocked behind a slow or hanging connection.
        let result = loop {
            tokio::select! {
                result = &mut connect => break result,
                cmd = self.cmd_rx.recv() => match cmd {
                    None | Some(Command::Disconnect) => {
                        info!("disconnect requested while connecting");
                        self.emit_lifecycle(Event::Disconnected {
                            reason: DisconnectReason::Requested,
                        })
                        .await;
                        return State::Idle;
                    }
                    Some(Command::Connect) => debug!("Connect ignored - already connecting"),
                    Some(Command::Send(_)) => warn!("Send ignored - not connected yet"),
                    Some(Command::Subscribe(topics)) => {
                        self.add_topics(topics);
                    }
                    Some(Command::Unsubscribe(topics)) => {
                        self.remove_topics(topics);
                    }
                },
            }
        };

        match result {
            Ok(Ok((ws_stream, _))) => {
                info!("websocket connected");
                self.emit_lifecycle(Event::Connected).await;

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
                    .await
            }
            Err(_) => {
                error!(attempt, "connection timed out");
                self.next_reconnect_state(
                    attempt + 1,
                    DisconnectReason::Error(String::from("connect timeout")),
                )
                .await
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

        let mut session = Session::default();
        let opened = match self.auth_message() {
            Some(auth) => {
                session.awaiting_auth = true;
                send_message(&mut sink, &auth).await
            }
            None => {
                session.ready = true;
                let topics = self.topics.clone();
                self.send_topics(&mut sink, &mut session, true, &topics)
                    .await
            }
        };
        if let Err(e) = opened {
            error!(error = %e, "send error");
            read_task.abort();
            return self
                .next_reconnect_state(1, DisconnectReason::Error(e.to_string()))
                .await;
        }

        loop {
            tokio::select! {
                biased;

                frame = frame_rx.recv() => match frame {
                    None => {
                        info!("remote closed the connection");
                        read_task.abort();
                        return self.next_reconnect_state(1, DisconnectReason::RemoteClosed).await;
                    }
                    Some(Ok(msg)) => {
                        match msg {
                            Message::Text(json) => {
                                match IncomingMessage::from_json(&json){
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

                                        if let Err(e) = self.on_command_reply(&msg, &mut session, &mut sink).await {
                                            error!(error = %e, "send error");
                                            read_task.abort();
                                            return self.next_reconnect_state(1, DisconnectReason::Error(e.to_string())).await;
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
                        return self.next_reconnect_state(1, DisconnectReason::Error(e.to_string())).await;
                    }
                },

                cmd = self.cmd_rx.recv() => match cmd {
                    None | Some(Command::Disconnect) => {
                        info!("disconnect requested");
                        return State::Closing { frame_rx, read_task, sink };
                    }
                    Some(cmd @ (Command::Send(_) | Command::Subscribe(_) | Command::Unsubscribe(_))) => {
                        let sent = match cmd {
                            Command::Send(msg) => send_message(&mut sink, &msg).await,
                            Command::Subscribe(topics) => {
                                let added = self.add_topics(topics);
                                if session.ready {
                                    self.send_topics(&mut sink, &mut session, true, &added).await
                                } else {
                                    Ok(())
                                }
                            }
                            Command::Unsubscribe(topics) => {
                                let removed = self.remove_topics(topics);
                                if session.ready {
                                    self.send_topics(&mut sink, &mut session, false, &removed).await
                                } else {
                                    Ok(())
                                }
                            }
                            Command::Connect | Command::Disconnect => unreachable!(),
                        };
                        if let Err(e) = sent {
                            error!(error = %e, "send error");
                            read_task.abort();
                            return self.next_reconnect_state(1, DisconnectReason::Error(e.to_string())).await;
                        }
                    }
                    Some(Command::Connect) => warn!("Connect ignored - already connected")
                },

                () = &mut ping_timer, if ping_interval.is_some() => {
                    debug!("sending heartbeat ping");
                    if let Err(e) = sink.send(ping()).await {
                        error!(error = %e, "ping send error");
                        read_task.abort();
                        return self.next_reconnect_state(1, DisconnectReason::Error(e.to_string())).await;
                    }

                    hb = HeartbeatState::PingSent;
                    pong_timer.as_mut().reset(Instant::now() + pong_timeout);
                    ping_timer.as_mut().reset(far_future_instant());
                },

                () = &mut pong_timer, if matches!(hb, HeartbeatState::PingSent) => {
                    warn!("pong timeout - connection appears dead");
                    read_task.abort();
                    return self.next_reconnect_state(1, DisconnectReason::PongTimeout).await;
                },
            }
        }
    }

    async fn step_reconnecting(&mut self, attempt: u32, delay_ms: u64) -> State {
        warn!(attempt, delay_ms, "waiting before reconnect");
        self.emit_lifecycle(Event::Reconnecting { attempt, delay_ms })
            .await;

        // Commands that arrive during the back-off must neither shorten the
        // delay nor vanish silently, so the deadline is fixed up front.
        let deadline = Instant::now() + Duration::from_millis(delay_ms);
        loop {
            tokio::select! {
                () = sleep_until(deadline) => return State::Connecting { attempt },
                cmd = self.cmd_rx.recv() => match cmd {
                    None | Some(Command::Disconnect) => {
                        self.emit_lifecycle(Event::Disconnected {
                            reason: DisconnectReason::Requested,
                        })
                        .await;
                        return State::Idle;
                    }
                    Some(Command::Connect) => debug!("Connect ignored - already reconnecting"),
                    Some(Command::Send(_)) => warn!("Send ignored - not connected"),
                    Some(Command::Subscribe(topics)) => {
                        self.add_topics(topics);
                    }
                    Some(Command::Unsubscribe(topics)) => {
                        self.remove_topics(topics);
                    }
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

        self.emit_lifecycle(Event::Disconnected {
            reason: DisconnectReason::Requested,
        })
        .await;
        State::Idle
    }

    /// Remember `topics`; returns those that were not subscribed yet.
    fn add_topics(&mut self, topics: Vec<Topic>) -> Vec<Topic> {
        let mut added = Vec::new();
        for topic in topics {
            if !self.topics.contains(&topic) && !added.contains(&topic) {
                added.push(topic);
            }
        }
        self.topics.extend(added.iter().cloned());
        added
    }

    /// Forget `topics`; returns those that were subscribed.
    fn remove_topics(&mut self, topics: Vec<Topic>) -> Vec<Topic> {
        let removed: Vec<Topic> = topics
            .into_iter()
            .filter(|topic| self.topics.contains(topic))
            .collect();
        self.topics.retain(|topic| !removed.contains(topic));
        removed
    }

    /// The `auth` message, if credentials are configured.
    fn auth_message(&self) -> Option<OutgoingMessage> {
        let api_key = self.config.api_key.clone()?;
        let api_secret = self.config.api_secret.clone()?;
        Some(create_outgoing_message_auth_at(
            api_key,
            api_secret,
            None,
            self.config.auth_recv_window,
            self.config.server_clock.now(),
        ))
    }

    /// Send `subscribe` (or `unsubscribe`) for `topics` in batches of
    /// [`MAX_TOPICS_PER_REQUEST`]; `subscribe` batches get a `req_id` so that
    /// a rejection can be reported with its topics.
    async fn send_topics(
        &mut self,
        sink: &mut Sink,
        session: &mut Session,
        subscribe: bool,
        topics: &[Topic],
    ) -> Result<(), tokio_tungstenite::tungstenite::Error> {
        for chunk in topics.chunks(MAX_TOPICS_PER_REQUEST) {
            let args = chunk.to_vec();
            let msg = if subscribe {
                self.next_req_id += 1;
                let req_id = format!("sub-{}", self.next_req_id);
                session.pending.insert(req_id.clone(), args.clone());
                OutgoingMessage::Subscribe {
                    req_id: Some(req_id),
                    args,
                }
            } else {
                OutgoingMessage::Unsubscribe { req_id: None, args }
            };
            send_message(sink, &msg).await?;
        }
        Ok(())
    }

    /// React to replies to driver-sent `auth` and `subscribe` messages.
    async fn on_command_reply(
        &mut self,
        msg: &IncomingMessage,
        session: &mut Session,
        sink: &mut Sink,
    ) -> Result<(), tokio_tungstenite::tungstenite::Error> {
        match msg {
            IncomingMessage::Command(CommandMsg::Auth {
                success, ret_msg, ..
            }) if session.awaiting_auth => {
                session.awaiting_auth = false;
                if *success {
                    info!("stream authenticated");
                    session.ready = true;
                    self.emit_lifecycle(Event::Authenticated).await;
                    let topics = self.topics.clone();
                    self.send_topics(sink, session, true, &topics).await?;
                } else {
                    warn!(?ret_msg, "stream authentication failed");
                    self.emit_lifecycle(Event::AuthFailed {
                        ret_msg: ret_msg.clone(),
                    })
                    .await;
                }
            }
            IncomingMessage::Command(CommandMsg::Subscribe {
                req_id: Some(req_id),
                success,
                ret_msg,
                ..
            }) => {
                if let Some(topics) = session.pending.remove(req_id)
                    && !*success
                {
                    warn!(?topics, ?ret_msg, "subscribe rejected");
                    self.emit_lifecycle(Event::SubscribeFailed {
                        topics,
                        ret_msg: ret_msg.clone(),
                    })
                    .await;
                }
            }
            _ => {}
        }
        Ok(())
    }

    async fn next_reconnect_state(&mut self, next_attempt: u32, reason: DisconnectReason) -> State {
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
                self.emit_lifecycle(Event::Disconnected { reason }).await;
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

/// Serialize and send `msg`; a serialization failure is logged and the message
/// dropped (it cannot be fixed by reconnecting).
async fn send_message(
    sink: &mut Sink,
    msg: &OutgoingMessage,
) -> Result<(), tokio_tungstenite::tungstenite::Error> {
    match serialize_json(msg) {
        Ok(json) => sink.send(Message::Text(json.into())).await,
        Err(e) => {
            warn!(error = %e, "serializing outgoing message failed, message dropped");
            Ok(())
        }
    }
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
        spawn_server_with_greeting(Vec::new()).await
    }

    /// Like [`spawn_server`], but sends `greeting` text frames right after
    /// every handshake.
    async fn spawn_server_with_greeting(greeting: Vec<String>) -> String {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            while let Ok((tcp, _)) = listener.accept().await {
                let greeting = greeting.clone();
                tokio::spawn(async move {
                    let Ok(mut ws) = tokio_tungstenite::accept_async(tcp).await else {
                        return;
                    };
                    for text in greeting {
                        if ws.send(Message::Text(text.into())).await.is_err() {
                            return;
                        }
                    }
                    while let Some(Ok(_)) = ws.next().await {}
                });
            }
        });
        format!("ws://{addr}")
    }

    /// An address nothing listens on. Connecting to it fails at once on
    /// Linux and macOS, but takes about 2 s on Windows (SYN retries).
    async fn closed_port_url() -> String {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        drop(listener);
        format!("ws://{addr}")
    }

    /// Generous upper bound for a single event; tests do not wait this long
    /// unless something is wrong (see [`closed_port_url`] for Windows).
    const EVENT_TIMEOUT: Duration = Duration::from_secs(10);

    async fn next_event(rx: &mut mpsc::Receiver<Event>) -> Event {
        timeout(EVENT_TIMEOUT, rx.recv())
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

    #[tokio::test]
    async fn dropped_data_events_are_reported_before_the_next_lifecycle_event() {
        let pong = r#"{"op":"pong","conn_id":"test"}"#.to_string();
        let url = spawn_server_with_greeting(vec![pong; 10]).await;
        let (handle, mut events) = Stream::new(test_config(url).event_queue_size(2));

        handle.connect().await.unwrap();
        // Let the driver fill the queue (Connected + 1 message) and drop the rest.
        sleep(Duration::from_millis(300)).await;
        assert!(matches!(next_event(&mut events).await, Event::Connected));
        assert!(matches!(next_event(&mut events).await, Event::Message(_)));

        handle.disconnect().await.unwrap();

        assert!(matches!(
            next_event(&mut events).await,
            Event::Lagged { dropped: 9 }
        ));
        assert!(matches!(
            next_event(&mut events).await,
            Event::Disconnected { .. }
        ));
    }

    /// Behaviour of [`spawn_bybit_server`].
    #[derive(Clone, Copy)]
    struct ServerScript {
        /// Reply `success` to `auth`.
        auth_ok: bool,
        /// On the first connection, close right after receiving this `op`.
        close_first_after: Option<&'static str>,
    }

    /// Text frames received, as (connection number, parsed JSON).
    type Received = std::sync::Arc<std::sync::Mutex<Vec<(usize, serde_json::Value)>>>;

    /// Local server that answers `auth` and `subscribe` like Bybit (topics
    /// containing "BAD" are rejected) and records what it receives.
    async fn spawn_bybit_server(script: ServerScript) -> (String, Received) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let received: Received = Default::default();
        let recorded = received.clone();
        tokio::spawn(async move {
            let mut conn = 0;
            while let Ok((tcp, _)) = listener.accept().await {
                conn += 1;
                let recorded = recorded.clone();
                tokio::spawn(async move {
                    let Ok(mut ws) = tokio_tungstenite::accept_async(tcp).await else {
                        return;
                    };
                    while let Some(Ok(frame)) = ws.next().await {
                        let Message::Text(text) = frame else { continue };
                        let json: serde_json::Value = serde_json::from_str(&text).unwrap();
                        recorded.lock().unwrap().push((conn, json.clone()));
                        let op = json["op"].as_str().unwrap_or_default().to_owned();
                        let reply = match op.as_str() {
                            "auth" => Some(serde_json::json!({
                                "op": "auth", "conn_id": "c", "success": script.auth_ok,
                                "ret_msg": if script.auth_ok { "" } else { "Invalid apikey" },
                            })),
                            "subscribe" => {
                                let ok = !json["args"].to_string().contains("BAD");
                                Some(serde_json::json!({
                                    "op": "subscribe", "conn_id": "c", "success": ok,
                                    "ret_msg": if ok { "" } else { "error:handler not found" },
                                    "req_id": json["req_id"],
                                }))
                            }
                            _ => None,
                        };
                        if let Some(reply) = reply
                            && ws
                                .send(Message::Text(reply.to_string().into()))
                                .await
                                .is_err()
                        {
                            return;
                        }
                        if conn == 1 && script.close_first_after == Some(op.as_str()) {
                            return;
                        }
                    }
                });
            }
        });
        (format!("ws://{addr}"), received)
    }

    /// Wait until `received` holds a frame matching `pred`.
    async fn wait_for_frame(received: &Received, pred: impl Fn(usize, &serde_json::Value) -> bool) {
        timeout(EVENT_TIMEOUT, async {
            loop {
                if received.lock().unwrap().iter().any(|(c, j)| pred(*c, j)) {
                    return;
                }
                sleep(Duration::from_millis(10)).await;
            }
        })
        .await
        .expect("timed out waiting for a frame");
    }

    /// Skip events until one matches `pred`.
    async fn wait_for_event(
        rx: &mut mpsc::Receiver<Event>,
        pred: impl Fn(&Event) -> bool,
    ) -> Event {
        loop {
            let event = next_event(rx).await;
            if pred(&event) {
                return event;
            }
        }
    }

    fn ops(received: &Received, conn: usize) -> Vec<String> {
        received
            .lock()
            .unwrap()
            .iter()
            .filter(|(c, _)| *c == conn)
            .map(|(_, j)| j["op"].as_str().unwrap_or_default().to_owned())
            .collect()
    }

    fn subscribed_args(received: &Received, conn: usize) -> Vec<String> {
        received
            .lock()
            .unwrap()
            .iter()
            .filter(|(c, j)| *c == conn && j["op"] == "subscribe")
            .flat_map(|(_, j)| j["args"].as_array().unwrap().clone())
            .map(|a| a.as_str().unwrap().to_owned())
            .collect()
    }

    fn ticker(symbol: &str) -> Topic {
        Topic::Ticker(symbol.to_owned())
    }

    #[tokio::test]
    async fn authenticates_then_restores_subscriptions_after_reconnect() {
        let (url, received) = spawn_bybit_server(ServerScript {
            auth_ok: true,
            close_first_after: Some("subscribe"),
        })
        .await;
        let (handle, mut events) = Stream::new(test_config(url).credentials("key", "secret"));

        handle
            .subscribe(vec![ticker("BTCUSDT"), ticker("ETHUSDT")])
            .await
            .unwrap();
        handle.connect().await.unwrap();

        wait_for_event(&mut events, |e| matches!(e, Event::Authenticated)).await;
        wait_for_frame(&received, |c, j| c == 2 && j["op"] == "subscribe").await;

        for conn in [1, 2] {
            assert_eq!(
                ops(&received, conn),
                ["auth", "subscribe"],
                "connection {conn}"
            );
            assert_eq!(
                subscribed_args(&received, conn),
                ["tickers.BTCUSDT", "tickers.ETHUSDT"],
                "connection {conn}"
            );
        }
        let auth = &received.lock().unwrap()[0].1;
        assert_eq!(auth["args"][0], "key");
    }

    #[tokio::test]
    async fn unsubscribed_topics_are_not_restored() {
        let (url, received) = spawn_bybit_server(ServerScript {
            auth_ok: true,
            close_first_after: Some("unsubscribe"),
        })
        .await;
        let (handle, _events) = Stream::new(test_config(url));
        handle.connect().await.unwrap();

        handle
            .subscribe(vec![ticker("BTCUSDT"), ticker("ETHUSDT")])
            .await
            .unwrap();
        wait_for_frame(&received, |c, j| c == 1 && j["op"] == "subscribe").await;
        handle.unsubscribe(vec![ticker("BTCUSDT")]).await.unwrap();
        wait_for_frame(&received, |c, j| c == 2 && j["op"] == "subscribe").await;

        assert_eq!(ops(&received, 1), ["subscribe", "unsubscribe"]);
        assert_eq!(subscribed_args(&received, 2), ["tickers.ETHUSDT"]);
    }

    #[tokio::test]
    async fn rejected_auth_is_reported_and_nothing_is_subscribed() {
        let (url, received) = spawn_bybit_server(ServerScript {
            auth_ok: false,
            close_first_after: None,
        })
        .await;
        let (handle, mut events) = Stream::new(test_config(url).credentials("key", "bad"));
        handle.subscribe(vec![ticker("BTCUSDT")]).await.unwrap();
        handle.connect().await.unwrap();

        let event = wait_for_event(&mut events, |e| matches!(e, Event::AuthFailed { .. })).await;
        sleep(Duration::from_millis(200)).await;

        assert!(matches!(
            event,
            Event::AuthFailed { ret_msg: Some(ref m) } if m == "Invalid apikey"
        ));
        assert_eq!(ops(&received, 1), ["auth"]);
    }

    #[tokio::test]
    async fn rejected_subscribe_is_reported_with_its_topics() {
        let (url, _received) = spawn_bybit_server(ServerScript {
            auth_ok: true,
            close_first_after: None,
        })
        .await;
        let (handle, mut events) = Stream::new(test_config(url));
        handle.connect().await.unwrap();

        handle.subscribe(vec![ticker("BAD")]).await.unwrap();
        let event =
            wait_for_event(&mut events, |e| matches!(e, Event::SubscribeFailed { .. })).await;

        assert!(matches!(
            event,
            Event::SubscribeFailed { ref topics, .. } if *topics == [ticker("BAD")]
        ));
    }

    #[tokio::test]
    async fn subscriptions_are_sent_in_batches_of_ten() {
        let (url, received) = spawn_bybit_server(ServerScript {
            auth_ok: true,
            close_first_after: None,
        })
        .await;
        let (handle, _events) = Stream::new(test_config(url));
        let topics: Vec<Topic> = (0..25).map(|i| ticker(&format!("SYM{i}"))).collect();
        handle.subscribe(topics).await.unwrap();
        // Subscribing again to known topics sends nothing.
        handle.subscribe(vec![ticker("SYM0")]).await.unwrap();
        handle.connect().await.unwrap();

        wait_for_frame(&received, |_, j| j["args"].to_string().contains("SYM24")).await;
        sleep(Duration::from_millis(100)).await;

        let sizes: Vec<usize> = received
            .lock()
            .unwrap()
            .iter()
            .map(|(_, j)| j["args"].as_array().unwrap().len())
            .collect();
        assert_eq!(sizes, [10, 10, 5]);
    }
}
