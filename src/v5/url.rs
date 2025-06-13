use std::fmt;

// Mainnet.
pub const BASE_URL_API_MAINNET_1: &str = "https://api.bybit.com";
pub const BASE_URL_API_MAINNET_2: &str = "https://api.bytick.com";
/// For Netherland users.
pub const BASE_URL_API_MAINNET_3: &str = "https://api.bybit.nl";
/// For Hong Kong users.
pub const BASE_URL_API_MAINNET_4: &str = "https://api.byhkbit.com";
/// For Turkey users.
pub const BASE_URL_API_MAINNET_5: &str = "wss://api.bybit-tr.com";
/// For Kazakhstan users.
pub const BASE_URL_API_MAINNET_6: &str = "wss://api.bybit.kz";

pub const BASE_URL_STREAM_MAINNET_1: &str = "wss://stream.bybit.com";
/// For Turkey users.
pub const BASE_URL_STREAM_MAINNET_2: &str = "wss://stream.bybit-tr.com";
/// For Kazakhstan users.
pub const BASE_URL_STREAM_MAINNET_3: &str = "wss://stream.bybit.kz";

// Testnet.
pub const BASE_URL_API_TESTNET: &str = "https://api-testnet.bybit.com";
pub const BASE_URL_STREAM_TESTNET: &str = "wss://stream-testnet.bybit.com";

// Demo trading.
pub const BASE_URL_API_DEMO_TRADING: &str = "https://api-demo.bybit.com";
pub const BASE_URL_STREAM_DEMO_TRADING: &str = "wss://stream-demo.bybit.com";

// The following HTTP header keys must be used for authentication:
/// API key.
pub const HEADER_X_BAPI_API_KEY: &str = "X-BAPI-API-KEY";
/// UTC timestamp in milliseconds.
pub const HEADER_X_BAPI_TIMESTAMP: &str = "X-BAPI-TIMESTAMP";
/// A signature derived from the request's parameters.
pub const HEADER_X_BAPI_SIGN: &str = "X-BAPI-SIGN";
/// The header for broker users only.
pub const HEADER_X_REFERER: &str = "X-Referer";
/// The header for specify how long an HTTP request is valid (unit in millisecond and default value is 5,000). It is also used to prevent replay attacks..
pub const HEADER_X_BAPI_RECV_WINDOW: &str = "X-BAPI-RECV-WINDOW";
/// Your remaining requests for current endpoint.
pub const HEADER_X_BAPI_LIMIT: &str = "X-Bapi-Limit";
/// Your current limit for current endpoint.
pub const HEADER_X_BAPI_LIMIT_STATUS: &str = "X-Bapi-Limit-Status";
/// The timestamp indicating when your request limit resets if you have exceeded your rate_limit. Otherwise, this is just the current timestamp (it may not exactly match timeNow).
pub const HEADER_X_BAPI_LIMIT_RESET_TIMESTAMP: &str = "X-Bapi-Limit-Reset-Timestamp";
///  To assist in diagnosing advanced network problems. Its value should be unique for each request.
#[allow(dead_code)]
pub const HEADER_CDN_REQUEST_ID: &str = "cdn-request-id";

pub const HEADER_RET_CODE: &str = "ret_code";
pub const HEADER_TRACE_ID: &str = "traceid";
pub const HEADER_TIME_NOW: &str = "timenow";

pub enum Path {
    // Candlestick, orderbook, ticker, platform transaction data, underlying financial rules, risk control rules
    MarketServerTime,
    MarketKline,
    MarketMarkPriceKline,
    MarketIndexPriceKline,
    MarketPremiumIndexPriceKline,
    MarketOrderbook,
    MarketTickers,
    MarketFundingHistory,
    MarketRecentTrade,
    MarketOpenInterest,
    MarketHistoricalVolatility,
    MarketInsurance,
    MarketInstrumentsInfo,
    MarketRiskLimit,
    MarketDeliveryPrice,

    // Order management
    OrderCreate,
    OrderAmend,
    OrderCancel,
    OrderRealtime,
    OrderCancelAll,
    OrderHistory,
    OrderCreateBatch,
    OrderAmendBatch,
    OrderCancelBatch,
    OrderSpotBorrowCheck,

    // Position management
    PositionList,
    PositionSetLeverage,
    PositionSetRiskLimit,
    PositionTradingStop,
    PositionSwitchIsolated,
    PositionSwitchMode,
    PositionSetAutoAddMargin,
    PositionClosedPnl,
    ExecutionList,

    // Single account operations only – unified funding account, rates, etc.
    AccountWalletBalance,
    AccountUpgradeToUta,
    AccountBorrowHistory,
    AccountCollateralInfo,
    AssetCoinGreeks,
    AccountInfo,
    AccountTransactionLog,
    AccountSetMarginMode,
    AccountSetMarginModeDemoApplyMoney,

    // Operations across multiple accounts – asset management, fund management, etc.
    AssetDeliveryRecord,
    AssetSettlementRecord,
    AssetTransferInterTransfer,
    AssetTransferQueryInterTransferList,
    AssetTransferSaveTransferSubMember,
    AssetTransferUniversalTransfer,
    AssetTransferQueryUniversalTransferList,
    AssetTransferQueryTransferCoinList,
    AssetTransferQuerySubMemberList,
    AssetTransferQueryAccountCoinBalance,
    AssetTransferQueryAssetInfo,
    AssetDepositQueryAllowedList,
    AssetDepositQueryRecord,
    AssetDepositQuerySubMemberRecord,
    AssetWithdrawQueryRecord,
    AssetCoinQueryInfo,
    AssetWithdrawCreate,
    AssetWithdrawCancel,
    AssetDepositQueryAddress,
    AssetDepositQuerySubMemberAddress,
    AssetExchangeOrderRecord,

    // Obtain quotes from Leveraged Tokens on Spot, and to exercise purchase and redeem functions
    SpotLeverTokenInfo,
    SpotLeverTokenReference,
    SpotLeverTokenPurchase,
    SpotLeverTokenRedeem,
    SpotLeverTokenOrderRecord,

    // Manage Margin Trading on Spot
    SpotMarginTradeSwitchMode,
    SpotMarginTradeSetLeverage,
    SpotMarginTradeSetPledgeToken,

    // Stream paths.
    PublicSpot,
    PublicLinear,
    PublicInverse,
    PublicOption,
    Private,
    Trade,
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::MarketServerTime => "/v5/market/time",
            Self::MarketKline => "/v5/market/kline",
            Self::MarketMarkPriceKline => "/v5/market/mark-price-kline",
            Self::MarketIndexPriceKline => "/v5/market/index-price-kline",
            Self::MarketPremiumIndexPriceKline => "/v5/market/premium-index-price-kline",
            Self::MarketOrderbook => "/v5/market/orderbook",
            Self::MarketTickers => "/v5/market/tickers",
            Self::MarketFundingHistory => "/v5/market/funding/history",
            Self::MarketRecentTrade => "/v5/market/recent-trade",
            Self::MarketOpenInterest => "/v5/market/open-interest",
            Self::MarketHistoricalVolatility => "/v5/market/historical-volatility",
            Self::MarketInsurance => "/v5/market/insurance",
            Self::MarketInstrumentsInfo => "/v5/market/instruments-info",
            Self::MarketRiskLimit => "/v5/market/risk-limit",
            Self::MarketDeliveryPrice => "/v5/market/delivery-price",

            Self::OrderCreate => "/v5/order/create",
            Self::OrderAmend => "/v5/order/amend",
            Self::OrderCancel => "/v5/order/cancel",
            Self::OrderRealtime => "/v5/order/realtime",
            Self::OrderCancelAll => "/v5/order/cancel-all",
            Self::OrderHistory => "/v5/order/history",
            Self::OrderCreateBatch => "/v5/order/create-batch",
            Self::OrderAmendBatch => "/v5/order/amend-batch",
            Self::OrderCancelBatch => "/v5/order/cancel-batch",
            Self::OrderSpotBorrowCheck => "/v5/order/spot-borrow-check",

            Self::PositionList => "/v5/position/list",
            Self::PositionSetLeverage => "/v5/position/set-leverage",
            Self::PositionSetRiskLimit => "/v5/position/set-risk-limit",
            Self::PositionTradingStop => "/v5/position/trading-stop",
            Self::PositionSwitchIsolated => "/v5/position/switch-isolated",
            Self::PositionSwitchMode => "/v5/position/switch-mode",
            Self::PositionSetAutoAddMargin => "/v5/position/set-auto-add-margin",
            Self::PositionClosedPnl => "/v5/position/closed-pnl",
            Self::ExecutionList => "/v5/execution/list",

            Self::AccountWalletBalance => "/v5/account/wallet-balance",
            Self::AccountUpgradeToUta => "/v5/account/upgrade-to-uta",
            Self::AccountBorrowHistory => "/v5/account/borrow-history",
            Self::AccountCollateralInfo => "/v5/account/collateral-info",
            Self::AssetCoinGreeks => "/v5/asset/coin-greeks",
            Self::AccountInfo => "/v5/account/info",
            Self::AccountTransactionLog => "/v5/account/transaction-log",
            Self::AccountSetMarginMode => "/v5/account/set-margin-mode",
            Self::AccountSetMarginModeDemoApplyMoney => "/v5/account/demo-apply-money",

            Self::AssetDeliveryRecord => "/v5/asset/delivery-record",
            Self::AssetSettlementRecord => "/v5/asset/settlement-record",
            Self::AssetTransferInterTransfer => "/v5/asset/transfer/inter-transfer",
            Self::AssetTransferQueryInterTransferList => {
                "/v5/asset/transfer/query-inter-transfer-list"
            }
            Self::AssetTransferSaveTransferSubMember => {
                "/v5/asset/transfer/save-transfer-sub-member"
            }
            Self::AssetTransferUniversalTransfer => "/v5/asset/transfer/universal-transfer",
            Self::AssetTransferQueryUniversalTransferList => {
                "/v5/asset/transfer/query-universal-transfer-list"
            }
            Self::AssetTransferQueryTransferCoinList => {
                "/v5/asset/transfer/query-transfer-coin-list"
            }
            Self::AssetTransferQuerySubMemberList => "/v5/asset/transfer/query-sub-member-list",
            Self::AssetTransferQueryAccountCoinBalance => {
                "/v5/asset/transfer/query-account-coin-balance"
            }
            Self::AssetTransferQueryAssetInfo => "/v5/asset/transfer/query-asset-info",
            Self::AssetDepositQueryAllowedList => "/v5/asset/deposit/query-allowed-list",
            Self::AssetDepositQueryRecord => "/v5/asset/deposit/query-record",
            Self::AssetDepositQuerySubMemberRecord => "/v5/asset/deposit/query-sub-member-record",
            Self::AssetWithdrawQueryRecord => "/v5/asset/withdraw/query-record",
            Self::AssetCoinQueryInfo => "/v5/asset/coin/query-info",
            Self::AssetWithdrawCreate => "/v5/asset/withdraw/create",
            Self::AssetWithdrawCancel => "/v5/asset/withdraw/cancel",
            Self::AssetDepositQueryAddress => "/v5/asset/deposit/query-address",
            Self::AssetDepositQuerySubMemberAddress => "/v5/asset/deposit/query-sub-member-address",
            Self::AssetExchangeOrderRecord => "/v5/asset/exchange/order-record",

            Self::SpotLeverTokenInfo => "/v5/spot-lever-token/info",
            Self::SpotLeverTokenReference => "/v5/spot-lever-token/reference",
            Self::SpotLeverTokenPurchase => "/v5/spot-lever-token/purchase",
            Self::SpotLeverTokenRedeem => "/v5/spot-lever-token/redeem",
            Self::SpotLeverTokenOrderRecord => "/v5/spot-lever-token/order-record",

            Self::SpotMarginTradeSwitchMode => "/v5/spot-margin-trade/switch-mode",
            Self::SpotMarginTradeSetLeverage => "/v5/spot-margin-trade/set-leverage",
            Self::SpotMarginTradeSetPledgeToken => "/v5/spot-margin-trade/set-pledge-token",

            Self::PublicSpot => "/v5/public/spot",
            Self::PublicLinear => "/v5/public/linear",
            Self::PublicInverse => "/v5/public/inverse",
            Self::PublicOption => "/v5/public/option",
            Self::Private => "/v5/private",
            Self::Trade => "/v5/trade",
        };

        write!(f, "{}", s)
    }
}
