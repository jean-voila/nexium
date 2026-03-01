import { invoke } from "@tauri-apps/api/core";
import { ResultAsync } from "neverthrow";
import type { Config, ServerInfos } from "@bindings";

export function getServerInfos(config: Config): ResultAsync<ServerInfos, string> {
    return ResultAsync.fromPromise(
        invoke("get_server_infos", { config }),
        (error) => `Failed to get server infos: ${error}`
    );
}
