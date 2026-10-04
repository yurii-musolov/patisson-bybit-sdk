//! Run with
//!
//! ```not_rust
//! cargo run --example stream-private
//! ```

use std::time::Duration;

use tokio::{self, time::sleep};
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

use bybit::{
    Environment, Topic,
    http::{self, Client},
    ws,
};

use Topic::{ExecutionAllCategory, OrderAllCategory, PositionAllCategory, Wallet};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let api_key = std::env::var("API_KEY").expect("environment variable API_KEY is required");
    let api_secret =
        std::env::var("API_SECRET").expect("environment variable API_SECRET is required");

    // Use the server clock for the `auth` expiry.
    let client = Client::new(http::Config::for_env(Environment::Demo))?;
    client.sync_time().await?;

    // The driver authenticates and restores the subscriptions on every
    // (re)connect.
    let cfg = ws::Config::private(Environment::Demo)
        .credentials(api_key, api_secret)
        .server_clock(client.clock());
    let (handle, mut events) = ws::Stream::new(cfg);

    let topics = vec![
        ExecutionAllCategory,
        OrderAllCategory,
        PositionAllCategory,
        Wallet,
    ];
    handle.subscribe(topics.clone()).await?;
    handle.connect().await?;

    tokio::spawn(async move {
        sleep(Duration::from_secs(24 * 60 * 60)).await;
        let _ = handle.unsubscribe(topics).await;
        sleep(Duration::from_secs(2)).await;
        let _ = handle.disconnect().await;
    });

    while let Some(event) = events.recv().await {
        info!(?event);
        if matches!(
            event,
            ws::Event::Disconnected { .. } | ws::Event::AuthFailed { .. }
        ) {
            break;
        }
    }

    Ok(())
}
