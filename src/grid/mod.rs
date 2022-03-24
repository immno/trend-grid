use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tracing::info;

use crate::trade::{MarketService, TradeService};
use crate::{Coin, Symbol, TgError};

pub use self::fixed_grid_service::FixedGridService;

mod fixed_grid_service;

/// Abstraction of Grid services
#[async_trait]
pub trait GridService: Send {
    /// whether to buy
    async fn execute(&mut self, symbol: &Symbol, price: f64) -> Result<()>;
}

pub fn factory(
    symbol: &Symbol,
    config: &Coin,
    market: Arc<dyn MarketService>,
    trade: Arc<dyn TradeService>,
) -> Result<Box<dyn GridService>> {
    info!("Initialize Grid Service: {:?} - {:?}", symbol, config);
    let fgs = FixedGridService::new(config, market, trade)?;
    Ok(Box::new(fgs))
}
