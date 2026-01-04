mod blockchain;
mod config;
mod network;
mod peers;

use blockchain::blockchain::Blockchain;
use colored::Colorize;
use config::Config;
use network::server::Server;
use peers::PeerList;
use nexium::{
    defaults::*,
    gitlab::{GitlabClient, TokenType},
    rsa::KeyPair,
};
use std::{env, fs, path::Path};
use tokio;

const HELP_ARG: &str = "--help";
const GEN_CONFIG_ARG: &str = "--generate-config";
const GEN_KEY_ARG: &str = "--generate-key";
const DEFAULT_CONFIG_NAME: &str = "config.json";

#[tokio::main]
async fn main() {
    let args = env::args().collect::<Vec<String>>();

    let local_path_str = NEXIUM_HOME;
    let local_path = Path::new(&local_path_str);
    let config_path = local_path.join(DEFAULT_CONFIG_NAME);
    let config_path_str = config_path.to_string_lossy();

    if !local_path.exists() {
        match fs::create_dir_all(&local_path) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Failed to create directory: {}", e);
                return;
            }
        }
    }

    if args.len() > 1 {
        match args[1].as_str() {
            HELP_ARG | "-h" => {
                println!(
                    "Usage: {} [options]\n\nOptions:\n  {}: Show this help message\n  {}: Generate the config file\n  {}: Generate a new key",
                    args[0].cyan().bold(),
                    HELP_ARG.cyan().bold(),
                    GEN_CONFIG_ARG.cyan().bold(),
                    GEN_KEY_ARG.cyan().bold()
                );
                return;
            }
            GEN_CONFIG_ARG => {
                if config_path.exists() {
                    let q = "Config file already exists. Do you want to overwrite it? (y/n): ";
                    let ans = Config::get_user_input(q);
                    if ans.to_lowercase() != "y" {
                        println!("Aborting config generation.");
                        return;
                    }
                }

                // Generate the config file
                println!(
                    "Generating the config file at {}...\n",
                    config_path_str.cyan().bold()
                );

                Config::generate(&config_path);
                println!(
                    "\nConfig file generated at {}",
                    config_path_str.cyan().bold()
                );

                // Generate the peers file
                let peers_path = PeerList::get_peers_file_path();
                if !PeerList::file_exists() {
                    print!("Generating peers file at {}... ", peers_path.cyan().bold());
                    match PeerList::generate() {
                        Ok(_) => println!("{}", "OK".green()),
                        Err(e) => println!("{}: {}", "FAILED".red(), e),
                    }
                    println!(
                        "{}  Edit {} to add known peers and bootstrap the network.",
                        "Note: ".yellow().bold(),
                        peers_path.cyan()
                    );
                } else {
                    println!("Peers file already exists at {}", peers_path.cyan().bold());
                }

                return;
            }
            GEN_KEY_ARG => {
                // Generate a new key
                if !config_path.exists() {
                    eprintln!(
                        "Config file not found at {}. Please generate it with {}",
                        config_path_str.cyan(),
                        GEN_CONFIG_ARG.yellow().bold()
                    );
                    return;
                }

                let config = Config::from_file(&config_path);
                let key_path_str = &config.key_filepath;
                let key_path = Path::new(&key_path_str);
                let gitlab =
                    GitlabClient::new(config.gitlab_token.clone(), TokenType::Classic);

                // Use block_in_place to allow blocking operations in async context
                let token_check = tokio::task::block_in_place(|| {
                    gitlab.check_token()
                });
                
                match token_check {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!(
                            "Failed to check Gitlab token: {}",
                            e.to_string().red()
                        );
                        return;
                    }
                }

                if key_path.exists() {
                    let q = "Key file already exists. Do you want to overwrite it? (y/n): ";
                    let ans = Config::get_user_input(q);
                    if ans.to_lowercase() != "y" {
                        println!("Aborting key generation.");
                        return;
                    }
                }

                println!(
                    "Generating new key at {}...",
                    key_path_str.cyan().bold()
                );

                let key =
                    KeyPair::generate(KEYPAIR_BIT_SIZE, &config.user_login);

                let priv_pem = key.priv_to_pem(&config.key_password);
                match fs::write(&key_path, priv_pem) {
                    Ok(_) => {
                        println!(
                            "Private key saved to {}",
                            key_path_str.cyan().bold()
                        );
                    }
                    Err(e) => {
                        eprintln!(
                            "Failed to save private key: {}",
                            e.to_string().red()
                        );
                        return;
                    }
                }

                let pub_pem = key.pub_to_pem();
                let add_key_result = tokio::task::block_in_place(|| {
                    gitlab.add_gpg_key(&pub_pem)
                });
                match add_key_result {
                    Ok(_) => {
                        println!(
                            "Public key added to Gitlab for user {}",
                            config.user_login.cyan().bold()
                        );
                    }
                    Err(e) => {
                        eprintln!(
                            "Failed to add public key to Gitlab: {}",
                            e.to_string().red()
                        );
                        return;
                    }
                }

                println!("New key generated successfully");
                return;
            }
            _ => {
                eprintln!(
                    "Unknown argument: {}. Use {} for help.",
                    args[1].red(),
                    HELP_ARG.cyan().bold()
                );
                return;
            }
        }
    }

    if !config_path.exists() {
        eprintln!("Config file not found. Please generate it with {}", GEN_CONFIG_ARG);
        return;
    }

    let config = Config::from_file(&config_path);

    let gitlab =
        GitlabClient::new(config.gitlab_token.clone(), TokenType::Classic);

    match gitlab.check_token_async().await {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Failed to check Gitlab token: {}", e);
            return;
        }
    }

    let key = match KeyPair::priv_from_file(
        &config.key_filepath,
        &config.user_login,
        &config.key_password,
    ) {
        Ok(k) => k,
        Err(e) => {
            eprintln!("Failed to load private key: {}", e);
            return;
        }
    };

    let mut blockchain = match Blockchain::init(gitlab.clone()) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Failed to create blockchain: {}", e);
            return;
        }
    };

    // Load and discover peers
    let mut peer_list = PeerList::load();
    let our_block_count = blockchain.cache.len() as u64;

    if !peer_list.peers.is_empty() {
        let (discovered, best_peer) = peer_list
            .discover(&config.listen, config.port)
            .await;
        
        if discovered > 0 {
            println!("Discovered {} new peer(s)", discovered);
        }

        // Sync blockchain if needed
        if let Some((peer, peer_info)) = best_peer {
            if our_block_count == 0 && peer_info.block_count > 0 {
                // We have no blockchain, download from peer
                println!(
                    "Downloading blockchain from {} ({} blocks)...",
                    peer.url().cyan(),
                    peer_info.block_count
                );
                match peer.download_blockchain().await {
                    Ok(data) => {
                        match blockchain.replace_from_data(&data) {
                            Ok(_) => println!(
                                "{} Blockchain synchronized ({} blocks)",
                                "OK".green(),
                                blockchain.cache.len()
                            ),
                            Err(e) => eprintln!("{} Failed to load blockchain: {}", "ERROR".red(), e),
                        }
                    }
                    Err(e) => eprintln!("{} Failed to download blockchain: {}", "ERROR".red(), e),
                }
            } else if peer_info.block_count > our_block_count {
                // Peer has longer chain, adopt it
                println!(
                    "Peer {} has longer chain ({} vs {}), syncing...",
                    peer.url().cyan(),
                    peer_info.block_count,
                    our_block_count
                );
                match peer.download_blockchain().await {
                    Ok(data) => {
                        match blockchain.replace_from_data(&data) {
                            Ok(_) => println!(
                                "{} Blockchain synchronized ({} blocks)",
                                "OK".green(),
                                blockchain.cache.len()
                            ),
                            Err(e) => eprintln!("{} Failed to load blockchain: {}", "ERROR".red(), e),
                        }
                    }
                    Err(e) => eprintln!("{} Failed to download blockchain: {}", "ERROR".red(), e),
                }
            } else if our_block_count > 0 {
                println!(
                    "Local blockchain is up to date ({} blocks)",
                    our_block_count.to_string().green()
                );
            }
        }
    }

    let server = match Server::new(&config, gitlab, key, blockchain, peer_list) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to create server: {}", e);
            return;
        }
    };

    server.listen().await;
}
