// Data model mirroring the Bybit V5 API (request parameters, responses,
// stream messages): field and variant docs are added module by module; see
// https://bybit-exchange.github.io/docs/v5/intro for the meaning of each field.
#![allow(missing_docs)]

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
    pub spot_margin_mode: SpotMarginMode,
}

impl SwitchSpotMarginModeRequest {
    pub fn enable() -> Self {
        Self {
            spot_margin_mode: SpotMarginMode::Enabled,
        }
    }

    pub fn disable() -> Self {
        Self {
            spot_margin_mode: SpotMarginMode::Disabled,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SwitchSpotMarginModeResult {
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
    pub fn new(leverage: Decimal) -> Self {
        Self {
            leverage,
            currency: None,
        }
    }

    pub fn with_currency(mut self, v: impl Into<String>) -> Self {
        self.currency = Some(v.into());
        self
    }
}
