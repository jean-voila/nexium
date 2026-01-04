import { writable } from 'svelte/store';

export const isConfigSet = writable(false);

export const showSendModal = writable(false);
export const showReceiveModal = writable(false);
export const showHistoryModal = writable(false);
export const showStatsModal = writable(false);
export const showServerListModal = writable(false);

export const userBalanceInt = writable('');
export const userBalanceDec = writable('');

export const globalErrorMessage = writable('');

export const serverPublicKey = writable('');

// Theme: 'dark' or 'light'
export const theme = writable('dark');

// Initialize theme from localStorage if available
if (typeof window !== 'undefined') {
    const savedTheme = localStorage.getItem('nexium-theme');
    if (savedTheme) {
        theme.set(savedTheme);
    }
    theme.subscribe((value) => {
        localStorage.setItem('nexium-theme', value);
        document.documentElement.setAttribute('data-theme', value);
    });
}

export const globalConfig = writable({
    server_address: '',
    port: '',
    user_login: '',
    pub_key: '',
    priv_key: '',
    gitlab_token: '',
    gitlab_token_type: 'Classic',
    is_testnet: false,
    password: '',
    server_login: '',
});

