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
    serde::{decimal_or_zero, empty_string_as_none, int_to_bool, timestamp_or_zero},
    ws::PositionMsg,
};

/// Query parameters of `GET /v5/position/list` ([`Client::get_position_info`](crate::http::Client::get_position_info)).
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
    /// Create with the required fields; set the optional ones with the `with_*` methods.
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

    /// Set `symbol`: symbol name, like BTCUSDT, uppercase only If symbol passed, it returns data regardless of having position or not. If symbol=null and settleCoin specified, it returns position size greater than zero.
    pub fn with_symbol(mut self, v: impl Into<String>) -> Self {
        self.symbol = Some(v.into());
        self
    }
    /// Set `base_coin`: base coin, uppercase only. option only. Return all option positions if not passed.
    pub fn with_base_coin(mut self, v: impl Into<String>) -> Self {
        self.base_coin = Some(v.into());
        self
    }
    /// Set `settle_coin`: settle coin linear: either symbol or settleCoin is required. symbol has a higher priority.
    pub fn with_settle_coin(mut self, v: impl Into<String>) -> Self {
        self.settle_coin = Some(v.into());
        self
    }
    /// Set `limit`: limit for data size per page. [1, 200]. Default: 20.
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

/// A position from `GET /v5/position/list`.
///
/// Fields checked against the Bybit docs (2026-10); the deprecated
/// `tpslMode`, `bustPrice`, `positionBalance` and `tradeMode` are omitted.
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
    #[serde(deserialize_with = "decimal_or_zero")]
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
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub position_status: Option<PositionStatus>,
    /// Position leverage
    /// for portfolio margin mode, this field returns "", which means leverage rules are invalid
    pub leverage: Decimal,
    /// Mark price
    #[serde(deserialize_with = "decimal_or_zero")]
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
    /// Break even price (linear and inverse only).
    #[serde(default, deserialize_with = "option_decimal")]
    pub break_even_price: Option<Decimal>,
    /// Net delta ratio; option delta is excluded from the calculation.
    #[serde(default, deserialize_with = "option_decimal")]
    pub net_delta_ratio: Option<Decimal>,
    /// Position open timestamp (ms); `None` when not provided or 0.
    #[serde(default, deserialize_with = "option_number")]
    pub open_time: Option<Timestamp>,
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
    #[serde(deserialize_with = "decimal_or_zero")]
    pub cur_realised_pnl: Decimal,
    /// Cumulative realised pnl
    /// Futures & Perpetuals: it is the all time cumulative realised P&L
    /// Option: always "", meaningless
    #[serde(deserialize_with = "decimal_or_zero")]
    pub cum_realised_pnl: Decimal,
    /// Auto-deleverage rank indicator. What is Auto-Deleveraging?
    pub adl_rank_indicator: AdlRankIndicator,
    /// Timestamp of the first time a position was created on this symbol (ms)
    #[serde(deserialize_with = "timestamp_or_zero")]
    pub created_time: Timestamp,
    /// Position updated timestamp (ms)
    #[serde(deserialize_with = "timestamp_or_zero")]
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
    /// Replace the position with a `position` stream update (the stream
    /// always sends the full position).
    pub fn update(&mut self, msg: PositionMsg) {
        *self = msg.into();
    }
}

impl From<PositionMsg> for Position {
    fn from(msg: PositionMsg) -> Self {
        Self {
            position_idx: msg.position_idx,
            risk_id: msg.risk_id,
            risk_limit_value: msg.risk_limit_value,
            symbol: msg.symbol,
            side: msg.side,
            size: msg.size,
            avg_price: msg.entry_price,
            position_value: Some(msg.position_value),
            auto_add_margin: msg.auto_add_margin,
            position_status: Some(msg.position_status),
            leverage: msg.leverage,
            mark_price: msg.mark_price,
            liq_price: msg.liq_price,
            position_im: msg.position_im,
            position_im_by_mp: msg.position_im_by_mp,
            position_mm: msg.position_mm,
            position_mm_by_mp: msg.position_mm_by_mp,
            take_profit: Some(msg.take_profit),
            stop_loss: Some(msg.stop_loss),
            trailing_stop: Some(msg.trailing_stop),
            session_avg_price: msg.session_avg_price,
            // Not sent by the position stream.
            break_even_price: None,
            net_delta_ratio: None,
            open_time: None,
            delta: msg.delta,
            gamma: msg.gamma,
            vega: msg.vega,
            theta: msg.theta,
            unrealised_pnl: Some(msg.unrealised_pnl),
            cur_realised_pnl: msg.cur_realised_pnl,
            cum_realised_pnl: msg.cum_realised_pnl,
            adl_rank_indicator: msg.adl_rank_indicator,
            created_time: msg.created_time,
            updated_time: msg.updated_time,
            seq: msg.seq,
            is_reduce_only: msg.is_reduce_only,
            mmr_sys_updated_time: msg.mmr_sys_updated_time,
            leverage_sys_updated_time: msg.leverage_sys_updated_time,
        }
    }
}

// ── Set Leverage ─────────────────────────────────────────────────────────────

/// Request body of `POST /v5/position/set-leverage` ([`Client::set_leverage`](crate::http::Client::set_leverage)).
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetLeverageRequest {
    /// linear, inverse
    pub category: Category,
    /// Symbol name, like BTCUSDT, uppercase only.
    pub symbol: String,
    /// Under cross margin mode: 0 < leverage ≤ maxLeverage (must equal sellLeverage).
    /// Under isolated margin mode: 0 < leverage ≤ maxLeverage.
    pub buy_leverage: Decimal,
    /// \[ 1, max leverage\].
    pub sell_leverage: Decimal,
}

impl SetLeverageRequest {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new(category: Category, symbol: impl Into<String>, leverage: Decimal) -> Self {
        let symbol: String = symbol.into();
        Self {
            category,
            symbol,
            buy_leverage: leverage,
            sell_leverage: leverage,
        }
    }

    /// Different leverage for the buy and sell side (isolated margin only).
    pub fn with_asymmetric(mut self, buy_leverage: Decimal, sell_leverage: Decimal) -> Self {
        self.buy_leverage = buy_leverage;
        self.sell_leverage = sell_leverage;
        self
    }
}

// ── Set Trading Stop ─────────────────────────────────────────────────────────

/// Request body of `POST /v5/position/trading-stop` ([`Client::set_trading_stop`](crate::http::Client::set_trading_stop)).
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetTradingStopRequest {
    /// linear, inverse
    pub category: Category,
    /// Symbol name, like BTCUSDT, uppercase only.
    pub symbol: String,
    /// Required. 0: one-way, 1: hedge Buy side, 2: hedge Sell side
    pub position_idx: PositionIdx,
    /// Cannot be less than 0, 0 means cancel TP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_profit: Option<Decimal>,
    /// Cannot be less than 0, 0 means cancel SL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<Decimal>,
    /// Trailing stop distance from market price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_stop: Option<Decimal>,
    /// Take profit trigger price type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_by: Option<TriggerBy>,
    /// Stop loss trigger price type.
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
    /// The order type when take profit is triggered. Market (default), Limit For tpslMode=Full, it only supports tpOrderType="Market".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_order_type: Option<OrderType>,
    /// The order type when stop loss is triggered. Market (default), Limit For tpslMode=Full, it only supports slOrderType="Market".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_order_type: Option<OrderType>,
    /// TP/SL mode Full: entire position TP/SL, option supports "tpslMode"= Full only Partial: partial position TP/SL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpsl_mode: Option<TpslMode>,
}

impl SetTradingStopRequest {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new(category: Category, symbol: impl Into<String>, position_idx: PositionIdx) -> Self {
        let symbol: String = symbol.into();
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

    /// Set `take_profit`: cannot be less than 0, 0 means cancel TP.
    pub fn with_take_profit(mut self, v: Decimal) -> Self {
        self.take_profit = Some(v);
        self
    }
    /// Set `stop_loss`: cannot be less than 0, 0 means cancel SL.
    pub fn with_stop_loss(mut self, v: Decimal) -> Self {
        self.stop_loss = Some(v);
        self
    }
    /// Set `trailing_stop`: trailing stop distance from market price.
    pub fn with_trailing_stop(mut self, v: Decimal) -> Self {
        self.trailing_stop = Some(v);
        self
    }
    /// Set `tp_trigger_by`: take profit trigger price type.
    pub fn with_tp_trigger_by(mut self, v: TriggerBy) -> Self {
        self.tp_trigger_by = Some(v);
        self
    }
    /// Set `sl_trigger_by`: stop loss trigger price type.
    pub fn with_sl_trigger_by(mut self, v: TriggerBy) -> Self {
        self.sl_trigger_by = Some(v);
        self
    }
    /// Set `active_price`: activation price for trailing stop.
    pub fn with_active_price(mut self, v: Decimal) -> Self {
        self.active_price = Some(v);
        self
    }
    /// Set `tp_size`: partial TP quantity.
    pub fn with_tp_size(mut self, v: Decimal) -> Self {
        self.tp_size = Some(v);
        self
    }
    /// Set `sl_size`: partial SL quantity.
    pub fn with_sl_size(mut self, v: Decimal) -> Self {
        self.sl_size = Some(v);
        self
    }
    /// Set `tp_limit_price`: limit price for TP limit order.
    pub fn with_tp_limit_price(mut self, v: Decimal) -> Self {
        self.tp_limit_price = Some(v);
        self
    }
    /// Set `sl_limit_price`: limit price for SL limit order.
    pub fn with_sl_limit_price(mut self, v: Decimal) -> Self {
        self.sl_limit_price = Some(v);
        self
    }
    /// Set `tp_order_type`: the order type when take profit is triggered. Market (default), Limit For tpslMode=Full, it only supports tpOrderType="Market".
    pub fn with_tp_order_type(mut self, v: OrderType) -> Self {
        self.tp_order_type = Some(v);
        self
    }
    /// Set `sl_order_type`: the order type when stop loss is triggered. Market (default), Limit For tpslMode=Full, it only supports slOrderType="Market".
    pub fn with_sl_order_type(mut self, v: OrderType) -> Self {
        self.sl_order_type = Some(v);
        self
    }
    /// Set `tpsl_mode`: tP/SL mode Full: entire position TP/SL, option supports "tpslMode"= Full only Partial: partial position TP/SL.
    pub fn with_tpsl_mode(mut self, v: TpslMode) -> Self {
        self.tpsl_mode = Some(v);
        self
    }
}

// ── Switch Cross / Isolated Margin ───────────────────────────────────────────

/// Request body of `POST /v5/position/switch-isolated` ([`Client::switch_cross_isolated_margin`](crate::http::Client::switch_cross_isolated_margin)).
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SwitchCrossIsolatedMarginRequest {
    /// linear, inverse
    pub category: Category,
    /// Symbol name.
    pub symbol: String,
    /// 0: cross margin, 1: isolated margin
    pub trade_mode: TradeMode,
    /// \[ 1, max leverage\] one-way mode: buyLeverage must be the same as sellLeverage Hedge mode: isolated margin: buyLeverage and sellLeverage can be different; cross margin: buyLeverage must be the same as sellLeverage.
    pub buy_leverage: Decimal,
    /// \[ 1, max leverage\].
    pub sell_leverage: Decimal,
}

impl SwitchCrossIsolatedMarginRequest {
    /// Switch to cross margin with the given leverage.
    pub fn cross(category: Category, symbol: String, leverage: Decimal) -> Self {
        Self {
            category,
            symbol,
            trade_mode: TradeMode::CrossMargin,
            buy_leverage: leverage,
            sell_leverage: leverage,
        }
    }

    /// Switch to isolated margin with the given buy and sell leverage.
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

/// Request body of `POST /v5/position/switch-mode` ([`Client::switch_position_mode`](crate::http::Client::switch_position_mode)).
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
    /// Switch to one-way mode.
    pub fn one_way(category: Category, symbol: String) -> Self {
        Self {
            category,
            symbol: Some(symbol),
            coin: None,
            mode: PositionMode::OneWay,
        }
    }

    /// Switch to hedge mode (separate buy and sell positions).
    pub fn hedge(category: Category, symbol: String) -> Self {
        Self {
            category,
            symbol: Some(symbol),
            coin: None,
            mode: PositionMode::Hedge,
        }
    }

    /// Set `coin`: coin. Either symbol or coin is required.
    pub fn with_coin(mut self, v: impl Into<String>) -> Self {
        self.symbol = None;
        self.coin = Some(v.into());
        self
    }
}

// ── Set Auto Add Margin ──────────────────────────────────────────────────────

/// Request body of `POST /v5/position/set-auto-add-margin` ([`Client::set_auto_add_margin`](crate::http::Client::set_auto_add_margin)).
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetAutoAddMarginRequest {
    /// linear, inverse
    pub category: Category,
    /// Symbol name, like BTCUSDT, uppercase only.
    pub symbol: String,
    /// 0: false, 1: true
    pub auto_add_margin: i32,
    /// Used to identify positions in different position modes. For hedge mode position, this param is required 0: one-way mode 1: hedge-mode Buy side 2: hedge-mode Sell side.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_idx: Option<PositionIdx>,
}

impl SetAutoAddMarginRequest {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new(category: Category, symbol: impl Into<String>, enabled: bool) -> Self {
        let symbol: String = symbol.into();
        Self {
            category,
            symbol,
            auto_add_margin: if enabled { 1 } else { 0 },
            position_idx: None,
        }
    }

    /// Set `position_idx`: used to identify positions in different position modes. For hedge mode position, this param is required 0: one-way mode 1: hedge-mode Buy side 2: hedge-mode Sell side.
    pub fn with_position_idx(mut self, v: PositionIdx) -> Self {
        self.position_idx = Some(v);
        self
    }
}

// ── Set Risk Limit ───────────────────────────────────────────────────────────

/// Request body of `POST /v5/position/set-risk-limit` ([`Client::set_risk_limit`](crate::http::Client::set_risk_limit)).
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetRiskLimitRequest {
    /// linear, inverse
    pub category: Category,
    /// Symbol name.
    pub symbol: String,
    /// Risk limit ID.
    pub risk_id: i64,
    /// Position index. Used to identify positions in different position modes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_idx: Option<PositionIdx>,
}

impl SetRiskLimitRequest {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new(category: Category, symbol: impl Into<String>, risk_id: i64) -> Self {
        let symbol: String = symbol.into();
        Self {
            category,
            symbol,
            risk_id,
            position_idx: None,
        }
    }

    /// Set `position_idx`: position index. Used to identify positions in different position modes.
    pub fn with_position_idx(mut self, v: PositionIdx) -> Self {
        self.position_idx = Some(v);
        self
    }
}

/// Result of `POST /v5/position/set-risk-limit` ([`Client::set_risk_limit`](crate::http::Client::set_risk_limit)).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetRiskLimitResponse {
    /// Risk limit ID.
    pub risk_id: i64,
    /// Risk limit value.
    pub risk_limit_value: String,
    /// Product type.
    pub category: String,
    /// Message, if any.
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub message: Option<String>,
}

// ── Closed P&L ───────────────────────────────────────────────────────────────

/// Query parameters of `GET /v5/position/closed-pnl` ([`Client::get_closed_pnl`](crate::http::Client::get_closed_pnl)).
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetClosedPnlParams {
    /// linear, inverse
    pub category: Category,
    /// Required for inverse; optional for linear
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// The start timestamp (ms) startTime and endTime are not passed, return 7 days by default Only startTime is passed, return range between startTime and startTime+7 days Only endTime is passed, return range between endTime-7 days and endTime ...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Timestamp>,
    /// The end timestamp (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<Timestamp>,
    /// [1, 100]. Default: 50
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Cursor. Use the nextPageCursor token from the response to retrieve the next page of the result set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetClosedPnlParams {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
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

    /// Set `symbol`: required for inverse; optional for linear.
    pub fn with_symbol(mut self, v: impl Into<String>) -> Self {
        self.symbol = Some(v.into());
        self
    }
    /// Set `start_time`: the start timestamp (ms) startTime and endTime are not passed, return 7 days by default Only startTime is passed, return range between startTime and startTime+7 days Only endTime is passed, return range between endTime-7 days and endTime ...
    pub fn with_start_time(mut self, v: Timestamp) -> Self {
        self.start_time = Some(v);
        self
    }
    /// Set `end_time`: the end timestamp (ms).
    pub fn with_end_time(mut self, v: Timestamp) -> Self {
        self.end_time = Some(v);
        self
    }
    /// Set `limit`: [1, 100]. Default: 50.
    pub fn with_limit(mut self, v: i32) -> Self {
        self.limit = Some(v);
        self
    }
    /// Set `cursor`: cursor. Use the nextPageCursor token from the response to retrieve the next page of the result set.
    pub fn with_cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
}

/// Item of the list returned by `GET /v5/position/closed-pnl` ([`Client::get_closed_pnl`](crate::http::Client::get_closed_pnl)).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClosedPnl {
    /// Symbol name.
    pub symbol: String,
    /// Order ID.
    pub order_id: String,
    /// User customised order ID.
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub order_link_id: Option<String>,
    /// Buy, Sell.
    pub side: Side,
    /// Order qty.
    #[serde(deserialize_with = "number")]
    pub qty: Decimal,
    /// Order price.
    #[serde(deserialize_with = "number")]
    pub order_price: Decimal,
    /// Order type. Market, Limit.
    pub order_type: OrderType,
    /// Exec type Trade, BustTrade SessionSettlePnL Settle, MovePosition ForwardSplitSettle, ReverseSplitSettle.
    pub exec_type: ExecType,
    /// Closed size.
    #[serde(deserialize_with = "number")]
    pub closed_size: Decimal,
    /// Cumulated Position value.
    pub cum_entry_value: Decimal,
    /// Average entry price.
    pub avg_entry_price: Decimal,
    /// Cumulated exit position value.
    pub cum_exit_value: Decimal,
    /// Average exit price.
    pub avg_exit_price: Decimal,
    /// Closed PnL.
    pub closed_pnl: Decimal,
    /// The number of fills in a single order.
    #[serde(deserialize_with = "number")]
    pub fill_count: i64,
    /// Leverage.
    pub leverage: Decimal,
    /// The created time (ms).
    #[serde(deserialize_with = "number")]
    pub created_time: Timestamp,
    /// The updated time (ms).
    #[serde(deserialize_with = "number")]
    pub updated_time: Timestamp,
}

// ── Execution List ────────────────────────────────────────────────────────────

/// Query parameters of `GET /v5/execution/list` ([`Client::get_execution_list`](crate::http::Client::get_execution_list)).
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetExecutionListParams {
    /// Product type linear, inverse, spot, option.
    pub category: Category,
    /// Symbol name, like BTCUSDT, uppercase only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// User customised order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    /// Base coin, uppercase only. For type option, default value is BTC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    /// The start timestamp (ms) startTime and endTime are not passed, return 7 days by default Only startTime is passed, return range between startTime and startTime+7 days Only endTime is passed, return range between endTime-7 days and endTime ...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Timestamp>,
    /// The end timestamp (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<Timestamp>,
    /// Execution type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_type: Option<ExecType>,
    /// [1, 100]. Default: 50
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Cursor. Use the nextPageCursor token from the response to retrieve the next page of the result set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetExecutionListParams {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
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

    /// Set `symbol`: symbol name, like BTCUSDT, uppercase only.
    pub fn with_symbol(mut self, v: impl Into<String>) -> Self {
        self.symbol = Some(v.into());
        self
    }
    /// Set `order_id`: order ID.
    pub fn with_order_id(mut self, v: impl Into<String>) -> Self {
        self.order_id = Some(v.into());
        self
    }
    /// Set `order_link_id`: user customised order ID.
    pub fn with_order_link_id(mut self, v: impl Into<String>) -> Self {
        self.order_link_id = Some(v.into());
        self
    }
    /// Set `base_coin`: base coin, uppercase only. For type option, default value is BTC.
    pub fn with_base_coin(mut self, v: impl Into<String>) -> Self {
        self.base_coin = Some(v.into());
        self
    }
    /// Set `start_time`: the start timestamp (ms) startTime and endTime are not passed, return 7 days by default Only startTime is passed, return range between startTime and startTime+7 days Only endTime is passed, return range between endTime-7 days and endTime ...
    pub fn with_start_time(mut self, v: Timestamp) -> Self {
        self.start_time = Some(v);
        self
    }
    /// Set `end_time`: the end timestamp (ms).
    pub fn with_end_time(mut self, v: Timestamp) -> Self {
        self.end_time = Some(v);
        self
    }
    /// Set `exec_type`: execution type.
    pub fn with_exec_type(mut self, v: ExecType) -> Self {
        self.exec_type = Some(v);
        self
    }
    /// Set `limit`: [1, 100]. Default: 50.
    pub fn with_limit(mut self, v: i32) -> Self {
        self.limit = Some(v);
        self
    }
    /// Set `cursor`: cursor. Use the nextPageCursor token from the response to retrieve the next page of the result set.
    pub fn with_cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
}

/// Item of the list returned by `GET /v5/execution/list` ([`Client::get_execution_list`](crate::http::Client::get_execution_list)).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionEntry {
    /// Symbol name.
    pub symbol: String,
    /// Order ID.
    pub order_id: String,
    /// User customized order ID.
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub order_link_id: Option<String>,
    /// Side. Buy, Sell.
    pub side: Side,
    /// Order price.
    pub order_price: Decimal,
    /// Order qty.
    pub order_qty: Decimal,
    /// Order type. Market, Limit.
    pub order_type: OrderType,
    /// Stop order type. If the order is not stop order, it either returns UNKNOWN or "".
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub stop_order_type: Option<StopOrderType>,
    /// Executed trading fee. You can get spot fee currency instruction here.
    pub exec_fee: Decimal,
    /// Execution ID.
    pub exec_id: String,
    /// Execution price.
    pub exec_price: Decimal,
    /// Execution qty.
    pub exec_qty: Decimal,
    /// Executed type.
    pub exec_type: ExecType,
    /// Executed order value.
    pub exec_value: Decimal,
    /// Executed timestamp (ms).
    #[serde(deserialize_with = "number")]
    pub exec_time: Timestamp,
    /// Trading fee rate.
    pub fee_rate: Decimal,
    /// Implied volatility. Valid for option.
    #[serde(default, deserialize_with = "option_decimal")]
    pub trade_iv: Option<Decimal>,
    /// Implied volatility of mark price. Valid for option.
    #[serde(default, deserialize_with = "option_decimal")]
    pub mark_iv: Option<Decimal>,
    /// The mark price of the symbol when executing.
    pub mark_price: Decimal,
    /// The index price of the symbol when executing. Valid for option only.
    #[serde(default, deserialize_with = "option_decimal")]
    pub index_price: Option<Decimal>,
    /// The underlying price of the symbol when executing. Valid for option.
    #[serde(default, deserialize_with = "option_decimal")]
    pub underlying_price: Option<Decimal>,
    /// Paradigm block trade ID.
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub block_trade_id: Option<String>,
    /// Closed position size.
    pub closed_size: Decimal,
    /// Cross sequence, used to associate each fill and each position update The seq will be the same when conclude multiple transactions at the same time Different symbols may have the same seq, please use seq + symbol to check unique.
    pub seq: i64,
    /// Is maker order. true: maker, false: taker.
    pub is_maker: bool,
}
