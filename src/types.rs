use std::fmt::{Debug, Display, Formatter};

pub trait Abbreviated {
    fn abbreviated(&self) -> String;
}

impl Abbreviated for String {
    fn abbreviated(&self) -> String {
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
}

#[derive(Debug, Clone)]
pub struct Account {
    pub address: String,
    pub balance: String,
}

impl Display for Account {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.address, self.balance)
    }
}
