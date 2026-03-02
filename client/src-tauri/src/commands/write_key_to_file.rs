use std::path::Path;

#[tauri::command]
pub async fn write_key_to_file(
    path_string: String,
    key: String,
) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || -> Result<(), String> {
        let path = Path::new(&path_string);

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }

        std::fs::write(path, key).map_err(|e| e.to_string())
    })
    .await
    .map_err(|err| format!("Failed to write key to file: {}", err))?
}
