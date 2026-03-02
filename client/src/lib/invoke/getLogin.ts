import { invoke } from "@tauri-apps/api/core";
import { ResultAsync } from "neverthrow";
import type { TokenType } from "@bindings";

export function getLogin(
    gitlab_token_type: TokenType,
    gitlab_token: string
): ResultAsync<string, string> {
    return ResultAsync.fromPromise(
        invoke("get_login", { gitlab_token_type, gitlab_token }),
        (error) => `Failed to get login: ${error}`
    );
}
