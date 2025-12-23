use reqwest::{self, Method, RequestBuilder, header::HeaderMap};

use crate::v5::{
    APIErrorResponse, APIKeyInformation, AccountInfo, AmendOrderRequest, AmendOrderResponse,
    GetPositionInfoParams, GetWalletBalanceParams, List, PlaceOrderRequest, PlaceOrderResponse,
    Position, Timestamp, WalletBalance,
    crypto::Signer,
    serde::{deserialize_str, serialize, serialize_query},
};

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
    pub recv_window: Timestamp,
    /// HTTP the header for broker users only.
    pub referer: Option<String>,
}

// TODO: use proxy
#[derive(Debug)]
pub struct Client {
    base_url: String,
    headers: HeaderMap,
    // TODO: use one instance reqwest::Client
    // client: reqwest::Client,
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

        let response = send(request).await?;
        Ok(response)
    }

    pub async fn get_kline(&self, params: GetKLinesParams) -> Result<Response<KLine>, Error> {
        let url = format!("{}{}", self.base_url, Path::MarketKline);

        let client = reqwest::Client::builder().build()?;
        let request = client.request(Method::GET, url).query(&params);

        let response = send(request).await?;
        Ok(response)
    }

    /// Get Tickers
    /// Query for the latest price snapshot, best bid/ask price, and trading volume in the last 24 hours.
    /// If category=option, symbol or baseCoin must be passed.
    pub async fn get_tickers(&self, params: GetTickersParams) -> Result<Response<Ticker>, Error> {
        let url = format!("{}{}", self.base_url, Path::MarketTickers);

        let client = reqwest::Client::builder().build()?;
        let request = client.request(Method::GET, url).query(&params);

        let response = send(request).await?;
        Ok(response)
    }

    pub async fn get_instruments_info(
        &self,
        params: GetInstrumentsInfoParams,
    ) -> Result<Response<InstrumentsInfo>, Error> {
        let url = format!("{}{}", self.base_url, Path::MarketInstrumentsInfo);

        let client = reqwest::Client::builder().build()?;
        let request = client.request(Method::GET, url).query(&params);

        let response = send(request).await?;
        Ok(response)
    }

    pub async fn get_public_recent_trading_history(
        &self,
        params: GetTradesParams,
    ) -> Result<Response<Trade>, Error> {
        let url = format!("{}{}", self.base_url, Path::MarketRecentTrade);

        let client = reqwest::Client::builder().build()?;
        let request = client.request(Method::GET, url).query(&params);

        let response = send(request).await?;
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
    pub async fn place_order(
        &self,
        request: PlaceOrderRequest,
    ) -> Result<Response<PlaceOrderResponse>, Error> {
        let url = format!("{}{}", self.base_url, Path::TradeOrderCreate);
        let json = serialize(&request)?;
        let headers = self.get_signed_headers(&json);

        let client = reqwest::Client::builder().build()?;
        let request = client
            .request(Method::POST, url)
            .headers(headers)
            .body(json);

        let response = send(request).await?;
        Ok(response)
    }

    /// Amend Order
    /// info
    /// You can only modify unfilled or partially filled orders.
    pub async fn amend_order(
        &self,
        request: AmendOrderRequest,
    ) -> Result<Response<AmendOrderResponse>, Error> {
        let url = format!("{}{}", self.base_url, Path::TradeOrderAmend);
        let json = serialize(&request)?;
        let headers = self.get_signed_headers(&json);

        let client = reqwest::Client::builder().build()?;
        let request = client
            .request(Method::POST, url)
            .headers(headers)
            .body(json);

        let response = send(request).await?;
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
    pub async fn get_open_closed_orders(
        &self,
        params: GetOpenClosedOrdersParams,
    ) -> Result<Response<CursorPagination<Order>>, Error> {
        let query = serialize_query(&params)?;
        let url = format!("{}{}?{query}", self.base_url, Path::TradeOrderRealtime);
        let headers = self.get_signed_headers(&query);

        let client = reqwest::Client::builder().build()?;
        let request = client.request(Method::GET, url).headers(headers);

        let response = send(request).await?;
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
    pub async fn get_position_info(
        &self,
        params: GetPositionInfoParams,
    ) -> Result<Response<CursorPagination<Position>>, Error> {
        let query = serialize_query(&params)?;
        let url = format!("{}{}?{query}", self.base_url, Path::PositionList);
        let headers = self.get_signed_headers(&query);

        let client = reqwest::Client::builder().build()?;
        let request = client.request(Method::GET, url).headers(headers);

        let response = send(request).await?;
        Ok(response)
    }
}

// Account.
impl Client {
    /// Obtain wallet balance, query asset information of each currency. By default, currency information with assets or liabilities of 0 is not returned.
    pub async fn get_wallet_balance(
        &self,
        params: GetWalletBalanceParams,
    ) -> Result<Response<List<WalletBalance>>, Error> {
        let query = serialize_query(&params)?;
        let url = format!("{}{}?{query}", self.base_url, Path::AccountWalletBalance);
        let headers = self.get_signed_headers(&query);

        let client = reqwest::Client::builder().build()?;
        let request = client.request(Method::GET, url).headers(headers);

        let response = send(request).await?;
        Ok(response)
    }

    /// Query the account information, like margin mode, account mode, etc.
    pub async fn get_account_info(&self) -> Result<Response<AccountInfo>, Error> {
        let url = format!("{}{}", self.base_url, Path::AccountInfo);
        let query = "";
        let headers = self.get_signed_headers(query);

        let client = reqwest::Client::builder().build()?;
        let request = client.request(Method::GET, url).headers(headers);

        let response = send(request).await?;
        Ok(response)
    }
}

// User.
impl Client {
    /// Get API Key Information.
    /// Get the information of the api key. Use the api key pending to be checked to call the endpoint. Both master and sub user's api key are applicable.
    pub async fn get_api_key_information(&self) -> Result<Response<APIKeyInformation>, Error> {
        let url = format!("{}{}", self.base_url, Path::UserQueryApi);
        let query = "";
        let headers = self.get_signed_headers(query);

        // TODO: The `Client` holds a connection pool internally, so it is advised that you create one and **reuse** it.
        let client = reqwest::Client::builder().build()?;
        let request = client.request(Method::GET, url).headers(headers);

        let response = send(request).await?;
        Ok(response)
    }
}

async fn send<T>(request: RequestBuilder) -> Result<Response<T>, Error>
where
    T: serde::de::DeserializeOwned,
{
    let response = request.send().await?;
    let headers = parse_headers(response.headers());
    let json = response.text().await?;
    if !headers.is_ret_code_ok() {
        let msg: APIErrorResponse = deserialize_str(&json)?;
        return Err(msg.into());
    }

    let response: Resp<_> = deserialize_str(&json)?;
    let response = Response {
        result: response.result,
        time: response.time,
        headers,
    };
    Ok(response)
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
