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

const PEM_TEST: &str = "-----BEGIN PGP PUBLIC KEY BLOCK-----

mFIEAAAAABMIKoZIzj0DAQcCAwTyB6wXKWxB8hF7FGX2uzWoggPMaYQ8ofVQQDD1
vCaqZKSZSn7P/wjUkK3wa+CAbdOCqQKHfcq7tDaPatJpakmTtAZwbDQyNTmIgAQT
EwgAHAUCAAAAAAILCQIbAwQVCAkKBBYCAwECF4ACHgEAFgkQHbINDYzAnWELGlRS
RVpPUi1HUEfbyQD8DzUNnulhS9tTBFsLJPNhZ0VmAjFCVWLSEyTQyojihvAA/A8c
Kg57IGqU4VZbKCq8OyK0zGAN1fjj7cmgCf0avrPItBtwbDQyNTkgPGNvbnRhY3RA
cGw0MjU5LmNvbT6IkwQTEwgAOxYhBDfN57ByaqLxM245bB2yDQ2MwJ1hBQJnwgI1
AhsDBQsJCAcCAiICBhUKCQgLAgQWAgMBAh4HAheAAAoJEB2yDQ2MwJ1hXtgA/1H5
bqXDBAhSW7hD8q5U356DaAbmml2Gvpgwivx48A08AP41PcQs6AqVLY9QWd8l7A3T
kigQ6/hMJQw3dQ2coviBvrhWBAAAAAASCCqGSM49AwEHAgMEQYK3EdptK30tZ2sT
7Ha9eJK36KSxWhFawl/k5fcieJA3XrlR7nhTsiT7CwRluTPD66s2hY/nmee3OT79
pih44wMBCAeIbQQYEwgACQUCAAAAAAIbDAAWCRAdsg0NjMCdYQsaVFJFWk9SLUdQ
R0jfAQCaT0UgLXlXBozk9nAmxYJ9uO9R7k8AsdfMg8aKrWRpLgD/XC34cAMDD7E1
2I9X1iEDbta+uo9UFkQkt5YsUllHvZw=
=NC2y
-----END PGP PUBLIC KEY BLOCK-----
";

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
    let gitlab_client = GitlabClient::new(
        config.gitlab_api_url.clone(),
        config.gitlab_token.clone(),
    );

    let keypair = KeyPair::generate(2048);

    let pub_pem = keypair.pub_to_pem(&config.user_login);

    match gitlab_client.add_gpg_key(&pub_pem) {
        Ok(_) => println!("GPG key added successfully"),
        Err(e) => println!("Failed to add GPG key: {:?}", e),
    }

    println!("Public key:\n{}", pub_pem);

    return;
}
