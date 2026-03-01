use crate::config::Config;

#[tauri::command]
pub async fn save_config(config: Config) -> bool {
    tauri::async_runtime::spawn_blocking(move || config.save())
        .await
        .map(|_| true)
        .unwrap_or_else(|e| {
            eprintln!("Error saving config: {}", e);
            false
        })
}
