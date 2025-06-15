use super::router::handler::handler;
use crate::{blockchain::blockchain::Blockchain, config::Config};
use colored::Colorize;
use nexium::{gitlab::GitlabClient, rsa::KeyPair};
use std::{process, sync::Arc};
use tokio::{net::TcpListener, sync::Mutex};

pub struct Server {
    gitlab: GitlabClient,
    blockchain: Blockchain,
    pub login: String,
    address: String,
    port: u16,
    pub key: KeyPair,
}

impl Server {
    pub fn new(
        config: &Config,
        gitlab: GitlabClient,
        key: KeyPair,
        blockchain: Blockchain,
    ) -> Result<Self, String> {
        Ok(Self {
            gitlab: gitlab,
            blockchain: blockchain,
            login: config.user_login.clone(),
            address: config.listen.clone(),
            port: config.port,
            key,
        })
    }

    pub async fn listen(self) {
        let addr = format!("{}:{}", self.address, self.port);

        let listener = match TcpListener::bind(&addr).await {
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

        {
            let blockchain_arc = Arc::new(Mutex::new(self.blockchain));
            let gitlab_arc = Arc::new(Mutex::new(self.gitlab));
            let login_arc = Arc::new(self.login);
            let key_arc = Arc::new(self.key);

            loop {
                match listener.accept().await {
                    Ok((stream, _)) => {
                        println!(
                            "Accepted connection from {}",
                            stream.peer_addr().unwrap()
                        );

                        let blockchain = blockchain_arc.clone();
                        let gitlab = gitlab_arc.clone();
                        let login = login_arc.clone();
                        let key = key_arc.clone();

                        tokio::spawn(async move {
                            handler(stream, gitlab, blockchain, login, key)
                                .await
                                .unwrap_or_else(|e| {
                                    eprintln!("Error handling request: {}", e);
                                });
                        });
                    }
                    Err(e) => {
                        eprintln!("Failed to accept connection: {}", e);
                    }
                }
            }
        }
    }
}
