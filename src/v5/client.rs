use reqwest::{self, Method, header::HeaderMap};
use serde_json::{Value, from_value};

use crate::v5::crypto::Signer;

use super::{
    CursorPagination, Error, GetInstrumentsInfoParams, GetKLinesParams, GetOpenClosedOrdersParams,
    GetTickersParams, GetTradesParams, Headers, InstrumentsInfo, KLine, Order, Resp, Response,
    ServerTime, Ticker, Trade, crypto::SensitiveString, url::*,
};

pub struct ClientConfig {
    pub base_url: String,
    pub api_key: Option<SensitiveString>,
    pub api_secret: Option<SensitiveString>,
    /// Milliseconds.
    pub recv_window: u64,
    /// HTTP the header for broker users only.
    pub referer: Option<String>,
}

pub struct Client {
    base_url: String,
    headers: HeaderMap,
    signer: Option<Signer>,
}

impl Client {
    pub fn new(cfg: ClientConfig) -> Self {
        let mut headers = HeaderMap::new();

        if let Some(api_key) = cfg.api_key.as_ref() {
            let api_key = api_key.expose().parse().unwrap();
            headers.append(HEADER_X_BAPI_API_KEY, api_key);
        }

        let recv_window = cfg.recv_window.to_string().parse().unwrap();
        headers.append(HEADER_X_BAPI_RECV_WINDOW, recv_window);

        if let Some(referer) = cfg.referer {
            let referer = referer.parse().unwrap();
            headers.append(HEADER_X_REFERER, referer);
        }

        let signer = cfg
            .api_secret
            .clone()
            .map(|api_secret| Signer::new(cfg.api_key.unwrap(), api_secret, cfg.recv_window, None));

        Self {
            base_url: cfg.base_url,
            headers,
            signer,
        }
    }

    fn get_signed_headers(&self, s: &str) -> HeaderMap {
        let mut headers = self.headers.clone();

        let (signature, timestamp) = self.signer.as_ref().unwrap().sign(s);
        let signature = signature.parse().unwrap();
        headers.append(HEADER_X_BAPI_SIGN, signature);
        let timestamp = timestamp.parse().unwrap();
        headers.append(HEADER_X_BAPI_TIMESTAMP, timestamp);

        headers
    }
}

// Market.
impl Client {
    pub async fn get_server_time(&self) -> Result<Response<ServerTime>, Error> {
        let url = format!("{}{}", self.base_url, Path::MarketServerTime);

        let client = reqwest::Client::builder().build()?;
        let request = client.request(Method::GET, url);

        let response = request.send().await?;
        let headers = parse_headers(&response.headers());
        let response: Resp<Value> = response.json().await?;
        if response.ret_code != 0 {
            return Err(response.into());
        }

        let result = from_value(response.result)?;
        let response = Response {
            result,
            time: response.time,
            headers,
        };
        Ok(response)
    }

    pub async fn get_kline(&self, params: GetKLinesParams) -> Result<Response<KLine>, Error> {
        let url = format!("{}{}", self.base_url, Path::MarketKline);

        let client = reqwest::Client::builder().build()?;
        let request = client.request(Method::GET, url).query(&params);

        let response = request.send().await?;
        let headers = parse_headers(&response.headers());
        let response: Resp<Value> = response.json().await?;
        if response.ret_code != 0 {
            return Err(response.into());
        }

        let result = from_value(response.result)?;
        let response = Response {
            result,
            time: response.time,
            headers,
        };
        Ok(response)
    }

    /// Get Tickers
    /// Query for the latest price snapshot, best bid/ask price, and trading volume in the last 24 hours.
    /// If category=option, symbol or baseCoin must be passed.
    pub async fn get_tickers(&self, params: GetTickersParams) -> Result<Response<Ticker>, Error> {
        let url = format!("{}{}", self.base_url, Path::MarketTickers);

        let client = reqwest::Client::builder().build()?;
        let request = client.request(Method::GET, url).query(&params);

        let response = request.send().await?;
        let headers = parse_headers(&response.headers());
        let response: Resp<Value> = response.json().await?;
        if response.ret_code != 0 {
            return Err(response.into());
        }

        let result = from_value(response.result)?;
        let response = Response {
            result,
            time: response.time,
            headers,
        };
        Ok(response)
    }

    pub async fn get_instruments_info(
        &self,
        params: GetInstrumentsInfoParams,
    ) -> Result<Response<InstrumentsInfo>, Error> {
        let url = format!("{}{}", self.base_url, Path::MarketInstrumentsInfo);

        let client = reqwest::Client::builder().build()?;
        let request = client.request(Method::GET, url).query(&params);

        let response = request.send().await?;
        let headers = parse_headers(&response.headers());
        let response: Resp<Value> = response.json().await?;
        if response.ret_code != 0 {
            return Err(response.into());
        }

        let result = from_value(response.result)?;
        let response = Response {
            result,
            time: response.time,
            headers,
        };
        Ok(response)
    }

    pub async fn get_public_recent_trading_history(
        &self,
        params: GetTradesParams,
    ) -> Result<Response<Trade>, Error> {
        let url = format!("{}{}", self.base_url, Path::MarketRecentTrade);

        let client = reqwest::Client::builder().build()?;
        let request = client.request(Method::GET, url).query(&params);

        let response = request.send().await?;
        let headers = parse_headers(&response.headers());
        let response: Resp<Value> = response.json().await?;
        if response.ret_code != 0 {
            return Err(response.into());
        }

        let result = from_value(response.result)?;
        let response = Response {
            result,
            time: response.time,
            headers,
        };
        Ok(response)
    }
}

// Trade.
impl Client {
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
    pub async fn get_open_closed_orders(
        &self,
        params: GetOpenClosedOrdersParams,
    ) -> Result<Response<CursorPagination<Order>>, Error> {
        let query = serde_urlencoded::to_string(&params)?;
        let url = format!("{}{}?{query}", self.base_url, Path::OrderRealtime);
        let headers = self.get_signed_headers(&query);

        let client = reqwest::Client::builder().build()?;
        let request = client
            .request(Method::GET, url)
            // .query(&query)
            .headers(headers);

        let response = request.send().await?;
        let headers = parse_headers(&response.headers());
        let response: Resp<Value> = response.json().await?;
        if response.ret_code != 0 {
            return Err(response.into());
        }

        let result = from_value(response.result)?;
        let response = Response {
            result,
            time: response.time,
            headers,
        };
        Ok(response)
    }
}

/// Parse response headers: ret_code, traceid, timenow, X-Bapi-Limit, X-Bapi-Limit-Status, X-Bapi-Limit-Reset-Timestamp
fn parse_headers(headers: &HeaderMap) -> Headers {
    let ret_code = headers
        .get(HEADER_RET_CODE)
        .map(|h| h.to_str().unwrap_or_default().parse().ok())
        .flatten();
    let trace_id = headers
        .get(HEADER_TRACE_ID)
        .map(|h| h.to_str().map(|str| str.into()).ok())
        .flatten();
    let time_now = headers
        .get(HEADER_TIME_NOW)
        .map(|h| h.to_str().unwrap_or_default().parse().ok())
        .flatten();

    let api_limit = headers
        .get(HEADER_X_BAPI_LIMIT)
        .map(|h| h.to_str().unwrap_or_default().parse().ok())
        .flatten();
    let api_limit_status = headers
        .get(HEADER_X_BAPI_LIMIT_STATUS)
        .map(|h| h.to_str().map(|str| str.into()).ok())
        .flatten();
    let api_limit_reset_timestamp = headers
        .get(HEADER_X_BAPI_LIMIT_RESET_TIMESTAMP)
        .map(|h| h.to_str().unwrap_or_default().parse().ok())
        .flatten();

    Headers {
        ret_code,
        trace_id,
        time_now,
        api_limit,
        api_limit_status,
        api_limit_reset_timestamp,
    }
}
