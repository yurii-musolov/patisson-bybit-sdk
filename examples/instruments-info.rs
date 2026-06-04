//! Run with
//!
//! ```not_rust
//! cargo run --example instruments-info
//! ```

use tokio;
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

use bybit::{
    BASE_URL_API_MAINNET_1, Category,
    http::{Client, Config, GetInstrumentsInfoParams},
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
        recv_window: 5000,
        referer: None,
    };
    let client = Client::new(cfg)?;
    let params = GetInstrumentsInfoParams {
        category: Category::Linear,
        symbol: Some(String::from("BTCUSDT")),
        status: None,
        base_coin: None,
        limit: None,
        cursor: None,
    };
    let response = client.get_instruments_info(params).await?;
    info!(?response);

    Ok(())
}
