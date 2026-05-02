//! Run with
//!
//! ```not_rust
//! cargo run --example get-open-orders
//! ```

use tokio;
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

use bybit::v5::{BASE_URL_API_DEMO, Category, Client, ClientConfig, GetOpenClosedOrdersParams};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let api_key = std::env::var("API_KEY")
        .expect("environment variable API_KEY is required")
        .into();
    let api_secret = std::env::var("API_SECRET")
        .expect("environment variable API_SECRET is required")
        .into();

    let base_url = BASE_URL_API_DEMO; // or BASE_URL_API_MAINNET_1, BASE_URL_API_TESTNET

    let cfg = ClientConfig {
        base_url: base_url.to_owned(),
        api_key: Some(api_key),
        api_secret: Some(api_secret),
        recv_window: 5000, // Milliseconds.
        referer: None,
    };
    let client = Client::new(cfg)?;

    let params = GetOpenClosedOrdersParams {
        category: Category::Linear,
        symbol: None,
        base_coin: None,
        settle_coin: Some(String::from("USDT")),
        order_id: None,
        order_link_id: None,
        open_only: None,
        order_filter: None,
        limit: Some(10),
        cursor: None,
    };
    let response = client.get_open_closed_orders(params).await?;
    info!(?response);

    Ok(())
}
