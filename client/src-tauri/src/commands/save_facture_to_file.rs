use crate::core::invoice::Invoice;

#[tauri::command]
pub async fn save_facture_to_file(
    invoice: Invoice,
    path_string: String,
) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        Invoice::to_file(&invoice, &path_string).map_err(|err| err.to_string())
    })
    .await
    .map_err(|err| format!("Failed to save facture to file: {}", err))?
}
