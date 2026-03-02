use nexium::{defaults::KEYPAIR_BIT_SIZE, rsa::KeyPair};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::config::ConfigError;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct KeyPairResult {
    pub_key: String,
    priv_key: String,
}

#[tauri::command]
pub async fn keypair_generation(
    login: String,
    password: String,
) -> Result<KeyPairResult, String> {
    // Utilise spawn_blocking pour éviter de bloquer le thread principal
    tauri::async_runtime::spawn_blocking(move || {
        let keypair = KeyPair::generate(KEYPAIR_BIT_SIZE, &login);
        let pub_key = KeyPair::pub_to_pem(&keypair);
        let priv_key = KeyPair::priv_to_pem(&keypair, &password);

        if pub_key.is_empty() || priv_key.is_empty() {
            Err(ConfigError::KeyGenerationError.to_string())
        } else {
            Ok(KeyPairResult { pub_key, priv_key })
        }
    })
    .await
    .map_err(|err| format!("Failed to generate keypair: {err}"))?
}
