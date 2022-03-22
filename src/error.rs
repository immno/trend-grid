use thiserror::Error;
use tracing::debug;

#[derive(Error, Debug)]
pub enum TgError {
    #[error("Not found: tgs.conf. Set env variable TGS_CONFIG")]
    ConfNotFound,
    #[error("I/O error")]
    IoError(#[from] std::io::Error),
    #[error("Parse config error")]
    ConfigError(#[from] toml::de::Error),
    #[error("Can not connect to the server")]
    PingError(),
    #[error("Url Error: {0}")]
    UrlError(String),
    #[error("Request error {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Binance error code:{0}-{1}")]
    BinanceError(u16, String),
    #[error("Decode response body error {0}")]
    DecodeError(String),
}

impl TgError {
    pub async fn bina_resp(resp: reqwest::Response) -> Result<String, Self> {
        if !resp.status().is_success() {
            let status_code = u16::from(resp.status());
            let error = resp.text().await.unwrap_or_default();
            Err(TgError::BinanceError(status_code, error))
        } else {
            let resp_text = resp
                .text()
                .await
                .map_err(|e| Self::DecodeError(e.to_string()))?;
            debug!("resp {}", &resp_text);
            Ok(resp_text)
        }
    }
}
