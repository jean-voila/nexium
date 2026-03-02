use crate::core::config::{Config, ConfigError};

#[tauri::command]
pub async fn load_config() -> Result<Config, String> {
    let result =
        tauri::async_runtime::spawn_blocking(move || Config::load()).await;

    match result {
        Ok(Some(config)) => Ok(config),
        _ => Err(ConfigError::FileReadError.to_string()),
    }
}
