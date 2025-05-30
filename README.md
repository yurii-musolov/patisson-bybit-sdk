# Bybit SDK

Unofficial Rust SDK for the [Bybit exchange API](https://bybit-exchange.github.io/docs/v5/intro).

## Features

- REST API support (Spot, Futures, Derivatives)
- Unauthenticated endpoints
- Only async clients

## Examples

### Get tickers

```rust
use bybit::v5::{BASE_URL_API_MAINNET_1, Category, Client, GetTickersParams};

let cfg = ClientConfig {
    base_url: BASE_URL_API_MAINNET_1.to_string(),
    api_key: String::default(),
    api_secret: String::default(),
    recv_window: 5000, // Milliseconds.
};
let client = Client::new(cfg);
let params = GetTickersParams {
    category: Category::Linear,
    symbol: Some(String::from("BTCUSDT")),
    base_coin: None,
    exp_date: None,
};
let response = client.get_tickers(params).await?;
println!("{response:#?}");
```

### Subscribe to ticker

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
        println!("{err}");
    }
});

while let Some(message) = rx.recv().await {
    println!("{message:#?}");
}
```

## License

This project is licensed under the [MIT license](LICENSE).
