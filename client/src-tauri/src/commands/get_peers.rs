use crate::{
    config::Config,
    core::peer_cache::save_peers_cache,
    nexium_api::{get_peers as get_peers_api, PeerInfo},
};

#[tauri::command]
pub async fn get_peers(config: Config) -> Result<Vec<PeerInfo>, String> {
    tauri::async_runtime::spawn_blocking(|| get_peers_api(config))
        .await
        .map_err(|err| format!("Failed to get peers: {}", err))?
        .and_then(|peers| {
            save_peers_cache(&peers);
            Ok(peers)
        })
}
