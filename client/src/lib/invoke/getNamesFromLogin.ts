import { invoke } from "@tauri-apps/api/core";
import { ResultAsync } from "neverthrow";
import type { LoginNames } from "@bindings";

export function getNamesFromLogin(login: string): ResultAsync<LoginNames, string> {
    return ResultAsync.fromPromise(
        invoke("get_names_from_login", { login }),
        (error) => `Failed to get names from login: ${error}`
    );
}
