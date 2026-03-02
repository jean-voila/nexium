import { invoke } from "@tauri-apps/api/core";
import { ResultAsync } from "neverthrow";
import type { Config } from "@bindings";

export function loadConfigFromFile(path_string: string): ResultAsync<Config, string> {
    return ResultAsync.fromPromise(
        invoke("load_config_from_file", { path_string }),
        (error) => `Failed to load config from file: ${error}`
    );
}
