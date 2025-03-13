use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Copy, Clone)]
pub struct Block {
    pub number: u64,
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub block_number: u64,
    pub hash: String,
    pub from: String,
    pub to: String,
    pub value: String,
}

impl Display for Transaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {} {}",
            self.block_number,
            truncate_hex(self.from.as_str()),
            truncate_hex(self.to.as_str()),
            truncate_hex(self.hash.as_str()),
            self.value,
        )
    }
}

pub fn truncate_hex(hex: &str) -> String {
    let prefix_len = 6;
    let suffix_len = 4;
    if hex.len() <= prefix_len + suffix_len {
        hex.to_string()
    } else {
        let prefix = &hex[..prefix_len];
        let suffix = &hex[hex.len() - suffix_len..];

        format!("{}..{}", prefix, suffix)
    }
}

#[derive(Debug, Clone)]
pub struct Account {
    pub id: String,
    pub balance: String,
}

impl Display for Account {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", truncate_hex(self.id.as_str()), self.balance)
    }
}
