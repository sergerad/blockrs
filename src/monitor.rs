use std::time::Duration;

use crate::providers::ChainProvider;
use crate::types::{Account, Block, Transaction};
use tokio::{
    sync::mpsc::{unbounded_channel, UnboundedReceiver as Receiver, UnboundedSender as Sender},
    time::interval,
};

pub struct ChainMonitor<P> {
    block_tx: Sender<Block>,
    transaction_tx: Sender<Transaction>,
    account_tx: Sender<Account>,
    block_rx: Option<Receiver<Block>>,
    transaction_rx: Option<Receiver<Transaction>>,
    account_rx: Option<Receiver<Account>>,
    provider: P,
}

impl<P> ChainMonitor<P> {
    pub fn new(provider: P) -> Self {
        let (block_tx, block_rx) = unbounded_channel();
        let (transaction_tx, transaction_rx) = unbounded_channel();
        let (account_tx, account_rx) = unbounded_channel();
        Self {
            provider,
            block_tx,
            transaction_tx,
            account_tx,
            block_rx: block_rx.into(),
            transaction_rx: transaction_rx.into(),
            account_rx: account_rx.into(),
        }
    }

    /// ...
    ///
    /// # Panics
    ///
    /// ...
    pub fn receivers(&mut self) -> (Receiver<Block>, Receiver<Transaction>, Receiver<Account>) {
        (
            self.block_rx.take().unwrap(),
            self.transaction_rx.take().unwrap(),
            self.account_rx.take().unwrap(),
        )
    }
}

impl<P: ChainProvider + Sync> ChainMonitor<P> {
    pub async fn run(&mut self) {
        let mut tick_interval = interval(Duration::from_secs(2));
        let head_number = 0u64;
        loop {
            tick_interval.tick().await;
            let block = self.provider.head().await.unwrap();
            if block.number > head_number {
                self.block_tx.send(block).unwrap();
                let txs = self.provider.transactions().await.unwrap();
                txs.into_iter().for_each(|tx| {
                    self.transaction_tx.send(tx).unwrap();
                });
            }
        }
    }
}
