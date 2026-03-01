import { invoke } from "@tauri-apps/api/core";
import { ResultAsync } from "neverthrow";
import type { ClassicTransactionReceived, Config } from "@bindings";

export function getTransactions(
    config: Config,
    login: string,
    n: number
): ResultAsync<ClassicTransactionReceived[], string> {
    return ResultAsync.fromPromise(
        invoke("get_transactions", { config, login, n }),
        (error) => `Failed to get transactions: ${error}`
    );
}
