use anyhow::Result;

/// Abstraction of transaction services
pub trait TradeService {
    /// Send in a new order.
    fn order(self) -> Result<bool>;
}
