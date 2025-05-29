//! Run with
//!
//! ```not_rust
//! cargo run --example stream-public
//! ```

use std::time::Duration;

use tokio::{self, time::sleep};

use bybit::v5::{BASE_URL_STREAM_MAINNET_1, Interval, OutgoingMessage, Path, Topic, stream};

const WEBSOCKET_PING_INTERVAL: Duration = Duration::from_secs(10);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = format!("{}{}", BASE_URL_STREAM_MAINNET_1, Path::PublicLinear);
    let symbol = String::from("BTCUSDT");
    let messages = {
        let ticker = Topic::Ticker(symbol.clone()).to_string();
        let trade = Topic::Trade(symbol.clone()).to_string();
        let kline = Topic::Kline {
            symbol: symbol.clone(),
            interval: Interval::Minute1,
        }
        .to_string();

        vec![
            OutgoingMessage::Subscribe {
                req_id: Some(String::from("req-0001")),
                args: vec![ticker.clone()],
            },
            OutgoingMessage::Unsubscribe {
                req_id: Some(String::from("req-0002")),
                args: vec![ticker],
            },
            OutgoingMessage::Subscribe {
                req_id: Some(String::from("req-0003")),
                args: vec![trade.clone()],
            },
            OutgoingMessage::Unsubscribe {
                req_id: Some(String::from("req-0004")),
                args: vec![trade],
            },
            OutgoingMessage::Subscribe {
                req_id: Some(String::from("req-0005")),
                args: vec![kline.clone()],
            },
            OutgoingMessage::Unsubscribe {
                req_id: Some(String::from("req-0006")),
                args: vec![kline],
            },
        ]
    };

    let (tx, mut rx, response) = stream(&url, WEBSOCKET_PING_INTERVAL).await?;
    println!("{response:#?}");

    tokio::spawn(async move {
        for message in messages {
            let _ = tx.send(message).await;
            sleep(Duration::from_secs(4)).await;
        }
    });

    while let Some(message) = rx.recv().await {
        println!("{message:#?}");
    }

    Ok(())
}
