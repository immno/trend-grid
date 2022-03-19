use std::env;

use anyhow::Result;
use tokio::fs;
use trend_grid::ServerConfig;

#[tokio::main]
async fn main() -> Result<()> {
    let config = match env::var("TGS_CONFIG") {
        Ok(path) => fs::read_to_string(&path).await?,
        Err(_) => include_str!("../fixtures/tgs.conf").to_string(),
    };
    let config: ServerConfig = toml::from_str(&config)?;
    let log = &config.log;

    env::set_var("RUST_LOG", &log.log_level);

    // let stdout_log = fmt::layer().compact();

    // let file_appender = match log.rotation {
    //     RotationConfig::Hourly => tracing_appender::rolling::hourly(&log.path, "tgs.log"),
    //     RotationConfig::Daily => tracing_appender::rolling::daily(&log.path, "tgs.log"),
    //     RotationConfig::Never => tracing_appender::rolling::never(&log.path, "tgs.log"),
    // };
    Ok(())
}
