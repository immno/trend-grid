use std::env;
use std::str::FromStr;

use anyhow::Result;
use tokio::fs;
use tracing::span;
use tracing_subscriber::{
    filter,
    fmt::{self, format},
    layer::SubscriberExt,
    prelude::*,
    EnvFilter,
};

use trend_grid::{start_server_with_config, LogConfig, LogRotationType, ServerConfig, TgError};

#[tokio::main]
async fn main() -> Result<()> {
    env::set_var(
        "TGS_CONFIG",
        "D:\\coding\\source\\trend-grid\\fixtures\\tgs.conf",
        // "/home/mno/IdeaProjects/trend-grid/fixtures/tgs.conf",
    );
    let config = match env::var("TGS_CONFIG") {
        Ok(path) => fs::read_to_string(&path)
            .await
            .map_err(|e| TgError::IoError(e)),
        Err(_) => Err(TgError::ConfNotFound),
    }?;
    let config = ServerConfig::from_str(&config)?;
    init_log(&config.log);

    start_server_with_config(&config).await?;

    Ok(())
}

// 初始化日志配置
fn init_log(log: &LogConfig) {
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
