//! Run with
//!
//! ```not_rust
//! cargo run --example get-api-key-information
//! ```

use tokio;
use tracing::{Level, debug};
use tracing_subscriber::FmtSubscriber;

use bybit::v5::{BASE_URL_API_DEMO, Client, ClientConfig, SensitiveString};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let api_key = SensitiveString::from(
        std::env::var("API_KEY").expect("environment variable API_KEY is required"),
    );
    let api_secret = SensitiveString::from(
        std::env::var("API_SECRET").expect("environment variable API_SECRET is required"),
    );

    let base_url = BASE_URL_API_DEMO; // or BASE_URL_API_MAINNET_1, BASE_URL_API_TESTNET

    let cfg = ClientConfig {
        base_url: base_url.to_owned(),
        api_key: Some(api_key),
        api_secret: Some(api_secret),
        recv_window: 5000, // Milliseconds.
        referer: None,
    };
    let client = Client::new(cfg);

    let response = client.get_api_key_information().await?;
    debug!(?response);

    Ok(())
}
