use std::{fmt, fs};

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
    pub url: String,
    pub proxy: Option<String>,
    pub key: String,
    pub secret: String,
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
pub enum Symbol {
    #[serde(rename = "ETHUSDT")]
    Eth,
    #[serde(rename = "BTCUSDT")]
    Btc,
    #[serde(rename = "BNBUSDT")]
    Bnb,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Coin {
    pub next_buy_price: f64,
    pub grid_sell_price: f64,
    pub step: usize,
    pub profit_ratio: f64,
    pub double_throw_ratio: f64,
    pub quantity: Vec<f64>,
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
        let url = &mut config.trade.url;
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
        if let Err(ref error) = result {
            println!("{:?}", error);
        }
        assert!(result.is_ok());
    }
}
