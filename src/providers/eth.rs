use crate::config::Config;
use crate::providers::{Account, Block, ChainProvider, Transaction};
use alloy::consensus::Transaction as AlloyTransaction;
use alloy::eips::BlockId;
use alloy::primitives::Address as AlloyAddress;
use alloy::providers::{DynProvider, Provider, ProviderBuilder};
use alloy::rpc::types::Block as AlloyBlock;
use alloy::transports::{RpcError, TransportErrorKind};
use config::ConfigError;
use std::str::FromStr;
use url::Url;

#[derive(Debug, Clone)]
pub struct EthProvider {
    provider: DynProvider,
    head: Option<AlloyBlock>,
    addrs: Vec<AlloyAddress>,
}

impl EthProvider {
    pub fn new(url: Url) -> Result<Self, ConfigError> {
        let config = Config::new()?;
        let addrs: Vec<_> = config
            .config
            .addresses
            .iter()
            .map(|addr| AlloyAddress::from_str(addr.as_str()).unwrap())
            .collect();
        let provider = ProviderBuilder::new().on_http(url);
        let provider = DynProvider::new(provider);
        Ok(Self {
            provider,
            addrs,
            head: None,
        })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum EthProviderError {
    #[error(transparent)]
    Transport(#[from] RpcError<TransportErrorKind>),

    #[error("Head block could not be found")]
    NoHead,
}

impl From<&AlloyBlock> for Block {
    fn from(block: &AlloyBlock) -> Self {
        Self {
            number: block.header.number,
        }
    }
}

#[async_trait::async_trait]
impl ChainProvider for EthProvider {
    type Error = EthProviderError;

    async fn head(&mut self) -> Result<Block, Self::Error> {
        let block = self.provider.get_block(BlockId::latest()).full().await?;
        let block = block.ok_or(EthProviderError::NoHead)?;
        let result_block = (&block).into();
        self.head = block.into();
        Ok(result_block)
    }

    async fn transactions(&self) -> Result<Vec<Transaction>, Self::Error> {
        if let Some(block) = &self.head {
            let txs: Vec<_> = block
                .transactions
                .as_transactions()
                .unwrap()
                .iter()
                .map(|tx| Transaction {
                    value: tx.inner.value().to_string(),
                    block_number: tx.block_number.unwrap(),
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

    async fn balances(&self) -> Result<Vec<Account>, Self::Error> {
        let block = self
            .head
            .as_ref()
            .map(|b| BlockId::from(b.header.number))
            .unwrap_or(BlockId::latest());
        let mut accounts = Vec::new();
        for addr in &self.addrs {
            let bal = self.provider.get_balance(*addr).block_id(block).await?;
            accounts.push(Account {
                balance: bal.to_string(),
                id: addr.to_string(),
            });
        }
        Ok(accounts)
    }
}
