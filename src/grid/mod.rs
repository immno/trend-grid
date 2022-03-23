use tracing::info;

use crate::{Coin, Symbol, TgError};

pub use self::fixed_grid_service::FixedGridService;

mod fixed_grid_service;

/// Abstraction of Grid services
pub trait GridService {
    /// whether to buy
    fn is_buy(&self, price: f64) -> bool;

    fn buy_quantity(&self) -> f64;

    /// whether to sell
    fn is_sell(&self, price: f64) -> bool;

    fn is_air(&self) -> bool;

    fn sell_quantity(&self) -> f64;

    /// 调整数据
    fn poise(&self) -> bool;

    fn record(&mut self, price: f64);

    fn modify_price(&mut self, price: f64);
}

pub fn factory(symbol: &Symbol, config: &Coin) -> Result<Box<dyn GridService>, TgError> {
    info!("Initialize Grid Service: {:?} - {:?}", symbol, config);
    let fgs = FixedGridService::new(config)?;
    Ok(Box::new(fgs))
}
