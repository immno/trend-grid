use std::ops::Deref;

use anyhow::Result;
use tracing::{info, warn};

pub use config::*;
pub use error::TgError;

use crate::grid::{FixedGridService, GridService};

mod config;
mod error;
mod grid;
mod trade;

/// 通过配置创建 KV 服务器
#[tracing::instrument(skip_all)]
pub async fn start_server_with_config(config: &ServerConfig) -> Result<()> {
    info!("Starting: Trend Grid Server");
    let (market, trade) = trade::factory(config.trade.as_ref())?;

    if let Some(eth) = config.coin.eth.as_ref() {
        let curr_price = market.deref().ticker_price().await?;
        let grid: FixedGridService = eth.into();
        let grid: &dyn GridService = &grid;
        loop {
            if grid.is_buy(&curr_price) {
                // if let Ok(_) = trade.buy(100).await {
                //     grid.poise();
                //     info!("挂单成功")
                // } else {
                //     return continue;
                // }
            } else if grid.is_sell(&curr_price) {
                if grid.is_air() {
                    grid.modify_price();
                } else {
                    // if let Ok(_) = trade.sell(80).await {
                    //     grid.poise();
                    // } else {
                    //     return continue;
                    // }
                }
            } else {
                warn!(
                    "币种:{},当前市价：{}。未能满足交易,继续运行",
                    "eth",
                    curr_price.price.as_str()
                )
            }
        }
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
