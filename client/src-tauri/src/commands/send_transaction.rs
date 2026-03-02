use crate::{
    config::Config, nexium_api::send_transaction as send_transaction_api,
    nexium_api::ClassicTransactionSent,
};

#[tauri::command]
pub async fn send_transaction(
    server_pubkey: String,
    config: Config,
    transaction: ClassicTransactionSent,
) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        send_transaction_api(server_pubkey, transaction, config)
    })
    .await
    .map_err(|err| format!("Failed to send transaction: {}", err))?
}
