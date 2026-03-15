//! Run with
//!
//! ```not_rust
//! cargo run --example stream-private
//! ```

use std::time::Duration;

use tokio::{self, time::sleep};
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

use bybit::v5::{
    BASE_URL_STREAM_DEMO, OutgoingMessage, Path, Topic, create_outgoing_message_auth,
    ws::{self, DEFAULT_PING_INTERVAL, DEFAULT_PONG_TIMEOUT},
};

use Topic::{ExecutionAllCategory, OrderAllCategory, PositionAllCategory, Wallet};

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

    let url = format!("{}{}", BASE_URL_STREAM_DEMO, Path::Private);
    let args = vec![
        ExecutionAllCategory,
        OrderAllCategory,
        PositionAllCategory,
        Wallet,
    ];
    let auth =
        create_outgoing_message_auth(api_key, api_secret, Some(String::from("req-0001")), 5_000);
    let sub = OutgoingMessage::Subscribe {
        req_id: Some(String::from("req-0002")),
        args: args.clone(),
    };
    let unsub = OutgoingMessage::Unsubscribe {
        req_id: Some(String::from("req-0003")),
        args,
    };

    let cfg = ws::Config::new(url)
        .ping_interval(Some(DEFAULT_PING_INTERVAL))
        .pong_timeout(DEFAULT_PONG_TIMEOUT);
    let (handle, mut events) = ws::spawn(cfg);

    tokio::spawn(async move {
        let _ = handle.connect().await;
        let _ = handle.send_command(auth).await;
        let _ = handle.send_command(sub).await;
        sleep(Duration::from_hours(24)).await;
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
