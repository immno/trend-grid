use std::env;

use anyhow::Result;
use tokio::fs;

use trend_grid::{init_log, start_server_with_config, ServerConfig};

#[tokio::main]
async fn main() -> Result<()> {
    let config = match env::var("TGS_CONFIG") {
        Ok(path) => fs::read_to_string(&path).await?,
        Err(_) => include_str!("../fixtures/tgs.conf").to_string(),
    };
    let config = ServerConfig::from_str(&config)?;
    init_log(&config.log);

    start_server_with_config(&config).await?;

    Ok(())
}
