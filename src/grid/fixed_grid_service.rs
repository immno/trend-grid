use std::borrow::Borrow;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub};
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use tracing::{info, warn};

use crate::grid::GridService;
use crate::trade::{MarketService, TradeService};
use crate::{Coin, Symbol, TgError};

pub struct FixedGridService {
    symbol: Symbol,
    pub market: Arc<dyn MarketService>,
    pub trade: Arc<dyn TradeService>,
    pub inner: Inner,
}

pub struct Inner {
    pub buy: f64,
    pub sell: f64,
    pub quantity: Vec<f64>,
    pub profit_ratio: f64,
    pub double_throw_ratio: f64,
    pub step: usize,
    pub history: Vec<f64>,
}

#[async_trait]
impl GridService for FixedGridService {
    async fn execute(&mut self, price: f64) -> anyhow::Result<()> {
        let symbol = self.symbol.borrow();
        let quantity = self.buy_quantity();
        if self.is_buy(price) {
            if let Ok(Some(success_price)) = self.trade.buy(symbol, quantity).await {
                self.reset_ratio().await?; // ok
                self.record(success_price); // ok
                self.modify_price(0, price, price);
                info!("挂单成功");
                tokio::time::sleep(Duration::from_secs(120)).await;
            }
        } else if self.is_sell(price) {
            if self.is_air() {
                // self.modify_price(price);
            } else {
                let sell_amount = self.get_quantity(false);
                if let Ok(Some(success_price)) = self.trade.sell(symbol, 80.0).await {
                    self.reset_ratio().await?; // ok

                    // self.poise();
                }
            }
        } else {
            warn!(
                "币种:{:?},当前市价：{}。未能满足交易,继续运行",
                "symbol", price
            );
        }

        Ok(())
    }
}

impl FixedGridService {
    pub fn new(
        symbol: Symbol,
        c: &Coin,
        market: Arc<dyn MarketService>,
        trade: Arc<dyn TradeService>,
    ) -> Result<Self, TgError> {
        if c.quantity.is_empty() {
            return Err(TgError::ConfError("quantity cannot be empty".to_string()));
        }
        let inner = Inner {
            buy: c.next_buy_price,
            sell: c.grid_sell_price,
            history: Vec::new(),
            profit_ratio: c.profit_ratio,
            double_throw_ratio: c.double_throw_ratio,
            quantity: c.quantity.to_vec(),
            step: c.step,
        };
        Ok(Self {
            symbol,
            market,
            trade,
            inner,
        })
    }

    fn is_buy(&self, price: f64) -> bool {
        self.inner.buy >= price
    }

    fn buy_quantity(&self) -> f64 {
        self.get_quantity(true)
    }

    fn is_sell(&self, price: f64) -> bool {
        self.inner.sell < price
    }

    fn is_air(&self) -> bool {
        self.inner.step == 0
    }

    fn sell_quantity(&self) -> f64 {
        self.get_quantity(false)
    }

    async fn reset_ratio(&mut self) -> anyhow::Result<()> {
        let k_lines = self.market.k_lines(self.symbol.borrow()).await?;
        let mut percent_total = 0.0;
        for line in k_lines.iter() {
            let v = (line.high - line.open).abs() / line.low;
            percent_total = v + percent_total;
        }
        let value = percent_total.div(k_lines.len() as f64).mul(100.0);
        self.inner.double_throw_ratio = value;
        self.inner.profit_ratio = value;
        Ok(())
    }

    fn record(&mut self, price: f64) {
        self.inner.history.push(price);
    }

    fn modify_price(&mut self, step_add: usize, deal_price: f64, market_price: f64) {
        self.inner.buy = deal_price.mul(1.0.sub(self.inner.double_throw_ratio.div(100.0)));
        self.inner.sell = deal_price.mul(1.0.add(self.inner.profit_ratio.div(100.0)));
        if self.is_buy(market_price) {
            self.inner.buy = market_price.mul(1.0.sub(self.inner.double_throw_ratio.div(100.0)));
        } else if self.is_sell(market_price) {
            self.inner.sell = market_price.mul(1.0.add(self.inner.profit_ratio.div(100.0)));
        }
        self.inner.step.add_assign(step_add);
        info!(
            "修改后的补仓价格为:{},修改后的网格价格为:{}.",
            self.inner.buy, self.inner.sell
        );
    }

    /// Calculate the number of buy/sell
    fn get_quantity(&self, buy: bool) -> f64 {
        let step = if buy {
            self.inner.step
        } else {
            self.inner.step - 1
        };
        let quantity = &self.inner.quantity;
        if step < quantity.len() {
            quantity[step]
        } else {
            *quantity.last().unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_usize() {
        let mut x: usize = 12;
        // x.add_assign(-1);
        // assert_eq!(x, 11);
    }
}
