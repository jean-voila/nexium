use core::fmt;
use std::fmt::Display;

pub enum Status {
    Ok,
    BadRequest,
    NotFound,
    InternalError,
}

impl Status {
    pub fn code(&self) -> u16 {
        match self {
            Self::Ok => 200,
            Self::BadRequest => 400,
            Self::NotFound => 404,
            Self::InternalError => 505,
        }
    }

    pub fn text(&self) -> &str {
        match self {
            Self::Ok => "OK",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
            Self::InternalError => "Internal Server Error",
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.code(), self.text())
    }
}
