use std::borrow::Borrow;
use std::time::{SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use reqwest::Url;
use ring::hmac;
use serde::Serialize;

use crate::trade::{MarketService, TickerPrice, TickerPriceDay, TradeService};
use crate::{TgError, TradeConfig};

pub type BianResult<T> = Result<T, TgError>;

pub struct BinanceTradeService {
    http_client: reqwest::Client,
    hmac_key: hmac::Key,
    pub api_key: String,
    pub base_url: reqwest::Url,
}

#[async_trait]
impl TradeService for BinanceTradeService {
    async fn buy_limit(&self, quantity: usize, price: Option<f64>) -> anyhow::Result<bool> {
        let mut url = self.base_url.clone();
        url.set_path("/fapi/v1/ping");
        Ok(reqwest::get(url).await?.status().is_success())
    }

    async fn buy(&self, quantity: usize) -> anyhow::Result<bool> {
        todo!()
    }

    async fn sell_limit(&self, quantity: usize, price: Option<f64>) -> anyhow::Result<bool> {
        todo!()
    }

    async fn sell(&self, quantity: usize) -> anyhow::Result<bool> {
        todo!()
    }
}

impl BinanceTradeService {
    pub fn new(config: &TradeConfig) -> Result<Self, TgError> {
        let base_url = reqwest::Url::parse(config.url.as_str())
            .map_err(|_| TgError::UrlError(config.url.to_string()))?;
        let http_client = reqwest::Client::new();
        let key = hmac::Key::new(hmac::HMAC_SHA256, config.secret.as_bytes());

        Ok(Self {
            http_client,
            hmac_key: key,
            api_key: config.key.to_string(),
            base_url,
        })
    }

    fn sign<P: Serialize>(&self, params: &P) -> anyhow::Result<String> {
        let qs = serde_qs::to_string(&params)?;
        let signature = hmac::sign(self.hmac_key.borrow(), qs.as_bytes());
        let signature = hex::encode(signature.as_ref());
        Ok(signature)
    }

    /// send request
    async fn send_request(
        &self,
        url: reqwest::Url,
        method: reqwest::Method,
    ) -> anyhow::Result<bool> {
        let res = self
            .http_client
            .request(method, url)
            .header("Content-Type", "application/json")
            .header("X-MBX-APIKEY", self.api_key.as_str())
            .send()
            .await?;
        let resp = TgError::bina_resp(res).await?;
        Ok(resp.is_empty())
    }
}

pub struct BinanceMarketService {
    http_client: reqwest::Client,
    pub url: Url,
}

#[async_trait]
impl MarketService for BinanceMarketService {
    async fn ping(&self) -> anyhow::Result<bool> {
        let url = self.url.join("/fapi/v1/ping")?;
        let json_resp = self.send_request(url, reqwest::Method::GET).await;
        json_resp
    }

    async fn ticker_price(&self) -> anyhow::Result<TickerPrice> {
        todo!()
    }

    async fn ticker_24hr(&self) -> anyhow::Result<TickerPriceDay> {
        todo!()
    }

    async fn k_lines(&self) -> anyhow::Result<bool> {
        todo!()
    }
}

impl BinanceMarketService {
    pub fn new(config: &TradeConfig) -> Result<Self, TgError> {
        let url_str = match config.market.as_ref() {
            Some(market) => market.as_str(),
            _ => config.url.as_str(),
        };
        let url = Url::parse(url_str).map_err(|_| TgError::UrlError(url_str.to_string()))?;
        let http_client = reqwest::Client::new();

        Ok(Self { http_client, url })
    }

    fn get_now() -> String {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(now) => now.as_millis().to_string(),
            Err(_) => "0".to_string(),
        }
    }

    /// send request
    async fn send_request(
        &self,
        url: reqwest::Url,
        method: reqwest::Method,
    ) -> anyhow::Result<bool> {
        let res = self
            .http_client
            .request(method, url)
            .header("Content-Type", "application/json")
            .send()
            .await?;
        let resp = TgError::bina_resp(res).await?;
        Ok(resp.is_empty())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use hex::ToHex;
    use ring::hmac;

    use super::*;

    #[tokio::test]
    async fn ping() {
        // let mut s = HashMap::new();
        // s.insert("type", "MARKET");
        // s.insert("symbol", "BTCUSDT");
        // s.insert("side", "BUY");
        // s.insert("quantity", "1");
        // s.insert("recvWindow", "5000");
        // let now = BinanceMarketService::get_now();
        // s.insert("timestamp", now.as_str());
        //
        // let mut url = reqwest::Url::parse("https://fapi.binance.com/fapi/v1/order").unwrap();
        // url.query_pairs_mut().clear();
        //
        // for (key, value) in s.iter() {
        //     url.query_pairs_mut().append_pair(key, value);
        // }
        //
        // let q = url.query().unwrap();
        // let key = hmac::Key::new(
        //     hmac::HMAC_SHA256,
        //     "4b42ddee81f19826959a40249d339eaae87a2caac1bce690c18a5ca52a2c3cfd".as_bytes(),
        // );
        // let signature = hmac::sign(&key, q.as_bytes());
        // let signature = hex::encode(signature.as_ref());
        // s.insert("signature", signature.as_str());
        // url.query_pairs_mut()
        //     .append_pair("signature", signature.as_str());
        //
        // println!("{}", url.as_str());
        // println!("https://fapi/binance.com/fapi/v1/order?symbol=BTCUSDT&side=BUY&type=LIMIT&quantity=1&price=9000&timeInForce=GTC&recvWindow=5000&timestamp=1591702613943&signature= 3c661234138461fcc7a7d8746c6558c9842d4e10870d2ecbedf7777cad694af9");
        // let client = reqwest::Client::new();
        // let mut build = client.post(url).header(
        //     "X-MBX-APIKEY",
        //     "5eb75348011c84276e69fd9f669a91fbd4a0e64e49405d0c218acc52ec600b8c",
        // );
        // let res = build
        //     // .query(url.as_str())
        //     .send()
        //     .await
        //     .unwrap()
        //     .text()
        //     .await
        //     .unwrap();
        // println!("{:?}", res);
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
    }

    #[test]
    fn sign2() {
        let url = Url::parse("https://fapi.binance.com/fapi/v1/time?sd=23").unwrap();
        let mut url1 = url.clone();
        url1.set_path("/sdf/gfg");
        url1.query_pairs_mut().append_pair("A", "B");
        println!("{}", url1.to_string());
        println!("{:?}", BinanceMarketService::get_now());
    }
}
