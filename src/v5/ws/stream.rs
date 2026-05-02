use futures_util::{SinkExt, StreamExt};
use std::time::Duration;
use tokio::{
    sync::mpsc,
    time::{Instant, sleep, timeout},
};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tracing::{debug, error, info, warn};

use crate::v5::{
    IncomingMessage, OutgoingMessage,
    serde::{deserialize_json, serialize_json},
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
                State::Closing { sink } => self.step_closing(sink).await,
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
                    return State::Connecting { attempt: 1 };
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
        debug!(attempt, "connecting…");

        match connect_async(&self.config.url).await {
            Ok((ws_stream, _)) => {
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
            Err(e) => {
                error!(error = %e, attempt, "connection failed");
                self.next_reconnect_state(attempt + 1, e.to_string())
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
                        return self.next_reconnect_state(1, "remote closed".into());
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
                                    Err(e) => warn!(error = %e, "parsing IncomingMessage failed")
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
                        return self.next_reconnect_state(1, e.to_string());
                    }
                },

                cmd = self.cmd_rx.recv() => match cmd {
                    None | Some(Command::Disconnect) => {
                        info!("disconnect requested");
                        read_task.abort();
                        return State::Closing { sink };
                    }
                    Some(Command::Send(msg)) => {
                        let json = serialize_json(&msg).expect("serialize outgoing message failed");
                        let msg = Message::Text(json.into());
                        if let Err(e) = sink.send(msg).await {
                            error!(error = %e, "send error");
                            read_task.abort();
                            return self.next_reconnect_state(1, e.to_string());
                        }
                    }
                    Some(Command::Connect) => warn!("Connect ignored - already connected")
                },

                () = &mut ping_timer, if ping_interval.is_some() => {
                    debug!("sending heartbeat ping");
                    if let Err(e) = sink.send(ping()).await {
                        error!(error = %e, "ping send error");
                        read_task.abort();
                        return self.next_reconnect_state(1, e.to_string());
                    }

                    hb = HeartbeatState::PingSent;
                    pong_timer.as_mut().reset(Instant::now() + pong_timeout);
                    ping_timer.as_mut().reset(far_future_instant());
                },

                () = &mut pong_timer, if matches!(hb, HeartbeatState::PingSent) => {
                    warn!("pong timeout - connection appears dead");
                    read_task.abort();
                    return self.next_reconnect_state( 1, "pong timeout".into());
                },
            }
        }
    }

    async fn step_reconnecting(&mut self, attempt: u32, delay_ms: u64) -> State {
        warn!(attempt, delay_ms, "waiting before reconnect");
        self.emit(Event::Reconnecting { attempt, delay_ms });

        let cancelled = tokio::select! {
            _ = sleep(Duration::from_millis(delay_ms)) => false,
            cmd = self.cmd_rx.recv() => matches!(cmd, None | Some(Command::Disconnect)),
        };

        if cancelled {
            self.emit(Event::Disconnected {
                reason: DisconnectReason::Requested,
            });
            State::Idle
        } else {
            State::Connecting { attempt }
        }
    }

    async fn step_closing(&mut self, mut sink: Sink) -> State {
        if let Err(e) = sink.send(Message::Close(None)).await {
            error!(error = %e, "send close message failed");
        }
        if let Err(e) = timeout(self.config.close_timeout, self.cmd_rx.recv()).await {
            error!(error = %e, "waiting for a clean close handshake failed");
        }

        self.emit(Event::Disconnected {
            reason: DisconnectReason::Requested,
        });
        State::Idle
    }

    fn next_reconnect_state(&self, next_attempt: u32, reason: String) -> State {
        if self.config.max_reconnect_attempts == 0
            || next_attempt > self.config.max_reconnect_attempts
        {
            self.emit(Event::Disconnected {
                reason: DisconnectReason::Error(String::from(
                    "all reconnection attempts have failed",
                )),
            });
            return State::Idle;
        }

        let base_ms = self.config.reconnect_base_delay.as_millis() as u64;
        let max_ms = self.config.reconnect_max_delay.as_millis() as u64;
        let delay_ms = (base_ms.saturating_mul(1u64 << (next_attempt - 1).min(10))).min(max_ms);

        debug!(next_attempt, delay_ms, reason, "scheduling reconnect");
        State::Reconnecting {
            attempt: next_attempt,
            delay_ms,
        }
    }
}

const FAR_FUTURE: Duration = Duration::from_secs(u64::MAX / 4);

#[inline]
fn far_future_instant() -> Instant {
    Instant::now() + FAR_FUTURE
}

#[inline]
fn ping() -> Message {
    let msg = OutgoingMessage::Ping { req_id: None };
    let json = serialize_json(&msg).expect("serialize ping outgoing message failed");
    Message::Text(json.into())
}
