import { writable } from 'svelte/store';

export const globalUrl = writable('');
export const globalPort = writable();
export const globalLogin = writable('');
export const globalGitlabToken = writable('');
export const globalPubKey = writable('');
export const globalPrivKey = writable('');