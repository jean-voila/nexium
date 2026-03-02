use nexium::{login::Login, types::login_names::LoginNames};

#[tauri::command]
pub async fn get_names_from_login(login: String) -> Result<LoginNames, String> {
    // Utilise login::new(login) et login::get_names() pour obtenir le nom et le prénom
    Login::new(login)
        .map_err(|e| format!("Failed to create login: {}", e))?
        .get_names()
        .map_err(|e| format!("Failed to get names from login: {}", e))
}
