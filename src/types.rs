use std::fmt::Debug;

pub trait Abridged {
    fn abridged(&self) -> String;
}

impl Abridged for String {
    fn abridged(&self) -> String {
        let prefix_len = 6;
        let suffix_len = 4;
        if self.len() <= prefix_len + suffix_len {
            self.clone()
        } else {
            let prefix = &self[..prefix_len];
            let suffix = &self[self.len() - suffix_len..];
            format!("{}..{}", prefix, suffix)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    pub number: u64,
    pub hash: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub hash: String,
    pub from: String,
    pub to: String,
    pub value: String,
    pub unit: String,
}

#[derive(Debug, Clone)]
pub struct Account {
    pub address: String,
    pub balance: String,
    pub unit: String,
}
