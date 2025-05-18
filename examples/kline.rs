//! Run with
//!
//! ```not_rust
//! cargo run --example kline
//! ```

use tokio;

use bybit::v5::{BASE_URL_API_MAINNET_1, Category, Client, GetKLinesParams, Interval};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new(BASE_URL_API_MAINNET_1);
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
