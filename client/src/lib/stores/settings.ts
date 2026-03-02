import { writable } from "svelte/store";
import type { Config } from "@bindings";
import type { Theme } from "@tauri-apps/api/window";

export const isConfigSet = writable<boolean>(false);

export const showSendModal = writable<boolean>(false);
export const showReceiveModal = writable<boolean>(false);
export const showHistoryModal = writable<boolean>(false);
export const showStatsModal = writable<boolean>(false);
export const showServerListModal = writable<boolean>(false);

export const userBalanceInt = writable(""); // TODO: verify type
export const userBalanceDec = writable(""); // TODO: verify type

export const globalErrorMessage = writable<string>(""); // TODO: verify type (string or null)

export const serverPublicKey = writable(""); // TODO: verify type

// Theme: 'dark' or 'light'
export const theme = writable<Theme>("dark");

// Initialize theme from localStorage if available
if (typeof window !== "undefined") {
    // TODO: use persisting store (@tauri-apps/plugin-store) instead of localStorage
    const savedTheme = localStorage.getItem("nexium-theme");

    if (savedTheme === "dark" || savedTheme === "light") {
        theme.set(savedTheme);
    } else {
        // Default to dark theme if no valid theme is saved
        theme.set("dark");
    }

    theme.subscribe((value) => {
        localStorage.setItem("nexium-theme", value);
        document.documentElement.setAttribute("data-theme", value);
    });
}

export const globalConfig = writable<Config>({
    // TODO: verify types (update to null if empty values to ensure type safety)
    server_address: "",
    port: "",
    user_login: "",
    pub_key: "",
    priv_key: "",
    gitlab_token: "",
    gitlab_token_type: "Classic",
    is_testnet: false,
    password: "",
    server_login: ""
});
