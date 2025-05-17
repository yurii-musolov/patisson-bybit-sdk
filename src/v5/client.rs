pub struct Client {
    base_url: String,
}

impl Client {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }

    pub fn get_tickers(&self) -> Result<String, Box<dyn std::error::Error>> {
        let tickers = "BTSUSDT,BNBUSDT,ETHUSDT,ADAUSDT";
        let res = format!("Ticker list from {}:\n{}", self.base_url, tickers);
        Ok(res)
    }
}
