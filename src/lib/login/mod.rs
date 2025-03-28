use core::fmt;

const EPITA_DOMAIN: &str = "epita.fr";
#[derive(Debug, Clone)]
pub struct Login {
    pub first_name: String,
    pub last_name: String,
}

impl Login {
    pub fn from(uid: String) -> Option<Login> {
        let parts: Vec<&str> = uid.split('.').collect();
        if parts.len() != 2 {
            return None;
        }
        if parts[0].len() == 0 || parts[1].len() == 0 {
            return None;
        }
        return Some(Login {
            first_name: parts[0].to_string(),
            last_name: parts[1].to_string(),
        });
    }

    pub fn check_existence(&self) -> bool {
        // check if user exists on Epita's Gitlab
        return true;
    }

    pub fn email(&self) -> String {
        return format!(
            "{}.{}@{}",
            self.first_name, self.last_name, EPITA_DOMAIN
        );
    }
}

/// Implemntation of the Login to String conversion
impl fmt::Display for Login {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.first_name, self.last_name)
    }
}
