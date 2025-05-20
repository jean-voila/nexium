use nexium::rsa::KeyPair;

#[derive(Clone)]
pub struct User {
    pub balance: Option<u32>,
    pub keys: Vec<KeyPair>,
}

impl User {
    pub fn new() -> Self {
        Self {
            balance: None,
            keys: vec![],
        }
    }
}
