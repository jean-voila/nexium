<script>
	import MenuGauche from '$lib/components/MenuGauche.svelte';
	import Solde from '$lib/components/Solde.svelte';

	import { Settings } from 'lucide-svelte';
	import { Server } from 'lucide-svelte';
	import { CircleAlert } from 'lucide-svelte';

	import SettingsModal from '$lib/components/SettingsModal.svelte';
	let showSettingsModal = false;

	import { globalUrl } from '$lib/stores/settings.js';

	/**
	 * @type {string}
	 */
	let currentServerUrl;
	import { onMount } from 'svelte';
	onMount(() => {
		const unsubscribe = globalUrl.subscribe((value) => {
			currentServerUrl = value;
		});

		return unsubscribe;
	});
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

	<div class="server-url-container" class:red={!currentServerUrl} class:gray={currentServerUrl}>
		{#if currentServerUrl}
			<Server strokeWidth={3} size={20} class="icon" />
			{currentServerUrl}
		{:else}
			<button onclick={() => (showSettingsModal = true)}>
				<CircleAlert strokeWidth={3} size={20} class="icon" />
				Configuration manquante
			</button>
		{/if}
	</div>
</div>
