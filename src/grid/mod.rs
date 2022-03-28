use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use tracing::info;

use crate::trade::{MarketService, TradeService};
use crate::{Coin, Symbol};

pub use self::grid_service::FixedGridService;

mod grid_service;

/// Abstraction of Grid services
#[async_trait]
pub trait GridService: Send {
    /// whether to buy
    async fn execute(&mut self, price: f64) -> Result<()>;
}

pub fn factory(
    symbol: &Symbol,
    config: &Coin,
    market: Arc<dyn MarketService>,
    trade: Arc<dyn TradeService>,
) -> Result<Box<dyn GridService>> {
    info!("Initialize Grid Service: {:?} - {:?}", symbol, config);
    let fgs = FixedGridService::new(symbol.clone(), config, market, trade)?;
    Ok(Box::new(fgs))
}
