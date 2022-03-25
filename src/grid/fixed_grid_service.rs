use std::borrow::Borrow;
use std::ops::Div;
use std::sync::Arc;

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
            if let Ok(_) = self.trade.buy(symbol, quantity).await {
                self.reset_ratio();
                self.poise();
                self.record(price);
                info!("挂单成功")
            }
        } else if self.is_sell(price) {
            if self.is_air() {
                self.modify_price(price);
            } else {
                if let Ok(_) = self.trade.sell(symbol, 80.0).await {
                    self.poise();
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
            let v = (line.high - line.open) / line.low;
            percent_total = v.abs() + percent_total;
        }
        self.inner.double_throw_ratio = percent_total.div(k_lines.len() as f64);
        Ok(())
    }

    fn poise(&self) -> bool {
        todo!()
    }

    fn record(&mut self, price: f64) {
        self.inner.history.push(price);
    }

    fn modify_price(&mut self, price: f64) {
        self.inner.buy = self.inner.sell * (1.0 - self.inner.double_throw_ratio / 100.0);
        self.inner.sell = self.inner.sell * (1.0 + self.inner.double_throw_ratio / 100.0);
        if self.is_buy(price) {
            self.inner.buy = price * (1.0 - self.inner.double_throw_ratio / 100.0);
        } else if self.is_sell(price) {
            self.inner.sell = price * (1.0 + self.inner.double_throw_ratio / 100.0);
        }
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
