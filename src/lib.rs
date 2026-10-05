//! Unofficial Rust SDK for the [Bybit V5 API](https://bybit-exchange.github.io/docs/v5/intro).
//!
//! The library crate is named `bybit` (package `patisson-bybit-sdk`).
//!
//! - [`http::Client`]: REST endpoints (market data, trading, positions, account,
//!   assets), request signing, optional local rate limiting
//!   ([`http::RateLimiterConfig`]) and clock synchronization
//!   ([`http::Client::sync_time`]).
//! - [`ws::Stream`]: WebSocket driver with automatic reconnects and heartbeat;
//!   commands go through [`ws::Handle`], messages and connection state arrive as
//!   [`ws::Event`]s.
//! - [`OrderBookState`]: local order book from snapshots and deltas.
//! - [`AccountState`]: local copy of open orders, positions and wallet
//!   balances from REST snapshots and the private stream.
//!
//! Everything is async and runs on Tokio.
//!
//! # TLS
//!
//! REST and WebSocket use the same TLS backend, chosen with a Cargo feature
//! (enable exactly one): `rustls` (default; aws-lc-rs and the operating
//! system's root certificates, no OpenSSL) or `native-tls` (OpenSSL,
//! Schannel or Security.framework).
//!
//! # REST
//!
//! ```no_run
//! use bybit::{
//!     Category, Environment,
//!     http::{Client, Config, GetTickersParams},
//! };
//!
//! # async fn run() -> Result<(), bybit::Error> {
//! let client = Client::new(Config::for_env(Environment::Mainnet))?;
//! let params = GetTickersParams::new(Category::Linear).with_symbol("BTCUSDT");
//! let tickers = client.get_tickers(&params).await?;
//! println!("{:?}", tickers.result);
//! # Ok(())
//! # }
//! ```
//!
//! Private endpoints need credentials. Call [`http::Client::sync_time`] once
//! so that signed requests use the server clock even if the local clock is
//! off:
//!
//! ```no_run
//! use bybit::{
//!     AccountType, Environment,
//!     http::{Client, Config, GetWalletBalanceParams},
//! };
//!
//! # async fn run() -> Result<(), bybit::Error> {
//! let client = Client::new(Config::for_env(Environment::Demo).credentials("API_KEY", "API_SECRET"))?;
//! client.sync_time().await?;
//! let params = GetWalletBalanceParams {
//!     account_type: AccountType::UNIFIED,
//!     coin: None,
//! };
//! let wallet = client.get_wallet_balance(&params).await?;
//! # Ok(())
//! # }
//! ```
//!
//! # WebSocket
//!
//! The driver reconnects by itself and restores the topics subscribed with
//! [`ws::Handle::subscribe`]. With [`ws::Config::credentials`] it also sends
//! `auth` on every connection (using [`ServerClock`], shared with
//! [`http::Client::clock`]) and subscribes once it succeeded:
//!
//! ```no_run
//! use bybit::{
//!     Environment, Topic,
//!     http::{Client, Config},
//!     ws,
//! };
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let client = Client::new(Config::for_env(Environment::Demo))?;
//! client.sync_time().await?;
//!
//! let config = ws::Config::private(Environment::Demo)
//!     .credentials("API_KEY", "API_SECRET")
//!     .server_clock(client.clock());
//! let (handle, mut events) = ws::Stream::new(config);
//! handle.subscribe(vec![Topic::OrderAllCategory, Topic::Wallet]).await?;
//! handle.connect().await?;
//!
//! while let Some(event) = events.recv().await {
//!     match event {
//!         ws::Event::Message(msg) => println!("{msg:?}"),
//!         ws::Event::AuthFailed { .. } | ws::Event::Disconnected { .. } => break,
//!         _ => {}
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! Public streams work the same way without credentials
//! ([`ws::Config::public`]). See the
//! [examples](https://github.com/yurii-musolov/patisson-bybit-sdk/tree/main/examples),
//! in particular `account-state` for [`AccountState`] and `stream-orderbook`
//! for [`OrderBookState`].
//!
//! # Versioning
//!
//! There are no stability guarantees yet: treat every version change as
//! breaking and pin an exact version (`patisson-bybit-sdk = "=x.y.z"`).

#[cfg(not(any(feature = "rustls", feature = "native-tls")))]
compile_error!("enable a TLS backend: the `rustls` (default) or the `native-tls` feature");

mod account_state;
mod clock;
mod common;
mod crypto;
mod enums;
mod environment;
mod error;
mod orderbook_state;
mod serde;
mod url;

pub mod http;
pub mod ws;

pub use account_state::*;
pub use clock::ServerClock;
pub use common::*;
pub use crypto::*;
pub use enums::*;
pub use environment::Environment;
pub use error::*;
pub use orderbook_state::*;
// The BASE_URL_* constants are deprecated in favour of `Environment`.
#[allow(deprecated)]
pub use url::{
    BASE_URL_API_DEMO, BASE_URL_API_MAINNET_1, BASE_URL_API_MAINNET_2, BASE_URL_API_MAINNET_3,
    BASE_URL_API_MAINNET_4, BASE_URL_API_MAINNET_5, BASE_URL_API_MAINNET_6, BASE_URL_API_TESTNET,
    BASE_URL_STREAM_DEMO, BASE_URL_STREAM_MAINNET_1, BASE_URL_STREAM_MAINNET_2,
    BASE_URL_STREAM_MAINNET_3, BASE_URL_STREAM_TESTNET, Path,
};
