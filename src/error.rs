use thiserror::Error;

#[derive(Error, Debug)]
pub enum TgError {
    #[error("Not found: tgs.conf. Set env variable TGS_CONFIG")]
    ConfNotFound,

    #[error("I/O error")]
    IoError(#[from] std::io::Error),

    #[error("Parse config error")]
    ConfigError(#[from] toml::de::Error),
}
