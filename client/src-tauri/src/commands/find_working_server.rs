use crate::config::Config;
use crate::core::peer_cache::get_cached_peers;
use crate::nexium_api::get_server_key_login;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct WorkingServerInfo {
    pub pub_key: String,
    pub login: String,
    pub config: Config,
}

/// Try to find a working server from the cached peer list
#[tauri::command]
pub async fn find_working_server(
    config: Config,
) -> Result<WorkingServerInfo, String> {
    // First try the current server
    let config_clone = config.clone();
    let current_result = tauri::async_runtime::spawn_blocking(move || {
        get_server_key_login(config_clone)
    })
    .await;

    if let Ok(Ok(server_infos)) = current_result {
        let login = server_infos.login.clone();
        let mut updated_config = config.clone();
        updated_config.server_login = login.clone();
        return Ok(WorkingServerInfo {
            pub_key: server_infos.pub_key,
            login,
            config: updated_config,
        });
    }

    // Current server failed, try to get cached peers
    // We need to try each peer until one works
    let peers = get_cached_peers();

    for peer in peers {
        // Skip current server
        if peer.address == config.server_address
            && peer.port.to_string() == config.port
        {
            continue;
        }

        let mut test_config = config.clone();
        test_config.server_address = peer.address.clone();
        test_config.port = peer.port.to_string();

        let test_config_clone = test_config.clone();
        let result = tauri::async_runtime::spawn_blocking(move || {
            get_server_key_login(test_config_clone)
        })
        .await;

        if let Ok(Ok(server_infos)) = result {
            let login = server_infos.login;
            test_config.server_login = login.clone();
            println!(
                "Failover: switched to server {}:{}",
                peer.address, peer.port
            );
            return Ok(WorkingServerInfo {
                pub_key: server_infos.pub_key,
                login: login,
                config: test_config,
            });
        }
    }

    Err("No available servers found".to_string())
}
