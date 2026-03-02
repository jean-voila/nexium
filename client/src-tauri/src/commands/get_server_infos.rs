use crate::core::config::Config;
use crate::core::nexium_api::get_server_key_login;
use crate::types::server_infos::ServerInfos;

#[tauri::command]
pub async fn get_server_infos(config: Config) -> Result<ServerInfos, String> {
    tauri::async_runtime::spawn_blocking(move || get_server_key_login(config))
        .await
        .map_err(|e| format!("Failed to get server infos: {}", e))?
}
