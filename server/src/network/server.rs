use super::router::handler::handler;
use crate::{
    blockchain::{blockchain::Blockchain, cache::cache::Cache},
    config::Config,
};
use colored::Colorize;
use nexium::{gitlab::GitlabClient, rsa::KeyPair};
use std::{net::TcpListener, process};

pub struct Server<'a> {
    pub cache: Cache<'a>,
    pub login: String,
    address: String,
    port: u16,
    pub key: &'a KeyPair,
}

impl<'a> Server<'a> {
    pub fn new(
        config: &Config,
        gitlab: &'a GitlabClient,
        key: &'a KeyPair,
        blockchain: &'a mut Blockchain,
    ) -> Result<Self, String> {
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
        println!(
            "Server listening on {}:{}",
            self.address.green(),
            self.port.to_string().yellow()
        );

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
