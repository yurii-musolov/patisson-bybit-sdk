//! Run with
//!
//! ```not_rust
//! cargo run --example wallet
//! ```

use tokio;
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

use bybit::{
    AccountType, BASE_URL_API_DEMO,
    http::{Client, Config, GetWalletBalanceParams},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let api_key = std::env::var("API_KEY").expect("environment variable API_KEY is required");
    let api_secret =
        std::env::var("API_SECRET").expect("environment variable API_SECRET is required");

    let base_url = BASE_URL_API_DEMO;

    let cfg = Config::new(base_url).credentials(api_key, api_secret);
    let client = Client::new(cfg)?;

    let params = GetWalletBalanceParams {
        account_type: AccountType::UNIFIED,
        coin: None,
    };

    let response = client.get_wallet_balance(&params).await?;
    info!(?response);

    Ok(())
}
