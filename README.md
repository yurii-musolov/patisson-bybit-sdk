# bybit-sdk

Unofficial Rust SDK for the [Bybit exchange API](https://bybit-exchange.github.io/docs/v5/intro).

## Example

```rust
let url = String::from("https://api.bybit.com");
let client = bybit::v5::Client::new(url);

let response = client.get_tickers()?;
println!("{response}");
```
