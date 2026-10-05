use serde::Serialize;

use crate::{
    SensitiveString, Timestamp, Topic, create_stream_signature,
    http::{
        AmendOrderBatchRequest, AmendOrderRequest, CancelOrderBatchRequest, CancelOrderRequest,
        PlaceOrderBatchRequest, PlaceOrderRequest,
    },
    timestamp,
};

/// Messages sent to a Bybit stream.
#[derive(Serialize, Debug)]
#[serde(tag = "op")]
pub enum OutgoingMessage {
    /// Subscribe to topics.
    #[serde(rename = "subscribe")]
    Subscribe {
        /// Client id echoed in the reply.
        #[serde(skip_serializing_if = "Option::is_none")]
        req_id: Option<String>,
        /// Topics.
        args: Vec<Topic>,
    },
    /// Unsubscribe from topics.
    #[serde(rename = "unsubscribe")]
    Unsubscribe {
        /// Client id echoed in the reply.
        #[serde(skip_serializing_if = "Option::is_none")]
        req_id: Option<String>,
        /// Topics.
        args: Vec<Topic>,
    },
    /// Authenticate a private stream (see [`create_outgoing_message_auth_at`]).
    #[serde(rename = "auth")]
    Auth {
        /// Client id echoed in the reply.
        #[serde(skip_serializing_if = "Option::is_none")]
        req_id: Option<String>,
        /// `(api_key, expires, signature)`.
        args: (String, Timestamp, String),
    },
    /// Application-level heartbeat.
    #[serde(rename = "ping")]
    Ping {
        /// Client id echoed in the reply.
        #[serde(skip_serializing_if = "Option::is_none")]
        req_id: Option<String>,
    },
    /// Reply to a server ping.
    #[serde(rename = "pong")]
    Pong {
        /// Client id echoed in the reply.
        #[serde(skip_serializing_if = "Option::is_none")]
        req_id: Option<String>,
    },
    /// Create an order on the order entry stream (`/v5/trade`).
    #[serde(rename = "order.create")]
    OrderCreate {
        /// Unique id (at most 36 characters) echoed in the reply.
        #[serde(rename = "reqId")]
        req_id: String,
        /// Timestamp and validity of the request.
        header: TradeHeader,
        /// Exactly one request.
        args: Vec<PlaceOrderRequest>,
    },
    /// Amend an order on the order entry stream.
    #[serde(rename = "order.amend")]
    OrderAmend {
        /// Unique id (at most 36 characters) echoed in the reply.
        #[serde(rename = "reqId")]
        req_id: String,
        /// Timestamp and validity of the request.
        header: TradeHeader,
        /// Exactly one request.
        args: Vec<AmendOrderRequest>,
    },
    /// Cancel an order on the order entry stream.
    #[serde(rename = "order.cancel")]
    OrderCancel {
        /// Unique id (at most 36 characters) echoed in the reply.
        #[serde(rename = "reqId")]
        req_id: String,
        /// Timestamp and validity of the request.
        header: TradeHeader,
        /// Exactly one request.
        args: Vec<CancelOrderRequest>,
    },
    /// Create orders in a batch on the order entry stream.
    #[serde(rename = "order.create-batch")]
    OrderCreateBatch {
        /// Unique id (at most 36 characters) echoed in the reply.
        #[serde(rename = "reqId")]
        req_id: String,
        /// Timestamp and validity of the request.
        header: TradeHeader,
        /// Exactly one batch.
        args: Vec<PlaceOrderBatchRequest>,
    },
    /// Amend orders in a batch on the order entry stream.
    #[serde(rename = "order.amend-batch")]
    OrderAmendBatch {
        /// Unique id (at most 36 characters) echoed in the reply.
        #[serde(rename = "reqId")]
        req_id: String,
        /// Timestamp and validity of the request.
        header: TradeHeader,
        /// Exactly one batch.
        args: Vec<AmendOrderBatchRequest>,
    },
    /// Cancel orders in a batch on the order entry stream.
    #[serde(rename = "order.cancel-batch")]
    OrderCancelBatch {
        /// Unique id (at most 36 characters) echoed in the reply.
        #[serde(rename = "reqId")]
        req_id: String,
        /// Timestamp and validity of the request.
        header: TradeHeader,
        /// Exactly one batch.
        args: Vec<CancelOrderBatchRequest>,
    },
}

/// `header` of an order entry request.
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct TradeHeader {
    /// Request time, milliseconds (use the server clock).
    #[serde(rename = "X-BAPI-TIMESTAMP")]
    pub timestamp: String,
    /// How long the request stays valid, milliseconds.
    #[serde(rename = "X-BAPI-RECV-WINDOW")]
    pub recv_window: String,
    /// Broker referer.
    #[serde(rename = "Referer", skip_serializing_if = "Option::is_none")]
    pub referer: Option<String>,
}

/// `auth` message for a private stream, valid for `recv_window` ms from now
/// by the local clock. With a skewed clock use
/// [`create_outgoing_message_auth_at`] and
/// [`Client::server_timestamp`](crate::http::Client::server_timestamp).
pub fn create_outgoing_message_auth(
    api_key: SensitiveString,
    api_secret: SensitiveString,
    req_id: Option<String>,
    recv_window: Timestamp,
) -> OutgoingMessage {
    create_outgoing_message_auth_at(api_key, api_secret, req_id, recv_window, timestamp())
}

/// `auth` message for a private stream, valid for `recv_window` ms from
/// `now` (milliseconds, ideally the server time).
pub fn create_outgoing_message_auth_at(
    api_key: SensitiveString,
    api_secret: SensitiveString,
    req_id: Option<String>,
    recv_window: Timestamp,
    now: Timestamp,
) -> OutgoingMessage {
    let api_key = api_key.expose().to_string();
    let expires = now + recv_window;

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
