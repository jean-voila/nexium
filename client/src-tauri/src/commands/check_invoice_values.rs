use crate::core::invoice::Invoice;

#[tauri::command]
pub async fn check_invoice_values(invoice: Invoice) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || invoice.check_values())
        .await
        .map_err(|err| format!("Failed to check invoice values: {}", err))?
}
