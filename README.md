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
patisson-bybit-sdk = "=0.2.3"
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

## Examples

See [`examples/`](examples) for runnable programs.

### Get tickers

```rust
use bybit::{
    BASE_URL_API_MAINNET_1, Category,
    http::{Client, Config, GetTickersParams},
};

let cfg = Config::new(BASE_URL_API_MAINNET_1);
let client = Client::new(cfg)?;
let params = GetTickersParams {
    category: Category::Linear,
    symbol: Some(String::from("BTCUSDT")),
    base_coin: None, // If category=option, symbol or baseCoin must be passed.
    exp_date: None,
};
let response = client.get_tickers(&params).await?;
println!("{response:#?}");
```

### Private endpoints, timeouts and rate limiting

```rust
use std::time::Duration;

use bybit::{
    BASE_URL_API_DEMO,
    http::{Client, Config, RateLimiterConfig},
};

let cfg = Config::new(BASE_URL_API_DEMO)
    .credentials(api_key, api_secret)
    .recv_window(5_000) // Milliseconds.
    .timeout(Some(Duration::from_secs(10)))
    .rate_limiter(RateLimiterConfig::bybit_base_tier_defaults());
let client = Client::new(cfg)?;
```

Calling a private endpoint on a client without credentials returns
`Error::MissingCredentials`.

### Subscribe to public channel 'ticker'

```rust
use bybit::{
    BASE_URL_STREAM_MAINNET_1, Path, Topic,
    ws::{self, OutgoingMessage},
};

let url = format!("{}{}", BASE_URL_STREAM_MAINNET_1, Path::PublicLinear);
let args = vec![Topic::Ticker(String::from("BTCUSDT"))];

let (handle, mut events) = ws::Stream::new(ws::Config::new(url));
handle.connect().await?;

while let Some(event) = events.recv().await {
    match event {
        // Sent after the first connection and after every reconnect:
        // subscriptions are not restored automatically.
        ws::Event::Connected => {
            let sub = OutgoingMessage::Subscribe {
                req_id: None,
                args: args.clone(),
            };
            handle.send_command(sub).await?;
        }
        ws::Event::Message(msg) => println!("{msg:?}"),
        // Data events were dropped because the event queue was full.
        ws::Event::Lagged { dropped } => eprintln!("lagged: {dropped} events dropped"),
        ws::Event::Disconnected { .. } => break,
        _ => {}
    }
}
```

### Subscribe to private channels

```rust
use bybit::{
    BASE_URL_STREAM_DEMO, Path, SensitiveString, Topic,
    ws::{self, OutgoingMessage, create_outgoing_message_auth},
};

let api_key = SensitiveString::from("XXXXXXXX");
let api_secret = SensitiveString::from("XXXXXXXXXXXXXXXX");

let url = format!("{}{}", BASE_URL_STREAM_DEMO, Path::Private);
let args = vec![
    Topic::ExecutionAllCategory,
    Topic::OrderAllCategory,
    Topic::PositionAllCategory,
    Topic::Wallet,
];

let (handle, mut events) = ws::Stream::new(ws::Config::new(url));
handle.connect().await?;

while let Some(event) = events.recv().await {
    match event {
        // Authenticate and subscribe again after every (re)connect.
        ws::Event::Connected => {
            let auth = create_outgoing_message_auth(
                api_key.clone(),
                api_secret.clone(),
                None,
                5_000, // Milliseconds.
            );
            let sub = OutgoingMessage::Subscribe {
                req_id: None,
                args: args.clone(),
            };
            handle.send_command(auth).await?;
            handle.send_command(sub).await?;
        }
        ws::Event::Message(msg) => println!("{msg:?}"),
        ws::Event::Disconnected { .. } => break,
        _ => {}
    }
}
```

## License

This project is licensed under the [MIT license](LICENSE).
