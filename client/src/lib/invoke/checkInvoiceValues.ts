import { invoke } from "@tauri-apps/api/core";
import { ResultAsync } from "neverthrow";
import type { Invoice } from "@bindings";

export function checkInvoiceValues(invoice: Invoice): ResultAsync<void, string> {
    return ResultAsync.fromPromise(
        invoke("check_invoice_values", { invoice }),
        (error) => `Failed to check invoice values: ${error}`
    );
}
