<script>
	import MenuGauche from '$lib/components/MenuGauche.svelte';
	import Solde from '$lib/components/Solde.svelte';

	import { Settings, TruckElectric } from 'lucide-svelte';
	import { CircleAlert } from 'lucide-svelte';

	import SendModal from '$lib/components/modals/SendModal.svelte';
	import ReceiveModal from '$lib/components/modals/ReceiveModal.svelte';

	import SettingsModal from '$lib/components/modals/SettingsModal.svelte';
	import {
		globalConfig,
		isConfigSet,
		showHistoryModal,
		showReceiveModal,
		showSendModal
	} from '$lib/stores/settings.js';
	import HistoryModal from '$lib/components/modals/HistoryModal.svelte';

	let showSettingsModal = false;
</script>

<div class="relative flex min-h-screen items-center justify-center">
	<!-- Buttons in the top-right corner -->
	<div class="absolute right-4 top-4 flex gap-2">
		<button onclick={() => (showSettingsModal = true)}>
			<Settings strokeWidth={2.4} size={25} class="bouton-settings" />
		</button>
	</div>

	<div class="flex gap-10" style="padding-top: 48px;">
		<MenuGauche />
		<Solde />
	</div>

	<SettingsModal bind:showSettingsModal></SettingsModal>

	{#if $showSendModal}
		<SendModal oncancel={showSendModal.set(false)} />
	{/if}

	{#if $showReceiveModal}
		<ReceiveModal oncancel={showReceiveModal.set(false)} />
	{/if}

	{#if $showHistoryModal}
		<HistoryModal oncancel={showHistoryModal.set(false)} />
	{/if}

	{#if !$isConfigSet}
		<div class="missing-config-container">
			<button onclick={() => (showSettingsModal = true)}>
				<CircleAlert strokeWidth={3.7} size={20} class="icon" id="missing-config-icon" />Aucune
				configuration</button
			>
		</div>
	{/if}
</div>
