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
    /// `dropped` data events (`Message`/`ParseError`) were discarded because
    /// the event queue was full. Local state built from the stream (e.g. an
    /// order book) may be stale and should be resynchronized.
    Lagged {
        dropped: u64,
    },
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
