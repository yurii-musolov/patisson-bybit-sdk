//! Run with
//!
//! ```not_rust
//! cargo run --example ticker
//! ```

use tokio;

use bybit::v5::{BASE_URL_API_MAINNET_1, Category, Client, ClientConfig, GetTickersParams};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

    Ok(())
}
