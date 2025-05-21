use super::user::User;
use nexium::{
    defaults::{INITIAL_BALANCE, SIG_SAMPLE},
    gitlab::GitlabClient,
    rsa::KeyPair,
};
use num_bigint::BigUint;
use std::{collections::HashMap, str::FromStr};

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

    fn get_user(&self, login: &String) -> User {
        match self.data.get(login) {
            Some(u) => u.clone(),
            None => User::new(),
        }
    }

    pub fn update_keys(
        &mut self,
        login: &String,
    ) -> Result<Vec<KeyPair>, String> {
        let keys = match self.gitlab.get_gpg_keys(login.as_str()) {
            Ok(keys) => keys,
            Err(e) => {
                return Err(format!("Failed to get GPG keys: {}", e));
            }
        };
        let mut user = self.get_user(login);
        let keys: Vec<KeyPair> = keys
            .iter()
            .filter_map(|k| match KeyPair::pub_from_pem(k, &login) {
                Ok(key) => Some(key),
                Err(_) => {
                    eprintln!("Failed to create key pair");
                    None
                }
            })
            .collect();
        user.keys = keys.clone();
        self.data.insert(login.clone(), user);
        Ok(keys)
    }

    pub fn update_balance(&mut self, login: &String) -> u32 {
        let mut user = self.get_user(login);
        let balance = INITIAL_BALANCE; //
        user.balance = Some(balance);
        self.data.insert(login.clone(), user);
        return balance;
    }

    fn check_keys(&self, keys: &Vec<KeyPair>, sig: &String) -> Option<KeyPair> {
        let s = match BigUint::from_str(sig) {
            Ok(s) => s,
            Err(_) => {
                eprintln!("Failed to parse signature");
                return None;
            }
        };

        for key in keys {
            match key.check_signature(SIG_SAMPLE, &s) {
                Ok(b) => {
                    if b {
                        return Some(key.clone());
                    }
                }
                Err(_) => {
                    eprintln!("Failed to check signature");
                }
            };
        }
        None
    }

    pub fn get_key(&mut self, login: &String, sig: &String) -> Option<KeyPair> {
        match self.data.get(login) {
            Some(u) => match self.check_keys(&u.keys, sig) {
                Some(k) => {
                    return Some(k);
                }
                _ => (),
            },
            _ => (),
        };

        match self.update_keys(&login) {
            Ok(keys) => {
                return self.check_keys(&keys, sig);
            }
            Err(_) => return None,
        }
    }
}
