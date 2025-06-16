use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum TransactionDataError {
    InvalidData,
}

impl TransactionDataError {
    pub fn as_str(&self) -> &str {
        match self {
            TransactionDataError::InvalidData => "Invalid transaction data",
        }
    }
}

impl Display for TransactionDataError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.as_str())
    }
}
