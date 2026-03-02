import { invoke } from "@tauri-apps/api/core";
import { ResultAsync } from "neverthrow";
import type { Config, TryConnectResult } from "@bindings";

export function tryConnectToServer(
    config: Config,
    address: string,
    port: number
): ResultAsync<TryConnectResult, string> {
    return ResultAsync.fromPromise(
        invoke("try_connect_to_server", { config, address, port }),
        (e) => `Failed to connect to server: ${e}`
    );
}
