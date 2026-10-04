//! Run with
//!
//! ```not_rust
//! cargo run --example stream-public
//! ```

use std::time::Duration;

use tokio::{self, time::sleep};
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

use bybit::{Category, DepthLevel, Environment, Interval, Topic, ws};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let symbol = String::from("BTCUSDT");
    let ticker = Topic::Ticker(symbol.clone());
    let trade = Topic::Trade(symbol.clone());
    let kline = Topic::Kline {
        symbol: symbol.clone(),
        interval: Interval::Minute1,
    };
    let orderbook = Topic::Orderbook {
        symbol,
        depth: DepthLevel::Level1000,
    };
    let topics = vec![ticker, trade, kline, orderbook];

    let cfg = ws::Config::public(Environment::Mainnet, Category::Linear);
    let (handle, mut events) = ws::Stream::new(cfg);

    // Subscriptions are restored automatically after every reconnect.
    handle.subscribe(topics.clone()).await?;
    handle.connect().await?;

    tokio::spawn(async move {
        sleep(Duration::from_secs(20)).await;
        let _ = handle.unsubscribe(topics).await;
        sleep(Duration::from_secs(2)).await;
        let _ = handle.disconnect().await;
    });

    while let Some(event) = events.recv().await {
        info!(?event);
        if matches!(event, ws::Event::Disconnected { reason: _ }) {
            break;
        }
    }

    Ok(())
}
