use crate::contacts::{Contact, ContactBook};

// Tauri commands for contacts
#[tauri::command]
pub fn contact_get() -> Vec<Contact> {
    ContactBook::load().contacts
}
