use serde::Serialize;

use crate::{SensitiveString, Timestamp, Topic, create_stream_signature, timestamp};

#[derive(Serialize, Debug)]
#[serde(tag = "op")]
pub enum OutgoingMessage {
    #[serde(rename = "subscribe")]
    Subscribe {
        #[serde(skip_serializing_if = "Option::is_none")]
        req_id: Option<String>,
        args: Vec<Topic>,
    },
    #[serde(rename = "unsubscribe")]
    Unsubscribe {
        #[serde(skip_serializing_if = "Option::is_none")]
        req_id: Option<String>,
        args: Vec<Topic>,
    },
    #[serde(rename = "auth")]
    Auth {
        #[serde(skip_serializing_if = "Option::is_none")]
        req_id: Option<String>,
        args: (String, Timestamp, String),
    },
    #[serde(rename = "ping")]
    Ping {
        #[serde(skip_serializing_if = "Option::is_none")]
        req_id: Option<String>,
    },
    #[serde(rename = "pong")]
    Pong {
        #[serde(skip_serializing_if = "Option::is_none")]
        req_id: Option<String>,
    },
}

pub fn create_outgoing_message_auth(
    api_key: SensitiveString,
    api_secret: SensitiveString,
    req_id: Option<String>,
    recv_window: Timestamp,
) -> OutgoingMessage {
    let api_key = api_key.expose().to_string();
    let expires = timestamp() + recv_window;

    let signature = create_stream_signature(expires, api_secret);

    OutgoingMessage::Auth {
        req_id,
        args: (api_key, expires, signature),
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    use super::*;

    #[test]
    fn test_serialize_outgoing_message_subscribe() {
        let msg = OutgoingMessage::Subscribe {
            req_id: Some(String::from("request_id")),
            args: vec![
                Topic::Ticker(String::from("BTCUSDT")),
                Topic::Order(Category::Linear),
            ],
        };
        let expected =
            r#"{"op":"subscribe","req_id":"request_id","args":["tickers.BTCUSDT","order.linear"]}"#;
        let serialized = serde_json::to_string(&msg).unwrap();
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_serialize_outgoing_message_unsubscribe() {
        let msg = OutgoingMessage::Unsubscribe {
            req_id: Some(String::from("request_id")),
            args: vec![Topic::Ticker(String::from("BTCUSDT"))],
        };
        let expected = r#"{"op":"unsubscribe","req_id":"request_id","args":["tickers.BTCUSDT"]}"#;
        let serialized = serde_json::to_string(&msg).unwrap();
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_serialize_outgoing_message_auth() {
        let msg = OutgoingMessage::Auth {
            req_id: Some(String::from("request_id")),
            args: (
                String::from("api_key"),
                1662350400000,
                String::from("signature"),
            ),
        };
        let expected =
            r#"{"op":"auth","req_id":"request_id","args":["api_key",1662350400000,"signature"]}"#;
        let serialized = serde_json::to_string(&msg).unwrap();
        assert_eq!(serialized, expected);
    }
}
