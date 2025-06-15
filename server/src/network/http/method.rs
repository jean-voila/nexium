use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Options,
    Patch,
}

impl Method {
    pub fn from_str<T>(s: T) -> Option<Self>
    where
        T: AsRef<[u8]>,
    {
        match s.as_ref().to_ascii_uppercase().as_slice() {
            b"GET" => Some(Method::Get),
            b"HEAD" => Some(Method::Head),
            b"POST" => Some(Method::Post),
            b"PUT" => Some(Method::Put),
            b"DELETE" => Some(Method::Delete),
            b"OPTIONS" => Some(Method::Options),
            b"PATCH" => Some(Method::Patch),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Method::Get => "GET",
            Method::Head => "HEAD",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Options => "OPTIONS",
            Method::Patch => "PATCH",
        }
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.as_str())
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::Method;

    #[test]
    fn test_method_from_str() {
        assert_eq!(Method::from_str("GET"), Some(Method::Get));
        assert_eq!(Method::from_str("HEAD"), Some(Method::Head));
        assert_eq!(Method::from_str("POST"), Some(Method::Post));
        assert_eq!(Method::from_str("PUT"), Some(Method::Put));
        assert_eq!(Method::from_str("DELETE"), Some(Method::Delete));
        assert_eq!(Method::from_str("OPTIONS"), Some(Method::Options));
        assert_eq!(Method::from_str("PATCH"), Some(Method::Patch));
        assert_eq!(Method::from_str("INVALID"), None);
    }

    #[test]
    fn test_method_from_str_2() {
        assert_eq!(Method::from_str(String::from("GET")), Some(Method::Get));
        assert_eq!(Method::from_str(&String::from("GET")), Some(Method::Get));
        assert_eq!(Method::from_str("GET"), Some(Method::Get));
        assert_eq!(Method::from_str(&"GET"), Some(Method::Get));
        assert_eq!(Method::from_str(b"GET"), Some(Method::Get));
        assert_eq!(Method::from_str(b"gEt"), Some(Method::Get));
        assert_eq!(Method::from_str(b"post"), Some(Method::Post));
        assert_eq!(Method::from_str(b"posdkf"), None);
        assert_eq!(Method::from_str(b"pos"), None);
    }
}
