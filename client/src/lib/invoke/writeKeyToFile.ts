import { invoke } from "@tauri-apps/api/core";
import { ResultAsync } from "neverthrow";

export function writeKeyToFile(path_string: string, key: string): ResultAsync<void, string> {
    return ResultAsync.fromPromise(
        invoke("write_key_to_file", { path_string, key }),
        (error) => `Failed to write key to file: ${error}`
    );
}
