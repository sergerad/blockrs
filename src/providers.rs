use std::fmt::Debug;

use alloy::primitives::FixedBytes;

pub mod eth;

#[derive(Debug, Copy, Clone)]
pub struct Block {
    pub number: u64,
}

#[derive(Debug, Copy, Clone)]
pub struct Transaction {
    pub hash: FixedBytes<32>,
}

#[derive(Debug, Copy, Clone)]
pub struct Account {}

#[async_trait::async_trait]
pub trait ChainProvider {
    type Error: Debug;
    async fn head(&mut self) -> Result<Block, Self::Error>;
    async fn transactions(&self) -> Result<Vec<Transaction>, Self::Error>;
    async fn balances(&self) -> Result<Vec<Account>, Self::Error>;
}
