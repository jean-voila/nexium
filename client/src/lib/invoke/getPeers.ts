import { invoke } from "@tauri-apps/api/core";
import { ResultAsync } from "neverthrow";
import type { Config, PeerInfo } from "@bindings";

export function getPeers(config: Config): ResultAsync<PeerInfo[], string> {
    return ResultAsync.fromPromise(
        invoke("get_peers", { config }),
        (e) => `Failed to get peers: ${e}`
    );
}
