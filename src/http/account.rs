use std::collections::HashMap;

use rust_decimal::{Decimal, serde::str_option::deserialize as option_decimal};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_number_from_string as number;

use crate::{
    AccountType, DCPProduct, MarginMode, Second, SpotHedgingStatus, Timestamp, UnifiedMarginStatus,
    UnifiedUpdateStatus,
    enums::Category,
    serde::{Unique, empty_string_as_none, hash_map},
    ws::WalletMsg,
};

/// Query parameters of `GET /v5/account/wallet-balance` ([`Client::get_wallet_balance`](crate::http::Client::get_wallet_balance)).
#[derive(Debug, Serialize, Clone)]
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

impl GetWalletBalanceParams {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new(account_type: AccountType) -> Self {
        Self {
            account_type,
            coin: None,
        }
    }

    /// Set `coin`: coin name, uppercase only If not passed, it returns non-zero asset info You can pass multiple coins to query, separated by comma. USDT,USDC.
    pub fn with_coin(mut self, v: impl Into<String>) -> Self {
        self.coin = Some(v.into());
        self
    }
}

/// Item of the list returned by `GET /v5/account/wallet-balance` ([`Client::get_wallet_balance`](crate::http::Client::get_wallet_balance)).
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
    /// Coin name, uppercase only If not passed, it returns non-zero asset info You can pass multiple coins to query, separated by comma. USDT,USDC.
    #[serde(deserialize_with = "hash_map")]
    pub coin: HashMap<String, WalletCoin>,
}

impl WalletBalance {
    /// Replace the balance with a `wallet` stream update (the stream always
    /// sends the full balance of the account type).
    pub fn update(&mut self, msg: WalletMsg) {
        *self = msg.into();
    }
}

impl From<WalletMsg> for WalletBalance {
    fn from(msg: WalletMsg) -> Self {
        Self {
            account_type: msg.account_type,
            account_im_rate: msg.account_im_rate,
            account_im_rate_by_mp: msg.account_im_rate_by_mp,
            account_mm_rate: msg.account_mm_rate,
            account_mm_rate_by_mp: msg.account_mm_rate_by_mp,
            total_equity: msg.total_equity,
            total_wallet_balance: msg.total_wallet_balance,
            total_margin_balance: msg.total_margin_balance,
            total_available_balance: msg.total_available_balance,
            total_perp_upl: msg.total_perp_upl,
            total_initial_margin: msg.total_initial_margin,
            total_initial_margin_by_mp: msg.total_initial_margin_by_mp,
            total_maintenance_margin: msg.total_maintenance_margin,
            total_maintenance_margin_by_mp: msg.total_maintenance_margin_by_mp,
            coin: msg.coin,
        }
    }
}

/// Part of the response of `/v5/account/wallet-balance`.
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

/// Result of `GET /v5/account/info` ([`Client::get_account_info`](crate::http::Client::get_account_info)).
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

/// Disconnection Cancellation Protection settings of one product.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DCPConfiguration {
    /// DCP config for each product
    pub dcp_infos: Vec<DCPInfo>,
}

/// Disconnection Cancellation Protection (DCP) settings of the account.
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

/// Query parameters of `GET /v5/account/transaction-log` ([`Client::get_transaction_log`](crate::http::Client::get_transaction_log)).
#[derive(Clone, Debug, Serialize)]
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
    pub r#type: Option<String>,
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

impl GetTransactionLogParams {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new() -> Self {
        Self {
            account_type: None,
            category: None,
            currency: None,
            base_coin: None,
            settle_coin: None,
            r#type: None,
            trans_sub_type: None,
            start_time: None,
            end_time: None,
            limit: None,
            cursor: None,
        }
    }

    /// Set `account_type`: account Type. UNIFIED.
    pub fn with_account_type(mut self, v: AccountType) -> Self {
        self.account_type = Some(v);
        self
    }
    /// Set `category`: product type spot,linear,option,inverse.
    pub fn with_category(mut self, v: Category) -> Self {
        self.category = Some(v);
        self
    }
    /// Set `currency`: currency, uppercase only.
    pub fn with_currency(mut self, v: impl Into<String>) -> Self {
        self.currency = Some(v.into());
        self
    }
    /// Set `base_coin`: baseCoin, uppercase only. e.g., BTC of BTCPERP.
    pub fn with_base_coin(mut self, v: impl Into<String>) -> Self {
        self.base_coin = Some(v.into());
        self
    }
    /// Set `settle_coin`: not documented. Settle coin linear: either symbol or settleCoin is required. symbol has a higher priority.
    pub fn with_settle_coin(mut self, v: impl Into<String>) -> Self {
        self.settle_coin = Some(v.into());
        self
    }
    /// Set `type`: types of transaction logs.
    pub fn with_type(mut self, v: impl Into<String>) -> Self {
        self.r#type = Some(v.into());
        self
    }
    /// Set `trans_sub_type`: movePosition, used to filter trans logs of Move Position only.
    pub fn with_trans_sub_type(mut self, v: impl Into<String>) -> Self {
        self.trans_sub_type = Some(v.into());
        self
    }
    /// Set `start_time`: the start timestamp (ms) startTime and endTime are not passed, return 24 hours by default Only startTime is passed, return range between startTime and startTime+24 hours Only endTime is passed, return range between endTime-24 hours and ...
    pub fn with_start_time(mut self, v: Timestamp) -> Self {
        self.start_time = Some(v);
        self
    }
    /// Set `end_time`: the end timestamp (ms).
    pub fn with_end_time(mut self, v: Timestamp) -> Self {
        self.end_time = Some(v);
        self
    }
    /// Set `limit`: limit for data size per page. [1, 50]. Default: 20.
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

impl Default for GetTransactionLogParams {
    fn default() -> Self {
        Self::new()
    }
}

/// Item of the list returned by `GET /v5/account/transaction-log` ([`Client::get_transaction_log`](crate::http::Client::get_transaction_log)).
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
    pub side: crate::Side,
    /// Transaction timestamp (ms)
    #[serde(deserialize_with = "number")]
    pub transaction_time: Timestamp,
    /// Type
    #[serde(rename = "type")]
    pub r#type: String,
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
    #[serde(default, deserialize_with = "crate::serde::empty_string_as_none")]
    pub order_link_id: Option<String>,
    /// Trading fee rate information. Currently, this data is returned only for spot orders placed on the Indonesian site or spot fiat currency orders placed on the EU site. In other cases, an empty string is returned. Enum: feeType, subFeeType
    #[serde(default)]
    pub extra_fees: Option<serde_json::Value>,
}

// --- Fee Rate ---

/// Query params for [`Client::get_fee_rate`](crate::http::Client::get_fee_rate).
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetFeeRateParams {
    /// Product type. spot, linear, inverse, option. Passing linear or inverse returns the same result - fee rates for all contracts (both linear and inverse).
    pub category: Category,
    /// Symbol name, like BTCUSDT, uppercase only. Valid for linear, inverse, spot.
    pub symbol: Option<String>,
    /// Only for `category = option`. Base coin, e.g. `"BTC"`.
    pub base_coin: Option<String>,
}

impl GetFeeRateParams {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new(category: Category) -> Self {
        Self {
            category,
            symbol: None,
            base_coin: None,
        }
    }

    /// Set `symbol`: symbol name, like BTCUSDT, uppercase only. Valid for linear, inverse, spot.
    pub fn with_symbol(mut self, v: impl Into<String>) -> Self {
        self.symbol = Some(v.into());
        self
    }

    /// Set `base_coin`: only for `category = option`. Base coin, e.g. `"BTC"`.
    pub fn with_base_coin(mut self, v: impl Into<String>) -> Self {
        self.base_coin = Some(v.into());
        self
    }
}

/// Item of the list returned by `GET /v5/account/fee-rate` ([`Client::get_fee_rate`](crate::http::Client::get_fee_rate)).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FeeRateEntry {
    /// Symbol name. Spot/linear/inverse only.
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub symbol: Option<String>,
    /// Base coin. Option only.
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub base_coin: Option<String>,
    /// Taker fee rate.
    pub taker_fee_rate: Decimal,
    /// Maker fee rate.
    pub maker_fee_rate: Decimal,
}

// --- Set Margin Mode ---

/// Request body for [`Client::set_margin_mode`](crate::http::Client::set_margin_mode).
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetMarginModeRequest {
    /// ISOLATED_MARGIN, REGULAR_MARGIN (i.e. Cross margin), PORTFOLIO_MARGIN.
    pub set_margin_mode: MarginMode,
}

impl SetMarginModeRequest {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new(set_margin_mode: MarginMode) -> Self {
        Self { set_margin_mode }
    }
}

/// Response for [`Client::set_margin_mode`](crate::http::Client::set_margin_mode).
///
/// On success `reasons` is empty. On failure it contains error details
/// explaining why the switch was rejected.
#[derive(Debug, Deserialize, PartialEq)]
pub struct SetMarginModeResponse {
    /// Object. If requested successfully, it is an empty array.
    pub reasons: Vec<MarginModeFailureReason>,
}

/// Part of the response of `/v5/account/set-margin-mode`.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MarginModeFailureReason {
    /// Fail reason code.
    pub reason_code: String,
    /// Fail reason msg.
    pub reason_msg: String,
}

// --- Upgrade to Unified Account (Pro) ---

/// Response for [`Client::upgrade_to_uta`](crate::http::Client::upgrade_to_uta).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UpgradeToUtaResult {
    /// Upgrade status. FAIL, PROCESS, SUCCESS.
    pub unified_update_status: UnifiedUpdateStatus,
    /// `None` unless `unified_update_status` is `Fail`.
    pub unified_update_msg: Option<UnifiedUpdateMsg>,
}

/// Part of the response of `/v5/account/upgrade-to-uta`.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UnifiedUpdateMsg {
    /// Error message array. Only FAIL will have this field.
    pub msg: Vec<String>,
}

// --- Get Borrow History ---

/// Query params for [`Client::get_borrow_history`](crate::http::Client::get_borrow_history).
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBorrowHistoryParams {
    /// Currency, uppercase only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// The start timestamp (ms). Combined with end_time, cannot exceed 30 days
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Timestamp>,
    /// The end timestamp (ms). Combined with start_time, cannot exceed 30 days
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<Timestamp>,
    /// Limit for data size per page. [1, 50]. Default: 20
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    /// Cursor. Use the nextPageCursor token from the response to retrieve the next page of the result set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetBorrowHistoryParams {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new() -> Self {
        Self {
            currency: None,
            start_time: None,
            end_time: None,
            limit: None,
            cursor: None,
        }
    }

    /// Set `currency`: currency, uppercase only.
    pub fn with_currency(mut self, v: impl Into<String>) -> Self {
        self.currency = Some(v.into());
        self
    }
    /// Set `start_time`: the start timestamp (ms). Combined with end_time, cannot exceed 30 days.
    pub fn with_start_time(mut self, v: Timestamp) -> Self {
        self.start_time = Some(v);
        self
    }
    /// Set `end_time`: the end timestamp (ms). Combined with start_time, cannot exceed 30 days.
    pub fn with_end_time(mut self, v: Timestamp) -> Self {
        self.end_time = Some(v);
        self
    }
    /// Set `limit`: limit for data size per page. [1, 50]. Default: 20.
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

impl Default for GetBorrowHistoryParams {
    fn default() -> Self {
        Self::new()
    }
}

/// Item of the list returned by `GET /v5/account/borrow-history` ([`Client::get_borrow_history`](crate::http::Client::get_borrow_history)).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BorrowHistoryEntry {
    /// Currency
    pub currency: String,
    /// Interest record timestamp (ms)
    #[serde(deserialize_with = "number")]
    pub created_time: Timestamp,
    /// Borrow cost, i.e., accrued interest
    pub borrow_cost: Decimal,
    /// Hourly borrow rate
    pub hourly_borrow_rate: Decimal,
    /// Borrowed amount that accrues interest
    #[serde(rename = "InterestBearingBorrowSize")]
    pub interest_bearing_borrow_size: Decimal,
    /// Amount exempted from interest
    pub cost_exemption: Decimal,
    /// Borrowed amount
    pub borrow_amount: Decimal,
    /// Unrealised loss
    pub unrealised_loss: Decimal,
    /// Amount borrowed within the interest-free threshold
    #[serde(default, deserialize_with = "option_decimal")]
    pub free_borrowed_amount: Option<Decimal>,
}

// --- Get Collateral Info ---

/// Query params for [`Client::get_collateral_info`](crate::http::Client::get_collateral_info).
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetCollateralInfoParams {
    /// Asset currency of all current collateral, uppercase only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

impl GetCollateralInfoParams {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new() -> Self {
        Self { currency: None }
    }

    /// Set `currency`: asset currency of all current collateral, uppercase only.
    pub fn with_currency(mut self, v: impl Into<String>) -> Self {
        self.currency = Some(v.into());
        self
    }
}

impl Default for GetCollateralInfoParams {
    fn default() -> Self {
        Self::new()
    }
}

/// Item of the list returned by `GET /v5/account/collateral-info` ([`Client::get_collateral_info`](crate::http::Client::get_collateral_info)).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CollateralInfoEntry {
    /// Currency of the collateral
    pub currency: String,
    /// Hourly interest rate for borrowing
    pub hourly_borrow_rate: Decimal,
    /// Maximum borrowable amount, shared across main and sub accounts
    pub max_borrowing_amount: Decimal,
    /// Maximum limit for interest-free borrowing
    pub free_borrowing_limit: Decimal,
    /// Amount borrowed within the interest-free threshold
    pub free_borrow_amount: Decimal,
    /// Current borrow amount
    pub borrow_amount: Decimal,
    /// Borrowing amount from other accounts under the same main account
    pub other_borrow_amount: Decimal,
    /// Remaining amount available to borrow
    pub available_to_borrow: Decimal,
    /// Whether currency can be borrowed
    pub borrowable: bool,
    /// Borrow usage ratio, e.g. 0.5 = 50%
    pub borrow_usage_rate: Decimal,
    /// Whether usable as margin collateral
    pub margin_collateral: bool,
    /// Whether collateral is enabled by user
    pub collateral_switch: bool,
    /// Deprecated. Refer to the tiered collateral ratio endpoint instead
    #[serde(default, deserialize_with = "option_decimal")]
    pub collateral_ratio: Option<Decimal>,
    /// Deprecated. Always empty
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub free_borrowing_amount: Option<String>,
}
