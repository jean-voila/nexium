mod blockchain;
mod config;
mod network;

use blockchain::blockchain::Blockchain;
use colored::Colorize;
use config::Config;
use network::server::Server;
use nexium::{
    defaults::*,
    gitlab::{GitlabClient, TokenType},
    rsa::KeyPair,
};
use std::{env, fs, path::Path};

const HELP_ARG: &str = "--help";
const GEN_CONFIG_ARG: &str = "--generate-config";
const GEN_KEY_ARG: &str = "--generate-key";
const DEFAULT_CONFIG_NAME: &str = "config.json";

fn main() {
    // Getting the arguments
    let args = env::args().collect::<Vec<String>>();

    let local_path = Path::new(&NEXIUM_HOME);
    let config_path = local_path.join(DEFAULT_CONFIG_NAME);

    if !local_path.exists() {
        print!(
            "Creating the {} directory...",
            local_path.to_str().unwrap().cyan().bold()
        );

        match fs::create_dir_all(&local_path) {
            Ok(_) => println!(
                "\rCreating the {} directory: {}",
                local_path.to_str().unwrap().cyan().bold(),
                "OK".green()
            ),
            Err(e) => {
                println!();
                eprintln!(
                    "Failed to create directory: {}",
                    e.to_string().red()
                );
                return;
            }
        }
    } else {
        println!(
            "Using the {} directory",
            local_path.to_str().unwrap().cyan().bold()
        );
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
                    config_path.to_str().unwrap().cyan().bold()
                );

                Config::generate(&config_path);
                println!(
                    "\nConfig file generated at {}",
                    config_path.to_str().unwrap().cyan().bold()
                );
                return;
            }
            GEN_KEY_ARG => {
                // Generate a new key
                if !config_path.exists() {
                    eprintln!(
                        "Config file not found at {}. Please generate it with {}",
                        config_path.to_str().unwrap().cyan(),
                        GEN_CONFIG_ARG.yellow().bold()
                    );
                    return;
                }

                let key_path = local_path.join(DEFAULT_KEY_PATH);
                let config = Config::from_file(&config_path);
                let gitlab = GitlabClient::new(
                    config.gitlab_token.clone(),
                    TokenType::Classic,
                );

                match gitlab.check_token() {
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
                    key_path.to_str().unwrap().cyan().bold()
                );

                let key =
                    KeyPair::generate(KEYPAIR_BIT_SIZE, &config.user_login);

                let priv_pem = key.priv_to_pem(&config.key_password);
                match fs::write(&key_path, priv_pem) {
                    Ok(_) => {
                        println!(
                            "Private key saved to {}",
                            key_path.to_str().unwrap().cyan().bold()
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
                match gitlab.add_gpg_key(&pub_pem) {
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
        eprintln!(
            "Config file not found at {}. Please generate it with {}",
            config_path.to_str().unwrap().cyan(),
            GEN_CONFIG_ARG.yellow().bold()
        );
        return;
    }

    println!(
        "Using the config file at {}",
        local_path.to_str().unwrap().cyan().bold()
    );

    let config = Config::from_file(&config_path);

    let gitlab =
        GitlabClient::new(config.gitlab_token.clone(), TokenType::Classic);

    print!("Checking Gitlab token...");
    match gitlab.check_token() {
        Ok(_) => {
            println!("\rChecking Gitlab token: {}", "OK".green());
        }
        Err(e) => {
            println!();
            eprintln!("Failed to check Gitlab token: {}", e.to_string().red());
            return;
        }
    }

    let key_path = local_path
        .join(&config.key_filepath)
        .to_str()
        .unwrap()
        .to_string();
    print!("Loading private key...");
    let key = match KeyPair::priv_from_file(
        &key_path,
        &config.user_login,
        &config.key_password,
    ) {
        Ok(k) => {
            println!("\rLoading private key: {: >4}", "OK".green());
            k
        }
        Err(e) => {
            eprintln!("Failed to load private key: {}", e.to_string().red());
            return;
        }
    };

    print!("Reading blockchain...");
    let mut blockchain = match Blockchain::init() {
        Ok(b) => {
            println!("\rReading blockchain: {: >5}", "OK".green());
            b
        }
        Err(e) => {
            println!();
            eprintln!("Failed to create blockchain: {}", e);
            return;
        }
    };

    let mut server = match Server::new(&config, &gitlab, &key, &mut blockchain)
    {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to create server: {}", e);
            return;
        }
    };
    server.listen();
}
