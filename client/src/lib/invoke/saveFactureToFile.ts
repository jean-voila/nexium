import { invoke } from "@tauri-apps/api/core";
import { ResultAsync } from "neverthrow";
import type { Invoice } from "@bindings";

export function saveFactureToFile(
    invoice: Invoice,
    path_string: string
): ResultAsync<void, string> {
    return ResultAsync.fromPromise(
        invoke("save_facture_to_file", { invoice, path_string }),
        (error) => `Failed to save facture to file: ${error}`
    );
}
