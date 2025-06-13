//! Run with
//!
//! ```not_rust
//! cargo run --example get-position-info API_KEY API_SECRET
//! ```

use tokio;

use bybit::v5::{
    BASE_URL_API_DEMO_TRADING, Category, Client, ClientConfig, GetPositionInfoParams,
    SensitiveString,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    let api_key = args.get(1).expect("API_KEY is required");
    let api_secret = args.get(2).expect("API_SECRET is required");

    let base_url = BASE_URL_API_DEMO_TRADING; // or BASE_URL_API_MAINNET_1, BASE_URL_API_TESTNET

    let cfg = ClientConfig {
        base_url: base_url.to_owned(),
        api_key: Some(SensitiveString::from(api_key.to_owned())),
        api_secret: Some(SensitiveString::from(api_secret.to_owned())),
        recv_window: 5000, // Milliseconds.
        referer: None,
    };
    let client = Client::new(cfg);

    let params = GetPositionInfoParams {
        category: Category::Linear,
        symbol: Some(String::from("BTCUSDT")),
        base_coin: None,
        settle_coin: None,
        limit: Some(10),
        cursor: None,
    };
    let response = client.get_position_info(params).await?;
    println!("{response:#?}");

    Ok(())
}
