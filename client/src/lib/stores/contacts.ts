import { writable } from "svelte/store";

export const showContactsModal = writable<boolean>(false);
export const selectedContact = writable(""); // TODO: verify type (string or null)
