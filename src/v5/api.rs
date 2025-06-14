use rust_decimal::{Decimal, serde::str_option::deserialize as option_decimal};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::{
    deserialize_number_from_string as number,
    deserialize_option_number_from_string as option_number,
};

use crate::v5::{
    AdlRankIndicator, CancelType, ContractType, CopyTrading, CreateType, CurAuctionPhase,
    OcoTriggerBy, OrderStatus, OrderType, PlaceType, PositionIdx, PositionStatus, RejectReason,
    Side, SmpType, Status, StopOrderType, TimeInForce, TpslMode, TradeMode, TriggerBy,
    TriggerDirection,
    enums::{Category, Interval},
    serde::{
        empty_string_as_none, int_to_bool, invalid_as_none, string_to_bool, string_to_option_bool,
    },
};

pub type Timestamp = u64;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Resp<T> {
    pub ret_code: i64,
    pub ret_msg: String,
    pub result: T,
    pub time: Timestamp,
    pub ret_ext_info: RetExtInfo,
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
    pub time: Timestamp,
    pub headers: Headers,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CursorPagination<T> {
    pub category: Category,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub next_page_cursor: Option<String>,
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
    /// The change in the last 24 hous
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
    /// optionType	false	string	Option type. Call or Put. Apply to option only
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
    /// boolean	Whether the trade is block trade
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
    /// boolean	Whether the trade is block trade
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
    #[serde(deserialize_with = "option_number")]
    pub delivery_fee_rate: Option<f64>,
    #[serde(deserialize_with = "number")]
    pub price_scale: i64,
    pub leverage_filter: LeverageFilter,
    pub price_filter: PriceFilter,
    pub lot_size_filter: LotSizeFilter,
    pub unified_margin_trade: bool,
    pub funding_interval: i64,
    pub settle_coin: String,
    pub copy_trading: CopyTrading,
    #[serde(deserialize_with = "number")]
    pub upper_funding_rate: f64,
    #[serde(deserialize_with = "number")]
    pub lower_funding_rate: f64,
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
    #[serde(deserialize_with = "option_number")]
    pub delivery_fee_rate: Option<f64>,
    #[serde(deserialize_with = "number")]
    pub price_scale: i64,
    pub leverage_filter: LeverageFilter,
    pub price_filter: PriceFilter,
    pub lot_size_filter: LotSizeFilter,
    pub unified_margin_trade: bool,
    pub funding_interval: i64,
    pub settle_coin: String,
    pub copy_trading: CopyTrading,
    #[serde(deserialize_with = "number")]
    pub upper_funding_rate: f64,
    #[serde(deserialize_with = "number")]
    pub lower_funding_rate: f64,
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
    #[serde(deserialize_with = "number")]
    pub min_leverage: f64,
    #[serde(deserialize_with = "number")]
    pub max_leverage: f64,
    #[serde(deserialize_with = "number")]
    pub leverage_step: f64,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PriceFilter {
    #[serde(deserialize_with = "number")]
    pub min_price: f64,
    #[serde(deserialize_with = "number")]
    pub max_price: f64,
    #[serde(deserialize_with = "number")]
    pub tick_size: f64,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SpotPriceFilter {
    /// The step to increase/reduce order price
    #[serde(deserialize_with = "number")]
    pub tick_size: f64,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LotSizeFilter {
    #[serde(deserialize_with = "number")]
    pub min_notional_value: f64,
    #[serde(deserialize_with = "number")]
    pub max_order_qty: f64,
    #[serde(deserialize_with = "number")]
    pub max_mkt_order_qty: f64,
    #[serde(deserialize_with = "number")]
    pub min_order_qty: f64,
    #[serde(deserialize_with = "number")]
    pub qty_step: f64,
    #[serde(deserialize_with = "number")]
    pub post_only_max_order_qty: f64,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SpotLotSizeFilter {
    /// The precision of base coin
    #[serde(deserialize_with = "number")]
    pub base_precision: f64,
    /// The precision of quote coin
    #[serde(deserialize_with = "number")]
    pub quote_precision: f64,
    /// Minimum order quantity
    #[serde(deserialize_with = "number")]
    pub min_order_qty: f64,
    /// Maximum order quantity
    #[serde(deserialize_with = "number")]
    pub max_order_qty: f64,
    /// Minimum order amount
    #[serde(deserialize_with = "number")]
    pub min_order_amt: f64,
    /// Maximum order amount
    #[serde(deserialize_with = "number")]
    pub max_order_amt: f64,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RiskParameters {
    #[serde(deserialize_with = "number")]
    pub price_limit_ratio_x: f64,
    #[serde(deserialize_with = "number")]
    pub price_limit_ratio_y: f64,
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
    #[serde(deserialize_with = "number")]
    pub auction_fee_rate: f64,
    #[serde(deserialize_with = "number")]
    pub taker_fee_rate: f64,
    #[serde(deserialize_with = "number")]
    pub maker_fee_rate: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOpenClosedOrdersParams {
    /// Product type
    /// UTA2.0, UTA1.0: linear, inverse, spot, option
    /// classic account: linear, inverse, spot
    pub category: Category,
    /// Symbol name, like BTCUSDT, uppercase only. For linear, either symbol, baseCoin, settleCoin is required
    pub symbol: Option<String>,
    /// Base coin, uppercase only
    /// Supports linear, inverse & option
    /// option: it returns all option open orders by default
    pub base_coin: Option<String>,
    /// Settle coin, uppercase only
    /// linear: either symbol, baseCoin or settleCoin is required
    /// spot: not supported
    /// option: USDT or USDC
    pub settle_coin: Option<String>,
    /// Order ID
    pub order_id: Option<String>,
    /// User customised order ID
    pub order_link_id: Option<String>,
    /// 0(default): UTA2.0, UTA1.0, classic account query open status orders (e.g., New, PartiallyFilled) only
    /// 1: UTA2.0, UTA1.0(except inverse)
    /// 2: UTA1.0(inverse), classic account
    /// Query a maximum of recent 500 closed status records are kept under each account each category (e.g., Cancelled, Rejected, Filled orders).
    /// If the Bybit service is restarted due to an update, this part of the data will be cleared and accumulated again, but the order records will still be queried in order history
    /// openOnly param will be ignored when query by orderId or orderLinkId
    /// Classic spot: not supported
    pub open_only: Option<i32>,
    /// Order: active order,
    /// StopOrder: conditional order for Futures and Spot,
    /// tpslOrder: spot TP/SL order,
    /// OcoOrder: Spot oco order,
    /// BidirectionalTpslOrder: Spot bidirectional TPSL order
    /// - classic account spot: return Order active order by default
    /// - Others: all kinds of orders by default
    #[serde(default, deserialize_with = "invalid_as_none")]
    pub order_filter: Option<OrderFilter>,
    /// Limit for data size per page. [1, 50]. Default: 20
    pub limit: Option<i32>,
    /// Cursor. Use the nextPageCursor token from the response to retrieve the next page of the result set
    pub cursor: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum OrderFilter {
    /// active order,
    Order,
    /// conditional order for Futures and Spot,
    StopOrder,
    /// spot TP/SL order,
    TpslOrder,
    /// Spot oco order,
    OcoOrder,
    /// Spot bidirectional TPSL order
    /// - classic account spot: return Order active order by default
    /// - Others: all kinds of orders by default
    BidirectionalTpslOrder,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    /// Order ID
    pub order_id: String,
    /// User customised order ID
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
    /// classic account: returns "0" for those orders without avg price, and also for those orders have partilly filled but cancelled at the end
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
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub order_iv: Option<String>,
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
    pub trigger_by: TriggerBy,
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
    pub symbol: Option<String>,
    /// Base coin, uppercase only. option only. Return all option positions if not passed
    pub base_coin: Option<String>,
    /// Settle coin
    /// linear: either symbol or settleCoin is required. symbol has a higher priority
    pub settle_coin: Option<String>,
    /// Limit for data size per page. [1, 200]. Default: 20
    pub limit: Option<u64>,
    /// Cursor. Use the nextPageCursor token from the response to retrieve the next page of the result set
    pub cursor: Option<String>,
}

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
    pub risk_limit_value: Decimal,
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
    /// Trade mode
    /// Classic & UTA1.0(inverse): 0: cross-margin, 1: isolated margin
    /// UTA2.0, UTA1.0(execpt inverse): deprecated, always 0, check Get Account Info to know the margin mode
    pub trade_mode: TradeMode,
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
    /// Bankruptcy price
    #[serde(default, deserialize_with = "option_decimal")]
    pub bust_price: Option<Decimal>,
    /// Initial margin
    /// Classic & UTA1.0(inverse): ignore this field
    /// UTA portfolio margin mode, it returns ""
    #[serde(rename = "positionIM")]
    pub position_im: Decimal,
    /// Maintenance margin
    /// Classic & UTA1.0(inverse): ignore this field
    /// UTA portfolio margin mode, it returns ""
    #[serde(rename = "positionMM")]
    pub position_mm: Decimal,
    /// Position margin
    /// Classic & UTA1.0(inverse) can refer to this field to get the position initial margin plus position closing fee
    pub position_balance: Decimal,
    /// Take profit price
    #[serde(default, deserialize_with = "option_decimal")]
    pub take_profit: Option<Decimal>,
    /// Stop loss price
    #[serde(default, deserialize_with = "option_decimal")]
    pub stop_loss: Option<Decimal>,
    /// Trailing stop (The distance from market price)
    pub trailing_stop: Decimal,
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
    /// Futures & Perps: it is the all time cumulative realised P&L
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
    /// deprecated, always "Full"
    pub tpsl_mode: TpslMode,
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
        let message: Resp<KLine> = deserialize_str(json).unwrap();
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
            time: 1672025956592,
            ret_ext_info: RetExtInfo {},
        };
        assert_eq!(message, expected);
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
        let message: Resp<Ticker> = deserialize_str(json).unwrap();
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
            time: 1672376496682,
            ret_ext_info: RetExtInfo {},
        };
        assert_eq!(message, expected);
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
        let message: Resp<Trade> = deserialize_str(json).unwrap();
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
            time: 1672053054358,
            ret_ext_info: RetExtInfo {},
        };
        assert_eq!(message, expected);
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
        let message: Resp<CursorPagination<Order>> = deserialize_str(json).unwrap();
        let expected = Resp {
            ret_code: 0,
            ret_msg: String::from("OK"),
            result: CursorPagination {
                category: Category::Linear,
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
                    trigger_by: TriggerBy::UNKNOWN,
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
            time: 1684765770483,
            ret_ext_info: RetExtInfo {},
        };
        assert_eq!(message, expected);
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
                        "positionIM": "0.00010923",
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
        let message: Resp<CursorPagination<Position>> = deserialize_str(json).unwrap();
        let expected = Resp {
            ret_code: 0,
            ret_msg: String::from("OK"),
            result: CursorPagination {
                category: Category::Inverse,
                next_page_cursor: None,
                list: vec![Position {
                    position_idx: PositionIdx::OneWay,
                    risk_id: 1,
                    risk_limit_value: dec!(150),
                    symbol: String::from("BTCUSD"),
                    side: Some(Side::Sell),
                    size: dec!(300),
                    avg_price: dec!(27464.50441675),
                    position_value: Some(dec!(0.01092319)),
                    trade_mode: TradeMode::CrossMargin,
                    auto_add_margin: true,
                    position_status: PositionStatus::Normal,
                    leverage: dec!(10),
                    mark_price: dec!(28224.50),
                    liq_price: None,
                    bust_price: Some(dec!(999999.00)),
                    position_im: dec!(0.00010923),
                    position_mm: dec!(0.0000015),
                    position_balance: dec!(0.00139186),
                    take_profit: Some(dec!(0.00)),
                    stop_loss: Some(dec!(0.00)),
                    trailing_stop: dec!(0.00),
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
                    tpsl_mode: TpslMode::Full,
                }],
            },
            time: 1697684980172,
            ret_ext_info: RetExtInfo {},
        };
        assert_eq!(message, expected);
    }
}
