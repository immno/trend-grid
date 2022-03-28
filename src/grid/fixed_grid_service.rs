use std::borrow::Borrow;
use std::ops::{Add, Div, Mul, Sub};
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use tracing::{info, warn};

use crate::grid::GridService;
use crate::trade::{MarketService, TradeService};
use crate::{Coin, Symbol, TgError};

const HUNDRED_PERCENT: f64 = 1.0;

pub struct FixedGridService {
    symbol: Symbol,
    market: Arc<dyn MarketService>,
    trade: Arc<dyn TradeService>,
    db: Db,
}

pub struct Db {
    buy: f64,
    sell: f64,
    quantity: f64,
    profit_ratio: f64,
    double_throw_ratio: f64,
    history: Vec<f64>,
}

#[async_trait]
impl GridService for FixedGridService {
    async fn execute(&mut self, price: f64) -> anyhow::Result<()> {
        let symbol = self.symbol.borrow();
        let quantity = self.db.quantity;
        if self.is_buy(price) {
            if let Ok(Some(success_price)) = self.trade.buy(symbol, quantity).await {
                info!(
                    "交易成功: 买入币种为: {},价格为: {}, 数量: {},",
                    symbol, success_price, quantity
                );
                self.reset_ratio().await?;
                self.db.push_record(success_price);
                self.modify_price_buy(price);
                tokio::time::sleep(Duration::from_secs(120)).await;
            }
        } else if self.is_sell(price) {
            if self.is_air() {
                self.modify_price_air(price);
            } else {
                if let Ok(Some(success_price)) = self.trade.sell(symbol, quantity).await {
                    println!("##Sell: {} {}", success_price, price);
                    let last_price = self.db.last_record().cloned().ok_or(TgError::Internal(
                        "Sell success. But price history is empty".to_string(),
                    ))?;
                    let profit = price.sub(last_price).mul(quantity);
                    warn!(
                        "交易成功：卖出币种为：{}。卖单量为：{}。预计盈利{}U",
                        symbol, quantity, profit
                    );
                    self.reset_ratio().await?;
                    self.modify_price(last_price, price);
                    self.db.pop_record();
                    tokio::time::sleep(Duration::from_secs(60)).await;
                }
            }
        } else {
            warn!("币种:{},当前市价：{}。未能满足交易,继续运行", symbol, price);
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
        let db = Db {
            buy: c.buy_price,
            sell: c.sell_price,
            history: Vec::new(),
            profit_ratio: c.profit_ratio,
            double_throw_ratio: c.double_throw_ratio,
            quantity: c.quantity,
        };
        Ok(Self {
            symbol,
            market,
            trade,
            db,
        })
    }

    fn is_buy(&self, price: f64) -> bool {
        self.db.buy >= price
    }

    fn is_sell(&self, price: f64) -> bool {
        self.db.sell < price
    }

    fn is_air(&self) -> bool {
        self.db.history.is_empty()
    }

    async fn reset_ratio(&mut self) -> anyhow::Result<()> {
        let value = self.calc_k_lines().await?;
        self.db.double_throw_ratio = value;
        self.db.profit_ratio = value;
        Ok(())
    }

    fn modify_price_buy(&mut self, market_price: f64) {
        self.modify_price(market_price, market_price);
    }

    fn modify_price_air(&mut self, market_price: f64) {
        self.modify_price(self.db.sell, market_price);
    }

    fn modify_price(&mut self, deal_price: f64, market_price: f64) {
        self.db.buy = deal_price.mul(HUNDRED_PERCENT.sub(self.db.double_throw_ratio));
        self.db.sell = deal_price.mul(HUNDRED_PERCENT.add(self.db.profit_ratio));
        if self.is_buy(market_price) {
            self.db.buy = market_price.mul(HUNDRED_PERCENT.sub(self.db.double_throw_ratio));
        } else if self.is_sell(market_price) {
            self.db.sell = market_price.mul(HUNDRED_PERCENT.add(self.db.profit_ratio));
        }
        info!(
            "修改后的补仓价格为:{},修改后的网格价格为:{}.",
            self.db.buy, self.db.sell
        );
    }

    async fn calc_k_lines(&self) -> anyhow::Result<f64> {
        let k_lines = self.market.k_lines(self.symbol.borrow()).await?;
        let mut percent_total = 0.0;
        for line in k_lines.iter() {
            let v = (line.high - line.open).abs() / line.low;
            percent_total = v + percent_total;
        }
        let value = percent_total.div(k_lines.len() as f64);
        Ok(value)
    }
}

impl Db {
    fn last_record(&self) -> Option<&f64> {
        self.history.last()
    }

    fn push_record(&mut self, price: f64) {
        self.history.push(price);
    }

    fn pop_record(&mut self) {
        self.history.pop();
    }
}

#[cfg(test)]
mod tests {}
