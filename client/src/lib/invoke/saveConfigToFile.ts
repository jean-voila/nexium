import { invoke } from "@tauri-apps/api/core";
import { ResultAsync } from "neverthrow";
import type { Config } from "@bindings";

export function saveConfigToFile(config: Config, path_string: string): ResultAsync<void, string> {
    return ResultAsync.fromPromise(
        invoke("save_config_to_file", { config, path_string }),
        (error) => `Failed to save config to file: ${error}`
    );
}
