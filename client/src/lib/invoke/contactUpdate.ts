import { invoke } from "@tauri-apps/api/core";
import { ResultAsync } from "neverthrow";

export function contactUpdate(
    login: string,
    nickname?: string | null,
    notes?: string | null,
    favorite?: boolean | null
): ResultAsync<void, string> {
    return ResultAsync.fromPromise(
        invoke("contact_update", { login, nickname, notes, favorite }),
        (error) => `Failed to update contact: ${error}`
    );
}
