use crate::types::constants::Constants;
use nexium::defaults::*;

#[tauri::command]
pub fn get_constants() -> Constants {
    Constants {
        nexium_invoice_extension: NEXIUM_INVOICE_EXTENSION.to_string(),
        is_testnet: IS_TESTNET,
    }
}
