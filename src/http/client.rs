use crate::{
    Error, Timestamp,
    crypto::{SensitiveString, Signer, timestamp},
    enums::Category,
    http::{
        APIErrorResponse, APIKeyInformation, AccountCoinBalance, AccountInfo,
        AmendOrderBatchRequest, AmendOrderBatchResult, AmendOrderRequest, AmendOrderResponse,
        AssetInfoResult, BorrowHistoryEntry, CancelAllOrdersRequest, CancelAllOrdersResponse,
        CancelOrderBatchRequest, CancelOrderBatchResult, CancelOrderRequest, CancelOrderResponse,
        CancelWithdrawalRequest, CancelWithdrawalResult, ClosedPnl, CoinGreeks, CoinInfoResult,
        CollateralInfoEntry, CursorPagination, DeliveryPrice, DeliveryRecord, DepositAddress,
        DepositAllowedCoinInfoResult, DepositRecords, EmptyResult, ExchangeOrderRecords,
        ExecutionEntry, FeeRateEntry, FundingRateHistory, GetAccountCoinBalanceParams,
        GetAssetInfoParams, GetBorrowHistoryParams, GetClosedPnlParams, GetCoinGreeksParams,
        GetCoinInfoParams, GetCollateralInfoParams, GetDeliveryPriceParams,
        GetDeliveryRecordParams, GetDepositAddressParams, GetDepositAllowedCoinInfoParams,
        GetDepositRecordsParams, GetExchangeOrderRecordParams, GetExecutionListParams,
        GetFeeRateParams, GetFundingRateHistoryParams, GetHistoricalVolatilityParams,
        GetInstrumentsInfoParams, GetInsuranceParams, GetInternalTransferRecordsParams,
        GetKLinesParams, GetLeverageTokenInfoParams, GetLeverageTokenMarketParams,
        GetLeverageTokenOrderRecordsParams, GetOpenClosedOrdersParams, GetOpenInterestParams,
        GetOrderHistoryParams, GetOrderbookParams, GetPositionInfoParams, GetRiskLimitParams,
        GetSettlementRecordParams, GetSpotBorrowCheckParams, GetSubDepositAddressParams,
        GetSubDepositRecordsParams, GetTickersParams, GetTradesParams, GetTransactionLogParams,
        GetTransferableCoinsParams, GetUniversalTransferRecordsParams, GetWalletBalanceParams,
        GetWithdrawableAmountParams, GetWithdrawalRecordsParams, Headers,
        HistoricalVolatilityEntry, InstrumentsInfo, Insurance, InternalTransferEntry,
        InternalTransferRequest, KLine, LeverageTokenInfo, LeverageTokenMarket,
        LeverageTokenOrderRecord, List, OpenInterest, Order, Orderbook, PlaceOrderBatchRequest,
        PlaceOrderBatchResult, PlaceOrderRequest, PlaceOrderResponse, Position,
        PurchaseLeverageTokenRequest, PurchaseLeverageTokenResult, RedeemLeverageTokenRequest,
        RedeemLeverageTokenResult, Resp, Response, RiskLimit, SaveTransferSubMemberRequest,
        ServerTime, SetAutoAddMarginRequest, SetLeverageRequest, SetMarginModeRequest,
        SetMarginModeResponse, SetRiskLimitRequest, SetRiskLimitResponse,
        SetSpotMarginLeverageRequest, SetTradingStopRequest, SettlementRecord, SpotBorrowCheck,
        SwitchCrossIsolatedMarginRequest, SwitchPositionModeRequest, SwitchSpotMarginModeRequest,
        SwitchSpotMarginModeResult, Ticker, Trade, TransactionLog, TransferResult,
        TransferableSubMembers, UniversalTransferEntry, UniversalTransferRequest,
        UpgradeToUtaResult, WalletBalance, WithdrawRecords, WithdrawRequest, WithdrawResult,
        WithdrawableAmountResult,
    },
    serde::{deserialize_json, serialize_json, serialize_query},
    url::*,
};
use reqwest::{
    self, Method, RequestBuilder, StatusCode,
    header::{CONTENT_TYPE, HeaderMap, HeaderValue},
};
use std::{
    sync::{
        Arc,
        atomic::{AtomicI64, Ordering},
    },
    time::Duration,
};

use super::rate_limiter::{RateLimitKey, RateLimiter, RateLimiterConfig, Rejection};

/// Default total timeout of one HTTP request.
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);
/// Default timeout of establishing a TCP/TLS connection.
pub const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(5);
/// Default `X-BAPI-RECV-WINDOW`, milliseconds.
pub const DEFAULT_RECV_WINDOW: Timestamp = 5000;

/// Maximum number of bytes of a non-2xx response body kept in [`Error::Http`].
const MAX_ERROR_BODY_LEN: usize = 1024;

pub struct Config {
    pub base_url: String,
    pub api_key: Option<SensitiveString>,
    pub api_secret: Option<SensitiveString>,
    /// Milliseconds.
    pub recv_window: Timestamp,
    /// HTTP the header for broker users only.
    pub referer: Option<String>,
    /// Optional rate limiter. When `Some`, every request is pre-flight-checked
    /// against a local token bucket and the last server-reported limit status.
    /// Set to `None` to disable rate limiting entirely.
    pub rate_limiter: Option<RateLimiterConfig>,
    /// Total timeout of one request (connect + send + receive).
    /// `None` disables it, which is not recommended: a hung request would
    /// then wait forever.
    pub timeout: Option<Duration>,
    /// Timeout of establishing a connection. `None` disables it.
    pub connect_timeout: Option<Duration>,
}

impl Config {
    /// Config for public endpoints with default timeouts and no rate limiter.
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            api_key: None,
            api_secret: None,
            recv_window: DEFAULT_RECV_WINDOW,
            referer: None,
            rate_limiter: None,
            timeout: Some(DEFAULT_TIMEOUT),
            connect_timeout: Some(DEFAULT_CONNECT_TIMEOUT),
        }
    }

    /// Credentials required by private endpoints.
    pub fn credentials(
        mut self,
        api_key: impl Into<SensitiveString>,
        api_secret: impl Into<SensitiveString>,
    ) -> Self {
        self.api_key = Some(api_key.into());
        self.api_secret = Some(api_secret.into());
        self
    }

    /// Milliseconds.
    pub fn recv_window(mut self, recv_window: Timestamp) -> Self {
        self.recv_window = recv_window;
        self
    }

    pub fn referer(mut self, referer: impl Into<String>) -> Self {
        self.referer = Some(referer.into());
        self
    }

    pub fn rate_limiter(mut self, rate_limiter: RateLimiterConfig) -> Self {
        self.rate_limiter = Some(rate_limiter);
        self
    }

    pub fn timeout(mut self, timeout: Option<Duration>) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn connect_timeout(mut self, connect_timeout: Option<Duration>) -> Self {
        self.connect_timeout = connect_timeout;
        self
    }
}

/// Bybit V5 REST client.
///
/// Cloning is cheap, and clones share the connection pool, the rate limiter
/// state and the clock offset measured by [`Client::sync_time`]: clone the
/// client into tasks instead of wrapping it in `Arc`.
// TODO: use proxy
#[derive(Debug, Clone)]
pub struct Client {
    base_url: String,
    headers: HeaderMap,
    client: reqwest::Client,
    signer: Option<Arc<Signer>>,
    rate_limiter: Option<Arc<RateLimiter>>,
    /// Server clock minus local clock, milliseconds (see [`Client::sync_time`]).
    time_offset_ms: Arc<AtomicI64>,
}

impl Client {
    pub fn new(cfg: Config) -> Result<Self, Error> {
        let mut headers = HeaderMap::new();

        if let Some(api_key) = cfg.api_key.as_ref() {
            let api_key = api_key.expose().parse()?;
            headers.append(HEADER_X_BAPI_API_KEY, api_key);
        }

        let recv_window = cfg.recv_window.to_string().parse()?;
        headers.append(HEADER_X_BAPI_RECV_WINDOW, recv_window);

        if let Some(referer) = cfg.referer {
            let referer = referer.parse()?;
            headers.append(HEADER_X_REFERER, referer);
        }

        let signer = cfg
            .api_secret
            .map(|api_secret| -> Result<Arc<Signer>, Error> {
                let api_key = cfg
                    .api_key
                    .ok_or_else(|| Error::from("api_key is required when api_secret is set"))?;
                Ok(Arc::new(Signer::new(
                    api_key,
                    api_secret,
                    cfg.recv_window,
                    None,
                )))
            })
            .transpose()?;

        let rate_limiter = cfg
            .rate_limiter
            .map(|config| Arc::new(RateLimiter::new(config)));

        // Bybit requires `Content-Type: application/json` for POST bodies;
        // it is harmless on GET requests.
        let mut default_headers = HeaderMap::new();
        default_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        let mut builder = reqwest::Client::builder().default_headers(default_headers);
        if let Some(timeout) = cfg.timeout {
            builder = builder.timeout(timeout);
        }
        if let Some(connect_timeout) = cfg.connect_timeout {
            builder = builder.connect_timeout(connect_timeout);
        }

        Ok(Self {
            base_url: cfg.base_url,
            headers,
            client: builder.build()?,
            signer,
            rate_limiter,
            time_offset_ms: Arc::new(AtomicI64::new(0)),
        })
    }

    /// Measure the offset between the Bybit server clock and the local clock
    /// and use it for all signed requests from now on. Returns the offset in
    /// milliseconds (positive: the local clock is behind).
    ///
    /// Bybit rejects signed requests whose timestamp is not within
    /// `[server_time - recv_window, server_time + 1000)` (`retCode` 10002).
    /// Call this after creating the client, periodically in long-running
    /// programs (clocks drift) and after a 10002 error. The offset is
    /// estimated as `server_time - (sent_at + received_at) / 2`, so its error
    /// is at most half the round trip.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn sync_time(&self) -> Result<i64, Error> {
        let sent_at = timestamp();
        let response = self.get_server_time().await?;
        let received_at = timestamp();

        let server_ms = response.result.time_nano / 1_000_000;
        let offset = estimate_time_offset(sent_at, received_at, server_ms);
        self.time_offset_ms.store(offset, Ordering::Relaxed);
        tracing::debug!(offset_ms = offset, "server time synchronized");
        Ok(offset)
    }

    /// Offset applied to signed requests: server clock minus local clock,
    /// milliseconds. `0` until [`Client::sync_time`] is called.
    pub fn time_offset(&self) -> i64 {
        self.time_offset_ms.load(Ordering::Relaxed)
    }

    /// Current server time estimated from the local clock and
    /// [`Client::time_offset`], milliseconds. Use it for the WebSocket
    /// `auth` message ([`create_outgoing_message_auth_at`](crate::ws::create_outgoing_message_auth_at)).
    pub fn server_timestamp(&self) -> Timestamp {
        timestamp().saturating_add_signed(self.time_offset())
    }

    fn get_signed_headers(&self, s: &str) -> Result<HeaderMap, Error> {
        let signer = self.signer.as_ref().ok_or(Error::MissingCredentials)?;
        let mut headers = self.headers.clone();

        let timestamp = self.server_timestamp();
        let signature = signer.sign_at(s, timestamp);
        headers.append(HEADER_X_BAPI_SIGN, signature.parse()?);
        headers.append(HEADER_X_BAPI_TIMESTAMP, timestamp.to_string().parse()?);

        Ok(headers)
    }
}

/// Server clock minus local clock, assuming the server read its clock halfway
/// between `sent_at` and `received_at` (local milliseconds).
fn estimate_time_offset(sent_at: Timestamp, received_at: Timestamp, server_ms: Timestamp) -> i64 {
    let midpoint = sent_at + received_at.saturating_sub(sent_at) / 2;
    server_ms as i64 - midpoint as i64
}

// Market.
impl Client {
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_server_time(&self) -> Result<Response<ServerTime>, Error> {
        let url = format!("{}{}", self.base_url, Path::MarketServerTime);

        let request = self.client.request(Method::GET, url);

        let response = self.send(Path::MarketServerTime, request).await?;
        Ok(response)
    }

    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_kline(&self, params: &GetKLinesParams) -> Result<Response<KLine>, Error> {
        let url = format!("{}{}", self.base_url, Path::MarketKline);

        let request = self.client.request(Method::GET, url).query(params);

        let response = self.send(Path::MarketKline, request).await?;
        Ok(response)
    }

    /// Get Mark Price Kline.
    /// Query the mark price kline data. Charts are returned in groups based on the requested interval.
    ///
    /// Covers: linear / inverse
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_mark_price_kline(
        &self,
        params: &GetKLinesParams,
    ) -> Result<Response<KLine>, Error> {
        let url = format!("{}{}", self.base_url, Path::MarketMarkPriceKline);

        let request = self.client.request(Method::GET, url).query(params);

        let response = self.send(Path::MarketMarkPriceKline, request).await?;
        Ok(response)
    }

    /// Get Index Price Kline.
    /// Query the index price kline data. Charts are returned in groups based on the requested interval.
    ///
    /// Covers: linear / inverse
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_index_price_kline(
        &self,
        params: &GetKLinesParams,
    ) -> Result<Response<KLine>, Error> {
        let url = format!("{}{}", self.base_url, Path::MarketIndexPriceKline);

        let request = self.client.request(Method::GET, url).query(params);

        let response = self.send(Path::MarketIndexPriceKline, request).await?;
        Ok(response)
    }

    /// Get Premium Index Price Kline.
    /// Retrieve the premium index price kline data. Charts are returned in groups based on the requested interval.
    ///
    /// Covers: linear
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_premium_index_price_kline(
        &self,
        params: &GetKLinesParams,
    ) -> Result<Response<KLine>, Error> {
        let url = format!("{}{}", self.base_url, Path::MarketPremiumIndexPriceKline);

        let request = self.client.request(Method::GET, url).query(params);

        let response = self
            .send(Path::MarketPremiumIndexPriceKline, request)
            .await?;
        Ok(response)
    }

    /// Get Tickers
    /// Query for the latest price snapshot, best bid/ask price, and trading volume in the last 24 hours.
    /// If category=option, symbol or baseCoin must be passed.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_tickers(&self, params: &GetTickersParams) -> Result<Response<Ticker>, Error> {
        let url = format!("{}{}", self.base_url, Path::MarketTickers);

        let request = self.client.request(Method::GET, url).query(params);

        let response = self.send(Path::MarketTickers, request).await?;
        Ok(response)
    }

    /// Get Orderbook.
    /// Query for orderbook depth data.
    ///
    /// Covers: Spot / USDT contract / USDC contract / Inverse contract / Option
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_orderbook(
        &self,
        params: &GetOrderbookParams,
    ) -> Result<Response<Orderbook>, Error> {
        let url = format!("{}{}", self.base_url, Path::MarketOrderbook);

        let request = self.client.request(Method::GET, url).query(params);

        let response = self.send(Path::MarketOrderbook, request).await?;
        Ok(response)
    }

    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_instruments_info(
        &self,
        params: &GetInstrumentsInfoParams,
    ) -> Result<Response<InstrumentsInfo>, Error> {
        let url = format!("{}{}", self.base_url, Path::MarketInstrumentsInfo);

        let request = self.client.request(Method::GET, url).query(params);

        let response = self.send(Path::MarketInstrumentsInfo, request).await?;
        Ok(response)
    }

    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_public_recent_trading_history(
        &self,
        params: &GetTradesParams,
    ) -> Result<Response<Trade>, Error> {
        let url = format!("{}{}", self.base_url, Path::MarketRecentTrade);

        let request = self.client.request(Method::GET, url).query(params);

        let response = self.send(Path::MarketRecentTrade, request).await?;
        Ok(response)
    }

    /// Get Funding Rate History.
    /// Query for historical funding rates. Each request returns up to 200 rows of data.
    ///
    /// Covers: linear / inverse
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_funding_rate_history(
        &self,
        params: &GetFundingRateHistoryParams,
    ) -> Result<Response<FundingRateHistory>, Error> {
        let url = format!("{}{}", self.base_url, Path::MarketFundingHistory);

        let request = self.client.request(Method::GET, url).query(params);

        let response = self.send(Path::MarketFundingHistory, request).await?;
        Ok(response)
    }

    /// Get Open Interest.
    /// Get the open interest of each symbol.
    ///
    /// Covers: linear / inverse
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_open_interest(
        &self,
        params: &GetOpenInterestParams,
    ) -> Result<Response<OpenInterest>, Error> {
        let url = format!("{}{}", self.base_url, Path::MarketOpenInterest);

        let request = self.client.request(Method::GET, url).query(params);

        let response = self.send(Path::MarketOpenInterest, request).await?;
        Ok(response)
    }

    /// Get Historical Volatility.
    /// Query option historical volatility.
    ///
    /// Covers: option only
    ///
    /// Note: the result is returned as a top-level JSON array, so the response
    /// wraps `Vec<HistoricalVolatilityEntry>` directly.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_historical_volatility(
        &self,
        params: &GetHistoricalVolatilityParams,
    ) -> Result<Response<Vec<HistoricalVolatilityEntry>>, Error> {
        let url = format!("{}{}", self.base_url, Path::MarketHistoricalVolatility);

        let request = self.client.request(Method::GET, url).query(params);

        let response = self.send(Path::MarketHistoricalVolatility, request).await?;
        Ok(response)
    }

    /// Get Insurance.
    /// Query for Bybit insurance pool data (1 day delay).
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_insurance(
        &self,
        params: &GetInsuranceParams,
    ) -> Result<Response<Insurance>, Error> {
        let url = format!("{}{}", self.base_url, Path::MarketInsurance);

        let request = self.client.request(Method::GET, url).query(params);

        let response = self.send(Path::MarketInsurance, request).await?;
        Ok(response)
    }

    /// Get Risk Limit.
    /// Query for the risk limit.
    ///
    /// Covers: linear / inverse
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_risk_limit(
        &self,
        params: &GetRiskLimitParams,
    ) -> Result<Response<RiskLimit>, Error> {
        let url = format!("{}{}", self.base_url, Path::MarketRiskLimit);

        let request = self.client.request(Method::GET, url).query(params);

        let response = self.send(Path::MarketRiskLimit, request).await?;
        Ok(response)
    }

    /// Get Delivery Price.
    /// Get the delivery price for option and USDC futures contracts.
    ///
    /// Covers: linear / inverse / option
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_delivery_price(
        &self,
        params: &GetDeliveryPriceParams,
    ) -> Result<Response<DeliveryPrice>, Error> {
        let url = format!("{}{}", self.base_url, Path::MarketDeliveryPrice);

        let request = self.client.request(Method::GET, url).query(params);

        let response = self.send(Path::MarketDeliveryPrice, request).await?;
        Ok(response)
    }
}

// Trade.
impl Client {
    /// Place Order
    /// This endpoint supports to create the order for Spot, Margin trading, USDT perpetual, USDT futures, USDC perpetual, USDC futures, Inverse Futures and Options.
    ///
    /// INFO:
    /// Supported order type (orderType):
    /// Limit order: orderType=Limit, it is necessary to specify order qty and price.
    ///
    /// Market order: orderType=Market, execute at the best price in the Bybit market until the transaction is completed. When selecting a market order, the "price" can be empty. In the futures trading system, in order to protect traders against the serious slippage of the Market order, Bybit trading engine will convert the market order into an IOC limit order for matching. If there are no orderbook entries within price slippage limit, the order will not be executed. If there is insufficient liquidity, the order will be cancelled. The slippage threshold refers to the percentage that the order price deviates from the mark price. You can learn more here: Adjustments to Bybit's Derivative Trading Price Limit Mechanism
    /// Supported timeInForce strategy:
    /// GTC
    /// IOC
    /// FOK
    /// PostOnly: If the order would be filled immediately when submitted, it will be cancelled. The purpose of this is to protect your order during the submission process. If the matching system cannot entrust the order to the order book due to price changes on the market, it will be cancelled.
    /// RPI: Retail Price Improvement order. Assigned market maker can place this kind of order, and it is a post only order, only match with the order from Web or APP.
    ///
    /// How to create a conditional order:
    /// When submitting an order, if triggerPrice is set, the order will be automatically converted into a conditional order. In addition, the conditional order does not occupy the margin. If the margin is insufficient after the conditional order is triggered, the order will be cancelled.
    ///
    /// Take profit / Stop loss: You can set TP/SL while placing orders. Besides, you could modify the position's TP/SL.
    ///
    /// Order quantity: The quantity of perpetual contracts you are going to buy/sell. For the order quantity, Bybit only supports positive number at present.
    ///
    /// Order price: Place a limit order, this parameter is required. If you have position, the price should be higher than the liquidation price. For the minimum unit of the price change, please refer to the priceFilter > tickSize field in the instruments-info endpoint.
    ///
    /// orderLinkId: You can customize the active order ID. We can link this ID to the order ID in the system. Once the active order is successfully created, we will send the unique order ID in the system to you. Then, you can use this order ID to cancel active orders, and if both orderId and orderLinkId are entered in the parameter input, Bybit will prioritize the orderId to process the corresponding order. Meanwhile, your customized order ID should be no longer than 36 characters and should be unique.
    ///
    /// Open orders up limit:
    /// Perps & Futures:
    /// a) Each account can hold a maximum of 500 active orders simultaneously per symbol.
    /// b) conditional orders: each account can hold a maximum of 10 active orders simultaneously per symbol.
    /// Spot: 500 orders in total, including a maximum of 30 open TP/SL orders, a maximum of 30 open conditional orders for each symbol per account
    /// Option: a maximum of 50 open orders per account
    ///
    /// Rate limit:
    /// Please refer to rate limit table. If you need to raise the rate limit, please contact your client manager or submit an application via here
    ///
    /// Risk control limit notice:
    /// Bybit will monitor on your API requests. When the total number of orders of a single user (aggregated the number of orders across main account and subaccounts) within a day (UTC 0 - UTC 24) exceeds a certain upper limit, the platform will reserve the right to remind, warn, and impose necessary restrictions. Customers who use API default to acceptance of these terms and have the obligation to cooperate with adjustments.
    ///
    /// Reduce only orders:
    /// If reduceOnly=true and order qty > max order qty, the order will automatically be split up into multiple orders.
    ///
    /// Spot Stop Order
    /// Spot supports TP/SL order, Conditional order, however, the system logic is different between classic account and Unified account
    /// classic account: When the stop order is created, you will get an order ID. After it is triggered, you will get a new order ID
    /// Unified account: When the stop order is created, you will get an order ID. After it is triggered, the order ID will not be changed
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn place_order(
        &self,
        request: &PlaceOrderRequest,
    ) -> Result<Response<PlaceOrderResponse>, Error> {
        let category = request.category;
        let url = format!("{}{}", self.base_url, Path::TradeOrderCreate);
        let json = serialize_json(request)?;
        let headers = self.get_signed_headers(&json)?;

        let request = self
            .client
            .request(Method::POST, url)
            .headers(headers)
            .body(json);

        let response = self
            .send_weighted(Path::TradeOrderCreate, Some(category), 1, request)
            .await?;
        Ok(response)
    }

    /// Amend Order
    /// info
    /// You can only modify unfilled or partially filled orders.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn amend_order(
        &self,
        request: &AmendOrderRequest,
    ) -> Result<Response<AmendOrderResponse>, Error> {
        let category = request.category;
        let url = format!("{}{}", self.base_url, Path::TradeOrderAmend);
        let json = serialize_json(request)?;
        let headers = self.get_signed_headers(&json)?;

        let request = self
            .client
            .request(Method::POST, url)
            .headers(headers)
            .body(json);

        let response = self
            .send_weighted(Path::TradeOrderAmend, Some(category), 1, request)
            .await?;
        Ok(response)
    }

    /// Cancel Order
    /// important
    /// You must specify orderId or orderLinkId to cancel the order.
    /// If orderId and orderLinkId do not match, the system will process orderId first.
    /// You can only cancel unfilled or partially filled orders.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn cancel_order(
        &self,
        request: &CancelOrderRequest,
    ) -> Result<Response<CancelOrderResponse>, Error> {
        let category = request.category;
        let url = format!("{}{}", self.base_url, Path::TradeOrderCancel);
        let json = serialize_json(request)?;
        let headers = self.get_signed_headers(&json)?;

        let request = self
            .client
            .request(Method::POST, url)
            .headers(headers)
            .body(json);

        let response = self
            .send_weighted(Path::TradeOrderCancel, Some(category), 1, request)
            .await?;
        Ok(response)
    }

    /// Get Open & Closed Orders.
    /// Primarily query unfilled or partially filled orders in real-time, but also supports querying recent 500 closed status (Cancelled, Filled) orders. Please see the usage of request param openOnly.
    /// And to query older order records, please use the order history interface.
    ///
    /// Tip
    /// UTA2.0 can query filled, canceled, and rejected orders to the most recent 500 orders for spot, linear, inverse and option categories
    /// UTA1.0 can query filled, canceled, and rejected orders to the most recent 500 orders for spot, linear, and option categories. The inverse category is not subject to this limitation.
    /// You can query by symbol, baseCoin, orderId and orderLinkId, and if you pass multiple params, the system will process them according to this priority: orderId > orderLinkId > symbol > baseCoin.
    /// The records are sorted by the createdTime from newest to oldest.
    ///
    /// info
    /// classic account spot can return open orders only
    /// After a server release or restart, filled, canceled, and rejected orders of Unified account should only be queried through order history.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_open_closed_orders(
        &self,
        params: &GetOpenClosedOrdersParams,
    ) -> Result<Response<CursorPagination<Order>>, Error> {
        let query = serialize_query(params)?;
        let url = format!("{}{}?{query}", self.base_url, Path::TradeOrderRealtime);
        let headers = self.get_signed_headers(&query)?;

        let request = self.client.request(Method::GET, url).headers(headers);

        let response = self.send(Path::TradeOrderRealtime, request).await?;
        Ok(response)
    }

    /// Collect all pages of open/closed orders into a single `Vec`.
    /// Repeatedly calls [`get_open_closed_orders`](Client::get_open_closed_orders) following `next_page_cursor`
    /// until the last page is reached (see [`MAX_PAGES`]).
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_open_closed_orders_all(
        &self,
        params: &GetOpenClosedOrdersParams,
    ) -> Result<Vec<Order>, Error> {
        collect_pages(|cursor| {
            let p = match cursor {
                Some(cursor) => params.clone().with_cursor(cursor),
                None => params.clone(),
            };
            async move { self.get_open_closed_orders(&p).await.map(|r| r.result) }
        })
        .await
    }

    /// Cancel All Orders.
    /// Cancel all open orders. Support linear, inverse, spot, and option.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn cancel_all_orders(
        &self,
        request: &CancelAllOrdersRequest,
    ) -> Result<Response<CancelAllOrdersResponse>, Error> {
        let category = request.category;
        let url = format!("{}{}", self.base_url, Path::TradeOrderCancelAll);
        let json = serialize_json(request)?;
        let headers = self.get_signed_headers(&json)?;

        let request = self
            .client
            .request(Method::POST, url)
            .headers(headers)
            .body(json);

        let response = self
            .send_weighted(Path::TradeOrderCancelAll, Some(category), 1, request)
            .await?;
        Ok(response)
    }

    /// Get Order History.
    /// Query order history. As order creation/cancellation is asynchronous, the data returned may be delayed.
    /// Supports up to 2 years of data.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_order_history(
        &self,
        params: &GetOrderHistoryParams,
    ) -> Result<Response<CursorPagination<Order>>, Error> {
        let query = serialize_query(params)?;
        let url = format!("{}{}?{query}", self.base_url, Path::TradeOrderHistory);
        let headers = self.get_signed_headers(&query)?;

        let request = self.client.request(Method::GET, url).headers(headers);

        let response = self.send(Path::TradeOrderHistory, request).await?;
        Ok(response)
    }

    /// Collect all pages of order history into a single `Vec`.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_order_history_all(
        &self,
        params: &GetOrderHistoryParams,
    ) -> Result<Vec<Order>, Error> {
        collect_pages(|cursor| {
            let p = match cursor {
                Some(cursor) => params.clone().with_cursor(cursor),
                None => params.clone(),
            };
            async move { self.get_order_history(&p).await.map(|r| r.result) }
        })
        .await
    }

    /// Place Batch Orders.
    /// Supports up to 20 orders per request.
    /// Per-item results are in `response.ret_ext_info.list` (parallel to `response.result.list`).
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn place_orders_batch(
        &self,
        request: &PlaceOrderBatchRequest,
    ) -> Result<Response<List<PlaceOrderBatchResult>>, Error> {
        let category = request.category;
        let cost = request.request.len() as u32;
        let url = format!("{}{}", self.base_url, Path::TradeOrderCreateBatch);
        let json = serialize_json(request)?;
        let headers = self.get_signed_headers(&json)?;

        let request = self
            .client
            .request(Method::POST, url)
            .headers(headers)
            .body(json);

        let response = self
            .send_weighted(Path::TradeOrderCreateBatch, Some(category), cost, request)
            .await?;
        Ok(response)
    }

    /// Amend Batch Orders.
    /// Supports up to 20 orders per request.
    /// Per-item results are in `response.ret_ext_info.list` (parallel to `response.result.list`).
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn amend_orders_batch(
        &self,
        request: &AmendOrderBatchRequest,
    ) -> Result<Response<List<AmendOrderBatchResult>>, Error> {
        let category = request.category;
        let cost = request.request.len() as u32;
        let url = format!("{}{}", self.base_url, Path::TradeOrderAmendBatch);
        let json = serialize_json(request)?;
        let headers = self.get_signed_headers(&json)?;

        let request = self
            .client
            .request(Method::POST, url)
            .headers(headers)
            .body(json);

        let response = self
            .send_weighted(Path::TradeOrderAmendBatch, Some(category), cost, request)
            .await?;
        Ok(response)
    }

    /// Cancel Batch Orders.
    /// Supports up to 20 orders per request.
    /// Per-item results are in `response.ret_ext_info.list` (parallel to `response.result.list`).
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn cancel_orders_batch(
        &self,
        request: &CancelOrderBatchRequest,
    ) -> Result<Response<List<CancelOrderBatchResult>>, Error> {
        let category = request.category;
        let cost = request.request.len() as u32;
        let url = format!("{}{}", self.base_url, Path::TradeOrderCancelBatch);
        let json = serialize_json(request)?;
        let headers = self.get_signed_headers(&json)?;

        let request = self
            .client
            .request(Method::POST, url)
            .headers(headers)
            .body(json);

        let response = self
            .send_weighted(Path::TradeOrderCancelBatch, Some(category), cost, request)
            .await?;
        Ok(response)
    }

    /// Get Spot Borrow Check.
    /// Query the maximum quantity for purchase or sale, and check the borrowable quantity based on the fee types.
    ///
    /// Covers: spot only. Requires authentication.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_spot_borrow_check(
        &self,
        params: &GetSpotBorrowCheckParams,
    ) -> Result<Response<SpotBorrowCheck>, Error> {
        let url = format!("{}{}", self.base_url, Path::TradeOrderSpotBorrowCheck);
        let query = serialize_query(params)?;
        let headers = self.get_signed_headers(&query)?;

        let request = self
            .client
            .request(Method::GET, url)
            .headers(headers)
            .query(params);

        let response = self.send(Path::TradeOrderSpotBorrowCheck, request).await?;
        Ok(response)
    }
}

// Position.
impl Client {
    /// Query real-time position data, such as position size, cumulative realized PNL, etc.
    /// Query real-time position data, such as position size, cumulative realized PNL, etc.
    ///
    /// INFO
    // UTA2.0(inverse)
    // You can query all open positions with /v5/position/list?category=inverse;
    // Cannot query multiple symbols in one request
    // UTA1.0(inverse) & Classic (inverse)
    // You can query all open positions with /v5/position/list?category=inverse;
    // symbol parameter can pass up to 10 symbols, e.g., symbol=BTCUSD,ETHUSD
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_position_info(
        &self,
        params: &GetPositionInfoParams,
    ) -> Result<Response<CursorPagination<Position>>, Error> {
        let query = serialize_query(params)?;
        let url = format!("{}{}?{query}", self.base_url, Path::PositionList);
        let headers = self.get_signed_headers(&query)?;

        let request = self.client.request(Method::GET, url).headers(headers);

        let response = self.send(Path::PositionList, request).await?;
        Ok(response)
    }

    /// Collect all pages of position info into a single `Vec`.
    /// Repeatedly calls [`get_position_info`](Client::get_position_info) following `next_page_cursor`
    /// until the last page is reached (see [`MAX_PAGES`]).
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_position_info_all(
        &self,
        params: &GetPositionInfoParams,
    ) -> Result<Vec<Position>, Error> {
        collect_pages(|cursor| {
            let p = match cursor {
                Some(cursor) => params.clone().with_cursor(cursor),
                None => params.clone(),
            };
            async move { self.get_position_info(&p).await.map(|r| r.result) }
        })
        .await
    }

    /// Set Leverage.
    /// Set the leverage for a position. Only for isolated margin mode.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn set_leverage(
        &self,
        request: &SetLeverageRequest,
    ) -> Result<Response<EmptyResult>, Error> {
        let url = format!("{}{}", self.base_url, Path::PositionSetLeverage);
        let json = serialize_json(request)?;
        let headers = self.get_signed_headers(&json)?;

        let request = self
            .client
            .request(Method::POST, url)
            .headers(headers)
            .body(json);

        let response = self.send(Path::PositionSetLeverage, request).await?;
        Ok(response)
    }

    /// Set Trading Stop.
    /// Set take profit, stop loss, or trailing stop for a position.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn set_trading_stop(
        &self,
        request: &SetTradingStopRequest,
    ) -> Result<Response<EmptyResult>, Error> {
        let url = format!("{}{}", self.base_url, Path::PositionTradingStop);
        let json = serialize_json(request)?;
        let headers = self.get_signed_headers(&json)?;

        let request = self
            .client
            .request(Method::POST, url)
            .headers(headers)
            .body(json);

        let response = self.send(Path::PositionTradingStop, request).await?;
        Ok(response)
    }

    /// Switch Cross/Isolated Margin.
    /// Switch the margin mode for a symbol between cross and isolated.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn switch_cross_isolated_margin(
        &self,
        request: &SwitchCrossIsolatedMarginRequest,
    ) -> Result<Response<EmptyResult>, Error> {
        let url = format!("{}{}", self.base_url, Path::PositionSwitchIsolated);
        let json = serialize_json(request)?;
        let headers = self.get_signed_headers(&json)?;

        let request = self
            .client
            .request(Method::POST, url)
            .headers(headers)
            .body(json);

        let response = self.send(Path::PositionSwitchIsolated, request).await?;
        Ok(response)
    }

    /// Switch Position Mode.
    /// Switch between one-way (merged single) and hedge (both sides) position mode.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn switch_position_mode(
        &self,
        request: &SwitchPositionModeRequest,
    ) -> Result<Response<EmptyResult>, Error> {
        let url = format!("{}{}", self.base_url, Path::PositionSwitchMode);
        let json = serialize_json(request)?;
        let headers = self.get_signed_headers(&json)?;

        let request = self
            .client
            .request(Method::POST, url)
            .headers(headers)
            .body(json);

        let response = self.send(Path::PositionSwitchMode, request).await?;
        Ok(response)
    }

    /// Set Auto Add Margin.
    /// Turn on/off auto-add-margin for an isolated margin position.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn set_auto_add_margin(
        &self,
        request: &SetAutoAddMarginRequest,
    ) -> Result<Response<EmptyResult>, Error> {
        let url = format!("{}{}", self.base_url, Path::PositionSetAutoAddMargin);
        let json = serialize_json(request)?;
        let headers = self.get_signed_headers(&json)?;

        let request = self
            .client
            .request(Method::POST, url)
            .headers(headers)
            .body(json);

        let response = self.send(Path::PositionSetAutoAddMargin, request).await?;
        Ok(response)
    }

    /// Set Risk Limit.
    /// Set the risk limit for a position. The response includes the new risk limit and its value.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn set_risk_limit(
        &self,
        request: &SetRiskLimitRequest,
    ) -> Result<Response<SetRiskLimitResponse>, Error> {
        let url = format!("{}{}", self.base_url, Path::PositionSetRiskLimit);
        let json = serialize_json(request)?;
        let headers = self.get_signed_headers(&json)?;

        let request = self
            .client
            .request(Method::POST, url)
            .headers(headers)
            .body(json);

        let response = self.send(Path::PositionSetRiskLimit, request).await?;
        Ok(response)
    }

    /// Get Closed P&L.
    /// Query the closed profit and loss records of positions.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_closed_pnl(
        &self,
        params: &GetClosedPnlParams,
    ) -> Result<Response<CursorPagination<ClosedPnl>>, Error> {
        let query = serialize_query(params)?;
        let url = format!("{}{}?{query}", self.base_url, Path::PositionClosedPnl);
        let headers = self.get_signed_headers(&query)?;

        let request = self.client.request(Method::GET, url).headers(headers);

        let response = self.send(Path::PositionClosedPnl, request).await?;
        Ok(response)
    }

    /// Collect all pages of closed P&L into a single `Vec`.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_closed_pnl_all(
        &self,
        params: &GetClosedPnlParams,
    ) -> Result<Vec<ClosedPnl>, Error> {
        collect_pages(|cursor| {
            let p = match cursor {
                Some(cursor) => params.clone().with_cursor(cursor),
                None => params.clone(),
            };
            async move { self.get_closed_pnl(&p).await.map(|r| r.result) }
        })
        .await
    }

    /// Get Execution List.
    /// Query users' execution (trading) records, sorted by execTime descending.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_execution_list(
        &self,
        params: &GetExecutionListParams,
    ) -> Result<Response<CursorPagination<ExecutionEntry>>, Error> {
        let query = serialize_query(params)?;
        let url = format!("{}{}?{query}", self.base_url, Path::ExecutionList);
        let headers = self.get_signed_headers(&query)?;

        let request = self.client.request(Method::GET, url).headers(headers);

        let response = self.send(Path::ExecutionList, request).await?;
        Ok(response)
    }

    /// Collect all pages of execution list entries into a single `Vec`.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_execution_list_all(
        &self,
        params: &GetExecutionListParams,
    ) -> Result<Vec<ExecutionEntry>, Error> {
        collect_pages(|cursor| {
            let p = match cursor {
                Some(cursor) => params.clone().with_cursor(cursor),
                None => params.clone(),
            };
            async move { self.get_execution_list(&p).await.map(|r| r.result) }
        })
        .await
    }
}

// Account.
impl Client {
    /// Obtain wallet balance, query asset information of each currency. By default, currency information with assets or liabilities of 0 is not returned.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_wallet_balance(
        &self,
        params: &GetWalletBalanceParams,
    ) -> Result<Response<List<WalletBalance>>, Error> {
        let query = serialize_query(params)?;
        let url = format!("{}{}?{query}", self.base_url, Path::AccountWalletBalance);
        let headers = self.get_signed_headers(&query)?;

        let request = self.client.request(Method::GET, url).headers(headers);

        let response = self.send(Path::AccountWalletBalance, request).await?;
        Ok(response)
    }

    /// Get Transaction Log
    /// Query for transaction logs in your Unified account. It supports up to 2 years worth of data.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_transaction_log(
        &self,
        params: &GetTransactionLogParams,
    ) -> Result<Response<CursorPagination<TransactionLog>>, Error> {
        let query = serialize_query(params)?;
        let url = format!("{}{}?{query}", self.base_url, Path::AccountTransactionLog);
        let headers = self.get_signed_headers(&query)?;

        let request = self.client.request(Method::GET, url).headers(headers);

        let response = self.send(Path::AccountTransactionLog, request).await?;
        Ok(response)
    }

    /// Collect all pages of transaction log entries into a single `Vec`.
    /// Repeatedly calls [`get_transaction_log`](Client::get_transaction_log) following `next_page_cursor`
    /// until the last page is reached (see [`MAX_PAGES`]).
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_transaction_log_all(
        &self,
        params: &GetTransactionLogParams,
    ) -> Result<Vec<TransactionLog>, Error> {
        collect_pages(|cursor| {
            let p = match cursor {
                Some(cursor) => params.clone().with_cursor(cursor),
                None => params.clone(),
            };
            async move { self.get_transaction_log(&p).await.map(|r| r.result) }
        })
        .await
    }

    /// Query the account information, like margin mode, account mode, etc.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_account_info(&self) -> Result<Response<AccountInfo>, Error> {
        let url = format!("{}{}", self.base_url, Path::AccountInfo);
        let query = "";
        let headers = self.get_signed_headers(query)?;

        let request = self.client.request(Method::GET, url).headers(headers);

        let response = self.send(Path::AccountInfo, request).await?;
        Ok(response)
    }

    /// Get Fee Rate.
    /// Get the trading fee rate.
    ///
    /// Requires authentication.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_fee_rate(
        &self,
        params: &GetFeeRateParams,
    ) -> Result<Response<List<FeeRateEntry>>, Error> {
        let url = format!("{}{}", self.base_url, Path::AccountFeeRate);
        let query = serialize_query(params)?;
        let headers = self.get_signed_headers(&query)?;

        let request = self
            .client
            .request(Method::GET, url)
            .headers(headers)
            .query(params);

        let response = self.send(Path::AccountFeeRate, request).await?;
        Ok(response)
    }

    /// Set Margin Mode.
    /// Switch between regular margin, isolated margin, and portfolio margin.
    ///
    /// Requires authentication.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn set_margin_mode(
        &self,
        request_body: &SetMarginModeRequest,
    ) -> Result<Response<SetMarginModeResponse>, Error> {
        let url = format!("{}{}", self.base_url, Path::AccountSetMarginMode);
        let body = serialize_json(request_body)?;
        let headers = self.get_signed_headers(&body)?;

        let request = self
            .client
            .request(Method::POST, url)
            .headers(headers)
            .body(body);

        let response = self.send(Path::AccountSetMarginMode, request).await?;
        Ok(response)
    }

    /// Upgrade to Unified Account (Pro).
    /// Upgrades an account to Unified Account Pro (UTA2.0 Pro). No open orders may exist
    /// during the upgrade.
    ///
    /// Requires authentication.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn upgrade_to_uta(&self) -> Result<Response<UpgradeToUtaResult>, Error> {
        let url = format!("{}{}", self.base_url, Path::AccountUpgradeToUta);
        let body = "{}".to_owned();
        let headers = self.get_signed_headers(&body)?;

        let request = self
            .client
            .request(Method::POST, url)
            .headers(headers)
            .body(body);

        let response = self.send(Path::AccountUpgradeToUta, request).await?;
        Ok(response)
    }

    /// Get Borrow History.
    /// Get interest records, sorted in reverse order of creation time.
    ///
    /// Requires authentication. Unified account only.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_borrow_history(
        &self,
        params: &GetBorrowHistoryParams,
    ) -> Result<Response<CursorPagination<BorrowHistoryEntry>>, Error> {
        let query = serialize_query(params)?;
        let url = format!("{}{}?{query}", self.base_url, Path::AccountBorrowHistory);
        let headers = self.get_signed_headers(&query)?;

        let request = self.client.request(Method::GET, url).headers(headers);

        let response = self.send(Path::AccountBorrowHistory, request).await?;
        Ok(response)
    }

    /// Collect all pages of borrow history entries into a single `Vec`.
    /// Repeatedly calls [`get_borrow_history`](Client::get_borrow_history) following
    /// `next_page_cursor` until the last page is reached.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_borrow_history_all(
        &self,
        params: &GetBorrowHistoryParams,
    ) -> Result<Vec<BorrowHistoryEntry>, Error> {
        collect_pages(|cursor| {
            let p = match cursor {
                Some(cursor) => params.clone().with_cursor(cursor),
                None => params.clone(),
            };
            async move { self.get_borrow_history(&p).await.map(|r| r.result) }
        })
        .await
    }

    /// Get Collateral Info.
    /// Query the collateral ratio, and information of the collateral ratio tier, for each
    /// currency.
    ///
    /// Requires authentication. Unified account only.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_collateral_info(
        &self,
        params: &GetCollateralInfoParams,
    ) -> Result<Response<List<CollateralInfoEntry>>, Error> {
        let url = format!("{}{}", self.base_url, Path::AccountCollateralInfo);
        let query = serialize_query(params)?;
        let headers = self.get_signed_headers(&query)?;

        let request = self
            .client
            .request(Method::GET, url)
            .headers(headers)
            .query(params);

        let response = self.send(Path::AccountCollateralInfo, request).await?;
        Ok(response)
    }
}

// User.
impl Client {
    /// Get API Key Information.
    /// Get the information of the api key. Use the api key pending to be checked to call the endpoint. Both master and sub user's api key are applicable.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_api_key_information(&self) -> Result<Response<APIKeyInformation>, Error> {
        let url = format!("{}{}", self.base_url, Path::UserQueryApi);
        let query = "";
        let headers = self.get_signed_headers(query)?;

        let request = self.client.request(Method::GET, url).headers(headers);

        let response = self.send(Path::UserQueryApi, request).await?;
        Ok(response)
    }
}

// Asset.
impl Client {
    /// Create Internal Transfer.
    /// Transfer between different account types under the same UID.
    ///
    /// Requires authentication.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn internal_transfer(
        &self,
        request: &InternalTransferRequest,
    ) -> Result<Response<TransferResult>, Error> {
        let url = format!("{}{}", self.base_url, Path::AssetTransferInterTransfer);
        let body = serialize_json(request)?;
        let headers = self.get_signed_headers(&body)?;

        let request = self
            .client
            .request(Method::POST, url)
            .headers(headers)
            .body(body);

        let response = self.send(Path::AssetTransferInterTransfer, request).await?;
        Ok(response)
    }

    /// Get Internal Transfer Records.
    /// Query the internal transfer records between different account types under the same UID.
    ///
    /// Requires authentication.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_internal_transfer_records(
        &self,
        params: &GetInternalTransferRecordsParams,
    ) -> Result<Response<CursorPagination<InternalTransferEntry>>, Error> {
        let query = serialize_query(params)?;
        let url = format!(
            "{}{}?{query}",
            self.base_url,
            Path::AssetTransferQueryInterTransferList
        );
        let headers = self.get_signed_headers(&query)?;

        let request = self.client.request(Method::GET, url).headers(headers);

        let response = self
            .send(Path::AssetTransferQueryInterTransferList, request)
            .await?;
        Ok(response)
    }

    /// Create Universal Transfer.
    /// Transfer between a master account and its sub-accounts, or between two sub-accounts.
    ///
    /// Requires authentication. Requires the master UID's API key, or a sub-account key with
    /// the `SubMemberTransferList` permission (may only transfer to the master account).
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn universal_transfer(
        &self,
        request: &UniversalTransferRequest,
    ) -> Result<Response<TransferResult>, Error> {
        let url = format!("{}{}", self.base_url, Path::AssetTransferUniversalTransfer);
        let body = serialize_json(request)?;
        let headers = self.get_signed_headers(&body)?;

        let request = self
            .client
            .request(Method::POST, url)
            .headers(headers)
            .body(body);

        let response = self
            .send(Path::AssetTransferUniversalTransfer, request)
            .await?;
        Ok(response)
    }

    /// Get Universal Transfer Records.
    ///
    /// Requires authentication. Requires the master UID's API key, or a sub-account key with
    /// the `SubMemberTransferList` permission.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_universal_transfer_records(
        &self,
        params: &GetUniversalTransferRecordsParams,
    ) -> Result<Response<CursorPagination<UniversalTransferEntry>>, Error> {
        let query = serialize_query(params)?;
        let url = format!(
            "{}{}?{query}",
            self.base_url,
            Path::AssetTransferQueryUniversalTransferList
        );
        let headers = self.get_signed_headers(&query)?;

        let request = self.client.request(Method::GET, url).headers(headers);

        let response = self
            .send(Path::AssetTransferQueryUniversalTransferList, request)
            .await?;
        Ok(response)
    }

    /// Get Transferable Coin.
    /// Query the transferable coins between two account types.
    ///
    /// Requires authentication.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_transferable_coins(
        &self,
        params: &GetTransferableCoinsParams,
    ) -> Result<Response<List<String>>, Error> {
        let query = serialize_query(params)?;
        let url = format!(
            "{}{}?{query}",
            self.base_url,
            Path::AssetTransferQueryTransferCoinList
        );
        let headers = self.get_signed_headers(&query)?;

        let request = self.client.request(Method::GET, url).headers(headers);

        let response = self
            .send(Path::AssetTransferQueryTransferCoinList, request)
            .await?;
        Ok(response)
    }

    /// Enable Universal Transfer For Sub UID.
    ///
    /// Deprecated by Bybit: all sub UIDs are now automatically enabled for universal transfer,
    /// so this call is no longer necessary. Kept for completeness. Requires the master UID's
    /// API key.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn save_transfer_sub_member(
        &self,
        request: &SaveTransferSubMemberRequest,
    ) -> Result<Response<EmptyResult>, Error> {
        let url = format!(
            "{}{}",
            self.base_url,
            Path::AssetTransferSaveTransferSubMember
        );
        let body = serialize_json(request)?;
        let headers = self.get_signed_headers(&body)?;

        let request = self
            .client
            .request(Method::POST, url)
            .headers(headers)
            .body(body);

        let response = self
            .send(Path::AssetTransferSaveTransferSubMember, request)
            .await?;
        Ok(response)
    }

    /// Get Sub UID (for universal transfer).
    /// Query the sub UIDs under a master account, and which of them are enabled for universal
    /// transfer.
    ///
    /// Requires authentication. Requires the master UID's API key.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_transferable_sub_members(
        &self,
    ) -> Result<Response<TransferableSubMembers>, Error> {
        let url = format!("{}{}", self.base_url, Path::AssetTransferQuerySubMemberList);
        let query = "";
        let headers = self.get_signed_headers(query)?;

        let request = self.client.request(Method::GET, url).headers(headers);

        let response = self
            .send(Path::AssetTransferQuerySubMemberList, request)
            .await?;
        Ok(response)
    }

    /// Get Single Coin Balance.
    /// Query the balance of a specific coin in a specific account type, or the balance that is
    /// safe to transfer between two accounts.
    ///
    /// Requires authentication.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_account_coin_balance(
        &self,
        params: &GetAccountCoinBalanceParams,
    ) -> Result<Response<AccountCoinBalance>, Error> {
        let query = serialize_query(params)?;
        let url = format!(
            "{}{}?{query}",
            self.base_url,
            Path::AssetTransferQueryAccountCoinBalance
        );
        let headers = self.get_signed_headers(&query)?;

        let request = self.client.request(Method::GET, url).headers(headers);

        let response = self
            .send(Path::AssetTransferQueryAccountCoinBalance, request)
            .await?;
        Ok(response)
    }

    /// Get Asset Info (Spot).
    /// Query for the balance of the Spot account.
    ///
    /// Requires authentication.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_asset_info(
        &self,
        params: &GetAssetInfoParams,
    ) -> Result<Response<AssetInfoResult>, Error> {
        let query = serialize_query(params)?;
        let url = format!(
            "{}{}?{query}",
            self.base_url,
            Path::AssetTransferQueryAssetInfo
        );
        let headers = self.get_signed_headers(&query)?;

        let request = self.client.request(Method::GET, url).headers(headers);

        let response = self
            .send(Path::AssetTransferQueryAssetInfo, request)
            .await?;
        Ok(response)
    }

    /// Get Allowed Deposit Coin Info.
    ///
    /// Requires authentication.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_deposit_allowed_coin_info(
        &self,
        params: &GetDepositAllowedCoinInfoParams,
    ) -> Result<Response<DepositAllowedCoinInfoResult>, Error> {
        let query = serialize_query(params)?;
        let url = format!(
            "{}{}?{query}",
            self.base_url,
            Path::AssetDepositQueryAllowedList
        );
        let headers = self.get_signed_headers(&query)?;

        let request = self.client.request(Method::GET, url).headers(headers);

        let response = self
            .send(Path::AssetDepositQueryAllowedList, request)
            .await?;
        Ok(response)
    }

    /// Get Deposit Records (on-chain).
    ///
    /// Requires authentication.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_deposit_records(
        &self,
        params: &GetDepositRecordsParams,
    ) -> Result<Response<DepositRecords>, Error> {
        let query = serialize_query(params)?;
        let url = format!("{}{}?{query}", self.base_url, Path::AssetDepositQueryRecord);
        let headers = self.get_signed_headers(&query)?;

        let request = self.client.request(Method::GET, url).headers(headers);

        let response = self.send(Path::AssetDepositQueryRecord, request).await?;
        Ok(response)
    }

    /// Get Sub Account Deposit Records (on-chain).
    ///
    /// Requires authentication. Requires the master UID's API key.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_sub_deposit_records(
        &self,
        params: &GetSubDepositRecordsParams,
    ) -> Result<Response<DepositRecords>, Error> {
        let query = serialize_query(params)?;
        let url = format!(
            "{}{}?{query}",
            self.base_url,
            Path::AssetDepositQuerySubMemberRecord
        );
        let headers = self.get_signed_headers(&query)?;

        let request = self.client.request(Method::GET, url).headers(headers);

        let response = self
            .send(Path::AssetDepositQuerySubMemberRecord, request)
            .await?;
        Ok(response)
    }

    /// Get Master Deposit Address.
    ///
    /// Requires authentication.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_deposit_address(
        &self,
        params: &GetDepositAddressParams,
    ) -> Result<Response<DepositAddress>, Error> {
        let query = serialize_query(params)?;
        let url = format!(
            "{}{}?{query}",
            self.base_url,
            Path::AssetDepositQueryAddress
        );
        let headers = self.get_signed_headers(&query)?;

        let request = self.client.request(Method::GET, url).headers(headers);

        let response = self.send(Path::AssetDepositQueryAddress, request).await?;
        Ok(response)
    }

    /// Get Sub Account Deposit Address.
    ///
    /// Requires authentication. Requires the master account's API key.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_sub_deposit_address(
        &self,
        params: &GetSubDepositAddressParams,
    ) -> Result<Response<DepositAddress>, Error> {
        let query = serialize_query(params)?;
        let url = format!(
            "{}{}?{query}",
            self.base_url,
            Path::AssetDepositQuerySubMemberAddress
        );
        let headers = self.get_signed_headers(&query)?;

        let request = self.client.request(Method::GET, url).headers(headers);

        let response = self
            .send(Path::AssetDepositQuerySubMemberAddress, request)
            .await?;
        Ok(response)
    }

    /// Get Withdrawal Records.
    ///
    /// Requires authentication.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_withdrawal_records(
        &self,
        params: &GetWithdrawalRecordsParams,
    ) -> Result<Response<WithdrawRecords>, Error> {
        let query = serialize_query(params)?;
        let url = format!(
            "{}{}?{query}",
            self.base_url,
            Path::AssetWithdrawQueryRecord
        );
        let headers = self.get_signed_headers(&query)?;

        let request = self.client.request(Method::GET, url).headers(headers);

        let response = self.send(Path::AssetWithdrawQueryRecord, request).await?;
        Ok(response)
    }

    /// Get Withdrawable Amount.
    /// Query the amount available to withdraw for a coin, combined across the FUND, UTA, SPOT
    /// and EARN wallets, along with any amount still frozen pending deposit risk review.
    ///
    /// Requires authentication.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_withdrawable_amount(
        &self,
        params: &GetWithdrawableAmountParams,
    ) -> Result<Response<WithdrawableAmountResult>, Error> {
        let query = serialize_query(params)?;
        let url = format!(
            "{}{}?{query}",
            self.base_url,
            Path::AssetWithdrawWithdrawableAmount
        );
        let headers = self.get_signed_headers(&query)?;

        let request = self.client.request(Method::GET, url).headers(headers);

        let response = self
            .send(Path::AssetWithdrawWithdrawableAmount, request)
            .await?;
        Ok(response)
    }

    /// Withdraw.
    /// Create a withdrawal request.
    ///
    /// Requires authentication.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn withdraw(
        &self,
        request: &WithdrawRequest,
    ) -> Result<Response<WithdrawResult>, Error> {
        let url = format!("{}{}", self.base_url, Path::AssetWithdrawCreate);
        let body = serialize_json(request)?;
        let headers = self.get_signed_headers(&body)?;

        let request = self
            .client
            .request(Method::POST, url)
            .headers(headers)
            .body(body);

        let response = self.send(Path::AssetWithdrawCreate, request).await?;
        Ok(response)
    }

    /// Cancel Withdrawal.
    /// Cancel a pending withdrawal request.
    ///
    /// Requires authentication.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn cancel_withdrawal(
        &self,
        request: &CancelWithdrawalRequest,
    ) -> Result<Response<CancelWithdrawalResult>, Error> {
        let url = format!("{}{}", self.base_url, Path::AssetWithdrawCancel);
        let body = serialize_json(request)?;
        let headers = self.get_signed_headers(&body)?;

        let request = self
            .client
            .request(Method::POST, url)
            .headers(headers)
            .body(body);

        let response = self.send(Path::AssetWithdrawCancel, request).await?;
        Ok(response)
    }

    /// Get Coin Info.
    /// Query coin information, including chains, deposit/withdrawal limits, and fees.
    ///
    /// Requires authentication.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_coin_info(
        &self,
        params: &GetCoinInfoParams,
    ) -> Result<Response<CoinInfoResult>, Error> {
        let url = format!("{}{}", self.base_url, Path::AssetCoinQueryInfo);
        let query = serialize_query(params)?;
        let headers = self.get_signed_headers(&query)?;

        let request = self
            .client
            .request(Method::GET, url)
            .headers(headers)
            .query(params);

        let response = self.send(Path::AssetCoinQueryInfo, request).await?;
        Ok(response)
    }

    /// Get Coin Exchange Records.
    ///
    /// Requires authentication.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_exchange_order_record(
        &self,
        params: &GetExchangeOrderRecordParams,
    ) -> Result<Response<ExchangeOrderRecords>, Error> {
        let query = serialize_query(params)?;
        let url = format!(
            "{}{}?{query}",
            self.base_url,
            Path::AssetExchangeOrderRecord
        );
        let headers = self.get_signed_headers(&query)?;

        let request = self.client.request(Method::GET, url).headers(headers);

        let response = self.send(Path::AssetExchangeOrderRecord, request).await?;
        Ok(response)
    }

    /// Get Delivery Record.
    /// Query delivery records of USDC futures and Options, sorted by descending delivery time.
    ///
    /// Requires authentication.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_delivery_record(
        &self,
        params: &GetDeliveryRecordParams,
    ) -> Result<Response<CursorPagination<DeliveryRecord>>, Error> {
        let query = serialize_query(params)?;
        let url = format!("{}{}?{query}", self.base_url, Path::AssetDeliveryRecord);
        let headers = self.get_signed_headers(&query)?;

        let request = self.client.request(Method::GET, url).headers(headers);

        let response = self.send(Path::AssetDeliveryRecord, request).await?;
        Ok(response)
    }

    /// Get USDC Session Settlement.
    /// Query session settlement records of USDC perpetual contracts.
    ///
    /// Requires authentication.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_settlement_record(
        &self,
        params: &GetSettlementRecordParams,
    ) -> Result<Response<CursorPagination<SettlementRecord>>, Error> {
        let query = serialize_query(params)?;
        let url = format!("{}{}?{query}", self.base_url, Path::AssetSettlementRecord);
        let headers = self.get_signed_headers(&query)?;

        let request = self.client.request(Method::GET, url).headers(headers);

        let response = self.send(Path::AssetSettlementRecord, request).await?;
        Ok(response)
    }

    /// Get Coin Greeks.
    /// Query the current account's Greeks information. Option only.
    ///
    /// Requires authentication.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_coin_greeks(
        &self,
        params: &GetCoinGreeksParams,
    ) -> Result<Response<List<CoinGreeks>>, Error> {
        let query = serialize_query(params)?;
        let url = format!("{}{}?{query}", self.base_url, Path::AssetCoinGreeks);
        let headers = self.get_signed_headers(&query)?;

        let request = self.client.request(Method::GET, url).headers(headers);

        let response = self.send(Path::AssetCoinGreeks, request).await?;
        Ok(response)
    }
}

// Spot Leveraged Token.
impl Client {
    /// Get Leverage Token Info.
    /// Query leveraged token information, such as purchase/redeem limits and fees.
    ///
    /// No authentication required.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_leverage_token_info(
        &self,
        params: &GetLeverageTokenInfoParams,
    ) -> Result<Response<List<LeverageTokenInfo>>, Error> {
        let url = format!("{}{}", self.base_url, Path::SpotLeverTokenInfo);

        let request = self.client.request(Method::GET, url).query(params);

        let response = self.send(Path::SpotLeverTokenInfo, request).await?;
        Ok(response)
    }

    /// Get Leverage Token Market.
    /// Query the leveraged token market data, such as net asset value and real leverage.
    ///
    /// No authentication required.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_leverage_token_market(
        &self,
        params: &GetLeverageTokenMarketParams,
    ) -> Result<Response<LeverageTokenMarket>, Error> {
        let url = format!("{}{}", self.base_url, Path::SpotLeverTokenReference);

        let request = self.client.request(Method::GET, url).query(params);

        let response = self.send(Path::SpotLeverTokenReference, request).await?;
        Ok(response)
    }

    /// Purchase.
    /// Purchase a leveraged token.
    ///
    /// Requires authentication.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn purchase_leverage_token(
        &self,
        request: &PurchaseLeverageTokenRequest,
    ) -> Result<Response<PurchaseLeverageTokenResult>, Error> {
        let url = format!("{}{}", self.base_url, Path::SpotLeverTokenPurchase);
        let body = serialize_json(request)?;
        let headers = self.get_signed_headers(&body)?;

        let request = self
            .client
            .request(Method::POST, url)
            .headers(headers)
            .body(body);

        let response = self.send(Path::SpotLeverTokenPurchase, request).await?;
        Ok(response)
    }

    /// Redeem.
    /// Redeem a leveraged token.
    ///
    /// Requires authentication.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn redeem_leverage_token(
        &self,
        request: &RedeemLeverageTokenRequest,
    ) -> Result<Response<RedeemLeverageTokenResult>, Error> {
        let url = format!("{}{}", self.base_url, Path::SpotLeverTokenRedeem);
        let body = serialize_json(request)?;
        let headers = self.get_signed_headers(&body)?;

        let request = self
            .client
            .request(Method::POST, url)
            .headers(headers)
            .body(body);

        let response = self.send(Path::SpotLeverTokenRedeem, request).await?;
        Ok(response)
    }

    /// Get Purchase/Redemption Records.
    ///
    /// Requires authentication.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn get_leverage_token_order_records(
        &self,
        params: &GetLeverageTokenOrderRecordsParams,
    ) -> Result<Response<List<LeverageTokenOrderRecord>>, Error> {
        let query = serialize_query(params)?;
        let url = format!(
            "{}{}?{query}",
            self.base_url,
            Path::SpotLeverTokenOrderRecord
        );
        let headers = self.get_signed_headers(&query)?;

        let request = self.client.request(Method::GET, url).headers(headers);

        let response = self.send(Path::SpotLeverTokenOrderRecord, request).await?;
        Ok(response)
    }
}

// Spot Margin Trade (UTA).
impl Client {
    /// Toggle Margin Trade.
    /// Turn spot margin trade on or off for the Unified account.
    ///
    /// Requires authentication. The account must have completed spot margin activation
    /// (including the required quiz) beforehand.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn switch_spot_margin_mode(
        &self,
        request: &SwitchSpotMarginModeRequest,
    ) -> Result<Response<SwitchSpotMarginModeResult>, Error> {
        let url = format!("{}{}", self.base_url, Path::SpotMarginTradeSwitchMode);
        let body = serialize_json(request)?;
        let headers = self.get_signed_headers(&body)?;

        let request = self
            .client
            .request(Method::POST, url)
            .headers(headers)
            .body(body);

        let response = self.send(Path::SpotMarginTradeSwitchMode, request).await?;
        Ok(response)
    }

    /// Set Leverage.
    /// Set the maximum leverage for spot margin trade.
    ///
    /// Requires authentication. The account must have completed spot margin activation
    /// (including the required quiz) beforehand.
    #[tracing::instrument(level = "debug", skip_all, err(level = "debug"))]
    pub async fn set_spot_margin_leverage(
        &self,
        request: &SetSpotMarginLeverageRequest,
    ) -> Result<Response<EmptyResult>, Error> {
        let url = format!("{}{}", self.base_url, Path::SpotMarginTradeSetLeverage);
        let body = serialize_json(request)?;
        let headers = self.get_signed_headers(&body)?;

        let request = self
            .client
            .request(Method::POST, url)
            .headers(headers)
            .body(body);

        let response = self.send(Path::SpotMarginTradeSetLeverage, request).await?;
        Ok(response)
    }
}

impl Client {
    /// Send a request against a category-independent rate-limit bucket, with a cost of 1.
    async fn send<T>(&self, path: Path, request: RequestBuilder) -> Result<Response<T>, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        self.send_weighted(path, None, 1, request).await
    }

    /// Send a request against the bucket identified by `path` and `category`, consuming
    /// `cost` units of that bucket's quota. Use `cost > 1` for batch endpoints, where Bybit
    /// consumes one unit per order in the batch rather than one per request.
    async fn send_weighted<T>(
        &self,
        path: Path,
        category: Option<Category>,
        cost: u32,
        request: RequestBuilder,
    ) -> Result<Response<T>, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        let key = RateLimitKey { path, category };

        if let Some(rl) = &self.rate_limiter {
            rl.check(key, cost).map_err(|rejection| match rejection {
                Rejection::RetryAfter(retry_after_ms) => Error::RateLimited { retry_after_ms },
                Rejection::Unsatisfiable { cost, burst } => {
                    Error::RateLimitUnsatisfiable { cost, burst }
                }
            })?;
        }

        let start = std::time::Instant::now();
        let response = request.send().await?;
        let elapsed_ms = start.elapsed().as_millis();
        let status = response.status();
        let headers = parse_headers(response.headers());

        if let Some(rl) = &self.rate_limiter {
            rl.update(
                key,
                headers.api_limit_status,
                headers.api_limit_reset_timestamp,
            );
        }

        let body = response.text().await?;
        let result = parse_response(status, headers, &body);
        match &result {
            Ok(response) => tracing::debug!(
                %path,
                elapsed_ms,
                api_limit = response.headers.api_limit,
                api_limit_status = response.headers.api_limit_status,
                "api call completed"
            ),
            Err(e) => tracing::debug!(%path, elapsed_ms, error = %e, "api call failed"),
        }
        result
    }
}

/// Upper bound on the number of pages fetched by the `*_all` methods.
pub const MAX_PAGES: usize = 1000;

/// Follow `next_page_cursor` until the last page and concatenate the lists.
///
/// `fetch` receives `None` for the first page and the cursor of the next page
/// afterwards. Stops when there is no cursor, when a page is empty or when the
/// server repeats the previous cursor (guards against endless loops), and
/// fails with [`Error::TooManyPages`] after [`MAX_PAGES`] pages.
async fn collect_pages<T, F, Fut>(mut fetch: F) -> Result<Vec<T>, Error>
where
    F: FnMut(Option<String>) -> Fut,
    Fut: std::future::Future<Output = Result<CursorPagination<T>, Error>>,
{
    let mut all = Vec::new();
    let mut cursor: Option<String> = None;
    for _ in 0..MAX_PAGES {
        let page = fetch(cursor.clone()).await?;
        let empty = page.list.is_empty();
        all.extend(page.list);
        match page.next_page_cursor {
            Some(next) if !empty && cursor.as_ref() != Some(&next) => cursor = Some(next),
            _ => return Ok(all),
        }
    }
    Err(Error::TooManyPages { max: MAX_PAGES })
}

/// Turn a raw HTTP response into a [`Response`] or an [`Error`].
///
/// A non-2xx status is reported as [`Error::Api`] when the body is a Bybit
/// error envelope and as [`Error::Http`] otherwise (e.g. an HTML page from a
/// CDN or gateway). For a 2xx status the `retCode` of the body decides.
fn parse_response<T>(status: StatusCode, headers: Headers, body: &str) -> Result<Response<T>, Error>
where
    T: serde::de::DeserializeOwned,
{
    let envelope = deserialize_json::<APIErrorResponse>(body);

    if !status.is_success() {
        return Err(match envelope {
            Ok(envelope) if envelope.ret_code != 0 => envelope.into(),
            _ => Error::Http {
                status: status.as_u16(),
                body: truncate(body, MAX_ERROR_BODY_LEN).to_owned(),
            },
        });
    }

    let envelope = envelope?;
    if envelope.ret_code != 0 {
        return Err(envelope.into());
    }

    let response: Resp<T> = deserialize_json(body)?;
    Ok(Response {
        result: response.result,
        time: response.time,
        headers,
        ret_ext_info: response.ret_ext_info,
    })
}

/// Longest prefix of `s` not exceeding `max` bytes that ends on a char boundary.
fn truncate(s: &str, max: usize) -> &str {
    if s.len() <= max {
        return s;
    }
    let mut end = max;
    while !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[..end]
}

/// Parse response headers: ret_code, traceid, timenow, X-Bapi-Limit, X-Bapi-Limit-Status, X-Bapi-Limit-Reset-Timestamp
fn parse_headers(headers: &HeaderMap) -> Headers {
    let ret_code = headers
        .get(HEADER_RET_CODE)
        .and_then(|h| h.to_str().unwrap_or_default().parse().ok());
    let trace_id = headers
        .get(HEADER_TRACE_ID)
        .and_then(|h| h.to_str().map(|str| str.into()).ok());
    let time_now = headers
        .get(HEADER_TIME_NOW)
        .and_then(|h| h.to_str().unwrap_or_default().parse().ok());

    let api_limit = headers
        .get(HEADER_X_BAPI_LIMIT)
        .and_then(|h| h.to_str().unwrap_or_default().parse().ok());
    let api_limit_status = headers
        .get(HEADER_X_BAPI_LIMIT_STATUS)
        .and_then(|h| h.to_str().unwrap_or_default().parse().ok());
    let api_limit_reset_timestamp = headers
        .get(HEADER_X_BAPI_LIMIT_RESET_TIMESTAMP)
        .and_then(|h| h.to_str().unwrap_or_default().parse().ok());

    Headers {
        ret_code,
        trace_id,
        time_now,
        api_limit,
        api_limit_status,
        api_limit_reset_timestamp,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn headers() -> Headers {
        parse_headers(&HeaderMap::new())
    }

    #[test]
    fn parse_response_success() {
        let body = r#"{"retCode":0,"retMsg":"OK","result":{"timeSecond":"1688639403","timeNano":"1688639403423213947"},"retExtInfo":{},"time":1688639403423}"#;

        let response: Response<ServerTime> =
            parse_response(StatusCode::OK, headers(), body).unwrap();

        assert_eq!(response.time, Some(1688639403423));
    }

    #[test]
    fn parse_response_api_error_with_http_200() {
        let body = r#"{"retCode":10001,"retMsg":"params error","result":{},"retExtInfo":{},"time":1688639403423}"#;

        let err = parse_response::<ServerTime>(StatusCode::OK, headers(), body).unwrap_err();

        assert!(matches!(err, Error::Api { code: 10001, ref msg } if msg == "params error"));
    }

    #[test]
    fn parse_response_api_error_with_http_error_status() {
        let body = r#"{"retCode":10006,"retMsg":"Too many visits!","result":{},"retExtInfo":{},"time":1688639403423}"#;

        let err = parse_response::<ServerTime>(StatusCode::TOO_MANY_REQUESTS, headers(), body)
            .unwrap_err();

        assert!(matches!(err, Error::Api { code: 10006, .. }));
    }

    #[test]
    fn parse_response_non_json_error_page() {
        let body = "<html><body>403 Forbidden</body></html>";

        let err = parse_response::<ServerTime>(StatusCode::FORBIDDEN, headers(), body).unwrap_err();

        assert!(matches!(err, Error::Http { status: 403, body: ref b } if b == body));
    }

    #[test]
    fn truncate_respects_char_boundaries() {
        assert_eq!(truncate("abc", 10), "abc");
        assert_eq!(truncate("ab\u{00e9}c", 3), "ab");
    }

    fn page(list: Vec<u32>, cursor: Option<&str>) -> CursorPagination<u32> {
        CursorPagination {
            category: None,
            next_page_cursor: cursor.map(String::from),
            list,
        }
    }

    /// Serves `pages` in order and records the cursors it was asked for.
    async fn collect_from(
        pages: Vec<CursorPagination<u32>>,
    ) -> (Result<Vec<u32>, Error>, Vec<Option<String>>) {
        let mut pages = pages.into_iter();
        let mut requested = Vec::new();
        let result = collect_pages(|cursor| {
            requested.push(cursor);
            let page = pages.next().expect("unexpected extra request");
            async move { Ok(page) }
        })
        .await;
        (result, requested)
    }

    #[tokio::test]
    async fn collect_pages_follows_cursor_until_none() {
        let (result, requested) = collect_from(vec![
            page(vec![1, 2], Some("a")),
            page(vec![3], Some("b")),
            page(vec![4], None),
        ])
        .await;

        assert_eq!(result.unwrap(), vec![1, 2, 3, 4]);
        assert_eq!(requested, vec![None, Some("a".into()), Some("b".into())]);
    }

    #[tokio::test]
    async fn collect_pages_stops_on_repeated_cursor() {
        let (result, _) =
            collect_from(vec![page(vec![1], Some("a")), page(vec![2], Some("a"))]).await;

        assert_eq!(result.unwrap(), vec![1, 2]);
    }

    #[tokio::test]
    async fn collect_pages_stops_on_empty_page_with_cursor() {
        let (result, _) =
            collect_from(vec![page(vec![1], Some("a")), page(vec![], Some("b"))]).await;

        assert_eq!(result.unwrap(), vec![1]);
    }

    #[tokio::test]
    async fn collect_pages_fails_after_max_pages() {
        let mut n = 0;
        let result = collect_pages(|_| {
            n += 1;
            let cursor = n.to_string();
            async move { Ok(page(vec![1], Some(&cursor))) }
        })
        .await;

        assert!(matches!(
            result,
            Err(Error::TooManyPages { max: MAX_PAGES })
        ));
        assert_eq!(n, MAX_PAGES);
    }

    #[test]
    fn time_offset_is_measured_from_the_middle_of_the_round_trip() {
        // Request sent at 1000, answer received at 1100, server said 61050.
        assert_eq!(estimate_time_offset(1000, 1100, 61_050), 60_000);
        // Local clock ahead of the server.
        assert_eq!(estimate_time_offset(1000, 1000, 400), -600);
    }

    /// Minimal HTTP server: answers every request with `body_for(path)` and
    /// records the request heads it received.
    async fn spawn_http_server(
        body_for: fn(&str) -> String,
    ) -> (String, std::sync::Arc<std::sync::Mutex<Vec<String>>>) {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let requests = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let recorded = requests.clone();
        tokio::spawn(async move {
            while let Ok((mut tcp, _)) = listener.accept().await {
                let mut head = Vec::new();
                let mut buf = [0u8; 1024];
                while !head.windows(4).any(|w| w == b"\r\n\r\n") {
                    match tcp.read(&mut buf).await {
                        Ok(0) | Err(_) => break,
                        Ok(n) => head.extend_from_slice(&buf[..n]),
                    }
                }
                let head = String::from_utf8_lossy(&head).into_owned();
                let path = head.split_whitespace().nth(1).unwrap_or("").to_owned();
                recorded.lock().unwrap().push(head);
                let body = body_for(&path);
                let response = format!(
                    "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}",
                    body.len()
                );
                let _ = tcp.write_all(response.as_bytes()).await;
            }
        });
        (format!("http://{addr}"), requests)
    }

    /// The fake server's clock runs this far ahead of the local one.
    const SERVER_AHEAD_MS: u64 = 60_000;

    fn fake_bybit(path: &str) -> String {
        if path.starts_with("/v5/market/time") {
            let server_ms = timestamp() + SERVER_AHEAD_MS;
            format!(
                r#"{{"retCode":0,"retMsg":"OK","result":{{"timeSecond":"{}","timeNano":"{}"}},"retExtInfo":{{}},"time":{server_ms}}}"#,
                server_ms / 1000,
                server_ms * 1_000_000
            )
        } else {
            r#"{"retCode":10001,"retMsg":"test","result":{},"retExtInfo":{},"time":1}"#.to_owned()
        }
    }

    #[tokio::test]
    async fn signed_requests_use_the_synchronized_server_time() {
        let (url, requests) = spawn_http_server(fake_bybit).await;
        let client = Client::new(Config::new(url).credentials("key", "secret")).unwrap();

        let offset = client.sync_time().await.unwrap();
        let _ = client.get_api_key_information().await;

        let expected = SERVER_AHEAD_MS as i64;
        assert!((offset - expected).abs() < 1000, "offset {offset}");
        assert_eq!(client.time_offset(), offset);

        let head = requests.lock().unwrap().last().cloned().unwrap();
        let sent: i64 = head
            .lines()
            .find_map(|line| {
                let (name, value) = line.split_once(':')?;
                name.eq_ignore_ascii_case(HEADER_X_BAPI_TIMESTAMP)
                    .then(|| value.trim().parse().unwrap())
            })
            .expect("no X-BAPI-TIMESTAMP header");
        let skew = sent - (timestamp() as i64 + expected);
        assert!(
            skew.abs() < 1000,
            "signed timestamp is {skew} ms off the server clock"
        );
    }

    #[tokio::test]
    async fn clones_share_the_clock_offset_and_the_rate_limiter() {
        let (url, _) = spawn_http_server(fake_bybit).await;
        let config = Config::new(url).rate_limiter(RateLimiterConfig::uniform(0.0, 2));
        let client = Client::new(config).unwrap();
        let clone = client.clone();

        clone.sync_time().await.unwrap();
        assert_ne!(client.time_offset(), 0);
        assert_eq!(client.time_offset(), clone.time_offset());

        // The clone used one of the two tokens; the original gets the last one.
        client.get_server_time().await.unwrap();
        assert!(matches!(
            clone.get_server_time().await,
            Err(Error::RateLimitUnsatisfiable { .. })
        ));
    }

    #[tokio::test]
    async fn private_endpoint_without_credentials_returns_error() {
        let client = Client::new(Config::new("http://127.0.0.1:1")).unwrap();

        let err = client.get_api_key_information().await.unwrap_err();

        assert!(matches!(err, Error::MissingCredentials));
    }
}
