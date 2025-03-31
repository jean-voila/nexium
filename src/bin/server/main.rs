mod blockchain;
mod config;

use config::Config;
use nexium::{gitlab::GitlabClient, rsa::KeyPair};

use std::{env, path::Path};

/// Default path to the Nexium home directory
const NEXIUM_HOME: &str = ".nexiumlocal";
/// Default path to the configuration file, relative Nxm home
const DEFAULT_CONFIG_NAME: &str = "config.json";
/// Argument to pass to the program to generate the config file
const GEN_CONFIG_ARG: &str = "--generate-config";

fn main() {
    // Getting the arguments
    let args = env::args().collect::<Vec<String>>();

    // Constructing the config path
    let mut config_path = Path::new(&NEXIUM_HOME).to_path_buf();
    config_path.push(DEFAULT_CONFIG_NAME);

    // Creating the config directory if it doesn't exist
    if !config_path.exists() {
        std::fs::create_dir_all(config_path.parent().unwrap())
            .expect("Failed to create config directory");
    }

    // If GEN_CONFIG_ARG is passed, generate the config file
    if args.len() > 1 && args[1] == GEN_CONFIG_ARG {
        Config::generate(&config_path);
    }

    // Constructing the config object
    let config = Config::from_file(&config_path);

    // Creating the gitlab API client
    let _gitlab_client = GitlabClient::new(
        config.gitlab_api_url.clone(),
        config.gitlab_token.clone(),
    );

    let keypair = KeyPair::generate(2048);

    let message = "Hello, world!".as_bytes().to_vec();
    let signature = keypair.sign(message.clone()).unwrap();
    if !keypair.check_signature(message, &signature).unwrap() {
        println!("Signature is invalid");
    } else {
        println!("Signature is valid");
    }

    return;
}
