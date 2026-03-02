import { invoke } from "@tauri-apps/api/core";
import { ResultAsync } from "neverthrow";
import type { Config, UserStats } from "@bindings";

export function getUserStats(login: string, config: Config): ResultAsync<UserStats, string> {
    return ResultAsync.fromPromise(
        invoke("get_user_stats", { login, config }),
        (error) => `Failed to get user stats: ${error}`
    );
}
