import { invoke } from "@tauri-apps/api/core";
import type { Config } from "@bindings";

export async function saveConfig(config: Config): Promise<boolean> {
    return invoke("save_config", { config });
}
