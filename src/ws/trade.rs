//! Order entry over WebSocket (`/v5/trade`).

use std::{
    collections::HashMap,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    time::Duration,
};

use serde::de::DeserializeOwned;
use tokio::{
    sync::{mpsc, oneshot},
    time::timeout,
};

use crate::{
    ServerClock, Timestamp,
    http::{
        AmendOrderBatchRequest, AmendOrderBatchResult, AmendOrderRequest, AmendOrderResponse,
        CancelOrderBatchRequest, CancelOrderBatchResult, CancelOrderRequest, CancelOrderResponse,
        List, PlaceOrderBatchRequest, PlaceOrderBatchResult, PlaceOrderRequest, PlaceOrderResponse,
        RetExtInfo,
    },
    timestamp,
    ws::{
        Config, Error, Event, Handle, IncomingMessage, OutgoingMessage, Stream, TradeHeader,
        TradeReply, TradeReplyHeader,
    },
};

/// A successful reply of the order entry stream.
///
/// An accepted request only means Bybit took it: the order stream
/// (`Topic::Order*`) confirms the order status.
#[derive(Debug, Clone, PartialEq)]
pub struct TradeResponse<T> {
    /// Same as `result` of the REST endpoint.
    pub data: T,
    /// Per-item results of batch requests, in request order.
    pub ret_ext_info: Option<RetExtInfo>,
    /// Rate limit status of the op.
    pub header: Option<TradeReplyHeader>,
}

/// Places, amends and cancels orders over the order entry stream
/// (`/v5/trade`), matching replies to requests by `reqId`.
///
/// Built on [`Stream`]: it authenticates on every (re)connect with the
/// credentials and [`ServerClock`] of the [`Config`]; requests are accepted
/// once [`Event::Authenticated`] arrived. Requests in flight when the
/// connection drops fail with [`Error::ConnectionLost`] and are not resent:
/// check the order (e.g. by `orderLinkId`) before retrying.
///
/// The order entry stream is not available on demo trading; use testnet.
///
/// ```no_run
/// use bybit::{
///     Category, Environment, OrderType, Side,
///     http::PlaceOrderRequest,
///     ws::{self, TradeClient},
/// };
/// use rust_decimal::dec;
///
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let config = ws::Config::trade(Environment::Testnet)?.credentials("API_KEY", "API_SECRET");
/// let (trade, mut events) = TradeClient::new(config)?;
/// trade.connect().await?;
/// while let Some(event) = events.recv().await {
///     if matches!(event, ws::Event::Authenticated) {
///         break;
///     }
/// }
///
/// let mut order = PlaceOrderRequest::new(
///     Category::Linear,
///     "BTCUSDT",
///     Side::Buy,
///     OrderType::Limit,
///     dec!(0.001),
/// );
/// order.price = Some(dec!(10000));
/// let reply = trade.place_order(&order).await?;
/// println!("order id: {}", reply.data.order_id);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct TradeClient {
    handle: Handle,
    shared: Arc<Shared>,
    clock: ServerClock,
    recv_window: Timestamp,
    request_timeout: Duration,
}

#[derive(Debug, Default)]
struct Shared {
    /// Waiting requests by `reqId`.
    pending: Mutex<HashMap<String, oneshot::Sender<TradeReply>>>,
    /// Authenticated on the current connection.
    ready: AtomicBool,
    next_id: AtomicU64,
    /// Prefix that keeps `reqId`s unique across clients and restarts.
    id_prefix: String,
}

impl Shared {
    /// Drop every waiting request; their callers get `ConnectionLost`.
    fn fail_pending(&self) {
        self.pending.lock().unwrap().clear();
    }
}

impl TradeClient {
    /// Spawn the stream driver (see [`Stream::new`]) and the reply router.
    /// Returns the client and the events that are not replies to its
    /// requests. Fails without credentials: the order entry stream requires
    /// authentication.
    pub fn new(config: Config) -> Result<(Self, mpsc::Receiver<Event>), Error> {
        if config.api_key.is_none() || config.api_secret.is_none() {
            return Err(Error::MissingCredentials);
        }
        let clock = config.server_clock.clone();
        let recv_window = config.auth_recv_window;
        let request_timeout = config.request_timeout;
        let event_queue_size = config.event_queue_size;

        let shared = Arc::new(Shared {
            id_prefix: format!("t{:x}", timestamp()),
            ..Default::default()
        });
        let (handle, driver_events) = Stream::new(config);
        let (tx, rx) = mpsc::channel(event_queue_size);
        tokio::spawn(route(driver_events, tx, shared.clone()));

        let client = Self {
            handle,
            shared,
            clock,
            recv_window,
            request_timeout,
        };
        Ok((client, rx))
    }

    /// The command handle of the underlying stream.
    pub fn handle(&self) -> &Handle {
        &self.handle
    }

    /// Open the connection; requests are accepted after
    /// [`Event::Authenticated`].
    pub async fn connect(&self) -> Result<(), Error> {
        self.handle.connect().await
    }

    /// Close the connection.
    pub async fn disconnect(&self) -> Result<(), Error> {
        self.handle.disconnect().await
    }

    /// `true` while connected and authenticated.
    pub fn is_ready(&self) -> bool {
        self.shared.ready.load(Ordering::Relaxed)
    }

    /// Create an order (`order.create`).
    pub async fn place_order(
        &self,
        request: &PlaceOrderRequest,
    ) -> Result<TradeResponse<PlaceOrderResponse>, Error> {
        self.call(|req_id, header| OutgoingMessage::OrderCreate {
            req_id,
            header,
            args: vec![request.clone()],
        })
        .await
    }

    /// Amend an order (`order.amend`).
    pub async fn amend_order(
        &self,
        request: &AmendOrderRequest,
    ) -> Result<TradeResponse<AmendOrderResponse>, Error> {
        self.call(|req_id, header| OutgoingMessage::OrderAmend {
            req_id,
            header,
            args: vec![request.clone()],
        })
        .await
    }

    /// Cancel an order (`order.cancel`).
    pub async fn cancel_order(
        &self,
        request: &CancelOrderRequest,
    ) -> Result<TradeResponse<CancelOrderResponse>, Error> {
        self.call(|req_id, header| OutgoingMessage::OrderCancel {
            req_id,
            header,
            args: vec![request.clone()],
        })
        .await
    }

    /// Create orders in a batch (`order.create-batch`); per-order results are
    /// in [`TradeResponse::ret_ext_info`].
    pub async fn place_order_batch(
        &self,
        request: &PlaceOrderBatchRequest,
    ) -> Result<TradeResponse<List<PlaceOrderBatchResult>>, Error> {
        self.call(|req_id, header| OutgoingMessage::OrderCreateBatch {
            req_id,
            header,
            args: vec![request.clone()],
        })
        .await
    }

    /// Amend orders in a batch (`order.amend-batch`).
    pub async fn amend_order_batch(
        &self,
        request: &AmendOrderBatchRequest,
    ) -> Result<TradeResponse<List<AmendOrderBatchResult>>, Error> {
        self.call(|req_id, header| OutgoingMessage::OrderAmendBatch {
            req_id,
            header,
            args: vec![request.clone()],
        })
        .await
    }

    /// Cancel orders in a batch (`order.cancel-batch`).
    pub async fn cancel_order_batch(
        &self,
        request: &CancelOrderBatchRequest,
    ) -> Result<TradeResponse<List<CancelOrderBatchResult>>, Error> {
        self.call(|req_id, header| OutgoingMessage::OrderCancelBatch {
            req_id,
            header,
            args: vec![request.clone()],
        })
        .await
    }

    async fn call<T: DeserializeOwned>(
        &self,
        build: impl FnOnce(String, TradeHeader) -> OutgoingMessage,
    ) -> Result<TradeResponse<T>, Error> {
        if !self.is_ready() {
            return Err(Error::NotConnected);
        }
        let n = self.shared.next_id.fetch_add(1, Ordering::Relaxed);
        let req_id = format!("{}-{n}", self.shared.id_prefix);
        let (tx, rx) = oneshot::channel();
        self.shared
            .pending
            .lock()
            .unwrap()
            .insert(req_id.clone(), tx);

        let header = TradeHeader {
            timestamp: self.clock.now().to_string(),
            recv_window: self.recv_window.to_string(),
            referer: None,
        };
        if let Err(e) = self
            .handle
            .send_command(build(req_id.clone(), header))
            .await
        {
            self.shared.pending.lock().unwrap().remove(&req_id);
            return Err(e);
        }

        let reply = match timeout(self.request_timeout, rx).await {
            Ok(Ok(reply)) => reply,
            Ok(Err(_)) => return Err(Error::ConnectionLost),
            Err(_) => {
                self.shared.pending.lock().unwrap().remove(&req_id);
                return Err(Error::Timeout);
            }
        };
        if reply.ret_code != 0 {
            return Err(Error::Api {
                code: reply.ret_code,
                msg: reply.ret_msg,
            });
        }
        let data = serde_json::from_value(reply.data.unwrap_or_default())
            .map_err(|e| Error::InvalidReply(e.to_string()))?;
        let ret_ext_info = reply
            .ret_ext_info
            .and_then(|value| serde_json::from_value(value).ok());
        Ok(TradeResponse {
            data,
            ret_ext_info,
            header: reply.header,
        })
    }
}

/// Deliver replies to waiting requests and forward every other event.
async fn route(
    mut driver_events: mpsc::Receiver<Event>,
    tx: mpsc::Sender<Event>,
    shared: Arc<Shared>,
) {
    while let Some(event) = driver_events.recv().await {
        match &event {
            Event::Authenticated => shared.ready.store(true, Ordering::Relaxed),
            Event::Connected
            | Event::Reconnecting { .. }
            | Event::Disconnected { .. }
            | Event::AuthFailed { .. } => {
                shared.ready.store(false, Ordering::Relaxed);
                shared.fail_pending();
            }
            Event::Message(IncomingMessage::TradeReply(reply)) => {
                let waiting = reply
                    .req_id
                    .as_ref()
                    .and_then(|id| shared.pending.lock().unwrap().remove(id));
                if let Some(waiting) = waiting {
                    let _ = waiting.send((**reply).clone());
                    continue;
                }
            }
            _ => {}
        }
        // A dropped receiver must not stop the routing of replies.
        let _ = tx.send(event).await;
    }
    shared.ready.store(false, Ordering::Relaxed);
    shared.fail_pending();
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use futures_util::{SinkExt, StreamExt};
    use rust_decimal::dec;
    use tokio::net::TcpListener;
    use tokio_tungstenite::tungstenite::Message;

    use super::*;
    use crate::{Category, OrderType, Side};

    /// How the fake order entry stream answers `order.*` requests.
    #[derive(Clone, Copy)]
    enum Answer {
        Accept,
        Reject,
        Silent,
        /// Close the connection instead of answering.
        Drop,
    }

    /// Received request frames.
    type Received = Arc<Mutex<Vec<serde_json::Value>>>;

    /// Local server speaking the order entry protocol: `auth` succeeds,
    /// `ping` gets an order-entry style `pong`, `order.*` per `answer`.
    async fn spawn_trade_server(answer: Answer) -> (String, Received) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let received: Received = Default::default();
        let recorded = received.clone();
        tokio::spawn(async move {
            while let Ok((tcp, _)) = listener.accept().await {
                let recorded = recorded.clone();
                tokio::spawn(async move {
                    let Ok(mut ws) = tokio_tungstenite::accept_async(tcp).await else {
                        return;
                    };
                    while let Some(Ok(Message::Text(text))) = ws.next().await {
                        let json: serde_json::Value = serde_json::from_str(&text).unwrap();
                        recorded.lock().unwrap().push(json.clone());
                        let op = json["op"].as_str().unwrap_or_default().to_owned();
                        let reply = match op.as_str() {
                            "auth" => serde_json::json!({
                                "retCode": 0, "retMsg": "OK", "op": "auth", "connId": "c"
                            }),
                            "ping" => serde_json::json!({
                                "retCode": 0, "retMsg": "OK", "op": "pong", "connId": "c"
                            }),
                            _ => match answer {
                                Answer::Accept => serde_json::json!({
                                    "reqId": json["reqId"], "retCode": 0, "retMsg": "OK",
                                    "op": op, "data": {"orderId": "order-1", "orderLinkId": ""},
                                    "retExtInfo": {},
                                    "header": {"X-Bapi-Limit": "10", "X-Bapi-Limit-Status": "9"},
                                    "connId": "c"
                                }),
                                Answer::Reject => serde_json::json!({
                                    "reqId": json["reqId"], "retCode": 10001,
                                    "retMsg": "params error", "op": op, "connId": "c"
                                }),
                                Answer::Silent => continue,
                                Answer::Drop => return,
                            },
                        };
                        if ws
                            .send(Message::Text(reply.to_string().into()))
                            .await
                            .is_err()
                        {
                            return;
                        }
                    }
                });
            }
        });
        (format!("ws://{addr}"), received)
    }

    fn config(url: String) -> Config {
        Config::new(url)
            .credentials("key", "secret")
            .ping_interval(None)
            .max_reconnect_attempts(0)
            .request_timeout(Duration::from_millis(300))
    }

    /// Connect and wait until requests are accepted.
    async fn connected(config: Config) -> (TradeClient, mpsc::Receiver<Event>) {
        let (trade, mut events) = TradeClient::new(config).unwrap();
        trade.connect().await.unwrap();
        timeout(Duration::from_secs(10), async {
            while let Some(event) = events.recv().await {
                if matches!(event, Event::Authenticated) {
                    return;
                }
            }
        })
        .await
        .expect("not authenticated");
        assert!(trade.is_ready());
        (trade, events)
    }

    fn order() -> PlaceOrderRequest {
        let mut order = PlaceOrderRequest::new(
            Category::Linear,
            "BTCUSDT",
            Side::Buy,
            OrderType::Limit,
            dec!(0.001),
        );
        order.price = Some(dec!(10000));
        order
    }

    #[tokio::test]
    async fn accepted_order_returns_the_reply_data() {
        let (url, received) = spawn_trade_server(Answer::Accept).await;
        let (trade, _events) = connected(config(url)).await;

        let reply = trade.place_order(&order()).await.unwrap();

        assert_eq!(reply.data.order_id, "order-1");
        assert_eq!(reply.header.unwrap().limit_status, Some(9));
        let request = received.lock().unwrap().last().cloned().unwrap();
        assert_eq!(request["op"], "order.create");
        assert_eq!(request["args"][0]["symbol"], "BTCUSDT");
        assert_eq!(request["header"]["X-BAPI-RECV-WINDOW"], "5000");
        let sent: i64 = request["header"]["X-BAPI-TIMESTAMP"]
            .as_str()
            .unwrap()
            .parse()
            .unwrap();
        assert!((sent - timestamp() as i64).abs() < 1000);
        assert!(request["reqId"].as_str().unwrap().len() <= 36);
    }

    #[tokio::test]
    async fn concurrent_requests_get_their_own_replies() {
        let (url, received) = spawn_trade_server(Answer::Accept).await;
        let (trade, _events) = connected(config(url)).await;

        let order = order();
        let (a, b) = tokio::join!(trade.place_order(&order), trade.place_order(&order));

        assert!(a.is_ok() && b.is_ok());
        let ids: Vec<_> = received
            .lock()
            .unwrap()
            .iter()
            .filter(|r| r["op"] == "order.create")
            .map(|r| r["reqId"].as_str().unwrap().to_owned())
            .collect();
        assert_eq!(ids.len(), 2);
        assert_ne!(ids[0], ids[1]);
    }

    #[tokio::test]
    async fn rejected_order_is_an_api_error() {
        let (url, _) = spawn_trade_server(Answer::Reject).await;
        let (trade, _events) = connected(config(url)).await;

        let err = trade.place_order(&order()).await.unwrap_err();

        assert!(matches!(err, Error::Api { code: 10001, .. }), "{err:?}");
    }

    #[tokio::test]
    async fn missing_reply_times_out() {
        let (url, _) = spawn_trade_server(Answer::Silent).await;
        let (trade, _events) = connected(config(url)).await;

        let err = trade.place_order(&order()).await.unwrap_err();

        assert!(matches!(err, Error::Timeout), "{err:?}");
        assert!(trade.shared.pending.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn lost_connection_fails_the_waiting_request() {
        let (url, _) = spawn_trade_server(Answer::Drop).await;
        let (trade, _events) =
            connected(config(url).request_timeout(Duration::from_secs(10))).await;

        let err = trade.place_order(&order()).await.unwrap_err();

        assert!(matches!(err, Error::ConnectionLost), "{err:?}");
        assert!(!trade.is_ready());
    }

    #[tokio::test]
    async fn requests_before_authentication_are_refused() {
        let (url, _) = spawn_trade_server(Answer::Accept).await;
        let (trade, _events) = TradeClient::new(config(url)).unwrap();

        let err = trade.place_order(&order()).await.unwrap_err();

        assert!(matches!(err, Error::NotConnected), "{err:?}");
    }

    #[tokio::test]
    async fn credentials_are_required() {
        let result = TradeClient::new(Config::new("ws://127.0.0.1:1"));

        assert!(matches!(result, Err(Error::MissingCredentials)));
    }

    #[tokio::test]
    async fn order_entry_heartbeat_keeps_the_connection() {
        let (url, received) = spawn_trade_server(Answer::Accept).await;
        let config = config(url)
            .ping_interval(Some(Duration::from_millis(50)))
            .pong_timeout(Duration::from_millis(200));
        let (_trade, mut events) = connected(config).await;

        tokio::time::sleep(Duration::from_millis(600)).await;

        let pings = received
            .lock()
            .unwrap()
            .iter()
            .filter(|r| r["op"] == "ping")
            .count();
        assert!(pings >= 3, "only {pings} pings");
        while let Ok(event) = events.try_recv() {
            assert!(
                !matches!(
                    event,
                    Event::Reconnecting { .. } | Event::Disconnected { .. }
                ),
                "{event:?}"
            );
        }
    }
}
