import { writable } from 'svelte/store';

export const isConfigSet = writable(false);

export const showSendModal = writable(false);
export const showReceiveModal = writable(false);

export const userBalance = writable('0.00');

export const globalConfig = writable({
    url_server: '',
    port: 0,
    user_login: '',
    pub_key: '',
    priv_key: '',
    gitlab_token: '',
    gitlab_token_type: 'Classic',
});

