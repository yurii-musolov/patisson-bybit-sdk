use crate::{
    Topic,
    ws::{IncomingMessage, OutgoingMessage},
};

/// Commands from a [`Handle`](crate::ws::Handle) to the driver.
#[derive(Debug)]
pub enum Command {
    /// Open the connection.
    Connect,
    /// Send a raw message. Subscriptions sent this way are not restored
    /// after a reconnect; use [`Command::Subscribe`] for that.
    Send(OutgoingMessage),
    /// Subscribe now (if connected) and after every reconnect.
    Subscribe(Vec<Topic>),
    /// Unsubscribe and stop restoring the topics.
    Unsubscribe(Vec<Topic>),
    /// Close the connection.
    Disconnect,
}

/// Events from the driver, received from [`Stream::new`](crate::ws::Stream::new).
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
        /// `ret_msg` of the rejected `auth`.
        ret_msg: Option<String>,
    },
    /// The stream rejected a `subscribe` sent by the driver for `topics`.
    SubscribeFailed {
        /// Topics of the rejected `subscribe`.
        topics: Vec<Topic>,
        /// `ret_msg` of the rejected `subscribe`.
        ret_msg: Option<String>,
    },
    /// A message from the stream (data or a reply to a command).
    Message(IncomingMessage),
    /// A WebSocket text frame arrived but could not be deserialized.
    /// The connection stays open — the raw error description is included.
    ParseError(String),
    /// `dropped` data events (`Message`/`ParseError`) were discarded because
    /// the event queue was full. Local state built from the stream (e.g. an
    /// order book) may be stale and should be resynchronized.
    Lagged {
        /// Number of dropped events.
        dropped: u64,
    },
    /// The connection was lost; the driver reconnects after `delay_ms`.
    Reconnecting {
        /// Number of this reconnect attempt, starting at 1.
        attempt: u32,
        /// Delay before the attempt, milliseconds.
        delay_ms: u64,
    },
    /// The connection is closed and will not be reopened automatically.
    Disconnected {
        /// Why the connection was closed.
        reason: DisconnectReason,
    },
}

/// Why the connection was closed.
#[derive(Debug, Clone)]
pub enum DisconnectReason {
    /// [`Handle::disconnect`](crate::ws::Handle::disconnect) was called (or all handles were dropped).
    Requested,
    /// The server closed the connection and reconnecting gave up.
    RemoteClosed,
    /// No heartbeat reply in time and reconnecting gave up.
    PongTimeout,
    /// A connection or protocol error and reconnecting gave up.
    Error(String),
}
