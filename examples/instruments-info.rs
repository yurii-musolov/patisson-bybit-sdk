//! Run with
//!
//! ```not_rust
//! cargo run --example instruments-info
//! ```

use tokio;

use bybit::v5::{BASE_URL_API_MAINNET_1, Category, Client, ClientConfig, GetInstrumentsInfoParams};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = ClientConfig {
        base_url: BASE_URL_API_MAINNET_1.to_string(),
        api_key: None,
        api_secret: None,
        recv_window: 5000,
        referer: None,
    };
    let client = Client::new(cfg);
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
