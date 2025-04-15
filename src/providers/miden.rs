use super::ChainProvider;
use crate::types::{Account, Block, Transaction};
use futures::executor::block_on;
use miden_client::{
    builder::ClientBuilder,
    rpc::{Endpoint, TonicRpcClient},
    Client,
};
use ratatui::text::ToSpan;
use std::sync::{
    mpsc::{self, SendError},
    Arc,
};
use tokio::sync::{
    mpsc::{unbounded_channel, UnboundedSender},
    oneshot,
};
use tracing::{error, info};
use url::Url;

#[derive(thiserror::Error, Debug)]
pub enum MidenProviderError {
    #[error("...")]
    Infallible(#[from] std::convert::Infallible),

    #[error("...")]
    Store(#[from] miden_client::store::StoreError),

    #[error("...")]
    Client(#[from] miden_client::ClientError),

    #[error("{0}")]
    Send(#[from] SendError<Request>),
}

enum Request {
    Head(oneshot::Sender<Block>),
    Transactions(oneshot::Sender<Vec<Transaction>>),
    Balances(oneshot::Sender<Vec<Account>>),
}

pub struct MidenProvider {
    tx: UnboundedSender<Request>,
}

impl MidenProvider {
    pub fn new(url: Url, _addrs: &[String]) -> Result<Self, MidenProviderError> {
        let (tx, mut rx) = unbounded_channel();
        std::thread::spawn(|| {
            tokio::runtime::Builder::new_current_thread()
                .build()
                .unwrap()
                .block_on(async move {
                    let endpoint = Endpoint::new(
                        url.scheme().into(),
                        url.host().unwrap().to_string(),
                        url.port(),
                    );
                    let tonic_client = TonicRpcClient::new(&endpoint, 10_000);
                    let tonic_client = Arc::new(tonic_client);
                    let builder = ClientBuilder::new()
                        .with_rpc(tonic_client)
                        .with_filesystem_keystore(".store.sqlite3");
                    let mut client = builder.build().await.unwrap();
                    loop {
                        if let Some(request) = rx.recv().await {
                            match request {
                                Request::Head(sender) => {
                                    if let Ok(block) = client.get_sync_height().await {
                                        //let num = block.block_num();
                                        let num = block;
                                        let block = Block {
                                            number: num.as_u64(),
                                            //timestamp: block.timestamp() as u64,
                                            timestamp: 0u64,
                                            //hash: block.chain_commitment().to_string(), // TODO: Proper hash?
                                            hash: "".to_string(),
                                        };
                                        sender.send(block).unwrap();
                                    }
                                }
                                Request::Transactions(sender) => {
                                    // ...
                                }
                                Request::Balances(sender) => {
                                    // ...
                                }
                            }
                        }
                    }
                });
        });
        Ok(Self { tx })
    }
}

impl ChainProvider for MidenProvider {
    type Error = MidenProviderError;

    async fn head(&mut self) -> Result<Block, Self::Error> {
        error!("head");
        let (tx, rx) = oneshot::channel();
        self.tx.send(Request::Head(tx)).unwrap();
        let block = rx.await.unwrap();
        error!("end {}", block.number);
        Ok(block)
    }

    async fn transactions(&self) -> Result<Vec<Transaction>, Self::Error> {
        Ok(vec![])
        //let (tx, rx) = oneshot::channel();
        //self.tx.send(Request::Transactions(tx)).unwrap();
        //let txs = rx.await.unwrap();
        //Ok(txs)
    }

    async fn balances(&self) -> Result<Vec<Account>, Self::Error> {
        Ok(vec![])
        //let (tx, rx) = oneshot::channel();
        //self.tx.send(Request::Balances(tx)).unwrap();
        //let bals = rx.await.unwrap();
        //Ok(bals)
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
        let _p = MidenProvider::new(u, &addrs).unwrap();
    }
}
