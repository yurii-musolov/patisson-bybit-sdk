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

pub enum Path {
    MarketKline,
    MarketTickers,
    MarketInstrumentsInfo,
    MarketRecentTrade,
}
impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Path::MarketKline => "/v5/market/kline",
            Path::MarketTickers => "/v5/market/tickers",
            Path::MarketInstrumentsInfo => "/v5/market/instruments-info",
            Path::MarketRecentTrade => "/v5/market/recent-trade",
        };

        write!(f, "{}", s)
    }
}
