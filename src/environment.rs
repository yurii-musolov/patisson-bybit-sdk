//! Bybit deployments and their REST and WebSocket base URLs.

use crate::{Category, Path};

/// A Bybit deployment: where to send REST requests and open WebSocket
/// streams.
///
/// Region-specific sites have their own domains; use the variant of the site
/// your account is registered on. Sites whose WebSocket domain Bybit does not
/// document (UAE, EEA) or that did not resolve when this list was made
/// (Japan, Hong Kong) are not listed: use [`Environment::Custom`].
///
/// See <https://bybit-exchange.github.io/docs/v5/guide> and
/// <https://bybit-exchange.github.io/docs/v5/ws/connect>.
///
/// ```
/// use bybit::{Category, Environment};
///
/// let env = Environment::Demo;
/// assert_eq!(env.api_url(), "https://api-demo.bybit.com");
/// // Demo trading has private streams only; public data comes from mainnet.
/// assert_eq!(
///     env.public_stream_url(Category::Linear),
///     "wss://stream.bybit.com/v5/public/linear"
/// );
/// assert_eq!(env.private_stream_url(), "wss://stream-demo.bybit.com/v5/private");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Environment {
    /// `api.bybit.com`, `stream.bybit.com`.
    Mainnet,
    /// Alternative mainnet REST domain `api.bytick.com` (streams on
    /// `stream.bybit.com`).
    MainnetBytick,
    /// `api-testnet.bybit.com`, `stream-testnet.bybit.com`.
    Testnet,
    /// Demo trading: `api-demo.bybit.com`; private streams on
    /// `stream-demo.bybit.com`, public data from mainnet `stream.bybit.com`.
    Demo,
    /// Accounts registered on bybit.tr.
    Turkey,
    /// Accounts registered on bybit.kz.
    Kazakhstan,
    /// Accounts registered on bybitgeorgia.ge.
    Georgia,
    /// Accounts registered on bybit.id.
    Indonesia,
    /// Any other deployment, e.g. `api: "https://api.bybit.ae"`.
    /// Both values are base URLs without a trailing slash.
    Custom {
        /// REST base URL, e.g. `https://api.bybit.com`.
        api: &'static str,
        /// WebSocket base URL, e.g. `wss://stream.bybit.com`.
        stream: &'static str,
    },
}

impl Environment {
    /// REST base URL, e.g. `https://api.bybit.com`.
    pub fn api_url(&self) -> &'static str {
        match self {
            Self::Mainnet => "https://api.bybit.com",
            Self::MainnetBytick => "https://api.bytick.com",
            Self::Testnet => "https://api-testnet.bybit.com",
            Self::Demo => "https://api-demo.bybit.com",
            Self::Turkey => "https://api.bybit.tr",
            Self::Kazakhstan => "https://api.bybit.kz",
            Self::Georgia => "https://api.bybitgeorgia.ge",
            Self::Indonesia => "https://api.bybit.id",
            Self::Custom { api, .. } => api,
        }
    }

    /// URL of the public stream of `category`, e.g.
    /// `wss://stream.bybit.com/v5/public/linear`.
    pub fn public_stream_url(&self, category: Category) -> String {
        let path = match category {
            Category::Spot => Path::PublicSpot,
            Category::Linear => Path::PublicLinear,
            Category::Inverse => Path::PublicInverse,
            Category::Option => Path::PublicOption,
        };
        format!("{}{path}", self.public_stream_base())
    }

    /// URL of the private stream (orders, positions, executions, wallet),
    /// e.g. `wss://stream.bybit.com/v5/private`.
    pub fn private_stream_url(&self) -> String {
        format!("{}{}", self.private_stream_base(), Path::Private)
    }

    /// URL of the order entry stream (`/v5/trade`), or `None` where Bybit
    /// does not offer it: demo trading, and sites without a documented
    /// order entry stream (Georgia, Indonesia).
    pub fn trade_stream_url(&self) -> Option<String> {
        match self {
            Self::Demo | Self::Georgia | Self::Indonesia => None,
            _ => Some(format!("{}{}", self.private_stream_base(), Path::Trade)),
        }
    }

    fn public_stream_base(&self) -> &'static str {
        match self {
            // Demo trading serves private streams only.
            Self::Demo => Self::Mainnet.private_stream_base(),
            _ => self.private_stream_base(),
        }
    }

    fn private_stream_base(&self) -> &'static str {
        match self {
            Self::Mainnet | Self::MainnetBytick => "wss://stream.bybit.com",
            Self::Testnet => "wss://stream-testnet.bybit.com",
            Self::Demo => "wss://stream-demo.bybit.com",
            Self::Turkey => "wss://stream.bybit.tr",
            Self::Kazakhstan => "wss://stream.bybit.kz",
            Self::Georgia => "wss://stream.bybitgeorgia.ge",
            Self::Indonesia => "wss://stream.bybit.id",
            Self::Custom { stream, .. } => stream,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urls_of_every_environment() {
        let cases = [
            (
                Environment::Mainnet,
                "https://api.bybit.com",
                "wss://stream.bybit.com/v5/public/spot",
                "wss://stream.bybit.com/v5/private",
            ),
            (
                Environment::MainnetBytick,
                "https://api.bytick.com",
                "wss://stream.bybit.com/v5/public/spot",
                "wss://stream.bybit.com/v5/private",
            ),
            (
                Environment::Testnet,
                "https://api-testnet.bybit.com",
                "wss://stream-testnet.bybit.com/v5/public/spot",
                "wss://stream-testnet.bybit.com/v5/private",
            ),
            (
                Environment::Demo,
                "https://api-demo.bybit.com",
                "wss://stream.bybit.com/v5/public/spot",
                "wss://stream-demo.bybit.com/v5/private",
            ),
            (
                Environment::Turkey,
                "https://api.bybit.tr",
                "wss://stream.bybit.tr/v5/public/spot",
                "wss://stream.bybit.tr/v5/private",
            ),
            (
                Environment::Kazakhstan,
                "https://api.bybit.kz",
                "wss://stream.bybit.kz/v5/public/spot",
                "wss://stream.bybit.kz/v5/private",
            ),
            (
                Environment::Georgia,
                "https://api.bybitgeorgia.ge",
                "wss://stream.bybitgeorgia.ge/v5/public/spot",
                "wss://stream.bybitgeorgia.ge/v5/private",
            ),
            (
                Environment::Indonesia,
                "https://api.bybit.id",
                "wss://stream.bybit.id/v5/public/spot",
                "wss://stream.bybit.id/v5/private",
            ),
            (
                Environment::Custom {
                    api: "https://api.example.com",
                    stream: "wss://stream.example.com",
                },
                "https://api.example.com",
                "wss://stream.example.com/v5/public/spot",
                "wss://stream.example.com/v5/private",
            ),
        ];

        assert_eq!(
            Environment::Mainnet.trade_stream_url().as_deref(),
            Some("wss://stream.bybit.com/v5/trade")
        );
        assert_eq!(
            Environment::Testnet.trade_stream_url().as_deref(),
            Some("wss://stream-testnet.bybit.com/v5/trade")
        );
        assert_eq!(Environment::Demo.trade_stream_url(), None);

        for (env, api, public, private) in cases {
            assert_eq!(env.api_url(), api, "{env:?}");
            assert_eq!(env.public_stream_url(Category::Spot), public, "{env:?}");
            assert_eq!(env.private_stream_url(), private, "{env:?}");
        }
    }

    #[test]
    fn public_stream_path_follows_the_category() {
        let env = Environment::Mainnet;

        assert!(
            env.public_stream_url(Category::Linear)
                .ends_with("/v5/public/linear")
        );
        assert!(
            env.public_stream_url(Category::Inverse)
                .ends_with("/v5/public/inverse")
        );
        assert!(
            env.public_stream_url(Category::Option)
                .ends_with("/v5/public/option")
        );
    }
}
