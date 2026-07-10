//! Run with
//!
//! ```not_rust
//! cargo run --example get-transaction-log
//! ```

use rust_decimal::Decimal;
use tokio;
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

use bybit::{
    BASE_URL_API_DEMO, Category, Timestamp,
    http::{Client, Config, GetTransactionLogParams},
    timestamp,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let api_key = std::env::var("API_KEY")
        .expect("environment variable API_KEY is required")
        .into();
    let api_secret = std::env::var("API_SECRET")
        .expect("environment variable API_SECRET is required")
        .into();

    let base_url = BASE_URL_API_DEMO;

    let cfg = Config {
        base_url: base_url.to_owned(),
        api_key: Some(api_key),
        api_secret: Some(api_secret),
        recv_window: 5_000, // Milliseconds.
        referer: None,
        rate_limiter: None,
    };
    let client = Client::new(cfg)?;

    let settle_coin = String::from("USDT");
    let day = 24 * 60 * 60 * 1000;
    let now = timestamp();
    // Beginning of the current day.
    let start_time = now - (now % day);
    // Last 24 hours.
    // let start_time = now - day;

    let changes = fetch_changes(&client, settle_coin, start_time).await?;

    let count = changes.len();
    info!(?count, "number of transactions");
    let daily_change = changes.iter().fold(Decimal::ZERO, |acc, cur| acc + cur);
    info!(?daily_change, "daily change");

    Ok(())
}

async fn fetch_changes(
    client: &Client,
    settle_coin: String,
    start_time: Timestamp,
) -> anyhow::Result<Vec<Decimal>> {
    let mut result = Vec::new();
    let mut cursor = None;

    loop {
        let params = GetTransactionLogParams {
            account_type: None,
            category: Some(Category::Linear),
            currency: None,
            base_coin: None,
            settle_coin: Some(settle_coin.clone()),
            log_type: Some(String::from("TRADE")),
            trans_sub_type: None,
            start_time: Some(start_time),
            end_time: None,
            limit: Some(50),
            cursor: cursor.take(),
        };
        let response = client.get_transaction_log(&params).await?;

        let changes: Vec<_> = response.result.list.iter().map(|t| t.change).collect();
        result.push(changes);

        cursor = response.result.next_page_cursor;

        if cursor.is_none() {
            break;
        }
    }

    Ok(result.concat())
}
