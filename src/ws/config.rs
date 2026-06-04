use std::time::Duration;

pub const DEFAULT_PING_INTERVAL: Duration = Duration::from_secs(20);
pub const DEFAULT_PONG_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug, Clone)]
pub struct Config {
    /// WebSocket server URL
    pub url: String,

    /// Maximum number of pending commands (backpressure)
    pub command_queue_size: usize,

    /// Maximum number of buffered events (backpressure)
    pub event_queue_size: usize,

    /// Maximum reconnect attempts (0 = no reconnect)
    pub max_reconnect_attempts: u32,

    /// Base delay between reconnect attempts
    pub reconnect_base_delay: Duration,

    /// Maximum delay cap for exponential back-off
    pub reconnect_max_delay: Duration,

    /// How long to wait for a clean close handshake
    pub close_timeout: Duration,

    /// How often to send a Ping frame to the server.
    /// `None` disables the heartbeat entirely.
    pub ping_interval: Option<Duration>,

    /// How long to wait for a Pong after sending a Ping before treating the
    /// connection as dead and triggering a reconnect.
    /// Ignored when `ping_interval` is `None`.
    pub pong_timeout: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            url: String::new(),
            command_queue_size: 64,
            event_queue_size: 256,
            max_reconnect_attempts: 5,
            reconnect_base_delay: Duration::from_millis(500),
            reconnect_max_delay: Duration::from_secs(30),
            close_timeout: Duration::from_secs(5),
            ping_interval: Some(DEFAULT_PING_INTERVAL),
            pong_timeout: DEFAULT_PONG_TIMEOUT,
        }
    }
}

impl Config {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            ..Default::default()
        }
    }

    pub fn command_queue_size(mut self, n: usize) -> Self {
        self.command_queue_size = n;
        self
    }

    pub fn event_queue_size(mut self, n: usize) -> Self {
        self.event_queue_size = n;
        self
    }

    pub fn max_reconnect_attempts(mut self, n: u32) -> Self {
        self.max_reconnect_attempts = n;
        self
    }

    pub fn reconnect_base_delay(mut self, d: Duration) -> Self {
        self.reconnect_base_delay = d;
        self
    }

    pub fn reconnect_max_delay(mut self, d: Duration) -> Self {
        self.reconnect_max_delay = d;
        self
    }

    pub fn close_timeout(mut self, d: Duration) -> Self {
        self.close_timeout = d;
        self
    }

    pub fn ping_interval(mut self, d: Option<Duration>) -> Self {
        self.ping_interval = d;
        self
    }

    pub fn pong_timeout(mut self, d: Duration) -> Self {
        self.pong_timeout = d;
        self
    }
}
