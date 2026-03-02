use crate::core::config::Config;

#[tauri::command]
pub async fn check_config_values(config: Config) -> bool {
    tauri::async_runtime::spawn_blocking(move || {
        match Config::check_values(&config) {
            Ok(_) => true,
            Err(e) => {
                eprintln!("Config validation failed: {}", e);
                false
            }
        }
    })
    .await
    .unwrap_or(false)
}
