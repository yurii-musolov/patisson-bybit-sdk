use rust_decimal::{Decimal, serde::str_option::deserialize as option_decimal};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::{
    deserialize_number_from_string as number,
    deserialize_option_number_from_string as option_number,
};

use crate::{
    AccountType, DepositStatus, Side, Timestamp, TransferStatus, WithdrawStatus, enums::Category,
    serde::empty_string_as_none,
};

// ════════════════════════════════════════ Transfer ════════════════════════════════════════

// --- Create Internal Transfer ---

/// Request body for [`Client::internal_transfer`](crate::http::Client::internal_transfer).
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InternalTransferRequest {
    /// UUID. Please manually generate a UUID and ensure it is unique. Used to prevent duplicate transfers
    pub transfer_id: String,
    /// Coin name, uppercase only
    pub coin: String,
    /// Transfer amount
    pub amount: Decimal,
    pub from_account_type: AccountType,
    pub to_account_type: AccountType,
}

impl InternalTransferRequest {
    pub fn new(
        transfer_id: String,
        coin: String,
        amount: Decimal,
        from_account_type: AccountType,
        to_account_type: AccountType,
    ) -> Self {
        Self {
            transfer_id,
            coin,
            amount,
            from_account_type,
            to_account_type,
        }
    }
}

/// Response for [`Client::internal_transfer`](crate::http::Client::internal_transfer) and
/// [`Client::universal_transfer`](crate::http::Client::universal_transfer).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TransferResult {
    pub transfer_id: String,
    pub status: TransferStatus,
}

// --- Get Internal Transfer Records ---

/// Query params for
/// [`Client::get_internal_transfer_records`](crate::http::Client::get_internal_transfer_records).
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInternalTransferRecordsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TransferStatus>,
    /// Without start_time and end_time, returns the past 7 days by default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<Timestamp>,
    /// Limit for data size per page. [1, 50]. Default: 20
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetInternalTransferRecordsParams {
    pub fn new() -> Self {
        Self {
            transfer_id: None,
            coin: None,
            status: None,
            start_time: None,
            end_time: None,
            limit: None,
            cursor: None,
        }
    }

    pub fn with_transfer_id(mut self, v: String) -> Self {
        self.transfer_id = Some(v);
        self
    }
    pub fn with_coin(mut self, v: String) -> Self {
        self.coin = Some(v);
        self
    }
    pub fn with_status(mut self, v: TransferStatus) -> Self {
        self.status = Some(v);
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
    pub fn with_limit(mut self, v: u64) -> Self {
        self.limit = Some(v);
        self
    }
    pub fn with_cursor(mut self, v: String) -> Self {
        self.cursor = Some(v);
        self
    }
}

impl Default for GetInternalTransferRecordsParams {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InternalTransferEntry {
    pub transfer_id: String,
    pub coin: String,
    pub amount: Decimal,
    pub from_account_type: AccountType,
    pub to_account_type: AccountType,
    #[serde(deserialize_with = "number")]
    pub timestamp: Timestamp,
    pub status: TransferStatus,
}

// --- Create Universal Transfer ---

/// Request body for [`Client::universal_transfer`](crate::http::Client::universal_transfer).
///
/// Transfers funds between a master account and its sub-accounts (or between two sub-accounts).
/// Requires the master UID's API key, or a sub-account key with the `SubMemberTransferList`
/// permission (which may only transfer to the master account).
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UniversalTransferRequest {
    /// UUID. Please manually generate a UUID and ensure it is unique. Used to prevent duplicate transfers
    pub transfer_id: String,
    /// Coin name, uppercase only
    pub coin: String,
    /// Transfer amount
    pub amount: Decimal,
    /// Source UID
    pub from_member_id: i64,
    /// Destination UID
    pub to_member_id: i64,
    pub from_account_type: AccountType,
    pub to_account_type: AccountType,
}

impl UniversalTransferRequest {
    pub fn new(
        transfer_id: String,
        coin: String,
        amount: Decimal,
        from_member_id: i64,
        to_member_id: i64,
        from_account_type: AccountType,
        to_account_type: AccountType,
    ) -> Self {
        Self {
            transfer_id,
            coin,
            amount,
            from_member_id,
            to_member_id,
            from_account_type,
            to_account_type,
        }
    }
}

// --- Get Universal Transfer Records ---

/// Query params for
/// [`Client::get_universal_transfer_records`](crate::http::Client::get_universal_transfer_records).
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUniversalTransferRecordsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TransferStatus>,
    /// Without start_time and end_time, returns the past 7 days by default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<Timestamp>,
    /// Limit for data size per page. [1, 50]. Default: 20
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetUniversalTransferRecordsParams {
    pub fn new() -> Self {
        Self {
            transfer_id: None,
            coin: None,
            status: None,
            start_time: None,
            end_time: None,
            limit: None,
            cursor: None,
        }
    }

    pub fn with_transfer_id(mut self, v: String) -> Self {
        self.transfer_id = Some(v);
        self
    }
    pub fn with_coin(mut self, v: String) -> Self {
        self.coin = Some(v);
        self
    }
    pub fn with_status(mut self, v: TransferStatus) -> Self {
        self.status = Some(v);
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
    pub fn with_limit(mut self, v: u64) -> Self {
        self.limit = Some(v);
        self
    }
    pub fn with_cursor(mut self, v: String) -> Self {
        self.cursor = Some(v);
        self
    }
}

impl Default for GetUniversalTransferRecordsParams {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UniversalTransferEntry {
    pub transfer_id: String,
    pub coin: String,
    pub amount: Decimal,
    pub from_member_id: String,
    pub to_member_id: String,
    pub from_account_type: AccountType,
    pub to_account_type: AccountType,
    #[serde(deserialize_with = "number")]
    pub timestamp: Timestamp,
    pub status: TransferStatus,
}

// --- Get Transferable Coin ---

/// Query params for
/// [`Client::get_transferable_coins`](crate::http::Client::get_transferable_coins).
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetTransferableCoinsParams {
    pub from_account_type: AccountType,
    pub to_account_type: AccountType,
}

impl GetTransferableCoinsParams {
    pub fn new(from_account_type: AccountType, to_account_type: AccountType) -> Self {
        Self {
            from_account_type,
            to_account_type,
        }
    }
}

// --- Enable Universal Transfer For Sub UID (deprecated by Bybit; all sub UIDs are now
// automatically enabled) ---

/// Request body for
/// [`Client::save_transfer_sub_member`](crate::http::Client::save_transfer_sub_member).
///
/// Deprecated by Bybit: sub UIDs no longer need to be configured, as all of them are now
/// automatically enabled for universal transfer. Kept for completeness.
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SaveTransferSubMemberRequest {
    /// Sub UIDs, separated by comma, e.g. "uid1,uid2,uid3"
    pub sub_member_ids: String,
}

impl SaveTransferSubMemberRequest {
    pub fn new(sub_member_ids: String) -> Self {
        Self { sub_member_ids }
    }
}

// --- Get Sub UID (for universal transfer) ---

/// Response for
/// [`Client::get_transferable_sub_members`](crate::http::Client::get_transferable_sub_members).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TransferableSubMembers {
    /// All sub UIDs of the master account
    pub sub_member_ids: Vec<String>,
    /// Sub UIDs enabled for universal transfer
    pub transferable_sub_member_ids: Vec<String>,
}

// --- Get Single Coin Balance ---

/// Query params for
/// [`Client::get_account_coin_balance`](crate::http::Client::get_account_coin_balance).
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountCoinBalanceParams {
    pub account_type: AccountType,
    /// Coin name, uppercase only
    pub coin: String,
    /// Master UID's api key can query itself or its sub UIDs' coin balance. If not passed,
    /// then by default it queries master UID's coin balance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// Used together with `to_account_type` to query the balance safe to transfer between two
    /// UIDs' wallets
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_member_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_account_type: Option<AccountType>,
    /// Whether to query bonus. 0: not query, 1: query. Valid for `account_type=FUND`. Default: 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_bonus: Option<i64>,
    /// Whether to query the available amount that is safe to withdraw or transfer.
    /// 0(default): not query, 1: query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_transfer_safe_amount: Option<i64>,
    /// Whether to query the available amount that is safe to transfer under the risk level of
    /// the OTC loan account. 0(default): not query, 1: query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_ltv_transfer_safe_amount: Option<i64>,
}

impl GetAccountCoinBalanceParams {
    pub fn new(account_type: AccountType, coin: String) -> Self {
        Self {
            account_type,
            coin,
            member_id: None,
            to_member_id: None,
            to_account_type: None,
            with_bonus: None,
            with_transfer_safe_amount: None,
            with_ltv_transfer_safe_amount: None,
        }
    }

    pub fn with_member_id(mut self, v: String) -> Self {
        self.member_id = Some(v);
        self
    }
    pub fn with_to_member_id(mut self, v: String) -> Self {
        self.to_member_id = Some(v);
        self
    }
    pub fn with_to_account_type(mut self, v: AccountType) -> Self {
        self.to_account_type = Some(v);
        self
    }
    pub fn with_bonus(mut self, v: i64) -> Self {
        self.with_bonus = Some(v);
        self
    }
    pub fn with_transfer_safe_amount(mut self, v: i64) -> Self {
        self.with_transfer_safe_amount = Some(v);
        self
    }
    pub fn with_ltv_transfer_safe_amount(mut self, v: i64) -> Self {
        self.with_ltv_transfer_safe_amount = Some(v);
        self
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AccountCoinBalance {
    pub account_type: AccountType,
    pub biz_type: i64,
    pub account_id: String,
    pub member_id: String,
    pub balance: CoinBalance,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CoinBalance {
    pub coin: String,
    pub wallet_balance: Decimal,
    pub transfer_balance: Decimal,
    pub bonus: Decimal,
    #[serde(default, deserialize_with = "option_decimal")]
    pub transfer_safe_amount: Option<Decimal>,
    #[serde(default, deserialize_with = "option_decimal")]
    pub ltv_transfer_safe_amount: Option<Decimal>,
}

// --- Get Asset Info (Spot) ---

/// Query params for [`Client::get_asset_info`](crate::http::Client::get_asset_info).
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetAssetInfoParams {
    /// Account type. Only `SPOT` is supported
    pub account_type: AccountType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
}

impl GetAssetInfoParams {
    pub fn new(account_type: AccountType) -> Self {
        Self {
            account_type,
            coin: None,
        }
    }

    pub fn with_coin(mut self, v: String) -> Self {
        self.coin = Some(v);
        self
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AssetInfoResult {
    pub spot: SpotAssetInfo,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SpotAssetInfo {
    /// `ACCOUNT_STATUS_NORMAL` or `ACCOUNT_STATUS_UNSPECIFIED`
    pub status: String,
    pub assets: Vec<SpotAsset>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SpotAsset {
    pub coin: String,
    /// Amount locked in active orders
    pub frozen: Decimal,
    /// Available balance
    pub free: Decimal,
    /// Amount currently being withdrawn
    pub withdraw: Decimal,
}

// ════════════════════════════════════════ Deposit ════════════════════════════════════════

// --- Get Allowed Deposit Coin Info ---

/// Query params for
/// [`Client::get_deposit_allowed_coin_info`](crate::http::Client::get_deposit_allowed_coin_info).
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDepositAllowedCoinInfoParams {
    /// Coin name, uppercase only. Must be paired with `chain` if passed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    /// Chain name. Must be paired with `coin` if passed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain: Option<String>,
    /// Limit for data size per page. [1, 35]. Default: 10
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetDepositAllowedCoinInfoParams {
    pub fn new() -> Self {
        Self {
            coin: None,
            chain: None,
            limit: None,
            cursor: None,
        }
    }

    pub fn with_coin(mut self, v: String) -> Self {
        self.coin = Some(v);
        self
    }
    pub fn with_chain(mut self, v: String) -> Self {
        self.chain = Some(v);
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

impl Default for GetDepositAllowedCoinInfoParams {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DepositAllowedCoinInfoResult {
    pub config_list: Vec<DepositCoinConfig>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub next_page_cursor: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DepositCoinConfig {
    pub coin: String,
    pub chain: String,
    pub coin_show_name: String,
    pub chain_type: String,
    pub block_confirm_number: String,
    pub min_deposit_amount: Decimal,
}

// --- Get Deposit Records / Get Sub Deposit Records ---

/// Query params for
/// [`Client::get_deposit_records`](crate::http::Client::get_deposit_records).
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDepositRecordsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Not queryable for records before 2024-01-01
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<Timestamp>,
    /// Limit for data size per page. [1, 50]. Default: 50
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetDepositRecordsParams {
    pub fn new() -> Self {
        Self {
            id: None,
            tx_id: None,
            coin: None,
            start_time: None,
            end_time: None,
            limit: None,
            cursor: None,
        }
    }

    pub fn with_id(mut self, v: String) -> Self {
        self.id = Some(v);
        self
    }
    pub fn with_tx_id(mut self, v: String) -> Self {
        self.tx_id = Some(v);
        self
    }
    pub fn with_coin(mut self, v: String) -> Self {
        self.coin = Some(v);
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
    pub fn with_limit(mut self, v: u64) -> Self {
        self.limit = Some(v);
        self
    }
    pub fn with_cursor(mut self, v: String) -> Self {
        self.cursor = Some(v);
        self
    }
}

impl Default for GetDepositRecordsParams {
    fn default() -> Self {
        Self::new()
    }
}

/// Query params for
/// [`Client::get_sub_deposit_records`](crate::http::Client::get_sub_deposit_records).
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSubDepositRecordsParams {
    pub sub_member_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<Timestamp>,
    /// Limit for data size per page. [1, 50]. Default: 50
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetSubDepositRecordsParams {
    pub fn new(sub_member_id: String) -> Self {
        Self {
            sub_member_id,
            id: None,
            tx_id: None,
            coin: None,
            start_time: None,
            end_time: None,
            limit: None,
            cursor: None,
        }
    }

    pub fn with_id(mut self, v: String) -> Self {
        self.id = Some(v);
        self
    }
    pub fn with_tx_id(mut self, v: String) -> Self {
        self.tx_id = Some(v);
        self
    }
    pub fn with_coin(mut self, v: String) -> Self {
        self.coin = Some(v);
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
    pub fn with_limit(mut self, v: u64) -> Self {
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
pub struct DepositRecords {
    pub rows: Vec<DepositRecord>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub next_page_cursor: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DepositRecord {
    pub id: String,
    pub coin: String,
    pub chain: String,
    pub amount: Decimal,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub tx_id: Option<String>,
    pub status: DepositStatus,
    pub to_address: String,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub tag: Option<String>,
    pub deposit_fee: Decimal,
    #[serde(deserialize_with = "option_number")]
    pub success_at: Option<Timestamp>,
    pub confirmations: String,
    pub tx_index: String,
    pub block_hash: String,
    /// Daily deposit ceiling. "-1" means unlimited
    pub batch_release_limit: String,
    /// 0: normal, 10: limit reached, 20: flagged by AML, 50: blocked
    pub deposit_type: i64,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub from_address: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub tax_deposit_records_id: Option<String>,
    /// 0: no tax reporting, 1: pending, 2: completed
    #[serde(default)]
    pub tax_status: Option<i64>,
    /// 0: approved, 1: info needed, 2: pending, 3: rejected
    #[serde(default)]
    pub travel_rule_status: Option<i64>,
}

// --- Get Deposit Address / Get Sub Deposit Address ---

/// Query params for
/// [`Client::get_deposit_address`](crate::http::Client::get_deposit_address).
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetDepositAddressParams {
    /// Coin name, uppercase only
    pub coin: String,
    /// Chain type, from [`Client::get_coin_info`](crate::http::Client::get_coin_info)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_type: Option<String>,
}

impl GetDepositAddressParams {
    pub fn new(coin: String) -> Self {
        Self {
            coin,
            chain_type: None,
        }
    }

    pub fn with_chain_type(mut self, v: String) -> Self {
        self.chain_type = Some(v);
        self
    }
}

/// Query params for
/// [`Client::get_sub_deposit_address`](crate::http::Client::get_sub_deposit_address).
///
/// Requires the master account API key.
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetSubDepositAddressParams {
    /// Coin name, uppercase only
    pub coin: String,
    /// Chain type, from [`Client::get_coin_info`](crate::http::Client::get_coin_info)
    pub chain_type: String,
    pub sub_member_id: String,
}

impl GetSubDepositAddressParams {
    pub fn new(coin: String, chain_type: String, sub_member_id: String) -> Self {
        Self {
            coin,
            chain_type,
            sub_member_id,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DepositAddress {
    pub coin: String,
    pub chains: Vec<DepositChain>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DepositChain {
    pub chain_type: String,
    pub address_deposit: String,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub tag_deposit: Option<String>,
    pub chain: String,
    /// Deposit limit. "-1" means unlimited
    pub batch_release_limit: String,
    /// Last 6 characters of the contract address. Empty if none
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub contract_address: Option<String>,
}

// ════════════════════════════════════════ Withdraw ════════════════════════════════════════

// --- Get Withdrawal Records ---

/// Query params for
/// [`Client::get_withdrawal_records`](crate::http::Client::get_withdrawal_records).
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWithdrawalRecordsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withdraw_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    /// 0: on chain, 1: off chain, 2: all. Default: 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withdraw_type: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<Timestamp>,
    /// Limit for data size per page. [1, 50]. Default: 50
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetWithdrawalRecordsParams {
    pub fn new() -> Self {
        Self {
            withdraw_id: None,
            tx_id: None,
            coin: None,
            withdraw_type: None,
            start_time: None,
            end_time: None,
            limit: None,
            cursor: None,
        }
    }

    pub fn with_withdraw_id(mut self, v: String) -> Self {
        self.withdraw_id = Some(v);
        self
    }
    pub fn with_tx_id(mut self, v: String) -> Self {
        self.tx_id = Some(v);
        self
    }
    pub fn with_coin(mut self, v: String) -> Self {
        self.coin = Some(v);
        self
    }
    pub fn with_withdraw_type(mut self, v: i64) -> Self {
        self.withdraw_type = Some(v);
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
    pub fn with_limit(mut self, v: u64) -> Self {
        self.limit = Some(v);
        self
    }
    pub fn with_cursor(mut self, v: String) -> Self {
        self.cursor = Some(v);
        self
    }
}

impl Default for GetWithdrawalRecordsParams {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawRecords {
    pub rows: Vec<WithdrawRecord>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub next_page_cursor: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawRecord {
    /// Empty if the withdrawal failed or was cancelled
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub tx_id: Option<String>,
    pub coin: String,
    pub chain: String,
    pub amount: Decimal,
    pub withdraw_fee: Decimal,
    pub status: WithdrawStatus,
    /// Destination address. For internal transfers, this is the receiving Bybit UID
    pub to_address: String,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub tag: Option<String>,
    #[serde(deserialize_with = "number")]
    pub create_time: Timestamp,
    #[serde(deserialize_with = "number")]
    pub update_time: Timestamp,
    pub withdraw_id: String,
    /// 0: on chain, 1: off chain
    pub withdraw_type: i64,
    #[serde(default, deserialize_with = "option_decimal")]
    pub tax: Option<Decimal>,
    #[serde(default, deserialize_with = "option_decimal")]
    pub tax_rate: Option<Decimal>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub tax_type: Option<String>,
}

// --- Withdraw ---

/// Request body for [`Client::withdraw`](crate::http::Client::withdraw).
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawRequest {
    /// Coin name, uppercase only
    pub coin: String,
    /// Destination wallet address, case-sensitive, or a Bybit UID when `force_chain=2`
    pub address: String,
    /// Withdraw amount
    pub amount: Decimal,
    /// Current timestamp (ms). Used to prevent replay attacks
    pub timestamp: i64,
    /// Wallet to withdraw from: `FUND`, `UNIFIED`/`UTA`, `EARN`, or a comma-separated
    /// combination, e.g. `"FUND,UTA,EARN"`. Not the [`AccountType`] enum: this field accepts
    /// combinations that enum can't represent
    pub account_type: String,
    /// Chain name, from [`Client::get_coin_info`](crate::http::Client::get_coin_info). Required
    /// unless `force_chain=2`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain: Option<String>,
    /// Tag/memo, required if the destination address needs one
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// 0(default): on chain if the address isn't recognized as an internal Bybit address,
    /// otherwise internal transfer. 1: force on chain. 2: force internal transfer by UID,
    /// `address` must be a Bybit UID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_chain: Option<i64>,
    /// 0(default): withdrawal fee is deducted from `amount`. 1: fee is deducted separately by
    /// the system
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_type: Option<i64>,
    /// Idempotent request ID, 1-32 alphanumeric characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

impl WithdrawRequest {
    pub fn new(
        coin: String,
        address: String,
        amount: Decimal,
        timestamp: i64,
        account_type: String,
    ) -> Self {
        Self {
            coin,
            address,
            amount,
            timestamp,
            account_type,
            chain: None,
            tag: None,
            force_chain: None,
            fee_type: None,
            request_id: None,
        }
    }

    pub fn with_chain(mut self, v: String) -> Self {
        self.chain = Some(v);
        self
    }
    pub fn with_tag(mut self, v: String) -> Self {
        self.tag = Some(v);
        self
    }
    pub fn with_force_chain(mut self, v: i64) -> Self {
        self.force_chain = Some(v);
        self
    }
    pub fn with_fee_type(mut self, v: i64) -> Self {
        self.fee_type = Some(v);
        self
    }
    pub fn with_request_id(mut self, v: String) -> Self {
        self.request_id = Some(v);
        self
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct WithdrawResult {
    pub id: String,
}

// --- Cancel Withdrawal ---

/// Request body for [`Client::cancel_withdrawal`](crate::http::Client::cancel_withdrawal).
#[derive(Debug, Serialize, Clone)]
pub struct CancelWithdrawalRequest {
    pub id: String,
}

impl CancelWithdrawalRequest {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct CancelWithdrawalResult {
    /// 0: fail, 1: success
    pub status: i64,
}

// --- Get Withdrawable Amount ---

/// Query params for
/// [`Client::get_withdrawable_amount`](crate::http::Client::get_withdrawable_amount).
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetWithdrawableAmountParams {
    /// Coin name, uppercase only
    pub coin: String,
}

impl GetWithdrawableAmountParams {
    pub fn new(coin: String) -> Self {
        Self { coin }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawableAmountResult {
    /// Amount frozen due to unresolved deposit risk, denominated in USD. "0" means all deposit
    /// risks have been released
    pub limit_amount_usd: Decimal,
    pub withdrawable_amount: WithdrawableAmountByWallet,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct WithdrawableAmountByWallet {
    /// Omitted if the Spot wallet has been removed
    #[serde(rename = "SPOT")]
    pub spot: Option<WithdrawableAmountEntry>,
    #[serde(rename = "FUND")]
    pub fund: Option<WithdrawableAmountEntry>,
    #[serde(rename = "UTA")]
    pub uta: Option<WithdrawableAmountEntry>,
    /// Omitted if the coin isn't supported by Earn
    #[serde(rename = "EARN")]
    pub earn: Option<WithdrawableAmountEntry>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawableAmountEntry {
    pub coin: String,
    pub withdrawable_amount: Decimal,
    pub available_balance: Decimal,
}

// ═══════════════════════════════════════ Coin / Misc ═══════════════════════════════════════

// --- Get Coin Info ---

/// Query params for [`Client::get_coin_info`](crate::http::Client::get_coin_info).
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetCoinInfoParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
}

impl GetCoinInfoParams {
    pub fn new() -> Self {
        Self { coin: None }
    }

    pub fn with_coin(mut self, v: String) -> Self {
        self.coin = Some(v);
        self
    }
}

impl Default for GetCoinInfoParams {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CoinInfoResult {
    pub rows: Vec<CoinInfo>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CoinInfo {
    pub name: String,
    pub coin: String,
    pub chains: Vec<CoinChain>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CoinChain {
    pub chain: String,
    pub chain_type: String,
    /// Number of block confirmations required to credit a deposit
    pub confirmation: String,
    /// Fixed withdrawal fee. Empty if withdrawal isn't supported
    #[serde(default, deserialize_with = "option_decimal")]
    pub withdraw_fee: Option<Decimal>,
    pub deposit_min: Decimal,
    pub withdraw_min: Decimal,
    /// Decimal precision for transactions on this chain
    pub min_accuracy: String,
    /// "0": suspended, "1": active
    pub chain_deposit: String,
    /// "0": suspended, "1": active
    pub chain_withdraw: String,
    pub withdraw_percentage_fee: Decimal,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub contract_address: Option<String>,
    pub safe_confirm_number: String,
    /// Maximum withdrawal per transaction. "-1" means unlimited
    pub withdraw_max: String,
}

// --- Get Coin Exchange Records ---

/// Query params for
/// [`Client::get_exchange_order_record`](crate::http::Client::get_exchange_order_record).
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetExchangeOrderRecordParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_coin: Option<String>,
    /// Limit for data size per page. [1, 50]. Default: 10
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetExchangeOrderRecordParams {
    pub fn new() -> Self {
        Self {
            from_coin: None,
            to_coin: None,
            limit: None,
            cursor: None,
        }
    }

    pub fn with_from_coin(mut self, v: String) -> Self {
        self.from_coin = Some(v);
        self
    }
    pub fn with_to_coin(mut self, v: String) -> Self {
        self.to_coin = Some(v);
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

impl Default for GetExchangeOrderRecordParams {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeOrderRecords {
    pub order_body: Vec<ExchangeOrderRecord>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub next_page_cursor: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeOrderRecord {
    pub from_coin: String,
    pub from_amount: Decimal,
    pub to_coin: String,
    pub to_amount: Decimal,
    pub exchange_rate: Decimal,
    /// Timestamp in seconds
    #[serde(deserialize_with = "number")]
    pub created_time: Timestamp,
    pub exchange_tx_id: String,
}

// --- Get Delivery Record ---

/// Query params for
/// [`Client::get_delivery_record`](crate::http::Client::get_delivery_record).
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDeliveryRecordParams {
    /// linear, inverse, option
    pub category: Category,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Without start_time and end_time, returns the past 30 days by default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<Timestamp>,
    /// Expiry date, e.g. "25MAR22". Returns all if not specified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_date: Option<String>,
    /// Limit for data size per page. [1, 50]. Default: 20
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetDeliveryRecordParams {
    pub fn new(category: Category) -> Self {
        Self {
            category,
            symbol: None,
            start_time: None,
            end_time: None,
            exp_date: None,
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
    pub fn with_exp_date(mut self, v: String) -> Self {
        self.exp_date = Some(v);
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

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryRecord {
    pub delivery_time: Timestamp,
    pub symbol: String,
    pub side: Side,
    pub position: Decimal,
    pub entry_price: Decimal,
    pub delivery_price: Decimal,
    /// Options only
    #[serde(default, deserialize_with = "option_decimal")]
    pub strike: Option<Decimal>,
    pub fee: Decimal,
    pub delivery_rpl: Decimal,
}

// --- Get USDC Session Settlement ---

/// Query params for
/// [`Client::get_settlement_record`](crate::http::Client::get_settlement_record).
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSettlementRecordParams {
    /// Must be `linear` (USDC contracts)
    pub category: Category,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Without start_time and end_time, returns the past 30 days by default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<Timestamp>,
    /// Limit for data size per page. [1, 50]. Default: 20
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetSettlementRecordParams {
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
    pub fn with_limit(mut self, v: u64) -> Self {
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
pub struct SettlementRecord {
    pub symbol: String,
    pub side: Side,
    pub size: Decimal,
    pub session_avg_price: Decimal,
    pub mark_price: Decimal,
    pub realised_pnl: Decimal,
    #[serde(deserialize_with = "number")]
    pub created_time: Timestamp,
}

// --- Get Coin Greeks ---

/// Query params for [`Client::get_coin_greeks`](crate::http::Client::get_coin_greeks).
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetCoinGreeksParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
}

impl GetCoinGreeksParams {
    pub fn new() -> Self {
        Self { base_coin: None }
    }

    pub fn with_base_coin(mut self, v: String) -> Self {
        self.base_coin = Some(v);
        self
    }
}

impl Default for GetCoinGreeksParams {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CoinGreeks {
    pub base_coin: String,
    pub total_delta: Decimal,
    pub total_gamma: Decimal,
    pub total_vega: Decimal,
    pub total_theta: Decimal,
}
