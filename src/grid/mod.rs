use anyhow::Result;

use crate::trade::TickerPrice;

pub use self::fixed_grid_service::FixedGridService;

mod fixed_grid_service;

/// Abstraction of Grid services
pub trait GridService {
    /// whether to buy
    fn is_buy(&self, price: &TickerPrice) -> bool;

    /// whether to sell
    fn is_sell(&self, price: &TickerPrice) -> bool;

    /// 调整数据
    fn poise(&self) -> bool;
    fn is_air(&self) -> bool;
    fn modify_price(&self) -> bool;
}

#[derive(Default)]
pub struct SimpleFactory;

impl SimpleFactory {
    pub fn new() -> Self {
        Self {}
    }
}
