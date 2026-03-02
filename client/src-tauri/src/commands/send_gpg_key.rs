use nexium::gitlab::{GitlabClient, TokenType};

#[tauri::command]
pub async fn send_gpg_key(
    gitlab_token_type: TokenType,
    gitlab_token: String,
    pub_key: String,
) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        GitlabClient::new(gitlab_token, gitlab_token_type)
            .add_gpg_key(&pub_key)
            .map_err(|err| err.to_string())
    })
    .await
    .map_err(|err| format!("Failed to send GPG key: {}", err))?
}
