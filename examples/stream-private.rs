//! Run with
//!
//! ```not_rust
//! cargo run --example stream-private
//! ```

use tokio;
use tracing::{Level, debug};
use tracing_subscriber::FmtSubscriber;

use Topic::{ExecutionAllCategory, OrderAllCategory, PositionAllCategory, Wallet};
use bybit::v5::{
    BASE_URL_STREAM_MAINNET_1, DEFAULT_PING_INTERVAL, OutgoingMessage, Path, Topic,
    create_outgoing_message_auth, stream,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let api_key = std::env::var("API_KEY")
        .expect("environment variable API_KEY is required")
        .into();
    let api_secret = std::env::var("API_SECRET")
        .expect("environment variable API_SECRET is required")
        .into();

    let url = format!("{}{}", BASE_URL_STREAM_MAINNET_1, Path::Private);
    let messages = vec![
        create_outgoing_message_auth(api_key, api_secret, Some(String::from("req-0001"))),
        OutgoingMessage::Subscribe {
            req_id: Some(String::from("req-0002")),
            args: vec![
                ExecutionAllCategory,
                OrderAllCategory,
                PositionAllCategory,
                Wallet,
            ],
        },
    ];

    let (tx, mut rx, response) = stream(&url, DEFAULT_PING_INTERVAL).await?;
    debug!(?response);

    tokio::spawn(async move {
        for message in messages {
            let _ = tx.send(message).await;
        }
    });

    while let Some(message) = rx.recv().await {
        debug!(?message);
    }

    Ok(())
}
