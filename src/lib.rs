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
//! # REST
//!
//! ```no_run
//! use bybit::{
//!     BASE_URL_API_MAINNET_1, Category,
//!     http::{Client, Config, GetTickersParams},
//! };
//!
//! # async fn run() -> Result<(), bybit::Error> {
//! let client = Client::new(Config::new(BASE_URL_API_MAINNET_1))?;
//! let params = GetTickersParams {
//!     category: Category::Linear,
//!     symbol: Some(String::from("BTCUSDT")),
//!     base_coin: None,
//!     exp_date: None,
//! };
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
//!     AccountType, BASE_URL_API_DEMO,
//!     http::{Client, Config, GetWalletBalanceParams},
//! };
//!
//! # async fn run() -> Result<(), bybit::Error> {
//! let client = Client::new(Config::new(BASE_URL_API_DEMO).credentials("API_KEY", "API_SECRET"))?;
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
//! The driver reconnects by itself, but subscriptions (and, on private
//! streams, authentication) must be sent again after every
//! [`ws::Event::Connected`]:
//!
//! ```no_run
//! use bybit::{
//!     BASE_URL_STREAM_MAINNET_1, Path, Topic,
//!     ws::{self, OutgoingMessage},
//! };
//!
//! # async fn run() -> Result<(), bybit::ws::Error> {
//! let url = format!("{BASE_URL_STREAM_MAINNET_1}{}", Path::PublicLinear);
//! let (handle, mut events) = ws::Stream::new(ws::Config::new(url));
//! handle.connect().await?;
//!
//! while let Some(event) = events.recv().await {
//!     match event {
//!         ws::Event::Connected => {
//!             let args = vec![Topic::Ticker(String::from("BTCUSDT"))];
//!             let sub = OutgoingMessage::Subscribe { req_id: None, args };
//!             handle.send_command(sub).await?;
//!         }
//!         ws::Event::Message(msg) => println!("{msg:?}"),
//!         ws::Event::Disconnected { .. } => break,
//!         _ => {}
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! For private streams build the `auth` message with
//! [`ws::create_outgoing_message_auth_at`] and
//! [`http::Client::server_timestamp`]. See the
//! [examples](https://github.com/yurii-musolov/patisson-bybit-sdk/tree/main/examples),
//! in particular `account-state` for [`AccountState`] and `stream-orderbook`
//! for [`OrderBookState`].
//!
//! # Versioning
//!
//! There are no stability guarantees yet: treat every version change as
//! breaking and pin an exact version (`patisson-bybit-sdk = "=x.y.z"`).

mod account_state;
mod common;
mod crypto;
mod enums;
mod error;
mod orderbook_state;
mod serde;
mod url;

pub mod http;
pub mod ws;

pub use account_state::*;
pub use common::*;
pub use crypto::*;
pub use enums::*;
pub use error::*;
pub use orderbook_state::*;
pub use url::{
    BASE_URL_API_DEMO, BASE_URL_API_MAINNET_1, BASE_URL_API_MAINNET_2, BASE_URL_API_MAINNET_3,
    BASE_URL_API_MAINNET_4, BASE_URL_API_MAINNET_5, BASE_URL_API_MAINNET_6, BASE_URL_API_TESTNET,
    BASE_URL_STREAM_DEMO, BASE_URL_STREAM_MAINNET_1, BASE_URL_STREAM_MAINNET_2,
    BASE_URL_STREAM_MAINNET_3, BASE_URL_STREAM_TESTNET, Path,
};
