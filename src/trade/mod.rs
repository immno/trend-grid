use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use tracing::info;

use crate::trade::binance_api_response::{QuerySpotOrder, RH24ticker, RKline, SpotAccount};
use crate::trade::binance_api_service::{BinanceMarketService, BinanceTradeService};
use crate::{Symbol, TradeConfig};

mod binance_api_params;
mod binance_api_response;
mod binance_api_service;
mod binance_api_ws;

/// Abstraction of Market Service
#[async_trait]
pub trait MarketService: Send + Sync + 'static {
    /// Test connectivity to the Rest API.
    async fn ping(&self) -> Result<bool>;

    /// Latest price for a symbol or symbols.
    async fn ticker_price(&self, symbol: &Symbol) -> Result<f64>;

    /// 24 hour rolling window price change statistics. Careful when accessing this with no symbol.
    async fn ticker_24hr(&self, symbol: &Symbol) -> Result<RH24ticker>;

    /// Kline/candlestick bars for a symbol. Klines are uniquely identified by their open time.
    async fn k_lines(&self, symbol: &Symbol) -> Result<Vec<RKline>>;
}

/// Abstraction of transaction services
#[async_trait]
pub trait TradeService: Send + Sync + 'static {
    async fn get_order(&self, symbol: &Symbol) -> Result<QuerySpotOrder>;

    /// Send in a new order.
    async fn buy_limit(&self, symbol: &Symbol, quantity: f64, price: f64) -> Result<Option<f64>>;

    async fn buy(&self, symbol: &Symbol, quantity: f64) -> Result<Option<f64>>;

    /// Send in a new order.
    async fn sell_limit(&self, symbol: &Symbol, quantity: f64, price: f64) -> Result<Option<f64>>;

    async fn sell(&self, symbol: &Symbol, quantity: f64) -> Result<Option<f64>>;

    async fn account(&self) -> Result<SpotAccount>;
}

pub trait WebsocketResponse<R: serde::de::DeserializeOwned> {
    fn read_stream_single(&mut self) -> Result<R>;
    fn read_stream_multi(&mut self) -> Result<R>;
    fn close_stream(&mut self);
}

pub fn factory(config: &TradeConfig) -> Result<(Arc<dyn MarketService>, Arc<dyn TradeService>)> {
    info!("Initialize Market&Trade Service: {}", config.url.as_str());
    let m = Arc::new(BinanceMarketService::new(config)?);
    let t = Arc::new(BinanceTradeService::new(config)?);
    Ok((m, t))
}
