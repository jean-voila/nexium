import { invoke } from "@tauri-apps/api/core";
import { ResultAsync } from "neverthrow";
import type { Config, WorkingServerInfo } from "@bindings";

export function findWorkingServer(config: Config): ResultAsync<WorkingServerInfo, string> {
    return ResultAsync.fromPromise(
        invoke("find_working_server", { config }),
        (error) => `Failed to get working server: ${error}`
    );
}
