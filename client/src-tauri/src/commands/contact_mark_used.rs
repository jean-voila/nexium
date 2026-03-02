use crate::core::contacts::ContactBook;

#[tauri::command]
pub fn contact_mark_used(login: String) -> Result<(), String> {
    ContactBook::load().mark_used(&login)
}
