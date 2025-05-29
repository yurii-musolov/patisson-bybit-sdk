#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Reqwest(reqwest::Error),
    SerdeJson(serde_json::Error),
    Msg(String),
    Api { code: i64, msg: String },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(error) => write!(f, "I/O error: {error}"),
            Error::Msg(msg) => write!(f, "{msg}"),
            Error::Api { code, msg } => write!(f, "Bybit API error: code: {code}, message: {msg}"),
            Error::Reqwest(error) => write!(f, "reqwest error: {error}"),
            Error::SerdeJson(error) => write!(f, "serde_json error: {error}"),
        }
    }
}

impl std::error::Error for Error {}

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

impl<T> From<super::Resp<T>> for Error {
    fn from(resp: super::Resp<T>) -> Self {
        Self::Api {
            code: resp.ret_code,
            msg: resp.ret_msg.into(),
        }
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
