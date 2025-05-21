use super::router::handler::handler;
use crate::{blockchain::cache::cache::Cache, config::Config};
use nexium::{blockchain::transaction::Transaction, gitlab::GitlabClient};
use std::{net::TcpListener, process};

pub struct Server<'a> {
    pub cache: Cache<'a>,
    mempool: Vec<Transaction>,
    pub gitlab: &'a GitlabClient,
    pub login: String,
    address: String,
    port: u16,
}

impl<'a> Server<'a> {
    pub fn new(config: &Config, gitlab: &'a GitlabClient) -> Self {
        Self {
            cache: Cache::new(gitlab),
            mempool: Vec::new(),
            gitlab,
            login: config.user_login.clone(),
            address: config.listen.clone(),
            port: config.port,
        }
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
