pub const HASH_SIZE: usize = 32;
pub const DOUBLE_HASH_SIZE: usize = HASH_SIZE * 2;
pub type Hash = [u8; HASH_SIZE];
pub type DoubleHash = [u8; DOUBLE_HASH_SIZE];
