import { invoke } from "@tauri-apps/api/core";
import { ResultAsync } from "neverthrow";
import type { BalanceInfo, Config } from "@bindings";

export function getBalance(login: string, config: Config): ResultAsync<BalanceInfo, string> {
    return ResultAsync.fromPromise(
        invoke("get_balance", { login, config }),
        (error) => `Failed to get balance for login: ${error}`
    );
}
