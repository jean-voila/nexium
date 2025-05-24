<script lang="ts">
	import { Minus } from 'lucide-svelte';
	import { Plus } from 'lucide-svelte';
	import { RefreshCw } from 'lucide-svelte';

	import { onMount } from 'svelte';
	import { fade, fly } from 'svelte/transition';
	import { writable, type Writable } from 'svelte/store';

	import { globalConfig, showHistoryModal } from '$lib/stores/settings.js';
	import { invoke } from '@tauri-apps/api/core';

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
		inOrOut: string;
	};

	let n = '10';

	const transactions: Writable<Transaction[]> = writable([]);

	async function refreshList() {
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
						inOrOut: t.inOrOut || 'OUT'
					}))
				);
			} else {
				transactions.set([]);
			}
		} catch (e) {
			console.log(e);
		}
	}

	let interval: number | null = null;

	onMount(() => {
		const unsubscribe = showHistoryModal.subscribe((visible) => {
			if (visible) {
				refreshList();
				if (!interval) {
					interval = setInterval(refreshList, 10000);
				}
			} else if (interval) {
				clearInterval(interval);
				interval = null;
			}
		});

		return () => {
			unsubscribe();
			if (interval) {
				clearInterval(interval);
				interval = null;
			}
		};
	});
</script>

<div class="history-modal" transition:fade={{ duration: 200 }}>
	<div class="history-modal-content" transition:fly={{ y: 30, duration: 200 }}>
		<h3 class="history-titre">Historique des transactions</h3>
		<div class="contenu-tableau scrollable-table">
			<Table>
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
								{#if t.inOrOut === 'IN'}
									<span class="icon-in"><Plus size="17" strokeWidth={5} /></span>
								{:else}
									<span class="icon-out"><Minus size="17" strokeWidth={5} /></span>
								{/if}
							</TableBodyCell>
							<TableBodyCell class="login">{t.emitter}</TableBodyCell>
							<TableBodyCell class="login">{t.receiver}</TableBodyCell>
							<TableBodyCell class="description-scroll-cell">
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

		<div class="mt-4 flex justify-end gap-2">
			<button
				class="pillule-bouton-history pillule-bouton-history-noir bouton-noir-settings flex items-center transition"
				onclick={handleClose}
			>
				<span class="texte-bouton-history texte-bouton-history-noir">Fermer</span>
			</button>
		</div>
	</div>
</div>
