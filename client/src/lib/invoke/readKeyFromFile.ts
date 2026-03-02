import { invoke } from "@tauri-apps/api/core";
import { ResultAsync } from "neverthrow";

export function readKeyFromFile(path_string: string): ResultAsync<string, string> {
    return ResultAsync.fromPromise(
        invoke("read_key_from_file", { path_string }),
        (error) => `Failed to read key from file: ${error}`
    );
}
