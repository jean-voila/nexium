import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";
import tailwindcss from "@tailwindcss/vite";
import { sveltekit } from "@sveltejs/kit/vite";
import Icons from "unplugin-icons/vite";

export default defineConfig({
    plugins: [
        tailwindcss(),
        sveltekit(),
        Icons({
            compiler: "svelte",
            autoInstall: true // Installe automatiquement les ensembles d'icônes nécessaires
        })
    ],
    resolve: {
        alias: {
            "@bindings": fileURLToPath(new URL("./src/types/bindings", import.meta.url)),
            "@services": fileURLToPath(new URL("./src/lib/services", import.meta.url)),
            "@stores": fileURLToPath(new URL("./src/lib/stores", import.meta.url)),
            "@invoke": fileURLToPath(new URL("./src/lib/invoke", import.meta.url))
        }
    }
});
