use nexium::login::Login;
use std::path::Path;

const _DEFAULT_PORT: u16 = 4242;
const _DEFAULT_DB_FILE: &str = "./blockchain.db";
const _DEFAULT_KEYS_DIR: &str = "./keys";

pub struct _Config<'a> {
    database_filepath: &'a Path,
    keys_filepath: &'a Path,
    port: u16,
    user_id: Login<'a>,
}
