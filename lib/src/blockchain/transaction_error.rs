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
