//! Run with
//!
//! ```not_rust
//! cargo run --example get-open-orders API_KEY API_SECRET
//! ```

use tokio;

use bybit::v5::{
    BASE_URL_API_DEMO_TRADING, Category, Client, ClientConfig, GetOpenClosedOrdersParams,
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

    let params = GetOpenClosedOrdersParams {
        category: Category::Linear,
        symbol: Some(String::from("BTCUSDT")), // For linear is required.
        base_coin: None,
        settle_coin: None,
        order_id: None,
        order_link_id: None,
        open_only: None,
        order_filter: None,
        limit: Some(10),
        cursor: None,
    };
    let response = client.get_open_closed_orders(params).await?;
    println!("{response:#?}");

    Ok(())
}
