<script lang="ts">
	import { Minus } from 'lucide-svelte';
	import { Plus } from 'lucide-svelte';
	import { RefreshCw } from 'lucide-svelte';

	import { onMount } from 'svelte';
	import { fade, fly } from 'svelte/transition';
	import { writable, type Writable } from 'svelte/store';
	import Spinner from '$lib/components/Spinner.svelte';

	import { globalConfig, showHistoryModal } from '$lib/stores/settings.js';
	import { invoke } from '@tauri-apps/api/core';

	let tooltipX = $state(0);
	let tooltipY = $state(0);
	let fullDescription = $state('');
	let tooltipTimeout: ReturnType<typeof setTimeout> | null = null;
	let hideTimeout: ReturnType<typeof setTimeout> | null = null;

	let copied = false;

	let { oncancel } = $props();

	function handleClose() {
		oncancel?.();
	}

	import {
		Table,
		TableBody,
		TableBodyCell,
		TableBodyRow,
		TableHead,
		TableHeadCell
	} from 'flowbite-svelte';

	type Transaction = {
		receiver: string;
		emitter: string;
		description: string;
		amount: string;
		date: string;
		inorout: string;
	};

	let n = '10';

	const transactions: Writable<Transaction[]> = writable([]);

	let lastRefresh = $state(0);
	let loading = $state(false);

	async function refreshList() {
		const now = Date.now();
		if (now - lastRefresh < 6000) {
			return; // Si moins de 6 secondes se sont écoulées, on ne fait rien
		}
		lastRefresh = now; // Mettez à jour le temps du dernier rafraîchissement
		loading = true; // Démarre le chargement

		try {
			const result = await invoke('get_transactions', {
				config: $globalConfig,
				login: $globalConfig.user_login,
				n: n
			});
			if (Array.isArray(result)) {
				transactions.set(
					result.map((t: any) => ({
						receiver: t.receiver || '',
						emitter: t.emitter || '',
						description: t.description || '',
						amount: t.amount || '',
						date: t.date || '',
						inorout: t.inorout || 'IN'
					}))
				);
			} else {
				transactions.set([]);
			}
		} catch (e) {
			console.log(e);
		} finally {
			loading = false; // Arrête le chargement
		}
	}

	onMount(() => {
		const unsubscribe = showHistoryModal.subscribe((visible) => {
			if (visible) {
				refreshList();
			}
		});

		return () => {
			unsubscribe();
		};
	});
</script>

<div class="history-modal" transition:fade={{ duration: 200 }}>
	<div class="history-modal-content" transition:fly={{ y: 30, duration: 200 }}>
		<h3 class="history-titre">Historique des transactions</h3>
		<div class="contenu-tableau scrollable-table">
			<Table class={loading ? 'loading' : ''}>
				<TableHead>
					<TableHeadCell>Type</TableHeadCell>
					<TableHeadCell>Émetteur</TableHeadCell>
					<TableHeadCell>Récepteur</TableHeadCell>
					<TableHeadCell>Description</TableHeadCell>
					<TableHeadCell>Date</TableHeadCell>
					<TableHeadCell>Montant</TableHeadCell>
				</TableHead>
				<TableBody>
					{#each $transactions as t}
						<TableBodyRow>
							<TableBodyCell>
								{#if t.inorout === 'IN'}
									<span class="icon-in"><Plus size="17" strokeWidth={4} /></span>
								{:else}
									<span class="icon-out"><Minus size="17" strokeWidth={4} /></span>
								{/if}
							</TableBodyCell>
							<TableBodyCell class="login">{t.emitter}</TableBodyCell>
							<TableBodyCell class="login">{t.receiver}</TableBodyCell>
							<TableBodyCell
								class="description-scroll-cell"
								onmouseenter={() => {
									if (hideTimeout) {
										clearTimeout(hideTimeout);
										hideTimeout = null;
									}
									if (fullDescription && fullDescription !== t.description) {
										fullDescription = t.description;
										return;
									}
									tooltipTimeout = setTimeout(() => {
										fullDescription = t.description;
									}, 1000);
								}}
								onmouseleave={() => {
									if (tooltipTimeout) {
										clearTimeout(tooltipTimeout);
										tooltipTimeout = null;
									}
									if (hideTimeout) {
										clearTimeout(hideTimeout);
									}
									hideTimeout = setTimeout(() => {
										fullDescription = '';
									}, 500);
								}}
								onmousemove={(e) => {
									tooltipX = e.clientX;
									tooltipY = e.clientY;
								}}
							>
								<div class="description-scroll-content">
									{t.description}
								</div>
							</TableBodyCell>
							<TableBodyCell>{t.date}</TableBodyCell>
							<TableBodyCell class="login">{t.amount}</TableBodyCell>
						</TableBodyRow>
					{/each}
				</TableBody>
			</Table>
		</div>

		<div class="mt-4 flex items-center justify-between gap-2 pl-4 pr-4">
			<div class="flex items-center gap-2">
				<button
					class="flex items-center justify-center transition"
					onclick={refreshList}
					aria-label="Rafraîchir"
					disabled={loading}
				>
					<RefreshCw
						size="19"
						strokeWidth="3.5"
						class={loading ? 'bouton-action-disabled' : 'bouton-action'}
					/>
				</button>
				{#if loading}
					<div class="flex items-center">
						<Spinner />
					</div>
				{/if}
			</div>

			<button
				class="pillule-bouton-history pillule-bouton-history-noir bouton-noir-settings flex items-center transition"
				onclick={handleClose}
			>
				<span class="texte-bouton-history texte-bouton-history-noir">Fermer</span>
			</button>
		</div>
	</div>
</div>

{#if fullDescription}
	<div
		class="description-tooltip"
		style="top: {tooltipY}px; left: {tooltipX}px;"
		transition:fade={{ duration: 200 }}
	>
		<span
			class="absolute"
			class:translate-y-0={!copied}
			class:-translate-y-3={copied}
			class:opacity-100={!copied}
			class:opacity-0={copied}
		>
			{fullDescription}
		</span>
	</div>
{/if}
