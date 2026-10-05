use rust_decimal::{Decimal, serde::str_option::deserialize as option_decimal};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::{
    deserialize_number_from_string as number,
    deserialize_option_number_from_string as option_number,
};

use crate::{
    ContractType, CopyTrading, CurAuctionPhase, OptionType, Side, Status, Timestamp,
    enums::{Category, Interval, IntervalTime},
    serde::{empty_string_as_none, int_to_bool, string_to_bool},
};

/// Query parameters of `GET /v5/market/kline` ([`Client::get_index_price_kline`](crate::http::Client::get_index_price_kline), [`Client::get_kline`](crate::http::Client::get_kline), [`Client::get_mark_price_kline`](crate::http::Client::get_mark_price_kline), [`Client::get_premium_index_price_kline`](crate::http::Client::get_premium_index_price_kline)).
#[derive(Debug, Serialize, Clone)]
pub struct GetKLinesParams {
    /// Product type. linear, inverse When category is not passed, use linear by default.
    pub category: Category,
    /// Symbol name, like BTCUSDT, uppercase only.
    pub symbol: String,
    /// Kline interval. 1, 3, 5, 15, 30, 60, 120, 240, 360, 720, D, W, M.
    pub interval: Interval,
    /// The start timestamp (ms).
    pub start: Option<Timestamp>,
    /// The end timestamp (ms).
    pub end: Option<Timestamp>,
    /// Limit for data size per page. \[ 1, 1000 \]. Default: 200.
    pub limit: Option<u64>,
}

impl GetKLinesParams {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new(category: Category, symbol: impl Into<String>, interval: Interval) -> Self {
        Self {
            category,
            symbol: symbol.into(),
            interval,
            start: None,
            end: None,
            limit: None,
        }
    }

    /// Set `start`: the start timestamp (ms).
    pub fn with_start(mut self, v: Timestamp) -> Self {
        self.start = Some(v);
        self
    }

    /// Set `end`: the end timestamp (ms).
    pub fn with_end(mut self, v: Timestamp) -> Self {
        self.end = Some(v);
        self
    }

    /// Set `limit`: limit for data size per page. \[ 1, 1000 \]. Default: 200.
    pub fn with_limit(mut self, v: u64) -> Self {
        self.limit = Some(v);
        self
    }
}

/// Result of `GET /v5/market/kline` ([`Client::get_kline`](crate::http::Client::get_kline)).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "category")]
pub enum KLine {
    /// Inverse contracts.
    #[serde(rename = "inverse")]
    Inverse {
        /// Symbol name.
        symbol: String,
        /// An string array of individual candle Sort in reverse by startTime.
        list: Vec<KLineRow>,
    },
    /// Linear (USDT/USDC) contracts.
    #[serde(rename = "linear")]
    Linear {
        /// Symbol name.
        symbol: String,
        /// An string array of individual candle Sort in reverse by startTime.
        list: Vec<KLineRow>,
    },
    /// Options.
    #[serde(rename = "option")]
    Option {
        /// Symbol name.
        symbol: String,
        /// An string array of individual candle Sort in reverse by startTime.
        list: Vec<KLineRow>,
    },
    /// Spot.
    #[serde(rename = "spot")]
    Spot {
        /// Symbol name.
        symbol: String,
        /// An string array of individual candle Sort in reverse by startTime.
        list: Vec<KLineRow>,
    },
}

/// Result of the mark, index and premium index price kline endpoints: rows
/// carry prices only (no volume or turnover).
#[derive(Debug, Deserialize, PartialEq)]
pub struct PriceKLine {
    /// Product type.
    pub category: Category,
    /// Symbol name.
    pub symbol: String,
    /// Klines, newest first.
    pub list: Vec<PriceKLineRow>,
}

/// A price-only kline row: `[startTime, open, high, low, close]`.
#[derive(Debug, Deserialize, PartialEq)]
pub struct PriceKLineRow {
    /// Start time of the candle, milliseconds.
    #[serde(deserialize_with = "number")]
    pub start_time: Timestamp,
    /// Open price.
    pub open_price: Decimal,
    /// Highest price.
    pub high_price: Decimal,
    /// Lowest price.
    pub low_price: Decimal,
    /// Close price; the latest price while the candle is open.
    pub close_price: Decimal,
}

/// Part of the response of `/v5/market/kline`.
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

/// Query parameters of `GET /v5/market/tickers` ([`Client::get_tickers`](crate::http::Client::get_tickers)).
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetTickersParams {
    /// Product type. spot, linear, inverse, option.
    pub category: Category,
    /// Symbol name, like BTCUSDT, uppercase only.
    pub symbol: Option<String>,
    /// Base coin, uppercase only. Apply to option only.
    pub base_coin: Option<String>,
    /// Expiry date. e.g., 25DEC22. Apply to option only.
    pub exp_date: Option<String>,
}

impl GetTickersParams {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new(category: Category) -> Self {
        Self {
            category,
            symbol: None,
            base_coin: None,
            exp_date: None,
        }
    }

    /// Set `symbol`: symbol name, like BTCUSDT, uppercase only.
    pub fn with_symbol(mut self, v: impl Into<String>) -> Self {
        self.symbol = Some(v.into());
        self
    }

    /// Set `base_coin`: base coin, uppercase only. Apply to option only.
    pub fn with_base_coin(mut self, v: impl Into<String>) -> Self {
        self.base_coin = Some(v.into());
        self
    }

    /// Set `exp_date`: expiry date. e.g., 25DEC22. Apply to option only.
    pub fn with_exp_date(mut self, v: impl Into<String>) -> Self {
        self.exp_date = Some(v.into());
        self
    }
}

/// Result of `GET /v5/market/tickers` ([`Client::get_tickers`](crate::http::Client::get_tickers)).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "category")]
pub enum Ticker {
    /// Inverse contracts.
    #[serde(rename = "inverse")]
    Inverse {
        /// Items.
        list: Vec<LinearInverseTicker>,
    },
    /// Linear (USDT/USDC) contracts.
    #[serde(rename = "linear")]
    Linear {
        /// Items.
        list: Vec<LinearInverseTicker>,
    },
    /// Options.
    #[serde(rename = "option")]
    Option {
        /// Items.
        list: Vec<OptionTicker>,
    },
    /// Spot.
    #[serde(rename = "spot")]
    Spot {
        /// Items.
        list: Vec<SpotTicker>,
    },
}

/// Part of the response of `/v5/market/tickers`.
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

/// Part of the response of `/v5/market/tickers`.
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

/// Part of the response of `/v5/market/tickers`.
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

/// Query parameters of `GET /v5/market/orderbook` ([`Client::get_orderbook`](crate::http::Client::get_orderbook)).
#[derive(Debug, Serialize, Clone)]
pub struct GetOrderbookParams {
    /// Product type. spot, linear, inverse, option.
    pub category: Category,
    /// Symbol name, like BTCUSDT, uppercase only.
    pub symbol: String,
    /// Limit size for each bid and ask
    /// - spot: [1, 1000]. Default: 1.
    /// - linear&inverse: [1, 1000]. Default: 25.
    /// - option: [1, 25]. Default: 1.
    pub limit: Option<u64>,
}

impl GetOrderbookParams {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new(category: Category, symbol: impl Into<String>) -> Self {
        Self {
            category,
            symbol: symbol.into(),
            limit: None,
        }
    }

    /// Set `limit`: limit size for each bid and ask - spot: [1, 1000]. Default: 1. - linear&inverse: [1, 1000]. Default: 25. - option: [1, 25]. Default: 1.
    pub fn with_limit(mut self, v: u64) -> Self {
        self.limit = Some(v);
        self
    }
}

/// Result of `GET /v5/market/orderbook` ([`Client::get_orderbook`](crate::http::Client::get_orderbook)).
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

/// Part of the response of `/v5/market/orderbook`.
#[derive(Debug, Deserialize, PartialEq)]
pub struct OrderbookLevel {
    /// Price
    pub price: Decimal,
    /// Size
    pub size: Decimal,
}

/// Query parameters of `GET /v5/market/recent-trade` ([`Client::get_public_recent_trading_history`](crate::http::Client::get_public_recent_trading_history)).
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetTradesParams {
    /// Product type. spot, linear, inverse, option.
    pub category: Category,
    /// required for spot/linear/inverse
    /// optional for option
    pub symbol: Option<String>,
    /// Apply to option only
    /// If the field is not passed, return BTC data by default
    pub base_coin: Option<String>,
    /// Option type. Apply to option only
    pub option_type: Option<OptionType>,
    /// spot: `[1, 60]`, default: 60
    /// others: `[1, 1000]`, default: 500
    pub limit: Option<u64>,
}

impl GetTradesParams {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new(category: Category) -> Self {
        Self {
            category,
            symbol: None,
            base_coin: None,
            option_type: None,
            limit: None,
        }
    }

    /// Set `symbol`: required for spot/linear/inverse optional for option.
    pub fn with_symbol(mut self, v: impl Into<String>) -> Self {
        self.symbol = Some(v.into());
        self
    }

    /// Set `base_coin`: apply to option only If the field is not passed, return BTC data by default.
    pub fn with_base_coin(mut self, v: impl Into<String>) -> Self {
        self.base_coin = Some(v.into());
        self
    }

    /// Set `option_type`: option type. Apply to option only.
    pub fn with_option_type(mut self, v: OptionType) -> Self {
        self.option_type = Some(v);
        self
    }

    /// Set `limit`: spot: `[1, 60]`, default: 60 others: `[1, 1000]`, default: 500.
    pub fn with_limit(mut self, v: u64) -> Self {
        self.limit = Some(v);
        self
    }
}

/// Result of `GET /v5/market/recent-trade` ([`Client::get_public_recent_trading_history`](crate::http::Client::get_public_recent_trading_history)).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "category")]
pub enum Trade {
    /// Inverse contracts.
    #[serde(rename = "inverse")]
    Inverse {
        /// An string array of individual candle Sort in reverse by startTime.
        list: Vec<InverseLinearSpotTrade>,
    },
    /// Linear (USDT/USDC) contracts.
    #[serde(rename = "linear")]
    Linear {
        /// An string array of individual candle Sort in reverse by startTime.
        list: Vec<InverseLinearSpotTrade>,
    },
    /// Options.
    #[serde(rename = "option")]
    Option {
        /// An string array of individual candle Sort in reverse by startTime.
        list: Vec<OptionTrade>,
    },
    /// Spot.
    #[serde(rename = "spot")]
    Spot {
        /// An string array of individual candle Sort in reverse by startTime.
        list: Vec<InverseLinearSpotTrade>,
    },
}

/// Part of the response of `/v5/account/transaction-log`.
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

/// Part of the response of `/v5/account/transaction-log`.
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
    /// Whether the trade is an RPI trade (not sent for options: `false`).
    #[serde(rename = "isRPITrade", default)]
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

/// Result of `GET /v5/market/time` ([`Client::get_server_time`](crate::http::Client::get_server_time)).
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

/// Query parameters of `GET /v5/market/instruments-info` ([`Client::get_instruments_info`](crate::http::Client::get_instruments_info)).
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetInstrumentsInfoParams {
    /// Product type. spot, linear, inverse, option.
    pub category: Category,
    /// Symbol name, like BTCUSDT, uppercase only.
    pub symbol: Option<String>,
    /// Symbol status filter linear & inverse & spot By default returns Trading & PendingOpen symbols option By default returns PreLaunch, Trading, and Delivering Spot has Trading only linear & inverse: when status=PreLaunch, it returns Pre-Market ...
    pub status: Option<Status>,
    /// Base coin, uppercase only Applies to linear, inverse, option only option: returns BTC by default Pass All to return all option symbols. Only valid when category=option.
    pub base_coin: Option<String>,
    /// Limit for data size per page. \[ 1, 1000 \]. Default: 500.
    pub limit: Option<i64>,
    /// Cursor. Use the nextPageCursor token from the response to retrieve the next page of the result set.
    pub cursor: Option<String>,
}

impl GetInstrumentsInfoParams {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new(category: Category) -> Self {
        Self {
            category,
            symbol: None,
            status: None,
            base_coin: None,
            limit: None,
            cursor: None,
        }
    }

    /// Set `symbol`: symbol name, like BTCUSDT, uppercase only.
    pub fn with_symbol(mut self, v: impl Into<String>) -> Self {
        self.symbol = Some(v.into());
        self
    }

    /// Set `status`: symbol status filter linear & inverse & spot By default returns Trading & PendingOpen symbols option By default returns PreLaunch, Trading, and Delivering Spot has Trading only linear & inverse: when status=PreLaunch, it returns Pre-Market ...
    pub fn with_status(mut self, v: Status) -> Self {
        self.status = Some(v);
        self
    }

    /// Set `base_coin`: base coin, uppercase only Applies to linear, inverse, option only option: returns BTC by default Pass All to return all option symbols. Only valid when category=option.
    pub fn with_base_coin(mut self, v: impl Into<String>) -> Self {
        self.base_coin = Some(v.into());
        self
    }

    /// Set `limit`: limit for data size per page. \[ 1, 1000 \]. Default: 500.
    pub fn with_limit(mut self, v: i64) -> Self {
        self.limit = Some(v);
        self
    }

    /// Set `cursor`: cursor. Use the nextPageCursor token from the response to retrieve the next page of the result set.
    pub fn with_cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
}

/// Result of `GET /v5/market/instruments-info` ([`Client::get_instruments_info`](crate::http::Client::get_instruments_info)).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "category")]
pub enum InstrumentsInfo {
    /// Inverse contracts.
    #[serde(rename = "inverse", rename_all = "camelCase")]
    Inverse {
        /// Cursor of the next page; empty on the last page.
        next_page_cursor: String,
        /// Items.
        list: Vec<InverseLinearInstrumentsInfo>,
    },
    /// Linear (USDT/USDC) contracts.
    #[serde(rename = "linear", rename_all = "camelCase")]
    Linear {
        /// Cursor of the next page; empty on the last page.
        next_page_cursor: String,
        /// Items.
        list: Vec<InverseLinearInstrumentsInfo>,
    },
    /// Options.
    #[serde(rename = "option", rename_all = "camelCase")]
    Option {
        /// Cursor of the next page; empty on the last page.
        next_page_cursor: String,
        /// Items.
        list: Vec<OptionInstrumentsInfo>,
    },
    /// Spot.
    #[serde(rename = "spot", rename_all = "camelCase")]
    Spot {
        /// Cursor of the next page; empty on the last page.
        #[serde(default, deserialize_with = "empty_string_as_none")]
        next_page_cursor: Option<String>,
        /// Items.
        list: Vec<SpotInstrumentsInfo>,
    },
}

/// Part of the response of `/v5/market/instruments-info`.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InverseLinearInstrumentsInfo {
    /// Symbol name.
    pub symbol: String,
    /// Contract type.
    pub contract_type: ContractType,
    /// Instrument status.
    pub status: Status,
    /// Base coin.
    pub base_coin: String,
    /// Quote coin.
    pub quote_coin: String,
    /// Launch timestamp (ms).
    #[serde(deserialize_with = "number")]
    pub launch_time: Timestamp,
    /// Delivery timestamp (ms) Expired futures delivery time Perpetual delisting time.
    #[serde(deserialize_with = "number")]
    pub delivery_time: Timestamp,
    /// Delivery fee rate.
    #[serde(deserialize_with = "option_decimal")]
    pub delivery_fee_rate: Option<Decimal>,
    /// Price scale.
    #[serde(deserialize_with = "number")]
    pub price_scale: i64,
    /// Leverage attributes.
    pub leverage_filter: LeverageFilter,
    /// Price attributes.
    pub price_filter: PriceFilter,
    /// Size attributes.
    pub lot_size_filter: LotSizeFilter,
    /// Whether to support unified margin trade.
    pub unified_margin_trade: bool,
    /// Funding interval (minute).
    pub funding_interval: i64,
    /// Settle coin.
    pub settle_coin: String,
    /// Copy trade symbol or not.
    pub copy_trading: CopyTrading,
    /// Upper limit of funding date.
    pub upper_funding_rate: Decimal,
    /// Lower limit of funding date.
    pub lower_funding_rate: Decimal,
    /// Risk parameters for limit order price. Note that the formula changed in May 2026.
    pub risk_parameters: RiskParameters,
    /// Whether the contract is a pre-market contract When the pre-market contract is converted to official contract, it will be false.
    pub is_pre_listing: bool,
    /// If isPreListing=false, preListingInfo=null If isPreListing=true, preListingInfo is an object.
    pub pre_listing_info: Option<PreListingInfo>,
}

/// An option of `GET /v5/market/instruments-info`.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OptionInstrumentsInfo {
    /// Symbol name, e.g. `ETH-25JUN27-7000-P-USDT`.
    pub symbol: String,
    /// Display name in the UI.
    #[serde(default)]
    pub display_name: Option<String>,
    /// Trading status.
    pub status: Status,
    /// Base coin.
    pub base_coin: String,
    /// Quote coin.
    pub quote_coin: String,
    /// Settle coin.
    pub settle_coin: String,
    /// Call or put.
    pub options_type: OptionType,
    /// Launch time, milliseconds.
    #[serde(deserialize_with = "number")]
    pub launch_time: Timestamp,
    /// Delivery time, milliseconds.
    #[serde(deserialize_with = "number")]
    pub delivery_time: Timestamp,
    /// Delivery fee rate.
    #[serde(default, deserialize_with = "option_decimal")]
    pub delivery_fee_rate: Option<Decimal>,
    /// Price limits and tick size.
    pub price_filter: PriceFilter,
    /// Order quantity limits.
    pub lot_size_filter: OptionLotSizeFilter,
}

/// Order quantity limits of an option.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OptionLotSizeFilter {
    /// Maximum order quantity.
    pub max_order_qty: Decimal,
    /// Minimum order quantity.
    pub min_order_qty: Decimal,
    /// Quantity step.
    pub qty_step: Decimal,
}

/// Part of the response of `/v5/market/instruments-info`.
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

/// Part of the response of `/v5/market/instruments-info`.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LeverageFilter {
    /// Minimum leverage.
    pub min_leverage: Decimal,
    /// Maximum leverage.
    pub max_leverage: Decimal,
    /// The step to increase/reduce leverage.
    pub leverage_step: Decimal,
}

/// Part of the response of `/v5/market/instruments-info`.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PriceFilter {
    /// Minimum order price.
    pub min_price: Decimal,
    /// Maximum order price.
    pub max_price: Decimal,
    /// The step to increase/reduce order price.
    pub tick_size: Decimal,
}

/// Part of the response of `/v5/market/instruments-info`.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SpotPriceFilter {
    /// The step to increase/reduce order price
    pub tick_size: Decimal,
}

/// Part of the response of `/v5/market/instruments-info`.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LotSizeFilter {
    /// Minimum notional value.
    pub min_notional_value: Decimal,
    /// Maximum quantity for Limit and PostOnly order.
    pub max_order_qty: Decimal,
    /// Maximum quantity for Market order.
    pub max_mkt_order_qty: Decimal,
    /// Minimum order quantity.
    pub min_order_qty: Decimal,
    /// The step to increase/reduce order quantity.
    pub qty_step: Decimal,
    /// Deprecated, please use maxOrderQty.
    pub post_only_max_order_qty: Decimal,
}

/// Part of the response of `/v5/market/instruments-info`.
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

/// Part of the response of `/v5/market/instruments-info`.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RiskParameters {
    /// Ratio X.
    pub price_limit_ratio_x: Decimal,
    /// Ratio Y.
    pub price_limit_ratio_y: Decimal,
}

/// Part of the response of `/v5/market/instruments-info`.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PreListingInfo {
    /// The current auction phase.
    pub cur_auction_phase: CurAuctionPhase,
    /// Each phase time info.
    pub phases: Vec<Phase>,
    /// Action fee info.
    pub auction_fee_info: AuctionFeeInfo,
}

/// Part of the response of `/v5/market/instruments-info`.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Phase {
    /// Pre-market trading phase.
    pub phase: CurAuctionPhase,
    /// The start time of the phase, timestamp(ms).
    #[serde(deserialize_with = "option_number")]
    pub start_time: Option<Timestamp>,
    /// The end time of the phase, timestamp(ms).
    #[serde(deserialize_with = "option_number")]
    pub end_time: Option<Timestamp>,
}

/// Part of the response of `/v5/market/instruments-info`.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AuctionFeeInfo {
    /// The trading fee rate during auction phase There is no trading fee until entering continues trading phase.
    pub auction_fee_rate: Decimal,
    /// The taker fee rate during continues trading phase.
    pub taker_fee_rate: Decimal,
    /// The maker fee rate during continues trading phase.
    pub maker_fee_rate: Decimal,
}

// --- Funding Rate History ---

/// Query params for [`Client::get_funding_rate_history`](crate::http::Client::get_funding_rate_history).
///
/// Covers: linear / inverse
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateHistoryParams {
    /// Product type. linear, inverse.
    pub category: Category,
    /// Symbol name, like BTCUSDT, uppercase only.
    pub symbol: String,
    /// The start timestamp (ms).
    pub start_time: Option<Timestamp>,
    /// The end timestamp (ms).
    pub end_time: Option<Timestamp>,
    /// Max 200. Default 200.
    pub limit: Option<u64>,
}

impl GetFundingRateHistoryParams {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new(category: Category, symbol: impl Into<String>) -> Self {
        Self {
            category,
            symbol: symbol.into(),
            start_time: None,
            end_time: None,
            limit: None,
        }
    }

    /// Set `start_time`: the start timestamp (ms).
    pub fn with_start_time(mut self, v: Timestamp) -> Self {
        self.start_time = Some(v);
        self
    }

    /// Set `end_time`: the end timestamp (ms).
    pub fn with_end_time(mut self, v: Timestamp) -> Self {
        self.end_time = Some(v);
        self
    }

    /// Set `limit`: max 200. Default 200.
    pub fn with_limit(mut self, v: u64) -> Self {
        self.limit = Some(v);
        self
    }
}

/// Response for [`Client::get_funding_rate_history`](crate::http::Client::get_funding_rate_history).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "category")]
pub enum FundingRateHistory {
    /// Linear (USDT/USDC) contracts.
    #[serde(rename = "linear")]
    Linear {
        /// Items.
        list: Vec<FundingRateEntry>,
    },
    /// Inverse contracts.
    #[serde(rename = "inverse")]
    Inverse {
        /// Items.
        list: Vec<FundingRateEntry>,
    },
}

/// Part of the response of `/v5/market/funding/history`.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FundingRateEntry {
    /// Symbol name.
    pub symbol: String,
    /// Funding rate.
    pub funding_rate: Decimal,
    /// Funding rate timestamp (ms).
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
    /// Product type. linear, inverse.
    pub category: Category,
    /// Symbol name, like BTCUSDT, uppercase only.
    pub symbol: String,
    /// Interval time. 5min, 15min, 30min, 1h, 4h, 1d.
    pub interval_time: IntervalTime,
    /// The start timestamp (ms).
    pub start_time: Option<Timestamp>,
    /// The end timestamp (ms).
    pub end_time: Option<Timestamp>,
    /// Max 200. Default 50.
    pub limit: Option<u64>,
    /// Cursor. Used to paginate.
    pub cursor: Option<String>,
}

impl GetOpenInterestParams {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new(category: Category, symbol: impl Into<String>, interval_time: IntervalTime) -> Self {
        Self {
            category,
            symbol: symbol.into(),
            interval_time,
            start_time: None,
            end_time: None,
            limit: None,
            cursor: None,
        }
    }

    /// Set `start_time`: the start timestamp (ms).
    pub fn with_start_time(mut self, v: Timestamp) -> Self {
        self.start_time = Some(v);
        self
    }

    /// Set `end_time`: the end timestamp (ms).
    pub fn with_end_time(mut self, v: Timestamp) -> Self {
        self.end_time = Some(v);
        self
    }

    /// Set `limit`: max 200. Default 50.
    pub fn with_limit(mut self, v: u64) -> Self {
        self.limit = Some(v);
        self
    }

    /// Set `cursor`: cursor. Used to paginate.
    pub fn with_cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
}

/// Response for [`Client::get_open_interest`](crate::http::Client::get_open_interest).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterest {
    /// Symbol name.
    pub symbol: String,
    /// Product type.
    pub category: String,
    /// Items.
    pub list: Vec<OpenInterestEntry>,
    /// Used to paginate.
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub next_page_cursor: Option<String>,
}

/// Part of the response of `/v5/market/open-interest`.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterestEntry {
    /// Open interest. The value is the sum of both sides. The unit of value, e.g., BTCUSD(inverse) is USD, BTCUSDT(linear) is BTC.
    pub open_interest: Decimal,
    /// The timestamp (ms).
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
    /// Product type. option.
    pub category: Category,
    /// Default: BTC.
    pub base_coin: Option<String>,
    /// Accepted values: 7, 14, 21, 30, 60, 90, 180, 270.
    pub period: Option<u16>,
    /// The start timestamp (ms).
    pub start_time: Option<Timestamp>,
    /// The end timestamp (ms).
    pub end_time: Option<Timestamp>,
}

impl GetHistoricalVolatilityParams {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new(category: Category) -> Self {
        Self {
            category,
            base_coin: None,
            period: None,
            start_time: None,
            end_time: None,
        }
    }

    /// Set `base_coin`: default: BTC.
    pub fn with_base_coin(mut self, v: impl Into<String>) -> Self {
        self.base_coin = Some(v.into());
        self
    }

    /// Set `period`: accepted values: 7, 14, 21, 30, 60, 90, 180, 270.
    pub fn with_period(mut self, v: u16) -> Self {
        self.period = Some(v);
        self
    }

    /// Set `start_time`: the start timestamp (ms).
    pub fn with_start_time(mut self, v: Timestamp) -> Self {
        self.start_time = Some(v);
        self
    }

    /// Set `end_time`: the end timestamp (ms).
    pub fn with_end_time(mut self, v: Timestamp) -> Self {
        self.end_time = Some(v);
        self
    }
}

/// One data point from
/// [`Client::get_historical_volatility`](crate::http::Client::get_historical_volatility).
///
/// The endpoint returns `result` as a top-level JSON array, so the full
/// response type is `Response<Vec<HistoricalVolatilityEntry>>`.
#[derive(Debug, Deserialize, PartialEq)]
pub struct HistoricalVolatilityEntry {
    /// Period.
    pub period: u16,
    /// Volatility.
    pub value: Decimal,
    /// Timestamp (ms).
    #[serde(deserialize_with = "number")]
    pub time: Timestamp,
}

// --- Insurance ---

/// Query params for [`Client::get_insurance`](crate::http::Client::get_insurance).
#[derive(Debug, Serialize, Clone, Default)]
pub struct GetInsuranceParams {
    /// Coin, uppercase only. Default: return all insurance coins.
    pub coin: Option<String>,
}

impl GetInsuranceParams {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new() -> Self {
        Self { coin: None }
    }

    /// Set `coin`: coin, uppercase only. Default: return all insurance coins.
    pub fn with_coin(mut self, v: impl Into<String>) -> Self {
        self.coin = Some(v.into());
        self
    }
}

/// Response for [`Client::get_insurance`](crate::http::Client::get_insurance).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Insurance {
    /// Data updated time (ms).
    #[serde(deserialize_with = "number")]
    pub updated_time: Timestamp,
    /// Items.
    pub list: Vec<InsuranceEntry>,
}

/// Part of the response of `/v5/market/insurance`.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InsuranceEntry {
    /// Coin.
    pub coin: String,
    /// Balance.
    pub balance: Decimal,
    /// USD value.
    pub value: Decimal,
}

// --- Risk Limit ---

/// Query params for [`Client::get_risk_limit`](crate::http::Client::get_risk_limit).
///
/// Covers: linear / inverse
#[derive(Debug, Serialize, Clone)]
pub struct GetRiskLimitParams {
    /// Product type. linear, inverse.
    pub category: Category,
    /// Symbol name, like BTCUSDT, uppercase only.
    pub symbol: Option<String>,
    /// Cursor. Use the nextPageCursor token from the response to retrieve the next page of the data set.
    pub cursor: Option<String>,
}

impl GetRiskLimitParams {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new(category: Category) -> Self {
        Self {
            category,
            symbol: None,
            cursor: None,
        }
    }

    /// Set `symbol`: symbol name, like BTCUSDT, uppercase only.
    pub fn with_symbol(mut self, v: impl Into<String>) -> Self {
        self.symbol = Some(v.into());
        self
    }

    /// Set `cursor`: cursor. Use the nextPageCursor token from the response to retrieve the next page of the data set.
    pub fn with_cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
}

/// Response for [`Client::get_risk_limit`](crate::http::Client::get_risk_limit).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RiskLimit {
    /// Product type.
    pub category: String,
    /// Items.
    pub list: Vec<RiskLimitEntry>,
}

/// Part of the response of `/v5/market/risk-limit`.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RiskLimitEntry {
    /// Risk ID.
    pub id: u32,
    /// Symbol name.
    pub symbol: String,
    /// Position limit.
    pub risk_limit_value: Decimal,
    /// Maintain margin rate.
    pub maintenance_margin: Decimal,
    /// Initial margin rate.
    pub initial_margin: Decimal,
    /// 1: true, 0: false.
    #[serde(deserialize_with = "int_to_bool")]
    pub is_lowest_risk: bool,
    /// Allowed max leverage.
    pub max_leverage: Decimal,
}

// --- Delivery Price ---

/// Query params for [`Client::get_delivery_price`](crate::http::Client::get_delivery_price).
///
/// Covers: linear / inverse / option
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetDeliveryPriceParams {
    /// Product type. linear, inverse, option.
    pub category: Category,
    /// Symbol name, like BTCUSDT, uppercase only.
    pub symbol: Option<String>,
    /// Base coin, uppercase only. Default: BTC. Valid for option only.
    pub base_coin: Option<String>,
    /// Max 200. Default 50.
    pub limit: Option<u64>,
    /// Cursor. Use the nextPageCursor token from the response to retrieve the next page of the result set.
    pub cursor: Option<String>,
}

impl GetDeliveryPriceParams {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new(category: Category) -> Self {
        Self {
            category,
            symbol: None,
            base_coin: None,
            limit: None,
            cursor: None,
        }
    }

    /// Set `symbol`: symbol name, like BTCUSDT, uppercase only.
    pub fn with_symbol(mut self, v: impl Into<String>) -> Self {
        self.symbol = Some(v.into());
        self
    }

    /// Set `base_coin`: base coin, uppercase only. Default: BTC. Valid for option only.
    pub fn with_base_coin(mut self, v: impl Into<String>) -> Self {
        self.base_coin = Some(v.into());
        self
    }

    /// Set `limit`: max 200. Default 50.
    pub fn with_limit(mut self, v: u64) -> Self {
        self.limit = Some(v);
        self
    }

    /// Set `cursor`: cursor. Use the nextPageCursor token from the response to retrieve the next page of the result set.
    pub fn with_cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
}

/// Response for [`Client::get_delivery_price`](crate::http::Client::get_delivery_price).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryPrice {
    /// Product type.
    pub category: String,
    /// Refer to the cursor request parameter.
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub next_page_cursor: Option<String>,
    /// Items.
    pub list: Vec<DeliveryPriceEntry>,
}

/// Part of the response of `/v5/market/delivery-price`.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryPriceEntry {
    /// Symbol name.
    pub symbol: String,
    /// Delivery price.
    pub delivery_price: Decimal,
    /// Delivery timestamp (ms).
    #[serde(deserialize_with = "number")]
    pub delivery_time: Timestamp,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Category, serde::serialize_query};

    #[test]
    fn trades_params_serialize_option_type_as_call_or_put() {
        let params = GetTradesParams {
            category: Category::Option,
            symbol: None,
            base_coin: Some(String::from("BTC")),
            option_type: Some(OptionType::Put),
            limit: None,
        };

        let query = serialize_query(&params).unwrap();

        assert_eq!(query, "category=option&baseCoin=BTC&optionType=Put");
    }

    #[test]
    fn base_coin_and_exp_date_are_sent_in_camel_case() {
        let tickers = GetTickersParams::new(Category::Option)
            .with_base_coin("ETH")
            .with_exp_date("25DEC26");
        let instruments = GetInstrumentsInfoParams::new(Category::Option).with_base_coin("ETH");

        assert_eq!(
            serialize_query(&tickers).unwrap(),
            "category=option&baseCoin=ETH&expDate=25DEC26"
        );
        assert_eq!(
            serialize_query(&instruments).unwrap(),
            "category=option&baseCoin=ETH"
        );
    }

    #[test]
    fn option_instruments_info_parses() {
        let json = r#"{
            "category": "option",
            "nextPageCursor": "",
            "list": [{
                "symbolId": 379919,
                "symbol": "ETH-25JUN27-7000-P-USDT",
                "status": "Trading",
                "baseCoin": "ETH",
                "quoteCoin": "USDT",
                "settleCoin": "USDT",
                "optionsType": "Put",
                "launchTime": "1787350800000",
                "deliveryTime": "1813910400000",
                "deliveryFeeRate": "0.00015",
                "priceFilter": {"minPrice": "0.1", "maxPrice": "41000", "tickSize": "0.1"},
                "lotSizeFilter": {"maxOrderQty": "5000", "minOrderQty": "0.1", "qtyStep": "0.1"},
                "displayName": "ETHUSDT-25JUN27-7000-P"
            }]
        }"#;

        let info: InstrumentsInfo = serde_json::from_str(json).unwrap();

        let InstrumentsInfo::Option { list, .. } = info else {
            panic!("not an option list");
        };
        assert_eq!(list[0].options_type, OptionType::Put);
        assert_eq!(list[0].lot_size_filter.qty_step, rust_decimal::dec!(0.1));
        assert_eq!(list[0].delivery_time, 1813910400000);
    }
}
