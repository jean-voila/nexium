mod config;
use std::fs;
use std::{env, path::Path};

/// Default path to the Nexium home directory
const NEXIUM_HOME_DEFAULT: &str = "~/.nexium/";

/// Default path to the configuration file
const DEFAULT_PATH: &str = "config.json";

const GEN_CONFIG_ARG: &str = "--generate-config";

fn main() {
    let nexium_home: &str =
        &NEXIUM_HOME_DEFAULT.replace("~", env::var("HOME").unwrap().as_str());

    if !Path::new(&nexium_home).exists() {
        fs::create_dir_all(nexium_home).unwrap();
    }

    let path = format!("{}/{}", &nexium_home, DEFAULT_PATH);

    let config: config::Config = match env::args().nth(1) {
        Some(arg) => {
            if arg == GEN_CONFIG_ARG {
                let res = config::Config::generate();
                res.to_file(Path::new(&path));
                res
            } else {
                config::Config::from_file(Path::new(&path))
            }
        }
        None => config::Config::from_file(Path::new(&path)),
    };

    println!("Valid config and valid token");
}
