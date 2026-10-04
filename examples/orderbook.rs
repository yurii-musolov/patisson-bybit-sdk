//! Run with
//!
//! ```not_rust
//! cargo run --example orderbook
//! ```

use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

use bybit::{
    Category, Environment,
    http::{Client, Config, GetOrderbookParams},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let env = Environment::Mainnet;

    let cfg = Config::for_env(env);
    let client = Client::new(cfg)?;
    let params = GetOrderbookParams::new(Category::Linear, "BTCUSDT").with_limit(1);
    let response = client.get_orderbook(&params).await?;
    info!(?response);

    Ok(())
}
