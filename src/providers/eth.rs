use crate::providers::{Account, Block, ChainProvider, Transaction};
use alloy::eips::BlockId;
use alloy::providers::{DynProvider, Provider, ProviderBuilder};
use alloy::rpc::types::{Block as AlloyBlock, Transaction as AlloyTransaction};
use alloy::transports::{RpcError, TransportErrorKind};
use url::Url;

#[derive(Debug, Clone)]
pub struct EthProvider {
    provider: DynProvider,
    head: Option<AlloyBlock>,
}

impl EthProvider {
    pub fn new(url: Url) -> Self {
        let provider = ProviderBuilder::new().on_http(url);
        let provider = DynProvider::new(provider);
        Self {
            provider,
            head: None,
        }
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
                .map(|tx| Transaction {})
                .into_iter()
                .collect();
            Ok(txs)
        } else {
            Err(EthProviderError::NoHead)
        }
    }

    async fn balances(&self) -> Result<Vec<Account>, Self::Error> {
        todo!()
    }
}
