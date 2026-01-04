import { writable } from 'svelte/store';

export const showContactsModal = writable(false);
export const selectedContact = writable('');
