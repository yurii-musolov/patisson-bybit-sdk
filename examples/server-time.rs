//! Run with
//!
//! ```not_rust
//! cargo run --example server-time
//! ```

use tokio;

use bybit::v5::{BASE_URL_API_MAINNET_1, Client, ClientConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = ClientConfig {
        base_url: BASE_URL_API_MAINNET_1.to_string(),
        api_key: String::from("API-KEY-XXXX"),
        api_secret: String::from("API-SECRET-XXXX"),
        recv_window: 5000, // Milliseconds.
    };
    let client = Client::new(cfg);
    let response = client.get_server_time().await?;
    println!("{response:#?}");

    Ok(())
}
