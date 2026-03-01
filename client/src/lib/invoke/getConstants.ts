import { invoke } from "@tauri-apps/api/core";
import type { Constants } from "@bindings";

export async function getConstants(): Promise<Constants> {
    return invoke("get_constants");
}
