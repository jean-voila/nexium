<script>
	import { SendHorizontal, HandCoins, History, Copy, Check } from 'lucide-svelte';
	import { onMount } from 'svelte';
	import { writeText } from '@tauri-apps/plugin-clipboard-manager';
	import { invoke } from '@tauri-apps/api/core';
	import { get } from 'svelte/store';

	import {
		globalConfig,
		isConfigSet,
		showReceiveModal,
		showSendModal,
		userBalanceInt,
		userBalanceDec,
		showHistoryModal,
		globalErrorMessage
	} from '$lib/stores/settings.js';

	let firstName = '';
	let lastName = '';
	let copied = false;

	async function copyLogin() {
		if (!$globalConfig?.user_login) return;
		await writeText($globalConfig.user_login);
		copied = true;
		setTimeout(() => {
			copied = false;
		}, 2000);
	}

	// React to user login changes
	$: if ($globalConfig?.user_login) {
		balanceUpdate();
		invoke('get_names_from_login', { login: $globalConfig.user_login })
			.then(([first, last]) => {
				firstName = first;
				lastName = last;
			})
			.catch(() => {
				firstName = '';
				lastName = '';
			});
	}

	async function balanceUpdate() {
		if (get(showHistoryModal) || get(showSendModal) || get(showReceiveModal)) return;
		if ($isConfigSet === false) return;
		try {
			const balance = await invoke('get_balance', {
				login: $globalConfig.user_login,
				config: $globalConfig
			});
			if (balance) {
				userBalanceInt.set(balance[0]);
				userBalanceDec.set(balance[1]);
			}
			globalErrorMessage.set('');
		} catch (error) {
			globalErrorMessage.set('Erreur lors de la récupération du solde.');
		}
	}

	onMount(() => {
		const interval = setInterval(balanceUpdate, 5000);
		return () => clearInterval(interval);
	});
</script>

<div class="user-panel">
	<!-- User Identity -->
	<div class="user-identity">
		{#if firstName || lastName}
			<div class="user-firstname">{firstName}</div>
			<div class="user-lastname">{lastName}</div>
		{:else if $isConfigSet}
			<div class="skeleton skeleton-text-lg"></div>
			<div class="skeleton skeleton-text-sm mt-2"></div>
		{/if}

		<!-- Quick Actions -->
		{#if $isConfigSet}
			<div class="user-actions mt-4">
				<button onclick={copyLogin} class="action-link" title="Copier le login">
					{#if copied}
						<Check strokeWidth={2} size={18} />
					{:else}
						<Copy strokeWidth={2} size={18} />
					{/if}
				</button>
				<button
					onclick={() => showHistoryModal.set(true)}
					class="action-link"
					title="Historique"
				>
					<History strokeWidth={2} size={18} />
				</button>
			</div>
		{/if}
	</div>

	<!-- Main Action Buttons -->
	{#if $isConfigSet}
		<div class="action-buttons">
			<button
				onclick={() => showSendModal.set(true)}
				class="btn-primary"
			>
				<SendHorizontal strokeWidth={2.5} size={20} />
				Envoyer
			</button>

			<button
				onclick={() => showReceiveModal.set(true)}
				class="btn-secondary"
			>
				<HandCoins strokeWidth={2.5} size={20} />
				Recevoir
			</button>
		</div>
	{/if}
</div>
