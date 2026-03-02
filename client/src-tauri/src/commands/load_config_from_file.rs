use crate::core::config::Config;

#[tauri::command]
pub async fn load_config_from_file(
    path_string: String,
) -> Result<Config, String> {
    tauri::async_runtime::spawn_blocking(|| Config::from_file(path_string))
        .await
        .map_err(|err| format!("Failed to load config from file: {err}"))?
}
