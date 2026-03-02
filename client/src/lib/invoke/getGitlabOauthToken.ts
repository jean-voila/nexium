import { invoke } from "@tauri-apps/api/core";
import { ResultAsync } from "neverthrow";

export function getGitlabOauthToken(): ResultAsync<string, string> {
    return ResultAsync.fromPromise(
        invoke("get_gitlab_oauth_token"),
        (e) => `Failed to get gitlab oauth token: ${e}`
    );
}
