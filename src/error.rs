use crate::ret_code;

#[derive(Debug)]
pub enum Error {
    Api {
        code: i64,
        msg: String,
    },
    /// A non-2xx HTTP response whose body is not a Bybit error envelope
    /// (e.g. an HTML page from a CDN or gateway). `body` may be truncated.
    Http {
        status: u16,
        body: String,
    },
    InvalidHeaderValue(reqwest::header::InvalidHeaderValue),
    Io(std::io::Error),
    /// A private endpoint was called on a client created without
    /// `api_key`/`api_secret`.
    MissingCredentials,
    Msg(String),
    /// The local rate limiter rejected the request before it was sent.
    /// `retry_after_ms` is the estimated wait until capacity is available.
    RateLimited {
        retry_after_ms: u64,
    },
    /// The local rate limiter can never let this request through: its cost
    /// exceeds the bucket's burst, or the bucket does not refill. Retrying
    /// will not help; adjust [`RateLimiterConfig`](crate::http::RateLimiterConfig).
    RateLimitUnsatisfiable {
        cost: u32,
        burst: u32,
    },
    Reqwest(reqwest::Error),
    SerdeJson(serde_json::Error),
    /// A `*_all` pagination helper fetched `max` pages without reaching the end.
    TooManyPages {
        max: usize,
    },
    SerdeUrlEncoded(serde_urlencoded::ser::Error),
    SerdePathToError(serde_path_to_error::Error<serde_json::Error>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Api { code, msg } => write!(f, "Bybit API error: code: {code}, message: {msg}"),
            Error::Http { status, body } => write!(f, "HTTP error: status: {status}, body: {body}"),
            Error::InvalidHeaderValue(error) => write!(f, "invalid header value: {error}"),
            Error::Io(error) => write!(f, "I/O error: {error}"),
            Error::MissingCredentials => {
                write!(
                    f,
                    "api_key and api_secret are required for private endpoints"
                )
            }
            Error::Msg(msg) => write!(f, "{msg}"),
            Error::RateLimited { retry_after_ms } => {
                write!(f, "rate limited: retry after {retry_after_ms} ms")
            }
            Error::RateLimitUnsatisfiable { cost, burst } => write!(
                f,
                "rate limit can never be satisfied: request cost {cost}, bucket burst {burst}"
            ),
            Error::Reqwest(error) => write!(f, "reqwest error: {error}"),
            Error::TooManyPages { max } => {
                write!(f, "pagination did not finish after {max} pages")
            }
            Error::SerdeJson(error) => write!(f, "serde_json error: {error}"),
            Error::SerdeUrlEncoded(error) => write!(f, "serde_urlencoded error: {error}"),
            Error::SerdePathToError(error) => write!(
                f,
                "serde_path_to_error error: path: {}, msg: {}",
                error.path(),
                error.inner()
            ),
        }
    }
}

impl Error {
    /// The Bybit `retCode` of an [`Error::Api`] (see [`ret_code`](crate::ret_code)).
    pub fn api_code(&self) -> Option<i64> {
        match self {
            Error::Api { code, .. } => Some(*code),
            _ => None,
        }
    }

    /// Bybit rejected the request timestamp (`retCode` 10002): the local clock
    /// is off; see [`http::Client::sync_time`](crate::http::Client::sync_time).
    pub fn is_timestamp_error(&self) -> bool {
        self.api_code() == Some(ret_code::TIMESTAMP_OUT_OF_WINDOW)
    }

    /// A rate limit was hit: locally ([`Error::RateLimited`]), by Bybit
    /// (`retCode` 10006/10018) or as HTTP 429.
    pub fn is_rate_limited(&self) -> bool {
        match self {
            Error::RateLimited { .. } => true,
            Error::Api { code, .. } => {
                matches!(*code, ret_code::RATE_LIMITED | ret_code::IP_RATE_LIMITED)
            }
            Error::Http { status, .. } => *status == 429,
            _ => false,
        }
    }

    /// The request was refused because the value is already set (e.g.
    /// "leverage not modified"); usually this can be treated as success.
    pub fn is_not_modified(&self) -> bool {
        matches!(
            self.api_code(),
            Some(ret_code::POSITION_MODE_NOT_MODIFIED | ret_code::LEVERAGE_NOT_MODIFIED)
        )
    }

    /// Sending the same request again later may succeed: network errors and
    /// timeouts, rate limits, server errors (HTTP 5xx, `retCode` 10016) and
    /// timestamp errors (after [`sync_time`](crate::http::Client::sync_time)).
    ///
    /// For orders a timeout does not prove the order was not placed: check
    /// it (e.g. by `orderLinkId`) before placing it again.
    pub fn is_retryable(&self) -> bool {
        match self {
            Error::Reqwest(e) => e.is_timeout() || e.is_connect() || e.is_request(),
            Error::Http { status, .. } => *status == 429 || *status >= 500,
            Error::Api { code, .. } => matches!(
                *code,
                ret_code::RATE_LIMITED
                    | ret_code::IP_RATE_LIMITED
                    | ret_code::SERVER_ERROR
                    | ret_code::TIMESTAMP_OUT_OF_WINDOW
            ),
            Error::RateLimited { .. } | Error::Io(_) => true,
            _ => false,
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::InvalidHeaderValue(error) => Some(error),
            Error::Io(error) => Some(error),
            Error::Reqwest(error) => Some(error),
            Error::SerdeJson(error) => Some(error),
            Error::SerdeUrlEncoded(error) => Some(error),
            Error::SerdePathToError(error) => Some(error),
            Error::Api { .. }
            | Error::Http { .. }
            | Error::MissingCredentials
            | Error::Msg(_)
            | Error::RateLimited { .. }
            | Error::RateLimitUnsatisfiable { .. }
            | Error::TooManyPages { .. } => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<String> for Error {
    fn from(msg: String) -> Self {
        Error::Msg(msg)
    }
}

impl From<&str> for Error {
    fn from(msg: &str) -> Self {
        Error::Msg(msg.to_string())
    }
}

impl From<super::http::APIErrorResponse> for Error {
    fn from(resp: super::http::APIErrorResponse) -> Self {
        Self::Api {
            code: resp.ret_code,
            msg: resp.ret_msg,
        }
    }
}

impl From<reqwest::header::InvalidHeaderValue> for Error {
    fn from(err: reqwest::header::InvalidHeaderValue) -> Self {
        Error::InvalidHeaderValue(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Reqwest(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::SerdeJson(err)
    }
}

impl From<serde_urlencoded::ser::Error> for Error {
    fn from(err: serde_urlencoded::ser::Error) -> Self {
        Error::SerdeUrlEncoded(err)
    }
}

impl From<serde_path_to_error::Error<serde_json::Error>> for Error {
    fn from(err: serde_path_to_error::Error<serde_json::Error>) -> Self {
        Error::SerdePathToError(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn api(code: i64) -> Error {
        Error::Api {
            code,
            msg: String::new(),
        }
    }

    #[test]
    fn classifies_api_errors() {
        assert_eq!(api(110001).api_code(), Some(ret_code::ORDER_NOT_FOUND));
        assert_eq!(Error::MissingCredentials.api_code(), None);

        assert!(api(10002).is_timestamp_error());
        assert!(!api(10001).is_timestamp_error());

        assert!(api(10006).is_rate_limited());
        assert!(api(10018).is_rate_limited());
        assert!(Error::RateLimited { retry_after_ms: 1 }.is_rate_limited());
        assert!(
            Error::Http {
                status: 429,
                body: String::new()
            }
            .is_rate_limited()
        );
        assert!(!api(10001).is_rate_limited());

        assert!(api(110043).is_not_modified());
        assert!(api(110025).is_not_modified());
        assert!(!api(110001).is_not_modified());
    }

    #[test]
    fn retryable_errors() {
        for retryable in [
            api(10002),
            api(10006),
            api(10016),
            api(10018),
            Error::Http {
                status: 503,
                body: String::new(),
            },
            Error::Http {
                status: 429,
                body: String::new(),
            },
            Error::RateLimited { retry_after_ms: 1 },
        ] {
            assert!(retryable.is_retryable(), "{retryable:?}");
        }
        for permanent in [
            api(10001),
            api(10003),
            api(110007),
            Error::Http {
                status: 403,
                body: String::new(),
            },
            Error::MissingCredentials,
            Error::RateLimitUnsatisfiable { cost: 2, burst: 1 },
            Error::TooManyPages { max: 1 },
        ] {
            assert!(!permanent.is_retryable(), "{permanent:?}");
        }
    }
}
