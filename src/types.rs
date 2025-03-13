use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone, Default)]
pub struct HexElement(String);

impl HexElement {
    pub fn to_full_string(&self) -> String {
        self.0.clone()
    }
}

impl Display for HexElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let prefix_len = 6;
        let suffix_len = 4;
        let hex = &self.0;
        if hex.len() <= prefix_len + suffix_len {
            write!(f, "{}", hex)
        } else {
            let prefix = &hex[..prefix_len];
            let suffix = &hex[hex.len() - suffix_len..];
            write!(f, "{}..{}", prefix, suffix)
        }
    }
}

impl<T: Into<String>> From<T> for HexElement {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Block {
    pub number: u64,
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub hash: HexElement,
    pub from: HexElement,
    pub to: HexElement,
    pub value: HexElement,
}

#[derive(Debug, Clone)]
pub struct Account {
    pub address: HexElement,
    pub balance: String,
}

impl Display for Account {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.address, self.balance)
    }
}
