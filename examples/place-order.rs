//! Run with
//!
//! ```not_rust
//! cargo run --example place-order
//! ```

use core::panic;

use rust_decimal::dec;
use tokio;
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

use bybit::v5::{
    BASE_URL_API_DEMO, Category, Client, ClientConfig, GetOpenClosedOrdersParams,
    GetPositionInfoParams, GetTickersParams, OrderType, PlaceOrderRequest, PositionIdx, Side,
    Ticker, TimeInForce, TpslMode, TriggerBy,
};

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

    let base_url = BASE_URL_API_DEMO; // or BASE_URL_API_MAINNET_1, BASE_URL_API_TESTNET

    let cfg = ClientConfig {
        base_url: base_url.to_owned(),
        api_key: Some(api_key),
        api_secret: Some(api_secret),
        recv_window: 15000, // Milliseconds.
        referer: None,
    };
    let client = Client::new(cfg)?;

    // -------------------------------------------------------------------------

    let category = Category::Linear;
    let symbol = String::from("BTCUSDT");
    let side = Side::Buy;
    let qty = dec!(0.001);

    // -------------------------------------------------------------------------

    let params = GetPositionInfoParams {
        category: category.clone(),
        symbol: Some(symbol.clone()),
        base_coin: None,
        settle_coin: None,
        limit: Some(10),
        cursor: None,
    };
    let response = client.get_position_info(params).await?;
    let position_idx = match response.result.list.len() {
        1 => PositionIdx::OneWay,
        2 => match side {
            Side::Buy => PositionIdx::Buy,
            Side::Sell => PositionIdx::Sell,
        },
        _ => unreachable!(),
    };

    // -------------------------------------------------------------------------

    let params = GetTickersParams {
        category: category.clone(),
        symbol: Some(symbol.clone()),
        base_coin: None,
        exp_date: None,
    };
    let price = match client.get_tickers(params).await?.result {
        Ticker::Linear { list } => list[0].last_price - dec!(5),
        _ => panic!(),
    };

    // -------------------------------------------------------------------------

    let order_type = OrderType::Limit;
    let mut request =
        PlaceOrderRequest::new(category.clone(), symbol.clone(), side, order_type, qty);
    request.position_idx = Some(position_idx);
    request.price = Some(price.clone());
    request.time_in_force = Some(TimeInForce::PostOnly);
    request.tpsl_mode = Some(TpslMode::Partial);
    request.tp_order_type = Some(OrderType::Limit);
    request.sl_order_type = Some(OrderType::Limit);
    request.tp_trigger_by = Some(TriggerBy::LastPrice);
    request.sl_trigger_by = Some(TriggerBy::LastPrice);
    request.tp_limit_price = Some(price.clone() + dec!(110));
    request.take_profit = Some(price.clone() + dec!(100));
    request.stop_loss = Some(price.clone() - dec!(100));
    request.sl_limit_price = Some(price.clone() - dec!(110));
    let response = client.place_order(request).await?;
    info!(?response);

    // -------------------------------------------------------------------------

    let params = GetOpenClosedOrdersParams {
        category,
        symbol: Some(symbol), // For linear is required.
        base_coin: None,
        settle_coin: None,
        order_id: None,
        order_link_id: None,
        open_only: None,
        order_filter: None,
        limit: None,
        cursor: None,
    };
    let response = client.get_open_closed_orders(params).await?;
    info!(?response);

    Ok(())
}
