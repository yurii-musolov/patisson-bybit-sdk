//! Enums Definitions
//!
//! Ref: https://bybit-exchange.github.io/docs/v5/enum

use serde::{Deserialize, Serialize};

/// Unified Account: spot | linear | inverse | option
/// Classic Account: linear | inverse | spot
#[derive(PartialEq, Debug, Deserialize, Serialize, Clone)]
pub enum Category {
    /// Inverse contract, including Inverse perp, Inverse futures.
    #[serde(rename = "inverse")]
    Inverse,
    /// USDT perpetual, and USDC contract, including USDC perp, USDC futures.
    #[serde(rename = "linear")]
    Linear,
    #[serde(rename = "option")]
    Option,
    #[serde(rename = "spot")]
    Spot,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Interval {
    #[serde(rename = "1")]
    Minute1,
    #[serde(rename = "3")]
    Minute3,
    #[serde(rename = "5")]
    Minute5,
    #[serde(rename = "15")]
    Minute15,
    #[serde(rename = "30")]
    Minute30,
    #[serde(rename = "60")]
    Hour1,
    #[serde(rename = "120")]
    Hour2,
    #[serde(rename = "240")]
    Hour4,
    #[serde(rename = "360")]
    Hour6,
    #[serde(rename = "720")]
    Hour12,
    #[serde(rename = "D")]
    Day1,
    #[serde(rename = "W")]
    Week1,
    #[serde(rename = "M")]
    Month1,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum ContractType {
    InversePerpetual,
    LinearPerpetual,
    /// USDT/USDC Futures
    LinearFutures,
    InverseFutures,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Status {
    PreLaunch,
    Trading,
    Delivering,
    Closed,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum CopyTrading {
    /// Regardless of normal account or UTA account, this trading pair does not support copy trading
    #[serde(rename = "none")]
    None,
    /// For both normal account and UTA account, this trading pair supports copy trading
    #[serde(rename = "both")]
    Both,
    /// Only for UTA account,this trading pair supports copy trading
    #[serde(rename = "utaOnly")]
    UtaOnly,
    /// Only for normal account, this trading pair supports copy trading
    #[serde(rename = "normalOnly")]
    NormalOnly,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum CurAuctionPhase {
    /// Pre-market trading is not started
    NotStarted,
    /// Pre-market trading is finished
    /// After the auction, if the pre-market contract fails to enter continues trading phase, it will be delisted and phase="Finished"
    /// After the continuous trading, if the pre-market contract fails to be converted to official contract, it will be delisted and phase="Finished"
    Finished,
    /// Auction phase of pre-market trading
    /// only timeInForce=GTC, orderType=Limit order is allowed to submit
    /// TP/SL are not supported; Conditional orders are not supported
    /// cannot modify the order at this stage
    /// order price range: [preOpenPrice x 0.5, maxPrice]
    CallAuction,
    /// Auction no cancel phase of pre-market trading
    /// only timeInForce=GTC, orderType=Limit order is allowed to submit
    /// TP/SL are not supported; Conditional orders are not supported
    /// cannot modify and cancel the order at this stage
    /// order price range: Buy [lastPrice x 0.5, markPrice x 1.1], Sell [markPrice x 0.9, maxPrice]
    CallAuctionNoCancel,
    /// cross matching phase
    /// cannot create, modify and cancel the order at this stage
    /// Candle data is released from this stage
    CrossMatching,
    /// Continuous trading phase
    /// There is no restriction to create, amend, cancel orders
    /// orderbook, public trade data is released from this stage
    ContinuousTrading,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Innovation {
    #[serde(rename = "0")]
    False,
    #[serde(rename = "1")]
    True,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Side {
    Buy,
    Sell,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_category() {
        let cases = vec![
            (Category::Inverse, r#""inverse""#),
            (Category::Linear, r#""linear""#),
            (Category::Option, r#""option""#),
            (Category::Spot, r#""spot""#),
        ];
        cases.iter().for_each(|(category, expected)| {
            let json = serde_json::to_string(category).unwrap();
            assert_eq!(json, *expected);
        });
    }

    #[test]
    fn deserialize_category() {
        let cases = vec![
            (r#""inverse""#, Category::Inverse),
            (r#""linear""#, Category::Linear),
            (r#""option""#, Category::Option),
            (r#""spot""#, Category::Spot),
        ];
        cases.iter().for_each(|(json, expected)| {
            let message: Category = serde_json::from_str(json).unwrap();
            assert_eq!(message, *expected);
        });
    }

    #[test]
    fn serialize_interval() {
        let cases = vec![
            (Interval::Minute1, r#""1""#),
            (Interval::Minute3, r#""3""#),
            (Interval::Minute5, r#""5""#),
            (Interval::Minute15, r#""15""#),
            (Interval::Minute30, r#""30""#),
            (Interval::Hour1, r#""60""#),
            (Interval::Hour2, r#""120""#),
            (Interval::Hour4, r#""240""#),
            (Interval::Hour6, r#""360""#),
            (Interval::Hour12, r#""720""#),
            (Interval::Day1, r#""D""#),
            (Interval::Week1, r#""W""#),
            (Interval::Month1, r#""M""#),
        ];
        cases.iter().for_each(|(category, expected)| {
            let json = serde_json::to_string(category).unwrap();
            assert_eq!(json, *expected);
        });
    }

    #[test]
    fn deserialize_interval() {
        let cases = vec![
            (r#""1""#, Interval::Minute1),
            (r#""3""#, Interval::Minute3),
            (r#""5""#, Interval::Minute5),
            (r#""15""#, Interval::Minute15),
            (r#""30""#, Interval::Minute30),
            (r#""60""#, Interval::Hour1),
            (r#""120""#, Interval::Hour2),
            (r#""240""#, Interval::Hour4),
            (r#""360""#, Interval::Hour6),
            (r#""720""#, Interval::Hour12),
            (r#""D""#, Interval::Day1),
            (r#""W""#, Interval::Week1),
            (r#""M""#, Interval::Month1),
        ];
        cases.iter().for_each(|(json, expected)| {
            let message: Interval = serde_json::from_str(json).unwrap();
            assert_eq!(message, *expected);
        });
    }
}
