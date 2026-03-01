import { invoke } from "@tauri-apps/api/core";
import { ResultAsync } from "neverthrow";
import type { Invoice } from "@bindings";

export function loadInvoiceFromFile(path_string: string): ResultAsync<Invoice, string> {
    return ResultAsync.fromPromise(
        invoke("load_invoice_from_file", { path_string }),
        (error) => `Failed to load invoice: ${error}`
    );
}
