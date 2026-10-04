use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;
use std::fmt;

use crate::Timestamp;

#[derive(Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SensitiveString(String);

impl fmt::Display for SensitiveString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "REDACTED")
    }
}

impl fmt::Debug for SensitiveString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", "REDACTED")
    }
}

impl std::ops::Deref for SensitiveString {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for SensitiveString {
    fn from(s: String) -> Self {
        SensitiveString(s)
    }
}

impl From<&str> for SensitiveString {
    fn from(s: &str) -> Self {
        SensitiveString(s.to_string())
    }
}

impl AsRef<str> for SensitiveString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl SensitiveString {
    pub fn expose(&self) -> &str {
        &self.0
    }
}

/// Generates HMAC using the given key and message.
///
/// # Arguments
/// * `key` - Secret key for HMAC.
/// * `message` - Message to authenticate.
///
/// # Returns
/// A String containing the hex of HMAC digest.
///
/// # Example
/// ```
/// use bybit::hmac_sha256;
/// let signature = hmac_sha256(b"my-secret-key", b"important message");
/// println!("HMAC: {signature}");
/// ```
pub fn hmac_sha256(key: impl AsRef<[u8]>, message: impl AsRef<[u8]>) -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(key.as_ref()).unwrap();
    mac.update(message.as_ref());
    let mac = mac.finalize().into_bytes();
    hex::encode(mac)
}

type Timer = fn() -> Timestamp;

#[derive(Debug)]
pub struct Signer {
    api_key: SensitiveString,
    api_secret: SensitiveString,
    /// Milliseconds.
    recv_window: Timestamp,
    timer: Timer,
}

impl Signer {
    /// Create Signer instance.
    pub fn new(
        api_key: SensitiveString,
        api_secret: SensitiveString,
        recv_window: Timestamp,
        timer: Option<Timer>,
    ) -> Self {
        Self {
            api_key,
            api_secret,
            recv_window,
            timer: timer.unwrap_or(timestamp),
        }
    }

    /// Sign `s` with the current time of this signer's timer.
    ///
    /// return: (signature, timestamp)
    pub fn sign(&self, s: &str) -> (String, String) {
        let timestamp = (self.timer)();
        (self.sign_at(s, timestamp), timestamp.to_string())
    }

    /// Sign `s` as if sent at `timestamp` (milliseconds), e.g. the local time
    /// corrected by the offset to the server clock.
    pub fn sign_at(&self, s: &str, timestamp: Timestamp) -> String {
        let api_key = self.api_key.expose();
        let api_secret = self.api_secret.expose();
        let message = format!("{timestamp}{}{}{s}", api_key, self.recv_window);

        hmac_sha256(api_secret, message)
    }
}

/// Return milliseconds.
pub fn timestamp() -> Timestamp {
    std::time::UNIX_EPOCH.elapsed().unwrap().as_millis() as Timestamp
}

pub fn create_stream_signature(expires: Timestamp, api_secret: SensitiveString) -> String {
    let api_secret = api_secret.expose();
    let message = format!("GET/realtime{expires}");
    hmac_sha256(api_secret, message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sign_get_request() {
        let api_key = SensitiveString("API_KEY".to_string());
        let api_secret = SensitiveString("API_SECRET".to_string());
        let recv_window = 5000;
        let signer = Signer::new(api_key, api_secret, recv_window, Some(|| 1658384314791));
        let query = "category=option&symbol=BTC-29JUL22-25000-C";
        let expected = "8b050bb7b9d53a91c42e16a7aa94a485d7aad9c7d08023a6249a184331a52ae7";

        let (signature, timestamp) = signer.sign(query);

        assert_eq!(signature, expected);
        assert_eq!(timestamp.len(), 13);
    }

    #[test]
    fn sign_post_request() {
        let api_key = SensitiveString("API_KEY".to_string());
        let api_secret = SensitiveString("API_SECRET".to_string());
        let recv_window = 5000;
        let signer = Signer::new(api_key, api_secret, recv_window, Some(|| 1658385579423));
        let json = r#"{"category":"option"}"#;
        let expected = "1b9b318f05208c9113f2612b2a6d76ca29427e6d8148937c03d6505f8c00804c";

        let (signature, timestamp) = signer.sign(json);

        assert_eq!(signature, expected);
        assert_eq!(timestamp.len(), 13);
    }

    #[test]
    fn sign_at_matches_sign_with_the_same_time() {
        let signer = Signer::new(
            "API_KEY".into(),
            "API_SECRET".into(),
            5000,
            Some(|| 1658384314791),
        );
        let query = "category=option&symbol=BTC-29JUL22-25000-C";

        let (signature, _) = signer.sign(query);

        assert_eq!(signer.sign_at(query, 1658384314791), signature);
    }
}
