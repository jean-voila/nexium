use crate::types::login_names::LoginNames;
use nexium::login::Login;

#[tauri::command]
pub async fn get_names_from_login(login: String) -> Result<LoginNames, String> {
    // Utilise login::new(login) et login::get_names() pour obtenir le nom et le prénom
    Login::new(login)
        .map_err(|e| format!("Failed to create login: {}", e))?
        .get_names()
        .map_err(|e| format!("Failed to get names from login: {}", e))
        .map(|(first_name, last_name)| LoginNames {
            first_name,
            last_name,
        })
}
