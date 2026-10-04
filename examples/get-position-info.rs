//! Run with
//!
//! ```not_rust
//! cargo run --example get-position-info
//! ```

use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

use bybit::{
    BASE_URL_API_DEMO, Category,
    http::{Client, Config, GetPositionInfoParams},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let api_key = std::env::var("API_KEY").expect("environment variable API_KEY is required");
    let api_secret =
        std::env::var("API_SECRET").expect("environment variable API_SECRET is required");

    let base_url = BASE_URL_API_DEMO; // or BASE_URL_API_MAINNET_1, BASE_URL_API_TESTNET

    let cfg = Config::new(base_url).credentials(api_key, api_secret);
    let client = Client::new(cfg)?;

    let params = GetPositionInfoParams {
        category: Category::Linear,
        symbol: None,
        base_coin: None,
        settle_coin: Some(String::from("USDT")),
        limit: Some(10),
        cursor: None,
    };
    let response = client.get_position_info(&params).await?;
    info!(?response);

    Ok(())
}
