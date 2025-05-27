pub mod eth;
pub mod miden;

use std::future::Future;

use crate::types::{Account, Block, Transaction};

/// Represents a service that provides chain data in the form of blocks, transactions, and accounts.
///
/// The data provided is designed to be chain agnostic. It is used to render
/// information to the UI of the app.
pub trait ChainProvider {
    type Error: std::error::Error + Send + Sync + 'static;

    /// Retrieve the latest head of the blockchain.
    ///
    /// Should be used to cache any data required to serve data derived from
    /// specific blocks such as transaction data.
    fn head(&mut self) -> impl Future<Output = Result<Block, Self::Error>> + Send;

    /// Retrieve the transactions pertaining to the last block retrieved
    /// from the chain.
    fn transactions(&self) -> impl Future<Output = Result<Vec<Transaction>, Self::Error>> + Send;

    /// Retrieve the account balances pertaining to the last block retrieved
    /// from the chain.
    fn balances(&self) -> impl Future<Output = Result<Vec<Account>, Self::Error>> + Send;
}
