use std::fmt::{Display, Formatter, Result};

pub enum Status {
    Ok,
    BadRequest,
    Unauthorized,
    NotFound,
    InternalServerError,
}

impl Status {
    pub fn code(&self) -> u16 {
        match self {
            Self::Ok => 200,
            Self::BadRequest => 400,
            Self::Unauthorized => 401,
            Self::NotFound => 404,
            Self::InternalServerError => 500,
        }
    }

    pub fn text(&self) -> &str {
        match self {
            Self::Ok => "OK",
            Self::BadRequest => "Bad Request",
            Self::Unauthorized => "Unauthorized",
            Self::NotFound => "Not Found",
            Self::InternalServerError => "Internal Server Error",
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {}", self.code(), self.text())
    }
}
