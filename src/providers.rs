pub mod eth;

use crate::types::{Account, Block, Transaction};

/// Represents a service that provides chain data in the form of blocks, transactions, and accounts.
///
/// The data provided is designed to be chain agnostic. It is used to render
/// information to the UI of the app.
#[async_trait::async_trait]
pub trait ChainProvider {
    type Error: std::fmt::Debug;

    /// Retrieve the latest head of the blockchain.
    ///
    /// Should be used to cache any data required to serve data derived from
    /// specific blocks such as transaction data.
    async fn head(&mut self) -> Result<Block, Self::Error>;

    /// Retrieve the transactions pertaining to the last block retrieved
    /// from the chain.
    async fn transactions(&self) -> Result<Vec<Transaction>, Self::Error>;

    /// Retrieve the account balances pertaining to the last block retrieved
    /// from the chain.
    async fn balances(&self) -> Result<Vec<Account>, Self::Error>;
}
