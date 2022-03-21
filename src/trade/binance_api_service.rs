use std::time::{SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use reqwest::Url;

use crate::trade::{MarketService, TickerPrice, TickerPriceDay, TradeService};
use crate::{TgError, TradeConfig};

pub struct BinanceTradeService {
    url: Url,
}

#[async_trait]
impl TradeService for BinanceTradeService {
    async fn buy_limit(&self, quantity: usize, price: Option<f64>) -> anyhow::Result<bool> {
        let mut url = self.url.clone();
        url.set_path("/fapi/v1/ping");
        Ok(reqwest::get(url).await?.status().is_success())
    }

    async fn sell_limit(&self, quantity: usize, price: Option<f64>) -> anyhow::Result<bool> {
        todo!()
    }
}

impl TryFrom<&TradeConfig> for BinanceTradeService {
    type Error = TgError;

    fn try_from(config: &TradeConfig) -> Result<Self, Self::Error> {
        // let _key = hmac::Key::new(hmac::HMAC_SHA256, config.secret.as_bytes());
        // let _signature = hmac::sign(&_key, "ss".as_bytes());
        Ok(Self {
            url: Url::parse(config.url.as_str()).unwrap(),
        })
    }
}

pub struct BinanceMarketService {
    url: Url,
}

#[async_trait]
impl MarketService for BinanceMarketService {
    async fn ping(&self) -> anyhow::Result<bool> {
        let mut url = self.url.clone();
        url.set_path("/fapi/v1/ping");
        Ok(reqwest::get(url).await?.status().is_success())
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
    fn get_now() -> String {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(now) => now.as_millis().to_string(),
            Err(_) => "0".to_string(),
        }
    }
}

impl TryFrom<&TradeConfig> for BinanceMarketService {
    type Error = TgError;

    fn try_from(config: &TradeConfig) -> Result<Self, Self::Error> {
        let url_str = match config.market.as_ref() {
            Some(market) => market.as_str(),
            _ => config.url.as_str(),
        };
        let url = Url::parse(url_str).map_err(|_| TgError::UrlError(url_str.to_string()))?;

        Ok(Self { url })
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
