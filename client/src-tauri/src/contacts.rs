use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub login: String,
    pub nickname: String,
    pub notes: String,
    #[serde(default)]
    pub favorite: bool,
    #[serde(default)]
    pub created_at: u64,
    #[serde(default)]
    pub last_used: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContactBook {
    pub contacts: Vec<Contact>,
}

fn get_contacts_path() -> PathBuf {
    let mut path = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."));
    path.push("nexium");
    fs::create_dir_all(&path).ok();
    path.push("contacts.json");
    path
}

impl ContactBook {
    pub fn load() -> Self {
        let path = get_contacts_path();
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(book) = serde_json::from_str(&content) {
                return book;
            }
        }
        ContactBook::default()
    }

    pub fn save(&self) -> Result<(), String> {
        let path = get_contacts_path();
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| e.to_string())?;
        fs::write(&path, content)
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn add_contact(&mut self, contact: Contact) -> Result<(), String> {
        // Check if contact already exists
        if self.contacts.iter().any(|c| c.login == contact.login) {
            return Err("Contact already exists".to_string());
        }
        self.contacts.push(contact);
        self.save()
    }

    pub fn update_contact(
        &mut self,
        login: &str,
        nickname: Option<String>,
        notes: Option<String>,
        favorite: Option<bool>,
    ) -> Result<(), String> {
        if let Some(contact) = self.contacts.iter_mut()
            .find(|c| c.login == login) 
        {
            if let Some(n) = nickname {
                contact.nickname = n;
            }
            if let Some(n) = notes {
                contact.notes = n;
            }
            if let Some(f) = favorite {
                contact.favorite = f;
            }
            return self.save();
        }
        Err("Contact not found".to_string())
    }

    pub fn remove_contact(&mut self, login: &str) -> Result<(), String> {
        let initial_len = self.contacts.len();
        self.contacts.retain(|c| c.login != login);
        if self.contacts.len() == initial_len {
            return Err("Contact not found".to_string());
        }
        self.save()
    }

    pub fn mark_used(&mut self, login: &str) -> Result<(), String> {
        if let Some(contact) = self.contacts.iter_mut()
            .find(|c| c.login == login) 
        {
            contact.last_used = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            return self.save();
        }
        Ok(()) // Don't error if contact doesn't exist
    }

    pub fn search(&self, query: &str) -> Vec<&Contact> {
        let query_lower = query.to_lowercase();
        self.contacts.iter()
            .filter(|c| {
                c.login.to_lowercase().contains(&query_lower) ||
                c.nickname.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    pub fn get_favorites(&self) -> Vec<&Contact> {
        self.contacts.iter()
            .filter(|c| c.favorite)
            .collect()
    }

    pub fn get_recent(&self, limit: usize) -> Vec<&Contact> {
        let mut sorted: Vec<_> = self.contacts.iter().collect();
        sorted.sort_by(|a, b| b.last_used.cmp(&a.last_used));
        sorted.into_iter().take(limit).collect()
    }
}

// Tauri commands for contacts
#[tauri::command]
pub fn get_contacts() -> Result<Vec<Contact>, String> {
    let book = ContactBook::load();
    Ok(book.contacts)
}

#[tauri::command]
pub fn add_contact(
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

#[tauri::command]
pub fn update_contact(
    login: String,
    nickname: Option<String>,
    notes: Option<String>,
    favorite: Option<bool>,
) -> Result<(), String> {
    let mut book = ContactBook::load();
    book.update_contact(&login, nickname, notes, favorite)
}

#[tauri::command]
pub fn remove_contact(login: String) -> Result<(), String> {
    let mut book = ContactBook::load();
    book.remove_contact(&login)
}

#[tauri::command]
pub fn search_contacts(query: String) -> Result<Vec<Contact>, String> {
    let book = ContactBook::load();
    Ok(book.search(&query).into_iter().cloned().collect())
}

#[tauri::command]
pub fn get_favorite_contacts() -> Result<Vec<Contact>, String> {
    let book = ContactBook::load();
    Ok(book.get_favorites().into_iter().cloned().collect())
}

#[tauri::command]
pub fn get_recent_contacts(limit: usize) -> Result<Vec<Contact>, String> {
    let book = ContactBook::load();
    Ok(book.get_recent(limit).into_iter().cloned().collect())
}

#[tauri::command]
pub fn mark_contact_used(login: String) -> Result<(), String> {
    let mut book = ContactBook::load();
    book.mark_used(&login)
}
