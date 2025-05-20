use super::user::User;
use nexium::{defaults::INITIAL_BALANCE, gitlab::GitlabClient, rsa::KeyPair};
use std::collections::HashMap;

pub struct Cache<'a> {
    pub data: HashMap<String, User>,
    pub gitlab: &'a GitlabClient,
}

impl<'a> Cache<'a> {
    pub fn new(gitlab: &'a GitlabClient) -> Self {
        Self {
            data: HashMap::new(),
            gitlab,
        }
    }

    pub fn get(&self, login: &String) -> Option<User> {
        self.data.get(login).cloned()
    }

    pub fn update_keys(&mut self, login: &String) -> Result<(), String> {
        let keys = match self.gitlab.get_gpg_keys(login.as_str()) {
            Ok(keys) => keys,
            Err(e) => {
                return Err(format!("Failed to get GPG keys: {}", e));
            }
        };
        let mut user = match self.data.get(login) {
            Some(u) => u.clone(),
            None => User::new(),
        };
        user.keys = keys
            .iter()
            .filter_map(|k| match KeyPair::pub_from_pem(k, &login) {
                Ok(key) => Some(key),
                Err(_) => {
                    eprintln!("Failed to create key pair");
                    None
                }
            })
            .collect();
        self.data.insert(login.clone(), user);
        Ok(())
    }

    pub fn update_balance(&mut self, login: &String) -> u32 {
        let mut user = match self.data.get(login) {
            Some(u) => u.clone(),
            None => User::new(),
        };

        let balance = INITIAL_BALANCE; //
        user.balance = Some(balance);
        self.data.insert(login.clone(), user);
        return balance;
    }
}
