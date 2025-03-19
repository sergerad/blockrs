use std::fmt::Debug;

use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub type BlockSender = UnboundedSender<Vec<Block>>;
pub type TransactionSender = UnboundedSender<Vec<Transaction>>;
pub type AccountSender = UnboundedSender<Vec<Account>>;
pub type BlockReceiver = UnboundedReceiver<Vec<Block>>;
pub type TransactionReceiver = UnboundedReceiver<Vec<Transaction>>;
pub type AccountReceiver = UnboundedReceiver<Vec<Account>>;

/// Contains the chain-agnostic data required to represent a block in the UI.
#[derive(Debug, Clone, Default)]
pub struct Block {
    pub number: u64,
    pub hash: String,
    pub timestamp: u64,
}

/// Contains the chain-agnostic data required to represent a transaction block in the UI.
#[derive(Debug, Clone, Default)]
pub struct Transaction {
    pub hash: String,
    pub from: String,
    pub to: String,
    pub value: String,
    pub units: String,
    pub kind: String,
    pub nonce: String,
}

/// Contains the chain-agnostic data required to represent an account block in the UI.
#[derive(Debug, Clone, Default)]
pub struct Account {
    pub address: String,
    pub balance: String,
    pub units: String,
}

/// Produces a shortened `String` representation of some type.
///
/// Used as an extension for `String`.
pub trait Abridged {
    fn abridged(&self) -> String;
}

impl<T: AsRef<str>> Abridged for T {
    /// Shortens the content by replacng content with ellipses.
    fn abridged(&self) -> String {
        let prefix_len = 6;
        let suffix_len = 4;
        let s = self.as_ref();
        if s.len() <= prefix_len + suffix_len {
            s.to_string()
        } else {
            let prefix = &s[..prefix_len];
            let suffix = &s[s.len() - suffix_len..];
            format!("{}..{}", prefix, suffix)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Abridged;
    #[test]
    fn abridged() {
        // &str
        let s = "012345abcd3210";
        assert_eq!("012345..3210".to_string(), s.abridged());
        // String
        let s = "012345abcd3210".to_string();
        assert_eq!("012345..3210".to_string(), s.abridged());
    }
}
