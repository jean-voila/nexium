use crate::contacts::{Contact, ContactBook};

#[tauri::command]
pub fn contact_search(query: String) -> Vec<Contact> {
    ContactBook::load()
        .search(&query)
        .into_iter()
        .cloned()
        .collect()
}
