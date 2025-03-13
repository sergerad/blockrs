pub mod eth;

use crate::types::{Account, Block, Transaction};

#[async_trait::async_trait]
pub trait ChainProvider {
    type Error: std::fmt::Debug;
    async fn head(&mut self) -> Result<Block, Self::Error>;
    async fn transactions(&self) -> Result<Vec<Transaction>, Self::Error>;
    async fn balances(&self) -> Result<Vec<Account>, Self::Error>;
}
