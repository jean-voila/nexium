use crate::core::contacts::ContactBook;

#[tauri::command]
pub fn contact_remove(login: String) -> Result<(), String> {
    ContactBook::load().remove_contact(&login)
}
