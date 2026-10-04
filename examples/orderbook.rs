//! Run with
//!
//! ```not_rust
//! cargo run --example orderbook
//! ```

use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

use bybit::{
    BASE_URL_API_MAINNET_1, Category,
    http::{Client, Config, GetOrderbookParams},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let base_url = BASE_URL_API_MAINNET_1;

    let cfg = Config::new(base_url);
    let client = Client::new(cfg)?;
    let params = GetOrderbookParams::new(Category::Linear, "BTCUSDT").with_limit(1);
    let response = client.get_orderbook(&params).await?;
    info!(?response);

    Ok(())
}
