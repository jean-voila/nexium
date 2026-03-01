import { invoke } from "@tauri-apps/api/core";
import { ResultAsync } from "neverthrow";

export function contactRemove(login: string): ResultAsync<void, string> {
    return ResultAsync.fromPromise(
        invoke("contact_remove", { login }),
        (error) => `Failed to remove contact: ${error}`
    );
}
