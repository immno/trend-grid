use std::time::{SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use reqwest::Url;

use crate::trade::{MarketService, TickerPrice, TickerPriceDay, TradeService};
use crate::{TgError, TradeConfig};

pub struct BinanceTradeService {
    url: Url,
}

impl TradeService for BinanceTradeService {
    fn order(&self) -> anyhow::Result<bool> {
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

    use super::*;

    #[tokio::test]
    async fn ping() {
        // let data = reqwest::get("https://fapi.binance.com/fapi/v1/time")
        //     .await
        //     .unwrap()
        //     .json::<HashMap<String, String>>()
        //     .await
        //     .unwrap();
        // println!("{:?}", data);
    }

    #[test]
    fn sign() {
        let url = Url::parse("https://fapi.binance.com/fapi/v1/time?sd=23").unwrap();
        let mut url1 = url.clone();
        url1.set_path("/sdf/gfg");
        url1.query_pairs_mut().append_pair("A", "B");
        println!("{}", url1.to_string());
        println!("{:?}", BinanceMarketService::get_now());
    }
}
