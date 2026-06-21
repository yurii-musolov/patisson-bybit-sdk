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

This package is developed only in the author’s free time.

Releases are best-effort and not planned in advance.

The scope of the package is intentionally limited to the most commonly used functionality.

## Features

- REST API support
- Websocket API support
- Only async clients
- All categories: Spot, Linear, Inverse, Option

## Examples

### Get tickers

```rust
use bybit::v5::{ BASE_URL_API_MAINNET_1, Category, http::{ Client, Config, GetTickersParams } };

let cfg = Config {
    base_url: BASE_URL_API_MAINNET_1.to_owned(),
    api_key: None,
    api_secret: None,
    recv_window: 5000, // Milliseconds.
    referer: None,
};
let client = Client::new(cfg);
let params = GetTickersParams {
    category: Category::Linear,
    symbol: Some(String::from("BTCUSDT")),
    base_coin: None, // If category=option, symbol or baseCoin must be passed.
    exp_date: None,
};
let response = client.get_tickers(params).await?;
println!("{response:#?}");
```

### Subscribe to public channel 'ticker'

```rust
use bybit::{
    BASE_URL_STREAM_MAINNET_1, Interval, OutgoingMessage, Path, Topic, ws,
};

let url = format!("{}{}", BASE_URL_STREAM_MAINNET_1, Path::PublicLinear);
let args = vec![Topic::Ticker(String::from("BTCUSDT"))];

let sub = OutgoingMessage::Subscribe {
    req_id: Some(String::from("req-0001")),
    args: args.clone(),
};
let unsub = OutgoingMessage::Unsubscribe {
    req_id: Some(String::from("req-0002")),
    args,
};

let cfg = ws::Config::new(url);
let (handle, mut events) = ws::Stream::new(cfg);

tokio::spawn(async move {
    let _ = handle.connect().await?;
    let _ = handle.send_command(sub).await?;
    sleep(Duration::from_secs(20)).await?;
    let _ = handle.send_command(unsub).await?;
    sleep(Duration::from_secs(2)).await?;
    let _ = handle.disconnect().await?;
});

while let Some(event) = events.recv().await {
    info!(?event);
    if matches!(event, ws::Event::Disconnected { reason: _ }) {
        break;
    }
}
```

### Subscribe to private channels

```rust
use bybit::{
    BASE_URL_STREAM_DEMO, Path, Topic, ws::{ create_outgoing_message_auth, OutgoingMessage },
};

use Topic::{ExecutionAllCategory, OrderAllCategory, PositionAllCategory, Wallet};

let api_key = SensitiveString::from("XXXXXXXX");
let api_secret = SensitiveString::from("XXXXXXXXXXXXXXXX");

let url = format!("{}{}", BASE_URL_STREAM_DEMO, Path::Private);
let args = vec![
    ExecutionAllCategory,
    OrderAllCategory,
    PositionAllCategory,
    Wallet,
];
let auth =
    create_outgoing_message_auth(api_key, api_secret, Some(String::from("req-0001")), 5_000);
let sub = OutgoingMessage::Subscribe {
    req_id: Some(String::from("req-0002")),
    args: args.clone(),
};
let unsub = OutgoingMessage::Unsubscribe {
    req_id: Some(String::from("req-0003")),
    args,
};

let cfg = ws::Config::new(url);
let (handle, mut events) = ws::spawn(cfg);

tokio::spawn(async move {
    let _ = handle.connect().await;
    let _ = handle.send_command(auth).await;
    let _ = handle.send_command(sub).await;
    sleep(Duration::from_hours(24)).await;
    let _ = handle.send_command(unsub).await;
    sleep(Duration::from_secs(2)).await;
    let _ = handle.disconnect().await;
});

while let Some(event) = events.recv().await {
    info!(?event);
    if matches!(event, ws::Event::Disconnected { reason: _ }) {
        break;
    }
}
```

## License

This project is licensed under the [MIT license](LICENSE).
