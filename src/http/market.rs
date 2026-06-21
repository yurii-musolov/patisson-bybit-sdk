use rust_decimal::{Decimal, serde::str_option::deserialize as option_decimal};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::{
    deserialize_number_from_string as number,
    deserialize_option_number_from_string as option_number,
};

use crate::{
    ContractType, CopyTrading, CurAuctionPhase, Side, Status, Timestamp,
    enums::{Category, Interval, IntervalTime},
    serde::{empty_string_as_none, int_to_bool, string_to_bool},
};

#[derive(Debug, Serialize, Clone)]
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

#[derive(Debug, Serialize, Clone)]
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
    #[serde(default, deserialize_with = "empty_string_as_none")]
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

#[derive(Debug, Serialize, Clone)]
pub struct GetOrderbookParams {
    pub category: Category,
    pub symbol: String,
    /// Limit size for each bid and ask
    /// - spot: [1, 1000]. Default: 1.
    /// - linear&inverse: [1, 1000]. Default: 25.
    /// - option: [1, 25]. Default: 1.
    pub limit: Option<u64>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Orderbook {
    /// Symbol name
    #[serde(rename = "s")]
    pub symbol: String,
    /// Bid, buy side. Sorted by price in descending order
    #[serde(rename = "b")]
    pub bids: Vec<OrderbookLevel>,
    /// Ask, sell side. Sorted by price in ascending order
    #[serde(rename = "a")]
    pub asks: Vec<OrderbookLevel>,
    /// The timestamp (ms) that the system generates the data
    pub ts: Timestamp,
    /// Update ID, is a sequence. Occasionally, you'll receive "u"=1, which is a snapshot
    /// data due to the restart of the service. So please overwrite your local orderbook
    #[serde(rename = "u")]
    pub update_id: i64,
    /// Cross sequence. You can use this field to compare different levels orderbook data,
    /// and for the smaller seq, then it means the data is generated earlier
    pub seq: i64,
    /// Cross timestamp (ms). Spot only
    #[serde(default)]
    pub cts: Option<Timestamp>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct OrderbookLevel {
    /// Price
    pub price: Decimal,
    /// Size
    pub size: Decimal,
}

#[derive(Debug, Serialize, Clone)]
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

#[derive(Debug, Serialize, Clone)]
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

// --- Funding Rate History ---

/// Query params for [`Client::get_funding_rate_history`](crate::http::Client::get_funding_rate_history).
///
/// Covers: linear / inverse
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateHistoryParams {
    pub category: Category,
    pub symbol: String,
    pub start_time: Option<Timestamp>,
    pub end_time: Option<Timestamp>,
    /// Max 200. Default 200.
    pub limit: Option<u64>,
}

/// Response for [`Client::get_funding_rate_history`](crate::http::Client::get_funding_rate_history).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "category")]
pub enum FundingRateHistory {
    #[serde(rename = "linear")]
    Linear { list: Vec<FundingRateEntry> },
    #[serde(rename = "inverse")]
    Inverse { list: Vec<FundingRateEntry> },
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FundingRateEntry {
    pub symbol: String,
    pub funding_rate: Decimal,
    #[serde(deserialize_with = "number")]
    pub funding_rate_timestamp: Timestamp,
}

// --- Open Interest ---

/// Query params for [`Client::get_open_interest`](crate::http::Client::get_open_interest).
///
/// Covers: linear / inverse
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetOpenInterestParams {
    pub category: Category,
    pub symbol: String,
    pub interval_time: IntervalTime,
    pub start_time: Option<Timestamp>,
    pub end_time: Option<Timestamp>,
    /// Max 200. Default 50.
    pub limit: Option<u64>,
    pub cursor: Option<String>,
}

/// Response for [`Client::get_open_interest`](crate::http::Client::get_open_interest).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterest {
    pub symbol: String,
    pub category: String,
    pub list: Vec<OpenInterestEntry>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub next_page_cursor: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterestEntry {
    pub open_interest: Decimal,
    #[serde(deserialize_with = "number")]
    pub timestamp: Timestamp,
}

// --- Historical Volatility ---

/// Query params for [`Client::get_historical_volatility`](crate::http::Client::get_historical_volatility).
///
/// Covers: option only
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetHistoricalVolatilityParams {
    pub category: Category,
    /// Default: BTC.
    pub base_coin: Option<String>,
    /// Accepted values: 7, 14, 21, 30, 60, 90, 180, 270.
    pub period: Option<u16>,
    pub start_time: Option<Timestamp>,
    pub end_time: Option<Timestamp>,
}

/// One data point from
/// [`Client::get_historical_volatility`](crate::http::Client::get_historical_volatility).
///
/// The endpoint returns `result` as a top-level JSON array, so the full
/// response type is `Response<Vec<HistoricalVolatilityEntry>>`.
#[derive(Debug, Deserialize, PartialEq)]
pub struct HistoricalVolatilityEntry {
    pub period: u16,
    pub value: Decimal,
    #[serde(deserialize_with = "number")]
    pub time: Timestamp,
}

// --- Insurance ---

/// Query params for [`Client::get_insurance`](crate::http::Client::get_insurance).
#[derive(Debug, Serialize, Clone, Default)]
pub struct GetInsuranceParams {
    pub coin: Option<String>,
}

/// Response for [`Client::get_insurance`](crate::http::Client::get_insurance).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Insurance {
    #[serde(deserialize_with = "number")]
    pub updated_time: Timestamp,
    pub list: Vec<InsuranceEntry>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InsuranceEntry {
    pub coin: String,
    pub balance: Decimal,
    pub value: Decimal,
}

// --- Risk Limit ---

/// Query params for [`Client::get_risk_limit`](crate::http::Client::get_risk_limit).
///
/// Covers: linear / inverse
#[derive(Debug, Serialize, Clone)]
pub struct GetRiskLimitParams {
    pub category: Category,
    pub symbol: Option<String>,
    pub cursor: Option<String>,
}

/// Response for [`Client::get_risk_limit`](crate::http::Client::get_risk_limit).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RiskLimit {
    pub category: String,
    pub list: Vec<RiskLimitEntry>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RiskLimitEntry {
    pub id: u32,
    pub symbol: String,
    pub risk_limit_value: Decimal,
    pub maintenance_margin: Decimal,
    pub initial_margin: Decimal,
    #[serde(deserialize_with = "int_to_bool")]
    pub is_lowest_risk: bool,
    pub max_leverage: Decimal,
}

// --- Delivery Price ---

/// Query params for [`Client::get_delivery_price`](crate::http::Client::get_delivery_price).
///
/// Covers: linear / inverse / option
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetDeliveryPriceParams {
    pub category: Category,
    pub symbol: Option<String>,
    pub base_coin: Option<String>,
    /// Max 200. Default 50.
    pub limit: Option<u64>,
    pub cursor: Option<String>,
}

/// Response for [`Client::get_delivery_price`](crate::http::Client::get_delivery_price).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryPrice {
    pub category: String,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub next_page_cursor: Option<String>,
    pub list: Vec<DeliveryPriceEntry>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryPriceEntry {
    pub symbol: String,
    pub delivery_price: Decimal,
    #[serde(deserialize_with = "number")]
    pub delivery_time: Timestamp,
}
