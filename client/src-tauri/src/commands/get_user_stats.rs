use crate::core::{
    config::Config,
    nexium_api::{get_user_stats as get_user_stats_api, UserStats},
};

#[tauri::command]
pub async fn get_user_stats(
    login: String,
    config: Config,
) -> Result<UserStats, String> {
    tauri::async_runtime::spawn_blocking(move || {
        get_user_stats_api(login, config)
    })
    .await
    .map_err(|err| format!("Failed to execute get_user_stats: {}", err))?
}
