//! Run with
//!
//! ```not_rust
//! cargo run --example ticker
//! ```

use tokio;
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

use bybit::{
    BASE_URL_API_MAINNET_1, Category,
    http::{Client, Config, GetTickersParams},
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
    };
    let client = Client::new(cfg)?;
    let params = GetTickersParams {
        category: Category::Linear,
        symbol: Some(String::from("BTCUSDT")),
        base_coin: None, // If category=option, symbol or baseCoin must be passed.
        exp_date: None,
    };
    let response = client.get_tickers(params).await?;
    info!(?response);

    Ok(())
}
