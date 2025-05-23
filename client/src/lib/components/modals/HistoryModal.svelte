<script lang="ts">
	import { Minus } from 'lucide-svelte';
	import { Plus } from 'lucide-svelte';

	import { onMount } from 'svelte';
	import { fade, fly } from 'svelte/transition';
	import { writable, type Writable } from 'svelte/store';

	import { globalConfig } from '$lib/stores/settings.js';
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
	/*
    Quand tu vas get le invoke get_transactions, tu vas recevoir une liste de transactions, chaque transaction sera dans ce format JS :
    transaction = {
        receiver: String,
        emitter: String,
        description: String,
        amount: String,
        date: String,
        inOrOut: String,
    }

    inOrOut = "IN" ou "OUT"
    */
	type Transaction = {
		emitter: string;
		receiver: string;
		description: string;
		amount: string;
		date: string;
		inOrOut: 'IN' | 'OUT';
	};

	const transactions: Writable<Transaction[]> = writable([]);

	onMount(async () => {
		try {
			const config = $globalConfig;
			const login = '';
			const n = '';
			const result = await invoke('get_transactions', { config, login, n });
			if (Array.isArray(result)) {
				transactions.set(
					result.map((t: any) => ({
						emitter: t.emitter || '',
						receiver: t.receiver || '',
						description: t.description || '',
						date: t.date || '',
						amount: t.amount || '',
						inOrOut: t.inOrOut || 'OUT'
					}))
				);
			} else {
				transactions.set([]);
			}
		} catch (e) {}
	});
	transactions.set([
		{
			emitter: 'jean.herail',
			receiver: 'milo.delbos',
			description: 'Pour le café du matin',
			amount: '42.50',
			date: '2025-05-23',
			inOrOut: 'OUT'
		},
		{
			emitter: 'milo.delbos',
			receiver: 'jean.herail',
			description: 'Pour le café du matin',
			amount: '42.50',
			date: '2025-05-23',
			inOrOut: 'IN'
		},
		{
			emitter: 'jean.herail',
			receiver: 'milo.delbos',
			description:
				'Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry s standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. ',
			amount: '42.50',
			date: '2025-05-23',
			inOrOut: 'OUT'
		},
		{
			emitter: 'milo.delbos',
			receiver: 'jean.herail',
			description:
				'Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry s standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. ',
			amount: '42.50',
			date: '2025-05-23',
			inOrOut: 'IN'
		},
		{
			emitter: 'jean.herail',
			receiver: 'milo.delbos',
			description:
				'Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry s standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. ',
			amount: '75.00',
			date: '2025-05-20',
			inOrOut: 'OUT'
		},
		{
			emitter: 'william.valenduc',
			receiver: 'jean.herail',
			description:
				'Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry s standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. ',
			amount: '75.00',
			date: '2025-05-20',
			inOrOut: 'IN'
		}
	]);
</script>

<div class="history-modal" transition:fade={{ duration: 200 }}>
	<div class="history-modal-content" transition:fly={{ y: 30, duration: 200 }}>
		<h3 class="history-titre">Historique des transactions</h3>
		<div class="contenu-tableau">
			<!-- Tableau avec les transactions-->
			<!--
            type (moins ou plus en rouge ou vert) | emetteur | recepteur | description | date | montant
        -->

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
