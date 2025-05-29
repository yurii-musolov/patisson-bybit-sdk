//! Run with
//!
//! ```not_rust
//! cargo run --example kline
//! ```

use tokio;

use bybit::v5::{
    BASE_URL_API_MAINNET_1, Category, Client, ClientConfig, GetKLinesParams, Interval,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = ClientConfig {
        base_url: BASE_URL_API_MAINNET_1.to_string(),
        api_key: String::from("API-KEY-XXXX"),
        api_secret: String::from("API-SECRET-XXXX"),
        recv_window: 5000, // Milliseconds.
    };
    let client = Client::new(cfg);
    let params = GetKLinesParams {
        category: Category::Linear,
        symbol: String::from("BTCUSDT"),
        interval: Interval::Minute1,
        start: None,
        end: None,
        limit: Some(2),
    };
    let response = client.get_kline(params).await?;
    println!("{response:#?}");

    Ok(())
}
