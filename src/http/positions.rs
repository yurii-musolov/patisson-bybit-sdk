use rust_decimal::{Decimal, serde::str_option::deserialize as option_decimal};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::{
    deserialize_number_from_string as number,
    deserialize_option_number_from_string as option_number,
};

use crate::{
    AdlRankIndicator, ExecType, OrderType, PositionIdx, PositionMode, PositionStatus, Side,
    Timestamp, TpslMode, TradeMode, TriggerBy,
    enums::{Category, StopOrderType},
    serde::{empty_string_as_none, int_to_bool},
    ws::PositionMsg,
};

use super::account::WalletCoin;

#[derive(Clone, Debug, Serialize)]
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

// ── Set Leverage ─────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetLeverageRequest {
    /// linear, inverse
    pub category: Category,
    pub symbol: String,
    /// Under cross margin mode: 0 < leverage ≤ maxLeverage (must equal sellLeverage).
    /// Under isolated margin mode: 0 < leverage ≤ maxLeverage.
    pub buy_leverage: Decimal,
    pub sell_leverage: Decimal,
}

impl SetLeverageRequest {
    pub fn new(category: Category, symbol: String, leverage: Decimal) -> Self {
        Self {
            category,
            symbol,
            buy_leverage: leverage,
            sell_leverage: leverage,
        }
    }

    pub fn with_asymmetric(mut self, buy_leverage: Decimal, sell_leverage: Decimal) -> Self {
        self.buy_leverage = buy_leverage;
        self.sell_leverage = sell_leverage;
        self
    }
}

// ── Set Trading Stop ─────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetTradingStopRequest {
    /// linear, inverse
    pub category: Category,
    pub symbol: String,
    /// Required. 0: one-way, 1: hedge Buy side, 2: hedge Sell side
    pub position_idx: PositionIdx,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_profit: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<Decimal>,
    /// Trailing stop distance from market price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_stop: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_by: Option<TriggerBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_by: Option<TriggerBy>,
    /// Activation price for trailing stop
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_price: Option<Decimal>,
    /// Partial TP quantity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_size: Option<Decimal>,
    /// Partial SL quantity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_size: Option<Decimal>,
    /// Limit price for TP limit order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_limit_price: Option<Decimal>,
    /// Limit price for SL limit order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_limit_price: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_order_type: Option<OrderType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_order_type: Option<OrderType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpsl_mode: Option<TpslMode>,
}

impl SetTradingStopRequest {
    pub fn new(category: Category, symbol: String, position_idx: PositionIdx) -> Self {
        Self {
            category,
            symbol,
            position_idx,
            take_profit: None,
            stop_loss: None,
            trailing_stop: None,
            tp_trigger_by: None,
            sl_trigger_by: None,
            active_price: None,
            tp_size: None,
            sl_size: None,
            tp_limit_price: None,
            sl_limit_price: None,
            tp_order_type: None,
            sl_order_type: None,
            tpsl_mode: None,
        }
    }

    pub fn with_take_profit(mut self, v: Decimal) -> Self {
        self.take_profit = Some(v);
        self
    }
    pub fn with_stop_loss(mut self, v: Decimal) -> Self {
        self.stop_loss = Some(v);
        self
    }
    pub fn with_trailing_stop(mut self, v: Decimal) -> Self {
        self.trailing_stop = Some(v);
        self
    }
    pub fn with_tp_trigger_by(mut self, v: TriggerBy) -> Self {
        self.tp_trigger_by = Some(v);
        self
    }
    pub fn with_sl_trigger_by(mut self, v: TriggerBy) -> Self {
        self.sl_trigger_by = Some(v);
        self
    }
    pub fn with_active_price(mut self, v: Decimal) -> Self {
        self.active_price = Some(v);
        self
    }
    pub fn with_tp_size(mut self, v: Decimal) -> Self {
        self.tp_size = Some(v);
        self
    }
    pub fn with_sl_size(mut self, v: Decimal) -> Self {
        self.sl_size = Some(v);
        self
    }
    pub fn with_tp_limit_price(mut self, v: Decimal) -> Self {
        self.tp_limit_price = Some(v);
        self
    }
    pub fn with_sl_limit_price(mut self, v: Decimal) -> Self {
        self.sl_limit_price = Some(v);
        self
    }
    pub fn with_tp_order_type(mut self, v: OrderType) -> Self {
        self.tp_order_type = Some(v);
        self
    }
    pub fn with_sl_order_type(mut self, v: OrderType) -> Self {
        self.sl_order_type = Some(v);
        self
    }
    pub fn with_tpsl_mode(mut self, v: TpslMode) -> Self {
        self.tpsl_mode = Some(v);
        self
    }
}

// ── Switch Cross / Isolated Margin ───────────────────────────────────────────

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SwitchCrossIsolatedMarginRequest {
    /// linear, inverse
    pub category: Category,
    pub symbol: String,
    /// 0: cross margin, 1: isolated margin
    pub trade_mode: TradeMode,
    pub buy_leverage: Decimal,
    pub sell_leverage: Decimal,
}

impl SwitchCrossIsolatedMarginRequest {
    pub fn cross(category: Category, symbol: String, leverage: Decimal) -> Self {
        Self {
            category,
            symbol,
            trade_mode: TradeMode::CrossMargin,
            buy_leverage: leverage,
            sell_leverage: leverage,
        }
    }

    pub fn isolated(
        category: Category,
        symbol: String,
        buy_leverage: Decimal,
        sell_leverage: Decimal,
    ) -> Self {
        Self {
            category,
            symbol,
            trade_mode: TradeMode::IsolatedMargin,
            buy_leverage,
            sell_leverage,
        }
    }
}

// ── Switch Position Mode ─────────────────────────────────────────────────────

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SwitchPositionModeRequest {
    /// linear, inverse
    pub category: Category,
    /// Symbol name. Either symbol or coin is required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Coin. Either symbol or coin is required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    /// 0: Merged Single (one-way), 3: Both Sides (hedge)
    pub mode: PositionMode,
}

impl SwitchPositionModeRequest {
    pub fn one_way(category: Category, symbol: String) -> Self {
        Self {
            category,
            symbol: Some(symbol),
            coin: None,
            mode: PositionMode::OneWay,
        }
    }

    pub fn hedge(category: Category, symbol: String) -> Self {
        Self {
            category,
            symbol: Some(symbol),
            coin: None,
            mode: PositionMode::Hedge,
        }
    }

    pub fn with_coin(mut self, v: String) -> Self {
        self.symbol = None;
        self.coin = Some(v);
        self
    }
}

// ── Set Auto Add Margin ──────────────────────────────────────────────────────

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetAutoAddMarginRequest {
    /// linear, inverse
    pub category: Category,
    pub symbol: String,
    /// 0: false, 1: true
    pub auto_add_margin: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_idx: Option<PositionIdx>,
}

impl SetAutoAddMarginRequest {
    pub fn new(category: Category, symbol: String, enabled: bool) -> Self {
        Self {
            category,
            symbol,
            auto_add_margin: if enabled { 1 } else { 0 },
            position_idx: None,
        }
    }

    pub fn with_position_idx(mut self, v: PositionIdx) -> Self {
        self.position_idx = Some(v);
        self
    }
}

// ── Set Risk Limit ───────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetRiskLimitRequest {
    /// linear, inverse
    pub category: Category,
    pub symbol: String,
    pub risk_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_idx: Option<PositionIdx>,
}

impl SetRiskLimitRequest {
    pub fn new(category: Category, symbol: String, risk_id: i64) -> Self {
        Self {
            category,
            symbol,
            risk_id,
            position_idx: None,
        }
    }

    pub fn with_position_idx(mut self, v: PositionIdx) -> Self {
        self.position_idx = Some(v);
        self
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetRiskLimitResponse {
    pub risk_id: i64,
    pub risk_limit_value: String,
    pub category: String,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub message: Option<String>,
}

// ── Closed P&L ───────────────────────────────────────────────────────────────

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetClosedPnlParams {
    /// linear, inverse
    pub category: Category,
    /// Required for inverse; optional for linear
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<Timestamp>,
    /// [1, 100]. Default: 50
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetClosedPnlParams {
    pub fn new(category: Category) -> Self {
        Self {
            category,
            symbol: None,
            start_time: None,
            end_time: None,
            limit: None,
            cursor: None,
        }
    }

    pub fn with_symbol(mut self, v: String) -> Self {
        self.symbol = Some(v);
        self
    }
    pub fn with_start_time(mut self, v: Timestamp) -> Self {
        self.start_time = Some(v);
        self
    }
    pub fn with_end_time(mut self, v: Timestamp) -> Self {
        self.end_time = Some(v);
        self
    }
    pub fn with_limit(mut self, v: i32) -> Self {
        self.limit = Some(v);
        self
    }
    pub fn with_cursor(mut self, v: String) -> Self {
        self.cursor = Some(v);
        self
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClosedPnl {
    pub symbol: String,
    pub order_id: String,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub order_link_id: Option<String>,
    pub side: Side,
    #[serde(deserialize_with = "number")]
    pub qty: Decimal,
    #[serde(deserialize_with = "number")]
    pub order_price: Decimal,
    pub order_type: OrderType,
    pub exec_type: ExecType,
    #[serde(deserialize_with = "number")]
    pub closed_size: Decimal,
    pub cum_entry_value: Decimal,
    pub avg_entry_price: Decimal,
    pub cum_exit_value: Decimal,
    pub avg_exit_price: Decimal,
    pub closed_pnl: Decimal,
    #[serde(deserialize_with = "number")]
    pub fill_count: i64,
    pub leverage: Decimal,
    #[serde(deserialize_with = "number")]
    pub created_time: Timestamp,
    #[serde(deserialize_with = "number")]
    pub updated_time: Timestamp,
}

// ── Execution List ────────────────────────────────────────────────────────────

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetExecutionListParams {
    pub category: Category,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_type: Option<ExecType>,
    /// [1, 100]. Default: 50
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetExecutionListParams {
    pub fn new(category: Category) -> Self {
        Self {
            category,
            symbol: None,
            order_id: None,
            order_link_id: None,
            base_coin: None,
            start_time: None,
            end_time: None,
            exec_type: None,
            limit: None,
            cursor: None,
        }
    }

    pub fn with_symbol(mut self, v: String) -> Self {
        self.symbol = Some(v);
        self
    }
    pub fn with_order_id(mut self, v: String) -> Self {
        self.order_id = Some(v);
        self
    }
    pub fn with_order_link_id(mut self, v: String) -> Self {
        self.order_link_id = Some(v);
        self
    }
    pub fn with_base_coin(mut self, v: String) -> Self {
        self.base_coin = Some(v);
        self
    }
    pub fn with_start_time(mut self, v: Timestamp) -> Self {
        self.start_time = Some(v);
        self
    }
    pub fn with_end_time(mut self, v: Timestamp) -> Self {
        self.end_time = Some(v);
        self
    }
    pub fn with_exec_type(mut self, v: ExecType) -> Self {
        self.exec_type = Some(v);
        self
    }
    pub fn with_limit(mut self, v: i32) -> Self {
        self.limit = Some(v);
        self
    }
    pub fn with_cursor(mut self, v: String) -> Self {
        self.cursor = Some(v);
        self
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionEntry {
    pub symbol: String,
    pub order_id: String,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub order_link_id: Option<String>,
    pub side: Side,
    pub order_price: Decimal,
    pub order_qty: Decimal,
    pub order_type: OrderType,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub stop_order_type: Option<StopOrderType>,
    pub exec_fee: Decimal,
    pub exec_id: String,
    pub exec_price: Decimal,
    pub exec_qty: Decimal,
    pub exec_type: ExecType,
    pub exec_value: Decimal,
    #[serde(deserialize_with = "number")]
    pub exec_time: Timestamp,
    pub fee_rate: Decimal,
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
    pub closed_size: Decimal,
    pub seq: i64,
    pub is_maker: bool,
}
