use crate::config::Config;

#[tauri::command]
pub async fn save_config_to_file(
    config: Config,
    path_string: String,
) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        Config::to_file(&config, &path_string).map_err(|err| err.to_string())
    })
    .await
    .map_err(|err| format!("Failed to save config to file: {err}"))?
}
