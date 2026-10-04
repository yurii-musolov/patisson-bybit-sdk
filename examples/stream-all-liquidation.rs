//! Run with
//!
//! ```not_rust
//! cargo run --example stream-all-liquidation
//! ```

use std::time::Duration;

use tokio::{self, time::sleep};
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

use bybit::{
    Category, Environment, Topic,
    ws::{self, OutgoingMessage},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let symbol = String::from("BTCUSDT");
    let args = vec![Topic::AllLiquidation(symbol)];

    let sub = OutgoingMessage::Subscribe {
        req_id: Some(String::from("req-0001")),
        args: args.clone(),
    };
    let unsub = OutgoingMessage::Unsubscribe {
        req_id: Some(String::from("req-0002")),
        args,
    };

    let cfg = ws::Config::public(Environment::Mainnet, Category::Linear);
    let (handle, mut events) = ws::Stream::new(cfg);

    tokio::spawn(async move {
        let _ = handle.connect().await;
        let _ = handle.send_command(sub).await;
        sleep(Duration::from_secs(24 * 60 * 60)).await;
        let _ = handle.send_command(unsub).await;
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
