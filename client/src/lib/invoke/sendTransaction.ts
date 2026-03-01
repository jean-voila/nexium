import { invoke } from "@tauri-apps/api/core";
import { ResultAsync } from "neverthrow";
import type { ClassicTransactionSent, Config } from "@bindings";

export function sendTransaction(
    server_pubkey: string,
    config: Config,
    transaction: ClassicTransactionSent
): ResultAsync<void, string> {
    return ResultAsync.fromPromise(
        invoke("send_transaction", { server_pubkey, config, transaction }),
        (error) => `Failed to send transaction: ${error}`
    );
}
