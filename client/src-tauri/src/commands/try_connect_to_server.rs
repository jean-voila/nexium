use crate::{
    config::{self, Config},
    nexium_api::get_server_key_login,
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct TryConnectResult {
    pub pub_key: String,
    pub login: String,
    pub config: Config,
}

/// Try to connect to a specific server and return the server info if successful
#[tauri::command]
pub async fn try_connect_to_server(
    mut config: Config,
    address: String,
    port: u16,
) -> Result<TryConnectResult, String> {
    // Update config with new server
    config.server_address = address;
    config.port = port.to_string();

    let config_clone = config.clone();
    tauri::async_runtime::spawn_blocking(|| get_server_key_login(config_clone))
        .await
        .map_err(|err| format!("Failed to connect to server: {}", err))?
        .map(|(pub_key, login)| {
            config.server_login = login.clone();

            TryConnectResult {
                pub_key,
                login,
                config,
            }
        })
}
