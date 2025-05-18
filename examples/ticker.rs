//! Run with
//!
//! ```not_rust
//! cargo run --example ticker
//! ```

use tokio;

use bybit::v5::{BASE_URL_API_MAINNET_1, Category, Client, GetTickersParams};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new(BASE_URL_API_MAINNET_1);
    let params = GetTickersParams {
        category: Category::Linear,
        symbol: Some(String::from("BTCUSDT")),
        base_coin: None,
        exp_date: None,
    };
    let response = client.get_tickers(params).await?;
    println!("{response:#?}");

    Ok(())
}
