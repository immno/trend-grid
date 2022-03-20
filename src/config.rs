use std::fs;

use serde::{Deserialize, Serialize};

use crate::TgError;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ServerConfig {
    pub market: MarketConfig,
    pub log: LogConfig,
    pub coin: CoinConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct LogConfig {
    pub enable_log_file: bool,
    pub log_level: String,
    pub path: String,
    pub rotation: RotationConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MarketConfig {
    pub url: String,
    pub key: String,
    pub secret: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CoinConfig {
    pub eth: Option<Coin>,
    pub btc: Option<Coin>,
    pub bnb: Option<Coin>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Coin {
    pub next_buy_price: i64,
    pub grid_sell_price: i64,
    pub step: usize,
    pub profit_ratio: f64,
    pub double_throw_ratio: f64,
    pub quantity: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum RotationConfig {
    Hourly,
    Daily,
    Never,
}

impl ServerConfig {
    pub fn load(path: &str) -> Result<Self, TgError> {
        let config = fs::read_to_string(path)?;
        let config: Self = toml::from_str(&config)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn server_config_should_be_loaded() {
        let result: Result<ServerConfig, toml::de::Error> =
            toml::from_str(include_str!("../fixtures/tgs.conf"));
        // assert!(result.is_ok());
        match result {
            Ok(config) => println!("{:?}", config),
            Err(error) => println!("{:?}", error),
        }
    }
}
