use rust_decimal::{Decimal, serde::str_option::deserialize as option_decimal};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::{
    deserialize_number_from_string as number,
    deserialize_option_number_from_string as option_number,
};

use crate::v5::{
    AdlRankIndicator, PositionIdx, PositionMsg, PositionStatus, Side,
    enums::Category,
    serde::{empty_string_as_none, int_to_bool},
};

use super::account::WalletCoin;
use super::common::Timestamp;

#[derive(Clone, Serialize)]
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

impl GetPositionInfoParams {
    pub fn new(category: Category) -> Self {
        Self {
            category,
            symbol: None,
            base_coin: None,
            settle_coin: None,
            limit: None,
            cursor: None,
        }
    }

    pub fn with_symbol(mut self, v: String) -> Self {
        self.symbol = Some(v);
        self
    }
    pub fn with_base_coin(mut self, v: String) -> Self {
        self.base_coin = Some(v);
        self
    }
    pub fn with_settle_coin(mut self, v: String) -> Self {
        self.settle_coin = Some(v);
        self
    }
    pub fn with_limit(mut self, v: u64) -> Self {
        self.limit = Some(v);
        self
    }
    pub fn with_cursor(mut self, v: String) -> Self {
        self.cursor = Some(v);
        self
    }
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
    #[serde(default, deserialize_with = "empty_string_as_none")]
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
