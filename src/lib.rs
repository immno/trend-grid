use std::env;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use tokio::task::JoinHandle;
use tracing::{error, info, span, warn};
use tracing_subscriber::{
    filter,
    fmt::{self, format},
    layer::SubscriberExt,
    prelude::*,
    EnvFilter,
};

pub use config::*;
pub use error::TgError;

use crate::trade::{MarketService, TradeService};

mod config;
mod error;
mod grid;
mod serde;
mod trade;

/// 通过配置创建 TG 服务器
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

// 初始化日志配置
pub fn init_log(log: &LogConfig) {
    env::set_var("RUST_LOG", &log.log_level);

    let stdout_log = fmt::layer().compact();

    let file_appender = match log.rotation {
        LogRotationType::Hourly => tracing_appender::rolling::hourly(&log.path, "server.log"),
        LogRotationType::Daily => tracing_appender::rolling::daily(&log.path, "server.log"),
        LogRotationType::Never => tracing_appender::rolling::never(&log.path, "server.log"),
    };

    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let fmt_layer = fmt::layer()
        .event_format(format().compact())
        .with_writer(non_blocking);

    let level = match filter::LevelFilter::from_str(&log.log_level) {
        Ok(level) => level,
        Err(_) => filter::LevelFilter::INFO,
    };

    let log_file_level = match log.enable_log_file {
        true => level,
        false => filter::LevelFilter::OFF,
    };

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(stdout_log)
        .with(fmt_layer.with_filter(log_file_level))
        .init();
}
