use std::collections::HashMap;

use crate::{
    AccountType, AdlRankIndicator, CancelType, Category, CreateType, ExecType, ExtraFeeType,
    ExtraSubFeeType, Interval, OcoTriggerBy, OrderStatus, OrderType, PlaceType, PositionIdx,
    PositionStatus, RejectReason, Side, SlippageToleranceType, SmpType, StopOrderType,
    TickDirection, TimeInForce, Timestamp, Topic, TpslMode, TriggerBy, TriggerDirection,
    http::{OrderbookLevel, WalletCoin},
    serde::hash_map,
    serde::{empty_string_as_none, int_to_bool, string_to_bool, string_to_option_bool},
};

use rust_decimal::{Decimal, serde::str_option::deserialize as option_decimal};
use serde::Deserialize;
use serde_aux::prelude::{
    deserialize_number_from_string as number,
    deserialize_option_number_from_string as option_number,
};

#[derive(PartialEq, Deserialize, Debug)]
#[serde(untagged)]
pub enum IncomingMessage {
    Command(CommandMsg),
    // TickerMsg is 584 bytes (TickerDeltaMsg alone is 528 bytes — 24 optional Decimals × 16 bytes
    // each, plus TickerSnapshotMsg at 448 bytes). Without Box the entire IncomingMessage enum
    // would be 584 bytes on every allocation, including the tiny Command/Trade/Topic variants that
    // flow through the mpsc channel far more frequently. Box keeps IncomingMessage at 104 bytes.
    Ticker(Box<TickerMsg>),
    Trade(TradeMsg),
    KLine(KLineMsg),
    Orderbook(OrderbookMsg),
    AllLiquidation(AllLiquidationMsg),
    Topic(TopicMessage),
}

impl IncomingMessage {
    pub fn is_pong(&self) -> bool {
        matches!(
            self,
            IncomingMessage::Command(CommandMsg::Pong {
                req_id: _,
                ret_msg: _,
                conn_id: _,
                args: _,
                success: _,
            })
        )
    }
    pub fn is_ping(&self) -> bool {
        matches!(
            self,
            IncomingMessage::Command(CommandMsg::Ping {
                req_id: _,
                ret_msg: _,
                conn_id: _,
                args: _,
                success: _,
            })
        )
    }
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(tag = "op")]
pub enum CommandMsg {
    #[serde(rename = "subscribe")]
    Subscribe {
        #[serde(default, deserialize_with = "empty_string_as_none")]
        req_id: Option<String>,
        #[serde(default, deserialize_with = "empty_string_as_none")]
        ret_msg: Option<String>,
        conn_id: String,
        success: bool,
    },
    #[serde(rename = "unsubscribe")]
    Unsubscribe {
        #[serde(default, deserialize_with = "empty_string_as_none")]
        req_id: Option<String>,
        #[serde(default, deserialize_with = "empty_string_as_none")]
        ret_msg: Option<String>,
        conn_id: String,
        success: bool,
    },
    #[serde(rename = "auth")]
    Auth {
        #[serde(default, deserialize_with = "empty_string_as_none")]
        req_id: Option<String>,
        #[serde(default, deserialize_with = "empty_string_as_none")]
        ret_msg: Option<String>,
        conn_id: String,
        success: bool,
    },
    #[serde(rename = "pong")]
    Pong {
        #[serde(default, deserialize_with = "empty_string_as_none")]
        req_id: Option<String>,
        #[serde(default, deserialize_with = "empty_string_as_none")]
        ret_msg: Option<String>,
        conn_id: String,
        args: Option<Vec<String>>,
        success: Option<bool>,
    },
    #[serde(rename = "ping")]
    Ping {
        #[serde(default, deserialize_with = "empty_string_as_none")]
        req_id: Option<String>,
        #[serde(default, deserialize_with = "empty_string_as_none")]
        ret_msg: Option<String>,
        conn_id: String,
        args: Option<Vec<String>>,
        success: bool,
    },
}

// TODO: Use PublicMsg<T>
#[derive(PartialEq, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum TickerMsg {
    #[serde(rename = "snapshot")]
    Snapshot {
        topic: Topic,
        #[serde(default, deserialize_with = "option_number")]
        cs: Option<u64>,
        ts: Timestamp,
        data: TickerSnapshotMsg,
    },
    #[serde(rename = "delta")]
    Delta {
        topic: Topic,
        #[serde(default, deserialize_with = "option_number")]
        cs: Option<u64>,
        ts: Timestamp,
        data: TickerDeltaMsg,
    },
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TickerSnapshotMsg {
    pub symbol: String,
    pub tick_direction: TickDirection,
    pub last_price: Decimal,
    #[serde(default, deserialize_with = "option_decimal")]
    pub pre_open_price: Option<Decimal>,
    #[serde(default, deserialize_with = "option_decimal")]
    pub pre_qty: Option<Decimal>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub cur_pre_listing_phase: Option<String>,
    pub prev_price24h: Decimal,
    pub price24h_pcnt: Decimal,
    pub high_price24h: Decimal,
    pub low_price24h: Decimal,
    pub prev_price1h: Decimal,
    pub mark_price: Decimal,
    pub index_price: Decimal,
    pub open_interest: Decimal,
    pub open_interest_value: Decimal,
    pub turnover24h: Decimal,
    pub volume24h: Decimal,
    pub funding_rate: Decimal,
    #[serde(default, deserialize_with = "number")]
    pub next_funding_time: Timestamp,
    pub bid1_price: Decimal,
    pub bid1_size: Decimal,
    pub ask1_price: Decimal,
    pub ask1_size: Decimal,
    #[serde(default, deserialize_with = "option_number")]
    pub delivery_time: Option<Timestamp>,
    #[serde(default, deserialize_with = "option_decimal")]
    pub basis_rate: Option<Decimal>,
    #[serde(default, deserialize_with = "option_decimal")]
    pub delivery_fee_rate: Option<Decimal>,
    #[serde(default, deserialize_with = "option_decimal")]
    pub predicted_delivery_price: Option<Decimal>,
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TickerDeltaMsg {
    pub symbol: String,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub tick_direction: Option<TickDirection>,
    #[serde(default, deserialize_with = "option_decimal")]
    pub last_price: Option<Decimal>,
    #[serde(default, deserialize_with = "option_decimal")]
    pub pre_open_price: Option<Decimal>,
    #[serde(default, deserialize_with = "option_decimal")]
    pub pre_qty: Option<Decimal>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub cur_pre_listing_phase: Option<String>,
    #[serde(default, deserialize_with = "option_decimal")]
    pub prev_price24h: Option<Decimal>,
    #[serde(default, deserialize_with = "option_decimal")]
    pub price24h_pcnt: Option<Decimal>,
    #[serde(default, deserialize_with = "option_decimal")]
    pub high_price24h: Option<Decimal>,
    #[serde(default, deserialize_with = "option_decimal")]
    pub low_price24h: Option<Decimal>,
    #[serde(default, deserialize_with = "option_decimal")]
    pub prev_price1h: Option<Decimal>,
    #[serde(default, deserialize_with = "option_decimal")]
    pub mark_price: Option<Decimal>,
    #[serde(default, deserialize_with = "option_decimal")]
    pub index_price: Option<Decimal>,
    #[serde(default, deserialize_with = "option_decimal")]
    pub open_interest: Option<Decimal>,
    #[serde(default, deserialize_with = "option_decimal")]
    pub open_interest_value: Option<Decimal>,
    #[serde(default, deserialize_with = "option_decimal")]
    pub turnover24h: Option<Decimal>,
    #[serde(default, deserialize_with = "option_decimal")]
    pub volume24h: Option<Decimal>,
    #[serde(default, deserialize_with = "option_decimal")]
    pub funding_rate: Option<Decimal>,
    #[serde(default, deserialize_with = "option_decimal")]
    pub next_funding_time: Option<Decimal>,
    #[serde(default, deserialize_with = "option_decimal")]
    pub bid1_price: Option<Decimal>,
    #[serde(default, deserialize_with = "option_decimal")]
    pub bid1_size: Option<Decimal>,
    #[serde(default, deserialize_with = "option_decimal")]
    pub ask1_price: Option<Decimal>,
    #[serde(default, deserialize_with = "option_decimal")]
    pub ask1_size: Option<Decimal>,
    #[serde(default, deserialize_with = "option_number")]
    pub delivery_time: Option<Timestamp>,
    #[serde(default, deserialize_with = "option_decimal")]
    pub basis_rate: Option<Decimal>,
    #[serde(default, deserialize_with = "option_decimal")]
    pub delivery_fee_rate: Option<Decimal>,
    #[serde(default, deserialize_with = "option_decimal")]
    pub predicted_delivery_price: Option<Decimal>,
}

// TODO: Use PublicMsg<T>
#[derive(PartialEq, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum TradeMsg {
    #[serde(rename = "snapshot")]
    Snapshot {
        #[serde(default, deserialize_with = "empty_string_as_none")]
        id: Option<String>,
        topic: Topic,
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
    #[serde(rename = "v")]
    pub size: Decimal,
    #[serde(rename = "p")]
    pub price: Decimal,
    #[serde(rename = "L")]
    pub tick_direction: TickDirection,
    #[serde(rename = "i")]
    pub trade_id: String,
    #[serde(rename = "BT")]
    pub block_trade: bool,
    #[serde(rename = "RPI")]
    pub rpi_trade: Option<bool>,
    #[serde(rename = "mP", default, deserialize_with = "empty_string_as_none")]
    pub mark_price: Option<String>,
    #[serde(rename = "iP", default, deserialize_with = "empty_string_as_none")]
    pub index_price: Option<String>,
    #[serde(rename = "mlv", default, deserialize_with = "empty_string_as_none")]
    pub mark_iv: Option<String>,
    #[serde(rename = "iv", default, deserialize_with = "empty_string_as_none")]
    pub iv: Option<String>,
}

// TODO: Use PublicMsg<T>
#[derive(PartialEq, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum KLineMsg {
    #[serde(rename = "snapshot")]
    Snapshot {
        topic: Topic,
        ts: Timestamp,
        data: Vec<KLineSnapshotMsg>,
    },
}

#[derive(PartialEq, Deserialize, Debug)]
pub struct KLineSnapshotMsg {
    pub start: Timestamp,
    pub end: Timestamp,
    pub interval: Interval,
    pub open: Decimal,
    pub close: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub volume: Decimal,
    pub turnover: Decimal,
    pub confirm: bool,
    pub timestamp: Timestamp,
}

// TODO: Use PublicMsg<T>
#[derive(PartialEq, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum OrderbookMsg {
    #[serde(rename = "snapshot")]
    Snapshot {
        topic: Topic,
        ts: Timestamp,
        data: OrderbookDataMsg,
        cts: Timestamp,
    },
    #[serde(rename = "delta")]
    Delta {
        topic: Topic,
        ts: Timestamp,
        data: OrderbookDataMsg,
        cts: Timestamp,
    },
}

#[derive(PartialEq, Deserialize, Debug)]
pub struct OrderbookDataMsg {
    /// Symbol name
    #[serde(rename = "s")]
    pub symbol: String,
    /// Bid, buy side. For snapshot, sorted by price in descending order.
    /// For delta, size 0 means delete the price level
    #[serde(rename = "b")]
    pub bids: Vec<OrderbookLevel>,
    /// Ask, sell side. For snapshot, sorted by price in ascending order.
    /// For delta, size 0 means delete the price level
    #[serde(rename = "a")]
    pub asks: Vec<OrderbookLevel>,
    /// Update ID, is a sequence. Occasionally, you'll receive "u"=1, which is a snapshot
    /// data due to the restart of the websocket service. So please overwrite your local orderbook
    #[serde(rename = "u")]
    pub update_id: i64,
    /// Cross sequence. You can use this field to compare different levels orderbook data,
    /// and for the smaller seq, then it means the data is generated earlier
    pub seq: i64,
}

// TODO: Use PublicMsg<T>
#[derive(PartialEq, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum AllLiquidationMsg {
    #[serde(rename = "snapshot")]
    Snapshot {
        topic: Topic,
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
    #[serde(rename = "v")]
    pub size: Decimal,
    #[serde(rename = "p")]
    pub price: Decimal,
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(tag = "topic")] // TODO: Use field topic
pub enum TopicMessage {
    #[serde(rename = "order")]
    Order(PrivateMsg<Vec<OrderMsg>>),
    #[serde(rename = "position")]
    Position(PrivateMsg<Vec<PositionMsg>>),
    #[serde(rename = "wallet")]
    Wallet(PrivateMsg<Vec<WalletMsg>>),
    #[serde(rename = "execution")]
    Execution(PrivateMsg<Vec<ExecutionMsg>>),
    /// Fast execution: fills arrive immediately, before settlement completes.
    #[serde(rename = "fastExecution")]
    FastExecution(PrivateMsg<Vec<FastExecutionMsg>>),
    /// Option greek updates.
    #[serde(rename = "greeks")]
    Greeks(PrivateMsg<Vec<GreekMsg>>),
    /// Disconnection and Cancellation Protection status update.
    #[serde(rename = "dcp")]
    Dcp(PrivateMsg<DcpMsg>),
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PublicMsg<T> {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    id: Option<String>,
    #[serde(default, deserialize_with = "option_number")]
    cs: Option<u64>,
    ts: Timestamp,
    data: T,
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PrivateMsg<T> {
    // TODO: pub topic: Topic, /// Topic name
    /// Message ID
    pub id: String,
    /// Data created timestamp (ms)
    pub creation_time: Timestamp,
    pub data: T,
}

#[derive(PartialEq, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderMsg {
    /// Product type
    /// UTA2.0, UTA1.0: spot, linear, inverse, option
    /// Classic account: spot, linear, inverse.
    pub category: Category,
    /// Order ID
    pub order_id: String,
    /// User customized order ID
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub order_link_id: Option<String>,
    /// Whether to borrow.
    /// Unified spot only. 0: false, 1: true.
    /// Classic spot is not supported, always 0
    #[serde(default, deserialize_with = "string_to_option_bool")]
    pub is_leverage: Option<bool>,
    /// Block trade ID
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub block_trade_id: Option<String>,
    /// Symbol name
    pub symbol: String,
    /// Order price
    pub price: Decimal,
    /// Dedicated field for EU liquidity provider
    #[serde(default, deserialize_with = "option_decimal")]
    pub broker_order_price: Option<Decimal>,
    /// Order qty
    pub qty: Decimal,
    /// Side. Buy,Sell
    pub side: Side,
    /// Position index. Used to identify positions in different position modes.
    pub position_idx: PositionIdx,
    /// Order status
    pub order_status: OrderStatus,
    /// Order create type
    /// Only for category=linear or inverse
    /// Spot, Option do not have this key
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub create_type: Option<CreateType>,
    /// Cancel type
    pub cancel_type: CancelType,
    /// Reject reason. Classic spot is not supported
    pub reject_reason: RejectReason,
    /// Average filled price
    /// returns "" for those orders without avg price, and also for those classic account orders have partilly filled but cancelled at the end
    /// Classic Spot: not supported, always ""
    #[serde(default, deserialize_with = "option_decimal")]
    pub avg_price: Option<Decimal>,
    /// The remaining qty not executed. Classic spot is not supported
    #[serde(default, deserialize_with = "option_decimal")]
    pub leaves_qty: Option<Decimal>,
    /// The estimated value not executed. Classic spot is not supported
    #[serde(default, deserialize_with = "option_decimal")]
    pub leaves_value: Option<Decimal>,
    /// Cumulative executed order qty
    pub cum_exec_qty: Decimal,
    /// Cumulative executed order value.
    pub cum_exec_value: Decimal,
    /// Cumulative executed trading fee.
    /// Classic spot: it is the latest execution fee for order.
    /// After upgraded to the Unified account, you can use execFee for each fill in Execution topic
    pub cum_exec_fee: Decimal,
    /// linear, spot: Cumulative trading fee details instead of cumExecFee
    pub cum_fee_detail: Option<serde_json::Value>,
    /// Closed profit and loss for each close position order. The figure is the same as "closedPnl" from Get Closed PnL
    pub closed_pnl: Decimal,
    /// Trading fee currency for Spot only. Please understand Spot trading fee currency here
    #[serde(deserialize_with = "option_decimal")]
    pub fee_currency: Option<Decimal>,
    /// Time in force
    pub time_in_force: TimeInForce,
    /// Order type. Market,Limit. For TP/SL order, it means the order type after triggered
    pub order_type: OrderType,
    /// Stop order type
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub stop_order_type: Option<StopOrderType>,
    /// The trigger type of Spot OCO order.OcoTriggerByUnknown, OcoTriggerByTp, OcoTriggerByBySl. Classic spot is not supported
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub oco_trigger_by: Option<OcoTriggerBy>,
    /// Implied volatility
    #[serde(deserialize_with = "option_decimal")]
    pub order_iv: Option<Decimal>,
    /// The unit for qty when create Spot market orders for UTA account. baseCoin, quoteCoin
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub market_unit: Option<String>,
    /// Spot and Futures market order slippage tolerance type TickSize, Percent, UNKNOWN(default)
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub slippage_tolerance_type: Option<SlippageToleranceType>,
    /// Slippage tolerance value
    #[serde(default, deserialize_with = "option_decimal")]
    pub slippage_tolerance: Option<Decimal>,
    /// Trigger price. If stopOrderType=TrailingStop, it is activate price. Otherwise, it is trigger price
    #[serde(deserialize_with = "option_decimal")]
    pub trigger_price: Option<Decimal>,
    /// Take profit price
    #[serde(deserialize_with = "option_decimal")]
    pub take_profit: Option<Decimal>,
    /// Stop loss price
    #[serde(deserialize_with = "option_decimal")]
    pub stop_loss: Option<Decimal>,
    /// TP/SL mode, Full: entire position for TP/SL. Partial: partial position tp/sl. Spot does not have this field, and Option returns always ""
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub tpsl_mode: Option<TpslMode>,
    /// The limit order price when take profit price is triggered
    #[serde(deserialize_with = "option_decimal")]
    pub tp_limit_price: Option<Decimal>,
    /// The limit order price when stop loss price is triggered
    #[serde(deserialize_with = "option_decimal")]
    pub sl_limit_price: Option<Decimal>,
    /// The price type to trigger take profit
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub tp_trigger_by: Option<TriggerBy>,
    /// The price type to trigger stop loss
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub sl_trigger_by: Option<TriggerBy>,
    /// Trigger direction. 1: rise, 2: fall
    pub trigger_direction: TriggerDirection,
    /// The price type of trigger price
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub trigger_by: Option<TriggerBy>,
    /// Last price when place the order, Spot is not applicable
    #[serde(deserialize_with = "option_decimal")]
    pub last_price_on_created: Option<Decimal>,
    /// Reduce only. true means reduce position size
    pub reduce_only: bool,
    /// Close on trigger.
    pub close_on_trigger: bool,
    /// Place type, option used. iv, price
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub place_type: Option<PlaceType>,
    /// SMP execution type
    pub smp_type: SmpType,
    /// Smp group ID. If the UID has no group, it is 0 by default
    #[serde(deserialize_with = "number")]
    pub smp_group: i64,
    /// The counterparty's orderID which triggers this SMP execution
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub smp_order_id: Option<String>,
    /// Order created timestamp (ms)
    #[serde(deserialize_with = "number")]
    pub created_time: Timestamp,
    /// Order updated timestamp (ms)
    #[serde(deserialize_with = "number")]
    pub updated_time: Timestamp,
}

#[derive(PartialEq, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PositionMsg {
    /// Product type
    pub category: Category,
    /// Symbol name
    pub symbol: String,
    /// Position side. Buy: long, Sell: short
    /// one-way mode: classic & UTA1.0(inverse), an empty position returns None.
    /// UTA2.0(linear, inverse) & UTA1.0(linear): either one-way or hedge mode returns an empty string "" for an empty position.
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub side: Option<Side>,
    /// Position size
    pub size: Decimal,
    /// Used to identify positions in different position modes
    pub position_idx: PositionIdx,
    /// Position value
    pub position_value: Decimal,
    /// Risk tier ID
    /// for portfolio margin mode, this field returns 0, which means risk limit rules are invalid
    #[serde(deserialize_with = "number")]
    pub risk_id: i64,
    /// Risk limit value
    /// for portfolio margin mode, this field returns 0, which means risk limit rules are invalid
    #[serde(default, deserialize_with = "option_decimal")]
    pub risk_limit_value: Option<Decimal>,
    /// Entry price
    pub entry_price: Decimal,
    /// Mark price
    pub mark_price: Decimal,
    /// Position leverage
    /// for portfolio margin mode, this field returns "", which means leverage rules are invalid
    pub leverage: Decimal,
    /// Whether to add margin automatically. 0: false, 1: true. For UTA, it is meaningful only when UTA enables ISOLATED_MARGIN
    #[serde(default, deserialize_with = "int_to_bool")]
    pub auto_add_margin: bool,
    /// Initial margin, the same value as positionIMByMp, please note this change The New Margin Calculation: Adjustments and Implications
    /// Portfolio margin mode: returns ""
    #[serde(rename = "positionIM", default, deserialize_with = "option_decimal")]
    pub position_im: Option<Decimal>,
    /// Maintenance margin, the same value as positionMMByMp
    /// Portfolio margin mode: returns ""
    #[serde(rename = "positionMM", default, deserialize_with = "option_decimal")]
    pub position_mm: Option<Decimal>,
    /// Initial margin calculated by mark price, the same value as positionIM
    /// Portfolio margin mode: returns ""
    #[serde(
        rename = "positionIMByMp",
        default,
        deserialize_with = "option_decimal"
    )]
    pub position_im_by_mp: Option<Decimal>,
    /// Maintenance margin calculated by mark price, the same value as positionMM
    /// Portfolio margin mode: returns ""
    #[serde(
        rename = "positionMMByMp",
        default,
        deserialize_with = "option_decimal"
    )]
    pub position_mm_by_mp: Option<Decimal>,
    /// Position liquidation price
    /// Isolated margin:
    /// it is the real price for isolated and cross positions, and keeps "" when liqPrice <= minPrice or liqPrice >= maxPrice
    /// Cross margin:
    /// it is an estimated price for cross positions(because the unified mode controls the risk rate according to the account), and keeps "" when liqPrice <= minPrice or liqPrice >= maxPrice
    /// this field is empty for Portfolio Margin Mode, and no liquidation price will be provided
    #[serde(default, deserialize_with = "option_decimal")]
    pub liq_price: Option<Decimal>,
    /// Take profit price
    pub take_profit: Decimal,
    /// Stop loss price
    pub stop_loss: Decimal,
    /// Trailing stop
    pub trailing_stop: Decimal,
    /// Unrealised profit and loss
    pub unrealised_pnl: Decimal,
    /// The realised PnL for the current holding position
    pub cur_realised_pnl: Decimal,
    /// USDC contract session avg price, it is the same figure as avg entry price shown in the web UI
    #[serde(default, deserialize_with = "option_decimal")]
    pub session_avg_price: Option<Decimal>,
    /// Delta
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub delta: Option<String>,
    /// Gamma
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub gamma: Option<String>,
    /// Vega
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub vega: Option<String>,
    /// Theta
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub theta: Option<String>,
    /// Cumulative realised pnl
    /// Futures & Perp: it is the all time cumulative realised P&L
    /// Option: it is the realised P&L when you hold that position
    pub cum_realised_pnl: Decimal,
    /// Position status. Normal, Liq, Adl
    pub position_status: PositionStatus,
    /// Auto-deleverage rank indicator. What is Auto-Deleveraging?
    pub adl_rank_indicator: AdlRankIndicator,
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

#[derive(PartialEq, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WalletMsg {
    /// Account type.
    /// UTA2.0: UNIFIED
    /// UTA1.0: UNIFIED (spot/linear/options), CONTRACT(inverse)
    /// Classic: CONTRACT, SPOT
    pub account_type: AccountType,
    /// Account IM rate
    /// You can refer to this Glossary to understand the below fields calculation and mearning
    /// All below account wide fields are not applicable to
    /// UTA2.0(isolated margin),
    /// UTA1.0(isolated margin), UTA1.0(CONTRACT),
    /// classic account(SPOT, CONTRACT)
    #[serde(rename = "accountIMRate")]
    pub account_im_rate: Decimal,
    /// Account MM rate
    #[serde(rename = "accountMMRate")]
    pub account_mm_rate: Decimal,
    /// Account total equity (USD)
    pub total_equity: Decimal,
    /// Account wallet balance (USD): ∑Asset Wallet Balance By USD value of each asset
    pub total_wallet_balance: Decimal,
    /// Account margin balance (USD): totalWalletBalance + totalPerpUPL
    pub total_margin_balance: Decimal,
    /// Account available balance (USD), Cross Margin: totalMarginBalance - totalInitialMargin
    pub total_available_balance: Decimal,
    /// Account Perps and Futures unrealised p&l (USD): ∑Each Perp and USDC Futures upl by base coin
    #[serde(rename = "totalPerpUPL")]
    pub total_perp_upl: Decimal,
    /// Account initial margin (USD): ∑Asset Total Initial Margin Base Coin
    pub total_initial_margin: Decimal,
    /// Account maintenance margin (USD): ∑ Asset Total Maintenance Margin Base Coin
    pub total_maintenance_margin: Decimal,
    /// You can ignore this field, and refer to accountIMRate, which has the same calculation
    #[serde(rename = "accountIMRateByMp")]
    pub account_im_rate_by_mp: Decimal,
    /// You can ignore this field, and refer to accountMMRate, which has the same calculation
    #[serde(rename = "accountMMRateByMp")]
    pub account_mm_rate_by_mp: Decimal,
    /// You can ignore this field, and refer to totalInitialMargin, which has the same calculation
    #[serde(rename = "totalInitialMarginByMp")]
    pub total_initial_margin_by_mp: Decimal,
    /// You can ignore this field, and refer to totalMaintenanceMargin, which has the same calculation
    #[serde(rename = "totalMaintenanceMarginByMp")]
    pub total_maintenance_margin_by_mp: Decimal,
    #[serde(deserialize_with = "hash_map")]
    pub coin: HashMap<String, WalletCoin>,
}

#[derive(PartialEq, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionMsg {
    /// Product type spot, linear, inverse, option
    pub category: Category,
    /// Symbol name
    pub symbol: String,
    /// Whether to borrow. 0: false, 1: true
    #[serde(default, deserialize_with = "string_to_bool")]
    pub is_leverage: bool,
    /// Order ID
    pub order_id: String,
    /// User customized order ID
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub order_link_id: Option<String>,
    /// Side. Buy,Sell
    pub side: Side,
    /// Order price
    pub order_price: Decimal,
    /// Order qty
    pub order_qty: Decimal,
    /// The remaining qty not executed
    pub leaves_qty: Decimal,
    /// Order create type
    /// Spot, Option do not have this key
    pub create_type: CreateType,
    /// Order type. Market,Limit
    pub order_type: OrderType,
    /// Stop order type. If the order is not stop order, any type is not returned
    pub stop_order_type: StopOrderType,
    /// Executed trading fee. You can get spot fee currency instruction here
    pub exec_fee: Decimal,
    /// Execution ID
    pub exec_id: String,
    /// Execution price
    pub exec_price: Decimal,
    /// Execution qty
    pub exec_qty: Decimal,
    /// Profit and Loss for each close position execution. The value keeps consistent with the field "cashFlow" in the Get Transaction Log
    pub exec_pnl: Decimal,
    /// Executed type
    pub exec_type: ExecType,
    /// Executed order value
    pub exec_value: Decimal,
    /// Executed timestamp (ms)
    #[serde(deserialize_with = "number")]
    pub exec_time: Timestamp,
    /// Is maker order. true: maker, false: taker
    pub is_maker: bool,
    /// Trading fee rate
    pub fee_rate: Decimal,
    /// Implied volatility. valid for option
    #[serde(default, deserialize_with = "option_decimal")]
    pub trade_iv: Option<Decimal>,
    /// Implied volatility of mark price. valid for option
    #[serde(default, deserialize_with = "option_decimal")]
    pub mark_iv: Option<Decimal>,
    /// The mark price of the symbol when executing. valid for option
    pub mark_price: Decimal,
    /// The index price of the symbol when executing. valid for option
    #[serde(default, deserialize_with = "option_decimal")]
    pub index_price: Option<Decimal>,
    /// The underlying price of the symbol when executing. valid for option
    #[serde(default, deserialize_with = "option_decimal")]
    pub underlying_price: Option<Decimal>,
    /// Paradigm block trade ID
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub block_trade_id: Option<String>,
    /// Closed position size
    pub closed_size: Decimal,
    /// Extra trading fee information. Currently, this data is returned only for kyc=Indian user or spot orders placed on the Indonesian site or spot fiat currency orders placed on the EU site. In other cases, an empty string is returned. Enum: feeType, subFeeType
    pub extra_fees: Option<Vec<ExtraFee>>, // TODO: !!! ignore if empty string !!!
    /// Cross sequence, used to associate each fill and each position update
    /// The seq will be the same when conclude multiple transactions at the same time
    /// Different symbols may have the same seq, please use seq + symbol to check unique
    pub seq: i64,
    /// Trading fee currency
    pub fee_currency: String,
}

#[derive(PartialEq, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExtraFee {
    pub fee_coin: String,
    pub fee_type: ExtraFeeType,
    pub sub_fee_type: ExtraSubFeeType,
    pub fee_rate: Decimal,
    pub fee: Decimal,
}

/// A fill from the `fastExecution` private WS topic.
///
/// This is a condensed view of an execution that arrives immediately when an
/// order is matched, before the normal settlement cycle that drives
/// [`ExecutionMsg`]. The field set is smaller than [`ExecutionMsg`].
#[derive(PartialEq, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FastExecutionMsg {
    pub category: Category,
    pub symbol: String,
    pub exec_id: String,
    pub exec_price: Decimal,
    pub exec_qty: Decimal,
    pub side: Side,
    pub order_qty: Decimal,
    pub order_id: String,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub order_link_id: Option<String>,
    pub is_maker: bool,
    pub fee_currency: String,
    pub fee_rate: Decimal,
    pub exec_fee: Decimal,
    #[serde(default, deserialize_with = "option_decimal")]
    pub trade_iv: Option<Decimal>,
    #[serde(default, deserialize_with = "option_decimal")]
    pub mark_iv: Option<Decimal>,
    pub mark_price: Decimal,
    #[serde(default, deserialize_with = "option_decimal")]
    pub index_price: Option<Decimal>,
    #[serde(default, deserialize_with = "option_decimal")]
    pub underlying_price: Option<Decimal>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub block_trade_id: Option<String>,
    #[serde(deserialize_with = "number")]
    pub exec_time: Timestamp,
}

/// A data point from the `greeks` private WS topic.
///
/// Sent once per second when there is an option position. Contains the
/// aggregate net greeks across all option positions for the base coin.
#[derive(PartialEq, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GreekMsg {
    pub base_coin: String,
    pub total_delta: Decimal,
    pub total_gamma: Decimal,
    pub total_vega: Decimal,
    pub total_theta: Decimal,
}

/// Data from the `dcp` private WS topic (Disconnection and Cancellation
/// Protection status update).
#[derive(PartialEq, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DcpMsg {
    /// Product class affected: e.g. `"OPTIONS"`, `"DERIVATIVES"`.
    pub product: String,
    /// Current DCP status. `"ON"` or `"OFF"`.
    pub dcp_status: String,
    /// The time window (seconds) configured for DCP.
    pub time_window: u64,
    /// Timestamp of the last status change (ms).
    #[serde(deserialize_with = "number")]
    pub updated_at: Timestamp,
}

#[cfg(test)]
mod tests {
    use rust_decimal::dec;

    use crate::DepthLevel;
    use crate::serde::{Unique, deserialize_json};

    use super::*;

    #[test]
    fn deserialize_incoming_message_command_subscribe() {
        let json = r#"{"success":true,"ret_msg":"","conn_id":"c0c928a4-daab-460d-b186-45e90a10a3d4","req_id":"","op":"subscribe"}"#;
        let expected = IncomingMessage::Command(CommandMsg::Subscribe {
            req_id: None,
            ret_msg: None,
            conn_id: String::from("c0c928a4-daab-460d-b186-45e90a10a3d4"),
            success: true,
        });

        let message = deserialize_json(json).unwrap();

        assert_eq!(expected, message);
    }

    #[test]
    fn deserialize_incoming_message_command_unsubscribe() {
        let json = r#"{"success":true,"ret_msg":"","conn_id":"c0c928a4-daab-460d-b186-45e90a10a3d4","req_id":"","op":"unsubscribe"}"#;
        let expected = IncomingMessage::Command(CommandMsg::Unsubscribe {
            req_id: None,
            ret_msg: None,
            conn_id: String::from("c0c928a4-daab-460d-b186-45e90a10a3d4"),
            success: true,
        });

        let message = deserialize_json(json).unwrap();

        assert_eq!(expected, message);
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
        let ticker_delta = TickerMsg::Delta {
            topic: Topic::Ticker(String::from("BTCUSDT")),
            cs: Some(195377749067),
            ts: 1718995014034,
            data: TickerDeltaMsg {
                symbol: String::from("BTCUSDT"),
                tick_direction: Some(TickDirection::PlusTick),
                last_price: Some(dec!(63948.5)),
                pre_open_price: None,
                pre_qty: None,
                cur_pre_listing_phase: None,
                prev_price24h: None,
                price24h_pcnt: Some(dec!(-0.015895)),
                high_price24h: None,
                low_price24h: None,
                prev_price1h: None,
                mark_price: None,
                index_price: None,
                open_interest: None,
                open_interest_value: None,
                turnover24h: Some(dec!(6793884423.5518)),
                volume24h: Some(dec!(105991.376)),
                funding_rate: None,
                next_funding_time: None,
                bid1_price: Some(dec!(63948.4)),
                bid1_size: Some(dec!(3.439)),
                ask1_price: Some(dec!(63948.5)),
                ask1_size: Some(dec!(2.566)),
                delivery_time: None,
                basis_rate: None,
                delivery_fee_rate: None,
                predicted_delivery_price: None,
            },
        };
        let expected = IncomingMessage::Ticker(Box::new(ticker_delta));

        let message = deserialize_json(json).unwrap();

        assert_eq!(expected, message);
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
        let ticker_snapshot = TickerMsg::Snapshot {
            topic: Topic::Ticker(String::from("BTCUSDT")),
            cs: Some(337149693308),
            ts: 1740622194359,
            data: TickerSnapshotMsg {
                symbol: String::from("BTCUSDT"),
                tick_direction: TickDirection::ZeroPlusTick,
                last_price: dec!(84594.40),
                pre_open_price: None,
                pre_qty: None,
                cur_pre_listing_phase: None,
                prev_price24h: dec!(88539.30),
                price24h_pcnt: dec!(-0.044555),
                high_price24h: dec!(89389.90),
                low_price24h: dec!(82055.60),
                prev_price1h: dec!(84307.20),
                mark_price: dec!(84594.00),
                index_price: dec!(84650.47),
                open_interest: dec!(52903.75),
                open_interest_value: dec!(4475339827.50),
                turnover24h: dec!(17166562011.6514),
                volume24h: dec!(200176.9910),
                funding_rate: dec!(-0.00016974),
                next_funding_time: 1740643200000,
                bid1_price: dec!(84594.30),
                bid1_size: dec!(6.777),
                ask1_price: dec!(84594.40),
                ask1_size: dec!(0.660),
                delivery_time: None,
                basis_rate: None,
                delivery_fee_rate: None,
                predicted_delivery_price: None,
            },
        };
        let expected = IncomingMessage::Ticker(Box::new(ticker_snapshot));

        let message = deserialize_json(json).unwrap();

        assert_eq!(expected, message);
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
        let expected = IncomingMessage::Trade(TradeMsg::Snapshot {
            id: None,
            topic: Topic::Trade(String::from("BTCUSDT")),
            ts: 1741433245359,
            data: vec![TradeSnapshotMsg {
                time: 1741433245357,
                symbol: String::from("BTCUSDT"),
                side: Side::Buy,
                size: dec!(0.007),
                price: dec!(85821.00),
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

        let message = deserialize_json(json).unwrap();

        assert_eq!(expected, message);
    }

    #[test]
    fn deserialize_incoming_message_orderbook_snapshot() {
        // Category: linear.
        let json = r#"{
            "topic":"orderbook.50.BTCUSDT",
            "type":"snapshot",
            "ts":1672304484978,
            "data":{
                "s":"BTCUSDT",
                "b":[
                    ["16493.50","0.006"],
                    ["16493.00","0.100"]
                ],
                "a":[
                    ["16611.00","0.029"],
                    ["16612.00","0.213"]
                ],
                "u":18521288,
                "seq":7961638724
            },
            "cts":1672304484976
        }"#;
        let expected = IncomingMessage::Orderbook(OrderbookMsg::Snapshot {
            topic: Topic::Orderbook {
                symbol: String::from("BTCUSDT"),
                depth: DepthLevel::Level50,
            },
            ts: 1672304484978,
            data: OrderbookDataMsg {
                symbol: String::from("BTCUSDT"),
                bids: vec![
                    OrderbookLevel {
                        price: dec!(16493.50),
                        size: dec!(0.006),
                    },
                    OrderbookLevel {
                        price: dec!(16493.00),
                        size: dec!(0.100),
                    },
                ],
                asks: vec![
                    OrderbookLevel {
                        price: dec!(16611.00),
                        size: dec!(0.029),
                    },
                    OrderbookLevel {
                        price: dec!(16612.00),
                        size: dec!(0.213),
                    },
                ],
                update_id: 18521288,
                seq: 7961638724,
            },
            cts: 1672304484976,
        });

        let message = deserialize_json(json).unwrap();

        assert_eq!(expected, message);
    }

    #[test]
    fn deserialize_incoming_message_orderbook_delta() {
        // Category: linear.
        let json = r#"{
            "topic":"orderbook.50.BTCUSDT",
            "type":"delta",
            "ts":1687940967466,
            "data":{
                "s":"BTCUSDT",
                "b":[
                    ["30247.20","30.028"],
                    ["30245.40","0.224"],
                    ["30242.10","0.000"]
                ],
                "a":[
                    ["30248.67","0.033"],
                    ["30249.20","0.000"]
                ],
                "u":177400507,
                "seq":66544703342
            },
            "cts":1687940967464
        }"#;
        let expected = IncomingMessage::Orderbook(OrderbookMsg::Delta {
            topic: Topic::Orderbook {
                symbol: String::from("BTCUSDT"),
                depth: DepthLevel::Level50,
            },
            ts: 1687940967466,
            data: OrderbookDataMsg {
                symbol: String::from("BTCUSDT"),
                bids: vec![
                    OrderbookLevel {
                        price: dec!(30247.20),
                        size: dec!(30.028),
                    },
                    OrderbookLevel {
                        price: dec!(30245.40),
                        size: dec!(0.224),
                    },
                    OrderbookLevel {
                        price: dec!(30242.10),
                        size: dec!(0.000),
                    },
                ],
                asks: vec![
                    OrderbookLevel {
                        price: dec!(30248.67),
                        size: dec!(0.033),
                    },
                    OrderbookLevel {
                        price: dec!(30249.20),
                        size: dec!(0.000),
                    },
                ],
                update_id: 177400507,
                seq: 66544703342,
            },
            cts: 1687940967464,
        });

        let message = deserialize_json(json).unwrap();

        assert_eq!(expected, message);
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
        let expected = AllLiquidationMsg::Snapshot {
            topic: Topic::AllLiquidation(String::from("BTCUSDT")),
            ts: 1741450605553,
            data: vec![AllLiquidationSnapshotMsg {
                time: 1741450605236,
                symbol: String::from("BTCUSDT"),
                side: Side::Buy,
                size: dec!(0.001),
                price: dec!(85823.60),
            }],
        };

        let message = deserialize_json(json).unwrap();

        assert_eq!(expected, message);
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
                    "feeCurrency": "",
                    "cumFeeDetail": {
                        "MNT": "0.00242968"
                    }
                }
            ]
        }"#;
        let order = PrivateMsg {
            id: String::from("5923240c6880ab-c59f-420b-9adb-3639adc9dd90"),
            creation_time: 1672364262474,
            data: vec![OrderMsg {
                category: Category::Option,
                order_id: String::from("5cf98598-39a7-459e-97bf-76ca765ee020"),
                order_link_id: None,
                is_leverage: None,
                block_trade_id: None,
                symbol: String::from("ETH-30DEC22-1400-C"),
                price: dec!(72.5),
                broker_order_price: None,
                qty: dec!(1.0),
                side: Side::Sell,
                position_idx: PositionIdx::OneWay,
                order_status: OrderStatus::Filled,
                create_type: None,
                cancel_type: CancelType::UNKNOWN,
                reject_reason: RejectReason::EcNoError,
                avg_price: Some(dec!(75.0)),
                leaves_qty: None,
                leaves_value: None,
                cum_exec_qty: dec!(1.0),
                cum_exec_value: dec!(75.0),
                cum_exec_fee: dec!(0.358635),
                closed_pnl: dec!(0.0),
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
                place_type: Some(PlaceType::Price),
                smp_type: SmpType::None,
                smp_group: 0,
                smp_order_id: None,
                created_time: 1672364262444,
                updated_time: 1672364262457,
                cum_fee_detail: Some(serde_json::from_str(r#"{"MNT": "0.00242968"}"#).unwrap()),
            }],
        };
        let expected = IncomingMessage::Topic(TopicMessage::Order(order));

        let message = deserialize_json(json).unwrap();

        assert_eq!(expected, message);
    }

    #[test]
    fn deserialize_incoming_message_order2() {
        let json = r#"{"topic":"order","id":"108985347_ADAUSDT_140667095077548","creationTime":1766436947942,"data":[{"category":"linear","symbol":"ADAUSDT","orderId":"ae802ad5-af70-4957-ba72-86ad7fc9c24d","orderLinkId":"","blockTradeId":"","side":"Buy","positionIdx":0,"orderStatus":"Filled","cancelType":"UNKNOWN","rejectReason":"EC_NoError","timeInForce":"IOC","isLeverage":"","price":"0.3862","qty":"15","avgPrice":"0.3679","leavesQty":"0","leavesValue":"0","cumExecQty":"15","cumExecValue":"5.5185","cumExecFee":"0.00303518","orderType":"Market","stopOrderType":"","orderIv":"","triggerPrice":"","takeProfit":"","stopLoss":"","triggerBy":"","tpTriggerBy":"","slTriggerBy":"","triggerDirection":0,"placeType":"","lastPriceOnCreated":"0.3679","closeOnTrigger":false,"reduceOnly":false,"smpGroup":0,"smpType":"None","smpOrderId":"","slLimitPrice":"0","tpLimitPrice":"0","tpslMode":"UNKNOWN","createType":"CreateByUser","marketUnit":"","createdTime":"1766436947940","updatedTime":"1766436947940","feeCurrency":"","closedPnl":"0","slippageTolerance":"0","slippageToleranceType":"UNKNOWN","cumFeeDetail":{}}]}"#;
        let order = PrivateMsg {
            id: String::from("108985347_ADAUSDT_140667095077548"),
            creation_time: 1766436947942,
            data: vec![OrderMsg {
                category: Category::Linear,
                order_id: String::from("ae802ad5-af70-4957-ba72-86ad7fc9c24d"),
                order_link_id: None,
                is_leverage: None,
                block_trade_id: None,
                symbol: String::from("ADAUSDT"),
                price: dec!(0.3862),
                broker_order_price: None,
                qty: dec!(15),
                side: Side::Buy,
                position_idx: PositionIdx::OneWay,
                order_status: OrderStatus::Filled,
                create_type: Some(CreateType::CreateByUser),
                cancel_type: CancelType::UNKNOWN,
                reject_reason: RejectReason::EcNoError,
                avg_price: Some(dec!(0.3679)),
                leaves_qty: Some(dec!(0)),
                leaves_value: Some(dec!(0)),
                cum_exec_qty: dec!(15),
                cum_exec_value: dec!(5.5185),
                cum_exec_fee: dec!(0.00303518),
                closed_pnl: dec!(0),
                fee_currency: None,
                time_in_force: TimeInForce::IOC,
                order_type: OrderType::Market,
                stop_order_type: None,
                oco_trigger_by: None,
                order_iv: None,
                market_unit: None,
                slippage_tolerance_type: Some(SlippageToleranceType::UNKNOWN),
                slippage_tolerance: Some(dec!(0)),
                trigger_price: None,
                take_profit: None,
                stop_loss: None,
                tpsl_mode: Some(TpslMode::UNKNOWN),
                tp_limit_price: Some(dec!(0)),
                sl_limit_price: Some(dec!(0)),
                tp_trigger_by: None,
                sl_trigger_by: None,
                trigger_direction: TriggerDirection::UNKNOWN,
                trigger_by: None,
                last_price_on_created: Some(dec!(0.3679)),
                reduce_only: false,
                close_on_trigger: false,
                place_type: None,
                smp_type: SmpType::None,
                smp_group: 0,
                smp_order_id: None,
                created_time: 1766436947940,
                updated_time: 1766436947940,
                cum_fee_detail: Some(serde_json::from_str(r#"{}"#).unwrap()),
            }],
        };
        let expected = IncomingMessage::Topic(TopicMessage::Order(order));

        let message = deserialize_json(json).unwrap();

        assert_eq!(expected, message);
    }

    #[test]
    fn deserialize_incoming_message_order3() {
        let json = r#"{"topic":"order","id":"108985347_ADAUSDT_140667102632416","creationTime":1766600379878,"data":[{"category":"linear","symbol":"ADAUSDT","orderId":"f0468cbc-ed2f-4fd7-9620-998f3e9f387c","orderLinkId":"BOT_LINK_ID-1","blockTradeId":"","side":"Buy","positionIdx":0,"orderStatus":"New","cancelType":"UNKNOWN","rejectReason":"EC_NoError","timeInForce":"GTC","isLeverage":"","price":"0.3539","qty":"15","avgPrice":"","leavesQty":"15","leavesValue":"5.3085","cumExecQty":"0","cumExecValue":"0","cumExecFee":"0","orderType":"Limit","stopOrderType":"","orderIv":"","triggerPrice":"","takeProfit":"","stopLoss":"","triggerBy":"","tpTriggerBy":"","slTriggerBy":"","triggerDirection":0,"placeType":"","lastPriceOnCreated":"0.355","closeOnTrigger":false,"reduceOnly":false,"smpGroup":0,"smpType":"None","smpOrderId":"","slLimitPrice":"0","tpLimitPrice":"0","tpslMode":"UNKNOWN","createType":"CreateByUser","marketUnit":"","createdTime":"1766600379876","updatedTime":"1766600379876","feeCurrency":"","closedPnl":"0","slippageTolerance":"0","slippageToleranceType":"UNKNOWN","cumFeeDetail":{}}]}"#;
        let order = PrivateMsg {
            id: String::from("108985347_ADAUSDT_140667102632416"),
            creation_time: 1766600379878,
            data: vec![OrderMsg {
                category: Category::Linear,
                order_id: String::from("f0468cbc-ed2f-4fd7-9620-998f3e9f387c"),
                order_link_id: Some(String::from("BOT_LINK_ID-1")),
                is_leverage: None,
                block_trade_id: None,
                symbol: String::from("ADAUSDT"),
                price: dec!(0.3539),
                broker_order_price: None,
                qty: dec!(15),
                side: Side::Buy,
                position_idx: PositionIdx::OneWay,
                order_status: OrderStatus::New,
                create_type: Some(CreateType::CreateByUser),
                cancel_type: CancelType::UNKNOWN,
                reject_reason: RejectReason::EcNoError,
                avg_price: None,
                leaves_qty: Some(dec!(15)),
                leaves_value: Some(dec!(5.3085)),
                cum_exec_qty: dec!(0),
                cum_exec_value: dec!(0),
                cum_exec_fee: dec!(0),
                closed_pnl: dec!(0),
                fee_currency: None,
                time_in_force: TimeInForce::GTC,
                order_type: OrderType::Limit,
                stop_order_type: None,
                oco_trigger_by: None,
                order_iv: None,
                market_unit: None,
                slippage_tolerance_type: Some(SlippageToleranceType::UNKNOWN),
                slippage_tolerance: Some(dec!(0)),
                trigger_price: None,
                take_profit: None,
                stop_loss: None,
                tpsl_mode: Some(TpslMode::UNKNOWN),
                tp_limit_price: Some(dec!(0)),
                sl_limit_price: Some(dec!(0)),
                tp_trigger_by: None,
                sl_trigger_by: None,
                trigger_direction: TriggerDirection::UNKNOWN,
                trigger_by: None,
                last_price_on_created: Some(dec!(0.355)),
                reduce_only: false,
                close_on_trigger: false,
                place_type: None, // "smpGroup":0,"smpType":"None","smpOrderId":"",
                smp_type: SmpType::None,
                smp_group: 0,
                smp_order_id: None,
                created_time: 1766600379876,
                updated_time: 1766600379876,
                cum_fee_detail: Some(serde_json::from_str(r#"{}"#).unwrap()),
            }],
        };
        let expected = IncomingMessage::Topic(TopicMessage::Order(order));

        let message = deserialize_json(json).unwrap();

        assert_eq!(expected, message);
    }

    #[test]
    fn deserialize_incoming_message_position() {
        let json = r#"{
            "id": "108985347_position_1765659601915",
            "topic": "position",
            "creationTime": 1765659601915,
            "data": [
                {
                    "positionIdx": 1,
                    "tradeMode": 0,
                    "riskId": 116,
                    "riskLimitValue": "200000",
                    "symbol": "ADAUSDT",
                    "side": "Buy",
                    "size": "18720",
                    "entryPrice": "0.41160027",
                    "sessionAvgPrice": "",
                    "leverage": "75",
                    "positionValue": "7705.157",
                    "positionBalance": "0",
                    "markPrice": "0.41",
                    "positionIM": "106.51735757",
                    "positionMM": "61.74535757",
                    "positionIMByMp": "106.51735757",
                    "positionMMByMp": "61.74535757",
                    "takeProfit": "0.4321",
                    "stopLoss": "0.3704",
                    "trailingStop": "0",
                    "unrealisedPnl": "-29.957",
                    "cumRealisedPnl": "-6712.87804378",
                    "curRealisedPnl": "-2.6317147",
                    "createdTime": "1714594321840",
                    "updatedTime": "1765645142548",
                    "tpslMode": "Full",
                    "liqPrice": "0.37000066",
                    "bustPrice": "",
                    "category": "linear",
                    "positionStatus": "Normal",
                    "adlRankIndicator": 2,
                    "autoAddMargin": 0,
                    "leverageSysUpdatedTime": "",
                    "mmrSysUpdatedTime": "",
                    "seq": 140667058318085,
                    "isReduceOnly": false
                },
                {
                    "positionIdx": 2,
                    "tradeMode": 0,
                    "riskId": 116,
                    "riskLimitValue": "200000",
                    "symbol": "ADAUSDT",
                    "side": "",
                    "size": "0",
                    "entryPrice": "0",
                    "sessionAvgPrice": "",
                    "leverage": "75",
                    "positionValue": "0",
                    "positionBalance": "0",
                    "markPrice": "0.41",
                    "positionIM": "",
                    "positionMM": "",
                    "positionIMByMp": "",
                    "positionMMByMp": "",
                    "takeProfit": "0",
                    "stopLoss": "0",
                    "trailingStop": "0",
                    "unrealisedPnl": "0",
                    "cumRealisedPnl": "1618.30675974",
                    "curRealisedPnl": "0",
                    "createdTime": "1714594321840",
                    "updatedTime": "1765046350698",
                    "tpslMode": "Full",
                    "liqPrice": "0",
                    "bustPrice": "",
                    "category": "linear",
                    "positionStatus": "Normal",
                    "adlRankIndicator": 0,
                    "autoAddMargin": 0,
                    "leverageSysUpdatedTime": "",
                    "mmrSysUpdatedTime": "",
                    "seq": 140667031311361,
                    "isReduceOnly": false
                }
            ]
        }"#;
        let position = PrivateMsg {
            id: String::from("108985347_position_1765659601915"),
            creation_time: 1765659601915,
            data: vec![
                PositionMsg {
                    category: Category::Linear,
                    symbol: String::from("ADAUSDT"),
                    side: Some(Side::Buy),
                    size: dec!(18720),
                    position_idx: PositionIdx::Buy,
                    position_value: dec!(7705.157),
                    risk_id: 116,
                    risk_limit_value: Some(dec!(200000)),
                    entry_price: dec!(0.41160027),
                    mark_price: dec!(0.41),
                    leverage: dec!(75),
                    auto_add_margin: false,
                    position_im: Some(dec!(106.51735757)),
                    position_mm: Some(dec!(61.74535757)),
                    position_im_by_mp: Some(dec!(106.51735757)),
                    position_mm_by_mp: Some(dec!(61.74535757)),
                    liq_price: Some(dec!(0.37000066)),
                    take_profit: dec!(0.4321),
                    stop_loss: dec!(0.3704),
                    trailing_stop: dec!(0),
                    unrealised_pnl: dec!(-29.957),
                    cur_realised_pnl: dec!(-2.6317147),
                    session_avg_price: None,
                    delta: None,
                    gamma: None,
                    vega: None,
                    theta: None,
                    cum_realised_pnl: dec!(-6712.87804378),
                    position_status: PositionStatus::Normal,
                    adl_rank_indicator: AdlRankIndicator::Two,
                    is_reduce_only: false,
                    mmr_sys_updated_time: None,
                    leverage_sys_updated_time: None,
                    created_time: 1714594321840,
                    updated_time: 1765645142548,
                    seq: 140667058318085,
                },
                PositionMsg {
                    category: Category::Linear,
                    symbol: String::from("ADAUSDT"),
                    side: None,
                    size: dec!(0),
                    position_idx: PositionIdx::Sell,
                    position_value: dec!(0),
                    risk_id: 116,
                    risk_limit_value: Some(dec!(200000)),
                    entry_price: dec!(0),
                    mark_price: dec!(0.41),
                    leverage: dec!(75),
                    auto_add_margin: false,
                    position_im: None,
                    position_mm: None,
                    position_im_by_mp: None,
                    position_mm_by_mp: None,
                    liq_price: Some(dec!(0)),
                    take_profit: dec!(0),
                    stop_loss: dec!(0),
                    trailing_stop: dec!(0),
                    unrealised_pnl: dec!(0),
                    cur_realised_pnl: dec!(0),
                    session_avg_price: None,
                    delta: None,
                    gamma: None,
                    vega: None,
                    theta: None,
                    cum_realised_pnl: dec!(1618.30675974),
                    position_status: PositionStatus::Normal,
                    adl_rank_indicator: AdlRankIndicator::Zero,
                    is_reduce_only: false,
                    mmr_sys_updated_time: None,
                    leverage_sys_updated_time: None,
                    created_time: 1714594321840,
                    updated_time: 1765046350698,
                    seq: 140667031311361,
                },
            ],
        };
        let expected = IncomingMessage::Topic(TopicMessage::Position(position));

        let message = deserialize_json(json).unwrap();

        assert_eq!(expected, message);
    }

    #[test]
    fn deserialize_incoming_message_position2() {
        let json = r#"{
            "id":"108985347_position_1766316605952",
            "topic":"position",
            "creationTime":1766316605952,
            "data":[
                {
                "positionIdx":1,
                "tradeMode":0,
                "riskId":116,
                "riskLimitValue":"200000",
                "symbol":"ADAUSDT",
                "side":"Buy",
                "size":"43",
                "entryPrice":"0.37293023",
                "sessionAvgPrice":"",
                "leverage":"75",
                "positionValue":"16.036",
                "positionBalance":"0",
                "markPrice":"0.3702",
                "positionIM":"0.22095025",
                "positionMM":"0.12809175",
                "positionIMByMp":"0.22095025",
                "positionMMByMp":"0.12809175",
                "takeProfit":"0",
                "stopLoss":"0",
                "trailingStop":"0",
                "unrealisedPnl":"-0.1174",
                "cumRealisedPnl":"-7547.8530836",
                "curRealisedPnl":"-0.00465061",
                "createdTime":"1714594321840",
                "updatedTime":"1766313370061",
                "tpslMode":"Full",
                "liqPrice":"",
                "bustPrice":"",
                "category":"linear",
                "positionStatus":"Normal",
                "adlRankIndicator":2,
                "autoAddMargin":0,
                "leverageSysUpdatedTime":"",
                "mmrSysUpdatedTime":"",
                "seq":140667089523042,
                "isReduceOnly":false
                },
                {
                "positionIdx":2,
                "tradeMode":0,
                "riskId":116,
                "riskLimitValue":"200000",
                "symbol":"ADAUSDT",
                "side":"",
                "size":"0",
                "entryPrice":"0",
                "sessionAvgPrice":"",
                "leverage":"75",
                "positionValue":"0",
                "positionBalance":"0",
                "markPrice":"0.3702",
                "positionIM":"",
                "positionMM":"",
                "positionIMByMp":"",
                "positionMMByMp":"",
                "takeProfit":"0",
                "stopLoss":"0",
                "trailingStop":"0",
                "unrealisedPnl":"0",
                "cumRealisedPnl":"1618.30675974",
                "curRealisedPnl":"0",
                "createdTime":"1714594321840",
                "updatedTime":"1765046350698",
                "tpslMode":"Full",
                "liqPrice":"0",
                "bustPrice":"",
                "category":"linear",
                "positionStatus":"Normal",
                "adlRankIndicator":0,
                "autoAddMargin":0,
                "leverageSysUpdatedTime":"",
                "mmrSysUpdatedTime":"",
                "seq":140667031311361,
                "isReduceOnly":false
                }
            ]
        }"#;
        let position = PrivateMsg {
            id: String::from("108985347_position_1766316605952"),
            creation_time: 1766316605952,
            data: vec![
                PositionMsg {
                    category: Category::Linear,
                    symbol: String::from("ADAUSDT"),
                    side: Some(Side::Buy),
                    size: dec!(43),
                    position_idx: PositionIdx::Buy,
                    position_value: dec!(16.036),
                    risk_id: 116,
                    risk_limit_value: Some(dec!(200000)),
                    entry_price: dec!(0.37293023),
                    mark_price: dec!(0.3702),
                    leverage: dec!(75),
                    auto_add_margin: false,
                    position_im: Some(dec!(0.22095025)),
                    position_mm: Some(dec!(0.12809175)),
                    position_im_by_mp: Some(dec!(0.22095025)),
                    position_mm_by_mp: Some(dec!(0.12809175)),
                    liq_price: None,
                    take_profit: dec!(0),
                    stop_loss: dec!(0),
                    trailing_stop: dec!(0),
                    unrealised_pnl: dec!(-0.1174),
                    cur_realised_pnl: dec!(-0.00465061),
                    session_avg_price: None,
                    delta: None,
                    gamma: None,
                    vega: None,
                    theta: None,
                    cum_realised_pnl: dec!(-7547.8530836),
                    position_status: PositionStatus::Normal,
                    adl_rank_indicator: AdlRankIndicator::Two,
                    is_reduce_only: false,
                    mmr_sys_updated_time: None,
                    leverage_sys_updated_time: None,
                    created_time: 1714594321840,
                    updated_time: 1766313370061,
                    seq: 140667089523042,
                },
                PositionMsg {
                    category: Category::Linear,
                    symbol: String::from("ADAUSDT"),
                    side: None,
                    size: dec!(0),
                    position_idx: PositionIdx::Sell,
                    position_value: dec!(0),
                    risk_id: 116,
                    risk_limit_value: Some(dec!(200000)),
                    entry_price: dec!(0),
                    mark_price: dec!(0.3702),
                    leverage: dec!(75),
                    auto_add_margin: false,
                    position_im: None,
                    position_mm: None,
                    position_im_by_mp: None,
                    position_mm_by_mp: None,
                    liq_price: Some(dec!(0)),
                    take_profit: dec!(0),
                    stop_loss: dec!(0),
                    trailing_stop: dec!(0),
                    unrealised_pnl: dec!(0),
                    cur_realised_pnl: dec!(0),
                    session_avg_price: None,
                    delta: None,
                    gamma: None,
                    vega: None,
                    theta: None,
                    cum_realised_pnl: dec!(1618.30675974),
                    position_status: PositionStatus::Normal,
                    adl_rank_indicator: AdlRankIndicator::Zero,
                    is_reduce_only: false,
                    mmr_sys_updated_time: None,
                    leverage_sys_updated_time: None,
                    created_time: 1714594321840,
                    updated_time: 1765046350698,
                    seq: 140667031311361,
                },
            ],
        };
        let expected = IncomingMessage::Topic(TopicMessage::Position(position));

        let message = deserialize_json(json).unwrap();

        assert_eq!(expected, message);
    }

    #[test]
    fn deserialize_incoming_message_wallet() {
        let json = r#"{
            "id": "592324d2bce751-ad38-48eb-8f42-4671d1fb4d4e",
            "topic": "wallet",
            "creationTime": 1700034722104,
            "data": [
                {
                    "accountIMRate": "0",
                    "accountIMRateByMp": "0",
                    "accountMMRate": "0",
                    "accountMMRateByMp": "0",
                    "totalEquity": "10262.91335023",
                    "totalWalletBalance": "9684.46297164",
                    "totalMarginBalance": "9684.46297164",
                    "totalAvailableBalance": "9556.6056555",
                    "totalPerpUPL": "0",
                    "totalInitialMargin": "0",
                    "totalInitialMarginByMp": "0",
                    "totalMaintenanceMargin": "0",
                    "totalMaintenanceMarginByMp": "0",
                    "coin": [
                        {
                            "coin": "BTC",
                            "equity": "0.00102964",
                            "usdValue": "36.70759517",
                            "walletBalance": "0.00102964",
                            "availableToWithdraw": "0.00102964",
                            "availableToBorrow": "",
                            "borrowAmount": "0",
                            "accruedInterest": "0",
                            "totalOrderIM": "",
                            "totalPositionIM": "",
                            "totalPositionMM": "",
                            "unrealisedPnl": "0",
                            "cumRealisedPnl": "-0.00000973",
                            "bonus": "0",
                            "collateralSwitch": true,
                            "marginCollateral": true,
                            "locked": "0",
                            "spotHedgingQty": "0.01592413",
                            "spotBorrow": "0"
                        }
                    ],
                    "accountLTV": "0",
                    "accountType": "UNIFIED"
                }
            ]
        }"#;
        let coin = WalletCoin {
            coin: String::from("BTC"),
            equity: dec!(0.00102964),
            usd_value: dec!(36.70759517),
            wallet_balance: dec!(0.00102964),
            locked: dec!(0),
            spot_hedging_qty: dec!(0.01592413),
            borrow_amount: dec!(0),
            accrued_interest: dec!(0),
            total_order_im: None,
            total_position_im: None,
            total_position_mm: None,
            unrealised_pnl: dec!(0),
            cum_realised_pnl: dec!(-0.00000973),
            bonus: dec!(0),
            collateral_switch: true,
            margin_collateral: true,
            spot_borrow: Some(dec!(0)),
        };
        let coin = HashMap::from([(Unique::unique_key(&coin), coin)]);
        let wallet = PrivateMsg {
            id: String::from("592324d2bce751-ad38-48eb-8f42-4671d1fb4d4e"),
            creation_time: 1700034722104,
            data: vec![WalletMsg {
                account_type: AccountType::UNIFIED,
                account_im_rate: dec!(0),
                account_im_rate_by_mp: dec!(0),
                account_mm_rate: dec!(0),
                account_mm_rate_by_mp: dec!(0),
                total_equity: dec!(10262.91335023),
                total_wallet_balance: dec!(9684.46297164),
                total_margin_balance: dec!(9684.46297164),
                total_available_balance: dec!(9556.6056555),
                total_perp_upl: dec!(0),
                total_initial_margin: dec!(0),
                total_initial_margin_by_mp: dec!(0),
                total_maintenance_margin: dec!(0),
                total_maintenance_margin_by_mp: dec!(0),
                coin,
            }],
        };
        let expected = IncomingMessage::Topic(TopicMessage::Wallet(wallet));

        let message = deserialize_json(json).unwrap();

        assert_eq!(expected, message);
    }

    #[test]
    fn deserialize_incoming_message_wallet2() {
        let json = r#"{
            "id":"108985347_wallet_1766318882965",
            "topic":"wallet",
            "creationTime":1766318882964,
            "data":[
                {
                "accountIMRate":"0.0007",
                "accountMMRate":"0.0004",
                "accountIMRateByMp":"0.0007",
                "accountMMRateByMp":"0.0004",
                "totalEquity":"102.7094181",
                "totalWalletBalance":"102.16591975",
                "totalMarginBalance":"102.16591975",
                "totalAvailableBalance":"102.09402758",
                "totalPerpUPL":"0",
                "totalInitialMargin":"0.07189217",
                "totalMaintenanceMargin":"0.04166941",
                "totalInitialMarginByMp":"0.07189217",
                "totalMaintenanceMarginByMp":"0.04166941",
                "coin":[
                    {
                    "coin":"USDT",
                    "equity":"75.5601152",
                    "usdValue":"75.53450032",
                    "walletBalance":"75.5601152",
                    "availableToWithdraw":"",
                    "availableToBorrow":"",
                    "borrowAmount":"0",
                    "accruedInterest":"0",
                    "totalOrderIM":"0",
                    "totalPositionIM":"0.07191655",
                    "totalPositionMM":"0.04168355",
                    "unrealisedPnl":"0",
                    "cumRealisedPnl":"36163.8134634",
                    "bonus":"0",
                    "collateralSwitch":true,
                    "marginCollateral":true,
                    "locked":"0",
                    "spotHedgingQty":"0"
                    }
                ],
                "accountLTV":"0",
                "accountType":"UNIFIED"
                }
            ]
            }"#;
        let coin = WalletCoin {
            coin: String::from("USDT"),
            equity: dec!(75.5601152),
            usd_value: dec!(75.53450032),
            wallet_balance: dec!(75.5601152),
            locked: dec!(0),
            spot_hedging_qty: dec!(0),
            borrow_amount: dec!(0),
            accrued_interest: dec!(0),
            total_order_im: Some(dec!(0)),
            total_position_im: Some(dec!(0.07191655)),
            total_position_mm: Some(dec!(0.04168355)),
            unrealised_pnl: dec!(0),
            cum_realised_pnl: dec!(36163.8134634),
            bonus: dec!(0),
            collateral_switch: true,
            margin_collateral: true,
            spot_borrow: None,
        };
        let coin = HashMap::from([(Unique::unique_key(&coin), coin)]);
        let wallet = PrivateMsg {
            id: String::from("108985347_wallet_1766318882965"),
            creation_time: 1766318882964,
            data: vec![WalletMsg {
                account_type: AccountType::UNIFIED,
                account_im_rate: dec!(0.0007),
                account_im_rate_by_mp: dec!(0.0007),
                account_mm_rate: dec!(0.0004),
                account_mm_rate_by_mp: dec!(0.0004),
                total_equity: dec!(102.7094181),
                total_wallet_balance: dec!(102.16591975),
                total_margin_balance: dec!(102.16591975),
                total_available_balance: dec!(102.09402758),
                total_perp_upl: dec!(0),
                total_initial_margin: dec!(0.07189217),
                total_initial_margin_by_mp: dec!(0.07189217),
                total_maintenance_margin: dec!(0.04166941),
                total_maintenance_margin_by_mp: dec!(0.04166941),
                coin,
            }],
        };
        let expected = IncomingMessage::Topic(TopicMessage::Wallet(wallet));

        let message = deserialize_json(json).unwrap();

        assert_eq!(expected, message);
    }

    #[test]
    fn deserialize_incoming_message_execution() {
        let json = r#"{
            "topic": "execution",
            "id": "386825804_BTCUSDT_140612148849382",
            "creationTime": 1746270400355,
            "data": [
                {
                    "category": "linear",
                    "symbol": "BTCUSDT",
                    "closedSize": "0.5",
                    "execFee": "26.3725275",
                    "execId": "0ab1bdf7-4219-438b-b30a-32ec863018f7",
                    "execPrice": "95900.1",
                    "execQty": "0.5",
                    "execType": "Trade",
                    "execValue": "47950.05",
                    "feeRate": "0.00055",
                    "tradeIv": "",
                    "markIv": "",
                    "blockTradeId": "",
                    "markPrice": "95901.48",
                    "indexPrice": "",
                    "underlyingPrice": "",
                    "leavesQty": "0",
                    "orderId": "9aac161b-8ed6-450d-9cab-c5cc67c21784",
                    "orderLinkId": "",
                    "orderPrice": "94942.5",
                    "orderQty": "0.5",
                    "orderType": "Market",
                    "stopOrderType": "UNKNOWN",
                    "side": "Sell",
                    "execTime": "1746270400353",
                    "isLeverage": "0",
                    "isMaker": false,
                    "seq": 140612148849382,
                    "marketUnit": "",
                    "execPnl": "0.05",
                    "createType": "CreateByUser",
                    "extraFees":[{"feeCoin":"USDT","feeType":"GST","subFeeType":"IND_GST","feeRate":"0.0000675","fee":"0.006403779"}],
                    "feeCurrency": "USDT"
                }
            ]
        }"#;
        let execution = PrivateMsg {
            id: String::from("386825804_BTCUSDT_140612148849382"),
            creation_time: 1746270400355,
            data: vec![ExecutionMsg {
                category: Category::Linear,
                symbol: String::from("BTCUSDT"),
                is_leverage: false,
                order_id: String::from("9aac161b-8ed6-450d-9cab-c5cc67c21784"),
                order_link_id: None,
                side: Side::Sell,
                order_price: dec!(94942.5),
                order_qty: dec!(0.5),
                leaves_qty: dec!(0),
                create_type: CreateType::CreateByUser,
                order_type: OrderType::Market,
                stop_order_type: StopOrderType::UNKNOWN,
                exec_fee: dec!(26.3725275),
                exec_id: String::from("0ab1bdf7-4219-438b-b30a-32ec863018f7"),
                exec_price: dec!(95900.1),
                exec_qty: dec!(0.5),
                exec_pnl: dec!(0.05),
                exec_type: ExecType::Trade,
                exec_value: dec!(47950.05),
                exec_time: 1746270400353,
                is_maker: false,
                fee_rate: dec!(0.00055),
                trade_iv: None,
                mark_iv: None,
                mark_price: dec!(95901.48),
                index_price: None,
                underlying_price: None,
                block_trade_id: None,
                closed_size: dec!(0.5),
                extra_fees: Some(vec![ExtraFee {
                    fee_coin: String::from("USDT"),
                    fee_type: ExtraFeeType::Gst,
                    sub_fee_type: ExtraSubFeeType::IndGst,
                    fee_rate: dec!(0.0000675),
                    fee: dec!(0.006403779),
                }]),
                seq: 140612148849382,
                fee_currency: String::from("USDT"),
            }],
        };
        let expected = IncomingMessage::Topic(TopicMessage::Execution(execution));

        let message = deserialize_json(json).unwrap();

        assert_eq!(expected, message);
    }
}
