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
}
