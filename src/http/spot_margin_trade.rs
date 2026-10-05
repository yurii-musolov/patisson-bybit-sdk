use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::SpotMarginMode;

// --- Toggle Margin Trade ---

/// Request body for
/// [`Client::switch_spot_margin_mode`](crate::http::Client::switch_spot_margin_mode).
///
/// The account must have completed spot margin activation (including the required quiz)
/// before this call succeeds.
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SwitchSpotMarginModeRequest {
    /// 1: on, 0: off.
    pub spot_margin_mode: SpotMarginMode,
}

impl SwitchSpotMarginModeRequest {
    /// Turn spot margin trading on.
    pub fn enable() -> Self {
        Self {
            spot_margin_mode: SpotMarginMode::Enabled,
        }
    }

    /// Turn spot margin trading off.
    pub fn disable() -> Self {
        Self {
            spot_margin_mode: SpotMarginMode::Disabled,
        }
    }
}

/// Result of `POST /v5/spot-margin-trade/switch-mode` ([`Client::switch_spot_margin_mode`](crate::http::Client::switch_spot_margin_mode)).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SwitchSpotMarginModeResult {
    /// Spot margin status. 1: on, 0: off.
    pub spot_margin_mode: SpotMarginMode,
}

// --- Set Leverage ---

/// Request body for
/// [`Client::set_spot_margin_leverage`](crate::http::Client::set_spot_margin_leverage).
///
/// The account must have completed spot margin activation (including the required quiz)
/// before this call succeeds.
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetSpotMarginLeverageRequest {
    /// Maximum leverage. Range: [2, 10]
    pub leverage: Decimal,
    /// Coin name, uppercase only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

impl SetSpotMarginLeverageRequest {
    /// Create with the required fields; set the optional ones with the `with_*` methods.
    pub fn new(leverage: Decimal) -> Self {
        Self {
            leverage,
            currency: None,
        }
    }

    /// Set `currency`: coin name, uppercase only.
    pub fn with_currency(mut self, v: impl Into<String>) -> Self {
        self.currency = Some(v.into());
        self
    }
}
