import { invoke } from "@tauri-apps/api/core";
import { ResultAsync } from "neverthrow";
import type { TokenType } from "@bindings";

export function sendGpgKey(
    gitlab_token_type: TokenType,
    gitlab_token: string,
    pub_key: string
): ResultAsync<void, string> {
    return ResultAsync.fromPromise(
        invoke("send_gpg_key", { gitlab_token_type, gitlab_token, pub_key }),
        (error) => `Failed to send GPG key: ${error}`
    );
}
