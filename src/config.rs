use std::fs;

use serde::{Deserialize, Serialize};

use crate::TgError;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ServerConfig {
    pub trade: TradeConfig,
    pub coin: CoinConfig,
    pub log: LogConfig,
}

impl AsRef<TradeConfig> for TradeConfig {
    fn as_ref(&self) -> &TradeConfig {
        self
    }
}

impl AsRef<CoinConfig> for CoinConfig {
    fn as_ref(&self) -> &CoinConfig {
        self
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TradeConfig {
    pub handle: TradeType,
    pub market: Option<String>,
    pub url: String,
    pub key: String,
    pub secret: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum TradeType {
    // BinanceProdApi,
    // BinanceProdWs,
    BinanceFakeApi,
    // BinanceFakeWs,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CoinConfig {
    pub eth: Option<Coin>,
    pub btc: Option<Coin>,
    pub bnb: Option<Coin>,
}

impl AsRef<Coin> for Coin {
    fn as_ref(&self) -> &Coin {
        self
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Coin {
    pub next_buy_price: f64,
    pub grid_sell_price: f64,
    pub step: usize,
    pub profit_ratio: f64,
    pub double_throw_ratio: f64,
    pub quantity: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct LogConfig {
    pub enable_log_file: bool,
    pub log_level: String,
    pub path: String,
    pub rotation: LogRotationType,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum LogRotationType {
    Hourly,
    Daily,
    Never,
}

impl ServerConfig {
    pub fn load(path: &str) -> Result<Self, TgError> {
        let str = fs::read_to_string(path)?;
        let config = Self::from_str(str.as_str())?;
        Ok(config)
    }

    pub fn from_str(s: &str) -> Result<Self, TgError> {
        let mut config: Self = toml::from_str(s)?;
        let mut url = &mut config.trade.url;
        if !url.ends_with('/') {
            url.push('/');
        }
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn server_config_should_be_loaded() {
        let result = ServerConfig::from_str(include_str!("../fixtures/tgs.conf"));
        assert!(result.is_ok());
        match result {
            Ok(config) => println!("{:?}", config),
            Err(error) => println!("{:?}", error),
        }
    }
}
