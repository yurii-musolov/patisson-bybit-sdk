//! Run with
//!
//! ```not_rust
//! cargo run --example stream-orderbook
//! ```

use tokio;
use tracing::{Level, info, warn};
use tracing_subscriber::FmtSubscriber;

use bybit::{
    ApplyOutcome, BASE_URL_API_MAINNET_1, BASE_URL_STREAM_MAINNET_1, Category, DepthLevel,
    OrderBookState, Path, Topic,
    http::{Client, Config, GetOrderbookParams},
    ws::{self, IncomingMessage, OutgoingMessage},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let category = Category::Linear;
    let symbol = String::from("BTCUSDT");
    let depth = DepthLevel::Level1000;
    let limit = 1000;
    let topic = Topic::Orderbook {
        symbol: symbol.clone(),
        depth,
    };

    let cfg = Config {
        base_url: BASE_URL_API_MAINNET_1.to_owned(),
        api_key: None,
        api_secret: None,
        recv_window: 5000, // Milliseconds.
        referer: None,
        rate_limiter: None,
    };
    let rest = Client::new(cfg)?;

    let url = format!("{}{}", BASE_URL_STREAM_MAINNET_1, Path::PublicLinear);
    let cfg = ws::Config::new(url);
    let (handle, mut events) = ws::Stream::new(cfg);

    let subscribe = OutgoingMessage::Subscribe {
        req_id: Some(String::from("req-0001")),
        args: vec![topic],
    };
    handle.connect().await?;
    handle.send_command(subscribe).await?;

    let mut book = OrderBookState::new();

    let params = GetOrderbookParams {
        category,
        symbol: symbol.clone(),
        limit: Some(limit),
    };
    let snapshot = rest.get_orderbook(&params).await?.result;
    info!(outcome = ?book.apply_snapshot(snapshot), "applied initial snapshot");

    while let Some(event) = events.recv().await {
        match event {
            ws::Event::Message(IncomingMessage::Orderbook(msg)) => {
                let mut outcome = book.apply(msg);

                if outcome == ApplyOutcome::ResyncRequired {
                    warn!("orderbook out of sync, fetching a fresh snapshot");
                    let snapshot = rest.get_orderbook(&params).await?.result;
                    outcome = book.apply_snapshot(snapshot);
                }

                info!(?outcome, best_bid = ?book.best_bid(), best_ask = ?book.best_ask());
            }
            ws::Event::Disconnected { reason } => {
                warn!(?reason, "disconnected");
                break;
            }
            _ => {}
        }
    }

    Ok(())
}
