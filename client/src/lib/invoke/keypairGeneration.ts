import { invoke } from "@tauri-apps/api/core";
import { ResultAsync } from "neverthrow";
import type { KeyPairResult } from "@bindings";

export function keypairGeneration(
    login: string,
    password: string
): ResultAsync<KeyPairResult, string> {
    return ResultAsync.fromPromise(
        invoke("keypair_generation", { login, password }),
        (error) => `Failed to generate keypair: ${error}`
    );
}
