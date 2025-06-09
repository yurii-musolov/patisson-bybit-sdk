use serde::Deserialize;
use serde_aux::prelude::{
    deserialize_number_from_string as number,
    deserialize_option_number_from_string as option_number,
};

use super::{
    AutoAddMargin, CancelType, Category, CreateType, Interval, OcoTriggerBy, OrderStatus,
    OrderType, PlaceType, PositionIdx, PositionStatus, RejectReason, Side, SlippageToleranceType,
    SmpType, StopOrderType, TickDirection, TimeInForce, Timestamp, TpslMode, TradeMode, TriggerBy,
    TriggerDirection, serde::invalid_as_none,
};

#[derive(PartialEq, Deserialize, Debug)]
#[serde(untagged)]
pub enum IncomingMessage {
    Command(CommandMsg),
    Ticker(Box<TickerMsg>),
    Trade(TradeMsg),
    KLine(KLineMsg),
    AllLiquidation(AllLiquidationMsg),
    Order(OrderMsg),
    Position(PositionMsg),
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(tag = "op")]
pub enum CommandMsg {
    #[serde(rename = "subscribe")]
    Subscribe {
        req_id: Option<String>,
        ret_msg: Option<String>,
        conn_id: String,
        success: Option<bool>,
    },
    #[serde(rename = "unsubscribe")]
    Unsubscribe {
        req_id: Option<String>,
        ret_msg: Option<String>,
        conn_id: String,
        success: Option<bool>,
    },
    #[serde(rename = "auth")]
    Auth {
        req_id: Option<String>,
        ret_msg: Option<String>,
        conn_id: String,
        success: bool,
    },
    #[serde(rename = "pong")]
    Pong {
        req_id: Option<String>,
        ret_msg: Option<String>,
        conn_id: String,
        args: Option<Vec<String>>,
        success: bool,
    },
    #[serde(rename = "ping")]
    Ping {
        req_id: Option<String>,
        ret_msg: Option<String>,
        conn_id: String,
        args: Option<Vec<String>>,
        success: bool,
    },
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum TickerMsg {
    #[serde(rename = "snapshot")]
    Snapshot {
        topic: String,
        #[serde(default, deserialize_with = "option_number")]
        cs: Option<u64>,
        ts: u64,
        data: TickerSnapshotMsg,
    },
    #[serde(rename = "delta")]
    Delta {
        topic: String,
        #[serde(default, deserialize_with = "option_number")]
        cs: Option<u64>,
        ts: u64,
        data: TickerDeltaMsg,
    },
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TickerSnapshotMsg {
    pub symbol: String,
    pub tick_direction: TickDirection,
    #[serde(deserialize_with = "number")]
    pub last_price: f64,
    #[serde(default, deserialize_with = "option_number")]
    pub pre_open_price: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub pre_qty: Option<f64>,
    pub cur_pre_listing_phase: Option<String>,
    #[serde(deserialize_with = "number")]
    pub prev_price24h: f64,
    #[serde(deserialize_with = "number")]
    pub price24h_pcnt: f64,
    #[serde(deserialize_with = "number")]
    pub high_price24h: f64,
    #[serde(deserialize_with = "number")]
    pub low_price24h: f64,
    #[serde(deserialize_with = "number")]
    pub prev_price1h: f64,
    #[serde(deserialize_with = "number")]
    pub mark_price: f64,
    #[serde(deserialize_with = "number")]
    pub index_price: f64,
    #[serde(deserialize_with = "number")]
    pub open_interest: f64,
    #[serde(deserialize_with = "number")]
    pub open_interest_value: f64,
    #[serde(deserialize_with = "number")]
    pub turnover24h: f64,
    #[serde(deserialize_with = "number")]
    pub volume24h: f64,
    #[serde(deserialize_with = "number")]
    pub funding_rate: f64,
    #[serde(deserialize_with = "number")]
    pub next_funding_time: Timestamp,
    #[serde(deserialize_with = "number")]
    pub bid1_price: f64,
    #[serde(deserialize_with = "number")]
    pub bid1_size: f64,
    #[serde(deserialize_with = "number")]
    pub ask1_price: f64,
    #[serde(deserialize_with = "number")]
    pub ask1_size: f64,
    #[serde(default, deserialize_with = "option_number")]
    pub delivery_time: Option<Timestamp>,
    #[serde(default, deserialize_with = "option_number")]
    pub basis_rate: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub delivery_fee_rate: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub predicted_delivery_price: Option<f64>,
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TickerDeltaMsg {
    pub symbol: String,
    pub tick_direction: Option<TickDirection>,
    #[serde(default, deserialize_with = "option_number")]
    pub last_price: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub pre_open_price: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub pre_qty: Option<f64>,
    pub cur_pre_listing_phase: Option<String>,
    #[serde(default, deserialize_with = "option_number")]
    pub prev_price24h: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub price24h_pcnt: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub high_price24h: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub low_price24h: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub prev_price1h: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub mark_price: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub index_price: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub open_interest: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub open_interest_value: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub turnover24h: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub volume24h: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub funding_rate: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub next_funding_time: Option<Timestamp>,
    #[serde(default, deserialize_with = "option_number")]
    pub bid1_price: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub bid1_size: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub ask1_price: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub ask1_size: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub delivery_time: Option<Timestamp>,
    #[serde(default, deserialize_with = "option_number")]
    pub basis_rate: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub delivery_fee_rate: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub predicted_delivery_price: Option<f64>,
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum TradeMsg {
    #[serde(rename = "snapshot")]
    Snapshot {
        id: Option<String>,
        topic: String,
        ts: Timestamp,
        data: Vec<TradeSnapshotMsg>,
    },
}

#[derive(PartialEq, Deserialize, Debug)]
pub struct TradeSnapshotMsg {
    #[serde(rename = "T")]
    pub time: Timestamp,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "S")]
    pub side: Side,
    #[serde(rename = "v", deserialize_with = "number")]
    pub size: f64,
    #[serde(rename = "p", deserialize_with = "number")]
    pub price: f64,
    #[serde(rename = "L")]
    pub tick_direction: TickDirection,
    #[serde(rename = "i")]
    pub trade_id: String,
    #[serde(rename = "BT")]
    pub block_trade: bool,
    #[serde(rename = "RPI")]
    pub rpi_trade: Option<bool>,
    #[serde(rename = "mP")]
    pub mark_price: Option<String>,
    #[serde(rename = "iP")]
    pub index_price: Option<String>,
    #[serde(rename = "mlv")]
    pub mark_iv: Option<String>,
    #[serde(rename = "iv")]
    pub iv: Option<String>,
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum KLineMsg {
    #[serde(rename = "snapshot")]
    Snapshot {
        topic: String,
        ts: u64,
        data: Vec<KLineSnapshotMsg>,
    },
}

#[derive(PartialEq, Deserialize, Debug)]
pub struct KLineSnapshotMsg {
    pub start: Timestamp,
    pub end: Timestamp,
    pub interval: Interval,
    #[serde(deserialize_with = "number")]
    pub open: f64,
    #[serde(deserialize_with = "number")]
    pub close: f64,
    #[serde(deserialize_with = "number")]
    pub high: f64,
    #[serde(deserialize_with = "number")]
    pub low: f64,
    #[serde(deserialize_with = "number")]
    pub volume: f64,
    #[serde(deserialize_with = "number")]
    pub turnover: f64,
    pub confirm: bool,
    pub timestamp: Timestamp,
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum AllLiquidationMsg {
    #[serde(rename = "snapshot")]
    Snapshot {
        topic: String,
        ts: Timestamp,
        data: Vec<AllLiquidationSnapshotMsg>,
    },
}

#[derive(PartialEq, Deserialize, Debug)]
pub struct AllLiquidationSnapshotMsg {
    #[serde(rename = "T")]
    pub time: Timestamp,
    #[serde(rename = "s")]
    pub symbol: String,
    /// When you receive a Buy update, this means that a long position has been liquidated
    #[serde(rename = "S")]
    pub side: Side,
    #[serde(rename = "v", deserialize_with = "number")]
    pub size: f64,
    #[serde(rename = "p", deserialize_with = "number")]
    pub price: f64,
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(tag = "topic")]
pub enum OrderMsg {
    #[serde(rename = "order", rename_all = "camelCase")]
    Update {
        id: String,
        creation_time: Timestamp,
        data: Vec<OrderUpdateMsg>,
    },
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderUpdateMsg {
    /// Product type
    /// UTA2.0, UTA1.0: spot, linear, inverse, option
    /// Classic account: spot, linear, inverse.
    pub category: Category,
    /// Order ID
    pub order_id: String,
    /// User customized order ID
    pub order_link_id: Option<String>,
    /// Whether to borrow.
    /// Unified spot only. 0: false, 1: true.
    /// Classic spot is not supported, always 0
    pub is_leverage: Option<bool>,
    /// Block trade ID
    pub block_trade_id: Option<String>,
    /// Symbol name
    pub symbol: String,
    /// Order price
    #[serde(deserialize_with = "number")]
    pub price: f64,
    /// Order qty
    #[serde(deserialize_with = "number")]
    pub qty: f64,
    /// Side. Buy,Sell
    pub side: Side,
    /// Position index. Used to identify positions in different position modes.
    pub position_idx: PositionIdx,
    /// Order create type
    /// Only for category=linear or inverse
    /// Spot, Option do not have this key
    pub create_type: Option<CreateType>,
    /// Order status
    pub order_status: OrderStatus,
    /// Cancel type
    pub cancel_type: CancelType,
    /// Reject reason. Classic spot is not supported
    pub reject_reason: RejectReason,
    /// Average filled price
    /// returns "" for those orders without avg price, and also for those classic account orders have partilly filled but cancelled at the end
    /// Classic Spot: not supported, always ""
    #[serde(deserialize_with = "number")]
    pub avg_price: f64,
    /// The remaining qty not executed. Classic spot is not supported
    #[serde(deserialize_with = "option_number")]
    pub leaves_qty: Option<f64>,
    /// The estimated value not executed. Classic spot is not supported
    #[serde(deserialize_with = "option_number")]
    pub leaves_value: Option<f64>,
    /// Cumulative executed order qty
    #[serde(deserialize_with = "number")]
    pub cum_exec_qty: f64,
    /// Cumulative executed order value.
    #[serde(deserialize_with = "number")]
    pub cum_exec_value: f64,
    /// Cumulative executed trading fee.
    /// Classic spot: it is the latest execution fee for order.
    /// After upgraded to the Unified account, you can use execFee for each fill in Execution topic
    #[serde(deserialize_with = "number")]
    pub cum_exec_fee: f64,
    /// Closed profit and loss for each close position order. The figure is the same as "closedPnl" from Get Closed PnL
    #[serde(deserialize_with = "number")]
    pub closed_pnl: f64,
    /// Trading fee currency for Spot only. Please understand Spot trading fee currency here
    #[serde(deserialize_with = "option_number")]
    pub fee_currency: Option<f64>,
    /// Time in force
    pub time_in_force: TimeInForce,
    /// Order type. Market,Limit. For TP/SL order, it means the order type after triggered
    pub order_type: OrderType,
    /// Stop order type
    #[serde(default, deserialize_with = "invalid_as_none")]
    pub stop_order_type: Option<StopOrderType>,
    /// The trigger type of Spot OCO order.OcoTriggerByUnknown, OcoTriggerByTp, OcoTriggerByBySl. Classic spot is not supported
    pub oco_trigger_by: Option<OcoTriggerBy>,
    /// Implied volatility
    #[serde(deserialize_with = "option_number")]
    pub order_iv: Option<f64>,
    /// The unit for qty when create Spot market orders for UTA account. baseCoin, quoteCoin
    pub market_unit: Option<String>,
    /// Spot and Futures market order slippage tolerance type TickSize, Percent, UNKNOWN(default)
    pub slippage_tolerance_type: Option<SlippageToleranceType>,
    /// Slippage tolerance value
    pub slippage_tolerance: Option<String>, // TODO: parse Option<f64> from "{}"
    /// Trigger price. If stopOrderType=TrailingStop, it is activate price. Otherwise, it is trigger price
    #[serde(deserialize_with = "option_number")]
    pub trigger_price: Option<f64>,
    /// Take profit price
    #[serde(deserialize_with = "option_number")]
    pub take_profit: Option<f64>,
    /// Stop loss price
    #[serde(deserialize_with = "option_number")]
    pub stop_loss: Option<f64>,
    /// TP/SL mode, Full: entire position for TP/SL. Partial: partial position tp/sl. Spot does not have this field, and Option returns always ""
    #[serde(default, deserialize_with = "invalid_as_none")]
    pub tpsl_mode: Option<TpslMode>,
    /// The limit order price when take profit price is triggered
    #[serde(deserialize_with = "option_number")]
    pub tp_limit_price: Option<f64>,
    /// The limit order price when stop loss price is triggered
    #[serde(deserialize_with = "option_number")]
    pub sl_limit_price: Option<f64>,
    /// The price type to trigger take profit
    #[serde(default, deserialize_with = "invalid_as_none")]
    pub tp_trigger_by: Option<TriggerBy>,
    /// The price type to trigger stop loss
    #[serde(default, deserialize_with = "invalid_as_none")]
    pub sl_trigger_by: Option<TriggerBy>,
    /// Trigger direction. 1: rise, 2: fall
    pub trigger_direction: TriggerDirection,
    /// The price type of trigger price
    #[serde(default, deserialize_with = "invalid_as_none")]
    pub trigger_by: Option<TriggerBy>,
    /// Last price when place the order, Spot is not applicable
    #[serde(deserialize_with = "option_number")]
    pub last_price_on_created: Option<f64>,
    /// Reduce only. true means reduce position size
    pub reduce_only: bool,
    /// Close on trigger.
    pub close_on_trigger: bool,
    /// Place type, option used. iv, price
    pub place_type: PlaceType,
    /// SMP execution type
    pub smp_type: SmpType,
    /// Smp group ID. If the UID has no group, it is 0 by default
    #[serde(deserialize_with = "number")]
    pub smp_group: i64,
    /// The counterparty's orderID which triggers this SMP execution
    pub smp_order_id: String,
    /// Order created timestamp (ms)
    #[serde(deserialize_with = "number")]
    pub created_time: Timestamp,
    /// Order updated timestamp (ms)
    #[serde(deserialize_with = "number")]
    pub updated_time: Timestamp,
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PositionMsg {
    /// Message ID
    id: String,
    /// Topic name
    topic: String,
    /// Data created timestamp (ms)
    creation_time: Timestamp,
    data: Vec<PositionUpdateMsg>,
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PositionUpdateMsg {
    /// Product type
    pub category: Category,
    /// Symbol name
    pub symbol: String,
    /// Position side. Buy: long, Sell: short
    /// one-way mode: classic & UTA1.0(inverse), an empty position returns None.
    /// UTA2.0(linear, inverse) & UTA1.0(linear): either one-way or hedge mode returns an empty string "" for an empty position.
    #[serde(default, deserialize_with = "invalid_as_none")]
    pub side: Option<Side>,
    /// Position size
    #[serde(deserialize_with = "number")]
    pub size: f64,
    /// Used to identify positions in different position modes
    pub position_idx: PositionIdx,
    /// Trade mode
    /// Classic & UTA1.0(inverse): 0: cross-margin, 1: isolated margin
    /// UTA2.0, UTA1.0(execpt inverse): deprecated, always 0, check Get Account Info to know the margin mode
    pub trade_mode: TradeMode,
    /// Position value
    #[serde(deserialize_with = "number")]
    pub position_value: f64,
    /// Risk tier ID
    /// for portfolio margin mode, this field returns 0, which means risk limit rules are invalid
    #[serde(deserialize_with = "number")]
    pub risk_id: i64,
    /// Risk limit value
    /// for portfolio margin mode, this field returns 0, which means risk limit rules are invalid
    #[serde(deserialize_with = "number")]
    pub risk_limit_value: f64,
    /// Entry price
    #[serde(deserialize_with = "number")]
    pub entry_price: f64,
    /// Mark price
    #[serde(deserialize_with = "number")]
    pub mark_price: f64,
    /// Position leverage
    /// for portfolio margin mode, this field returns "", which means leverage rules are invalid
    #[serde(deserialize_with = "number")]
    pub leverage: f64,
    /// Position margin
    /// Classic & UTA1.0(inverse) can refer to this field to get the position initial margin
    #[serde(deserialize_with = "number")]
    pub position_balance: f64,
    /// Whether to add margin automatically. 0: false, 1: true. For UTA, it is meaningful only when UTA enables ISOLATED_MARGIN
    pub auto_add_margin: AutoAddMargin,
    /// Initial margin
    /// Classic & UTA1.0(inverse): ignore this field
    /// UTA portfolio margin mode, it returns ""
    #[serde(deserialize_with = "number")]
    pub position_i_m: f64,
    /// Maintenance margin
    /// Classic & UTA1.0(inverse): ignore this field
    /// UTA portfolio margin mode, it returns ""
    #[serde(deserialize_with = "number")]
    pub position_m_m: f64,
    /// Position liquidation price
    /// UTA1.0(inverse) & UTA(isolated margin enabled) & Classic account: it is the real price for isolated and cross positions, and keeps "" when liqPrice <= minPrice or liqPrice >= maxPrice
    /// UTA (Cross margin mode): it is an estimated price for cross positions(because the unified mode controls the risk rate according to the account), and keeps "" when liqPrice <= minPrice or liqPrice >= maxPrice
    /// However, this field is empty for Portfolio Margin Mode, and no liquidation price will be provided
    #[serde(deserialize_with = "number")]
    pub liq_price: f64,
    /// Bankruptcy price
    /// Unified mode returns "", no position bankruptcy price (except UTA1.0(inverse))
    #[serde(deserialize_with = "option_number")]
    pub bust_price: Option<f64>,
    /// deprecated, meaningless here, always "Full"
    #[serde(default, deserialize_with = "invalid_as_none")]
    pub tpsl_mode: Option<TpslMode>,
    /// Take profit price
    #[serde(deserialize_with = "number")]
    pub take_profit: f64,
    /// Stop loss price
    #[serde(deserialize_with = "number")]
    pub stop_loss: f64,
    /// Trailing stop
    #[serde(deserialize_with = "number")]
    pub trailing_stop: f64,
    /// Unrealised profit and loss
    #[serde(default, deserialize_with = "option_number")]
    pub unrealized_pnl: Option<f64>,
    /// The realised PnL for the current holding position
    #[serde(deserialize_with = "number")]
    pub cur_realised_pnl: f64,
    /// USDC contract session avg price, it is the same figure as avg entry price shown in the web UI
    #[serde(deserialize_with = "number")]
    pub session_avg_price: f64,
    /// Delta
    pub delta: Option<String>,
    /// Gamma
    pub gamma: Option<String>,
    /// Vega
    pub vega: Option<String>,
    /// Theta
    pub theta: Option<String>,
    /// Cumulative realised pnl
    /// Futures & Perp: it is the all time cumulative realised P&L
    /// Option: it is the realised P&L when you hold that position
    #[serde(deserialize_with = "number")]
    pub cum_realised_pnl: f64,
    /// Position status. Normal, Liq, Adl
    pub position_status: PositionStatus,
    /// Auto-deleverage rank indicator. What is Auto-Deleveraging?
    #[serde(deserialize_with = "number")]
    pub adl_rank_indicator: i64,
    /// Useful when Bybit lower the risk limit
    /// true: Only allowed to reduce the position. You can consider a series of measures, e.g., lower the risk limit, decrease leverage or reduce the position, add margin, or cancel orders, after these operations, you can call confirm new risk limit endpoint to check if your position can be removed the reduceOnly mark
    /// false: There is no restriction, and it means your position is under the risk when the risk limit is systematically adjusted
    /// Only meaningful for isolated margin & cross margin of USDT Perp, USDC Perp, USDC Futures, Inverse Perp and Inverse Futures, meaningless for others
    pub is_reduce_only: bool,
    /// Useful when Bybit lower the risk limit
    /// When isReduceOnly=true: the timestamp (ms) when the MMR will be forcibly adjusted by the system
    /// When isReduceOnly=false: the timestamp when the MMR had been adjusted by system
    /// It returns the timestamp when the system operates, and if you manually operate, there is no timestamp
    /// Keeps "" by default, if there was a lower risk limit system adjustment previously, it shows that system operation timestamp
    /// Only meaningful for isolated margin & cross margin of USDT Perp, USDC Perp, USDC Futures, Inverse Perp and Inverse Futures, meaningless for others
    #[serde(deserialize_with = "option_number")]
    pub mmr_sys_updated_time: Option<Timestamp>,
    /// Useful when Bybit lower the risk limit
    /// When isReduceOnly=true: the timestamp (ms) when the leverage will be forcibly adjusted by the system
    /// When isReduceOnly=false: the timestamp when the leverage had been adjusted by system
    /// It returns the timestamp when the system operates, and if you manually operate, there is no timestamp
    /// Keeps "" by default, if there was a lower risk limit system adjustment previously, it shows that system operation timestamp
    /// Only meaningful for isolated margin & cross margin of USDT Perp, USDC Perp, USDC Futures, Inverse Perp and Inverse Futures, meaningless for others
    #[serde(deserialize_with = "option_number")]
    pub leverage_sys_updated_time: Option<Timestamp>,
    /// Timestamp of the first time a position was created on this symbol (ms)
    #[serde(deserialize_with = "number")]
    pub created_time: Timestamp,
    /// Position data updated timestamp (ms)
    #[serde(deserialize_with = "number")]
    pub updated_time: Timestamp,
    /// Cross sequence, used to associate each fill and each position update
    /// Different symbols may have the same seq, please use seq + symbol to check unique
    /// Returns "-1" if the symbol has never been traded
    /// Returns the seq updated by the last transaction when there are setting like leverage, risk limit
    pub seq: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_incoming_message_command_subscribe() {
        let json = r#"{"success":true,"ret_msg":"","conn_id":"c0c928a4-daab-460d-b186-45e90a10a3d4","req_id":"","op":"subscribe"}"#;
        let message: IncomingMessage = serde_json::from_str(json).unwrap();
        let expected = IncomingMessage::Command(CommandMsg::Subscribe {
            req_id: Some(String::new()),
            ret_msg: Some(String::new()),
            conn_id: String::from("c0c928a4-daab-460d-b186-45e90a10a3d4"),
            success: Some(true),
        });
        assert_eq!(message, expected);
    }

    #[test]
    fn deserialize_incoming_message_command_unsubscribe() {
        let json = r#"{"success":true,"ret_msg":"","conn_id":"c0c928a4-daab-460d-b186-45e90a10a3d4","req_id":"","op":"unsubscribe"}"#;
        let message: IncomingMessage = serde_json::from_str(json).unwrap();
        let expected = IncomingMessage::Command(CommandMsg::Unsubscribe {
            req_id: Some(String::new()),
            ret_msg: Some(String::new()),
            conn_id: String::from("c0c928a4-daab-460d-b186-45e90a10a3d4"),
            success: Some(true),
        });
        assert_eq!(message, expected);
    }

    #[test]
    fn deserialize_incoming_message_ticker_delta() {
        let json = r#"{
		    "topic": "tickers.BTCUSDT",
		    "type": "delta",
		    "data": {
		        "symbol": "BTCUSDT",
		        "tickDirection": "PlusTick",
		        "price24hPcnt": "-0.015895",
		        "lastPrice": "63948.50",
		        "turnover24h": "6793884423.5518",
		        "volume24h": "105991.3760",
		        "bid1Price": "63948.40",
		        "bid1Size": "3.439",
		        "ask1Price": "63948.50",
		        "ask1Size": "2.566"
		    },
		    "cs": 195377749067,
		    "ts": 1718995014034
		}"#;
        let message: IncomingMessage = serde_json::from_str(json).unwrap();
        let ticker_delta = TickerMsg::Delta {
            topic: String::from("tickers.BTCUSDT"),
            cs: Some(195377749067),
            ts: 1718995014034,
            data: TickerDeltaMsg {
                symbol: String::from("BTCUSDT"),
                tick_direction: Some(TickDirection::PlusTick),
                last_price: Some(63948.5),
                pre_open_price: None,
                pre_qty: None,
                cur_pre_listing_phase: None,
                prev_price24h: None,
                price24h_pcnt: Some(-0.015895),
                high_price24h: None,
                low_price24h: None,
                prev_price1h: None,
                mark_price: None,
                index_price: None,
                open_interest: None,
                open_interest_value: None,
                turnover24h: Some(6793884423.5518),
                volume24h: Some(105991.376),
                funding_rate: None,
                next_funding_time: None,
                bid1_price: Some(63948.4),
                bid1_size: Some(3.439),
                ask1_price: Some(63948.5),
                ask1_size: Some(2.566),
                delivery_time: None,
                basis_rate: None,
                delivery_fee_rate: None,
                predicted_delivery_price: None,
            },
        };
        let expected = IncomingMessage::Ticker(Box::new(ticker_delta));
        assert_eq!(message, expected);
    }

    #[test]
    fn deserialize_incoming_message_ticker_snapshot() {
        // Category: linear.
        let json = r#"{
		    "topic": "tickers.BTCUSDT",
		    "type": "snapshot",
		    "data": {
                "symbol":"BTCUSDT",
                "tickDirection":"ZeroPlusTick",
                "price24hPcnt":"-0.044555",
                "lastPrice":"84594.40",
                "prevPrice24h":"88539.30",
                "highPrice24h":"89389.90",
                "lowPrice24h":"82055.60",
                "prevPrice1h":"84307.20",
                "markPrice":"84594.00",
                "indexPrice":"84650.47",
                "openInterest":"52903.75",
                "openInterestValue":"4475339827.50",
                "turnover24h":"17166562011.6514",
                "volume24h":"200176.9910",
                "nextFundingTime":"1740643200000",
                "fundingRate":"-0.00016974",
                "bid1Price":"84594.30",
                "bid1Size":"6.777",
                "ask1Price":"84594.40",
                "ask1Size":"0.660",
                "preOpenPrice":"",
                "preQty":"",
                "curPreListingPhase":""
		    },
		    "cs": 337149693308,
		    "ts": 1740622194359
		}"#;
        let message: IncomingMessage = serde_json::from_str(json).unwrap();
        let ticker_snapshot = TickerMsg::Snapshot {
            topic: String::from("tickers.BTCUSDT"),
            cs: Some(337149693308),
            ts: 1740622194359,
            data: TickerSnapshotMsg {
                symbol: String::from("BTCUSDT"),
                tick_direction: TickDirection::ZeroPlusTick,
                last_price: 84594.40,
                pre_open_price: None,
                pre_qty: None,
                cur_pre_listing_phase: Some(String::from("")),
                prev_price24h: 88539.30,
                price24h_pcnt: -0.044555,
                high_price24h: 89389.90,
                low_price24h: 82055.60,
                prev_price1h: 84307.20,
                mark_price: 84594.00,
                index_price: 84650.47,
                open_interest: 52903.75,
                open_interest_value: 4475339827.50,
                turnover24h: 17166562011.6514,
                volume24h: 200176.9910,
                funding_rate: -0.00016974,
                next_funding_time: 1740643200000,
                bid1_price: 84594.30,
                bid1_size: 6.777,
                ask1_price: 84594.40,
                ask1_size: 0.660,
                delivery_time: None,
                basis_rate: None,
                delivery_fee_rate: None,
                predicted_delivery_price: None,
            },
        };
        let expected = IncomingMessage::Ticker(Box::new(ticker_snapshot));
        assert_eq!(message, expected);
    }

    #[test]
    fn deserialize_incoming_message_trade_snapshot() {
        // Category: linear.
        let json = r#"{
            "topic":"publicTrade.BTCUSDT",
            "type":"snapshot",
            "ts":1741433245359,
            "data":[
                {
                    "T":1741433245357,
                    "s":"BTCUSDT",
                    "S":"Buy",
                    "v":"0.007",
                    "p":"85821.00",
                    "L":"PlusTick",
                    "i":"485eaa70-df6e-5260-bbef-4f7324e3c5d9",
                    "BT":false
                }
            ]
        }"#;
        let message: IncomingMessage = serde_json::from_str(json).unwrap();
        let expected = IncomingMessage::Trade(TradeMsg::Snapshot {
            id: None,
            topic: String::from("publicTrade.BTCUSDT"),
            ts: 1741433245359,
            data: vec![TradeSnapshotMsg {
                time: 1741433245357,
                symbol: String::from("BTCUSDT"),
                side: Side::Buy,
                size: 0.007,
                price: 85821.00,
                tick_direction: TickDirection::PlusTick,
                trade_id: String::from("485eaa70-df6e-5260-bbef-4f7324e3c5d9"),
                block_trade: false,
                rpi_trade: None,
                mark_price: None,
                index_price: None,
                mark_iv: None,
                iv: None,
            }],
        });
        assert_eq!(message, expected);
    }

    #[test]
    fn deserialize_incoming_message_all_liquidation_snapshot() {
        // Category: linear.
        let json = r#"{
            "topic":"allLiquidation.BTCUSDT",
            "type":"snapshot",
            "ts":1741450605553,
            "data":[
                {
                    "T":1741450605236,
                    "s":"BTCUSDT",
                    "S":"Buy",
                    "v":"0.001",
                    "p":"85823.60"
                }
            ]
        }"#;
        let message: IncomingMessage = serde_json::from_str(json).unwrap();
        let expected = IncomingMessage::AllLiquidation(AllLiquidationMsg::Snapshot {
            topic: String::from("allLiquidation.BTCUSDT"),
            ts: 1741450605553,
            data: vec![AllLiquidationSnapshotMsg {
                time: 1741450605236,
                symbol: String::from("BTCUSDT"),
                side: Side::Buy,
                size: 0.001,
                price: 85823.60,
            }],
        });
        assert_eq!(message, expected);
    }

    #[test]
    fn deserialize_incoming_message_order() {
        let json = r#"{
            "id": "5923240c6880ab-c59f-420b-9adb-3639adc9dd90",
            "topic": "order",
            "creationTime": 1672364262474,
            "data": [
                {
                    "symbol": "ETH-30DEC22-1400-C",
                    "orderId": "5cf98598-39a7-459e-97bf-76ca765ee020",
                    "side": "Sell",
                    "orderType": "Market",
                    "cancelType": "UNKNOWN",
                    "price": "72.5",
                    "qty": "1",
                    "orderIv": "",
                    "timeInForce": "IOC",
                    "orderStatus": "Filled",
                    "orderLinkId": "",
                    "lastPriceOnCreated": "",
                    "reduceOnly": false,
                    "leavesQty": "",
                    "leavesValue": "",
                    "cumExecQty": "1",
                    "cumExecValue": "75",
                    "avgPrice": "75",
                    "blockTradeId": "",
                    "positionIdx": 0,
                    "cumExecFee": "0.358635",
                    "closedPnl": "0",
                    "createdTime": "1672364262444",
                    "updatedTime": "1672364262457",
                    "rejectReason": "EC_NoError",
                    "stopOrderType": "",
                    "tpslMode": "",
                    "triggerPrice": "",
                    "takeProfit": "",
                    "stopLoss": "",
                    "tpTriggerBy": "",
                    "slTriggerBy": "",
                    "tpLimitPrice": "",
                    "slLimitPrice": "",
                    "triggerDirection": 0,
                    "triggerBy": "",
                    "closeOnTrigger": false,
                    "category": "option",
                    "placeType": "price",
                    "smpType": "None",
                    "smpGroup": 0,
                    "smpOrderId": "",
                    "feeCurrency": ""
                }
            ]
        }"#;
        let message: IncomingMessage = serde_json::from_str(json).unwrap();
        let expected = IncomingMessage::Order(OrderMsg::Update {
            id: String::from("5923240c6880ab-c59f-420b-9adb-3639adc9dd90"),
            creation_time: 1672364262474,
            data: vec![OrderUpdateMsg {
                category: Category::Option,
                order_id: String::from("5cf98598-39a7-459e-97bf-76ca765ee020"),
                order_link_id: Some(String::new()),
                is_leverage: None,
                block_trade_id: Some(String::new()),
                symbol: String::from("ETH-30DEC22-1400-C"),
                price: 72.5,
                qty: 1.0,
                side: Side::Sell,
                position_idx: PositionIdx::OneWay,
                order_status: OrderStatus::Filled,
                create_type: None,
                cancel_type: CancelType::UNKNOWN,
                reject_reason: RejectReason::EcNoError,
                avg_price: 75.0,
                leaves_qty: None,
                leaves_value: None,
                cum_exec_qty: 1.0,
                cum_exec_value: 75.0,
                cum_exec_fee: 0.358635,
                closed_pnl: 0.0,
                fee_currency: None,
                time_in_force: TimeInForce::IOC,
                order_type: OrderType::Market,
                stop_order_type: None,
                oco_trigger_by: None,
                order_iv: None,
                market_unit: None,
                slippage_tolerance_type: None,
                slippage_tolerance: None,
                trigger_price: None,
                take_profit: None,
                stop_loss: None,
                tpsl_mode: None,
                tp_limit_price: None,
                sl_limit_price: None,
                tp_trigger_by: None,
                sl_trigger_by: None,
                trigger_direction: TriggerDirection::UNKNOWN,
                trigger_by: None,
                last_price_on_created: None,
                reduce_only: false,
                close_on_trigger: false,
                place_type: PlaceType::Price,
                smp_type: SmpType::None,
                smp_group: 0,
                smp_order_id: String::new(),
                created_time: 1672364262444,
                updated_time: 1672364262457,
            }],
        });
        assert_eq!(message, expected);
    }

    #[test]
    fn deserialize_incoming_message_position() {
        let json = r#"{
            "id": "1003076014fb7eedb-c7e6-45d6-a8c1-270f0169171a",
            "topic": "position",
            "creationTime": 1697682317044,
            "data": [
                {
                    "positionIdx": 2,
                    "tradeMode": 0,
                    "riskId": 1,
                    "riskLimitValue": "2000000",
                    "symbol": "BTCUSDT",
                    "side": "",
                    "size": "0",
                    "entryPrice": "0",
                    "leverage": "10",
                    "positionValue": "0",
                    "positionBalance": "0",
                    "markPrice": "28184.5",
                    "positionIM": "0",
                    "positionMM": "0",
                    "takeProfit": "0",
                    "stopLoss": "0",
                    "trailingStop": "0",
                    "unrealisedPnl": "0",
                    "curRealisedPnl": "1.26",
                    "cumRealisedPnl": "-25.06579337",
                    "sessionAvgPrice": "0",
                    "createdTime": "1694402496913",
                    "updatedTime": "1697682317038",
                    "tpslMode": "Full",
                    "liqPrice": "0",
                    "bustPrice": "",
                    "category": "linear",
                    "positionStatus": "Normal",
                    "adlRankIndicator": 0,
                    "autoAddMargin": 0,
                    "leverageSysUpdatedTime": "",
                    "mmrSysUpdatedTime": "",
                    "seq": 8327597863,
                    "isReduceOnly": false
                }
            ]
        }"#;
        let message: IncomingMessage = serde_json::from_str(json).unwrap();
        let expected = IncomingMessage::Position(PositionMsg {
            id: String::from("1003076014fb7eedb-c7e6-45d6-a8c1-270f0169171a"),
            topic: String::from("position"),
            creation_time: 1697682317044,
            data: vec![PositionUpdateMsg {
                category: Category::Linear,
                symbol: String::from("BTCUSDT"),
                side: None,
                size: 0.0,
                position_idx: PositionIdx::Sell,
                trade_mode: TradeMode::CrossMargin,
                position_value: 0.0,
                risk_id: 1,
                risk_limit_value: 2000000.0,
                entry_price: 0.0,
                mark_price: 28184.5,
                leverage: 10.0,
                position_balance: 0.0,
                auto_add_margin: AutoAddMargin::False,
                position_i_m: 0.0,
                position_m_m: 0.0,
                liq_price: 0.0,
                bust_price: None,
                tpsl_mode: Some(TpslMode::Full),
                take_profit: 0.0,
                stop_loss: 0.0,
                trailing_stop: 0.0,
                unrealized_pnl: None,
                cur_realised_pnl: 1.26,
                session_avg_price: 0.0,
                delta: None,
                gamma: None,
                vega: None,
                theta: None,
                cum_realised_pnl: -25.06579337,
                position_status: PositionStatus::Normal,
                adl_rank_indicator: 0,
                is_reduce_only: false,
                mmr_sys_updated_time: None,
                leverage_sys_updated_time: None,
                created_time: 1694402496913,
                updated_time: 1697682317038,
                seq: 8327597863,
            }],
        });
        assert_eq!(message, expected);
    }
}
