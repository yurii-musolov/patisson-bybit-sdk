//! Run with
//!
//! ```not_rust
//! cargo run --example kline
//! ```

use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

use bybit::{
    Category, Environment, Interval,
    http::{Client, Config, GetKLinesParams},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let env = Environment::Mainnet;

    let cfg = Config::for_env(env);
    let client = Client::new(cfg)?;
    let params = GetKLinesParams::new(Category::Linear, "BTCUSDT", Interval::Minute1).with_limit(2);
    let response = client.get_kline(&params).await?;
    info!(?response);

    Ok(())
}
