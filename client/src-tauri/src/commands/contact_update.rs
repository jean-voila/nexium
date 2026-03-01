use crate::contacts::ContactBook;

#[tauri::command]
pub fn contact_update(
    login: String,
    nickname: Option<String>,
    notes: Option<String>,
    favorite: Option<bool>,
) -> Result<(), String> {
    let mut book = ContactBook::load();
    book.update_contact(&login, nickname, notes, favorite)
}
