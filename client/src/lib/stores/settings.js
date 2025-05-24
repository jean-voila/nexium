import { writable } from 'svelte/store';

export const isConfigSet = writable(false);

export const showSendModal = writable(false);
export const showReceiveModal = writable(false);
export const showHistoryModal = writable(false);

export const userBalanceInt = writable('');
export const userBalanceDec = writable('');

export const globalErrorMessage = writable('');

export const serverPublicKey = writable('');

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
});

