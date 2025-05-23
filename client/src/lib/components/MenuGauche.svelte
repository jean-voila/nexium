<script>
	import Bouton from '$lib/components/Bouton.svelte';
	import { SendHorizontal } from 'lucide-svelte';
	import { HandCoins } from 'lucide-svelte';
	import { onMount } from 'svelte';

	import { Copy } from 'lucide-svelte';
	import { writeText, readText } from '@tauri-apps/plugin-clipboard-manager';

	import { Check } from 'lucide-svelte';
	import { fade } from 'svelte/transition';

	import { invoke } from '@tauri-apps/api/core';

	let showCopyMessage = false;
	let tooltipX = 0;
	let tooltipY = 0;

	import {
		globalConfig,
		isConfigSet,
		showReceiveModal,
		showSendModal
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

	// Réagit aux changements de login utilisateur
	$: if ($globalConfig?.user_login) {
		invoke('get_names_from_login', { login: $globalConfig.user_login })
			.then(([first, last]) => {
				firstName = first;
				lastName = last;
			})
			.catch((err) => {
				firstName = '';
				lastName = '';
			});
	}
</script>

<div class="flex flex-col justify-center gap-14">
	<div class="flex flex-col gap-2">
		<div class="flex flex-col">
			{#if firstName || lastName}
				<div class="prenom">{firstName}</div>
				<div class="nom">{lastName}</div>
			{:else}
				<div class="placeholder w-20">
					<div class="animated-background"></div>
				</div>
				<div class="placeholder w-40">
					<div class="animated-background"></div>
				</div>
			{/if}
		</div>
		<div class="flex gap-1">
			<button
				onclick={copyLogin}
				onmouseenter={() => (showCopyMessage = true)}
				onmouseleave={() => (showCopyMessage = false)}
				onmousemove={(e) => {
					tooltipX = e.clientX;
					tooltipY = e.clientY;
				}}
				hidden={!$isConfigSet}
			>
				<Copy strokeWidth={2.4} size={25} class="bouton-action" />
			</button>
		</div>
	</div>

	<div class="flex flex-col gap-2">
		<button onclick={() => showSendModal.set(true)} disabled={!$isConfigSet}>
			<Bouton label="Envoyer" Icon={SendHorizontal} disabled={!$isConfigSet} />
		</button>

		<button onclick={() => showReceiveModal.set(true)} disabled={!$isConfigSet}>
			<Bouton label="Recevoir" type="secondaire" Icon={HandCoins} disabled={!$isConfigSet} />
		</button>
	</div>
</div>

{#if showCopyMessage}
	<div class="copy-tooltip" style="top: {tooltipY}px; left: {tooltipX}px;">
		<span
			class="absolute transition-all duration-500 ease-in-out"
			class:translate-y-0={!copied}
			class:-translate-y-3={copied}
			class:opacity-100={!copied}
			class:opacity-0={copied}
		>
			{$globalConfig.user_login}
		</span>
		<span
			class="absolute transition-all duration-500 ease-in-out"
			class:translate-y-3={!copied}
			class:translate-y-0={copied}
			class:opacity-0={!copied}
			class:opacity-100={copied}
		>
			<Check size={20} strokeWidth={4} class="green-icon" />
		</span>
	</div>
{/if}
