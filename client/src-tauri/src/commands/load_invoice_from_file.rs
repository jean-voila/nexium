use crate::invoice::{Invoice, InvoiceError};
use std::path::Path;

#[tauri::command]
pub async fn load_invoice_from_file(
    path_string: String,
) -> Result<Invoice, String> {
    tauri::async_runtime::spawn_blocking(move || {
        if Path::new(&path_string).exists() == false {
            return Err(InvoiceError::FileNotFound.to_string());
        }

        Invoice::from_file(&path_string)
    })
    .await
    .map_err(|e| format!("Failed to read invoice file: {}", e))?
}
