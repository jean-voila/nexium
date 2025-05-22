use std::fmt;
pub enum LoginError {
    EmptyLogin,
    TooMuchPoints,
    NoPoint,
    InvalidCharacter,
    MissingField,
    InvalidFirstName,
    InvalidLastName,
    UnknownError,
}

// Implement the Display trait for the LoginError enum
impl fmt::Display for LoginError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            LoginError::EmptyLogin => "Le login est vide.",
            LoginError::TooMuchPoints => "Le login contient trop de points.",
            LoginError::NoPoint => "Le login ne contient pas de point.",
            LoginError::InvalidCharacter => {
                "Le login contient un/des caractère(s) invalide(s)."
            }
            LoginError::MissingField => {
                "Le login ne contient pas tous les champs requis."
            }
            LoginError::InvalidFirstName => "Le prénom est invalide.",
            LoginError::InvalidLastName => "Le nom de famille est invalide.",
            LoginError::UnknownError => "Erreur inconnue.",
        };
        write!(f, "{msg}")
    }
}

pub struct Login {
    pub login: String,
}

impl Login {
    pub fn new(login: String) -> Result<Self, LoginError> {
        // Check if the login has only ASCII characters
        if !login.is_ascii() {
            return Err(LoginError::InvalidCharacter);
        }

        // Convert the login into ASCII lowercase
        let login = login.to_ascii_lowercase();

        // Check if there's exactly one dot in the login
        match login.matches('.').count() {
            0 => return Err(LoginError::NoPoint),
            1 => {}
            _ => return Err(LoginError::TooMuchPoints),
        }

        // Check if the login is an empty string
        if login.chars().count() == 0 {
            return Err(LoginError::EmptyLogin);
        }

        // Check if the login has only letters or points
        if login
            .chars()
            .any(|c| !(c.is_alphabetic() || c == '.' || c == '-'))
        {
            return Err(LoginError::InvalidCharacter);
        }

        let parts = login.split('.');

        // Check if the firstname is valid
        match parts.clone().nth(0) {
            Some(first_part) => {
                // Check if the firstname has at least 2 characters
                if first_part.chars().count() < 2 {
                    return Err(LoginError::InvalidFirstName);
                }
                // Check if the firstname begins with a dash, end with a dash or has two dashes in a row
                if first_part.starts_with('-') || first_part.ends_with('-') {
                    return Err(LoginError::InvalidFirstName);
                }
                if first_part.contains("--") {
                    return Err(LoginError::InvalidFirstName);
                }
            }
            // Check if there's two parts in the login
            None => return Err(LoginError::MissingField),
        }

        // Check if the lastname is valid
        match parts.clone().nth(1) {
            Some(last_part) => {
                // Check if the lastname has at least 2 characters
                if last_part.chars().count() < 2 {
                    return Err(LoginError::InvalidLastName);
                }
                // Check if the lastname begins with a dash, end with a dash or has two dashes in a row
                if last_part.starts_with('-') || last_part.ends_with('-') {
                    return Err(LoginError::InvalidLastName);
                }
                if last_part.contains("--") {
                    return Err(LoginError::InvalidLastName);
                }
            }
            // Check if there's two parts in the login
            None => return Err(LoginError::MissingField),
        }

        return Ok(Self { login });
    }

    pub fn get_names(&self) -> Result<(String, String), LoginError> {
        let parts = self.login.split('.');
        let first_part = match parts.clone().nth(0) {
            Some(first_part) => first_part,
            None => return Err(LoginError::UnknownError),
        };
        let last_part = match parts.clone().nth(1) {
            Some(last_part) => last_part,
            None => return Err(LoginError::UnknownError),
        };

        Ok((
            capitalize(first_part.to_string()),
            capitalize(last_part.to_string()),
        ))
    }
}

fn capitalize(s: String) -> String {
    let mut chars = s.chars();
    let first_char = match chars.next() {
        Some(c) => c.to_uppercase().to_string(),
        None => return s,
    };
    let rest = chars.as_str().to_string();
    let capitalized = format!("{}{}", first_char, rest);
    capitalized
}
