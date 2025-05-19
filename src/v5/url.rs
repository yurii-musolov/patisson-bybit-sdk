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
/// The header for broker users only.
pub const HEADER_REFERER: &str = "Referer";
/// The header for specify how long an HTTP request is valid (unit in millisecond and default value is 5,000). It is also used to prevent replay attacks..
pub const HEADER_X_BAPI_RECV_WINDOW: &str = "X-BAPI-RECV-WINDOW";
/// Your remaining requests for current endpoint.
pub const HEADER_X_BAPI_LIMIT: &str = "X-Bapi-Limit";
/// Your current limit for current endpoint.
pub const HEADER_X_BAPI_LIMIT_STATUS: &str = "X-Bapi-Limit-Status";
/// The timestamp indicating when your request limit resets if you have exceeded your rate_limit. Otherwise, this is just the current timestamp (it may not exactly match timeNow).
pub const HEADER_X_BAPI_LIMIT_RESET_TIMESTAMP: &str = "X-Bapi-Limit-Reset-Timestamp";
///  To assist in diagnosing advanced network problems. Its value should be unique for each request.
pub const HEADER_CDN_REQUEST_ID: &str = "cdn-request-id";

pub enum Path {
    // Candlestick, orderbook, ticker, platform transaction data, underlying financial rules, risk control rules
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
            Path::MarketKline => "/v5/market/kline",
            Path::MarketMarkPriceKline => "/v5/market/mark-price-kline",
            Path::MarketIndexPriceKline => "/v5/market/index-price-kline",
            Path::MarketPremiumIndexPriceKline => "/v5/market/premium-index-price-kline",
            Path::MarketOrderbook => "/v5/market/orderbook",
            Path::MarketTickers => "/v5/market/tickers",
            Path::MarketFundingHistory => "/v5/market/funding/history",
            Path::MarketRecentTrade => "/v5/market/recent-trade",
            Path::MarketOpenInterest => "/v5/market/open-interest",
            Path::MarketHistoricalVolatility => "/v5/market/historical-volatility",
            Path::MarketInsurance => "/v5/market/insurance",
            Path::MarketInstrumentsInfo => "/v5/market/instruments-info",
            Path::MarketRiskLimit => "/v5/market/risk-limit",
            Path::MarketDeliveryPrice => "/v5/market/delivery-price",

            Path::OrderCreate => "/v5/order/create",
            Path::OrderAmend => "/v5/order/amend",
            Path::OrderCancel => "/v5/order/cancel",
            Path::OrderRealtime => "/v5/order/realtime",
            Path::OrderCancelAll => "/v5/order/cancel-all",
            Path::OrderHistory => "/v5/order/history",
            Path::OrderCreateBatch => "/v5/order/create-batch",
            Path::OrderAmendBatch => "/v5/order/amend-batch",
            Path::OrderCancelBatch => "/v5/order/cancel-batch",
            Path::OrderSpotBorrowCheck => "/v5/order/spot-borrow-check",

            Path::PositionList => "/v5/position/list",
            Path::PositionSetLeverage => "/v5/position/set-leverage",
            Path::PositionSetRiskLimit => "/v5/position/set-risk-limit",
            Path::PositionTradingStop => "/v5/position/trading-stop",
            Path::PositionSwitchIsolated => "/v5/position/switch-isolated",
            Path::PositionSwitchMode => "/v5/position/switch-mode",
            Path::PositionSetAutoAddMargin => "/v5/position/set-auto-add-margin",
            Path::PositionClosedPnl => "/v5/position/closed-pnl",
            Path::ExecutionList => "/v5/execution/list",

            Path::AccountWalletBalance => "/v5/account/wallet-balance",
            Path::AccountUpgradeToUta => "/v5/account/upgrade-to-uta",
            Path::AccountBorrowHistory => "/v5/account/borrow-history",
            Path::AccountCollateralInfo => "/v5/account/collateral-info",
            Path::AssetCoinGreeks => "/v5/asset/coin-greeks",
            Path::AccountInfo => "/v5/account/info",
            Path::AccountTransactionLog => "/v5/account/transaction-log",
            Path::AccountSetMarginMode => "/v5/account/set-margin-mode",
            Path::AccountSetMarginModeDemoApplyMoney => "/v5/account/demo-apply-money",

            Path::AssetDeliveryRecord => "/v5/asset/delivery-record",
            Path::AssetSettlementRecord => "/v5/asset/settlement-record",
            Path::AssetTransferInterTransfer => "/v5/asset/transfer/inter-transfer",
            Path::AssetTransferQueryInterTransferList => {
                "/v5/asset/transfer/query-inter-transfer-list"
            }
            Path::AssetTransferSaveTransferSubMember => {
                "/v5/asset/transfer/save-transfer-sub-member"
            }
            Path::AssetTransferUniversalTransfer => "/v5/asset/transfer/universal-transfer",
            Path::AssetTransferQueryUniversalTransferList => {
                "/v5/asset/transfer/query-universal-transfer-list"
            }
            Path::AssetTransferQueryTransferCoinList => {
                "/v5/asset/transfer/query-transfer-coin-list"
            }
            Path::AssetTransferQuerySubMemberList => "/v5/asset/transfer/query-sub-member-list",
            Path::AssetTransferQueryAccountCoinBalance => {
                "/v5/asset/transfer/query-account-coin-balance"
            }
            Path::AssetTransferQueryAssetInfo => "/v5/asset/transfer/query-asset-info",
            Path::AssetDepositQueryAllowedList => "/v5/asset/deposit/query-allowed-list",
            Path::AssetDepositQueryRecord => "/v5/asset/deposit/query-record",
            Path::AssetDepositQuerySubMemberRecord => "/v5/asset/deposit/query-sub-member-record",
            Path::AssetWithdrawQueryRecord => "/v5/asset/withdraw/query-record",
            Path::AssetCoinQueryInfo => "/v5/asset/coin/query-info",
            Path::AssetWithdrawCreate => "/v5/asset/withdraw/create",
            Path::AssetWithdrawCancel => "/v5/asset/withdraw/cancel",
            Path::AssetDepositQueryAddress => "/v5/asset/deposit/query-address",
            Path::AssetDepositQuerySubMemberAddress => "/v5/asset/deposit/query-sub-member-address",
            Path::AssetExchangeOrderRecord => "/v5/asset/exchange/order-record",

            Path::SpotLeverTokenInfo => "/v5/spot-lever-token/info",
            Path::SpotLeverTokenReference => "/v5/spot-lever-token/reference",
            Path::SpotLeverTokenPurchase => "/v5/spot-lever-token/purchase",
            Path::SpotLeverTokenRedeem => "/v5/spot-lever-token/redeem",
            Path::SpotLeverTokenOrderRecord => "/v5/spot-lever-token/order-record",

            Path::SpotMarginTradeSwitchMode => "/v5/spot-margin-trade/switch-mode",
            Path::SpotMarginTradeSetLeverage => "/v5/spot-margin-trade/set-leverage",
            Path::SpotMarginTradeSetPledgeToken => "/v5/spot-margin-trade/set-pledge-token",

            Path::PublicSpot => "/v5/public/spot",
            Path::PublicLinear => "/v5/public/linear",
            Path::PublicInverse => "/v5/public/inverse",
            Path::PublicOption => "/v5/public/option",
            Path::Private => "/v5/private",
            Path::Trade => "/v5/trade",
        };

        write!(f, "{}", s)
    }
}
