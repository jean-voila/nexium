import { invoke } from "@tauri-apps/api/core";
import { ResultAsync } from "neverthrow";

export function checkPeerStatus(address: string, port: number): ResultAsync<boolean, string> {
    return ResultAsync.fromPromise(
        invoke("check_peer_status", { address, port }),
        (error) => `Failed to check peer status: ${error}`
    );
}
