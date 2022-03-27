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
        // let symbol = self.symbol.borrow();
        // let quantity = self.db.get_quantity();
        // if self.is_buy(price) {
        //     if let Ok(Some(success_price)) = self.trade.buy(symbol, quantity).await {
        //         self.reset_ratio().await?;
        //         self.add_record(success_price);
        //         self.modify_price(0, price, price);
        //         info!("挂单成功");
        //         tokio::time::sleep(Duration::from_secs(120)).await;
        //     }
        // } else if self.is_sell(price) {
        //     if self.is_air() {
        //         self.modify_price(0, self.db.sell, price);
        //     } else {
        //         let last_price = self.db.last_record();
        //         let sell_amount = self.db.get_quantity();
        //         let porfit_usdt = price.sub(last_price).mul(sell_amount);
        //
        //         if let Ok(Some(success_price)) = self.trade.sell(symbol, sell_amount).await {
        //             warn!(
        //                 "报警：币种为：{}。卖单量为：{}。预计盈利{}U",
        //                 symbol, sell_amount, porfit_usdt
        //             );
        //             self.reset_ratio().await?;
        //             self.modify_price(-1, last_price, price);
        //             self.remove_record();
        //         }
        //     }
        // } else {
        //     warn!(
        //         "币种:{:?},当前市价：{}。未能满足交易,继续运行",
        //         "symbol", price
        //     );
        // }

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
        let k_lines = self.market.k_lines(self.symbol.borrow()).await?;
        let mut percent_total = 0.0;
        for line in k_lines.iter() {
            let v = (line.high - line.open).abs() / line.low;
            percent_total = v + percent_total;
        }
        let value = percent_total.div(k_lines.len() as f64).mul(100.0);
        self.db.double_throw_ratio = value;
        self.db.profit_ratio = value;
        Ok(())
    }

    fn modify_price(&mut self, step_add: i8, deal_price: f64, market_price: f64) {
        self.db.buy = deal_price.mul(1.0.sub(self.db.double_throw_ratio.div(100.0)));
        self.db.sell = deal_price.mul(1.0.add(self.db.profit_ratio.div(100.0)));
        if self.is_buy(market_price) {
            self.db.buy = market_price.mul(1.0.sub(self.db.double_throw_ratio.div(100.0)));
        } else if self.is_sell(market_price) {
            self.db.sell = market_price.mul(1.0.add(self.db.profit_ratio.div(100.0)));
        }
        info!(
            "修改后的补仓价格为:{},修改后的网格价格为:{}.",
            self.db.buy, self.db.sell
        );
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
mod tests {
    use super::*;

    #[test]
    fn add_usize() {
        let mut x: usize = 12;
        // x.add_assign(-1);
        // assert_eq!(x, 11);
    }

    #[test]
    fn calc() {
        // let price = 5;
        // let price1 = 2;
        // let price2 = 3;
        // let x = price.sub(price1).mul(price2);
        // assert_eq!(x, 9);
    }
}
