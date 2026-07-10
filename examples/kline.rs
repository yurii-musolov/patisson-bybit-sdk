//! Run with
//!
//! ```not_rust
//! cargo run --example kline
//! ```

use tokio;
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

    let cfg = Config {
        base_url: base_url.to_owned(),
        api_key: None,
        api_secret: None,
        recv_window: 5000, // Milliseconds.
        referer: None,
        rate_limiter: None,
    };
    let client = Client::new(cfg)?;
    let params = GetKLinesParams {
        category: Category::Linear,
        symbol: String::from("BTCUSDT"),
        interval: Interval::Minute1,
        start: None,
        end: None,
        limit: Some(2),
    };
    let response = client.get_kline(&params).await?;
    info!(?response);

    Ok(())
}
