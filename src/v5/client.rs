use reqwest::{self, Method};
use serde_json::{Value, from_value};

use super::{
    GetInstrumentsInfoParams, GetKLinesParams, GetTickersParams, GetTradesParams, InstrumentsInfo,
    KLine, Response, Ticker, Trade, url::Path,
};

pub struct Client {
    base_url: String,
}

impl Client {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_owned(),
        }
    }

    pub async fn get_kline(&self, params: GetKLinesParams) -> anyhow::Result<KLine> {
        let url = format!("{}{}", self.base_url, Path::MarketKline);

        let client = reqwest::Client::builder().build()?;
        let request_builder = client.request(Method::GET, url).query(&params);

        let response = request_builder.send().await?;
        let response: Response<Value> = response.json().await?;
        if response.ret_code != 0 {
            let _message = response.ret_msg;
            // TODO: return error
        }

        let response = from_value(response.result)?;
        Ok(response)
    }

    pub async fn get_tickers(&self, params: GetTickersParams) -> anyhow::Result<Ticker> {
        let url = format!("{}{}", self.base_url, Path::MarketTickers);

        let client = reqwest::Client::builder().build()?;
        let request_builder = client.request(Method::GET, url).query(&params);

        let response = request_builder.send().await?;
        let response: Response<Value> = response.json().await?;
        if response.ret_code != 0 {
            let _message = response.ret_msg;
            // TODO: return error
        }

        let response = from_value(response.result)?;
        Ok(response)
    }

    pub async fn get_instruments_info(
        &self,
        params: GetInstrumentsInfoParams,
    ) -> anyhow::Result<InstrumentsInfo> {
        let url = format!("{}{}", self.base_url, Path::MarketInstrumentsInfo);

        let client = reqwest::Client::builder().build()?;
        let request_builder = client.request(Method::GET, url).query(&params);

        let response = request_builder.send().await?;
        let response: Response<Value> = response.json().await?;
        if response.ret_code != 0 {
            let _message = response.ret_msg;
            // TODO: return error
        }

        let response = from_value(response.result)?;
        Ok(response)
    }

    pub async fn get_public_recent_trading_history(
        &self,
        params: GetTradesParams,
    ) -> anyhow::Result<Trade> {
        let url = format!("{}{}", self.base_url, Path::MarketRecentTrade);

        let client = reqwest::Client::builder().build()?;
        let request_builder = client.request(Method::GET, url).query(&params);

        let response = request_builder.send().await?;
        let response: Response<Value> = response.json().await?;
        if response.ret_code != 0 {
            let _message = response.ret_msg;
            // TODO: return error
        }

        let response = from_value(response.result)?;
        Ok(response)
    }
}
