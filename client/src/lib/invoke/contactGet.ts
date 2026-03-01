import { invoke } from "@tauri-apps/api/core";
import type { Contact } from "@bindings";

export async function contactGet(): Promise<Contact[]> {
    return invoke("contact_get");
}
