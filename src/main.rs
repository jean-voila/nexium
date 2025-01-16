use std::env;

const USAGE: &str = "Usage: nexium <first_name> <last_name>";

mod accounts;
mod maths;
mod random;
mod rsa;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 || args.contains(&String::from("--help")) {
        println!("{}", USAGE);
        return;
    }

    let first_name = &args[1];
    let last_name = &args[2];

    let account = match accounts::Account::new(first_name, last_name) {
        Some(account) => account,
        None => {
            println!("{}", USAGE);
            return;
        }
    };

    println!("{}", account);
    return;
}
