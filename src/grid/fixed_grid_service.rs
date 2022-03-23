use std::borrow::{Borrow, BorrowMut};

use crate::grid::GridService;
use crate::trade::TickerPrice;
use crate::{Coin, TgError};

pub struct FixedGridService {
    pub buy: f64,
    pub sell: f64,
    pub quantity: Vec<f64>,
    pub profit_ratio: f64,
    pub double_throw_ratio: f64,
    pub step: usize,
    pub history: Vec<f64>,
}

impl GridService for FixedGridService {
    fn is_buy(&self, price: f64) -> bool {
        self.buy >= price
    }

    fn buy_quantity(&self) -> f64 {
        self.get_quantity(true)
    }

    fn is_sell(&self, price: f64) -> bool {
        self.sell < price
    }

    fn is_air(&self) -> bool {
        self.step == 0
    }

    fn sell_quantity(&self) -> f64 {
        self.get_quantity(false)
    }

    fn poise(&self) -> bool {
        todo!()
    }

    fn record(&mut self, price: f64) {
        self.history.push(price);
    }

    fn modify_price(&mut self, price: f64) {
        self.buy = self.sell * (1.0 - self.double_throw_ratio / 100.0);
        self.sell = self.sell * (1.0 + self.double_throw_ratio / 100.0);
        if self.is_buy(price) {
            self.buy = price * (1.0 - self.double_throw_ratio / 100.0);
        } else if self.is_sell(price) {
            self.sell = price * (1.0 + self.double_throw_ratio / 100.0);
        }
    }
}

impl FixedGridService {
    pub fn new(c: &Coin) -> Result<Self, TgError> {
        if c.quantity.is_empty() {
            return Err(TgError::ConfError("quantity cannot be empty".to_string()));
        }
        Ok(Self {
            buy: c.next_buy_price,
            sell: c.grid_sell_price,
            history: Vec::new(),
            profit_ratio: c.profit_ratio,
            double_throw_ratio: c.double_throw_ratio,
            quantity: c.quantity.to_vec(),
            step: c.step,
        })
    }

    fn get_quantity(&self, buy: bool) -> f64 {
        let step = if buy { self.step } else { self.step - 1 };
        let quantity = &self.quantity;
        if step < quantity.len() {
            quantity[step]
        } else {
            *quantity.last().unwrap()
        }
    }
}
