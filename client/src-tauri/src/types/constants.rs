use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Constants {
    pub nexium_invoice_extension: String,
    pub is_testnet: bool,
}
