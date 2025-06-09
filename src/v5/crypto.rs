use hex;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SensitiveString(String);

impl Display for SensitiveString {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "REDACTED")
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
/// use bybit::v5::hmac_sha256;
/// let signature = hmac_sha256(b"my-secret-key", b"important message");
/// println!("HMAC: {signature}");
/// ```
pub fn hmac_sha256(key: impl AsRef<[u8]>, message: impl AsRef<[u8]>) -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(key.as_ref()).unwrap();
    mac.update(message.as_ref());
    let mac = mac.finalize().into_bytes().to_vec();
    hex::encode(&mac)
}

type Timer = fn() -> u128;
pub struct Signer {
    api_key: SensitiveString,
    api_secret: SensitiveString,
    /// Milliseconds.
    recv_window: u64,
    timer: Timer,
}

impl Signer {
    /// Create Signer instance.
    pub fn new(
        api_key: SensitiveString,
        api_secret: SensitiveString,
        recv_window: u64,
        timer: Option<Timer>,
    ) -> Self {
        Self {
            api_key,
            api_secret,
            recv_window,
            timer: timer.unwrap_or(timestamp),
        }
    }

    /// return: (signature, timestamp)
    pub fn sign(&self, s: &str) -> (String, String) {
        let timestamp = (self.timer)().to_string();
        let api_key = self.api_key.expose();
        let api_secret = &self.api_secret.expose();
        let message = format!("{timestamp}{}{}{s}", api_key, &self.recv_window);

        let signature = hmac_sha256(api_secret, message);

        (signature, timestamp)
    }
}

/// Return milliseconds.
fn timestamp() -> u128 {
    std::time::UNIX_EPOCH.elapsed().unwrap().as_millis()
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

        let (signature, timestamp) = signer.sign(&json);

        assert_eq!(signature, expected);
        assert_eq!(timestamp.len(), 13);
    }
}
