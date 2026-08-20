use std::{collections::HashMap, sync::Mutex, time::Instant};

use crate::{Path, enums::Category};

/// Sustained throughput and burst capacity for one rate-limit bucket.
#[derive(Debug, Clone, Copy)]
pub struct BucketLimit {
    /// Sustained throughput in requests per second.
    pub requests_per_second: f64,
    /// Maximum burst capacity (also the initial token count for a bucket).
    ///
    /// Must be at least as large as the biggest single call's cost you expect to make against
    /// this bucket, or that call can never succeed. In particular, a bucket used for a batch
    /// order endpoint needs `burst >= ` the largest batch size you'll send (Bybit currently
    /// caps batches at 20 orders for linear, 10 for others).
    pub burst: u32,
}

/// Identifies one of Bybit's independent per-UID rate-limit buckets.
///
/// Bybit tracks limits per UID *per endpoint*
/// (<https://bybit-exchange.github.io/docs/v5/rate-limit>: "Each endpoint has its own
/// independent limit"), and for the order endpoints (create / amend / cancel / cancel-all,
/// single and batch) the limit additionally varies by `category`. `category: None` means the
/// endpoint has one limit regardless of category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RateLimitKey {
    pub path: Path,
    pub category: Option<Category>,
}

impl RateLimitKey {
    pub const fn new(path: Path) -> Self {
        Self {
            path,
            category: None,
        }
    }

    pub const fn with_category(path: Path, category: Category) -> Self {
        Self {
            path,
            category: Some(category),
        }
    }
}

/// Configuration for the optional per-endpoint rate limiter.
///
/// Pass this via [`Config::rate_limiter`](crate::http::Config::rate_limiter) to have the
/// client pre-flight-check a local bucket before every request and reject calls that would
/// obviously hit the exchange's limits.
///
/// Bybit tracks a separate limit per endpoint (and, for order endpoints, per category) rather
/// than one account-wide limit, and batch order endpoints consume one unit of quota *per order
/// in the batch*, not one per request
/// (<https://bybit-exchange.github.io/docs/v5/rate-limit>). This limiter mirrors that: each
/// [`RateLimitKey`] gets its own independent token bucket and its own server-feedback state,
/// and every call site reports how many units it actually costs.
///
/// Two checks are combined per bucket:
///
/// 1. **Token bucket** — tokens refill at the bucket's `requests_per_second`. An initial burst
///    of `burst` tokens is available. When the bucket doesn't have enough tokens for the
///    request's cost, the request is rejected immediately with [`Error::RateLimited`].
///
/// 2. **Server feedback** — after every response, Bybit's `X-Bapi-Limit-Status` /
///    `X-Bapi-Limit-Reset-Timestamp` headers are recorded against that response's bucket. If
///    the server-reported remaining count reaches **zero**, the next request against that same
///    bucket is rejected with [`Error::RateLimited`] until the reset window passes, regardless
///    of local token state.
///
/// Bybit's limits vary by account VIP/Pro tier and change over time, so this crate does not
/// hardcode them: populate `overrides` yourself from
/// <https://bybit-exchange.github.io/docs/v5/rate-limit>, or start from
/// [`RateLimiterConfig::bybit_base_tier_defaults`] and adjust it for your tier.
///
/// [`Error::RateLimited`]: crate::Error::RateLimited
#[derive(Debug, Clone)]
pub struct RateLimiterConfig {
    /// Limit applied to any [`RateLimitKey`] not present in `overrides`.
    pub default: BucketLimit,
    /// Per-endpoint (optionally per-category) overrides.
    pub overrides: HashMap<RateLimitKey, BucketLimit>,
}

impl RateLimiterConfig {
    /// A single limit shared by every endpoint, with no per-endpoint overrides.
    ///
    /// Equivalent to this crate's previous, endpoint-unaware rate limiter. Prefer
    /// [`RateLimiterConfig::bybit_base_tier_defaults`] for a more accurate starting point.
    pub fn uniform(requests_per_second: f64, burst: u32) -> Self {
        Self {
            default: BucketLimit {
                requests_per_second,
                burst,
            },
            overrides: HashMap::new(),
        }
    }

    /// Starting-point overrides for the base (non-VIP) tier's most commonly used endpoints,
    /// taken from <https://bybit-exchange.github.io/docs/v5/rate-limit>. Not exhaustive — it
    /// only covers the Trade endpoints (where the limit varies sharply by category and, for
    /// batch endpoints, is easy to blow through without per-order weighting) plus a couple of
    /// high-traffic Account ones. Add your own overrides for anything else, and adjust these
    /// if your account has a VIP/Pro tier.
    ///
    /// `default` is set to a conservative 5 req/s, 5 burst for everything not listed here.
    pub fn bybit_base_tier_defaults() -> Self {
        let mut overrides = HashMap::new();

        let mut set = |path: Path, category: Category, rps: f64, burst: u32| {
            overrides.insert(
                RateLimitKey::with_category(path, category),
                BucketLimit {
                    requests_per_second: rps,
                    burst,
                },
            );
        };

        // Create Order.
        set(Path::TradeOrderCreate, Category::Linear, 20.0, 20);
        set(Path::TradeOrderCreate, Category::Inverse, 10.0, 10);
        set(Path::TradeOrderCreate, Category::Spot, 10.0, 10);
        set(Path::TradeOrderCreate, Category::Option, 10.0, 10);

        // Amend Order.
        set(Path::TradeOrderAmend, Category::Linear, 10.0, 10);
        set(Path::TradeOrderAmend, Category::Inverse, 10.0, 10);
        set(Path::TradeOrderAmend, Category::Spot, 10.0, 10);
        set(Path::TradeOrderAmend, Category::Option, 10.0, 10);

        // Cancel Order.
        set(Path::TradeOrderCancel, Category::Linear, 20.0, 20);
        set(Path::TradeOrderCancel, Category::Inverse, 10.0, 10);
        set(Path::TradeOrderCancel, Category::Spot, 10.0, 10);
        set(Path::TradeOrderCancel, Category::Option, 10.0, 10);

        // Cancel All Orders.
        set(Path::TradeOrderCancelAll, Category::Linear, 20.0, 20);
        set(Path::TradeOrderCancelAll, Category::Inverse, 1.0, 1);
        set(Path::TradeOrderCancelAll, Category::Spot, 10.0, 10);
        set(Path::TradeOrderCancelAll, Category::Option, 10.0, 10);

        // Batch endpoints: independent limit from the single-order ones above, and the burst
        // must cover the largest batch you'll send (Bybit currently caps batches at 20 orders
        // for linear, 10 for others), since one call consumes `cost = orders.len()`.
        for path in [
            Path::TradeOrderCreateBatch,
            Path::TradeOrderAmendBatch,
            Path::TradeOrderCancelBatch,
        ] {
            set(path, Category::Linear, 20.0, 20);
            set(path, Category::Inverse, 10.0, 10);
            set(path, Category::Spot, 10.0, 10);
        }

        // A few high-traffic, category-independent endpoints.
        overrides.insert(
            RateLimitKey::new(Path::TradeOrderRealtime),
            BucketLimit {
                requests_per_second: 50.0,
                burst: 50,
            },
        );
        overrides.insert(
            RateLimitKey::new(Path::TradeOrderHistory),
            BucketLimit {
                requests_per_second: 50.0,
                burst: 50,
            },
        );
        overrides.insert(
            RateLimitKey::new(Path::AccountWalletBalance),
            BucketLimit {
                requests_per_second: 50.0,
                burst: 50,
            },
        );
        overrides.insert(
            RateLimitKey::new(Path::AccountFeeRate),
            BucketLimit {
                requests_per_second: 5.0,
                burst: 5,
            },
        );

        Self {
            default: BucketLimit {
                requests_per_second: 5.0,
                burst: 5,
            },
            overrides,
        }
    }

    pub fn with_override(mut self, key: RateLimitKey, limit: BucketLimit) -> Self {
        self.overrides.insert(key, limit);
        self
    }
}

pub(crate) struct RateLimiter {
    config: RateLimiterConfig,
    buckets: Mutex<HashMap<RateLimitKey, Bucket>>,
}

impl std::fmt::Debug for RateLimiter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let g = self.buckets.lock().unwrap();
        f.debug_struct("RateLimiter")
            .field("default", &self.config.default)
            .field("overrides", &self.config.overrides.len())
            .field("active_buckets", &g.len())
            .finish()
    }
}

struct Bucket {
    limit: BucketLimit,
    tokens: f64,
    last_refill: Instant,
    server_remaining: Option<u64>,
    server_reset_at_ms: Option<u64>,
}

impl Bucket {
    fn new(limit: BucketLimit) -> Self {
        Self {
            tokens: limit.burst as f64,
            limit,
            last_refill: Instant::now(),
            server_remaining: None,
            server_reset_at_ms: None,
        }
    }
}

impl RateLimiter {
    pub(crate) fn new(config: RateLimiterConfig) -> Self {
        Self {
            config,
            buckets: Mutex::new(HashMap::new()),
        }
    }

    fn limit_for(&self, key: &RateLimitKey) -> BucketLimit {
        self.config
            .overrides
            .get(key)
            .copied()
            .unwrap_or(self.config.default)
    }

    /// Returns `Ok(())` if the request may proceed, consuming `cost` tokens from the bucket
    /// identified by `key`. Returns `Err(retry_after_ms)` if it should be rejected pre-flight.
    pub(crate) fn check(&self, key: RateLimitKey, cost: u32) -> Result<(), u64> {
        let limit = self.limit_for(&key);
        let mut g = self.buckets.lock().unwrap();
        let bucket = g.entry(key).or_insert_with(|| Bucket::new(limit));

        // Server-feedback check: reject immediately if the server told us this bucket's
        // window is exhausted and the reset time hasn't arrived yet.
        if let (Some(0), Some(reset_at)) = (bucket.server_remaining, bucket.server_reset_at_ms) {
            let now_ms = unix_ms();
            if now_ms < reset_at {
                return Err(reset_at - now_ms);
            }
            // Window has expired — clear stale server state.
            bucket.server_remaining = None;
            bucket.server_reset_at_ms = None;
        }

        // Token bucket: refill proportional to elapsed time, then try to consume `cost` tokens.
        let now = Instant::now();
        let elapsed = now.duration_since(bucket.last_refill).as_secs_f64();
        bucket.tokens = (bucket.tokens + elapsed * bucket.limit.requests_per_second)
            .min(bucket.limit.burst as f64);
        bucket.last_refill = now;

        let cost = cost as f64;
        if bucket.tokens < cost {
            let wait_ms =
                ((cost - bucket.tokens) / bucket.limit.requests_per_second * 1000.0).ceil() as u64;
            return Err(wait_ms);
        }

        bucket.tokens -= cost;
        Ok(())
    }

    /// Store server-reported rate-limit state from response headers against `key`'s bucket.
    pub(crate) fn update(
        &self,
        key: RateLimitKey,
        remaining: Option<u64>,
        reset_at_ms: Option<u64>,
    ) {
        if remaining.is_none() && reset_at_ms.is_none() {
            return;
        }
        let limit = self.limit_for(&key);
        let mut g = self.buckets.lock().unwrap();
        let bucket = g.entry(key).or_insert_with(|| Bucket::new(limit));
        if let Some(r) = remaining {
            bucket.server_remaining = Some(r);
        }
        if let Some(t) = reset_at_ms {
            bucket.server_reset_at_ms = Some(t);
        }
    }
}

fn unix_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}
