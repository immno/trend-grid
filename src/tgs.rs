use std::env;

use anyhow::Result;
use tokio::fs;

use trend_grid::{init_log, start_server_with_config, ServerConfig, TgError};

#[tokio::main]
async fn main() -> Result<()> {
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
