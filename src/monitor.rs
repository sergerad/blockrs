use crate::providers::ChainProvider;
use crate::types::{
    AccountReceiver, AccountSender, BlockReceiver, BlockSender, TransactionReceiver,
    TransactionSender,
};
use tokio::sync::mpsc::unbounded_channel;

/// Runtime responsible for managing retrieval of latest chain data.
pub struct ChainMonitor<P> {
    block_tx: BlockSender,
    transaction_tx: TransactionSender,
    account_tx: AccountSender,
    block_rx: Option<BlockReceiver>,
    transaction_rx: Option<TransactionReceiver>,
    account_rx: Option<AccountReceiver>,
    provider: P,
    head_number: u64,
}

impl<P> ChainMonitor<P> {
    /// Constructs a new `ChainMonitor` based on a specific `ChainProvider`.
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
            head_number: 0u64,
        }
    }

    /// Relinquishes ownership of receivers for various chain data receivers
    /// required for the delivery of chain data to the UI of the app.
    ///
    /// # Panics
    ///
    /// If this function is called more than once, it will panic.
    pub fn receivers(&mut self) -> (BlockReceiver, TransactionReceiver, AccountReceiver) {
        (
            self.block_rx.take().unwrap(),
            self.transaction_rx.take().unwrap(),
            self.account_rx.take().unwrap(),
        )
    }
}

impl<P: ChainProvider + Sync> ChainMonitor<P> {
    /// Uses a [`ChainProvider`] to get the latest block, transactions, and account balances.
    pub async fn run(&mut self) -> color_eyre::Result<()> {
        // Retrieve the latest block.
        let block = self.provider.head().await?;
        // Do not send duplicate blocks.
        if block.number > self.head_number {
            self.head_number = block.number;
            // Send the block.
            self.block_tx.send(vec![block])?;
            // Get and send the transactions.
            let txs = self.provider.transactions().await?;
            self.transaction_tx.send(txs)?;
            // Get and send the account balances.
            let bals = self.provider.balances().await?;
            self.account_tx.send(bals)?;
        }
        Ok(())
    }
}
