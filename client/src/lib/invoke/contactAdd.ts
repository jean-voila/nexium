import { invoke } from "@tauri-apps/api/core";
import { ResultAsync } from "neverthrow";

export function contactAdd(
    login: string,
    nickname: string,
    notes: string,
    favorite: boolean
): ResultAsync<void, string> {
    return ResultAsync.fromPromise(
        invoke("contact_add", { login, nickname, notes, favorite }),
        (error) => `Failed to add contact: ${error}`
    );
}
