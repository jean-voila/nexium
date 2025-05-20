mod config;

use chrono::Utc;
use colored::Colorize;
use config::Config;
use nexium::defaults::*;
use nexium::gitlab::*;
use nexium::rsa::KeyPair;
use nexium::sha256::sha256;
use std::io::{self, Write};
use std::{env, path::Path, thread, time};

const GEN_CONFIG_ARG: &str = "--generate-config";
const DEFAULT_CONFIG_NAME: &str = "config.json";

fn main() {
    let wait_time = time::Duration::from_millis(0);
    // Getting the arguments
    let args = env::args().collect::<Vec<String>>();

    let mut config_path = Path::new(&NEXIUM_HOME).to_path_buf();
    config_path.push(DEFAULT_CONFIG_NAME);

    thread::sleep(wait_time);
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
    thread::sleep(wait_time);

    let config = if args.len() > 1 && args[1] == GEN_CONFIG_ARG {
        // Generate the config file
        print!(
            "Argument {} passed.\nGenerating the config in {}... ",
            args[1].bold(),
            config_path.to_str().unwrap().red().bold()
        );

        Config::generate(&config_path)
    } else {
        print!(
            "Looking for the config in {}... ",
            config_path.to_str().unwrap().red().bold()
        );
        Config::from_file(&config_path.clone())
    };
    io::stdout().flush().unwrap();
    thread::sleep(wait_time);
    println!("{}", "CONFIG READY".green().bold());
    thread::sleep(wait_time);

    print!("Generating a {}-bits RSA keypair... ", "2048".bold());
    std::io::stdout().flush().unwrap();
    let keypair = KeyPair::generate(2048, &config.user_login);
    println!("{}", "DONE".green().bold());

    print!("Getting this new key's public PEM... ");
    thread::sleep(wait_time);
    std::io::stdout().flush().unwrap();
    let public_pem = keypair.pub_to_pem();
    println!("{}", to_truncated_hash(&public_pem).yellow().bold());
    thread::sleep(wait_time);

    let gitlab_client =
        GitlabClient::new(config.gitlab_token.clone(), TokenType::Classic);
    println!("Created the {}", "Gitlab client".bold());
    thread::sleep(wait_time);
    print_token(&config.gitlab_token);
    print!("Checking if the token is valid... ");
    thread::sleep(wait_time);
    std::io::stdout().flush().unwrap();
    match gitlab_client.check_token() {
        Ok(valid) => {
            if !valid {
                println!("{}", "INVALID".red().bold());
                panic!();
            } else {
                println!("{}", "VALID".green().bold());
            }
        }
        Err(e) => {
            println!("{}", "ERROR".yellow().bold());
            println!("Error: {:?}", e);
            panic!();
        }
    };

    thread::sleep(wait_time);
    print!(
        "Getting already existing keys for {}... ",
        config.user_login.yellow().bold()
    );
    std::io::stdout().flush().unwrap();
    let user_keys = gitlab_client
        .get_gpg_keys(&config.user_login.clone())
        .unwrap();
    thread::sleep(wait_time);
    println!("Found {} keys", user_keys.len().to_string().blue().bold());

    thread::sleep(wait_time);
    print!("Adding the new PEM key to the user... ");
    std::io::stdout().flush().unwrap();
    match gitlab_client.add_gpg_key(&public_pem) {
        Ok(_) => {
            println!("{}", "SUCCESS".green().bold());
        }
        Err(e) => {
            println!("{}", "ERROR".red().bold());
            println!("Error: {:?}", e);
            panic!();
        }
    }
    thread::sleep(wait_time);

    println!(
        "Creating a sample transaction of {} NXM to {} with {} µNXM/B fee... ",
        "15".red().bold(),
        "milo.delbos".blue().bold(),
        "40".green().bold()
    );
    thread::sleep(wait_time);
    let timestamp = Utc::now();

    let transaction = format!(
        "{}{}{}{}{}{}{}{}",
        "142".green(),
        timestamp.timestamp().to_string().yellow(),
        "40".green(),
        config.user_login.yellow(),
        "1",
        "milo.delbos".blue(),
        "15".red(),
        "0".green()
    );
    println!("Transaction: {}", transaction);
    thread::sleep(wait_time);
    let transaction_hash: String = to_truncated_hash(&transaction);
    println!("Transaction hash: {}", transaction_hash.yellow().bold());
    thread::sleep(wait_time);
    print!("Signing the transaction... ");
    thread::sleep(wait_time);
    std::io::stdout().flush().unwrap();
    let signature = match keypair.sign(transaction.as_bytes().to_vec()) {
        Ok(signature) => {
            println!(
                "{}",
                to_truncated_hash(&signature.to_string()).green().bold()
            );
            signature
        }
        Err(e) => {
            println!("{}", "ERROR".red().bold());
            println!("Error: {:?}", e);
            panic!();
        }
    };

    println!("{}", print_formatted(&signature.to_string()));

    print!(
        "Press {} to open the Gitlab login page",
        "ENTER".red().bold()
    );
    io::stdout().flush().unwrap();
    // Waiting for the user to press enter
    io::stdin()
        .read_line(&mut String::new())
        .expect("Failed to read line");

    match GitlabClient::get_token() {
        Ok(token) => {
            println!("{}{}", "Token: ".bold(), token);
            print_token(&token);
        }
        Err(e) => {
            println!("{}", "ERROR".red().bold());
            println!("Error: {:?}", e);
            panic!();
        }
    }
}

fn print_token(token: &str) {
    // Prints token in bold with stars in the middle
    println!(
        "Gitlab Token: {}{}{}",
        token.chars().take(5).collect::<String>().purple().bold(),
        "*".repeat(token.len() - 10).purple(),
        token
            .chars()
            .rev()
            .take(5)
            .collect::<String>()
            .purple()
            .bold()
    );
}

fn to_truncated_hash(src: &str) -> String {
    let hashed = sha256(src.as_bytes().to_vec())
        .to_vec()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();

    let mut compressed = String::new();
    for i in 0..15 {
        compressed.push(hashed.chars().nth(i).unwrap());
    }
    compressed.push_str("..");
    return compressed;
}

fn print_formatted(signature: &str) -> String {
    // Get the hash of the signature
    let hash = sha256(signature.as_bytes().to_vec())
        .to_vec()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();

    return hash;
}
