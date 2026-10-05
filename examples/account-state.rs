//! Keep a local copy of open orders, positions and the wallet balance
//! (`AccountState`) from REST snapshots and the private WebSocket stream.
//!
//! Signed requests and the stream `auth` use the server clock
//! (`Client::sync_time`), so the example also works when the local clock is
//! off by more than `recv_window`.
//!
//! Run with
//!
//! ```not_rust
//! API_KEY=... API_SECRET=... cargo run --example account-state
//! ```

use tracing::{Level, info, warn};
use tracing_subscriber::FmtSubscriber;

use bybit::{
    AccountScope, AccountState, AccountType, Environment, Topic,
    http::{Client, Config},
    ws::{self, IncomingMessage},
};

/// Validity of signed requests and of the stream `auth`, milliseconds.
const RECV_WINDOW: u64 = 5_000;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let api_key = std::env::var("API_KEY").expect("environment variable API_KEY is required");
    let api_secret =
        std::env::var("API_SECRET").expect("environment variable API_SECRET is required");

    let client = Client::new(
        Config::for_env(Environment::Demo)
            .credentials(api_key.clone(), api_secret.clone())
            .recv_window(RECV_WINDOW),
    )?;
    let offset_ms = client.sync_time().await?;
    info!(offset_ms, "server time synchronized");

    // The driver authenticates (with the server clock of `client`) and
    // restores the subscriptions on every (re)connect.
    let config = ws::Config::private(Environment::Demo)
        .credentials(api_key, api_secret)
        .auth_recv_window(RECV_WINDOW)
        .server_clock(client.clock());
    let (handle, mut events) = ws::Stream::new(config);
    handle
        .subscribe(vec![
            Topic::OrderAllCategory,
            Topic::PositionAllCategory,
            Topic::Wallet,
        ])
        .await?;
    handle.connect().await?;

    // What to load from REST: USDT perpetual orders and positions and the
    // unified wallet.
    let scope = AccountScope::new()
        .linear("USDT")
        .wallet(AccountType::UNIFIED);
    let mut state = AccountState::new();

    while let Some(event) = events.recv().await {
        match event {
            // After every (re)connect, once authenticated: (re)load the
            // snapshots. Stream messages that arrive meanwhile wait in the
            // event queue and are merged afterwards.
            ws::Event::Authenticated => {
                // Re-measure the offset: the clock of a long-running process
                // drifts, and the next reconnect authenticates with it.
                let offset_ms = client.sync_time().await?;
                info!(offset_ms, "server time synchronized");
                state.reload(&client, &scope).await?;
                print_summary(&state);
            }
            ws::Event::AuthFailed { ret_msg } => {
                warn!(?ret_msg, "stream authentication failed");
                break;
            }
            ws::Event::Message(IncomingMessage::Topic(msg)) => {
                for change in state.apply(msg) {
                    info!(?change);
                }
                print_summary(&state);
            }
            // Stream messages were dropped: the local copy may be stale.
            ws::Event::Lagged { dropped } => {
                warn!(dropped, "events dropped, reloading snapshots");
                state.reload(&client, &scope).await?;
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

fn print_summary(state: &AccountState) {
    let equity = state
        .wallet(AccountType::UNIFIED)
        .map(|wallet| wallet.total_equity);
    info!(
        open_orders = state.orders().count(),
        open_positions = state.open_positions().count(),
        ?equity,
        "account state"
    );
}
