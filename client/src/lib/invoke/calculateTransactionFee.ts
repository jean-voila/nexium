import { invoke } from "@tauri-apps/api/core";
import { ResultAsync } from "neverthrow";

export function calculateTransactionFee(
    fees: number,
    has_description: boolean
): ResultAsync<string, string> {
    return ResultAsync.fromPromise(
        invoke("calculate_transaction_fee", { fees, has_description }),
        (e) => `Failed to calculate transaction fee: ${e}`
    );
}
