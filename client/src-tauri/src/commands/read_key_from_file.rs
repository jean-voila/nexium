use std::path::Path;

#[tauri::command]
pub async fn read_key_from_file(path_string: String) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let path = Path::new(&path_string);
        if !path.exists() {
            Err("Le fichier n'existe pas".to_string())
        } else {
            std::fs::read_to_string(path).map_err(|e| e.to_string())
        }
    })
    .await
    .map_err(|err| format!("Failed to read key from file: {}", err))?
}
