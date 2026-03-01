use crate::config::Config;
use crate::nexium_api::get_server_key_login;
use crate::types::server_infos::ServerInfos;
use nexium::gitlab::GitlabError;

#[tauri::command]
pub async fn get_server_infos(config: Config) -> Result<ServerInfos, String> {
    tauri::async_runtime::spawn_blocking(move || {
        match get_server_key_login(config) {
            Ok(res) => Ok(ServerInfos {
                pub_key: res.0,
                login: res.1,
            }),
            Err(_) => Err(GitlabError::UserNotFound.to_string()),
        }
    })
    .await
    .map_err(|e| format!("Failed to get server infos: {}", e))?
}
