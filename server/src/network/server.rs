use super::router::handler::handler;
use crate::{
    blockchain::{blockchain::Blockchain, cache::cache::Cache},
    config::Config,
};
use colored::Colorize;
use nexium::{gitlab::GitlabClient, rsa::KeyPair};
use std::{process, sync::Arc};
use tokio::{net::TcpListener, sync::Mutex};

pub struct Server {
    pub cache: Cache,
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
            cache: Cache::new(gitlab.clone()),
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
            let cache_arc = Arc::new(Mutex::new(self.cache));

            loop {
                match listener.accept().await {
                    Ok((stream, _)) => {
                        println!(
                            "Accepted connection from {}",
                            stream.peer_addr().unwrap()
                        );

                        let blockchain_arc_clone = blockchain_arc.clone();
                        let cache_arc_clone = cache_arc.clone();
                        let l = self.login.clone();
                        let k = self.key.clone();
                        let gitlab = self.gitlab.clone();

                        tokio::spawn(async move {
                            handler(
                                stream,
                                gitlab,
                                cache_arc_clone,
                                blockchain_arc_clone,
                                l,
                                k,
                            )
                            .await;
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
