//! Run with
//!
//! ```not_rust
//! cargo run --example stream-all-liquidation
//! ```

use tokio;

use bybit::v5::{
    BASE_URL_STREAM_MAINNET_1, DEFAULT_PING_INTERVAL, IncomingMessage, OutgoingMessage, Path,
    Topic, stream,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = format!("{}{}", BASE_URL_STREAM_MAINNET_1, Path::PublicLinear);
    let symbol = String::from("BTCUSDT");
    let topic = Topic::AllLiquidation(symbol).to_string();
    let message = OutgoingMessage::Subscribe {
        req_id: Some(String::from("req-0001")),
        args: vec![topic],
    };

    let (tx, mut rx, response) = stream(&url, DEFAULT_PING_INTERVAL).await?;
    println!("{response:#?}");

    tokio::spawn(async move {
        if let Err(err) = tx.send(message).await {
            println!("{err}");
        }
    });

    while let Some(message) = rx.recv().await {
        if let IncomingMessage::AllLiquidation(message) = message {
            println!("{message:#?}");
        }
    }

    Ok(())
}
