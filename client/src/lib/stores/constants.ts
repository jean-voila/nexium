import { getConstants } from "@invoke";
import type { Constants } from "@bindings";

export const constants: Constants = await getConstants();
