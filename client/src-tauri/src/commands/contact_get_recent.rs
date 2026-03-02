use crate::core::contacts::{Contact, ContactBook};

#[tauri::command]
pub fn contact_get_recent(limit: usize) -> Vec<Contact> {
    ContactBook::load()
        .get_recent(limit)
        .into_iter()
        .cloned()
        .collect()
}
