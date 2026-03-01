import { invoke } from "@tauri-apps/api/core";
import { ResultAsync } from "neverthrow";
import type { ClassicTransactionReceived, Config } from "@bindings";

export function getTransactions(
    config: Config,
    login: string,
    n: number
): ResultAsync<ClassicTransactionReceived[], string> {
    return ResultAsync.fromPromise(
        invoke("get_server_infos", { config, login, n }),
        (error) => `Failed to get server infos: ${error}`
    );
}
