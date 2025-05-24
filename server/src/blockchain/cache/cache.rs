use crate::blockchain::blockchain::Blockchain;

use super::user::User;
use nexium::{defaults::SIG_SAMPLE, gitlab::GitlabClient, rsa::KeyPair};
use num_bigint::BigUint;
use std::{collections::HashMap, str::FromStr};

pub struct Cache<'a> {
    pub data: HashMap<String, User>,
    pub gitlab: &'a GitlabClient,
    pub blockchain: &'a mut Blockchain<'a>,
}

impl<'a> Cache<'a> {
    pub fn new(
        gitlab: &'a GitlabClient,
        blockchain: &'a mut Blockchain<'a>,
    ) -> Self {
        Self {
            data: HashMap::new(),
            gitlab,
            blockchain,
        }
    }

    // pub fn get(&self, login: &String) -> Option<User> {
    //     self.data.get(login).cloned()
    // }

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

    // pub fn update_balance(&mut self, login: &String) -> Result<f32, String> {
    //     let mut user = self.get_user(login);
    //     let mut offset = 0;
    //     let mut balance = INITIAL_BALANCE as f32;

    //     while offset <= self.blockchain.size {
    //         let block = match self.blockchain.read_block(offset) {
    //             Ok(b) => b,
    //             Err(_) => {
    //                 return Err("Failed to read blockchain file".to_string());
    //             }
    //         };

    //         for tr in &block.transactions {
    //             if tr.header.get_login() == *login
    //                 || tr.header.data_type == DataType::ClassicTransaction
    //             {
    //                 match tr.get_data() {
    //                     Ok(data) => match data {
    //                         TransactionData::ClassicTransaction {
    //                             amount,
    //                             receiver,
    //                             ..
    //                         } => {
    //                             if login.as_bytes() == receiver {
    //                                 balance += amount;
    //                             } else if tr.header.get_login() == *login {
    //                                 balance -= amount;
    //                             }
    //                         }
    //                         _ => (),
    //                     },
    //                     Err(_) => {
    //                         return Err(
    //                             "Failed to get transaction data".to_string()
    //                         );
    //                     }
    //                 }
    //             }
    //         }
    //         offset += block.size() as u64;
    //     }

    //     // let balance = INITIAL_BALANCE; //
    //     user.balance = Some(balance);
    //     self.data.insert(login.clone(), user);
    //     return Ok(balance);
    // }

    fn check_keys(
        &self,
        keys: &Vec<KeyPair>,
        sig: &String,
        message: &Vec<u8>,
    ) -> Option<KeyPair> {
        let s = match BigUint::from_str(sig) {
            Ok(s) => s,
            Err(_) => {
                eprintln!("Failed to parse signature");
                return None;
            }
        };

        for key in keys {
            match key.check_signature(message, &s) {
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

    pub fn get_key(
        &mut self,
        login: &String,
        sig: &String,
        message: Option<&Vec<u8>>,
    ) -> Option<KeyPair> {
        let msg = match message {
            Some(m) => m,
            None => &SIG_SAMPLE.as_bytes().to_vec(),
        };

        match self.data.get(login) {
            Some(u) => match self.check_keys(&u.keys, sig, msg) {
                Some(k) => {
                    return Some(k);
                }
                _ => (),
            },
            _ => (),
        };

        match self.update_keys(&login) {
            Ok(keys) => {
                return self.check_keys(&keys, sig, msg);
            }
            Err(_) => return None,
        }
    }
}
