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
};
use std::{env, path::Path};

const GEN_CONFIG_ARG: &str = "--generate-config";
const DEFAULT_CONFIG_NAME: &str = "config.json";

fn main() {
    // Getting the arguments
    let args = env::args().collect::<Vec<String>>();

    let mut config_path = Path::new(&NEXIUM_HOME).to_path_buf();
    config_path.push(DEFAULT_CONFIG_NAME);

    if !config_path.exists() {
        println!(
            "Config file {} does not exist. Creating the directory... ",
            config_path.to_str().unwrap().red().bold()
        );
        std::fs::create_dir_all(config_path.parent().unwrap())
            .expect("Failed to create config directory");
    } else {
        println!(
            "Config file {} already exists.",
            config_path.to_str().unwrap().red().bold()
        );
    }

    let config = if args.len() > 1 && args[1] == GEN_CONFIG_ARG {
        // Generate the config file
        println!(
            "Argument {} passed.\nGenerating the config in {}... ",
            args[1].bold(),
            config_path.to_str().unwrap().red().bold()
        );

        Config::generate(&config_path)
    } else {
        println!(
            "Looking for the config in {}... ",
            config_path.to_str().unwrap().red().bold()
        );
        Config::from_file(&config_path.clone())
    };

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

    let mut blockchain = match Blockchain::init() {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Failed to create blockchain: {}", e);
            return;
        }
    };

    let mut server = match Server::new(&config, &gitlab, &mut blockchain) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to create server: {}", e);
            return;
        }
    };
    server.listen();
}
