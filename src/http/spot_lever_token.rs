use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_number_from_string as number;

use crate::{LtOrderStatus, LtOrderType, LtStatus, Timestamp, serde::empty_string_as_none};

// --- Get Leverage Token Info ---

/// Query params for
/// [`Client::get_leverage_token_info`](crate::http::Client::get_leverage_token_info).
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetLeverageTokenInfoParams {
    /// Abbreviation of the LT, such as `BTC3L`. Returns all tokens if not passed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt_coin: Option<String>,
}

impl GetLeverageTokenInfoParams {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new() -> Self {
        Self { lt_coin: None }
    }

    /// Set `lt_coin`: abbreviation of the LT, such as `BTC3L`. Returns all tokens if not passed.
    pub fn with_lt_coin(mut self, v: impl Into<String>) -> Self {
        self.lt_coin = Some(v.into());
        self
    }
}

impl Default for GetLeverageTokenInfoParams {
    fn default() -> Self {
        Self::new()
    }
}

/// Item of the list returned by `GET /v5/spot-lever-token/info` ([`Client::get_leverage_token_info`](crate::http::Client::get_leverage_token_info)).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LeverageTokenInfo {
    /// Abbreviation.
    pub lt_coin: String,
    /// Full name of leveraged token.
    pub lt_name: String,
    /// Single maximum purchase amount
    pub max_purchase: Decimal,
    /// Single minimum purchase amount
    pub min_purchase: Decimal,
    /// Maximum purchase amount in a single day
    pub max_purchase_daily: Decimal,
    /// Single maximum redemption quantity
    pub max_redeem: Decimal,
    /// Single minimum redemption quantity
    pub min_redeem: Decimal,
    /// Maximum redemption quantity in a single day
    pub max_redeem_daily: Decimal,
    /// Purchase fee rate.
    pub purchase_fee_rate: Decimal,
    /// Redeem fee rate.
    pub redeem_fee_rate: Decimal,
    /// Whether the leverage token can be purchased or redeemed.
    pub lt_status: LtStatus,
    /// Funding fee charged daily for users holding the leveraged token
    pub fund_fee: Decimal,
    /// The time to charge funding fee.
    #[serde(deserialize_with = "number")]
    pub fund_fee_time: Timestamp,
    /// Management fee rate.
    pub manage_fee_rate: Decimal,
    /// The time to charge management fee.
    #[serde(deserialize_with = "number")]
    pub manage_fee_time: Timestamp,
    /// Nominal asset value
    pub value: Decimal,
    /// Net value.
    pub net_value: Decimal,
    /// Total purchase upper limit
    pub total: Decimal,
}

// --- Get Leverage Token Market ---

/// Query params for
/// [`Client::get_leverage_token_market`](crate::http::Client::get_leverage_token_market).
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetLeverageTokenMarketParams {
    /// Abbreviation of the LT, such as `BTC3L`
    pub lt_coin: String,
}

impl GetLeverageTokenMarketParams {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new(lt_coin: impl Into<String>) -> Self {
        let lt_coin: String = lt_coin.into();
        Self { lt_coin }
    }
}

/// Result of `GET /v5/spot-lever-token/reference` ([`Client::get_leverage_token_market`](crate::http::Client::get_leverage_token_market)).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LeverageTokenMarket {
    /// Abbreviation of the LT, such as BTC3L.
    pub lt_coin: String,
    /// Net asset value
    pub nav: Decimal,
    /// Update time for net asset value (in milliseconds and UTC time zone).
    #[serde(deserialize_with = "number")]
    pub nav_time: Timestamp,
    /// Circulating supply in the secondary market
    pub circulation: Decimal,
    /// Basket composition value
    pub basket: Decimal,
    /// Real leverage, calculated by the last traded price
    pub leverage: Decimal,
}

// --- Purchase ---

/// Request body for
/// [`Client::purchase_leverage_token`](crate::http::Client::purchase_leverage_token).
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PurchaseLeverageTokenRequest {
    /// Abbreviation of the LT, such as `BTC3L`
    pub lt_coin: String,
    /// Quantity to purchase
    pub lt_amount: Decimal,
    /// Custom order identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_no: Option<String>,
}

impl PurchaseLeverageTokenRequest {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new(lt_coin: impl Into<String>, lt_amount: Decimal) -> Self {
        let lt_coin: String = lt_coin.into();
        Self {
            lt_coin,
            lt_amount,
            serial_no: None,
        }
    }

    /// Set `serial_no`: custom order identifier.
    pub fn with_serial_no(mut self, v: impl Into<String>) -> Self {
        self.serial_no = Some(v.into());
        self
    }
}

/// Result of `POST /v5/spot-lever-token/purchase` ([`Client::purchase_leverage_token`](crate::http::Client::purchase_leverage_token)).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PurchaseLeverageTokenResult {
    /// Abbreviation of the LT, such as BTC3L.
    pub lt_coin: String,
    /// Order status. 1: completed, 2: in progress, 3: failed.
    pub lt_order_status: LtOrderStatus,
    /// Executed quantity of leverage tokens
    pub exec_qty: Decimal,
    /// Executed amount of leverage tokens
    pub exec_amt: Decimal,
    /// Total purchase amount submitted
    pub amount: Decimal,
    /// Order ID.
    pub purchase_id: String,
    /// Serial number, customised order ID.
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub serial_no: Option<String>,
    /// Quote currency for the transaction
    pub value_coin: String,
}

// --- Redeem ---

/// Request body for
/// [`Client::redeem_leverage_token`](crate::http::Client::redeem_leverage_token).
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RedeemLeverageTokenRequest {
    /// Abbreviation of the LT, such as `BTC3L`
    pub lt_coin: String,
    /// Redeem quantity of the LT
    pub quantity: Decimal,
    /// Serial number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_no: Option<String>,
}

impl RedeemLeverageTokenRequest {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new(lt_coin: impl Into<String>, quantity: Decimal) -> Self {
        let lt_coin: String = lt_coin.into();
        Self {
            lt_coin,
            quantity,
            serial_no: None,
        }
    }

    /// Set `serial_no`: serial number.
    pub fn with_serial_no(mut self, v: impl Into<String>) -> Self {
        self.serial_no = Some(v.into());
        self
    }
}

/// Result of `POST /v5/spot-lever-token/redeem` ([`Client::redeem_leverage_token`](crate::http::Client::redeem_leverage_token)).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RedeemLeverageTokenResult {
    /// Abbreviation of the LT.
    pub lt_coin: String,
    /// Order status. 1: completed, 2: in progress, 3: failed.
    pub lt_order_status: LtOrderStatus,
    /// Quantity.
    pub quantity: Decimal,
    /// Leverage token quantity executed
    pub exec_qty: Decimal,
    /// Executed amount of the LT
    pub exec_amt: Decimal,
    /// Order ID.
    pub redeem_id: String,
    /// Serial number.
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub serial_no: Option<String>,
    /// Quote coin
    pub value_coin: String,
}

// --- Get Purchase/Redemption Records ---

/// Query params for
/// [`Client::get_leverage_token_order_records`](crate::http::Client::get_leverage_token_order_records).
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLeverageTokenOrderRecordsParams {
    /// Abbreviation of the LT, such as `BTC3L`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt_coin: Option<String>,
    /// Order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// The start timestamp (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Timestamp>,
    /// The end timestamp (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<Timestamp>,
    /// Limit for data size per page. [1, 500]. Default: 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    /// LT order type. 1: purchase, 2: redemption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt_order_type: Option<LtOrderType>,
    /// Serial number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_no: Option<String>,
}

impl GetLeverageTokenOrderRecordsParams {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new() -> Self {
        Self {
            lt_coin: None,
            order_id: None,
            start_time: None,
            end_time: None,
            limit: None,
            lt_order_type: None,
            serial_no: None,
        }
    }

    /// Set `lt_coin`: abbreviation of the LT, such as `BTC3L`.
    pub fn with_lt_coin(mut self, v: impl Into<String>) -> Self {
        self.lt_coin = Some(v.into());
        self
    }
    /// Set `order_id`: order ID.
    pub fn with_order_id(mut self, v: impl Into<String>) -> Self {
        self.order_id = Some(v.into());
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
    /// Set `limit`: limit for data size per page. [1, 500]. Default: 100.
    pub fn with_limit(mut self, v: u64) -> Self {
        self.limit = Some(v);
        self
    }
    /// Set `lt_order_type`: lT order type. 1: purchase, 2: redemption.
    pub fn with_lt_order_type(mut self, v: LtOrderType) -> Self {
        self.lt_order_type = Some(v);
        self
    }
    /// Set `serial_no`: serial number.
    pub fn with_serial_no(mut self, v: impl Into<String>) -> Self {
        self.serial_no = Some(v.into());
        self
    }
}

impl Default for GetLeverageTokenOrderRecordsParams {
    fn default() -> Self {
        Self::new()
    }
}

/// Item of the list returned by `GET /v5/spot-lever-token/order-record` ([`Client::get_leverage_token_order_records`](crate::http::Client::get_leverage_token_order_records)).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LeverageTokenOrderRecord {
    /// Abbreviation of the LT, such as BTC3L.
    pub lt_coin: String,
    /// Order ID.
    pub order_id: String,
    /// LT order type. 1: purchase, 2: redeem.
    pub lt_order_type: LtOrderType,
    /// Order time.
    pub order_time: Timestamp,
    /// Last update time of the order status.
    pub update_time: Timestamp,
    /// Order status. 1: completed, 2: in progress, 3: failed.
    pub lt_order_status: LtOrderStatus,
    /// Trading fees.
    pub fee: Decimal,
    /// Order quantity of the LT
    pub amount: Decimal,
    /// Filled value
    pub value: Decimal,
    /// Quote coin.
    pub value_coin: String,
    /// Serial number.
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub serial_no: Option<String>,
}
