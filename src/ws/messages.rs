use crate::{
    Topic,
    ws::{IncomingMessage, OutgoingMessage},
};

#[derive(Debug)]
pub enum Command {
    Connect,
    /// Send a raw message. Subscriptions sent this way are not restored
    /// after a reconnect; use [`Command::Subscribe`] for that.
    Send(OutgoingMessage),
    /// Subscribe now (if connected) and after every reconnect.
    Subscribe(Vec<Topic>),
    /// Unsubscribe and stop restoring the topics.
    Unsubscribe(Vec<Topic>),
    Disconnect,
}

#[derive(Debug)]
pub enum Event {
    /// The connection is open. With credentials configured, `auth` has been
    /// sent; subscriptions follow once it succeeds.
    Connected,
    /// The stream accepted the credentials (private streams).
    Authenticated,
    /// The stream rejected the credentials; no topics are subscribed on this
    /// connection.
    AuthFailed {
        ret_msg: Option<String>,
    },
    /// The stream rejected a `subscribe` sent by the driver for `topics`.
    SubscribeFailed {
        topics: Vec<Topic>,
        ret_msg: Option<String>,
    },
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
