use std::time::{SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use reqwest::Url;
use ring::hmac;

use crate::market::MarketService;
use crate::{MarketConfig, TgError};

struct BinanceService {
    url: Url,
}

#[async_trait]
impl MarketService for BinanceService {
    async fn ping(self) -> anyhow::Result<bool> {
        let mut url = self.url.clone();
        url.set_path("/fapi/v1/ping");
        Ok(reqwest::get(url).await?.status().is_success())
    }

    async fn sign(self) -> bool {
        todo!()
    }
}

impl BinanceService {
    fn get_now() -> String {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(now) => now.as_millis().to_string(),
            Err(_) => "0".to_string(),
        }
    }
}

impl TryFrom<&MarketConfig> for BinanceService {
    type Error = TgError;

    fn try_from(config: &MarketConfig) -> Result<Self, Self::Error> {
        let url_str = config.url.as_str();
        let url = Url::parse(url_str).map_err(|_| TgError::UrlError(url_str.to_string()))?;
        let _key = hmac::Key::new(hmac::HMAC_SHA256, config.secret.as_bytes());
        let _signature = hmac::sign(&_key, "ss".as_bytes());

        Ok(Self { url })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[tokio::test]
    // async fn ping() {
    //     let data = reqwest::get("https://fapi.binance.com/fapi/v1/time")
    //         .await
    //         .unwrap()
    //         .json::<HashMap<String, String>>()
    //         .await
    //         .unwrap();
    //     println!("{:?}", data);
    // }

    #[test]
    fn sign() {
        let url = Url::parse("https://fapi.binance.com/fapi/v1/time?sd=23").unwrap();
        let mut url1 = url.clone();
        url1.set_path("/sdf/gfg");
        url1.query_pairs_mut().append_pair("A", "B");
        println!("{}", url1.to_string());
        println!("{:?}", BinanceService::get_now());
    }
}
