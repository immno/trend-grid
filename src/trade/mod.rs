use anyhow::Result;
use async_trait::async_trait;
use tracing::info;

use crate::trade::binance_api_service::{BinanceMarketService, BinanceTradeService};
use crate::{TgError, TradeConfig, TradeType};

mod binance_api_params;
mod binance_api_service;

/// Abstraction of Market Service
#[async_trait]
pub trait MarketService {
    /// Test connectivity to the Rest API.
    async fn ping(&self) -> Result<bool>;

    /// Latest price for a symbol or symbols.
    async fn ticker_price(&self) -> Result<TickerPrice>;

    /// 24 hour rolling window price change statistics. Careful when accessing this with no symbol.
    async fn ticker_24hr(&self) -> Result<TickerPriceDay>;

    /// Kline/candlestick bars for a symbol. Klines are uniquely identified by their open time.
    async fn k_lines(&self) -> Result<bool>;
}

/// Abstraction of transaction services
#[async_trait]
pub trait TradeService {
    /// Send in a new order.
    async fn buy_limit(&self, quantity: usize, price: Option<f64>) -> Result<bool>;

    async fn buy(&self, quantity: usize) -> Result<bool>;

    /// Send in a new order.
    async fn sell_limit(&self, quantity: usize, price: Option<f64>) -> Result<bool>;

    async fn sell(&self, quantity: usize) -> Result<bool>;
}

pub fn factory(
    config: &TradeConfig,
) -> Result<(Box<dyn MarketService>, Box<dyn TradeService>), TgError> {
    info!(
        "Initialize Market&Trade Service: {:?} - {}",
        config.handle,
        config.url.as_str()
    );
    match config.handle {
        TradeType::BinanceFakeApi => {
            let m = Box::new(BinanceMarketService::new(config)?);
            let t = Box::new(BinanceTradeService::new(config)?);
            Ok((m, t))
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TickerPrice {
    pub symbol: String,
    pub price: String,
    pub time: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TickerPriceDay {
    pub symbol: String,
    pub price_change: f64,
    pub price_change_percent: f64,
    pub weighted_avg_price: f64,
    pub last_price: f64,
    pub last_qty: f64,
    pub open_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub volume: f64,
    pub quote_volume: f64,
    pub open_time: u64,
    pub close_time: u64,
    pub first_id: u32,
    // First tradeId
    pub last_id: u32,
    // Last tradeId
    pub count: usize, // Trade count
}
