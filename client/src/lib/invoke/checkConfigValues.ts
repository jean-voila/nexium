import { invoke } from "@tauri-apps/api/core";
import type { Config } from "@bindings";

export async function checkConfigValues(config: Config): Promise<boolean> {
    return invoke("check_config_values", { config });
}
