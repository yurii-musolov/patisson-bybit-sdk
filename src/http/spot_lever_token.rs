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
    pub fn new() -> Self {
        Self { lt_coin: None }
    }

    pub fn with_lt_coin(mut self, v: String) -> Self {
        self.lt_coin = Some(v);
        self
    }
}

impl Default for GetLeverageTokenInfoParams {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LeverageTokenInfo {
    pub lt_coin: String,
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
    pub purchase_fee_rate: Decimal,
    pub redeem_fee_rate: Decimal,
    pub lt_status: LtStatus,
    /// Funding fee charged daily for users holding the leveraged token
    pub fund_fee: Decimal,
    #[serde(deserialize_with = "number")]
    pub fund_fee_time: Timestamp,
    pub manage_fee_rate: Decimal,
    #[serde(deserialize_with = "number")]
    pub manage_fee_time: Timestamp,
    /// Nominal asset value
    pub value: Decimal,
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
    pub fn new(lt_coin: String) -> Self {
        Self { lt_coin }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LeverageTokenMarket {
    pub lt_coin: String,
    /// Net asset value
    pub nav: Decimal,
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
    pub fn new(lt_coin: String, lt_amount: Decimal) -> Self {
        Self {
            lt_coin,
            lt_amount,
            serial_no: None,
        }
    }

    pub fn with_serial_no(mut self, v: String) -> Self {
        self.serial_no = Some(v);
        self
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PurchaseLeverageTokenResult {
    pub lt_coin: String,
    pub lt_order_status: LtOrderStatus,
    /// Executed quantity of leverage tokens
    pub exec_qty: Decimal,
    /// Executed amount of leverage tokens
    pub exec_amt: Decimal,
    /// Total purchase amount submitted
    pub amount: Decimal,
    pub purchase_id: String,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_no: Option<String>,
}

impl RedeemLeverageTokenRequest {
    pub fn new(lt_coin: String, quantity: Decimal) -> Self {
        Self {
            lt_coin,
            quantity,
            serial_no: None,
        }
    }

    pub fn with_serial_no(mut self, v: String) -> Self {
        self.serial_no = Some(v);
        self
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RedeemLeverageTokenResult {
    pub lt_coin: String,
    pub lt_order_status: LtOrderStatus,
    pub quantity: Decimal,
    /// Leverage token quantity executed
    pub exec_qty: Decimal,
    /// Executed amount of the LT
    pub exec_amt: Decimal,
    pub redeem_id: String,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<Timestamp>,
    /// Limit for data size per page. [1, 500]. Default: 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt_order_type: Option<LtOrderType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_no: Option<String>,
}

impl GetLeverageTokenOrderRecordsParams {
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

    pub fn with_lt_coin(mut self, v: String) -> Self {
        self.lt_coin = Some(v);
        self
    }
    pub fn with_order_id(mut self, v: String) -> Self {
        self.order_id = Some(v);
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
    pub fn with_lt_order_type(mut self, v: LtOrderType) -> Self {
        self.lt_order_type = Some(v);
        self
    }
    pub fn with_serial_no(mut self, v: String) -> Self {
        self.serial_no = Some(v);
        self
    }
}

impl Default for GetLeverageTokenOrderRecordsParams {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LeverageTokenOrderRecord {
    pub lt_coin: String,
    pub order_id: String,
    pub lt_order_type: LtOrderType,
    pub order_time: Timestamp,
    pub update_time: Timestamp,
    pub lt_order_status: LtOrderStatus,
    pub fee: Decimal,
    /// Order quantity of the LT
    pub amount: Decimal,
    /// Filled value
    pub value: Decimal,
    pub value_coin: String,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub serial_no: Option<String>,
}
