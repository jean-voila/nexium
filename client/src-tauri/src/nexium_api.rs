use super::config::*;
use nexium::blockchain::transaction::*;
use nexium::defaults::*;
use nexium::rsa::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransactionInOrOout {
    IN,
    OUT,
}

#[derive(Debug)]
pub enum NexiumAPIError {
    UnknownError,
}

impl fmt::Display for NexiumAPIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            NexiumAPIError::UnknownError => "Unknown error occurred.",
        };
        write!(f, "{}", msg)
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassicTransactionSent {
    pub receiver: String,
    pub amount: String,
    pub description: String,
    pub fees: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassicTransactionReceived {
    pub receiver: String,
    pub emitter: String,
    pub description: String,
    pub amount: String,
    pub date: String,
    pub in_or_out: TransactionInOrOout,
}

pub fn send_transaction(
    transaction: ClassicTransactionSent,
    config: Config,
) -> Result<(), NexiumAPIError> {
    todo!();
}

pub fn get_balance(
    login: String,
    config: Config,
) -> Result<(String, String), NexiumAPIError> {
    todo!();
}

pub fn get_transactions(
    config: Config,
    login: String,
    n: String,
) -> Result<Vec<ClassicTransactionReceived>, NexiumAPIError> {
    todo!();
}
