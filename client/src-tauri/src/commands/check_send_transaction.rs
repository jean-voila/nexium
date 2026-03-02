use nexium::{
    blockchain::consts::estimate_classic_transaction_fee, gitlab::GitlabClient,
    login::Login,
};

use crate::{
    config::Config,
    nexium_api::{get_balance, ClassicTransactionSent, NexiumAPIError},
};

#[tauri::command]
pub async fn check_send_transaction(
    transaction: ClassicTransactionSent,
    config: Config,
) -> Result<(), String> {
    let result = tauri::async_runtime::spawn_blocking(move || {
        let amount = match transaction.amount.parse::<f64>() {
            Ok(amount) => {
                if amount <= 0.0 {
                    return Err(NexiumAPIError::InvalidAmount.to_string());
                }
                amount
            }
            Err(_) => return Err(NexiumAPIError::InvalidAmount.to_string()),
        };

        // Parse and validate fees
        let fees = match transaction.fees.parse::<u16>() {
            Ok(n) => n,
            Err(_) => return Err(NexiumAPIError::InvalidFees.to_string()),
        };

        // Calculate the estimated fee cost based on transaction type
        let has_description = !transaction.description.is_empty();
        let fee_cost = estimate_classic_transaction_fee(fees, has_description);

        // Total cost = amount + fees
        let total_cost = amount + fee_cost;

        let available_balance =
            match get_balance(config.user_login.clone(), config.clone()) {
                Ok(balance_info) => format!(
                    "{}.{}",
                    balance_info.integer_part, balance_info.decimal_part
                )
                .parse::<f64>()
                .unwrap_or(0.0),
                Err(_) => {
                    return Err(NexiumAPIError::BalanceFetchError.to_string())
                }
            };

        if total_cost > available_balance {
            return Err(NexiumAPIError::InsufficientFunds.to_string());
        }

        // check if the receiver and the sender are not the same
        if transaction.receiver == config.user_login {
            return Err(NexiumAPIError::SenderAndReceiverSame.to_string());
        }

        let rec_login = match Login::new(transaction.receiver.clone()) {
            Ok(rec) => rec,
            Err(_) => return Err(NexiumAPIError::InvalidReceiver.to_string()),
        };

        match rec_login.get_names() {
            Ok(names) => {
                if names.first_name.chars().count() < 2
                    || names.last_name.chars().count() < 2
                {
                    return Err(NexiumAPIError::InvalidReceiver.to_string());
                }
            }
            Err(_) => return Err(NexiumAPIError::InvalidReceiver.to_string()),
        };

        let gitlab_client =
            GitlabClient::new(config.gitlab_token, config.gitlab_token_type);

        match gitlab_client.check_user_existence(&transaction.receiver) {
            Ok(exists) => {
                if !exists {
                    return Err(NexiumAPIError::ReceiverNotFound.to_string());
                }
                return Ok(());
            }
            Err(_) => return Err(NexiumAPIError::ReceiverNotFound.to_string()),
        }
    })
    .await;
    match result {
        Ok(r) => r,
        Err(_) => Err(NexiumAPIError::UnknownError.to_string()),
    }
}
