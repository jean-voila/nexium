import { invoke } from "@tauri-apps/api/core";
import { ResultAsync } from "neverthrow";
import type { Config } from "@bindings";

export function searchFirstUsers(config: Config, search: string): ResultAsync<string[], string> {
    return ResultAsync.fromPromise(
        invoke("search_first_users", { config, search }),
        (error) => `Failed to search first users: ${error}`
    );
}
