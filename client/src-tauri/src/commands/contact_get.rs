use crate::core::contacts::{Contact, ContactBook};

#[tauri::command]
pub fn contact_get(favorite: bool) -> Vec<Contact> {
    let book = ContactBook::load();
    if favorite {
        book.get_favorites()
    } else {
        book.contacts
    }
}
