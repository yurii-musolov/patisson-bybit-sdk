//! Run with
//!
//! ```not_rust
//! cargo run --example recent-trading-history
//! ```

use tokio;

use bybit::v5::{BASE_URL_API_MAINNET_1, Category, Client, GetTradesParams};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new(BASE_URL_API_MAINNET_1);
    let params = GetTradesParams {
        category: Category::Linear,
        symbol: Some(String::from("BTCUSDT")),
        base_coin: None,
        option_type: None,
        limit: Some(2),
    };
    let response = client.get_public_recent_trading_history(params).await?;
    println!("{response:#?}");

    Ok(())
}
