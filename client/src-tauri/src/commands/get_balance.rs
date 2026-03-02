use crate::config::Config;
use crate::nexium_api::get_balance as get_balance_api;
use crate::types::balance::BalanceInfo;

#[tauri::command]
pub async fn get_balance(
    login: String,
    config: Config,
) -> Result<BalanceInfo, String> {
    tauri::async_runtime::spawn_blocking(move || get_balance_api(login, config))
        .await
        .map_err(|e| format!("Failed to get balance: {}", e))?
}
