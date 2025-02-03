pub struct Login<'a> {
    pub uid: &'a str,
}

impl Login<'_> {
    pub fn new(uid: &str) -> Login {
        Login { uid }
    }
}
