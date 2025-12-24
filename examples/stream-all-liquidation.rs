//! Run with
//!
//! ```not_rust
//! cargo run --example stream-all-liquidation
//! ```

use tokio;
use tracing::{Level, debug, warn};
use tracing_subscriber::FmtSubscriber;

use bybit::v5::{
    BASE_URL_STREAM_MAINNET_1, DEFAULT_PING_INTERVAL, IncomingMessage, OutgoingMessage, Path,
    Topic, stream,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let url = format!("{}{}", BASE_URL_STREAM_MAINNET_1, Path::PublicLinear);
    let symbol = String::from("BTCUSDT");
    let topic = Topic::AllLiquidation(symbol);
    let message = OutgoingMessage::Subscribe {
        req_id: Some(String::from("req-0001")),
        args: vec![topic],
    };

    let (tx, mut rx, response) = stream(&url, DEFAULT_PING_INTERVAL).await?;
    debug!(?response);

    tokio::spawn(async move {
        if let Err(err) = tx.send(message).await {
            warn!(?err);
        }
    });

    while let Some(message) = rx.recv().await {
        if let IncomingMessage::AllLiquidation(message) = message {
            debug!(?message);
        }
    }

    Ok(())
}
