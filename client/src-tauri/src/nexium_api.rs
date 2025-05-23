use super::config::*;
use nexium::blockchain::transaction::*;
use nexium::defaults::*;
use nexium::rsa::*;
use reqwest::header::PUBLIC_KEY_PINS_REPORT_ONLY;
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
    InvalidPrivateKeyOrPassword,
}

impl fmt::Display for NexiumAPIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            NexiumAPIError::UnknownError => "Erreur inconnue.",
            NexiumAPIError::InvalidPrivateKeyOrPassword => {
                "Clé privée ou mot de passe invalide."
            }
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
    pub_key: String,
    transaction: ClassicTransactionSent,
    config: Config,
) -> Result<(), String> {
    let headers = build_headers(&config);
    todo!();
}

pub fn get_balance(
    pub_key: String,
    login: String,
    config: Config,
) -> Result<(String, String), String> {
    let headers = build_headers(&config);
    todo!();
}

pub fn get_transactions(
    pub_key: String,
    config: Config,
    login: String,
    n: String,
) -> Result<Vec<ClassicTransactionReceived>, String> {
    let headers = build_headers(&config);
    todo!();
}

fn build_headers(
    config: &Config,
) -> Result<reqwest::header::HeaderMap, String> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Login",
        reqwest::header::HeaderValue::from_str(&config.user_login).unwrap(),
    );

    let private_key = match KeyPair::priv_from_pem(
        &config.priv_key,
        &config.password,
        &config.user_login,
    ) {
        Ok(key) => key,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    let signature = match private_key.sign(SIG_SAMPLE) {
        Ok(sig) => sig,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    headers.insert(
        "Sig-Sample",
        match reqwest::header::HeaderValue::from_str(&signature.to_string()) {
            Ok(sig) => sig,
            Err(e) => {
                return Err(e.to_string());
            }
        },
    );

    headers.insert(
        "Content-Type",
        reqwest::header::HeaderValue::from_static("text/plain"),
    );
    return Ok(headers);
}
