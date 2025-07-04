//! Run with
//!
//! ```not_rust
//! cargo run --example wallet
//! ```

use tokio;

use bybit::v5::{
    BASE_URL_API_MAINNET_1, Client, ClientConfig, GetWalletBalanceParams, SensitiveString,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_key = std::env::var("API_KEY").expect("environment variable API_KEY is required");
    let api_secret =
        std::env::var("API_SECRET").expect("environment variable API_SECRET is required");

    let base_url = BASE_URL_API_MAINNET_1;

    let cfg = ClientConfig {
        base_url: base_url.to_owned(),
        api_key: Some(SensitiveString::from(api_key.to_owned())),
        api_secret: Some(SensitiveString::from(api_secret.to_owned())),
        recv_window: 5000, // Milliseconds.
        referer: None,
    };
    let client = Client::new(cfg);

    let params = GetWalletBalanceParams {
        account_type: String::from("UNIFIED"),
        coin: None,
    };

    let response = client.get_wallet_balance(params).await?;
    println!("{response:#?}");

    Ok(())
}
