//! Run with
//!
//! ```not_rust
//! cargo run --example server-time
//! ```

use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

use bybit::{
    Environment,
    http::{Client, Config},
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
    let response = client.get_server_time().await?;
    info!(?response);

    Ok(())
}
