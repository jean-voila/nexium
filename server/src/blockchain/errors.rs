// use std::fmt::Display;

use futures::io;

#[derive(Debug)]
pub enum BlockchainError {
    InvalidPreviousHash,
    HashNotFound,
    // FailedToCreateBlock,
    // FailedToReadBlockchainFile,
}

impl BlockchainError {
    pub fn as_str(&self) -> &str {
        match self {
            BlockchainError::InvalidPreviousHash => {
                "Invalid previous block hash"
            }
            BlockchainError::HashNotFound => "Block hash not found",
            // BlockchainError::FailedToCreateBlock => "Failed to create block",
            // BlockchainError::FailedToReadBlockchainFile => {
            //     "Failed to read blockchain file"
            // }
        }
    }
}

// impl Display for BlockchainError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.as_str())
//     }
// }

#[derive(Debug)]
pub enum BlockchainFileError {
    FailedToSeek(io::Error),
    FailedToRead(io::Error),
}

impl BlockchainFileError {
    pub fn to_string(&self) -> String {
        match self {
            BlockchainFileError::FailedToSeek(e) => {
                format!("Failed to seek in blockchain file: {}", e)
            }
            BlockchainFileError::FailedToRead(e) => {
                format!("Failed to read from blockchain file: {}", e)
            }
        }
    }
}
