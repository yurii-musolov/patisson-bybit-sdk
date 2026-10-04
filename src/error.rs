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
