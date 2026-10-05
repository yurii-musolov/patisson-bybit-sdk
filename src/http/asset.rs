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
    /// From account type.
    pub from_account_type: AccountType,
    /// To account type.
    pub to_account_type: AccountType,
}

impl InternalTransferRequest {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new(
        transfer_id: impl Into<String>,
        coin: impl Into<String>,
        amount: Decimal,
        from_account_type: AccountType,
        to_account_type: AccountType,
    ) -> Self {
        let transfer_id: String = transfer_id.into();
        let coin: String = coin.into();
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
    /// UUID.
    pub transfer_id: String,
    /// Transfer status STATUS_UNKNOWN SUCCESS PENDING FAILED.
    pub status: TransferStatus,
}

// --- Get Internal Transfer Records ---

/// Query params for
/// [`Client::get_internal_transfer_records`](crate::http::Client::get_internal_transfer_records).
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInternalTransferRecordsParams {
    /// UUID. Use the one you generated in createTransfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_id: Option<String>,
    /// Coin, uppercase only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    /// Transfer status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TransferStatus>,
    /// Without start_time and end_time, returns the past 7 days by default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Timestamp>,
    /// The end timestamp (ms) Note: the query logic is actually effective based on second level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<Timestamp>,
    /// Limit for data size per page. [1, 50]. Default: 20
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    /// Cursor. Use the nextPageCursor token from the response to retrieve the next page of the result set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetInternalTransferRecordsParams {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
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

    /// Set `transfer_id`: uUID. Use the one you generated in createTransfer.
    pub fn with_transfer_id(mut self, v: impl Into<String>) -> Self {
        self.transfer_id = Some(v.into());
        self
    }
    /// Set `coin`: coin, uppercase only.
    pub fn with_coin(mut self, v: impl Into<String>) -> Self {
        self.coin = Some(v.into());
        self
    }
    /// Set `status`: transfer status.
    pub fn with_status(mut self, v: TransferStatus) -> Self {
        self.status = Some(v);
        self
    }
    /// Set `start_time`: without start_time and end_time, returns the past 7 days by default.
    pub fn with_start_time(mut self, v: Timestamp) -> Self {
        self.start_time = Some(v);
        self
    }
    /// Set `end_time`: the end timestamp (ms) Note: the query logic is actually effective based on second level.
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

impl Default for GetInternalTransferRecordsParams {
    fn default() -> Self {
        Self::new()
    }
}

/// Item of the list returned by `GET /v5/asset/transfer/query-inter-transfer-list` ([`Client::get_internal_transfer_records`](crate::http::Client::get_internal_transfer_records)).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InternalTransferEntry {
    /// Transfer ID.
    pub transfer_id: String,
    /// Transferred coin.
    pub coin: String,
    /// Transferred amount.
    pub amount: Decimal,
    /// From account type.
    pub from_account_type: AccountType,
    /// To account type.
    pub to_account_type: AccountType,
    /// Transfer created timestamp (ms).
    #[serde(deserialize_with = "number")]
    pub timestamp: Timestamp,
    /// Transfer status.
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
    /// From account type.
    pub from_account_type: AccountType,
    /// To account type.
    pub to_account_type: AccountType,
}

impl UniversalTransferRequest {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new(
        transfer_id: impl Into<String>,
        coin: impl Into<String>,
        amount: Decimal,
        from_member_id: i64,
        to_member_id: i64,
        from_account_type: AccountType,
        to_account_type: AccountType,
    ) -> Self {
        let transfer_id: String = transfer_id.into();
        let coin: String = coin.into();
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
    /// UUID. Use the one you generated in createTransfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_id: Option<String>,
    /// Coin, uppercase only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    /// Transfer status. SUCCESS, FAILED, PENDING.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TransferStatus>,
    /// Without start_time and end_time, returns the past 7 days by default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Timestamp>,
    /// The end timestamp (ms) Note: the query logic is actually effective based on second level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<Timestamp>,
    /// Limit for data size per page. [1, 50]. Default: 20
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    /// Cursor. Use the nextPageCursor token from the response to retrieve the next page of the result set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetUniversalTransferRecordsParams {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
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

    /// Set `transfer_id`: uUID. Use the one you generated in createTransfer.
    pub fn with_transfer_id(mut self, v: impl Into<String>) -> Self {
        self.transfer_id = Some(v.into());
        self
    }
    /// Set `coin`: coin, uppercase only.
    pub fn with_coin(mut self, v: impl Into<String>) -> Self {
        self.coin = Some(v.into());
        self
    }
    /// Set `status`: transfer status. SUCCESS, FAILED, PENDING.
    pub fn with_status(mut self, v: TransferStatus) -> Self {
        self.status = Some(v);
        self
    }
    /// Set `start_time`: without start_time and end_time, returns the past 7 days by default.
    pub fn with_start_time(mut self, v: Timestamp) -> Self {
        self.start_time = Some(v);
        self
    }
    /// Set `end_time`: the end timestamp (ms) Note: the query logic is actually effective based on second level.
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

impl Default for GetUniversalTransferRecordsParams {
    fn default() -> Self {
        Self::new()
    }
}

/// Item of the list returned by `GET /v5/asset/transfer/query-universal-transfer-list` ([`Client::get_universal_transfer_records`](crate::http::Client::get_universal_transfer_records)).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UniversalTransferEntry {
    /// Transfer ID.
    pub transfer_id: String,
    /// Transferred coin.
    pub coin: String,
    /// Transferred amount.
    pub amount: Decimal,
    /// From UID.
    pub from_member_id: String,
    /// To UID.
    pub to_member_id: String,
    /// From account type.
    pub from_account_type: AccountType,
    /// To account type.
    pub to_account_type: AccountType,
    /// Transfer created timestamp (ms).
    #[serde(deserialize_with = "number")]
    pub timestamp: Timestamp,
    /// Transfer status.
    pub status: TransferStatus,
}

// --- Get Transferable Coin ---

/// Query params for
/// [`Client::get_transferable_coins`](crate::http::Client::get_transferable_coins).
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetTransferableCoinsParams {
    /// From account type.
    pub from_account_type: AccountType,
    /// To account type.
    pub to_account_type: AccountType,
}

impl GetTransferableCoinsParams {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
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
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new(sub_member_ids: impl Into<String>) -> Self {
        let sub_member_ids: String = sub_member_ids.into();
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
    /// Account type.
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
    /// To account type. Required when querying the transferable balance between different account types.
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
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new(account_type: AccountType, coin: impl Into<String>) -> Self {
        let coin: String = coin.into();
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

    /// Set `member_id`: master UID's api key can query itself or its sub UIDs' coin balance. If not passed, then by default it queries master UID's coin balance.
    pub fn with_member_id(mut self, v: impl Into<String>) -> Self {
        self.member_id = Some(v.into());
        self
    }
    /// Set `to_member_id`: used together with `to_account_type` to query the balance safe to transfer between two UIDs' wallets.
    pub fn with_to_member_id(mut self, v: impl Into<String>) -> Self {
        self.to_member_id = Some(v.into());
        self
    }
    /// Set `to_account_type`: to account type. Required when querying the transferable balance between different account types.
    pub fn with_to_account_type(mut self, v: AccountType) -> Self {
        self.to_account_type = Some(v);
        self
    }
    /// Set `withBonus`: 1 includes the bonus in the balance.
    pub fn with_bonus(mut self, v: i64) -> Self {
        self.with_bonus = Some(v);
        self
    }
    /// Set `withTransferSafeAmount`: 1 also returns the amount that can be transferred safely.
    pub fn with_transfer_safe_amount(mut self, v: i64) -> Self {
        self.with_transfer_safe_amount = Some(v);
        self
    }
    /// Set `withLtvTransferSafeAmount`: 1 also returns the safely transferable amount of an institutional loan account.
    pub fn with_ltv_transfer_safe_amount(mut self, v: i64) -> Self {
        self.with_ltv_transfer_safe_amount = Some(v);
        self
    }
}

/// Result of `GET /v5/asset/transfer/query-account-coin-balance` ([`Client::get_account_coin_balance`](crate::http::Client::get_account_coin_balance)).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AccountCoinBalance {
    /// Account type.
    pub account_type: AccountType,
    /// Biz type.
    pub biz_type: i64,
    /// Account ID.
    pub account_id: String,
    /// Uid.
    pub member_id: String,
    /// Total asset (margin coins converted to USDT). When liqStatus != 0, empty string is returned.
    pub balance: CoinBalance,
}

/// Part of the response of `/v5/asset/transfer/query-account-coin-balance`.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CoinBalance {
    /// Coin.
    pub coin: String,
    /// Wallet balance.
    pub wallet_balance: Decimal,
    /// Transferable balance.
    pub transfer_balance: Decimal,
    /// Bonus.
    pub bonus: Decimal,
    /// Safe amount to transfer. Keep "" if not query.
    #[serde(default, deserialize_with = "option_decimal")]
    pub transfer_safe_amount: Option<Decimal>,
    /// Transferable amount for ins loan account. Keep "" if not query.
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
    /// Coin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
}

impl GetAssetInfoParams {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new(account_type: AccountType) -> Self {
        Self {
            account_type,
            coin: None,
        }
    }

    /// Set `coin`: coin.
    pub fn with_coin(mut self, v: impl Into<String>) -> Self {
        self.coin = Some(v.into());
        self
    }
}

/// Result of `GET /v5/asset/transfer/query-asset-info` ([`Client::get_asset_info`](crate::http::Client::get_asset_info)).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AssetInfoResult {
    /// Object. Earning for Spot trading. If do not have any rebate, keep empty array.
    pub spot: SpotAssetInfo,
}

/// Part of the response of `/v5/asset/transfer/query-asset-info`.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SpotAssetInfo {
    /// `ACCOUNT_STATUS_NORMAL` or `ACCOUNT_STATUS_UNSPECIFIED`
    pub status: String,
    /// Assets by coin.
    pub assets: Vec<SpotAsset>,
}

/// Part of the response of `/v5/asset/transfer/query-asset-info`.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SpotAsset {
    /// Coin.
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
    /// Cursor. Use the nextPageCursor token from the response to retrieve the next page of the result set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetDepositAllowedCoinInfoParams {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new() -> Self {
        Self {
            coin: None,
            chain: None,
            limit: None,
            cursor: None,
        }
    }

    /// Set `coin`: coin name, uppercase only. Must be paired with `chain` if passed.
    pub fn with_coin(mut self, v: impl Into<String>) -> Self {
        self.coin = Some(v.into());
        self
    }
    /// Set `chain`: chain name. Must be paired with `coin` if passed.
    pub fn with_chain(mut self, v: impl Into<String>) -> Self {
        self.chain = Some(v.into());
        self
    }
    /// Set `limit`: limit for data size per page. [1, 35]. Default: 10.
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

impl Default for GetDepositAllowedCoinInfoParams {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of `GET /v5/asset/deposit/query-allowed-list` ([`Client::get_deposit_allowed_coin_info`](crate::http::Client::get_deposit_allowed_coin_info)).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DepositAllowedCoinInfoResult {
    /// Coins and chains that accept deposits.
    pub config_list: Vec<DepositCoinConfig>,
    /// Refer to the cursor request parameter.
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub next_page_cursor: Option<String>,
}

/// Part of the response of `/v5/asset/deposit/query-allowed-list`.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DepositCoinConfig {
    /// Coin.
    pub coin: String,
    /// Chain.
    pub chain: String,
    /// Display name of the coin.
    pub coin_show_name: String,
    /// Chain type.
    pub chain_type: String,
    /// Number of block confirmations for the deposit to be credited.
    #[serde(deserialize_with = "number")]
    pub block_confirm_number: u32,
    /// Minimum deposit amount.
    pub min_deposit_amount: Decimal,
}

// --- Get Deposit Records / Get Sub Deposit Records ---

/// Query params for
/// [`Client::get_deposit_records`](crate::http::Client::get_deposit_records).
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDepositRecordsParams {
    /// Internal ID: Can be used to uniquely identify and filter the deposit. When combined with other parameters, this field takes the highest priority.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Not queryable for records before 2024-01-01
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<String>,
    /// Coin, uppercase only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    /// The start timestamp (ms) Note: the query logic is actually effective based on second level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Timestamp>,
    /// The end timestamp (ms) Note: the query logic is actually effective based on second level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<Timestamp>,
    /// Limit for data size per page. [1, 50]. Default: 50
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    /// Cursor. Use the nextPageCursor token from the response to retrieve the next page of the result set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetDepositRecordsParams {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
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

    /// Set `id`: internal ID: Can be used to uniquely identify and filter the deposit. When combined with other parameters, this field takes the highest priority.
    pub fn with_id(mut self, v: impl Into<String>) -> Self {
        self.id = Some(v.into());
        self
    }
    /// Set `tx_id`: not queryable for records before 2024-01-01.
    pub fn with_tx_id(mut self, v: impl Into<String>) -> Self {
        self.tx_id = Some(v.into());
        self
    }
    /// Set `coin`: coin, uppercase only.
    pub fn with_coin(mut self, v: impl Into<String>) -> Self {
        self.coin = Some(v.into());
        self
    }
    /// Set `start_time`: the start timestamp (ms) Note: the query logic is actually effective based on second level.
    pub fn with_start_time(mut self, v: Timestamp) -> Self {
        self.start_time = Some(v);
        self
    }
    /// Set `end_time`: the end timestamp (ms) Note: the query logic is actually effective based on second level.
    pub fn with_end_time(mut self, v: Timestamp) -> Self {
        self.end_time = Some(v);
        self
    }
    /// Set `limit`: limit for data size per page. [1, 50]. Default: 50.
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
    /// Sub UID.
    pub sub_member_id: String,
    /// Internal ID: Can be used to uniquely identify and filter the deposit. When combined with other parameters, this field takes the highest priority.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Transaction id (hash) on the chain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<String>,
    /// Coin, uppercase only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    /// The start timestamp (ms) Note: the query logic is actually effective based on second level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Timestamp>,
    /// The end timestamp (ms) Note: the query logic is actually effective based on second level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<Timestamp>,
    /// Limit for data size per page. [1, 50]. Default: 50
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    /// Cursor. Use the nextPageCursor token from the response to retrieve the next page of the result set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetSubDepositRecordsParams {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new(sub_member_id: impl Into<String>) -> Self {
        let sub_member_id: String = sub_member_id.into();
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

    /// Set `id`: internal ID: Can be used to uniquely identify and filter the deposit. When combined with other parameters, this field takes the highest priority.
    pub fn with_id(mut self, v: impl Into<String>) -> Self {
        self.id = Some(v.into());
        self
    }
    /// Set `tx_id`: transaction id (hash) on the chain.
    pub fn with_tx_id(mut self, v: impl Into<String>) -> Self {
        self.tx_id = Some(v.into());
        self
    }
    /// Set `coin`: coin, uppercase only.
    pub fn with_coin(mut self, v: impl Into<String>) -> Self {
        self.coin = Some(v.into());
        self
    }
    /// Set `start_time`: the start timestamp (ms) Note: the query logic is actually effective based on second level.
    pub fn with_start_time(mut self, v: Timestamp) -> Self {
        self.start_time = Some(v);
        self
    }
    /// Set `end_time`: the end timestamp (ms) Note: the query logic is actually effective based on second level.
    pub fn with_end_time(mut self, v: Timestamp) -> Self {
        self.end_time = Some(v);
        self
    }
    /// Set `limit`: limit for data size per page. [1, 50]. Default: 50.
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

/// Result of `GET /v5/asset/deposit/query-record` ([`Client::get_deposit_records`](crate::http::Client::get_deposit_records), [`Client::get_sub_deposit_records`](crate::http::Client::get_sub_deposit_records)).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DepositRecords {
    /// Items.
    pub rows: Vec<DepositRecord>,
    /// Refer to the cursor request parameter.
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub next_page_cursor: Option<String>,
}

/// Part of the response of `/v5/asset/deposit/query-record`.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DepositRecord {
    /// Unique ID.
    pub id: String,
    /// Coin.
    pub coin: String,
    /// Chain.
    pub chain: String,
    /// Amount.
    pub amount: Decimal,
    /// Transaction id (hash) on the chain.
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub tx_id: Option<String>,
    /// Deposit status.
    pub status: DepositStatus,
    /// Deposit target address.
    pub to_address: String,
    /// Tag of deposit target address.
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub tag: Option<String>,
    /// Deposit fee.
    pub deposit_fee: Decimal,
    /// Deposit's success time.
    #[serde(deserialize_with = "option_number")]
    pub success_at: Option<Timestamp>,
    /// Number of confirmation blocks.
    pub confirmations: String,
    /// Transaction sequence number.
    pub tx_index: String,
    /// Hash number on the chain.
    pub block_hash: String,
    /// Daily deposit ceiling. "-1" means unlimited
    pub batch_release_limit: String,
    /// 0: normal, 10: limit reached, 20: flagged by AML, 50: blocked
    pub deposit_type: i64,
    /// From address of deposit, only shown when the deposit comes from on-chain and from address is unique, otherwise gives "".
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub from_address: Option<String>,
    /// This field is used for tax purposes by Bybit EU (Austria) users, declare tax id.
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
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new(coin: impl Into<String>) -> Self {
        let coin: String = coin.into();
        Self {
            coin,
            chain_type: None,
        }
    }

    /// Set `chain_type`: chain type, from [`Client::get_coin_info`](crate::http::Client::get_coin_info).
    pub fn with_chain_type(mut self, v: impl Into<String>) -> Self {
        self.chain_type = Some(v.into());
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
    /// Sub user ID.
    pub sub_member_id: String,
}

impl GetSubDepositAddressParams {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new(
        coin: impl Into<String>,
        chain_type: impl Into<String>,
        sub_member_id: impl Into<String>,
    ) -> Self {
        let coin: String = coin.into();
        let chain_type: String = chain_type.into();
        let sub_member_id: String = sub_member_id.into();
        Self {
            coin,
            chain_type,
            sub_member_id,
        }
    }
}

/// Result of `GET /v5/asset/deposit/query-address` ([`Client::get_deposit_address`](crate::http::Client::get_deposit_address), [`Client::get_sub_deposit_address`](crate::http::Client::get_sub_deposit_address)).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DepositAddress {
    /// Coin.
    pub coin: String,
    /// Deposit addresses by chain.
    pub chains: Vec<DepositChain>,
}

/// Part of the response of `/v5/asset/deposit/query-address`.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DepositChain {
    /// Chain type.
    pub chain_type: String,
    /// The address for deposit.
    pub address_deposit: String,
    /// Tag of deposit.
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub tag_deposit: Option<String>,
    /// Chain.
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
    /// Withdraw ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withdraw_id: Option<String>,
    /// Transaction id (hash) on the chain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<String>,
    /// Coin, uppercase only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    /// 0: on chain, 1: off chain, 2: all. Default: 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withdraw_type: Option<i64>,
    /// The start timestamp (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Timestamp>,
    /// The end timestamp (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<Timestamp>,
    /// Limit for data size per page. [1, 50]. Default: 50
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    /// Cursor. Use the nextPageCursor token from the response to retrieve the next page of the result set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetWithdrawalRecordsParams {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
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

    /// Set `withdraw_id`: withdraw ID.
    pub fn with_withdraw_id(mut self, v: impl Into<String>) -> Self {
        self.withdraw_id = Some(v.into());
        self
    }
    /// Set `tx_id`: transaction id (hash) on the chain.
    pub fn with_tx_id(mut self, v: impl Into<String>) -> Self {
        self.tx_id = Some(v.into());
        self
    }
    /// Set `coin`: coin, uppercase only.
    pub fn with_coin(mut self, v: impl Into<String>) -> Self {
        self.coin = Some(v.into());
        self
    }
    /// Set `withdraw_type`: 0: on chain, 1: off chain, 2: all. Default: 0.
    pub fn with_withdraw_type(mut self, v: i64) -> Self {
        self.withdraw_type = Some(v);
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
    /// Set `limit`: limit for data size per page. [1, 50]. Default: 50.
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

impl Default for GetWithdrawalRecordsParams {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of `GET /v5/asset/withdraw/query-record` ([`Client::get_withdrawal_records`](crate::http::Client::get_withdrawal_records)).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawRecords {
    /// Items.
    pub rows: Vec<WithdrawRecord>,
    /// Cursor. Used for pagination.
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub next_page_cursor: Option<String>,
}

/// Part of the response of `/v5/asset/withdraw/query-record`.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawRecord {
    /// Empty if the withdrawal failed or was cancelled
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub tx_id: Option<String>,
    /// Coin.
    pub coin: String,
    /// Chain.
    pub chain: String,
    /// Amount.
    pub amount: Decimal,
    /// Withdraw fee.
    pub withdraw_fee: Decimal,
    /// Withdraw status.
    pub status: WithdrawStatus,
    /// Destination address. For internal transfers, this is the receiving Bybit UID
    pub to_address: String,
    /// Tag.
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub tag: Option<String>,
    /// Withdraw created timestamp (ms).
    #[serde(deserialize_with = "number")]
    pub create_time: Timestamp,
    /// Withdraw updated timestamp (ms).
    #[serde(deserialize_with = "number")]
    pub update_time: Timestamp,
    /// Withdraw ID.
    pub withdraw_id: String,
    /// 0: on chain, 1: off chain
    pub withdraw_type: i64,
    /// Tax amount. Applicable to specific users based on Tax Centre configuration. Default: "0".
    #[serde(default, deserialize_with = "option_decimal")]
    pub tax: Option<Decimal>,
    /// Tax rate. Applicable to specific users based on Tax Centre configuration. Default: "0".
    #[serde(default, deserialize_with = "option_decimal")]
    pub tax_rate: Option<Decimal>,
    /// Tax type. Applicable to specific users based on Tax Centre configuration. Default: "".
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
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new(
        coin: impl Into<String>,
        address: impl Into<String>,
        amount: Decimal,
        timestamp: i64,
        account_type: impl Into<String>,
    ) -> Self {
        let coin: String = coin.into();
        let address: String = address.into();
        let account_type: String = account_type.into();
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

    /// Set `chain`: chain name, from [`Client::get_coin_info`](crate::http::Client::get_coin_info). Required unless `force_chain=2`.
    pub fn with_chain(mut self, v: impl Into<String>) -> Self {
        self.chain = Some(v.into());
        self
    }
    /// Set `tag`: tag/memo, required if the destination address needs one.
    pub fn with_tag(mut self, v: impl Into<String>) -> Self {
        self.tag = Some(v.into());
        self
    }
    /// Set `force_chain`: 0(default): on chain if the address isn't recognized as an internal Bybit address, otherwise internal transfer. 1: force on chain. 2: force internal transfer by UID, `address` must be a Bybit UID.
    pub fn with_force_chain(mut self, v: i64) -> Self {
        self.force_chain = Some(v);
        self
    }
    /// Set `fee_type`: 0(default): withdrawal fee is deducted from `amount`. 1: fee is deducted separately by the system.
    pub fn with_fee_type(mut self, v: i64) -> Self {
        self.fee_type = Some(v);
        self
    }
    /// Set `request_id`: idempotent request ID, 1-32 alphanumeric characters.
    pub fn with_request_id(mut self, v: impl Into<String>) -> Self {
        self.request_id = Some(v.into());
        self
    }
}

/// Result of `POST /v5/asset/withdraw/create` ([`Client::withdraw`](crate::http::Client::withdraw)).
#[derive(Debug, Deserialize, PartialEq)]
pub struct WithdrawResult {
    /// Withdrawal ID.
    pub id: String,
}

// --- Cancel Withdrawal ---

/// Request body for [`Client::cancel_withdrawal`](crate::http::Client::cancel_withdrawal).
#[derive(Debug, Serialize, Clone)]
pub struct CancelWithdrawalRequest {
    /// Withdrawal ID.
    pub id: String,
}

impl CancelWithdrawalRequest {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new(id: impl Into<String>) -> Self {
        let id: String = id.into();
        Self { id }
    }
}

/// Result of `POST /v5/asset/withdraw/cancel` ([`Client::cancel_withdrawal`](crate::http::Client::cancel_withdrawal)).
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
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new(coin: impl Into<String>) -> Self {
        let coin: String = coin.into();
        Self { coin }
    }
}

/// Result of `GET /v5/asset/withdraw/withdrawable-amount` ([`Client::get_withdrawable_amount`](crate::http::Client::get_withdrawable_amount)).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawableAmountResult {
    /// Amount frozen due to unresolved deposit risk, denominated in USD. "0" means all deposit
    /// risks have been released
    pub limit_amount_usd: Decimal,
    /// Amount that can be withdrawn.
    pub withdrawable_amount: WithdrawableAmountByWallet,
}

/// Part of the response of `/v5/asset/withdraw/withdrawable-amount`.
#[derive(Debug, Deserialize, PartialEq)]
pub struct WithdrawableAmountByWallet {
    /// Omitted if the Spot wallet has been removed
    #[serde(rename = "SPOT")]
    pub spot: Option<WithdrawableAmountEntry>,
    /// Funding wallet.
    #[serde(rename = "FUND")]
    pub fund: Option<WithdrawableAmountEntry>,
    /// Unified wallet.
    #[serde(rename = "UTA")]
    pub uta: Option<WithdrawableAmountEntry>,
    /// Omitted if the coin isn't supported by Earn
    #[serde(rename = "EARN")]
    pub earn: Option<WithdrawableAmountEntry>,
}

/// Part of the response of `/v5/asset/withdraw/withdrawable-amount`.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawableAmountEntry {
    /// Coin name.
    pub coin: String,
    /// Amount that can be withdrawn.
    pub withdrawable_amount: Decimal,
    /// Available balance.
    pub available_balance: Decimal,
}

// ═══════════════════════════════════════ Coin / Misc ═══════════════════════════════════════

// --- Get Coin Info ---

/// Query params for [`Client::get_coin_info`](crate::http::Client::get_coin_info).
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetCoinInfoParams {
    /// Coin, uppercase only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
}

impl GetCoinInfoParams {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new() -> Self {
        Self { coin: None }
    }

    /// Set `coin`: coin, uppercase only.
    pub fn with_coin(mut self, v: impl Into<String>) -> Self {
        self.coin = Some(v.into());
        self
    }
}

impl Default for GetCoinInfoParams {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of `GET /v5/asset/coin/query-info` ([`Client::get_coin_info`](crate::http::Client::get_coin_info)).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CoinInfoResult {
    /// Items.
    pub rows: Vec<CoinInfo>,
}

/// Part of the response of `/v5/asset/coin/query-info`.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CoinInfo {
    /// Coin name.
    pub name: String,
    /// Coin.
    pub coin: String,
    /// Chains of the coin with their deposit and withdrawal settings.
    pub chains: Vec<CoinChain>,
}

/// Part of the response of `/v5/asset/coin/query-info`.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CoinChain {
    /// Chain.
    pub chain: String,
    /// Chain type.
    pub chain_type: String,
    /// Number of block confirmations required to credit a deposit
    pub confirmation: String,
    /// Fixed withdrawal fee. Empty if withdrawal isn't supported
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub withdraw_fee: Option<Decimal>,
    /// Min. deposit.
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub deposit_min: Option<Decimal>,
    /// Min. withdraw.
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub withdraw_min: Option<Decimal>,
    /// Decimal precision for transactions on this chain
    pub min_accuracy: String,
    /// "0": suspended, "1": active
    pub chain_deposit: String,
    /// "0": suspended, "1": active
    pub chain_withdraw: String,
    /// The withdraw fee percentage. It is a real figure, e.g., 0.022 means 2.2%.
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub withdraw_percentage_fee: Option<Decimal>,
    /// Contract address. "" means no contract address.
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub contract_address: Option<String>,
    /// Number of security confirmations: Once this number is reached, your USD equivalent worth funds will be fully unlocked and available for withdrawal.
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
    /// The currency to convert from, uppercase only. e.g, BTC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_coin: Option<String>,
    /// The currency to convert to, uppercase only. e.g, USDT.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_coin: Option<String>,
    /// Limit for data size per page. [1, 50]. Default: 10
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    /// Cursor. Use the nextPageCursor token from the response to retrieve the next page of the result set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetExchangeOrderRecordParams {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new() -> Self {
        Self {
            from_coin: None,
            to_coin: None,
            limit: None,
            cursor: None,
        }
    }

    /// Set `from_coin`: the currency to convert from, uppercase only. e.g, BTC.
    pub fn with_from_coin(mut self, v: impl Into<String>) -> Self {
        self.from_coin = Some(v.into());
        self
    }
    /// Set `to_coin`: the currency to convert to, uppercase only. e.g, USDT.
    pub fn with_to_coin(mut self, v: impl Into<String>) -> Self {
        self.to_coin = Some(v.into());
        self
    }
    /// Set `limit`: limit for data size per page. [1, 50]. Default: 10.
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

impl Default for GetExchangeOrderRecordParams {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of `GET /v5/asset/exchange/order-record` ([`Client::get_exchange_order_record`](crate::http::Client::get_exchange_order_record)).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeOrderRecords {
    /// Convert records.
    pub order_body: Vec<ExchangeOrderRecord>,
    /// Refer to the cursor request parameter.
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub next_page_cursor: Option<String>,
}

/// Part of the response of `/v5/asset/exchange/order-record`.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeOrderRecord {
    /// The currency to convert from.
    pub from_coin: String,
    /// The amount to convert from.
    pub from_amount: Decimal,
    /// The currency to convert to.
    pub to_coin: String,
    /// The amount to convert to.
    pub to_amount: Decimal,
    /// Exchange rate.
    pub exchange_rate: Decimal,
    /// Timestamp in seconds
    #[serde(deserialize_with = "number")]
    pub created_time: Timestamp,
    /// Exchange transaction ID.
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
    /// Symbol name, like BTCUSDT, uppercase only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Without start_time and end_time, returns the past 30 days by default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Timestamp>,
    /// The end timestamp (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<Timestamp>,
    /// Expiry date, e.g. "25MAR22". Returns all if not specified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_date: Option<String>,
    /// Limit for data size per page. [1, 50]. Default: 20
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    /// Cursor. Use the nextPageCursor token from the response to retrieve the next page of the result set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetDeliveryRecordParams {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
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

    /// Set `symbol`: symbol name, like BTCUSDT, uppercase only.
    pub fn with_symbol(mut self, v: impl Into<String>) -> Self {
        self.symbol = Some(v.into());
        self
    }
    /// Set `start_time`: without start_time and end_time, returns the past 30 days by default.
    pub fn with_start_time(mut self, v: Timestamp) -> Self {
        self.start_time = Some(v);
        self
    }
    /// Set `end_time`: the end timestamp (ms).
    pub fn with_end_time(mut self, v: Timestamp) -> Self {
        self.end_time = Some(v);
        self
    }
    /// Set `exp_date`: expiry date, e.g. "25MAR22". Returns all if not specified.
    pub fn with_exp_date(mut self, v: impl Into<String>) -> Self {
        self.exp_date = Some(v.into());
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

/// Item of the list returned by `GET /v5/asset/delivery-record` ([`Client::get_delivery_record`](crate::http::Client::get_delivery_record)).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryRecord {
    /// Delivery time (ms).
    pub delivery_time: Timestamp,
    /// Symbol name.
    pub symbol: String,
    /// Buy, Sell.
    pub side: Side,
    /// Executed size.
    pub position: Decimal,
    /// Avg entry price.
    pub entry_price: Decimal,
    /// Delivery price.
    pub delivery_price: Decimal,
    /// Options only
    #[serde(default, deserialize_with = "option_decimal")]
    pub strike: Option<Decimal>,
    /// Trading fee.
    pub fee: Decimal,
    /// Realized PnL of the delivery.
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
    /// Symbol name, like BTCPERP, uppercase only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Without start_time and end_time, returns the past 30 days by default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Timestamp>,
    /// The end time. timestamp (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<Timestamp>,
    /// Limit for data size per page. [1, 50]. Default: 20
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    /// Cursor. Use the nextPageCursor token from the response to retrieve the next page of the result set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetSettlementRecordParams {
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

    /// Set `symbol`: symbol name, like BTCPERP, uppercase only.
    pub fn with_symbol(mut self, v: impl Into<String>) -> Self {
        self.symbol = Some(v.into());
        self
    }
    /// Set `start_time`: without start_time and end_time, returns the past 30 days by default.
    pub fn with_start_time(mut self, v: Timestamp) -> Self {
        self.start_time = Some(v);
        self
    }
    /// Set `end_time`: the end time. timestamp (ms).
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

/// Item of the list returned by `GET /v5/asset/settlement-record` ([`Client::get_settlement_record`](crate::http::Client::get_settlement_record)).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SettlementRecord {
    /// Symbol name.
    pub symbol: String,
    /// Buy, Sell.
    pub side: Side,
    /// Position size.
    pub size: Decimal,
    /// Settlement price.
    pub session_avg_price: Decimal,
    /// Mark price.
    pub mark_price: Decimal,
    /// Realised PnL.
    pub realised_pnl: Decimal,
    /// Created time (ms).
    #[serde(deserialize_with = "number")]
    pub created_time: Timestamp,
}

// --- Get Coin Greeks ---

/// Query params for [`Client::get_coin_greeks`](crate::http::Client::get_coin_greeks).
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetCoinGreeksParams {
    /// Base coin, uppercase only. If not passed, all supported base coin greeks will be returned by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
}

impl GetCoinGreeksParams {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new() -> Self {
        Self { base_coin: None }
    }

    /// Set `base_coin`: base coin, uppercase only. If not passed, all supported base coin greeks will be returned by default.
    pub fn with_base_coin(mut self, v: impl Into<String>) -> Self {
        self.base_coin = Some(v.into());
        self
    }
}

impl Default for GetCoinGreeksParams {
    fn default() -> Self {
        Self::new()
    }
}

/// Item of the list returned by `GET /v5/asset/coin-greeks` ([`Client::get_coin_greeks`](crate::http::Client::get_coin_greeks)).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CoinGreeks {
    /// Base coin. e.g., BTC, ETH, SOL.
    pub base_coin: String,
    /// Delta value.
    pub total_delta: Decimal,
    /// Gamma value.
    pub total_gamma: Decimal,
    /// Vega value.
    pub total_vega: Decimal,
    /// Theta value.
    pub total_theta: Decimal,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coin_chain_with_empty_limits() {
        // Chains that do not support deposits/withdrawals send "" limits.
        let json = r#"{"rows":[{"name":"USDT","coin":"USDT","remainAmount":"100","chains":[{
            "chainType":"X","confirmation":"","withdrawFee":"","depositMin":"","withdrawMin":"",
            "chain":"X","chainDeposit":"0","chainWithdraw":"0","minAccuracy":"8",
            "withdrawPercentageFee":"","contractAddress":"","safeConfirmNumber":"","withdrawMax":""
        }]}]}"#;

        let result: CoinInfoResult = serde_json::from_str(json).unwrap();

        let chain = &result.rows[0].chains[0];
        assert_eq!(chain.deposit_min, None);
        assert_eq!(chain.withdraw_fee, None);
        assert_eq!(chain.contract_address, None);
    }

    #[test]
    fn deposit_coin_config_with_numeric_block_confirm_number() {
        let json = r#"{"configList":[{"coin":"USDT","chain":"ETH","coinShowName":"USDT",
            "chainType":"ERC20","blockConfirmNumber":1,"minDepositAmount":"0.01"}],
            "nextPageCursor":""}"#;

        let result: DepositAllowedCoinInfoResult = serde_json::from_str(json).unwrap();

        assert_eq!(result.config_list[0].block_confirm_number, 1);
    }
}
