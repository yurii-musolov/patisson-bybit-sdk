use std::{sync::Mutex, time::Instant};

/// Configuration for the optional token-bucket rate limiter.
///
/// Pass this via [`Config::rate_limiter`] to have the client pre-flight-check
/// a local bucket before every request and reject calls that would obviously
/// hit the exchange's limits.
///
/// Two checks are combined:
///
/// 1. **Token bucket** — tokens refill at `requests_per_second`. An initial
///    burst of `burst` tokens is available. When the bucket is empty the
///    request is rejected immediately with [`Error::RateLimited`].
///
/// 2. **Server feedback** — after every response Bybit's
///    `X-Bapi-Limit-Status` / `X-Bapi-Limit-Reset-Timestamp` headers are
///    recorded. If the server-reported remaining count reaches **zero** the
///    next request is rejected with [`Error::RateLimited`] until the reset
///    window passes, regardless of local token state.
///
/// [`Error::RateLimited`]: crate::Error::RateLimited
#[derive(Debug, Clone)]
pub struct RateLimiterConfig {
    /// Sustained throughput in requests per second.
    pub requests_per_second: f64,
    /// Maximum burst capacity (also the initial token count on startup).
    pub burst: u32,
}

pub(crate) struct RateLimiter {
    rps: f64,
    max_tokens: f64,
    inner: Mutex<Inner>,
}

impl std::fmt::Debug for RateLimiter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let g = self.inner.lock().unwrap();
        f.debug_struct("RateLimiter")
            .field("rps", &self.rps)
            .field("max_tokens", &self.max_tokens)
            .field("tokens", &g.tokens)
            .finish()
    }
}

struct Inner {
    tokens: f64,
    last_refill: Instant,
    server_remaining: Option<u64>,
    server_reset_at_ms: Option<u64>,
}

impl RateLimiter {
    pub(crate) fn new(cfg: RateLimiterConfig) -> Self {
        let burst = cfg.burst as f64;
        Self {
            rps: cfg.requests_per_second,
            max_tokens: burst,
            inner: Mutex::new(Inner {
                tokens: burst,
                last_refill: Instant::now(),
                server_remaining: None,
                server_reset_at_ms: None,
            }),
        }
    }

    /// Returns `Ok(())` if the request may proceed.
    /// Returns `Err(retry_after_ms)` if it should be rejected pre-flight.
    pub(crate) fn check(&self) -> Result<(), u64> {
        let mut g = self.inner.lock().unwrap();

        // Server-feedback check: reject immediately if the server told us the
        // window is exhausted and the reset time hasn't arrived yet.
        if let (Some(0), Some(reset_at)) = (g.server_remaining, g.server_reset_at_ms) {
            let now_ms = unix_ms();
            if now_ms < reset_at {
                return Err(reset_at - now_ms);
            }
            // Window has expired — clear stale server state.
            g.server_remaining = None;
            g.server_reset_at_ms = None;
        }

        // Token bucket: refill proportional to elapsed time, then try to
        // consume one token.
        let now = Instant::now();
        let elapsed = now.duration_since(g.last_refill).as_secs_f64();
        g.tokens = (g.tokens + elapsed * self.rps).min(self.max_tokens);
        g.last_refill = now;

        if g.tokens < 1.0 {
            let wait_ms = ((1.0 - g.tokens) / self.rps * 1000.0).ceil() as u64;
            return Err(wait_ms);
        }

        g.tokens -= 1.0;
        Ok(())
    }

    /// Store server-reported rate-limit state from response headers.
    pub(crate) fn update(&self, remaining: Option<u64>, reset_at_ms: Option<u64>) {
        if remaining.is_none() && reset_at_ms.is_none() {
            return;
        }
        let mut g = self.inner.lock().unwrap();
        if let Some(r) = remaining {
            g.server_remaining = Some(r);
        }
        if let Some(t) = reset_at_ms {
            g.server_reset_at_ms = Some(t);
        }
    }
}

fn unix_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}
