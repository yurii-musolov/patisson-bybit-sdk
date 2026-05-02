#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid URL: {0}")]
    InvalidUrl(String),

    #[error("command queue full - backpressure limit reached")]
    QueueFull,

    #[error("driver has shut down")]
    DriverGone,

    #[error("websocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
