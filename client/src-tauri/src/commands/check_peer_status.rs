use crate::core::nexium_api::check_peer_status as check_peer_status_api;

#[tauri::command]
pub async fn check_peer_status(address: String, port: u16) -> bool {
    tauri::async_runtime::spawn_blocking(move || {
        check_peer_status_api(address, port)
    })
    .await
    .unwrap_or(false)
}
