# Bybit SDK

[![Crates.io](https://img.shields.io/crates/v/patisson-bybit-sdk.svg)](https://crates.io/crates/patisson-bybit-sdk)
[![Documentation](https://docs.rs/patisson-bybit-sdk/badge.svg)](https://docs.rs/patisson-bybit-sdk)
[![MIT licensed](https://img.shields.io/crates/l/patisson-bybit-sdk.svg)](LICENSE)

Unofficial Rust SDK for the [Bybit exchange API](https://bybit-exchange.github.io/docs/v5/intro).

## Disclaimer

### Stability & Versioning Policy

No stability or backward-compatibility guarantees are provided.

Every version change must be treated as a breaking change, including minor and patch releases.

Users are strongly advised to pin an exact version, for example:

```rs
patisson-bybit-sdk = "=0.2.6"
```

### Maintenance Policy

This package is developed only in the author's free time.

Releases are best-effort and not planned in advance.

The scope of the package is intentionally limited to the most commonly used functionality.

## Features

- REST API support
- Websocket API support
- Only async clients
- All categories: Spot, Linear, Inverse, Option

## TLS

REST and WebSocket share one TLS backend, selected with a Cargo feature
(enable exactly one):

- `rustls` (default): rustls with aws-lc-rs and the operating system's root
  certificates; no OpenSSL needed.
- `native-tls`: the platform TLS library (OpenSSL on Linux, Schannel on
  Windows, Security.framework on macOS).

```toml
patisson-bybit-sdk = { version = "=0.3.0", default-features = false, features = ["native-tls"] }
```

## Examples

See [`examples/`](examples) for runnable programs.

### Get tickers

```rust
use bybit::{
    Category, Environment,
    http::{Client, Config, GetTickersParams},
};

let cfg = Config::for_env(Environment::Mainnet);
let client = Client::new(cfg)?;
// If category=option, symbol or baseCoin must be passed.
let params = GetTickersParams::new(Category::Linear).with_symbol("BTCUSDT");
let response = client.get_tickers(&params).await?;
println!("{response:#?}");
```

### Private endpoints, timeouts and rate limiting

```rust
use std::time::Duration;

use bybit::{
    Environment,
    http::{Client, Config, RateLimiterConfig},
};

let cfg = Config::for_env(Environment::Demo)
    .credentials(api_key, api_secret)
    .recv_window(5_000) // Milliseconds.
    .timeout(Some(Duration::from_secs(10)))
    .rate_limiter(RateLimiterConfig::bybit_base_tier_defaults());
let client = Client::new(cfg)?;
```

Calling a private endpoint on a client without credentials returns
`Error::MissingCredentials`.

REST requests honour the `HTTP(S)_PROXY` environment variables; set a proxy
explicitly with `Config::proxy("http://127.0.0.1:8080")`.

### Clock synchronization

Bybit rejects signed requests whose timestamp is more than `recv_window` behind
(or 1 s ahead of) its own clock (`retCode` 10002). `Client::sync_time` measures
the offset to the server clock and applies it to every signed request; call it
after creating the client and periodically in long-running programs. When a
signed request is rejected with 10002 anyway, the client re-synchronizes and
retries it once (`Config::resync_time_on_timestamp_error`, enabled by default).
`Client::clock()` shares the offset with WebSocket streams (see below).

`Error` has helpers for common decisions: `api_code()` (compare with the
constants in `bybit::ret_code`), `is_retryable()`, `is_rate_limited()`,
`is_timestamp_error()` and `is_not_modified()`.

```rust
let offset_ms = client.sync_time().await?;
```

### Subscribe to public channel 'ticker'

Topics subscribed with `Handle::subscribe` are restored automatically after
every reconnect.

```rust
use bybit::{Category, Environment, Topic, ws};

let config = ws::Config::public(Environment::Mainnet, Category::Linear);
let (handle, mut events) = ws::Stream::new(config);
handle.subscribe(vec![Topic::Ticker(String::from("BTCUSDT"))]).await?;
handle.connect().await?;

while let Some(event) = events.recv().await {
    match event {
        ws::Event::Message(msg) => println!("{msg:?}"),
        // Data events were dropped because the event queue was full.
        ws::Event::Lagged { dropped } => eprintln!("lagged: {dropped} events dropped"),
        ws::Event::SubscribeFailed { topics, ret_msg } => eprintln!("{topics:?}: {ret_msg:?}"),
        ws::Event::Disconnected { .. } => break,
        _ => {}
    }
}
```

### Subscribe to private channels

With credentials the driver sends `auth` on every (re)connect and subscribes
once it succeeded. Pass the REST client's clock so that `auth` uses the server
time measured by `sync_time`.

```rust
use bybit::{Environment, Topic, ws};

let config = ws::Config::private(Environment::Demo)
    .credentials(api_key, api_secret)
    .server_clock(client.clock());
let (handle, mut events) = ws::Stream::new(config);
handle
    .subscribe(vec![
        Topic::ExecutionAllCategory,
        Topic::OrderAllCategory,
        Topic::PositionAllCategory,
        Topic::Wallet,
    ])
    .await?;
handle.connect().await?;

while let Some(event) = events.recv().await {
    match event {
        ws::Event::Authenticated => println!("authenticated"),
        ws::Event::AuthFailed { ret_msg } => {
            eprintln!("auth rejected: {ret_msg:?}");
            break;
        }
        ws::Event::Message(msg) => println!("{msg:?}"),
        ws::Event::Disconnected { .. } => break,
        _ => {}
    }
}
```

### Local account state

`AccountState` keeps open orders, positions and wallet balances from REST
snapshots and the private stream, ignoring stale updates. Reload the snapshots
after every authentication and after `Event::Lagged`:

```rust
use bybit::{AccountScope, AccountState, AccountType, ws};

let scope = AccountScope::new().linear("USDT").wallet(AccountType::UNIFIED);
let mut state = AccountState::new();

while let Some(event) = events.recv().await {
    match event {
        ws::Event::Authenticated | ws::Event::Lagged { .. } => {
            state.reload(&client, &scope).await?;
        }
        ws::Event::Message(ws::IncomingMessage::Topic(msg)) => {
            for change in state.apply(msg) {
                println!("{change:?}");
            }
        }
        ws::Event::Disconnected { .. } => break,
        _ => {}
    }
}
```

See `examples/account-state.rs` for a complete program.

## License

This project is licensed under the [MIT license](LICENSE).
