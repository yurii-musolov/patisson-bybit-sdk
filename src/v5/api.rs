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
    pub funding_rate: Decimal,
    /// Next funding timestamp (ms)
    #[serde(deserialize_with = "number")]
    pub next_funding_time: Timestamp,
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
    pub bid1_price: Decimal,
    /// Best bid size
    pub bid1_size: Decimal,
    /// Best ask price
    pub ask1_price: Decimal,
    /// Best ask size
    pub ask1_size: Decimal,
    /// Estimated pre-market contract open price. The value is meaningless when entering continuous trading phase.
    pub pre_open_price: Option<String>,
    /// Estimated pre-market contract open qty. The value is meaningless when entering continuous trading phase.
    pub pre_qty: Option<String>,
    /// Enum: NotStarted, Finished, CallAuction, CallAuctionNoCancel, CrossMatching, ContinuousTrading.
    pub cur_pre_listing_phase: Option<String>,
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
    pub usd_index_price: Decimal,
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
        let message: Response<Ticker> = serde_json::from_str(json).unwrap();
        let expected = Response {
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
                    funding_rate: dec!(-0.001034),
                    next_funding_time: 1672387200000,
                    predicted_delivery_price: String::new(),
                    basis_rate: String::new(),
                    basis: String::new(),
                    delivery_fee_rate: String::new(),
                    delivery_time: String::from("0"),
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
}
