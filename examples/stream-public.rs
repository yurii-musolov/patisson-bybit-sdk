//! Run with
//!
//! ```not_rust
//! cargo run --example stream-public
//! ```

use std::time::Duration;

use tokio::{self, time::sleep};
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

use bybit::v5::{
    BASE_URL_STREAM_MAINNET_1, Interval, OutgoingMessage, Path, Topic,
    ws::{self, DEFAULT_PING_INTERVAL, DEFAULT_PONG_TIMEOUT},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let url = format!("{}{}", BASE_URL_STREAM_MAINNET_1, Path::PublicLinear);
    let symbol = String::from("BTCUSDT");
    let ticker = Topic::Ticker(symbol.clone());
    let trade = Topic::Trade(symbol.clone());
    let kline = Topic::Kline {
        symbol,
        interval: Interval::Minute1,
    };
    let args = vec![ticker, trade, kline];

    let sub = OutgoingMessage::Subscribe {
        req_id: Some(String::from("req-0001")),
        args: args.clone(),
    };
    let unsub = OutgoingMessage::Unsubscribe {
        req_id: Some(String::from("req-0002")),
        args,
    };

    let cfg = ws::Config::new(url)
        .ping_interval(Some(DEFAULT_PING_INTERVAL))
        .pong_timeout(DEFAULT_PONG_TIMEOUT);
    let (handle, mut events) = ws::spawn(cfg);

    tokio::spawn(async move {
        let _ = handle.connect().await;
        let _ = handle.send_command(sub).await;
        sleep(Duration::from_secs(20)).await;
        let _ = handle.send_command(unsub).await;
        sleep(Duration::from_secs(2)).await;
        let _ = handle.disconnect().await;
    });

    while let Some(event) = events.recv().await {
        info!(?event);
        if matches!(event, ws::Event::Disconnected { reason: _ }) {
            info!("disconnected");
            break;
        }
    }

    Ok(())
}
