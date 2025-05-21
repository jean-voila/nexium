import { writable } from 'svelte/store';



export const globalConfig = writable({
    url_server: '',
    port: 0,
    user_login: '',
    pub_key: '',
    priv_key: '',
    gitlab_token: '',
    gitlab_token_type: 'Classic',
});

