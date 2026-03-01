import { invoke } from "@tauri-apps/api/core";
import type { Contact } from "@bindings";

export async function contactSearch(query: string): Promise<Contact[]> {
    return invoke("contact_search", { query });
}
