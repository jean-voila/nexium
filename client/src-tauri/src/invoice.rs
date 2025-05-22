use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;
use std::fs;
use std::path::Path;

pub enum InvoiceError {
    InvalidAmount,
    TooLongDescription,
    InvalidLogin,
    FileFormatError,
    FileWriteError,
    FileNotFound,
}

impl fmt::Display for InvoiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            InvoiceError::InvalidAmount => "Montant invalide.",
            InvoiceError::TooLongDescription => "Description trop longue.",
            InvoiceError::InvalidLogin => "Login invalide.",
            InvoiceError::FileFormatError => "Erreur de format de fichier.",
            InvoiceError::FileWriteError => "Erreur d'écriture de fichier.",
            InvoiceError::FileNotFound => "Fichier introuvable.",
        };
        write!(f, "{msg}")
    }
}

const MAX_DESCRIPTION_LENGTH: usize = 256;

#[derive(Serialize, Deserialize, Debug)]
pub struct Invoice {
    pub sender_login: String,
    pub amount: String,
    pub description: String,
}

impl Invoice {
    pub fn check_values(&self) -> Result<(), InvoiceError> {
        // Check if the login is valid

        // Check if the amount is a valid number
        match self.amount.parse::<f64>() {
            Ok(n) => {
                if n <= 0.0 {
                    return Err(InvoiceError::InvalidAmount);
                }
            }
            Err(_) => return Err(InvoiceError::InvalidAmount),
        }
        match self.description.len() {
            0..=MAX_DESCRIPTION_LENGTH => {}
            _ => return Err(InvoiceError::TooLongDescription),
        }
        Ok(())
    }

    pub fn from_file(path: &Path) -> Result<Invoice, InvoiceError> {
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => {
                return Err(InvoiceError::FileNotFound);
            }
        };
        let invoice: Invoice = match serde_json::from_str(&content) {
            Ok(c) => c,
            Err(_) => {
                return Err(InvoiceError::FileFormatError);
            }
        };
        match invoice.check_values() {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }
        return Ok(invoice);
    }

    pub fn to_file(&self, path: &Path) -> Result<(), InvoiceError> {
        let content = match serde_json::to_string_pretty(self) {
            Ok(c) => c,
            Err(_) => {
                return Err(InvoiceError::FileFormatError);
            }
        };
        match fs::write(path, content) {
            Ok(_) => {}
            Err(_) => {
                return Err(InvoiceError::FileWriteError);
            }
        }
        return Ok(());
    }
}
