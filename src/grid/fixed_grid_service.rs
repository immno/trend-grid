use std::collections::HashMap;

use crate::grid::GridService;
use crate::trade::TickerPrice;
use crate::Coin;

pub struct FixedGridService {
    pub buy: f64,
    pub sell: f64,
    pub history: Vec<f64>,
    pub profit_ratio: f64,
    pub double_throw_ratio: f64,
    pub quantity: usize,
    pub step: usize,
}

impl GridService for FixedGridService {
    fn is_buy(&self, price: &TickerPrice) -> bool {
        todo!()
    }

    fn is_sell(&self, price: &TickerPrice) -> bool {
        todo!()
    }

    fn poise(&self) -> bool {
        todo!()
    }

    fn is_air(&self) -> bool {
        self.step == 0
    }

    fn modify_price(&self) -> bool {
        todo!()
    }
}

impl From<&Coin> for FixedGridService {
    fn from(c: &Coin) -> Self {
        Self {
            buy: c.next_buy_price,
            sell: c.grid_sell_price,
            history: Vec::new(),
            profit_ratio: c.profit_ratio,
            double_throw_ratio: c.double_throw_ratio,
            quantity: c.quantity,
            step: c.step,
        }
    }
}
