import { invoke } from "@tauri-apps/api/core";
import type { Contact } from "@bindings";

export async function contactGet(favorite: boolean = false): Promise<Contact[]> {
    return invoke("contact_get", { favorite });
}
