use nexium::gitlab::{GitlabClient, TokenType};

#[tauri::command]
pub async fn get_login(
    gitlab_token_type: TokenType,
    gitlab_token: String,
) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
        GitlabClient::new(gitlab_token, gitlab_token_type)
            .get_login()
            .map_err(|err| err.to_string())
    })
    .await
    .map_err(|err| format!("Failed to get login: {}", err))?
}
