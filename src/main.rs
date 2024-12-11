use std::env;

// This trait is used to check if a string is valid.
// It contains a single method is_valid that returns a boolean.
trait NameChecker {
    fn is_valid(&self) -> bool;
}

impl NameChecker for str {

    // The method checks if the name is valid.
    // A name is valid if:
    // - It is not empty.
    // - It only contains alphabetic characters and dashes.
    // - It does not begin or end with a dash.
    fn is_valid(&self) -> bool {

        // Is the string empty?
        if self.is_empty() {
            return false;
        }
        // Does the string contain invalid characters?
        for letter in self.chars() {
            if !letter.is_alphabetic() && letter != '-' {
                return false;
            }
        }

        // We don't want the name to begin or end with a middle dash.
        if 
            self.chars().nth(0).unwrap() == '-' ||
            self.chars().last().unwrap() == '-' {
                return false;
        }
        
        return true;
    }

}

// The Account struct is used to store informations on an Epita student.
// first_name: The first name of the student.
// last_name: The last name of the student.
struct Account {
    first_name: String,
    last_name: String,
}

impl Account {
    // The new function returns an Option<Account> because it can fail.
    // (meaning that the first_name or last_name are invalid)
    fn new(first_name: &str, last_name: &str) -> Option<Account> {

        if !first_name.is_valid() || !last_name.is_valid() {
            return None;
        }

        return Some(Account {
            first_name: first_name.to_lowercase(),
            last_name: last_name.to_lowercase(),
        })
    }

    // The get_login function returns the Epita login of the student.
    // It concatenates the first_name and last_name with a dot.
    fn get_login(&self) -> String {
        // Easy way
        return format!("{}.{}", self.first_name, self.last_name);
    }

    // The get_email function returns the Epita email of the student.
    // It concatenates the login with "@epita.fr".
    fn get_email(&self) -> String {
        return format!("{}@epita.fr", self.get_login());
    }

}


fn main() {
    // We first transform the arguments into a vector of strings.
    let args: Vec<String> = env::args().collect();

    // We check if the number of arguments is correct.
    if args.len() != 3 {
        println!("Usage: {} <first_name> <last_name>", args[0]);
        return;
    }

    // We create a new Account.
    let account = Account::new(
        &args[1],
        &args[2]
    );

    // We check if the Account was created successfully.
    match account {
        Some(account) => {
            println!("Your Epita login is: {}", account.get_login());
            println!("Your Epita email is: {}", account.get_email());
        },
        None => {
            println!("Invalid first_name or last_name.");
        }
    }

    return;

}
