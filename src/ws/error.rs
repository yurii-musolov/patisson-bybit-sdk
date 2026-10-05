/// Errors of the WebSocket driver and its [`Handle`](crate::ws::Handle).
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The stream URL is invalid.
    #[error("invalid URL: {0}")]
    InvalidUrl(String),

    /// The command queue is full (`try_*` methods of [`Handle`](crate::ws::Handle)).
    #[error("command queue full - backpressure limit reached")]
    QueueFull,

    /// The driver task has stopped.
    #[error("driver has shut down")]
    DriverGone,

    /// WebSocket protocol or transport error.
    #[error("websocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),

    /// I/O error.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    /// [`TradeClient`](crate::ws::TradeClient) needs `api_key` and
    /// `api_secret` in the [`Config`](crate::ws::Config).
    #[error("api_key and api_secret are required for the order entry stream")]
    MissingCredentials,

    /// An order entry request was sent while the stream was not connected
    /// and authenticated ([`TradeClient`](crate::ws::TradeClient)).
    #[error("order entry stream is not connected and authenticated")]
    NotConnected,

    /// No reply to an order entry request within the request timeout. The
    /// order may or may not have been accepted: check it before retrying.
    #[error("no reply within the request timeout")]
    Timeout,

    /// The connection was lost before the reply arrived. The order may or
    /// may not have been accepted: check it before retrying.
    #[error("connection lost before the reply arrived")]
    ConnectionLost,

    /// Bybit rejected the request (see [`ret_code`](crate::ret_code)).
    #[error("Bybit API error: code: {code}, message: {msg}")]
    Api {
        /// Bybit `retCode`.
        code: i64,
        /// Bybit `retMsg`.
        msg: String,
    },

    /// The reply data could not be parsed.
    #[error("invalid reply: {0}")]
    InvalidReply(String),
}
