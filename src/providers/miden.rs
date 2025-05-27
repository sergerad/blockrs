use std::sync::Arc;

use miden_client::{
    builder::ClientBuilder,
    note::BlockNumber,
    rpc::{Endpoint, NodeRpcClient, TonicRpcClient},
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
    client: TonicRpcClient,
}

impl MidenProvider {
    pub async fn new(url: Url, addrs: &[String]) -> Result<Self, MidenProviderError> {
        let endpoint = Endpoint::new(
            url.scheme().into(),
            url.host().unwrap().to_string(),
            url.port(),
        );
        let client = TonicRpcClient::new(&endpoint, 10_000);
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
        let block = self
            .client
            .get_block_by_number(0u32.into())
            .await
            .expect("todo");
        let number = 0; // TODO
        Ok(Block {
            number,
            timestamp: 0u64,      // TODO
            hash: "".to_string(), // TODO
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
