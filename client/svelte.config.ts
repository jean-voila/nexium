import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";
import adapter from "@sveltejs/adapter-static";
import type { Config } from "@sveltejs/kit";

/** @type {import('@sveltejs/kit').Config} */
const config: Config = {
    // Consult https://svelte.dev/docs/kit/integrations
    // for more information about preprocessors
    preprocess: vitePreprocess(),

    kit: {
        // adapter-auto only supports some environments, see https://svelte.dev/docs/kit/adapter-auto for a list.
        // If your environment is not supported, or you settled on a specific environment, switch out the adapter.
        // See https://svelte.dev/docs/kit/adapters for more information about adapters.
        adapter: adapter({
            pages: "dist",
            assets: "dist",
            precompress: false,
            strict: true,
            fallback: "index.html"
        }),
        alias: {
            "@bindings": "./src/types/bindings",
            "@components": "./src/lib/components",
            "@modals": "./src/lib/components/modals",
            "@invoke": "./src/lib/invoke",
            "@services": "./src/lib/services",
            "@stores": "./src/lib/stores",
            "@consts": "./src/lib/consts"
        }
    }
};

export default config;
