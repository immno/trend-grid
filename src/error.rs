use thiserror::Error;

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
}
