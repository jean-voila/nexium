use core::fmt;

#[derive(Debug, Clone)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
}

impl User {
    pub fn from(uid: String) -> User {
        let parts: Vec<&str> = uid.split('.').collect();
        if parts.len() != 2 {
            panic!("Invalid user id");
        }
        if parts[0].len() == 0 || parts[1].len() == 0 {
            panic!("Invalid user id");
        }
        return User {
            first_name: parts[0].to_string(),
            last_name: parts[1].to_string(),
        };
    }

    pub fn check_existence(&self) -> bool {
        // check if user exists on the Gitlab
        return true;
    }
}

/// Implemntation of the Login to String conversion
impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.first_name, self.last_name)
    }
}
