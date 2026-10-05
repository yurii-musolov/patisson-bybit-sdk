//! Frequent Bybit `retCode` values, as found in [`Error::Api`](crate::Error::Api).
//!
//! See <https://bybit-exchange.github.io/docs/v5/error> for the full list.

/// Invalid request parameters.
pub const PARAMS_ERROR: i64 = 10001;
/// The request timestamp is outside `recv_window` (see
/// [`http::Client::sync_time`](crate::http::Client::sync_time)).
pub const TIMESTAMP_OUT_OF_WINDOW: i64 = 10002;
/// The API key is invalid or belongs to another environment (mainnet,
/// testnet, demo).
pub const INVALID_API_KEY: i64 = 10003;
/// The signature does not match.
pub const INVALID_SIGNATURE: i64 = 10004;
/// The API key lacks the permission for this endpoint.
pub const PERMISSION_DENIED: i64 = 10005;
/// Too many requests: the per-UID rate limit was exceeded.
pub const RATE_LIMITED: i64 = 10006;
/// User authentication failed.
pub const AUTHENTICATION_FAILED: i64 = 10007;
/// The request IP is not bound to the API key.
pub const IP_NOT_ALLOWED: i64 = 10010;
/// Internal server error or service restarting.
pub const SERVER_ERROR: i64 = 10016;
/// Route not found.
pub const ROUTE_NOT_FOUND: i64 = 10017;
/// Too many requests: the per-IP rate limit was exceeded.
pub const IP_RATE_LIMITED: i64 = 10018;
/// Trading is banned for the account.
pub const TRADING_BANNED: i64 = 10027;
/// The symbol is not in the API key's symbol whitelist.
pub const SYMBOL_NOT_ALLOWED: i64 = 10029;
/// The order does not exist (derivatives).
pub const ORDER_NOT_FOUND: i64 = 110001;
/// The wallet balance is insufficient (derivatives).
pub const INSUFFICIENT_WALLET_BALANCE: i64 = 110004;
/// The available balance is insufficient (derivatives).
pub const INSUFFICIENT_AVAILABLE_BALANCE: i64 = 110007;
/// The position mode is already the requested one.
pub const POSITION_MODE_NOT_MODIFIED: i64 = 110025;
/// The leverage is already the requested one.
pub const LEVERAGE_NOT_MODIFIED: i64 = 110043;
/// The balance is insufficient (spot).
pub const SPOT_INSUFFICIENT_BALANCE: i64 = 170131;
/// The order does not exist (spot).
pub const SPOT_ORDER_NOT_FOUND: i64 = 170213;
