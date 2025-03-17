use std::time::Duration;

use crate::providers::ChainProvider;
use crate::types::{
    AccountReceiver, AccountSender, BlockReceiver, BlockSender, TransactionReceiver,
    TransactionSender,
};
use tokio::{sync::mpsc::unbounded_channel, time::interval};

pub struct ChainMonitor<P> {
    block_tx: BlockSender,
    transaction_tx: TransactionSender,
    account_tx: AccountSender,
    block_rx: Option<BlockReceiver>,
    transaction_rx: Option<TransactionReceiver>,
    account_rx: Option<AccountReceiver>,
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
    pub fn receivers(&mut self) -> (BlockReceiver, TransactionReceiver, AccountReceiver) {
        (
            self.block_rx.take().unwrap(),
            self.transaction_rx.take().unwrap(),
            self.account_rx.take().unwrap(),
        )
    }
}

impl<P: ChainProvider + Sync> ChainMonitor<P> {
    pub async fn run(&mut self, tick_rate: Duration) {
        let mut tick_interval = interval(tick_rate);
        let mut head_number = 0u64;
        loop {
            tick_interval.tick().await;
            let block = self.provider.head().await.unwrap();
            if block.number > head_number {
                head_number = block.number;
                self.block_tx.send(vec![block]).unwrap();
                let txs = self.provider.transactions().await.unwrap();
                self.transaction_tx.send(txs).unwrap();
                let bals = self.provider.balances().await.unwrap();
                self.account_tx.send(bals).unwrap();
            }
        }
    }
}
