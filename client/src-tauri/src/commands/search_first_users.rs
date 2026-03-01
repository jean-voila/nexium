use crate::config::Config;
use nexium::gitlab::GitlabClient;

#[tauri::command]
pub async fn search_first_users(
    config: Config,
    search: String,
) -> Result<Vec<String>, String> {
    tauri::async_runtime::spawn_blocking(move || {
        GitlabClient::new(config.gitlab_token, config.gitlab_token_type)
            .search_users(&search)
            .map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| format!("Failed to search first users: {}", e))?
}
