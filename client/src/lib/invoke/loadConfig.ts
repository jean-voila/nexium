import { invoke } from "@tauri-apps/api/core";
import { ResultAsync } from "neverthrow";
import type { Config } from "@bindings";

export function loadConfig(): ResultAsync<Config, string> {
    return ResultAsync.fromPromise(
        invoke("load_config"),
        (error) => `Failed to load config: ${error}`
    );
}
