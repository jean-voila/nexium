import { getConstants as getConstantsInvoke } from "@invoke";
import type { Constants } from "@bindings";

let p: Promise<Constants> = getConstantsInvoke();

export async function getConstants(): Promise<Constants> {
    return p;
}
