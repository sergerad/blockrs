use crate::providers::{Account, Block, ChainProvider, Transaction};
use alloy::consensus::Transaction as AlloyTransaction;
use alloy::eips::BlockId;
use alloy::hex::FromHexError;
use alloy::primitives::utils::format_units;
use alloy::primitives::{Address as AlloyAddress, Uint};
use alloy::providers::{DynProvider, Provider, ProviderBuilder};
use alloy::rpc::types::Block as AlloyBlock;
use alloy::transports::{RpcError, TransportErrorKind};
use config::ConfigError;
use std::str::FromStr;
use url::Url;

#[derive(thiserror::Error, Debug)]
pub enum EthProviderError {
    #[error("invalid content in configuration file")]
    Config(#[from] ConfigError),

    #[error("invalid eth address specified")]
    InvalidAddress(#[from] FromHexError),

    #[error(transparent)]
    Transport(#[from] RpcError<TransportErrorKind>),

    #[error("head block could not be found")]
    NoHead,
}

#[derive(Debug, Clone)]
pub struct EthProvider {
    provider: DynProvider,
    head: Option<AlloyBlock>,
    addrs: Vec<AlloyAddress>,
}

impl EthProvider {
    pub fn new(url: Url, addrs: &[String]) -> Result<Self, EthProviderError> {
        let addrs = addrs
            .iter()
            .map(|a| AlloyAddress::from_str(a.as_str()))
            .collect::<Result<Vec<_>, _>>()?;
        let provider = ProviderBuilder::new().on_http(url);
        let provider = DynProvider::new(provider);
        Ok(Self {
            provider,
            addrs,
            head: None,
        })
    }
}

impl From<&AlloyBlock> for Block {
    fn from(block: &AlloyBlock) -> Self {
        Self {
            number: block.header.number,
            timestamp: block.header.timestamp,
            hash: block.header.hash.to_string(),
        }
    }
}

#[async_trait::async_trait]
impl ChainProvider for EthProvider {
    type Error = EthProviderError;

    /// Queries for the head of the chain and returns data pertaining to it
    /// that is relevant for the UI.
    async fn head(&mut self) -> Result<Block, Self::Error> {
        // Get the latest block.
        let block = self.provider.get_block(BlockId::latest()).full().await?;
        let block = block.ok_or(EthProviderError::NoHead)?;

        // Convert to block types required by the UI.
        let result_block = (&block).into();

        // Keep track of latest block.
        self.head = block.into();

        // Return the block.
        Ok(result_block)
    }

    /// Returns transactions pertaining to the latest block retrieved from the chain.
    async fn transactions(&self) -> Result<Vec<Transaction>, Self::Error> {
        if let Some(block) = &self.head {
            // Map the transactions in latest block to transaction types
            // required by the UI.
            let txs: Vec<_> = block
                .transactions
                .as_transactions()
                .into_iter()
                .flatten()
                .map(|tx| Transaction {
                    nonce: tx.inner.nonce().to_string(),
                    kind: tx.inner.tx_type().to_string(),
                    units: "gwei".to_string(),
                    value: gwei(tx.inner.value()),
                    hash: tx.inner.tx_hash().to_string(),
                    from: tx.inner.signer().to_string(),
                    to: tx
                        .inner
                        .to()
                        .map(|addr| addr.to_string())
                        .unwrap_or_default(),
                })
                .collect();
            Ok(txs)
        } else {
            Err(EthProviderError::NoHead)
        }
    }

    /// Retrieves balances pertaining to the latest block or the last
    /// block previously retrieved, if there is one.
    async fn balances(&self) -> Result<Vec<Account>, Self::Error> {
        // Use last retrieved block if there is one. Latest otherwise.
        let block = self
            .head
            .as_ref()
            .map(|b| BlockId::from(b.header.number))
            .unwrap_or(BlockId::latest());

        // Map the accounts to UI data elements.
        let mut accounts = Vec::new();
        for addr in &self.addrs {
            let bal = self.provider.get_balance(*addr).block_id(block).await?;
            accounts.push(Account {
                balance: gwei(bal),
                address: addr.to_string(),
                units: "gwei".to_string(),
            });
        }
        Ok(accounts)
    }
}

/// Formats a U256 into gwei string.
fn gwei(num: Uint<256, 4>) -> String {
    format_units(num, "gwei").expect("gwei is valid unit format")
}

#[cfg(test)]
mod tests {
    use super::{EthProvider, EthProviderError};
    use url::Url;

    #[test]
    fn instantiate() {
        let u = Url::parse("http://localhost:8545").unwrap();
        let addrs = vec![];
        let _p = EthProvider::new(u, &addrs).unwrap();
    }

    #[test]
    fn instantiate_addrs() {
        let u = Url::parse("http://localhost:8545").unwrap();
        let addrs = vec![
            "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045".to_string(),
            "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045".to_string(),
            "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045".to_string(),
        ];
        let _p = EthProvider::new(u, &addrs).unwrap();
    }

    #[test]
    fn instantiate_invalid_addrs() {
        let u = Url::parse("http://localhost:8545").unwrap();
        let addrs = vec!["0x999d8dA6BF26964aF9D7eEd9e03E53415D37aA96045".to_string()];
        let e = EthProvider::new(u, &addrs);
        assert!(matches!(e, Err(EthProviderError::InvalidAddress(_))));
    }
}
