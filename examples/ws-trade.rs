//! Place and cancel an order over the order entry stream (`/v5/trade`).
//!
//! The order entry stream is not available on demo trading, so this example
//! uses testnet keys. It places a minimal post-only limit order far below the
//! market, cancels it at once and checks over REST that it is cancelled.
//!
//! Run with
//!
//! ```not_rust
//! API_KEY=... API_SECRET=... cargo run --example ws-trade
//! ```

use rust_decimal::{Decimal, dec};
use tracing::{Level, info, warn};
use tracing_subscriber::FmtSubscriber;

use bybit::{
    Category, Environment, OrderType, Side, TimeInForce,
    http::{
        CancelOrderRequest, Client, Config, GetOpenClosedOrdersParams, GetTickersParams,
        PlaceOrderRequest, Ticker,
    },
    ws::{self, TradeClient},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let api_key = std::env::var("API_KEY").expect("environment variable API_KEY is required");
    let api_secret =
        std::env::var("API_SECRET").expect("environment variable API_SECRET is required");

    let env = Environment::Testnet;
    let category = Category::Linear;
    let symbol = "BTCUSDT";

    let client =
        Client::new(Config::for_env(env).credentials(api_key.clone(), api_secret.clone()))?;
    client.sync_time().await?;

    let config = ws::Config::trade(env)?
        .credentials(api_key, api_secret)
        .server_clock(client.clock());
    let (trade, mut events) = TradeClient::new(config)?;
    trade.connect().await?;
    while let Some(event) = events.recv().await {
        match event {
            ws::Event::Authenticated => break,
            ws::Event::AuthFailed { ret_msg } => anyhow::bail!("auth rejected: {ret_msg:?}"),
            ws::Event::Disconnected { reason } => anyhow::bail!("disconnected: {reason:?}"),
            _ => {}
        }
    }
    info!("order entry stream authenticated");

    // A post-only buy at half the last price never fills.
    let params = GetTickersParams::new(category).with_symbol(symbol);
    let Ticker::Linear { list } = client.get_tickers(&params).await?.result else {
        anyhow::bail!("unexpected ticker category");
    };
    let last_price = list.first().map(|t| t.last_price).unwrap_or_default();
    anyhow::ensure!(last_price > Decimal::ZERO, "no last price for {symbol}");
    let price = (last_price / Decimal::TWO).round_dp(0);

    let mut order =
        PlaceOrderRequest::new(category, symbol, Side::Buy, OrderType::Limit, dec!(0.001));
    order.price = Some(price);
    order.time_in_force = Some(TimeInForce::PostOnly);
    let placed = trade.place_order(&order).await?;
    let order_id = placed.data.order_id;
    info!(%order_id, %price, "order placed");

    let cancel = CancelOrderRequest::new(category, symbol).with_order_id(order_id.clone());
    match trade.cancel_order(&cancel).await {
        Ok(_) => info!(%order_id, "order cancelled"),
        Err(e) => warn!(error = %e, %order_id, "cancel failed, check the order manually"),
    }

    // The ack only means Bybit accepted the request: confirm over REST.
    let params = GetOpenClosedOrdersParams::new(category)
        .with_symbol(symbol)
        .with_order_id(order_id.clone());
    for order in client.get_open_closed_orders(&params).await?.result.list {
        info!(order_id = %order.order_id, status = ?order.order_status, "order status");
    }
    let open = client
        .get_open_closed_orders(&GetOpenClosedOrdersParams::new(category).with_symbol(symbol))
        .await?
        .result
        .list;
    info!(open_orders = open.len(), "open orders left on {symbol}");

    trade.disconnect().await?;
    Ok(())
}
