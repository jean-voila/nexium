use nexium::rsa::KeyPair;

#[derive(Clone)]
pub struct User {
    #[allow(unused)]
    pub balance: Option<f32>,
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
