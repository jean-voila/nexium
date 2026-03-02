use crate::core::contacts::{Contact, ContactBook};

#[tauri::command]
pub fn contact_add(
    login: String,
    nickname: String,
    notes: String,
    favorite: bool,
) -> Result<(), String> {
    let mut book = ContactBook::load();
    let contact = Contact {
        login,
        nickname,
        notes,
        favorite,
        created_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0),
        last_used: 0,
    };
    book.add_contact(contact)
}
