use std::borrow::Borrow;
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

    if let Some(eth) = config.coin.eth.as_ref() {
        start_with_coin(Symbol::Eth, eth, market.clone(), trade.clone())?;
    };

    if let Some(bnb) = config.coin.bnb.as_ref() {
        start_with_coin(Symbol::Bnb, bnb, market.clone(), trade.clone())?;
    };

    if let Some(btc) = config.coin.btc.as_ref() {
        start_with_coin(Symbol::Btc, btc, market.clone(), trade.clone())?;
    };

    warn!("No option coin is running.");

    Ok(())
}

fn start_with_coin(
    symbol: Symbol,
    coin: &Coin,
    market: Arc<dyn MarketService>,
    trade: Arc<dyn TradeService>,
) -> Result<()> {
    let mut grid = grid::factory(&symbol, coin, market.clone(), trade.clone())?;

    let root = span!(tracing::Level::INFO, "Grid", "{}", &symbol);
    let _enter = root.enter();

    tokio::spawn(async move {
        loop {
            match market.ticker_price(&symbol).await {
                Ok(price) => {
                    if let Err(e) = grid.execute(&symbol, price).await {
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
    Ok(())
}
