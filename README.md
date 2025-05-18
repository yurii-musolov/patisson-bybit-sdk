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

let client = Client::new(BASE_URL_API_MAINNET_1);
let params = GetTickersParams {
    category: Category::Linear,
    symbol: Some(String::from("BTCUSDT")),
    base_coin: None,
    exp_date: None,
};
let response = client.get_tickers(params).await?;
println!("{response:#?}");
```

## License

This project is licensed under the [MIT license](LICENSE).
