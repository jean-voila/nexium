use crate::rsa;
use std::fmt;

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
pub struct Account {
    first_name: String,
    last_name: String,
    pub key_pair: rsa::KeyPair,
}

impl Account {
    // The new function returns an Option<Account> because it can fail.
    // (meaning that the first_name or last_name are invalid)
    pub fn new(first_name: &str, last_name: &str) -> Option<Account> {

        if !first_name.is_valid() || !last_name.is_valid() {
            return None;
        }

        return Some(Account {
            first_name: first_name.to_lowercase(),
            last_name: last_name.to_lowercase(),
            key_pair: rsa::KeyPair::new(),
        })
    }

    // The get_login function returns the Epita login of the student.
    // It concatenates the first_name and last_name with a dot.
    pub fn get_login(&self) -> String {
        // Easy way
        return format!("{}.{}", self.first_name, self.last_name);
    }

    // The get_email function returns the Epita email of the student.
    // It concatenates the login with "@epita.fr".
    pub fn get_email(&self) -> String {
        return format!("{}@epita.fr", self.get_login());
    }

}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (pbk, prk) = self.key_pair.to_strings();
        write!(f, "Account: {} {} ({})\nMail: {}\nPUB: {}\nPRIV: {}\n",
        _colored(200, 200, 100, &self.first_name),
        _colored(155, 200, 200, &self.last_name),
        _colored(100, 200, 100, &self.get_login()),
        _colored(255, 200, 255, &self.get_email()),
        _colored(255, 230, 200, &pbk),
        _colored(100, 255, 255, &prk)
    )
    }
}

// Returns a colored string. This string will be displayed in the terminal with
// print! or println!. The color is defined by the RGB values.
fn _colored(r: i32, g: i32, b: i32, text: &str) -> String {
    return format!("\x1B[38;2;{};{};{}m{}\x1B[0m", r, g, b, text);
}