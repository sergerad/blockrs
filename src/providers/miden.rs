use std::sync::Arc;

use miden_client::{
    builder::ClientBuilder,
    rpc::{Endpoint, TonicRpcClient},
    Client,
};
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
}

pub struct MidenProvider {
    client: Client,
}

impl MidenProvider {
    pub async fn new(url: Url, addrs: &[String]) -> Result<Self, MidenProviderError> {
        let endpoint = Endpoint::new(
            url.scheme().into(),
            url.host().unwrap().to_string(),
            url.port(),
        );
        let tonic_client = TonicRpcClient::new(&endpoint, 10_000);
        let tonic_client = Arc::new(tonic_client);
        let builder = ClientBuilder::new()
            .with_rpc(tonic_client)
            .with_filesystem_keystore("/tmp/");
        let client = builder.build().await?;
        Ok(Self { client })
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

#[async_trait::async_trait]
impl ChainProvider for MidenProvider {
    type Error = MidenProviderError;

    async fn head(&mut self) -> Result<Block, Self::Error> {
        let block = self.client.get_latest_epoch_block().await?;
        let num = block.block_num();
        Ok(Block {
            number: num.as_u64(),
            timestamp: block.timestamp() as u64,
            hash: block.chain_commitment().to_string(), // TODO: Proper hash?
        })
    }

    async fn transactions(&self) -> Result<Vec<Transaction>, Self::Error> {
        // TODO: impl
        Ok(vec![])
    }

    async fn balances(&self) -> Result<Vec<Account>, Self::Error> {
        // TODO: impl
        Ok(vec![])
    }
}
