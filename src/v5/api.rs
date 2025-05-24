use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_aux::prelude::{
    deserialize_number_from_string as number,
    deserialize_option_number_from_string as option_number,
};

use super::{
    ContractType, CopyTrading, CurAuctionPhase, Innovation, Side, Status,
    enums::{Category, Interval},
};

type Timestamp = u64;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Response<T> {
    #[serde(rename = "retCode")]
    pub ret_code: i64,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: T,
    pub time: Timestamp,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: RetExtInfo,
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

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinearInverseTicker {
    /// Symbol name
    pub symbol: String,
    /// Last price
    #[serde(deserialize_with = "number")]
    pub last_price: f64,
    /// Mark price
    #[serde(deserialize_with = "number")]
    pub mark_price: f64,
    /// Index price
    #[serde(deserialize_with = "number")]
    pub index_price: f64,
    /// Market price 24 hours ago
    #[serde(deserialize_with = "number")]
    pub prev_price24h: f64,
    /// Percentage change of market price in the last 24 hours
    #[serde(deserialize_with = "number")]
    pub price24h_pcnt: f64,
    /// The highest price in the last 24 hours
    #[serde(deserialize_with = "number")]
    pub high_price24h: f64,
    /// The lowest price in the last 24 hours
    #[serde(deserialize_with = "number")]
    pub low_price24h: f64,
    /// Market price an hour ago
    #[serde(deserialize_with = "number")]
    pub prev_price1h: f64,
    /// Open interest size
    #[serde(deserialize_with = "number")]
    pub open_interest: f64,
    /// Open interest value
    #[serde(deserialize_with = "number")]
    pub open_interest_value: f64,
    /// Turnover for 24h
    #[serde(deserialize_with = "number")]
    pub turnover24h: f64,
    /// Volume for 24h
    #[serde(deserialize_with = "number")]
    pub volume24h: f64,
    /// Funding rate
    #[serde(deserialize_with = "number")]
    pub funding_rate: f64,
    /// Next funding timestamp (ms)
    #[serde(deserialize_with = "number")]
    pub next_funding_time: u64,
    /// Predicated delivery price. It has value when 30 min before delivery
    pub predicted_delivery_price: String,
    /// Basis rate. Unique field for inverse futures & USDC futures
    pub basis_rate: String,
    /// Basis. Unique field for inverse futures & USDC futures
    pub basis: String,
    /// Delivery fee rate. Unique field for inverse futures & USDC futures
    pub delivery_fee_rate: String,
    /// Delivery date time (UTC+0). Unique field for inverse futures & USDC futures
    pub delivery_time: String,
    /// Best bid price
    #[serde(deserialize_with = "number")]
    pub bid1_price: f64,
    /// Best bid size
    #[serde(deserialize_with = "number")]
    pub bid1_size: f64,
    /// Best ask price
    #[serde(deserialize_with = "number")]
    pub ask1_price: f64,
    /// Best ask size
    #[serde(deserialize_with = "number")]
    pub ask1_size: f64,
    /// Estimated pre-market contract open price. The value is meaningless when entering continuous trading phase.
    pub pre_open_price: String,
    /// Estimated pre-market contract open qty. The value is meaningless when entering continuous trading phase.
    pub pre_qty: String,
    /// Enum: NotStarted, Finished, CallAuction, CallAuctionNoCancel, CrossMatching, ContinuousTrading.
    pub cur_pre_listing_phase: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionTicker {
    /// Symbol name
    pub symbol: String,
    /// Best bid price
    #[serde(deserialize_with = "number")]
    pub bid1_price: f64,
    /// Best bid size
    #[serde(deserialize_with = "number")]
    pub bid1_size: f64,
    /// Best bid iv
    #[serde(deserialize_with = "number")]
    pub bid1_iv: f64,
    /// Best ask price
    #[serde(deserialize_with = "number")]
    pub ask1_price: f64,
    /// Best ask size
    #[serde(deserialize_with = "number")]
    pub ask1_size: f64,
    /// Best ask iv
    #[serde(deserialize_with = "number")]
    pub ask1_iv: f64,
    /// Last price
    #[serde(deserialize_with = "number")]
    pub last_price: f64,
    /// The highest price in the last 24 hours
    #[serde(deserialize_with = "number")]
    pub high_price24h: f64,
    /// The lowest price in the last 24 hours
    #[serde(deserialize_with = "number")]
    pub low_price24h: f64,
    /// Mark price
    #[serde(deserialize_with = "number")]
    pub mark_price: f64,
    /// Index price
    #[serde(deserialize_with = "number")]
    pub index_price: f64,
    /// Mark price iv
    #[serde(deserialize_with = "number")]
    pub mark_iv: f64,
    /// Underlying price
    #[serde(deserialize_with = "number")]
    pub underlying_price: f64,
    /// Open interest size
    #[serde(deserialize_with = "number")]
    pub open_interest: f64,
    /// Turnover for 24h
    #[serde(deserialize_with = "number")]
    pub turnover24h: f64,
    /// Volume for 24h
    #[serde(deserialize_with = "number")]
    pub volume24h: f64,
    /// Total volume
    #[serde(deserialize_with = "number")]
    pub total_volume: f64,
    /// Total turnover
    #[serde(deserialize_with = "number")]
    pub total_turnover: f64,
    /// Delta
    #[serde(deserialize_with = "number")]
    pub delta: f64,
    /// Gamma
    #[serde(deserialize_with = "number")]
    pub gamma: f64,
    /// Vega
    #[serde(deserialize_with = "number")]
    pub vega: f64,
    /// Theta
    #[serde(deserialize_with = "number")]
    pub theta: f64,
    /// Predicated delivery price. It has value when 30 min before delivery
    #[serde(deserialize_with = "number")]
    pub predicted_delivery_price: f64,
    /// The change in the last 24 hous
    #[serde(deserialize_with = "number")]
    pub change24h: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotTicker {
    /// Symbol name
    pub symbol: String,
    /// Best bid price
    #[serde(deserialize_with = "number")]
    pub bid1_price: f64,
    /// Best bid size
    #[serde(deserialize_with = "number")]
    pub bid1_size: f64,
    /// Best ask price
    #[serde(deserialize_with = "number")]
    pub ask1_price: f64,
    /// Best ask size
    #[serde(deserialize_with = "number")]
    pub ask1_size: f64,
    /// Last price
    #[serde(deserialize_with = "number")]
    pub last_price: f64,
    /// Market price 24 hours ago
    #[serde(deserialize_with = "number")]
    pub prev_price24h: f64,
    /// Percentage change of market price in the last 24 hours
    #[serde(deserialize_with = "number")]
    pub price24h_pcnt: f64,
    /// The highest price in the last 24 hours
    #[serde(deserialize_with = "number")]
    pub high_price24h: f64,
    /// The lowest price in the last 24 hours
    #[serde(deserialize_with = "number")]
    pub low_price24h: f64,
    /// Turnover for 24h
    #[serde(deserialize_with = "number")]
    pub turnover24h: f64,
    /// Volume for 24h
    #[serde(deserialize_with = "number")]
    pub volume24h: f64,
    /// USD index price
    /// - used to calculate USD value of the assets in Unified account
    /// - non-collateral margin coin returns ""
    /// - Only those trading pairs like "XXX/USDT" or "XXX/USDC" have the value
    #[serde(deserialize_with = "number")]
    pub usd_index_price: f64,
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

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InverseLinearSpotTrade {
    pub exec_id: String,
    pub symbol: String,
    #[serde(deserialize_with = "number")]
    pub price: f64,
    #[serde(deserialize_with = "number")]
    pub size: f64,
    pub side: Side,
    #[serde(deserialize_with = "number")]
    pub time: u64,
    pub is_block_trade: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionTrade {
    pub exec_id: String,
    pub symbol: String,
    #[serde(deserialize_with = "number")]
    pub price: f64,
    #[serde(deserialize_with = "number")]
    pub size: f64,
    pub side: Side,
    #[serde(deserialize_with = "number")]
    pub time: u64,
    pub is_block_trade: bool,
    #[serde(rename = "mP", deserialize_with = "number")]
    pub mark_price: f64,
    #[serde(rename = "iP", deserialize_with = "number")]
    pub index_price: f64,
    #[serde(rename = "mIv", deserialize_with = "number")]
    pub mark_iv: f64,
    #[serde(rename = "iv", deserialize_with = "number")]
    pub iv: f64,
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
    pub innovation: Innovation,
    /// Instrument status
    pub status: Status,
    /// Margin trade symbol or not
    /// This is to identify if the symbol support margin trading under different account modes
    /// You may find some symbols not supporting margin buy or margin sell, so you need to go to Collateral Info (UTA) to check if that coin is borrowable
    pub margin_trading: String,
    /// Whether or not it has an special treatment label. 0: false, 1: true
    pub st_tag: String,
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
    pub start_time: Option<i64>,
    #[serde(deserialize_with = "option_number")]
    pub end_time: Option<i64>,
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

#[cfg(test)]
mod tests {
    use rust_decimal::dec;

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
        let message: Response<KLine> = serde_json::from_str(json).unwrap();
        let expected = Response {
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
}
