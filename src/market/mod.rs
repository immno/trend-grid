use anyhow::Result;
use async_trait::async_trait;

mod binance_service;

/// Abstraction of Market Service
#[async_trait]
pub trait MarketService {
    /// Test connectivity to the Rest API.
    async fn ping(self) -> Result<bool>;

    async fn sign(self) -> bool;
}
