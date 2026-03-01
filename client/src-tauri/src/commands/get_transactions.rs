use crate::config::Config;
use crate::nexium_api::{
    get_transactions as get_transactions_api, NexiumAPIError,
};
use crate::types::classic_tr_received::ClassicTransactionReceived;

#[tauri::command]
pub async fn get_transactions(
    config: Config,
    login: String,
    n: u32,
) -> Result<Vec<ClassicTransactionReceived>, String> {
    tauri::async_runtime::spawn_blocking(move || {
        get_transactions_api(config, login, n)
    })
    .await
    .map_err(|_| NexiumAPIError::UnknownError.to_string())?
}
