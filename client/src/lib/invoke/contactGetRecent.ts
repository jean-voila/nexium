import { invoke } from "@tauri-apps/api/core";
import { ResultAsync } from "neverthrow";
import type { Contact } from "@bindings";

export function contactGetRecent(limit: number): ResultAsync<Contact[], string> {
    return ResultAsync.fromPromise(
        invoke("contact_get_recent", { limit }),
        (e) => `Failed to get recent contacts: ${e}`
    );
}
