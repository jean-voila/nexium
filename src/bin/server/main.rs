mod config;
use config::Config;
use nexium::gitlab::{GitlabClient, GitlabError};
use std::{env, path::Path};

/// Default path to the Nexium home directory
const NEXIUM_HOME_DEFAULT: &str = ".nexiumlocal";
/// Default path to the configuration file, relative Nxm home
const DEFAULT_CONFIG_NAME: &str = "config.json";
/// Argument to pass to the program to generate the config file
const GEN_CONFIG_ARG: &str = "--generate-config";

fn main() {
    // Getting the arguments
    let args = env::args().collect::<Vec<String>>();

    // Constructing the config path
    let mut config_path = Path::new(&NEXIUM_HOME_DEFAULT).to_path_buf();
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
    let gitlab_client = GitlabClient::new(
        config.gitlab_api_url.clone(),
        config.gitlab_token.clone(),
    );

    // Checking if the gitlab token is valid
    match gitlab_client.check_token() {
        Ok(_) => println!("Gitlab token is valid"),
        Err(GitlabError::InvalidToken) => {
            panic!("Invalid Gitlab token");
        }
        Err(e) => {
            panic!("Error checking Gitlab token: {:?}", e);
        }
    }
    return;
}
