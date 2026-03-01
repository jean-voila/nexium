// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
declare global {
    namespace App {
        // interface Error {}
        // interface Locals {}
        // interface PageData {}
        // interface PageState {}
        // interface Platform {}
    }
}

export {};
import "unplugin-icons/types/svelte";

/// <reference types="svelte" />
/// <reference types="vite/client" />

declare module "*.svelte" {
    import { SvelteComponentTyped } from "svelte";
    export default class Component extends SvelteComponentTyped<any, any, any> {}
}
