//! Run with
//!
//! ```not_rust
//! cargo run --example ticker
//! ```

use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

use bybit::{
    Category, Environment,
    http::{Client, Config, GetTickersParams},
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
    let params = GetTickersParams::new(Category::Linear).with_symbol("BTCUSDT");
    let response = client.get_tickers(&params).await?;
    info!(?response);

    Ok(())
}
