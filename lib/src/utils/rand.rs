use rand::{distr::Alphanumeric, Rng};

pub fn create_noise() -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
}
