<script lang="ts">
    import "../app.css";
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { initNotifications, startTransactionWatcher } from "@services/notifications.js";
    import { theme, globalConfig, isConfigSet, serverPublicKey } from "@stores/settings.js";
    import {
        getServerInfos,
        loadConfig,
        checkConfigValues,
        findWorkingServer,
        saveConfig
    } from "@invoke";

    let { children } = $props();

    onMount(async () => {
        // Try to load saved config
        try {
            const savedConfigRes = await loadConfig();
            if (savedConfigRes.isOk()) {
                // Validate the config silently
                const savedConfig = savedConfigRes.value;
                try {
                    await checkConfigValues(savedConfig);
                    const server_pub_key_login = await getServerInfos(savedConfig);

                    if (server_pub_key_login.isOk()) {
                        serverPublicKey.set(server_pub_key_login.value.pub_key);
                        savedConfig.server_login = server_pub_key_login.value.login;
                    }
                    globalConfig.set(savedConfig);
                    isConfigSet.set(true);
                } catch (e) {
                    // Main server unreachable, try failover
                    console.log("Main server unreachable, trying failover...", e);
                    const workingServerRes = await findWorkingServer(savedConfig);

                    if (workingServerRes.isOk()) {
                        const { pub_key, config: updatedConfig } = workingServerRes.value;
                        serverPublicKey.set(pub_key);
                        globalConfig.set(updatedConfig);
                        isConfigSet.set(true);
                        // Save the updated config with new server

                        if (await saveConfig(updatedConfig)) {
                            console.log(
                                `Failover successful: switched to ${updatedConfig.server_address}:${updatedConfig.port}`
                            );
                        } else {
                            console.log("Failed to save config after failover");
                        }
                    } else {
                        console.log(
                            "No working servers found during failover:",
                            workingServerRes.error
                        );
                    }
                }
            }
        } catch (e) {
            console.log("No saved config found");
        }

        // Initialize notifications
        await initNotifications();
        startTransactionWatcher();

        // Apply theme on load
        const savedTheme = localStorage.getItem("nexium-theme") || "dark";
        document.documentElement.setAttribute("data-theme", savedTheme);
    });

    // React to theme changes
    $effect(() => {
        document.documentElement.setAttribute("data-theme", $theme);
    });
</script>

{@render children()}
