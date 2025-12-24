//! Run with
//!
//! ```not_rust
//! cargo run --example server-time
//! ```

use tokio;
use tracing::{Level, debug};
use tracing_subscriber::FmtSubscriber;

use bybit::v5::{BASE_URL_API_MAINNET_1, Client, ClientConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let cfg = ClientConfig {
        base_url: BASE_URL_API_MAINNET_1.to_string(),
        api_key: None,
        api_secret: None,
        recv_window: 5000, // Milliseconds.
        referer: None,
    };
    let client = Client::new(cfg);
    let response = client.get_server_time().await?;
    debug!(?response);

    Ok(())
}
