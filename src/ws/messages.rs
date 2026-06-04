use crate::ws::{IncomingMessage, OutgoingMessage};

#[derive(Debug)]
pub enum Command {
    Connect,
    Send(OutgoingMessage),
    Disconnect,
}

#[derive(Debug)]
pub enum Event {
    Connected,
    Message(IncomingMessage),
    /// A WebSocket text frame arrived but could not be deserialized.
    /// The connection stays open — the raw error description is included.
    ParseError(String),
    Reconnecting {
        attempt: u32,
        delay_ms: u64,
    },
    Disconnected {
        reason: DisconnectReason,
    },
}

#[derive(Debug, Clone)]
pub enum DisconnectReason {
    Requested,
    RemoteClosed,
    PongTimeout,
    Error(String),
}
