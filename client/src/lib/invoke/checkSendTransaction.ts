import { invoke } from "@tauri-apps/api/core";
import { ResultAsync } from "neverthrow";
import type { ClassicTransactionSent, Config } from "@bindings";

export function checkSendTransaction(
    transaction: ClassicTransactionSent,
    config: Config
): ResultAsync<void, string> {
    return ResultAsync.fromPromise(
        invoke("check_send_transaction", { transaction, config }),
        (error) => `Failed to check send transaction: ${error}`
    );
}
