use super::router::handler::handler;
use crate::{
    blockchain::{blockchain::Blockchain, cache::cache::Cache},
    config::Config,
};
use nexium::{gitlab::GitlabClient, rsa::KeyPair};
use std::{net::TcpListener, process};

pub struct Server<'a> {
    pub cache: Cache<'a>,
    pub login: String,
    address: String,
    port: u16,
    pub key: KeyPair,
}

impl<'a> Server<'a> {
    pub fn new(
        config: &Config,
        gitlab: &'a GitlabClient,
        blockchain: &'a mut Blockchain,
    ) -> Result<Self, String> {
        let key = match KeyPair::priv_from_file(
            &config.key_filepath.to_string(),
            &config.user_login.to_string(),
            &config.key_password.to_string(),
        ) {
            Ok(key) => key,
            Err(e) => {
                return Err(format!(
                    "Failed to load private key from file: {}",
                    e
                ));
            }
        };

        Ok(Self {
            cache: Cache::new(gitlab, blockchain),
            login: config.user_login.clone(),
            address: config.listen.clone(),
            port: config.port,
            key,
        })
    }

    pub fn listen(&mut self) {
        let addr = format!("{}:{}", self.address, self.port);
        let listener = match TcpListener::bind(&addr) {
            Ok(l) => l,
            Err(_) => {
                eprintln!("Failed to listen on {addr}");
                process::exit(1);
            }
        };
        println!("Server listening on {addr}");

        for s in listener.incoming() {
            match s {
                Ok(mut stream) => handler(self, &mut stream),
                Err(e) => {
                    eprintln!("Failed to accept connection: {}", e);
                }
            }
        }
    }
}
