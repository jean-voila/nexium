use std::env;
const USAGE: &str = "Usage: nexium <first_name> <last_name> [crypt]";

mod rsa;
mod accounts;
mod random;
mod maths;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 || args.contains(&String::from("--help")) {
        println!("{}", USAGE);
        return;
    }
    let crypt: bool = args.len() > 3 && args[2] == "crypt";

    let first_name = &args[1];
    let last_name = &args[2];

    let mut account = match accounts::Account::new(first_name, last_name) {
        Some(account) => account,
        None => {
            println!("{}", USAGE);
            return;
        }
    };

    let _ = account.key_pair.generate_keys(crypt);

    println!("{}", account);
    return;

}


