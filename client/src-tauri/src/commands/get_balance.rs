use crate::nexium_api::get_balance as get_balance_api;
use crate::types::balance::BalanceInfo;
use crate::Config;

#[tauri::command]
pub async fn get_balance(
    login: String,
    config: Config,
) -> Result<BalanceInfo, String> {
    tauri::async_runtime::spawn_blocking(move || {
        match get_balance_api(login, config) {
            Ok((int, dec)) => Ok(BalanceInfo {
                integer_part: int,
                decimal_part: dec,
            }),
            Err(e) => Err(format!("Failed to get balance: {}", e)),
        }
    })
    .await
    .map_err(|e| format!("Failed to get balance: {}", e))?
}
