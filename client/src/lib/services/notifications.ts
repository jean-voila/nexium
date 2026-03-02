import { writable, get } from "svelte/store";
import {
    isPermissionGranted,
    requestPermission,
    sendNotification
} from "@tauri-apps/plugin-notification";
import { globalConfig, isConfigSet } from "@stores/settings.js";
import { getTransactions } from "@invoke";
import type { ClassicTransactionReceived } from "@bindings";

// Store for notification settings
export const notificationsEnabled = writable(true);
export const lastKnownTransactionCount = writable(0);

let checkInterval: ReturnType<typeof setInterval> | null = null;

export async function initNotifications(): Promise<boolean> {
    // Check/request permission
    let permissionGranted = await isPermissionGranted();
    if (!permissionGranted) {
        const permission = await requestPermission();
        permissionGranted = permission === "granted";
    }
    return permissionGranted;
}

export async function checkForNewTransactions(): Promise<void> {
    const config = get(globalConfig);
    const configSet = get(isConfigSet);
    const enabled = get(notificationsEnabled);

    if (!configSet || !enabled || !config.user_login) {
        return;
    }

    const transactionsRes = await getTransactions(config, config.user_login, 5);

    if (transactionsRes.isOk()) {
        const transactions = transactionsRes.value;

        if (transactions.length > 0) {
            const lastCount = get(lastKnownTransactionCount);

            // If this is the first check, just store the count
            if (lastCount === 0) {
                lastKnownTransactionCount.set(transactions.length);
                return;
            }

            // Check for new incoming transactions
            const newIncoming = transactions.filter((t) => t.inorout === "received");

            if (transactions.length > lastCount && newIncoming.length > 0) {
                const latestIncoming = newIncoming[0];
                await sendTransactionNotification(latestIncoming);
            }

            lastKnownTransactionCount.set(transactions.length);
        }
    } else {
        console.error("Error fetching transactions:", transactionsRes.error);
    }
}

async function sendTransactionNotification(transaction: ClassicTransactionReceived): Promise<void> {
    const permissionGranted = await isPermissionGranted();
    if (!permissionGranted) return;

    const amount = transaction.amount || "0";
    const sender = transaction.emitter || "Inconnu";

    sendNotification({
        title: "Nouvelle transaction reçue!",
        body: `Vous avez reçu ${amount} NXM de ${sender}`,
        icon: "icons/icon.png"
    });
}

export function startTransactionWatcher(): void {
    if (checkInterval != null) return;

    // Check every 30 seconds
    checkInterval = setInterval(checkForNewTransactions, 30_000);

    // Also check immediately
    checkForNewTransactions();
}

export function stopTransactionWatcher(): void {
    if (checkInterval != null) {
        clearInterval(checkInterval);
        checkInterval = null;
    }
}

// Subscribe to config changes
isConfigSet.subscribe((isSet) => {
    if (isSet) {
        startTransactionWatcher();
    } else {
        stopTransactionWatcher();
    }
});
