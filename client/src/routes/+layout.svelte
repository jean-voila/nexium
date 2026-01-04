<script lang="ts">
	import '../app.css';
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { initNotifications, startTransactionWatcher } from '$lib/services/notifications';
	import { theme, globalConfig, isConfigSet, serverPublicKey } from '$lib/stores/settings.js';

	let { children } = $props();

	onMount(async () => {
		// Try to load saved config
		try {
			const savedConfig = await invoke('load_config');
			if (savedConfig) {
				// Validate the config silently
				try {
					await invoke('check_config_values', { config: savedConfig });
					const server_pub_key_login = await invoke('get_server_infos', { config: savedConfig });
					if (server_pub_key_login) {
						serverPublicKey.set(server_pub_key_login[0]);
						savedConfig.server_login = server_pub_key_login[1];
					}
					globalConfig.set(savedConfig);
					isConfigSet.set(true);
				} catch (e) {
					// Main server unreachable, try failover
					console.log('Main server unreachable, trying failover...', e);
					try {
						const result: [string, string, any] = await invoke('find_working_server', { config: savedConfig });
						if (result) {
							const [pubKey, login, updatedConfig] = result;
							serverPublicKey.set(pubKey);
							globalConfig.set(updatedConfig);
							isConfigSet.set(true);
							// Save the updated config with new server
							await invoke('save_config', { config: updatedConfig });
							console.log(`Failover successful: switched to ${updatedConfig.server_address}:${updatedConfig.port}`);
						}
					} catch (failoverError) {
						console.log('Failover failed, no available servers:', failoverError);
					}
				}
			}
		} catch (e) {
			console.log('No saved config found');
		}

		// Initialize notifications
		await initNotifications();
		startTransactionWatcher();

		// Apply theme on load
		const savedTheme = localStorage.getItem('nexium-theme') || 'dark';
		document.documentElement.setAttribute('data-theme', savedTheme);
	});

	// React to theme changes
	$effect(() => {
		document.documentElement.setAttribute('data-theme', $theme);
	});
</script>

{@render children()}

