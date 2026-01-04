import { writable, get } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/plugin-notification';
import { globalConfig, isConfigSet } from '$lib/stores/settings.js';

// Store for notification settings
export const notificationsEnabled = writable(true);
export const lastKnownTransactionCount = writable(0);

let checkInterval: ReturnType<typeof setInterval> | null = null;

export async function initNotifications() {
    // Check/request permission
    let permissionGranted = await isPermissionGranted();
    if (!permissionGranted) {
        const permission = await requestPermission();
        permissionGranted = permission === 'granted';
    }
    return permissionGranted;
}

export async function checkForNewTransactions() {
    const config = get(globalConfig);
    const configSet = get(isConfigSet);
    const enabled = get(notificationsEnabled);

    if (!configSet || !enabled || !config.user_login) {
        return;
    }

    try {
        const transactions = await invoke('get_transactions', {
            config: config,
            login: config.user_login,
            n: '5'
        });

        if (Array.isArray(transactions) && transactions.length > 0) {
            const lastCount = get(lastKnownTransactionCount);
            
            // If this is the first check, just store the count
            if (lastCount === 0) {
                lastKnownTransactionCount.set(transactions.length);
                return;
            }

            // Check for new incoming transactions
            const newIncoming = transactions.filter(
                (t: any) => t.inorout === 'IN'
            );

            if (transactions.length > lastCount && newIncoming.length > 0) {
                const latestIncoming = newIncoming[0];
                await sendTransactionNotification(latestIncoming);
            }

            lastKnownTransactionCount.set(transactions.length);
        }
    } catch (e) {
        console.error('Error checking for new transactions:', e);
    }
}

async function sendTransactionNotification(transaction: any) {
    const permissionGranted = await isPermissionGranted();
    if (!permissionGranted) return;

    const amount = transaction.amount || '0';
    const sender = transaction.emitter || 'Inconnu';

    sendNotification({
        title: 'Nouvelle transaction reçue!',
        body: `Vous avez reçu ${amount} NXM de ${sender}`,
        icon: 'icons/icon.png'
    });
}

export function startTransactionWatcher() {
    if (checkInterval) return;
    
    // Check every 30 seconds
    checkInterval = setInterval(checkForNewTransactions, 30000);
    
    // Also check immediately
    checkForNewTransactions();
}

export function stopTransactionWatcher() {
    if (checkInterval) {
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
