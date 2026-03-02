use nexium::gitlab::GitlabClient;

#[tauri::command]
pub async fn get_gitlab_oauth_token() -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
        GitlabClient::get_token().map_err(|err| err.to_string())
    })
    .await
    .map_err(|err| format!("Failed to get gitlab oauth token: {}", err))?
}
