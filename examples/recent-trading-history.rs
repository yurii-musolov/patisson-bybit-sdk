//! Run with
//!
//! ```not_rust
//! cargo run --example recent-trading-history
//! ```

use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

use bybit::{
    Category, Environment,
    http::{Client, Config, GetTradesParams},
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
    let params = GetTradesParams::new(Category::Linear)
        .with_symbol("BTCUSDT")
        .with_limit(2);
    let response = client.get_public_recent_trading_history(&params).await?;
    info!(?response);

    Ok(())
}
