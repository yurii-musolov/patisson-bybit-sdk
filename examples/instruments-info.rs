//! Run with
//!
//! ```not_rust
//! cargo run --example instruments-info
//! ```

use tokio;

use bybit::v5::{BASE_URL_API_MAINNET_1, Category, Client, GetInstrumentsInfoParams};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new(BASE_URL_API_MAINNET_1);
    let params = GetInstrumentsInfoParams {
        category: Category::Linear,
        symbol: Some(String::from("BTCUSDT")),
        status: None,
        base_coin: None,
        limit: None,
        cursor: None,
    };
    let response = client.get_instruments_info(params).await?;
    println!("{response:#?}");

    Ok(())
}
