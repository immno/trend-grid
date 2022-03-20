use anyhow::Result;
use tracing::info;

pub use config::*;
pub use error::TgError;

mod config;
mod error;
mod grid;
mod market;
mod trade;

/// 通过配置创建 KV 服务器
#[tracing::instrument(skip_all)]
pub async fn start_server_with_config(config: &ServerConfig) -> Result<()> {
    info!("Starting Trend-Grid-Server");
    info!("config:{:?}", config);

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
