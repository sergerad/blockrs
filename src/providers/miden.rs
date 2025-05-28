use miden_client::rpc::{Endpoint, NodeRpcClient, RpcError, TonicRpcClient};
use miden_objects::block::ProvenBlock;
use url::Url;

use crate::types::{Account, Block, Transaction};

use super::ChainProvider;

#[derive(thiserror::Error, Debug)]
pub enum MidenProviderError {
    #[error("...")]
    Infallible(#[from] std::convert::Infallible),

    #[error("...")]
    Store(#[from] miden_client::store::StoreError),

    #[error("...")]
    Client(#[from] miden_client::ClientError),

    #[error("...")]
    Rpc(#[from] RpcError),
}

pub struct MidenProvider {
    client: TonicRpcClient,
    head: Option<ProvenBlock>,
}

impl MidenProvider {
    pub async fn new(url: Url, _addrs: &[String]) -> Result<Self, MidenProviderError> {
        let endpoint = Endpoint::new(
            url.scheme().into(),
            url.host().unwrap().to_string(),
            url.port(),
        );
        let client = TonicRpcClient::new(&endpoint, 10_000);
        Ok(Self { client, head: None })
    }
}

impl ChainProvider for MidenProvider {
    type Error = MidenProviderError;

    async fn head(&mut self) -> Result<Block, Self::Error> {
        // Retrieve latest block header info.
        let (block, _) = self.client.get_block_header_by_number(None, false).await?;
        let number = block.block_num().as_u64();
        let timestamp = block.timestamp() as u64;
        let hash = block.chain_commitment().to_string();

        // Retrieve the latest block and store it in the provider.
        let block = self
            .client
            .get_block_by_number((number as u32).into())
            .await?;
        self.head = block;

        // Return the block header info.
        Ok(Block {
            number,
            timestamp,
            hash,
        })
    }

    async fn transactions(&self) -> Result<Vec<Transaction>, Self::Error> {
        Ok(vec![])
    }

    async fn balances(&self) -> Result<Vec<Account>, Self::Error> {
        // TODO: impl
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::MidenProvider;
    use url::Url;

    #[tokio::test]
    async fn instantiate() {
        let u = Url::parse("http://localhost:57291").unwrap();
        let addrs = vec![];
        let _p = MidenProvider::new(u, &addrs).await.unwrap();
    }
}
