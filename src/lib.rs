use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use tokio::task::JoinHandle;
use tracing::{error, info, span, warn};

pub use config::*;
pub use error::TgError;

use crate::trade::{MarketService, TradeService};

mod config;
mod error;
mod grid;
mod serde;
mod trade;

/// 通过配置创建 KV 服务器
pub async fn start_server_with_config(config: &ServerConfig) -> Result<()> {
    info!("Starting: Trend Grid Server");
    let (market, trade) = trade::factory(config.trade.as_ref())?;

    // If the server fails, shut down the server directly
    if let Err(e) = market.ping().await {
        let url = config.trade.as_ref().url.as_str();
        error!("Unable to ping service: {}", url);
        return Err(e);
    }

    let mut handles = Vec::new();

    if let Some(eth) = config.coin.eth.as_ref() {
        let join_eth = start_with_coin(Symbol::Eth, eth, market.clone(), trade.clone())?;
        handles.push(join_eth);
    };

    if let Some(bnb) = config.coin.bnb.as_ref() {
        let join_eth = start_with_coin(Symbol::Bnb, bnb, market.clone(), trade.clone())?;
        handles.push(join_eth);
    };

    if let Some(btc) = config.coin.btc.as_ref() {
        let join_eth = start_with_coin(Symbol::Btc, btc, market.clone(), trade.clone())?;
        handles.push(join_eth);
    };

    if handles.is_empty() {
        warn!("No option coin is running.");
    }

    for handle in handles {
        if let (Err(e),) = tokio::join!(handle) {
            error!("Processor error: {}", e);
        };
    }

    Ok(())
}

fn start_with_coin(
    symbol: Symbol,
    coin: &Coin,
    market: Arc<dyn MarketService>,
    trade: Arc<dyn TradeService>,
) -> Result<JoinHandle<()>> {
    let mut grid = grid::factory(&symbol, coin, market.clone(), trade.clone())?;

    let root = span!(tracing::Level::INFO, "Grid", "{}", &symbol);
    let _enter = root.enter();

    let join = tokio::spawn(async move {
        loop {
            match market.ticker_price(&symbol).await {
                Ok(price) => {
                    if let Err(e) = grid.execute(price).await {
                        error!("Grid execute error: {}", e);
                    }
                }
                Err(e) => {
                    error!("Get ticker price error: {}.", e);
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }
    });
    Ok(join)
}
