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
    AccountState, AccountType, BASE_URL_API_DEMO, BASE_URL_STREAM_DEMO, Category, Error, Path,
    Topic,
    http::{
        Client, Config, GetOpenClosedOrdersParams, GetPositionInfoParams, GetWalletBalanceParams,
    },
    ws::{self, CommandMsg, IncomingMessage, OutgoingMessage, create_outgoing_message_auth_at},
};

/// Bybit `retCode`: the request timestamp is outside `recv_window`.
const RET_CODE_TIMESTAMP: i64 = 10002;
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
        Config::new(BASE_URL_API_DEMO)
            .credentials(api_key.clone(), api_secret.clone())
            .recv_window(RECV_WINDOW),
    )?;
    let offset_ms = client.sync_time().await?;
    info!(offset_ms, "server time synchronized");

    let url = format!("{}{}", BASE_URL_STREAM_DEMO, Path::Private);
    let (handle, mut events) = ws::Stream::new(ws::Config::new(url));
    handle.connect().await?;

    let mut state = AccountState::new();

    while let Some(event) = events.recv().await {
        match event {
            // After the first connection and after every reconnect:
            // authenticate, subscribe, then (re)load the snapshots. Stream
            // messages that arrive while the snapshots load wait in the event
            // queue and are merged afterwards.
            ws::Event::Connected => {
                // Re-measure the offset on every (re)connect: the clock of a
                // long-running process drifts.
                let offset_ms = client.sync_time().await?;
                info!(offset_ms, "server time synchronized");
                let auth = create_outgoing_message_auth_at(
                    api_key.as_str().into(),
                    api_secret.as_str().into(),
                    None,
                    RECV_WINDOW,
                    client.server_timestamp(),
                );
                let sub = OutgoingMessage::Subscribe {
                    req_id: None,
                    args: vec![
                        Topic::OrderAllCategory,
                        Topic::PositionAllCategory,
                        Topic::Wallet,
                    ],
                };
                handle.send_command(auth).await?;
                handle.send_command(sub).await?;

                load_snapshots(&client, &mut state).await?;
                print_summary(&state);
            }
            ws::Event::Message(IncomingMessage::Command(CommandMsg::Auth {
                success: false,
                ret_msg,
                ..
            })) => {
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
                load_snapshots(&client, &mut state).await?;
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

/// Load the snapshots; if Bybit rejects the request timestamp (the clock
/// drifted since the last sync), re-synchronize and try once more.
async fn load_snapshots(client: &Client, state: &mut AccountState) -> anyhow::Result<()> {
    match try_load_snapshots(client, state).await {
        Err(Error::Api { code, .. }) if code == RET_CODE_TIMESTAMP => {
            let offset_ms = client.sync_time().await?;
            warn!(
                offset_ms,
                "request timestamp rejected, server time re-synchronized"
            );
            Ok(try_load_snapshots(client, state).await?)
        }
        result => Ok(result?),
    }
}

/// Load REST snapshots of USDT perpetual orders and positions and of the
/// unified wallet into `state`.
async fn try_load_snapshots(client: &Client, state: &mut AccountState) -> Result<(), Error> {
    let category = Category::Linear;
    let settle_coin = String::from("USDT");

    // Server time taken before the requests: anything the stream updated
    // after this moment is newer than the snapshot. Stream messages carry
    // server timestamps, so the local clock must not be used here.
    let snapshot_time = client.server_timestamp();

    let params = GetOpenClosedOrdersParams::new(category).with_settle_coin(settle_coin.clone());
    let orders = client.get_open_closed_orders_all(&params).await?;
    state.set_orders(category, orders, snapshot_time);

    let params = GetPositionInfoParams::new(category).with_settle_coin(settle_coin);
    let positions = client.get_position_info_all(&params).await?;
    state.set_positions(category, positions, snapshot_time);

    let params = GetWalletBalanceParams {
        account_type: AccountType::UNIFIED,
        coin: None,
    };
    for balance in client.get_wallet_balance(&params).await?.result.list {
        state.set_wallet(balance);
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
