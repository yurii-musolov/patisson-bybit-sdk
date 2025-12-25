# Bybit SDK

[![Crates.io](https://img.shields.io/crates/v/patisson-bybit-sdk.svg)](https://crates.io/crates/patisson-bybit-sdk)
[![Documentation](https://docs.rs/patisson-bybit-sdk/badge.svg)](https://docs.rs/patisson-bybit-sdk)
[![MIT licensed](https://img.shields.io/crates/l/patisson-bybit-sdk.svg)](LICENSE)

Unofficial Rust SDK for the [Bybit exchange API](https://bybit-exchange.github.io/docs/v5/intro).

## Features

- REST API support
- Websocket API support
- Only async clients
- All categories: Spot, Linear, Inverse, Option

## Examples

### Get tickers

```rust
use bybit::v5::{BASE_URL_API_MAINNET_1, Category, Client, GetTickersParams};

let cfg = ClientConfig {
    base_url: BASE_URL_API_MAINNET_1.to_string(),
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
use bybit::v5::{
    BASE_URL_STREAM_MAINNET_1, DEFAULT_PING_INTERVAL, IncomingMessage, OutgoingMessage, Path,
    Topic, stream,
};

let url = format!("{}{}", BASE_URL_STREAM_MAINNET_1, Path::PublicLinear);
let symbol = String::from("BTCUSDT");
let topic = Topic::Ticker(symbol).to_string();
let message = OutgoingMessage::Subscribe {
    req_id: Some(String::from("req-0001")),
    args: vec![topic],
};

let (tx, mut rx, response) = stream(&url, DEFAULT_PING_INTERVAL).await?;
println!("{response:#?}");

tokio::spawn(async move {
    if let Err(err) = tx.send(message).await {
        eprintln!("{err}");
    }
});

while let Some(message) = rx.recv().await {
    println!("{message:#?}");
}
```

### Subscribe to private channels

```rust
use Topic::{ExecutionAllCategory, OrderAllCategory, PositionAllCategory, Wallet};
use bybit::v5::{
    BASE_URL_STREAM_MAINNET_1, DEFAULT_PING_INTERVAL, OutgoingMessage, Path, SensitiveString,
    Topic, create_outgoing_message_auth, stream,
};

let api_key = SensitiveString::from("XXXXXXXX");
let api_secret = SensitiveString::from("XXXXXXXXXXXXXXXX");

let url = format!("{}{}", BASE_URL_STREAM_MAINNET_1, Path::Private);
let messages = vec![
    create_outgoing_message_auth(api_key, api_secret, Some(String::from("req-0001"))),
    OutgoingMessage::Subscribe {
        req_id: Some(String::from("req-0002")),
        args: vec![
            ExecutionAllCategory,
            OrderAllCategory,
            PositionAllCategory,
            Wallet,
        ],
    },
];

let (tx, mut rx, response) = stream(&url, DEFAULT_PING_INTERVAL).await?;
println!("{response:#?}");

tokio::spawn(async move {
    for message in messages {
        if let Err(err) = tx.send(message).await {
            eprintln!("{err}");
        }
    }
});

while let Some(message) = rx.recv().await {
    println!("{message:#?}");
}
```

## License

This project is licensed under the [MIT license](LICENSE).
