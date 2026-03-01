use nexium::blockchain::consts::estimate_classic_transaction_fee;

/// Calculate the estimated fee cost for a transaction
/// Returns the fee cost in NEX as a formatted string
#[tauri::command]
async fn calculate_transaction_fee(fees: u16, has_description: bool) -> String {
    let fee_cost = estimate_classic_transaction_fee(fees, has_description);
    format!("{:.6}", fee_cost)
}
