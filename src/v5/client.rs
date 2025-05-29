use reqwest::{self, Method};
use serde_json::{Value, from_value};

use super::{
    Error, GetInstrumentsInfoParams, GetKLinesParams, GetTickersParams, GetTradesParams, Headers,
    InstrumentsInfo, KLine, Resp, Response, Ticker, Trade,
    url::{Path, *},
};

pub struct ClientConfig {
    pub base_url: String,
    pub api_key: String,
    pub api_secret: String,
    /// Milliseconds.
    pub recv_window: u64,
}

pub struct Client {
    cfg: ClientConfig,
}

impl Client {
    pub fn new(cfg: ClientConfig) -> Self {
        Self { cfg }
    }
}

// Market.
impl Client {
    pub async fn get_kline(&self, params: GetKLinesParams) -> Result<Response<KLine>, Error> {
        let url = format!("{}{}", self.cfg.base_url, Path::MarketKline);

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

    pub async fn get_tickers(&self, params: GetTickersParams) -> Result<Response<Ticker>, Error> {
        let url = format!("{}{}", self.cfg.base_url, Path::MarketTickers);

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
        let url = format!("{}{}", self.cfg.base_url, Path::MarketInstrumentsInfo);

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
        let url = format!("{}{}", self.cfg.base_url, Path::MarketRecentTrade);

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

fn parse_headers(headers: &reqwest::header::HeaderMap) -> Headers {
    println!("{headers:#?}");

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
