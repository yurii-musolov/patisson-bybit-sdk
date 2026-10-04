//! Run with
//!
//! ```not_rust
//! cargo run --example kline
//! ```

use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

use bybit::{
    BASE_URL_API_MAINNET_1, Category, Interval,
    http::{Client, Config, GetKLinesParams},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let base_url = BASE_URL_API_MAINNET_1;

    let cfg = Config::new(base_url);
    let client = Client::new(cfg)?;
    let params = GetKLinesParams::new(Category::Linear, "BTCUSDT", Interval::Minute1).with_limit(2);
    let response = client.get_kline(&params).await?;
    info!(?response);

    Ok(())
}
