use std::time::Duration;

use crate::{Category, Environment, SensitiveString, ServerClock, Timestamp};

/// Default interval of the application-level heartbeat ping.
pub const DEFAULT_PING_INTERVAL: Duration = Duration::from_secs(20);
/// Default time to wait for the heartbeat reply before reconnecting.
pub const DEFAULT_PONG_TIMEOUT: Duration = Duration::from_secs(10);
/// Default timeout of the TCP/TLS/WebSocket handshake.
pub const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
/// Default time [`TradeClient`](crate::ws::TradeClient) waits for a reply.
pub const DEFAULT_REQUEST_TIMEOUT: Duration = Duration::from_secs(10);
/// Default validity of the stream `auth` message, milliseconds.
pub const DEFAULT_AUTH_RECV_WINDOW: Timestamp = 5_000;

/// Configuration of a [`Stream`](crate::ws::Stream); start with [`Config::public`], [`Config::private`] or [`Config::new`].
#[derive(Debug, Clone)]
pub struct Config {
    /// WebSocket server URL
    pub url: String,

    /// Maximum number of pending commands (backpressure)
    pub command_queue_size: usize,

    /// Maximum number of buffered events (backpressure)
    pub event_queue_size: usize,

    /// Maximum number of consecutive reconnect attempts without a successful
    /// connection (0 = no reconnect). The counter resets once a connection is
    /// established.
    pub max_reconnect_attempts: u32,

    /// Base delay between reconnect attempts
    pub reconnect_base_delay: Duration,

    /// Maximum delay cap for exponential back-off
    pub reconnect_max_delay: Duration,

    /// How long to wait for a clean close handshake
    pub close_timeout: Duration,

    /// How long to wait for the TCP/TLS/WebSocket handshake to complete.
    pub connect_timeout: Duration,

    /// How often to send a Ping frame to the server.
    /// `None` disables the heartbeat entirely.
    pub ping_interval: Option<Duration>,

    /// How long to wait for a Pong after sending a Ping before treating the
    /// connection as dead and triggering a reconnect.
    /// Ignored when `ping_interval` is `None`.
    pub pong_timeout: Duration,

    /// API key for private streams. With [`Config::api_secret`] set, the
    /// driver authenticates on every (re)connect before subscribing.
    pub api_key: Option<SensitiveString>,

    /// API secret for private streams (see [`Config::api_key`]).
    pub api_secret: Option<SensitiveString>,

    /// How long the `auth` message (and, for
    /// [`TradeClient`](crate::ws::TradeClient), each order entry request)
    /// stays valid, milliseconds.
    pub auth_recv_window: Timestamp,

    /// Clock used for the `auth` expiry; share
    /// [`http::Client::clock`](crate::http::Client::clock) to use the offset
    /// measured by `sync_time`.
    pub server_clock: ServerClock,

    /// How long [`TradeClient`](crate::ws::TradeClient) waits for the reply
    /// to an order entry request.
    pub request_timeout: Duration,
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
            connect_timeout: DEFAULT_CONNECT_TIMEOUT,
            ping_interval: Some(DEFAULT_PING_INTERVAL),
            pong_timeout: DEFAULT_PONG_TIMEOUT,
            api_key: None,
            api_secret: None,
            auth_recv_window: DEFAULT_AUTH_RECV_WINDOW,
            server_clock: ServerClock::new(),
            request_timeout: DEFAULT_REQUEST_TIMEOUT,
        }
    }
}

impl Config {
    /// Config for a stream URL, e.g. `wss://stream.bybit.com/v5/public/linear`, with default settings.
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            ..Default::default()
        }
    }

    /// Config for the public stream of `category` in `env`.
    pub fn public(env: Environment, category: Category) -> Self {
        Self::new(env.public_stream_url(category))
    }

    /// Config for the order entry stream of `env`, used by
    /// [`TradeClient`](crate::ws::TradeClient). Fails where Bybit offers no
    /// order entry stream (see [`Environment::trade_stream_url`]).
    pub fn trade(env: Environment) -> Result<Self, crate::ws::Error> {
        env.trade_stream_url().map(Self::new).ok_or_else(|| {
            crate::ws::Error::InvalidUrl(format!("{env:?} has no order entry stream"))
        })
    }

    /// Config for the private stream (orders, positions, executions,
    /// wallet) in `env`.
    pub fn private(env: Environment) -> Self {
        Self::new(env.private_stream_url())
    }

    /// Maximum number of pending commands; further commands wait
    /// (or fail with `QueueFull` for `try_*`).
    pub fn command_queue_size(mut self, n: usize) -> Self {
        self.command_queue_size = n;
        self
    }

    /// Maximum number of buffered events; data events beyond it are dropped
    /// and reported by [`Event::Lagged`](crate::ws::Event::Lagged).
    pub fn event_queue_size(mut self, n: usize) -> Self {
        self.event_queue_size = n;
        self
    }

    /// Maximum consecutive reconnect attempts without a successful
    /// connection; 0 disables reconnecting.
    pub fn max_reconnect_attempts(mut self, n: u32) -> Self {
        self.max_reconnect_attempts = n;
        self
    }

    /// Delay before the first reconnect attempt; it doubles per attempt.
    pub fn reconnect_base_delay(mut self, d: Duration) -> Self {
        self.reconnect_base_delay = d;
        self
    }

    /// Upper bound of the reconnect delay.
    pub fn reconnect_max_delay(mut self, d: Duration) -> Self {
        self.reconnect_max_delay = d;
        self
    }

    /// How long to wait for the close handshake on disconnect.
    pub fn close_timeout(mut self, d: Duration) -> Self {
        self.close_timeout = d;
        self
    }

    /// Timeout of the TCP/TLS/WebSocket handshake.
    pub fn connect_timeout(mut self, d: Duration) -> Self {
        self.connect_timeout = d;
        self
    }

    /// Heartbeat interval; `None` disables the heartbeat.
    pub fn ping_interval(mut self, d: Option<Duration>) -> Self {
        self.ping_interval = d;
        self
    }

    /// How long to wait for the heartbeat reply before reconnecting.
    pub fn pong_timeout(mut self, d: Duration) -> Self {
        self.pong_timeout = d;
        self
    }

    /// Credentials for a private stream: the driver sends `auth` on every
    /// (re)connect and subscribes only after it succeeded.
    pub fn credentials(
        mut self,
        api_key: impl Into<SensitiveString>,
        api_secret: impl Into<SensitiveString>,
    ) -> Self {
        self.api_key = Some(api_key.into());
        self.api_secret = Some(api_secret.into());
        self
    }

    /// Validity of the `auth` message, milliseconds.
    pub fn auth_recv_window(mut self, recv_window: Timestamp) -> Self {
        self.auth_recv_window = recv_window;
        self
    }

    /// How long [`TradeClient`](crate::ws::TradeClient) waits for a reply.
    pub fn request_timeout(mut self, d: Duration) -> Self {
        self.request_timeout = d;
        self
    }

    /// Clock for the `auth` expiry, e.g. `client.clock()`.
    pub fn server_clock(mut self, clock: ServerClock) -> Self {
        self.server_clock = clock;
        self
    }
}
