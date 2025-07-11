use super::router::handler::handler;
use crate::{
    blockchain::blockchain::Blockchain, config::Config,
    network::cache::ServerCache,
};
use colored::Colorize;
use nexium::{gitlab::GitlabClient, rsa::KeyPair};
use std::{
    process,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use tokio::{net::TcpListener, sync::Mutex};

pub struct Server {
    gitlab: Arc<Mutex<GitlabClient>>,
    blockchain: Arc<Mutex<Blockchain>>,
    login: Arc<String>,
    address: String,
    port: u16,
    key: Arc<KeyPair>,
    cache: Arc<Mutex<ServerCache>>,
    stopped: Arc<AtomicBool>,
}

impl Server {
    pub fn new(
        config: &Config,
        gitlab: GitlabClient,
        key: KeyPair,
        blockchain: Blockchain,
    ) -> Result<Self, String> {
        Ok(Self {
            gitlab: Arc::new(Mutex::new(gitlab)),
            blockchain: Arc::new(Mutex::new(blockchain)),
            login: Arc::new(config.user_login.clone()),
            address: config.listen.clone(),
            port: config.port,
            key: Arc::new(key),
            cache: Arc::new(Mutex::new(ServerCache::new())),
            stopped: Arc::new(AtomicBool::new(false)),
        })
    }

    pub async fn listen(&self) {
        let addr = format!("{}:{}", self.address, self.port);

        let listener = match TcpListener::bind(&addr).await {
            Ok(l) => l,
            Err(_) => {
                eprintln!("Failed to listen on {addr}");
                process::exit(1);
            }
        };

        println!(
            "Server listening on {}:{}\n",
            self.address.green(),
            self.port.to_string().yellow()
        );

        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    if self.stopped.load(Ordering::Relaxed) {
                        println!(
                            "Server is stopped, not accepting new connections"
                        );
                        continue;
                    }

                    // println!(
                    //     "Accepted connection from {}",
                    //     stream.peer_addr().unwrap()
                    // );

                    let blockchain = self.blockchain.clone();
                    let gitlab = self.gitlab.clone();
                    let login = self.login.clone();
                    let key = self.key.clone();
                    let cache = self.cache.clone();
                    let stopped_arc = self.stopped.clone();

                    let id = self.cache.lock().await.generate_id();
                    // println!("Generated connection ID: {}", id);

                    let handle = tokio::spawn(async move {
                        // println!("Handling connection ID: {}", id);
                        handler(stream, gitlab, blockchain, login, key)
                            .await
                            .unwrap_or_else(|e| {
                                eprintln!("Error handling request: {}", e);
                            });
                        // println!("Connection ID {} finished", id);

                        if !stopped_arc.load(Ordering::Relaxed) {
                            cache.lock().await.remove_conn(id);
                            // println!("Connection ID {} removed from cache", id);
                        }
                    });

                    self.cache.lock().await.add_conn(id, handle);
                    // println!("Connection {} added", id);
                }
                Err(e) => {
                    eprintln!("Failed to accept connection: {}", e);
                }
            }
        }
    }

    pub async fn stop(&self) {
        self.stopped.store(true, Ordering::Relaxed);
        self.cache.lock().await.clear().await;
    }
}
