use std::borrow::Borrow;

use anyhow::Result;
use tracing::{error, info, warn};

pub use config::*;
pub use error::TgError;

mod config;
mod error;
mod grid;
mod serde;
mod trade;

/// 通过配置创建 KV 服务器
#[tracing::instrument(skip_all)]
pub async fn start_server_with_config(config: &ServerConfig) -> Result<()> {
    info!("Starting: Trend Grid Server");
    let (market, trade) = trade::factory(config.trade.as_ref())?;
    let ping = market.ping().await;
    if ping.is_err() {
        error!(
            "Unable to ping service: {}",
            config.trade.as_ref().url.as_str()
        );
    }

    if let Some(eth) = config.coin.eth.as_ref() {
        let symbol = Symbol::Eth.borrow();
        let curr_price = market.ticker_price(symbol).await?;
        let mut grid = grid::factory(symbol, eth)?;
        loop {
            let quantity = grid.buy_quantity();
            if grid.is_buy(curr_price) {
                if let Ok(_) = trade.buy(symbol, quantity).await {
                    grid.poise();
                    grid.record(curr_price);
                    info!("挂单成功")
                } else {
                    return continue;
                }
            } else if grid.is_sell(curr_price) {
                if grid.is_air() {
                    grid.modify_price(curr_price);
                } else {
                    if let Ok(_) = trade.sell(symbol, 80.0).await {
                        grid.poise();
                    } else {
                        return continue;
                    }
                }
            } else {
                warn!(
                    "币种:{:?},当前市价：{}。未能满足交易,继续运行",
                    symbol, curr_price
                );
            }
        }
    };

    Ok(())
}
