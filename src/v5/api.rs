use std::collections::HashMap;

use rust_decimal::{Decimal, serde::str_option::deserialize as option_decimal};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::{
    deserialize_number_from_string as number,
    deserialize_option_number_from_string as option_number,
};

use crate::v5::{
    AccountType, AdlRankIndicator, CancelType, ContractType, CopyTrading, CreateType,
    CurAuctionPhase, DCPProduct, MarginMode, OcoTriggerBy, OrderMsg, OrderStatus, OrderType,
    PlaceType, PositionIdx, PositionMsg, PositionStatus, RejectReason, Side, SmpType,
    SpotHedgingStatus, Status, TimeInForce, TpslMode, TriggerBy, TriggerDirection,
    UnifiedMarginStatus, VipLevel, WalletMsg,
    enums::{Category, Interval, StopOrderType},
    serde::{
        Unique, empty_string_as_none, hash_map, int_to_bool, invalid_as_none, string_to_bool,
        string_to_option_bool,
    },
};

pub type Timestamp = u64;
pub type Second = u64;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Resp<T> {
    pub ret_code: i64,
    pub ret_msg: String,
    pub result: T,
    pub time: Option<Timestamp>,
    pub ret_ext_info: Option<RetExtInfo>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct APIErrorResponse {
    pub ret_code: i64,
    pub ret_msg: String,
}

#[derive(Debug, PartialEq)]
pub struct Response<T> {
    pub result: T,
    pub time: Option<Timestamp>,
    pub headers: Headers,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CursorPagination<T> {
    pub category: Option<Category>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub next_page_cursor: Option<String>,
    pub list: Vec<T>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct List<T> {
    pub list: Vec<T>,
}

#[derive(Debug, PartialEq)]
pub struct Headers {
    pub ret_code: Option<i32>,
    pub trace_id: Option<String>,
    pub time_now: Option<Timestamp>,
    pub api_limit: Option<u64>,
    pub api_limit_status: Option<u64>,
    pub api_limit_reset_timestamp: Option<Timestamp>,
}

impl Headers {
    pub fn is_ret_code_ok(&self) -> bool {
        match self.ret_code {
            Some(code) => code == 0,
            None => false,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct RetExtInfo {}

#[derive(Serialize)]
pub struct GetKLinesParams {
    pub category: Category,
    pub symbol: String,
    pub interval: Interval,
    pub start: Option<Timestamp>,
    pub end: Option<Timestamp>,
    pub limit: Option<u64>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "category")]
pub enum KLine {
    #[serde(rename = "inverse")]
    Inverse { symbol: String, list: Vec<KLineRow> },
    #[serde(rename = "linear")]
    Linear { symbol: String, list: Vec<KLineRow> },
    #[serde(rename = "option")]
    Option { symbol: String, list: Vec<KLineRow> },
    #[serde(rename = "spot")]
    Spot { symbol: String, list: Vec<KLineRow> },
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct KLineRow {
    /// Start time of the candle (ms)
    #[serde(rename = "startTime", deserialize_with = "number")]
    pub start_time: Timestamp,
    /// Open price
    #[serde(rename = "openPrice")]
    pub open_price: Decimal,
    /// Highest price
    #[serde(rename = "highPrice")]
    pub high_price: Decimal,
    /// Lowest price
    #[serde(rename = "lowPrice")]
    pub low_price: Decimal,
    /// Close price. Is the last traded price when the candle is not closed
    #[serde(rename = "closePrice")]
    pub close_price: Decimal,
    /// Trade volume. Unit of contract: pieces of contract. Unit of spot: quantity of coins
    #[serde(rename = "volume")]
    pub volume: Decimal,
    /// Turnover. Unit of figure: quantity of quota coin
    #[serde(rename = "turnover")]
    pub turnover: Decimal,
}

#[derive(Serialize)]
pub struct GetTickersParams {
    pub category: Category,
    pub symbol: Option<String>,
    pub base_coin: Option<String>,
    pub exp_date: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "category")]
pub enum Ticker {
    #[serde(rename = "inverse")]
    Inverse { list: Vec<LinearInverseTicker> },
    #[serde(rename = "linear")]
    Linear { list: Vec<LinearInverseTicker> },
    #[serde(rename = "option")]
    Option { list: Vec<OptionTicker> },
    #[serde(rename = "spot")]
    Spot { list: Vec<SpotTicker> },
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LinearInverseTicker {
    /// Symbol name
    pub symbol: String,
    /// Last price
    pub last_price: Decimal,
    /// Mark price
    pub mark_price: Decimal,
    /// Index price
    pub index_price: Decimal,
    /// Market price 24 hours ago
    pub prev_price24h: Decimal,
    /// Percentage change of market price in the last 24 hours
    pub price24h_pcnt: Decimal,
    /// The highest price in the last 24 hours
    pub high_price24h: Decimal,
    /// The lowest price in the last 24 hours
    pub low_price24h: Decimal,
    /// Market price an hour ago
    pub prev_price1h: Decimal,
    /// Open interest size
    pub open_interest: Decimal,
    /// Open interest value
    pub open_interest_value: Decimal,
    /// Turnover for 24h
    pub turnover24h: Decimal,
    /// Volume for 24h
    pub volume24h: Decimal,
    /// Funding rate
    #[serde(default, deserialize_with = "option_decimal")]
    pub funding_rate: Option<Decimal>,
    /// Next funding timestamp (ms)
    #[serde(deserialize_with = "number")]
    pub next_funding_time: Timestamp,
    /// Predicated delivery price. It has value when 30 min before delivery
    #[serde(default, deserialize_with = "option_decimal")]
    pub predicted_delivery_price: Option<Decimal>,
    /// Basis rate. Unique field for inverse futures & USDC futures
    #[serde(default, deserialize_with = "option_decimal")]
    pub basis_rate: Option<Decimal>,
    /// Basis. Unique field for inverse futures & USDC futures
    #[serde(default, deserialize_with = "option_decimal")]
    pub basis: Option<Decimal>,
    /// Delivery fee rate. Unique field for inverse futures & USDC futures
    #[serde(default, deserialize_with = "option_decimal")]
    pub delivery_fee_rate: Option<Decimal>,
    /// Delivery date time (UTC+0). Unique field for inverse futures & USDC futures
    #[serde(deserialize_with = "option_number")]
    pub delivery_time: Option<Timestamp>,
    /// Best bid price
    pub bid1_price: Decimal,
    /// Best bid size
    pub bid1_size: Decimal,
    /// Best ask price
    pub ask1_price: Decimal,
    /// Best ask size
    pub ask1_size: Decimal,
    /// Estimated pre-market contract open price. The value is meaningless when entering continuous trading phase.
    #[serde(default, deserialize_with = "option_decimal")]
    pub pre_open_price: Option<Decimal>,
    /// Estimated pre-market contract open qty. The value is meaningless when entering continuous trading phase.
    #[serde(default, deserialize_with = "option_decimal")]
    pub pre_qty: Option<Decimal>,
    /// Enum: NotStarted, Finished, CallAuction, CallAuctionNoCancel, CrossMatching, ContinuousTrading.
    #[serde(default, deserialize_with = "invalid_as_none")]
    pub cur_pre_listing_phase: Option<CurAuctionPhase>,
}
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OptionTicker {
    /// Symbol name
    pub symbol: String,
    /// Best bid price
    pub bid1_price: Decimal,
    /// Best bid size
    pub bid1_size: Decimal,
    /// Best bid iv
    pub bid1_iv: Decimal,
    /// Best ask price
    pub ask1_price: Decimal,
    /// Best ask size
    pub ask1_size: Decimal,
    /// Best ask iv
    pub ask1_iv: Decimal,
    /// Last price
    pub last_price: Decimal,
    /// The highest price in the last 24 hours
    pub high_price24h: Decimal,
    /// The lowest price in the last 24 hours
    pub low_price24h: Decimal,
    /// Mark price
    pub mark_price: Decimal,
    /// Index price
    pub index_price: Decimal,
    /// Mark price iv
    pub mark_iv: Decimal,
    /// Underlying price
    pub underlying_price: Decimal,
    /// Open interest size
    pub open_interest: Decimal,
    /// Turnover for 24h
    pub turnover24h: Decimal,
    /// Volume for 24h
    pub volume24h: Decimal,
    /// Total volume
    pub total_volume: Decimal,
    /// Total turnover
    pub total_turnover: Decimal,
    /// Delta
    pub delta: Decimal,
    /// Gamma
    pub gamma: Decimal,
    /// Vega
    pub vega: Decimal,
    /// Theta
    pub theta: Decimal,
    /// Predicated delivery price. It has value when 30 min before delivery
    pub predicted_delivery_price: Decimal,
    /// The change in the last 24 hours
    pub change24h: Decimal,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SpotTicker {
    /// Symbol name
    pub symbol: String,
    /// Best bid price
    pub bid1_price: Decimal,
    /// Best bid size
    pub bid1_size: Decimal,
    /// Best ask price
    pub ask1_price: Decimal,
    /// Best ask size
    pub ask1_size: Decimal,
    /// Last price
    pub last_price: Decimal,
    /// Market price 24 hours ago
    pub prev_price24h: Decimal,
    /// Percentage change of market price in the last 24 hours
    pub price24h_pcnt: Decimal,
    /// The highest price in the last 24 hours
    pub high_price24h: Decimal,
    /// The lowest price in the last 24 hours
    pub low_price24h: Decimal,
    /// Turnover for 24h
    pub turnover24h: Decimal,
    /// Volume for 24h
    pub volume24h: Decimal,
    /// USD index price
    /// - used to calculate USD value of the assets in Unified account
    /// - non-collateral margin coin returns ""
    /// - Only those trading pairs like "XXX/USDT" or "XXX/USDC" have the value
    #[serde(default, deserialize_with = "option_decimal")]
    pub usd_index_price: Option<Decimal>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTradesParams {
    pub category: Category,
    /// required for spot/linear/inverse
    /// optional for option
    pub symbol: Option<String>,
    /// Apply to option only
    /// If the field is not passed, return BTC data by default
    pub base_coin: Option<String>,
    /// optionType false string Option type. Call or Put. Apply to option only
    pub option_type: Option<u64>,
    /// spot: [1,60], default: 60
    /// others: [1,1000], default: 500
    pub limit: Option<u64>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "category")]
pub enum Trade {
    #[serde(rename = "inverse")]
    Inverse { list: Vec<InverseLinearSpotTrade> },
    #[serde(rename = "linear")]
    Linear { list: Vec<InverseLinearSpotTrade> },
    #[serde(rename = "option")]
    Option { list: Vec<OptionTrade> },
    #[serde(rename = "spot")]
    Spot { list: Vec<InverseLinearSpotTrade> },
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InverseLinearSpotTrade {
    /// Execution ID
    pub exec_id: String,
    /// Symbol name
    pub symbol: String,
    /// Trade price
    pub price: Decimal,
    /// Trade size
    pub size: Decimal,
    /// Side of taker Buy, Sell
    pub side: Side,
    /// Trade time (ms)
    #[serde(deserialize_with = "number")]
    pub time: Timestamp,
    /// boolean Whether the trade is block trade
    pub is_block_trade: bool,
    /// Whether the trade is RPI trade
    #[serde(rename = "isRPITrade")]
    pub is_rpi_trade: bool,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OptionTrade {
    /// Execution ID
    pub exec_id: String,
    /// Symbol name
    pub symbol: String,
    /// Trade price
    pub price: Decimal,
    /// Trade size
    pub size: Decimal,
    /// Side of taker Buy, Sell
    pub side: Side,
    /// Trade time (ms)
    #[serde(deserialize_with = "number")]
    pub time: Timestamp,
    /// boolean Whether the trade is block trade
    pub is_block_trade: bool,
    /// Whether the trade is RPI trade
    #[serde(rename = "isRPITrade")]
    pub is_rpi_trade: bool,
    /// Mark price
    #[serde(rename = "mP")]
    pub mark_price: Decimal,
    /// Index price
    #[serde(rename = "iP")]
    pub index_price: Decimal,
    /// Mark iv
    #[serde(rename = "mIv")]
    pub mark_iv: Decimal,
    /// iv
    #[serde(rename = "iv")]
    pub iv: Decimal,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ServerTime {
    /// Bybit server timestamp (sec)
    #[serde(deserialize_with = "number")]
    pub time_second: u64,
    /// Bybit server timestamp (nano)
    #[serde(deserialize_with = "number")]
    pub time_nano: u64,
}
#[derive(Serialize)]
pub struct GetInstrumentsInfoParams {
    pub category: Category,
    pub symbol: Option<String>,
    pub status: Option<Status>,
    pub base_coin: Option<String>,
    pub limit: Option<i64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "category")]
pub enum InstrumentsInfo {
    #[serde(rename = "inverse", rename_all = "camelCase")]
    Inverse {
        next_page_cursor: String,
        list: Vec<InverseLinearInstrumentsInfo>,
    },
    #[serde(rename = "linear", rename_all = "camelCase")]
    Linear {
        next_page_cursor: String,
        list: Vec<InverseLinearInstrumentsInfo>,
    },
    #[serde(rename = "option", rename_all = "camelCase")]
    Option {
        next_page_cursor: String,
        list: Vec<OptionInstrumentsInfo>,
    },
    #[serde(rename = "spot", rename_all = "camelCase")]
    Spot {
        #[serde(default, deserialize_with = "empty_string_as_none")]
        next_page_cursor: Option<String>,
        list: Vec<SpotInstrumentsInfo>,
    },
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InverseLinearInstrumentsInfo {
    pub symbol: String,
    pub contract_type: ContractType,
    pub status: Status,
    pub base_coin: String,
    pub quote_coin: String,
    #[serde(deserialize_with = "number")]
    pub launch_time: Timestamp,
    #[serde(deserialize_with = "number")]
    pub delivery_time: Timestamp,
    #[serde(deserialize_with = "option_decimal")]
    pub delivery_fee_rate: Option<Decimal>,
    #[serde(deserialize_with = "number")]
    pub price_scale: i64,
    pub leverage_filter: LeverageFilter,
    pub price_filter: PriceFilter,
    pub lot_size_filter: LotSizeFilter,
    pub unified_margin_trade: bool,
    pub funding_interval: i64,
    pub settle_coin: String,
    pub copy_trading: CopyTrading,
    pub upper_funding_rate: Decimal,
    pub lower_funding_rate: Decimal,
    pub risk_parameters: RiskParameters,
    pub is_pre_listing: bool,
    pub pre_listing_info: Option<PreListingInfo>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OptionInstrumentsInfo {
    pub symbol: String,
    pub contract_type: ContractType,
    pub status: Status,
    pub base_coin: String,
    pub quote_coin: String,
    #[serde(deserialize_with = "number")]
    pub launch_time: i64,
    #[serde(deserialize_with = "number")]
    pub delivery_time: i64,
    #[serde(deserialize_with = "option_decimal")]
    pub delivery_fee_rate: Option<Decimal>,
    #[serde(deserialize_with = "number")]
    pub price_scale: i64,
    pub leverage_filter: LeverageFilter,
    pub price_filter: PriceFilter,
    pub lot_size_filter: LotSizeFilter,
    pub unified_margin_trade: bool,
    pub funding_interval: i64,
    pub settle_coin: String,
    pub copy_trading: CopyTrading,
    pub upper_funding_rate: Decimal,
    pub lower_funding_rate: Decimal,
    pub risk_parameters: RiskParameters,
    pub is_pre_listing: bool,
    pub pre_listing_info: Option<PreListingInfo>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SpotInstrumentsInfo {
    /// Symbol name
    pub symbol: String,
    /// Base coin
    pub base_coin: String,
    /// Quote coin
    pub quote_coin: String,
    /// Whether or not this is an innovation zone token. 0: false, 1: true
    #[serde(deserialize_with = "string_to_bool")]
    pub innovation: bool,
    /// Instrument status
    pub status: Status,
    /// Margin trade symbol or not
    /// This is to identify if the symbol support margin trading under different account modes
    /// You may find some symbols not supporting margin buy or margin sell, so you need to go to Collateral Info (UTA) to check if that coin is borrowable
    pub margin_trading: String,
    /// Whether or not it has an special treatment label. 0: false, 1: true
    #[serde(deserialize_with = "string_to_bool")]
    pub st_tag: bool,
    /// Size attributes
    pub lot_size_filter: SpotLotSizeFilter,
    /// Price attributes
    pub price_filter: SpotPriceFilter,
    /// Risk parameters for limit order price, refer to announcement
    pub risk_parameters: RiskParameters,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LeverageFilter {
    pub min_leverage: Decimal,
    pub max_leverage: Decimal,
    pub leverage_step: Decimal,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PriceFilter {
    pub min_price: Decimal,
    pub max_price: Decimal,
    pub tick_size: Decimal,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SpotPriceFilter {
    /// The step to increase/reduce order price
    pub tick_size: Decimal,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LotSizeFilter {
    pub min_notional_value: Decimal,
    pub max_order_qty: Decimal,
    pub max_mkt_order_qty: Decimal,
    pub min_order_qty: Decimal,
    pub qty_step: Decimal,
    pub post_only_max_order_qty: Decimal,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SpotLotSizeFilter {
    /// The precision of base coin
    pub base_precision: Decimal,
    /// The precision of quote coin
    pub quote_precision: Decimal,
    /// Minimum order quantity
    pub min_order_qty: Decimal,
    /// Maximum order quantity
    pub max_order_qty: Decimal,
    /// Minimum order amount
    pub min_order_amt: Decimal,
    /// Maximum order amount
    pub max_order_amt: Decimal,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RiskParameters {
    pub price_limit_ratio_x: Decimal,
    pub price_limit_ratio_y: Decimal,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PreListingInfo {
    pub cur_auction_phase: CurAuctionPhase,
    pub phases: Vec<Phase>,
    pub auction_fee_info: AuctionFeeInfo,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Phase {
    pub phase: CurAuctionPhase,
    #[serde(deserialize_with = "option_number")]
    pub start_time: Option<Timestamp>,
    #[serde(deserialize_with = "option_number")]
    pub end_time: Option<Timestamp>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AuctionFeeInfo {
    pub auction_fee_rate: Decimal,
    pub taker_fee_rate: Decimal,
    pub maker_fee_rate: Decimal,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOpenClosedOrdersParams {
    /// Product type
    /// UTA2.0, UTA1.0: linear, inverse, spot, option
    /// classic account: linear, inverse, spot
    pub category: Category,
    /// Symbol name, like BTCUSDT, uppercase only. For linear, either symbol, baseCoin, settleCoin is required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Base coin, uppercase only
    /// Supports linear, inverse & option
    /// option: it returns all option open orders by default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    /// Settle coin, uppercase only
    /// linear: either symbol, baseCoin or settleCoin is required
    /// spot: not supported
    /// option: USDT or USDC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_coin: Option<String>,
    /// Order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// User customized order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    /// 0(default): UTA2.0, UTA1.0, classic account query open status orders (e.g., New, PartiallyFilled) only
    /// 1: UTA2.0, UTA1.0(except inverse)
    /// 2: UTA1.0(inverse), classic account
    /// Query a maximum of recent 500 closed status records are kept under each account each category (e.g., Cancelled, Rejected, Filled orders).
    /// If the Bybit service is restarted due to an update, this part of the data will be cleared and accumulated again, but the order records will still be queried in order history
    /// openOnly param will be ignored when query by orderId or orderLinkId
    /// Classic spot: not supported
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_only: Option<i32>,
    /// Order: active order,
    /// StopOrder: conditional order for Futures and Spot,
    /// tpslOrder: spot TP/SL order,
    /// OcoOrder: Spot oco order,
    /// BidirectionalTpslOrder: Spot bidirectional TPSL order
    /// - classic account spot: return Order active order by default
    /// - Others: all kinds of orders by default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_filter: Option<OrderFilter>,
    /// Limit for data size per page. [1, 50]. Default: 20
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Cursor. Use the nextPageCursor token from the response to retrieve the next page of the result set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Serialize)]
pub enum OrderFilter {
    /// active order,
    Order,
    /// conditional order for Futures and Spot,
    StopOrder,
    /// spot TP/SL order,
    #[serde(rename = "camelCase")]
    TpslOrder,
    /// Spot oco order,
    OcoOrder,
    /// Spot bidirectional TPSL order
    /// - classic account spot: return Order active order by default
    /// - Others: all kinds of orders by default
    BidirectionalTpslOrder,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    /// Order ID
    pub order_id: String,
    /// User customized order ID
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub order_link_id: Option<String>,
    /// Paradigm block trade ID
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub block_trade_id: Option<String>,
    /// Symbol name
    pub symbol: String,
    /// Order price
    pub price: Decimal,
    /// Order qty
    pub qty: Decimal,
    /// Side. Buy,Sell
    pub side: Side,
    /// Whether to borrow. Unified spot only. 0: false, 1: true. Classic spot is not supported, always 0
    #[serde(default, deserialize_with = "string_to_option_bool")]
    pub is_leverage: Option<bool>,
    /// Position index. Used to identify positions in different position modes.
    pub position_idx: PositionIdx,
    /// Order status
    pub order_status: OrderStatus,
    /// Order create type
    /// Only for category=linear or inverse
    /// Spot, Option do not have this key
    #[serde(default, deserialize_with = "invalid_as_none")]
    pub create_type: Option<CreateType>,
    /// Cancel type
    pub cancel_type: CancelType,
    /// Reject reason. Classic spot is not supported
    pub reject_reason: RejectReason,
    /// Average filled price
    /// UTA: returns "" for those orders without avg price
    /// classic account: returns "0" for those orders without avg price, and also for those orders have partially filled but cancelled at the end
    #[serde(default, deserialize_with = "option_decimal")]
    pub avg_price: Option<Decimal>,
    /// The remaining qty not executed. Classic spot is not supported
    pub leaves_qty: Decimal,
    /// The estimated value not executed. Classic spot is not supported
    pub leaves_value: Decimal,
    /// Cumulative executed order qty
    pub cum_exec_qty: Decimal,
    /// Cumulative executed order value. Classic spot is not supported
    pub cum_exec_value: Decimal,
    /// Cumulative executed trading fee. Classic spot is not supported
    pub cum_exec_fee: Decimal,
    /// Time in force
    pub time_in_force: TimeInForce,
    /// Order type. Market,Limit. For TP/SL order, it means the order type after triggered
    pub order_type: OrderType,
    /// Stop order type
    #[serde(default, deserialize_with = "invalid_as_none")]
    pub stop_order_type: Option<StopOrderType>,
    /// Implied volatility
    #[serde(default, deserialize_with = "option_decimal")]
    pub order_iv: Option<Decimal>,
    /// The unit for qty when create Spot market orders for UTA account. baseCoin, quoteCoin
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub market_unit: Option<String>,
    /// Trigger price. If stopOrderType=TrailingStop, it is activate price. Otherwise, it is trigger price
    #[serde(default, deserialize_with = "option_decimal")]
    pub trigger_price: Option<Decimal>,
    /// Take profit price
    #[serde(default, deserialize_with = "option_decimal")]
    pub take_profit: Option<Decimal>,
    /// Stop loss price
    #[serde(default, deserialize_with = "option_decimal")]
    pub stop_loss: Option<Decimal>,
    /// TP/SL mode, Full: entire position for TP/SL. Partial: partial position tp/sl. Spot does not have this field, and Option returns always ""
    #[serde(default, deserialize_with = "invalid_as_none")]
    pub tpsl_mode: Option<TpslMode>,
    /// The trigger type of Spot OCO order.OcoTriggerByUnknown, OcoTriggerByTp, OcoTriggerByBySl. Classic spot is not supported
    #[serde(default, deserialize_with = "invalid_as_none")]
    pub oco_trigger_by: Option<OcoTriggerBy>,
    /// The limit order price when take profit price is triggered
    #[serde(default, deserialize_with = "option_decimal")]
    pub tp_limit_price: Option<Decimal>,
    /// The limit order price when stop loss price is triggered
    #[serde(default, deserialize_with = "option_decimal")]
    pub sl_limit_price: Option<Decimal>,
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
    #[serde(default, deserialize_with = "option_decimal")]
    pub last_price_on_created: Option<Decimal>,
    /// Last price when place the order, Spot has this field only
    #[serde(default, deserialize_with = "option_decimal")]
    pub base_price: Option<Decimal>,
    /// Reduce only. true means reduce position size
    pub reduce_only: bool,
    /// Close on trigger. What is a close on trigger order?
    pub close_on_trigger: bool,
    /// Place type, option used. iv, price
    #[serde(default, deserialize_with = "invalid_as_none")]
    pub place_type: Option<PlaceType>,
    /// SMP execution type
    pub smp_type: SmpType,
    /// Smp group ID. If the UID has no group, it is 0 by default
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

impl Order {
    pub fn is_open_status(&self) -> bool {
        self.order_status.is_open()
    }
    pub fn is_closed_status(&self) -> bool {
        self.order_status.is_closed()
    }
    pub fn update(&mut self, msg: OrderMsg) {
        self.order_id = msg.order_id;
        self.order_link_id = msg.order_link_id;
        self.block_trade_id = msg.block_trade_id;
        self.symbol = msg.symbol;
        self.price = msg.price;
        self.qty = msg.qty;
        self.side = msg.side;
        self.is_leverage = msg.is_leverage;
        self.position_idx = msg.position_idx;
        self.order_status = msg.order_status;
        self.create_type = msg.create_type;
        self.cancel_type = msg.cancel_type;
        self.reject_reason = msg.reject_reason;
        self.avg_price = msg.avg_price;
        if let Some(leaves_qty) = msg.leaves_qty {
            self.leaves_qty = leaves_qty;
        }
        if let Some(leaves_value) = msg.leaves_value {
            self.leaves_value = leaves_value;
        }
        self.cum_exec_qty = msg.cum_exec_qty;
        self.cum_exec_value = msg.cum_exec_value;
        self.cum_exec_fee = msg.cum_exec_fee;
        self.time_in_force = msg.time_in_force;
        self.order_type = msg.order_type;
        self.stop_order_type = msg.stop_order_type;
        self.order_iv = msg.order_iv;
        self.market_unit = msg.market_unit;
        self.trigger_price = msg.trigger_price;
        self.take_profit = msg.take_profit;
        self.stop_loss = msg.stop_loss;
        self.tpsl_mode = msg.tpsl_mode;
        self.oco_trigger_by = msg.oco_trigger_by;
        self.tp_limit_price = msg.tp_limit_price;
        self.sl_limit_price = msg.sl_limit_price;
        self.tp_trigger_by = msg.tp_trigger_by;
        self.sl_trigger_by = msg.sl_trigger_by;
        self.trigger_direction = msg.trigger_direction;
        self.trigger_by = msg.trigger_by;
        self.last_price_on_created = msg.last_price_on_created;
        // TODO: self.base_price
        self.reduce_only = msg.reduce_only;
        self.close_on_trigger = msg.close_on_trigger;
        self.place_type = msg.place_type;
        self.smp_type = msg.smp_type;
        self.smp_group = msg.smp_group;
        self.smp_order_id = msg.smp_order_id;
        self.created_time = msg.created_time;
        self.updated_time = msg.updated_time;
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderRequest {
    /// Product type
    /// UTA2.0, UTA1.0: linear, inverse, spot, option
    /// classic account: linear, inverse, spot
    pub category: Category,
    /// Symbol name, like BTCUSDT, uppercase only
    pub symbol: String,
    // Whether to borrow. Unified account Spot trading only.
    /// 0(default): false, spot trading
    /// 1: true, margin trading, make sure you turn on margin trading, and set the relevant currency as collateral
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_leverage: Option<i64>,
    /// Buy, Sell
    pub side: Side,
    /// Market, Limit
    pub order_type: OrderType,
    /// Order quantity
    /// UTA account
    /// Spot: Market Buy order by value by default, you can set marketUnit field to choose order by value or qty for market orders
    /// Perps, Futures & Option: always order by qty
    /// classic account
    /// Spot: Market Buy order by value by default
    /// Perps, Futures: always order by qty
    /// Perps & Futures: if you pass qty="0" and specify reduceOnly=true&closeOnTrigger=true, you can close the position up to maxMktOrderQty or maxOrderQty shown on Get Instruments Info of current symbol
    pub qty: Decimal,
    /// Select the unit for qty when create Spot market orders for UTA account
    /// baseCoin: for example, buy BTCUSDT, then "qty" unit is BTC
    /// quoteCoin: for example, sell BTCUSDT, then "qty" unit is USDT
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_unit: Option<String>,
    /// Slippage tolerance Type for market order, TickSize, Percent
    /// take profit, stoploss, conditional orders are not supported
    /// TickSize:
    /// the highest price of Buy order = ask1 + slippageTolerance x tickSize;
    /// the lowest price of Sell order = bid1 - slippageTolerance x tickSize
    /// Percent:
    /// the highest price of Buy order = ask1 x (1 + slippageTolerance x 0.01);
    /// the lowest price of Sell order = bid1 x (1 - slippageTolerance x 0.01)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slippage_tolerance_type: Option<Decimal>,
    /// Slippage tolerance value
    /// TickSize: range is [1, 10000], integer only
    /// Percent: range is [0.01, 10], up to 2 decimals
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slippage_tolerance: Option<Decimal>,
    /// Order price
    /// Market order will ignore this field
    /// Please check the min price and price precision from instrument info endpoint
    /// If you have position, price needs to be better than liquidation price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Decimal>,
    /// Conditional order param. Used to identify the expected direction of the conditional order.
    /// 1: triggered when market price rises to triggerPrice
    /// 2: triggered when market price falls to triggerPrice
    /// Valid for linear & inverse
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_direction: Option<TriggerDirection>,
    /// If it is not passed, Order by default.
    /// Order
    /// tpslOrder: Spot TP/SL order, the assets are occupied even before the order is triggered
    /// StopOrder: Spot conditional order, the assets will not be occupied until the price of the underlying asset reaches the trigger price, and the required assets will be occupied after the Conditional order is triggered
    /// Valid for spot only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_filter: Option<String>,
    /// For Perps & Futures, it is the conditional order trigger price. If you expect the price to rise to trigger your conditional order, make sure:
    /// triggerPrice > market price
    /// Else, triggerPrice < market price
    /// For spot, it is the TP/SL and Conditional order trigger price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<Decimal>,
    /// Trigger price type, Conditional order param for Perps & Futures.
    /// LastPrice
    /// IndexPrice
    /// MarkPrice
    /// Valid for linear & inverse
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_by: Option<TriggerBy>,
    /// Implied volatility. option only. Pass the real value, e.g for 10%, 0.1 should be passed. orderIv has a higher priority when price is passed as well
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_iv: Option<Decimal>,
    /// Time in force
    /// Market order will always use IOC
    /// If not passed, GTC is used by default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,
    /// Used to identify positions in different position modes. Under hedge-mode, this param is required
    /// 0: one-way mode
    /// 1: hedge-mode Buy side
    /// 2: hedge-mode Sell side
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_idx: Option<PositionIdx>,
    /// User customised order ID. A max of 36 characters. Combinations of numbers, letters (upper and lower cases), dashes, and underscores are supported.
    /// Futures & Perps: orderLinkId rules:
    /// optional param
    /// always unique
    /// option orderLinkId rules:
    /// required param
    /// always unique
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    /// Take profit price
    /// UTA: Spot Limit order supports take profit, stop loss or limit take profit, limit stop loss when creating an order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_profit: Option<Decimal>,
    /// Stop loss price
    /// UTA: Spot Limit order supports take profit, stop loss or limit take profit, limit stop loss when creating an order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<Decimal>,
    /// The price type to trigger take profit. MarkPrice, IndexPrice, default: LastPrice. Valid for linear & inverse
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_by: Option<TriggerBy>,
    /// The price type to trigger stop loss. MarkPrice, IndexPrice, default: LastPrice. Valid for linear & inverse
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_by: Option<TriggerBy>,
    /// What is a reduce-only order? true means your position can only reduce in size if this order is triggered.
    /// You must specify it as true when you are about to close/reduce the position
    /// When reduceOnly is true, take profit/stop loss cannot be set
    /// Valid for linear, inverse & option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    /// What is a close on trigger order? For a closing order. It can only reduce your position, not increase it. If the account has insufficient available balance when the closing order is triggered, then other active orders of similar contracts will be cancelled or reduced. It can be used to ensure your stop loss reduces your position regardless of current available margin.
    /// Valid for linear & inverse
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_on_trigger: Option<bool>,
    /// Smp execution type. What is SMP?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smp_type: Option<SmpType>,
    /// Market maker protection. option only. true means set the order as a market maker protection order. What is mmp?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mmp: Option<bool>,
    /// TP/SL mode
    /// Full: entire position for TP/SL. Then, tpOrderType or slOrderType must be Market
    /// Partial: partial position tp/sl (as there is no size option, so it will create tp/sl orders with the qty you actually fill). Limit TP/SL order are supported. Note: When create limit tp/sl, tpslMode is required and it must be Partial
    /// Valid for linear & inverse
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpsl_mode: Option<TpslMode>,
    /// The limit order price when take profit price is triggered
    /// linear & inverse: only works when tpslMode=Partial and tpOrderType=Limit
    /// Spot(UTA): it is required when the order has takeProfit and "tpOrderType"=Limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_limit_price: Option<Decimal>,
    /// The limit order price when stop loss price is triggered
    /// linear & inverse: only works when tpslMode=Partial and slOrderType=Limit
    /// Spot(UTA): it is required when the order has stopLoss and "slOrderType"=Limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_limit_price: Option<Decimal>,
    /// The order type when take profit is triggered
    /// linear & inverse: Market(default), Limit. For tpslMode=Full, it only supports tpOrderType=Market
    /// Spot(UTA):
    /// Market: when you set "takeProfit",
    /// Limit: when you set "takeProfit" and "tpLimitPrice"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_order_type: Option<OrderType>,
    /// The order type when stop loss is triggered
    /// linear & inverse: Market(default), Limit. For tpslMode=Full, it only supports slOrderType=Market
    /// Spot(UTA):
    /// Market: when you set "stopLoss",
    /// Limit: when you set "stopLoss" and "slLimitPrice"
    /// bboSideType	false	string
    /// Queue: use the order price on the orderbook in the same direction as the side
    /// Counterparty: use the order price on the orderbook in the opposite direction as the side
    /// Valid for linear & inverse
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_order_type: Option<OrderType>,
    /// 1,2,3,4,5 Valid for linear & inverse
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbo_level: Option<String>,
}

impl PlaceOrderRequest {
    pub fn new(
        category: Category,
        symbol: String,
        side: Side,
        order_type: OrderType,
        qty: Decimal,
    ) -> Self {
        Self {
            category,
            symbol,
            is_leverage: None,
            side,
            order_type,
            qty,
            market_unit: None,
            slippage_tolerance_type: None,
            slippage_tolerance: None,
            price: None,
            trigger_direction: None,
            order_filter: None,
            trigger_price: None,
            trigger_by: None,
            order_iv: None,
            time_in_force: None,
            position_idx: None,
            order_link_id: None,
            take_profit: None,
            stop_loss: None,
            tp_trigger_by: None,
            sl_trigger_by: None,
            reduce_only: None,
            close_on_trigger: None,
            smp_type: None,
            mmp: None,
            tpsl_mode: None,
            tp_limit_price: None,
            sl_limit_price: None,
            tp_order_type: None,
            sl_order_type: None,
            bbo_level: None,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderResponse {
    /// Order ID
    pub order_id: String,
    /// User customised order ID
    pub order_link_id: String,
}

// TODO: Implement.
// https://bybit-exchange.github.io/docs/v5/order/amend-order
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AmendOrderRequest {
    pub category: Category,
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Decimal>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AmendOrderResponse {
    /// Order ID
    pub order_id: String,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    /// User customised order ID
    pub order_link_id: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRequest {
    /// Product type. linear, inverse, spot, option
    pub category: Category,
    /// Symbol name, like BTCUSDT, uppercase only
    pub symbol: String,
    /// Order ID. Either orderId or orderLinkId is required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// User customised order ID. Either orderId or orderLinkId is required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    /// Spot trading only
    /// Order, tpslOrder, StopOrder
    /// If not passed, Order by default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_filter: Option<OrderFilter>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse {
    /// Order ID
    pub order_id: String,
    /// User customised order ID
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub order_link_id: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionInfoParams {
    /// Product type
    /// UTA2.0, UTA1.0: linear, inverse, option
    /// Classic account: linear, inverse
    pub category: Category,
    /// Symbol name, like BTCUSDT, uppercase only
    /// If symbol passed, it returns data regardless of having position or not.
    /// If symbol=null and settleCoin specified, it returns position size greater than zero.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Base coin, uppercase only. option only. Return all option positions if not passed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    /// Settle coin
    /// linear: either symbol or settleCoin is required. symbol has a higher priority
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_coin: Option<String>,
    /// Limit for data size per page. [1, 200]. Default: 20
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    /// Cursor. Use the nextPageCursor token from the response to retrieve the next page of the result set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

// TODO: check fields
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    /// Position idx, used to identify positions in different position modes
    /// 0: One-Way Mode
    /// 1: Buy side of both side mode
    /// 2: Sell side of both side mode
    pub position_idx: PositionIdx,
    /// Risk tier ID
    /// for portfolio margin mode, this field returns 0, which means risk limit rules are invalid
    pub risk_id: i64,
    /// Risk limit value
    /// for portfolio margin mode, this field returns 0, which means risk limit rules are invalid
    #[serde(default, deserialize_with = "option_decimal")]
    pub risk_limit_value: Option<Decimal>,
    /// Symbol name
    pub symbol: String,
    /// Position side. Buy: long, Sell: short
    /// one-way mode: classic & UTA1.0(inverse), an empty position returns None.
    /// UTA2.0(linear, inverse) & UTA1.0(linear): either one-way or hedge mode returns an empty string "" for an empty position.
    #[serde(default, deserialize_with = "invalid_as_none")]
    pub side: Option<Side>,
    /// Position size, always positive
    pub size: Decimal,
    /// Average entry price
    /// For USDC Perp & Futures, it indicates average entry price, and it will not be changed with 8-hour session settlement
    pub avg_price: Decimal,
    /// Position value
    #[serde(default, deserialize_with = "option_decimal")]
    pub position_value: Option<Decimal>,
    /// Whether to add margin automatically when using isolated margin mode
    /// 0: false
    /// 1: true
    #[serde(deserialize_with = "int_to_bool")]
    pub auto_add_margin: bool,
    /// Position status. Normal, Liq, Adl
    pub position_status: PositionStatus,
    /// Position leverage
    /// for portfolio margin mode, this field returns "", which means leverage rules are invalid
    pub leverage: Decimal,
    /// Mark price
    pub mark_price: Decimal,
    /// Position liquidation price
    /// UTA2.0(isolated margin), UTA1.0(isolated margin), UTA1.0(inverse), Classic account:
    /// it is the real price for isolated and cross positions, and keeps "" when liqPrice <= minPrice or liqPrice >= maxPrice
    /// UTA2.0(Cross margin), UTA1.0(Cross margin):
    /// it is an estimated price for cross positions(because the unified mode controls the risk rate according to the account), and keeps "" when liqPrice <= minPrice or liqPrice >= maxPrice
    /// this field is empty for Portfolio Margin Mode, and no liquidation price will be provided
    #[serde(default, deserialize_with = "option_decimal")]
    pub liq_price: Option<Decimal>,
    /// Initial margin
    /// Classic & UTA1.0(inverse): ignore this field
    /// UTA portfolio margin mode, it returns ""
    #[serde(rename = "positionIM", default, deserialize_with = "option_decimal")]
    pub position_im: Option<Decimal>,
    /// Initial margin calculated by mark price
    /// Classic & UTA1.0(inverse) : ignore this field
    /// UTA portfolio margin mode, it returns ""
    #[serde(
        rename = "positionIMByMp",
        default,
        deserialize_with = "option_decimal"
    )]
    pub position_im_by_mp: Option<Decimal>,
    /// Maintenance margin
    /// Classic & UTA1.0(inverse): ignore this field
    /// UTA portfolio margin mode, it returns ""
    #[serde(rename = "positionMM", default, deserialize_with = "option_decimal")]
    pub position_mm: Option<Decimal>,
    /// Maintenance margin calculated by mark price
    /// Classic & UTA1.0(inverse) : ignore this field
    /// UTA portfolio margin mode, it returns ""
    #[serde(
        rename = "positionMMByMp",
        default,
        deserialize_with = "option_decimal"
    )]
    pub position_mm_by_mp: Option<Decimal>,
    /// Take profit price
    #[serde(default, deserialize_with = "option_decimal")]
    pub take_profit: Option<Decimal>,
    /// Stop loss price
    #[serde(default, deserialize_with = "option_decimal")]
    pub stop_loss: Option<Decimal>,
    /// Trailing stop (The distance from market price)
    #[serde(default, deserialize_with = "option_decimal")]
    pub trailing_stop: Option<Decimal>,
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
    /// Unrealised PnL
    #[serde(default, deserialize_with = "option_decimal")]
    pub unrealised_pnl: Option<Decimal>,
    /// The realised PnL for the current holding position
    pub cur_realised_pnl: Decimal,
    /// Cumulative realised pnl
    /// Futures & Perpetuals: it is the all time cumulative realised P&L
    /// Option: always "", meaningless
    pub cum_realised_pnl: Decimal,
    /// Auto-deleverage rank indicator. What is Auto-Deleveraging?
    pub adl_rank_indicator: AdlRankIndicator,
    /// Timestamp of the first time a position was created on this symbol (ms)
    #[serde(deserialize_with = "number")]
    pub created_time: Timestamp,
    /// Position updated timestamp (ms)
    #[serde(deserialize_with = "number")]
    pub updated_time: Timestamp,
    /// Cross sequence, used to associate each fill and each position update
    /// Different symbols may have the same seq, please use seq + symbol to check unique
    /// Returns "-1" if the symbol has never been traded
    /// Returns the seq updated by the last transaction when there are settings like leverage, risk limit
    pub seq: i64,
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
}

impl Position {
    pub fn update(&mut self, msg: PositionMsg) {
        self.position_idx = msg.position_idx;
        self.risk_id = msg.risk_id;
        self.risk_limit_value = msg.risk_limit_value;
        self.symbol = msg.symbol;
        self.side = msg.side;
        self.size = msg.size;
        self.avg_price = msg.entry_price;
        self.position_value = Some(msg.position_value);
        self.auto_add_margin = msg.auto_add_margin;
        self.position_status = msg.position_status;
        self.leverage = msg.leverage;
        self.mark_price = msg.mark_price;
        self.liq_price = msg.liq_price;
        // INFO: self.position_im updated in self.update_with_a_wallet_coin
        // INFO: self.position_mm updated in self.update_with_a_wallet_coin
        self.take_profit = Some(msg.take_profit);
        self.stop_loss = Some(msg.stop_loss);
        self.trailing_stop = Some(msg.trailing_stop);
        // self.trailing_stop = msg.trailing_stop;
        self.session_avg_price = msg.session_avg_price;
        self.delta = msg.delta;
        self.gamma = msg.gamma;
        self.vega = msg.vega;
        self.theta = msg.theta;
        // INFO: self.unrealised_pnl updated in self.update_with_a_wallet_coin
        self.cur_realised_pnl = msg.cur_realised_pnl;
        self.cum_realised_pnl = msg.cum_realised_pnl;
        self.adl_rank_indicator = msg.adl_rank_indicator;
        self.created_time = msg.created_time;
        self.updated_time = msg.updated_time;
        self.seq = msg.seq;
        self.is_reduce_only = msg.is_reduce_only;
        self.mmr_sys_updated_time = msg.mmr_sys_updated_time;
        self.leverage_sys_updated_time = msg.leverage_sys_updated_time;
    }

    pub fn update_with_a_wallet_coin(&mut self, msg: &WalletCoin) {
        self.position_mm = msg.total_position_im;
        self.position_im = msg.total_position_mm;
        self.unrealised_pnl = Some(msg.unrealised_pnl);
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWalletBalanceParams {
    /// Account type
    /// UTA2.0: UNIFIED
    /// UTA1.0: UNIFIED, CONTRACT(inverse derivatives wallet)
    /// Classic account: CONTRACT, SPOT
    /// To get Funding wallet balance, please go to this endpoint
    pub account_type: AccountType,
    /// Coin name, uppercase only
    /// If not passed, it returns non-zero asset info
    /// You can pass multiple coins to query, separated by comma. USDT,USDC
    pub coin: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WalletBalance {
    /// Account type
    pub account_type: AccountType,
    /// Account IM rate
    /// You can refer to this Glossary to understand the below fields calculation and meaning
    /// All account wide fields are not applicable to
    /// UTA2.0(isolated margin),
    /// UTA1.0(isolated margin), UTA1.0(CONTRACT),
    /// classic account(SPOT, CONTRACT)
    #[serde(rename = "accountIMRate")]
    pub account_im_rate: Decimal,
    /// Account initial margin (USD) calculated by mark price: ∑Asset Total Initial Margin Base Coin calculated by mark price
    #[serde(rename = "accountIMRateByMp")]
    pub account_im_rate_by_mp: Decimal,
    /// Account MM rate
    #[serde(rename = "accountMMRate")]
    pub account_mm_rate: Decimal,
    /// Account maintenance margin (USD) calculated by mark price: ∑ Asset Total Maintenance Margin Base Coin calculated by mark price
    #[serde(rename = "accountMMRateByMp")]
    pub account_mm_rate_by_mp: Decimal,
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
    /// Account initial margin (USD) calculated by mark price: ∑Asset Total Initial Margin Base Coin calculated by mark price
    pub total_initial_margin_by_mp: Decimal,
    /// Account maintenance margin (USD): ∑ Asset Total Maintenance Margin Base Coin
    pub total_maintenance_margin: Decimal,
    /// Account maintenance margin (USD) calculated by mark price: ∑ Asset Total Maintenance Margin Base Coin calculated by mark price
    pub total_maintenance_margin_by_mp: Decimal,
    #[serde(deserialize_with = "hash_map")]
    pub coin: HashMap<String, WalletCoin>,
}

impl WalletBalance {
    pub fn update(&mut self, msg: WalletMsg) {
        self.account_type = msg.account_type;
        self.account_im_rate = msg.account_im_rate;
        self.account_mm_rate = msg.account_mm_rate;
        self.total_equity = msg.total_equity;
        self.total_wallet_balance = msg.total_wallet_balance;
        self.total_margin_balance = msg.total_margin_balance;
        self.total_available_balance = msg.total_available_balance;
        self.total_perp_upl = msg.total_perp_upl;
        self.total_initial_margin = msg.total_initial_margin;
        self.total_maintenance_margin = msg.total_maintenance_margin;
        self.coin = msg.coin;
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WalletCoin {
    /// Coin name, such as BTC, ETH, USDT, USDC
    pub coin: String,
    /// Equity of coin
    pub equity: Decimal,
    /// USD value of coin. If this coin cannot be collateral, then it is 0
    pub usd_value: Decimal,
    /// Wallet balance of coin
    pub wallet_balance: Decimal,
    /// Locked balance due to the Spot open order
    pub locked: Decimal,
    /// The spot asset qty that is used to hedge in the portfolio margin, truncate to 8 decimals and "0" by default. This is a unique field for Unified account.
    pub spot_hedging_qty: Decimal,
    /// Borrow amount of current coin
    pub borrow_amount: Decimal,
    /// Accrued interest
    pub accrued_interest: Decimal,
    /// Pre-occupied margin for order. For portfolio margin mode, it returns ""
    #[serde(rename = "totalOrderIM", default, deserialize_with = "option_decimal")]
    pub total_order_im: Option<Decimal>,
    /// Sum of initial margin of all positions + Pre-occupied liquidation fee. For portfolio margin mode, it returns ""
    #[serde(
        rename = "totalPositionIM",
        default,
        deserialize_with = "option_decimal"
    )]
    pub total_position_im: Option<Decimal>,
    /// Sum of maintenance margin for all positions. For portfolio margin mode, it returns ""
    #[serde(
        rename = "totalPositionMM",
        default,
        deserialize_with = "option_decimal"
    )]
    pub total_position_mm: Option<Decimal>,
    /// Unrealised P&L
    pub unrealised_pnl: Decimal,
    /// Cumulative Realised P&L
    pub cum_realised_pnl: Decimal,
    /// Bonus. This is a unique field for accountType=UNIFIED
    pub bonus: Decimal,
    /// Whether the collateral is turned on by user (user), true: ON, false: OFF
    /// When marginCollateral=true, then collateralSwitch is meaningful
    pub collateral_switch: bool,
    /// Whether it can be used as a margin collateral currency (platform), true: YES, false: NO
    /// When marginCollateral=false, then collateralSwitch is meaningless
    pub margin_collateral: bool,
    /// Borrow amount by spot margin trade and manual borrow amount (does not include borrow amount by spot margin active order). spotBorrow field corresponding to spot liabilities is detailed in the announcement.
    #[serde(default, deserialize_with = "option_decimal")]
    pub spot_borrow: Option<Decimal>,
}

impl Unique<String> for WalletCoin {
    fn unique_key(&self) -> String {
        self.coin.clone()
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    /// Account status
    pub unified_margin_status: UnifiedMarginStatus,
    /// ISOLATED_MARGIN, REGULAR_MARGIN, PORTFOLIO_MARGIN
    pub margin_mode: MarginMode,
    /// Whether this account is a leader (copytrading). true, false
    pub is_master_trader: bool,
    /// Whether the unified account enables Spot hedging. ON, OFF
    pub spot_hedging_status: SpotHedgingStatus,
    /// Account data updated timestamp (ms)
    #[serde(deserialize_with = "number")]
    pub updated_time: Timestamp,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DCPConfiguration {
    /// DCP config for each product
    pub dcp_infos: Vec<DCPInfo>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DCPInfo {
    /// SPOT, DERIVATIVES, OPTIONS
    pub product: DCPProduct,
    /// Disconnected-CancelAll-Prevention status: ON
    #[serde(rename = "dcpStatus")]
    pub status: String,
    /// DCP trigger time window which user pre-set. Between [3, 300] seconds, default: 10 sec
    #[serde(deserialize_with = "number")]
    pub time_window: Second,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct APIKeyInformation {
    /// Unique ID. Internal use
    pub id: String,
    /// The remark
    pub note: String,
    /// Api key
    pub api_key: String,
    /// 0：Read and Write. 1：Read only
    pub read_only: i64,
    /// Always ""
    pub secret: String,
    /// The types of permission
    pub permissions: APIKeyPermissions,
    /// IP bound
    pub ips: Vec<String>,
    /// The type of api key. 1：personal, 2：connected to the third-party app
    #[serde(rename = "type")]
    pub key_type: i64,
    /// The remaining valid days of api key. Only for those api key with no IP bound or the password has been changed
    pub deadline_day: u64,
    /// The expiry day of the api key. Only for those api key with no IP bound or the password has been changed
    pub expired_at: String,
    /// The create day of the api key
    pub created_at: String,
    /// Whether the account to which the account upgrade to unified trade account. 0：regular account; 1：unified trade account
    pub uta: i64,
    /// User ID
    #[serde(rename = "userID")]
    pub user_id: i64,
    /// Inviter ID (the UID of the account which invited this account to the platform)
    #[serde(rename = "inviterID")]
    pub inviter_id: i64,
    /// VIP Level
    pub vip_level: VipLevel,
    /// Market maker level
    pub mkt_maker_level: String,
    /// Affiliate Id. 0 represents that there is no binding relationship.
    #[serde(rename = "affiliateID")]
    pub affiliate_id: i64,
    /// Rsa public key
    pub rsa_public_key: String,
    /// If this api key belongs to master account or not
    pub is_master: bool,
    /// The main account uid. Returns "0" when the endpoint is called by main account
    pub parent_uid: String,
    /// Personal account kyc level. LEVEL_DEFAULT, LEVEL_1， LEVEL_2
    pub kyc_level: String,
    /// Personal account kyc region
    pub kyc_region: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct APIKeyPermissions {
    /// Permission of contract trade Order, Position
    pub contract_trade: Vec<String>,
    /// Permission of spot SpotTrade
    pub spot: Vec<String>,
    /// Permission of wallet AccountTransfer, SubMemberTransfer(master account), SubMemberTransferList(sub account), Withdraw(master account)
    pub wallet: Vec<String>,
    /// Permission of USDC Contract. It supports trade option and USDC perpetual. OptionsTrade
    pub options: Vec<String>,
    /// Unified account has this permission by default DerivativesTrade
    /// For classic account, it is always []
    pub derivatives: Vec<String>,
    /// Permission of convert ExchangeHistory
    pub exchange: Vec<String>,
    /// Permission of earn product Earn
    #[serde(default)]
    pub earn: Vec<String>,
    /// Permission of blocktrade. Not applicable to subaccount, always []
    pub block_trade: Vec<String>,
    /// Permission of Affiliate. Only affiliate can have this permission, otherwise always []
    pub affiliate: Vec<String>,
    /// Always [] as Master Trader account just use ContractTrade to start CopyTrading
    pub copy_trading: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionLogParams {
    /// Account Type. UNIFIED
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<AccountType>,
    /// Product type spot,linear,option,inverse
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Category>,
    /// Currency, uppercase only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// BaseCoin, uppercase only. e.g., BTC of BTCPERP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    /// Not documented.
    /// Settle coin
    /// linear: either symbol or settleCoin is required. symbol has a higher priority
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_coin: Option<String>,
    /// Types of transaction logs
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub log_type: Option<String>,
    /// movePosition, used to filter trans logs of Move Position only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trans_sub_type: Option<String>,
    /// The start timestamp (ms)
    /// startTime and endTime are not passed, return 24 hours by default
    /// Only startTime is passed, return range between startTime and startTime+24 hours
    /// Only endTime is passed, return range between endTime-24 hours and endTime
    /// If both are passed, the rule is endTime - startTime <= 7 days
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Timestamp>,
    /// The end timestamp (ms)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<Timestamp>,
    /// Limit for data size per page. [1, 50]. Default: 20
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    /// Cursor. Use the nextPageCursor token from the response to retrieve the next page of the result set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TransactionLog {
    /// Unique id
    pub id: String,
    /// Symbol name
    pub symbol: String,
    /// Product type
    pub category: Category,
    /// Side. Buy,Sell,None
    pub side: Side,
    /// Transaction timestamp (ms)
    #[serde(deserialize_with = "number")]
    pub transaction_time: Timestamp,
    /// Type
    #[serde(rename = "type")]
    pub log_type: String,
    /// Transaction sub type, movePosition, used for the logs generated by move position. "" by default
    pub trans_sub_type: String,
    /// Quantity
    /// Spot: the negative means the qty of this currency is decreased, the positive means the qty of this currency is increased
    /// Perps & Futures: it is the quantity for each trade entry and it does not have direction
    pub qty: Decimal,
    /// Size. The rest position size after the trade is executed, and it has direction, i.e., short with "-"
    pub size: Decimal,
    /// e.g., USDC, USDT, BTC, ETH
    pub currency: String,
    /// Trade price
    pub trade_price: Decimal,
    /// Funding fee
    /// Positive fee value means receive funding; negative fee value means pay funding. This is opposite to the execFee from Get Trade History.
    /// For USDC Perp, as funding settlement and session settlement occur at the same time, they are represented in a single record at settlement. Please refer to funding to understand funding fee, and cashFlow to understand 8-hour P&L.
    pub funding: Decimal,
    /// Trading fee
    /// Positive fee value means expense
    /// Negative fee value means rebates
    pub fee: Decimal,
    /// Cash flow, e.g., (1) close the position, and unRPL converts to RPL, (2) 8-hour session settlement for USDC Perp and Futures, (3) transfer in or transfer out. This does not include trading fee, funding fee
    pub cash_flow: Decimal,
    /// Change = cashFlow + funding - fee
    pub change: Decimal,
    /// Cash balance. This is the wallet balance after a cash change
    pub cash_balance: Decimal,
    ///
    /// When type=TRADE, then it is trading fee rate
    /// When type=SETTLEMENT, it means funding fee rate. For side=Buy, feeRate=market fee rate; For side=Sell, feeRate= - market fee rate
    pub fee_rate: Decimal,
    /// The change of bonus
    pub bonus_change: Decimal,
    /// Trade ID
    pub trade_id: String,
    /// Order ID
    pub order_id: String,
    /// User customised order ID
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub order_link_id: Option<String>,
    /// Trading fee rate information. Currently, this data is returned only for spot orders placed on the Indonesian site or spot fiat currency orders placed on the EU site. In other cases, an empty string is returned. Enum: feeType, subFeeType
    #[serde(default)]
    pub extra_fees: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use rust_decimal::dec;

    use crate::v5::serde::deserialize_str;

    use super::*;

    #[test]
    fn deserialize_response_kline_inverse() {
        let json = r#"{
            "retCode": 0,
            "retMsg": "OK",
            "result": {
                "symbol": "BTCUSD",
                "category": "inverse",
                "list": [
                    [
                        "1670608800000",
                        "17071",
                        "17073",
                        "17027",
                        "17055.5",
                        "268611",
                        "15.74462667"
                    ],
                    [
                        "1670605200000",
                        "17071.5",
                        "17071.5",
                        "17061",
                        "17071",
                        "4177",
                        "0.24469757"
                    ],
                    [
                        "1670601600000",
                        "17086.5",
                        "17088",
                        "16978",
                        "17071.5",
                        "6356",
                        "0.37288112"
                    ]
                ]
            },
            "retExtInfo": {},
            "time": 1672025956592
        }"#;
        let expected = Resp {
            ret_code: 0,
            ret_msg: String::from("OK"),
            result: KLine::Inverse {
                symbol: String::from("BTCUSD"),
                list: vec![
                    KLineRow {
                        start_time: 1670608800000,
                        open_price: dec!(17071),
                        high_price: dec!(17073),
                        low_price: dec!(17027),
                        close_price: dec!(17055.5),
                        volume: dec!(268611),
                        turnover: dec!(15.74462667),
                    },
                    KLineRow {
                        start_time: 1670605200000,
                        open_price: dec!(17071.5),
                        high_price: dec!(17071.5),
                        low_price: dec!(17061),
                        close_price: dec!(17071),
                        volume: dec!(4177),
                        turnover: dec!(0.24469757),
                    },
                    KLineRow {
                        start_time: 1670601600000,
                        open_price: dec!(17086.5),
                        high_price: dec!(17088),
                        low_price: dec!(16978),
                        close_price: dec!(17071.5),
                        volume: dec!(6356),
                        turnover: dec!(0.37288112),
                    },
                ],
            },
            time: Some(1672025956592),
            ret_ext_info: Some(RetExtInfo {}),
        };

        let message = deserialize_str(json).unwrap();

        assert_eq!(expected, message);
    }

    #[test]
    fn deserialize_response_ticker_inverse() {
        let json = r#"{
            "retCode": 0,
            "retMsg": "OK",
            "result": {
                "category": "inverse",
                "list": [
                    {
                        "symbol": "BTCUSD",
                        "lastPrice": "16597.00",
                        "indexPrice": "16598.54",
                        "markPrice": "16596.00",
                        "prevPrice24h": "16464.50",
                        "price24hPcnt": "0.008047",
                        "highPrice24h": "30912.50",
                        "lowPrice24h": "15700.00",
                        "prevPrice1h": "16595.50",
                        "openInterest": "373504107",
                        "openInterestValue": "22505.67",
                        "turnover24h": "2352.94950046",
                        "volume24h": "49337318",
                        "fundingRate": "-0.001034",
                        "nextFundingTime": "1672387200000",
                        "predictedDeliveryPrice": "",
                        "basisRate": "",
                        "deliveryFeeRate": "",
                        "deliveryTime": "0",
                        "ask1Size": "1",
                        "bid1Price": "16596.00",
                        "ask1Price": "16597.50",
                        "bid1Size": "1",
                        "basis": ""
                    }
                ]
            },
            "retExtInfo": {},
            "time": 1672376496682
        }"#;
        let expected = Resp {
            ret_code: 0,
            ret_msg: String::from("OK"),
            result: Ticker::Inverse {
                list: vec![LinearInverseTicker {
                    symbol: String::from("BTCUSD"),
                    last_price: dec!(16597.00),
                    mark_price: dec!(16596.00),
                    index_price: dec!(16598.54),
                    prev_price24h: dec!(16464.50),
                    price24h_pcnt: dec!(0.008047),
                    high_price24h: dec!(30912.50),
                    low_price24h: dec!(15700.00),
                    prev_price1h: dec!(16595.50),
                    open_interest: dec!(373504107),
                    open_interest_value: dec!(22505.67),
                    turnover24h: dec!(2352.94950046),
                    volume24h: dec!(49337318),
                    funding_rate: Some(dec!(-0.001034)),
                    next_funding_time: 1672387200000,
                    predicted_delivery_price: None,
                    basis_rate: None,
                    basis: None,
                    delivery_fee_rate: None,
                    delivery_time: Some(0),
                    bid1_price: dec!(16596.00),
                    bid1_size: dec!(1),
                    ask1_price: dec!(16597.50),
                    ask1_size: dec!(1),
                    pre_open_price: None,
                    pre_qty: None,
                    cur_pre_listing_phase: None,
                }],
            },
            time: Some(1672376496682),
            ret_ext_info: Some(RetExtInfo {}),
        };

        let message = deserialize_str(json).unwrap();

        assert_eq!(expected, message);
    }

    #[test]
    fn deserialize_response_trad_spot() {
        let json = r#"{
            "retCode": 0,
            "retMsg": "OK",
            "result": {
                "category": "spot",
                "list": [
                    {
                        "execId": "2100000000007764263",
                        "symbol": "BTCUSDT",
                        "price": "16618.49",
                        "size": "0.00012",
                        "side": "Buy",
                        "time": "1672052955758",
                        "isBlockTrade": false,
                        "isRPITrade": true
                    }
                ]
            },
            "retExtInfo": {},
            "time": 1672053054358
        }"#;
        let expected = Resp {
            ret_code: 0,
            ret_msg: String::from("OK"),
            result: Trade::Spot {
                list: vec![InverseLinearSpotTrade {
                    exec_id: String::from("2100000000007764263"),
                    symbol: String::from("BTCUSDT"),
                    price: dec!(16618.49),
                    size: dec!(0.00012),
                    side: Side::Buy,
                    time: 1672052955758,
                    is_block_trade: false,
                    is_rpi_trade: true,
                }],
            },
            time: Some(1672053054358),
            ret_ext_info: Some(RetExtInfo {}),
        };

        let message = deserialize_str(json).unwrap();

        assert_eq!(expected, message);
    }

    #[test]
    fn deserialize_response_get_open_closed_orders_linear() {
        let json = r#"{
            "retCode": 0,
            "retMsg": "OK",
            "result": {
                "list": [
                    {
                        "orderId": "fd4300ae-7847-404e-b947-b46980a4d140",
                        "orderLinkId": "test-000005",
                        "blockTradeId": "",
                        "symbol": "ETHUSDT",
                        "price": "1600.00",
                        "qty": "0.10",
                        "side": "Buy",
                        "isLeverage": "",
                        "positionIdx": 1,
                        "orderStatus": "New",
                        "cancelType": "UNKNOWN",
                        "rejectReason": "EC_NoError",
                        "avgPrice": "0",
                        "leavesQty": "0.10",
                        "leavesValue": "160",
                        "cumExecQty": "0.00",
                        "cumExecValue": "0",
                        "cumExecFee": "0",
                        "timeInForce": "GTC",
                        "orderType": "Limit",
                        "stopOrderType": "UNKNOWN",
                        "orderIv": "",
                        "triggerPrice": "0.00",
                        "takeProfit": "2500.00",
                        "stopLoss": "1500.00",
                        "tpTriggerBy": "LastPrice",
                        "slTriggerBy": "LastPrice",
                        "triggerDirection": 0,
                        "triggerBy": "UNKNOWN",
                        "lastPriceOnCreated": "",
                        "reduceOnly": false,
                        "closeOnTrigger": false,
                        "smpType": "None",
                        "smpGroup": 0,
                        "smpOrderId": "",
                        "tpslMode": "Full",
                        "tpLimitPrice": "",
                        "slLimitPrice": "",
                        "placeType": "",
                        "createdTime": "1684738540559",
                        "updatedTime": "1684738540561"
                    }
                ],
                "nextPageCursor": "page_args%3Dfd4300ae-7847-404e-b947-b46980a4d140%26symbol%3D6%26",
                "category": "linear"
            },
            "retExtInfo": {},
            "time": 1684765770483
        }"#;
        let expected = Resp {
            ret_code: 0,
            ret_msg: String::from("OK"),
            result: CursorPagination {
                category: Some(Category::Linear),
                next_page_cursor: Some(String::from(
                    "page_args%3Dfd4300ae-7847-404e-b947-b46980a4d140%26symbol%3D6%26",
                )),
                list: vec![Order {
                    order_id: String::from("fd4300ae-7847-404e-b947-b46980a4d140"),
                    order_link_id: Some(String::from("test-000005")),
                    block_trade_id: None,
                    symbol: String::from("ETHUSDT"),
                    price: dec!(1600.00),
                    qty: dec!(0.10),
                    side: Side::Buy,
                    is_leverage: None,
                    position_idx: PositionIdx::Buy,
                    order_status: OrderStatus::New,
                    create_type: None,
                    cancel_type: CancelType::UNKNOWN,
                    reject_reason: RejectReason::EcNoError,
                    avg_price: Some(dec!(0.0)),
                    leaves_qty: dec!(0.10),
                    leaves_value: dec!(160),
                    cum_exec_qty: dec!(0.00),
                    cum_exec_value: dec!(0),
                    cum_exec_fee: dec!(0),
                    time_in_force: TimeInForce::GTC,
                    order_type: OrderType::Limit,
                    stop_order_type: Some(StopOrderType::UNKNOWN),
                    order_iv: None,
                    market_unit: None,
                    trigger_price: Some(dec!(0.00)),
                    take_profit: Some(dec!(2500.00)),
                    stop_loss: Some(dec!(1500.00)),
                    tpsl_mode: Some(TpslMode::Full),
                    oco_trigger_by: None,
                    tp_limit_price: None,
                    sl_limit_price: None,
                    tp_trigger_by: Some(TriggerBy::LastPrice),
                    sl_trigger_by: Some(TriggerBy::LastPrice),
                    trigger_direction: TriggerDirection::UNKNOWN,
                    trigger_by: Some(TriggerBy::UNKNOWN),
                    last_price_on_created: None,
                    base_price: None,
                    reduce_only: false,
                    close_on_trigger: false,
                    place_type: None,
                    smp_type: SmpType::None,
                    smp_group: 0,
                    smp_order_id: None,
                    created_time: 1684738540559,
                    updated_time: 1684738540561,
                }],
            },
            time: Some(1684765770483),
            ret_ext_info: Some(RetExtInfo {}),
        };

        let message = deserialize_str(json).unwrap();

        assert_eq!(expected, message);
    }

    #[test]
    fn deserialize_response_get_open_closed_orders_linear2() {
        let json = r#"{
            "retCode":0,
            "retMsg":"OK",
            "result":{
                "nextPageCursor":"aed77e97-492f-45be-8ada-4ff350ec07a5%3A1762701687113%2Caed77e97-492f-45be-8ada-4ff350ec07a5%3A1762701687113",
                "category":"linear",
                "list":[
                    {
                        "symbol":"BTCUSDT",
                        "orderType":"Limit",
                        "orderLinkId":"",
                        "slLimitPrice":"0",
                        "orderId":"aed77e97-492f-45be-8ada-4ff350ec07a5",
                        "cancelType":"UNKNOWN",
                        "avgPrice":"",
                        "stopOrderType":"",
                        "lastPriceOnCreated":"103550",
                        "orderStatus":"New",
                        "createType":"CreateByUser",
                        "takeProfit":"",
                        "cumExecValue":"0",
                        "tpslMode":"",
                        "smpType":"None",
                        "triggerDirection":0,
                        "blockTradeId":"",
                        "isLeverage":"",
                        "rejectReason":"EC_NoError",
                        "price":"103000",
                        "orderIv":"",
                        "createdTime":"1762701687113",
                        "tpTriggerBy":"",
                        "positionIdx":1,
                        "timeInForce":"GTC",
                        "leavesValue":"1030",
                        "updatedTime":"1762701687113",
                        "side":"Buy",
                        "smpGroup":0,
                        "triggerPrice":"",
                        "tpLimitPrice":"0",
                        "cumExecFee":"0",
                        "leavesQty":"0.01",
                        "slTriggerBy":"",
                        "closeOnTrigger":false,
                        "placeType":"",
                        "cumExecQty":"0",
                        "reduceOnly":false,
                        "qty":"0.01",
                        "stopLoss":"",
                        "marketUnit":"",
                        "smpOrderId":"",
                        "triggerBy":""
                    }
                ]
            },
            "retExtInfo":{},
            "time":1762711342768
        }"#;
        let expected = Resp {
            ret_code: 0,
            ret_msg: String::from("OK"),
            result: CursorPagination {
                category: Some(Category::Linear),
                next_page_cursor: Some(String::from(
                    "aed77e97-492f-45be-8ada-4ff350ec07a5%3A1762701687113%2Caed77e97-492f-45be-8ada-4ff350ec07a5%3A1762701687113",
                )),
                list: vec![Order {
                    order_id: String::from("aed77e97-492f-45be-8ada-4ff350ec07a5"),
                    order_link_id: None,
                    block_trade_id: None,
                    symbol: String::from("BTCUSDT"),
                    price: dec!(103000),
                    qty: dec!(0.01),
                    side: Side::Buy,
                    is_leverage: None,
                    position_idx: PositionIdx::Buy,
                    order_status: OrderStatus::New,
                    create_type: Some(CreateType::CreateByUser),
                    cancel_type: CancelType::UNKNOWN,
                    reject_reason: RejectReason::EcNoError,
                    avg_price: None,
                    leaves_qty: dec!(0.01),
                    leaves_value: dec!(1030),
                    cum_exec_qty: dec!(0),
                    cum_exec_value: dec!(0),
                    cum_exec_fee: dec!(0),
                    time_in_force: TimeInForce::GTC,
                    order_type: OrderType::Limit,
                    stop_order_type: None,
                    order_iv: None,
                    market_unit: None,
                    trigger_price: None,
                    take_profit: None,
                    stop_loss: None,
                    tpsl_mode: None,
                    oco_trigger_by: None,
                    tp_limit_price: Some(dec!(0)),
                    sl_limit_price: Some(dec!(0)),
                    tp_trigger_by: None,
                    sl_trigger_by: None,
                    trigger_direction: TriggerDirection::UNKNOWN,
                    trigger_by: None,
                    last_price_on_created: Some(dec!(103550)),
                    base_price: None,
                    reduce_only: false,
                    close_on_trigger: false,
                    place_type: None,
                    smp_type: SmpType::None,
                    smp_group: 0,
                    smp_order_id: None,
                    created_time: 1762701687113,
                    updated_time: 1762701687113,
                }],
            },
            time: Some(1762711342768),
            ret_ext_info: Some(RetExtInfo {}),
        };

        let message = deserialize_str(json).unwrap();

        assert_eq!(expected, message);
    }

    #[test]
    fn deserialize_response_get_position_info_inverse() {
        let json = r#"{
            "retCode": 0,
            "retMsg": "OK",
            "result": {
                "list": [
                    {
                        "positionIdx": 0,
                        "riskId": 1,
                        "riskLimitValue": "150",
                        "symbol": "BTCUSD",
                        "side": "Sell",
                        "size": "300",
                        "avgPrice": "27464.50441675",
                        "positionValue": "0.01092319",
                        "tradeMode": 0,
                        "positionStatus": "Normal",
                        "autoAddMargin": 1,
                        "adlRankIndicator": 2,
                        "leverage": "10",
                        "positionBalance": "0.00139186",
                        "markPrice": "28224.50",
                        "liqPrice": "",
                        "bustPrice": "999999.00",
                        "positionMM": "0.0000015",
                        "positionMMByMp": "0.0000015",
                        "positionIM": "0.00010923",
                        "positionIMByMp": "0.00010923",
                        "tpslMode": "Full",
                        "takeProfit": "0.00",
                        "stopLoss": "0.00",
                        "trailingStop": "0.00",
                        "unrealisedPnl": "-0.00029413",
                        "curRealisedPnl": "0.00013123",
                        "cumRealisedPnl": "-0.00096902",
                        "seq": 5723621632,
                        "isReduceOnly": false,
                        "mmrSysUpdatedTime": "",
                        "leverageSysUpdatedTime": "",
                        "sessionAvgPrice": "",
                        "createdTime": "1676538056258",
                        "updatedTime": "1697673600012"
                    }
                ],
                "nextPageCursor": "",
                "category": "inverse"
            },
            "retExtInfo": {},
            "time": 1697684980172
        }"#;
        let expected = Resp {
            ret_code: 0,
            ret_msg: String::from("OK"),
            result: CursorPagination {
                category: Some(Category::Inverse),
                next_page_cursor: None,
                list: vec![Position {
                    position_idx: PositionIdx::OneWay,
                    risk_id: 1,
                    risk_limit_value: Some(dec!(150)),
                    symbol: String::from("BTCUSD"),
                    side: Some(Side::Sell),
                    size: dec!(300),
                    avg_price: dec!(27464.50441675),
                    position_value: Some(dec!(0.01092319)),
                    auto_add_margin: true,
                    position_status: PositionStatus::Normal,
                    leverage: dec!(10),
                    mark_price: dec!(28224.50),
                    liq_price: None,
                    position_im: Some(dec!(0.00010923)),
                    position_im_by_mp: Some(dec!(0.00010923)),
                    position_mm: Some(dec!(0.0000015)),
                    position_mm_by_mp: Some(dec!(0.0000015)),
                    take_profit: Some(dec!(0.00)),
                    stop_loss: Some(dec!(0.00)),
                    trailing_stop: Some(dec!(0.00)),
                    session_avg_price: None,
                    delta: None,
                    gamma: None,
                    vega: None,
                    theta: None,
                    unrealised_pnl: Some(dec!(-0.00029413)),
                    cur_realised_pnl: dec!(0.00013123),
                    cum_realised_pnl: dec!(-0.00096902),
                    adl_rank_indicator: AdlRankIndicator::Two,
                    created_time: 1676538056258,
                    updated_time: 1697673600012,
                    seq: 5723621632,
                    is_reduce_only: false,
                    mmr_sys_updated_time: None,
                    leverage_sys_updated_time: None,
                }],
            },
            time: Some(1697684980172),
            ret_ext_info: Some(RetExtInfo {}),
        };

        let message: Resp<CursorPagination<Position>> = deserialize_str(json).unwrap();

        assert_eq!(message, expected);
    }

    #[test]
    fn deserialize_response_get_wallet_balance() {
        let json = r#"{
            "retCode": 0,
            "retMsg": "OK",
            "result": {
                "list": [
                    {
                        "totalEquity": "3.31216591",
                        "accountIMRate": "0",
                        "accountIMRateByMp": "0",
                        "totalMarginBalance": "3.00326056",
                        "totalInitialMargin": "0",
                        "totalInitialMarginByMp": "0",
                        "accountType": "UNIFIED",
                        "totalAvailableBalance": "3.00326056",
                        "accountMMRate": "0",
                        "accountMMRateByMp": "0",
                        "totalPerpUPL": "0",
                        "totalWalletBalance": "3.00326056",
                        "accountLTV": "0",
                        "totalMaintenanceMargin": "0",
                        "totalMaintenanceMarginByMp": "0",
                        "coin": [
                            {
                                "availableToBorrow": "3",
                                "bonus": "0",
                                "accruedInterest": "0",
                                "availableToWithdraw": "0",
                                "totalOrderIM": "0",
                                "equity": "0",
                                "totalPositionMM": "0",
                                "usdValue": "0",
                                "spotHedgingQty": "0.01592413",
                                "unrealisedPnl": "0",
                                "collateralSwitch": true,
                                "borrowAmount": "0.0",
                                "totalPositionIM": "0",
                                "walletBalance": "0",
                                "cumRealisedPnl": "0",
                                "locked": "0",
                                "marginCollateral": true,
                                "coin": "BTC",
                                "spotBorrow": "0"
                            }
                        ]
                    }
                ]
            },
            "retExtInfo": {},
            "time": 1690872862481
        }"#;
        let coin = WalletCoin {
            coin: String::from("BTC"),
            equity: dec!(0),
            usd_value: dec!(0),
            wallet_balance: dec!(0),
            locked: dec!(0),
            spot_hedging_qty: dec!(0.01592413),
            borrow_amount: dec!(0.0),
            accrued_interest: dec!(0),
            total_order_im: Some(dec!(0)),
            total_position_im: Some(dec!(0)),
            total_position_mm: Some(dec!(0)),
            unrealised_pnl: dec!(0),
            cum_realised_pnl: dec!(0),
            bonus: dec!(0),
            margin_collateral: true,
            collateral_switch: true,
            spot_borrow: Some(dec!(0)),
        };
        let coin = HashMap::from([(Unique::unique_key(&coin), coin)]);
        let expected = Resp {
            ret_code: 0,
            ret_msg: String::from("OK"),
            result: List {
                list: vec![WalletBalance {
                    account_type: AccountType::UNIFIED,
                    account_im_rate: dec!(0),
                    account_im_rate_by_mp: dec!(0),
                    account_mm_rate: dec!(0),
                    account_mm_rate_by_mp: dec!(0),
                    total_equity: dec!(3.31216591),
                    total_wallet_balance: dec!(3.00326056),
                    total_margin_balance: dec!(3.00326056),
                    total_available_balance: dec!(3.00326056),
                    total_perp_upl: dec!(0),
                    total_initial_margin: dec!(0),
                    total_initial_margin_by_mp: dec!(0),
                    total_maintenance_margin: dec!(0),
                    total_maintenance_margin_by_mp: dec!(0),
                    coin,
                }],
            },
            time: Some(1690872862481),
            ret_ext_info: Some(RetExtInfo {}),
        };

        let message = deserialize_str(json).unwrap();

        assert_eq!(expected, message);
    }

    #[test]
    fn deserialize_response_get_wallet_balance_2() {
        let json = r#"{
            "retCode":0,
            "retMsg":"OK",
            "result":{
                "list":[
                    {
                        "totalEquity":"36.42053792",
                        "accountIMRate":"0",
                        "accountIMRateByMp":"0",
                        "totalMarginBalance":"36.42053792",
                        "totalInitialMargin":"0",
                        "totalInitialMarginByMp":"0",
                        "accountType":"UNIFIED",
                        "totalAvailableBalance":"36.42053792",
                        "accountMMRate":"0",
                        "accountMMRateByMp":"0",
                        "totalPerpUPL":"0",
                        "totalWalletBalance":"36.42053792",
                        "accountLTV":"0",
                        "totalMaintenanceMargin":"0",
                        "totalMaintenanceMarginByMp":"0",
                        "coin":[
                            {
                                "availableToBorrow":"",
                                "bonus":"0",
                                "accruedInterest":"0",
                                "availableToWithdraw":"",
                                "totalOrderIM":"0",
                                "equity":"36.4061211",
                                "totalPositionMM":"0",
                                "usdValue":"36.42053792",
                                "unrealisedPnl":"0",
                                "collateralSwitch":true,
                                "spotHedgingQty":"0",
                                "borrowAmount":"0.000000000000000000",
                                "totalPositionIM":"0",
                                "walletBalance":"36.4061211",
                                "cumRealisedPnl":"-2084.9938789",
                                "locked":"0",
                                "marginCollateral":true,
                                "coin":"USDT",
                                "spotBorrow": "0"
                            }
                        ]
                    }
                ]
            },
            "retExtInfo":{},
            "time":1751570498412
        }"#;
        let coin = WalletCoin {
            coin: String::from("USDT"),
            equity: dec!(36.4061211),
            usd_value: dec!(36.42053792),
            wallet_balance: dec!(36.4061211),
            locked: dec!(0),
            spot_hedging_qty: dec!(0),
            borrow_amount: dec!(0.000000000000000000),
            accrued_interest: dec!(0),
            total_order_im: Some(dec!(0)),
            total_position_im: Some(dec!(0)),
            total_position_mm: Some(dec!(0)),
            unrealised_pnl: dec!(0),
            cum_realised_pnl: dec!(-2084.9938789),
            bonus: dec!(0),
            margin_collateral: true,
            collateral_switch: true,
            spot_borrow: Some(dec!(0)),
        };
        let coin = HashMap::from([(Unique::unique_key(&coin), coin)]);
        let expected = Resp {
            ret_code: 0,
            ret_msg: String::from("OK"),
            result: List {
                list: vec![WalletBalance {
                    account_type: AccountType::UNIFIED,
                    account_im_rate: dec!(0),
                    account_im_rate_by_mp: dec!(0),
                    account_mm_rate: dec!(0),
                    account_mm_rate_by_mp: dec!(0),
                    total_equity: dec!(36.42053792),
                    total_wallet_balance: dec!(36.42053792),
                    total_margin_balance: dec!(36.42053792),
                    total_available_balance: dec!(36.42053792),
                    total_perp_upl: dec!(0),
                    total_initial_margin: dec!(0),
                    total_initial_margin_by_mp: dec!(0),
                    total_maintenance_margin: dec!(0),
                    total_maintenance_margin_by_mp: dec!(0),
                    coin,
                }],
            },
            time: Some(1751570498412),
            ret_ext_info: Some(RetExtInfo {}),
        };

        let message = deserialize_str(json).unwrap();

        assert_eq!(expected, message);
    }

    #[test]
    fn deserialize_response_get_account_info() {
        let json = r#"{
            "retCode": 0,
            "retMsg": "OK",
            "result": {
                "marginMode": "REGULAR_MARGIN",
                "updatedTime": "1697078946000",
                "unifiedMarginStatus": 4,
                "dcpStatus": "OFF",
                "timeWindow": 10,
                "smpGroup": 0,
                "isMasterTrader": false,
                "spotHedgingStatus": "OFF"
            }
        }"#;
        let expected = Resp {
            ret_code: 0,
            ret_msg: String::from("OK"),
            result: AccountInfo {
                unified_margin_status: UnifiedMarginStatus::UnifiedTradingAccount1Pro,
                margin_mode: MarginMode::RegularMargin,
                is_master_trader: false,
                spot_hedging_status: SpotHedgingStatus::Off,
                updated_time: 1697078946000,
            },
            time: None,
            ret_ext_info: None,
        };

        let message = deserialize_str(json).unwrap();

        assert_eq!(expected, message);
    }
}
