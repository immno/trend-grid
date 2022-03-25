use std::borrow::{Borrow, BorrowMut};

use anyhow::Result;
use async_trait::async_trait;
use reqwest::{Client, Url};
use ring::hmac;
use serde::Serialize;

use crate::trade::binance_api_params::{
    Interval, OrderSide, PEmpty, PKline, PQuerySpotOrder, PSpotOrder, PSymbol,
};
use crate::trade::binance_api_response::{
    QuerySpotOrder, RH24ticker, RKline, RSpotPrice, SpotOrder,
};
use crate::trade::{MarketService, TradeService};
use crate::{Symbol, TgError, TradeConfig};

pub struct BinanceTradeService {
    http_client: reqwest::Client,
    hmac_key: hmac::Key,
    api_key: String,
    base_url: reqwest::Url,
}

#[async_trait]
impl TradeService for BinanceTradeService {
    async fn get_order(&self, symbol: &Symbol) -> Result<QuerySpotOrder> {
        let param = PQuerySpotOrder::new(symbol);
        let json_str = self
            .send_request("order", reqwest::Method::GET, &param)
            .await?;
        let order: QuerySpotOrder = serde_json::from_str(json_str.as_str())?;
        Ok(order)
    }

    async fn buy_limit(&self, symbol: &Symbol, quantity: f64, price: f64) -> Result<Option<f64>> {
        self.order_ops(symbol, OrderSide::Buy, quantity, Some(price))
            .await
    }

    async fn buy(&self, symbol: &Symbol, quantity: f64) -> Result<Option<f64>> {
        self.order_ops(symbol, OrderSide::Buy, quantity, None).await
    }

    async fn sell_limit(&self, symbol: &Symbol, quantity: f64, price: f64) -> Result<Option<f64>> {
        self.order_ops(symbol, OrderSide::Sell, quantity, Some(price))
            .await
    }

    async fn sell(&self, symbol: &Symbol, quantity: f64) -> Result<Option<f64>> {
        self.order_ops(symbol, OrderSide::Sell, quantity, None)
            .await
    }
}

impl BinanceTradeService {
    pub fn new(config: &TradeConfig) -> Result<Self> {
        let base_url = reqwest::Url::parse(config.url.as_str())
            .map_err(|_| TgError::UrlError(config.url.to_string()))?;
        let http_client = build_client(config)?;
        let key = hmac::Key::new(hmac::HMAC_SHA256, config.secret.as_bytes());

        Ok(Self {
            http_client,
            hmac_key: key,
            api_key: config.key.to_string(),
            base_url,
        })
    }

    fn sign_and_query<P: Serialize>(&self, params: &P) -> Result<String> {
        let qs = serde_qs::to_string(&params)?;
        let signature = hmac::sign(self.hmac_key.borrow(), qs.as_bytes());
        let signature = hex::encode(signature.as_ref());
        Ok(format!("{}&signature={}", qs, signature))
    }

    /// send request
    async fn send_request<P: serde::Serialize>(
        &self,
        path: &str,
        method: reqwest::Method,
        params: &P,
    ) -> Result<String> {
        let query = self.sign_and_query(params)?;
        let mut url = self.base_url.join(path)?;
        // http_client.query(query.as_str()) 会报错，有点奇怪
        url.borrow_mut().set_query(Some(query.as_str()));
        let res = self
            .http_client
            .request(method, url)
            .header("Content-Type", "application/json")
            .header("X-MBX-APIKEY", self.api_key.as_str())
            .send()
            .await?;
        let resp = TgError::bina_resp(res).await?;
        Ok(resp)
    }

    /// order ops
    async fn order_ops(
        &self,
        symbol: &Symbol,
        side: OrderSide,
        quantity: f64,
        price: Option<f64>,
    ) -> Result<Option<f64>> {
        let param = PSpotOrder::new(symbol, side, quantity, price);
        let json_str = self
            .send_request("order", reqwest::Method::POST, &param)
            .await?;
        let order: SpotOrder = serde_json::from_str(json_str.as_str())?;
        let fills = match order {
            SpotOrder::Ack(_) => None,
            SpotOrder::Result(_) => None,
            SpotOrder::Full(full) => {
                if full.fills.is_empty() {
                    None
                } else {
                    Some(full.fills[0].price)
                }
            }
        };
        Ok(fills)
    }
}

fn build_client(config: &TradeConfig) -> Result<Client> {
    let http_client = match config.proxy.as_ref() {
        Some(proxy) => reqwest::Client::builder()
            .proxy(reqwest::Proxy::all(proxy)?)
            .build()?,
        None => reqwest::Client::new(),
    };
    Ok(http_client)
}

pub struct BinanceMarketService {
    http_client: reqwest::Client,
    url: Url,
}

#[async_trait]
impl MarketService for BinanceMarketService {
    async fn ping(&self) -> Result<bool> {
        let json_str = self.send_request("ping", &PEmpty).await?;
        Ok(!json_str.is_empty())
    }

    async fn ticker_price(&self, symbol: &Symbol) -> Result<f64> {
        let param = PSymbol { symbol };
        let json_str = self.send_request("ticker/price", &param).await?;
        let obj: RSpotPrice = serde_json::from_str(json_str.as_str())?;
        Ok(obj.price)
    }

    async fn ticker_24hr(&self, symbol: &Symbol) -> Result<RH24ticker> {
        let param = PSymbol { symbol };
        let json_str = self.send_request("ticker/24hr", &param).await?;
        let obj: RH24ticker = serde_json::from_str(json_str.as_str())?;
        Ok(obj)
    }

    async fn k_lines(&self, symbol: &Symbol) -> Result<Vec<RKline>> {
        let param = PKline {
            symbol,
            interval: Interval::Hour4,
            start_time: None,
            end_time: None,
            limit: Some(20),
        };
        let json_str = self.send_request("klines", &param).await?;
        let obj: Vec<RKline> = serde_json::from_str(json_str.as_str())?;
        Ok(obj)
    }
}

impl BinanceMarketService {
    pub fn new(config: &TradeConfig) -> Result<Self> {
        let url_str = config.url.as_str();
        let url = Url::parse(url_str).map_err(|_| TgError::UrlError(url_str.to_string()))?;
        let http_client = build_client(config)?;

        Ok(Self { http_client, url })
    }

    /// send request
    async fn send_request<P: serde::Serialize>(&self, path: &str, params: &P) -> Result<String> {
        let mut url = self.url.join(path)?;
        let param = serde_qs::to_string(params).map_or(None, |s| Some(s));
        url.borrow_mut().set_query(param.as_deref());
        let res = self
            .http_client
            .request(reqwest::Method::GET, url)
            .header("Content-Type", "application/json")
            .send()
            .await?;
        let resp = TgError::bina_resp(res).await?;
        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn struct_is_empty_to_json() {
        let s = serde_json::to_string(&PEmpty).unwrap();
        assert_eq!("null", s.as_str());
    }

    #[test]
    fn serde_qs() {
        let qs = serde_qs::to_string(&PEmpty).unwrap_or("".to_string());
        assert_eq!("", qs.as_str());
    }

    #[test]
    fn url_join() {
        let base_url = reqwest::Url::parse("https://api.binance.com/api/").unwrap();
        let url = base_url.join("v1/sss").unwrap();
        assert_eq!("https://api.binance.com/api/v1/sss", url.as_str());
    }

    #[test]
    fn sign() {
        let key = hmac::Key::new(
            hmac::HMAC_SHA256,
            b"2b5eb11e18796d12d88f13dc27dbbd02c2cc51ff7059765ed9821957d82bb4d9",
        );
        let signature = hmac::sign(&key, b"symbol=BTCUSDT&side=BUY&type=LIMIT&quantity=1&price=9000&timeInForce=GTC&recvWindow=5000&timestamp=1591702613943");
        assert_eq!(
            "3c661234138461fcc7a7d8746c6558c9842d4e10870d2ecbedf7777cad694af9",
            hex::encode(signature.as_ref())
        );
        let signature = hmac::sign(&key, b"");
        assert_eq!(
            "21fd819734bf0e5c68740eed892909414d693635c5f7fffab1313925ae13556a",
            hex::encode(signature.as_ref())
        );
    }

    lazy_static::lazy_static! {
        static ref TC: TradeConfig = TradeConfig {
            url: "https://testnet.binance.vision/api/v3/".to_string(),
            proxy: None,
            key: "FzNvOmpEc0iRSz958NGPx58oOUkJCUf2KctzYlI0CKCguVkZw7TBxZVolzVjnqYS".to_string(),
            secret: "fI6ObqSQriAjmMGX0JpEiLEOWcx14W8i3sY7cpLFWai6FKs6mc16ktr2mhqBOi0x".to_string(),
        };
    }

    #[tokio::test]
    async fn ping_should_be_successful() {
        // let res = BinanceMarketService::new(&TC).unwrap().ping().await;
        // assert!(res.is_ok());
    }

    #[tokio::test]
    async fn ticker_price_should_be_successful() {
        // let res = BinanceMarketService::new(&TC)
        //     .unwrap()
        //     .ticker_price(&Symbol::Eth)
        //     .await;
        // assert!(res.is_ok());
    }

    #[tokio::test]
    async fn ticker_24hr_should_be_successful() {
        // let res = BinanceMarketService::new(&TC)
        //     .unwrap()
        //     .ticker_24hr(&Symbol::Eth)
        //     .await;
        // assert!(res.is_ok());
    }

    #[tokio::test]
    async fn k_line_should_be_successful() {
        // let res = BinanceMarketService::new(&TC)
        //     .unwrap()
        //     .k_lines(&Symbol::Eth)
        //     .await;
        // assert!(res.is_ok());
    }

    #[tokio::test]
    async fn get_order_should_be_successful() {
        // let res = BinanceTradeService::new(&TC)
        //     .unwrap()
        //     .get_order(&Symbol::Eth)
        //     .await
        //     .unwrap();
    }

    #[tokio::test]
    async fn buy_should_be_successful() {
        // let res = BinanceTradeService::new(&TC)
        //     .unwrap()
        //     .buy(&Symbol::Eth, 0.003)
        //     .await.unwrap();
    }
}
