import { invoke } from "@tauri-apps/api/core";
import { ResultAsync } from "neverthrow";

export function contactMarkUsed(login: string): ResultAsync<void, string> {
    return ResultAsync.fromPromise(
        invoke("contact_mark_used", { login }),
        (error) => `Failed to mark contact as used: ${error}`
    );
}
