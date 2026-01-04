<script lang="ts">
	import { Plus, Minus, RefreshCw, Download, X } from 'lucide-svelte';
	import { onMount } from 'svelte';
	import { fade, fly } from 'svelte/transition';
	import { writable, type Writable } from 'svelte/store';
	import Spinner from '$lib/components/Spinner.svelte';
	import { save } from '@tauri-apps/plugin-dialog';
	import { writeTextFile } from '@tauri-apps/plugin-fs';
	import { globalConfig, showHistoryModal } from '$lib/stores/settings.js';
	import { invoke } from '@tauri-apps/api/core';

	let tooltipX = $state(0);
	let tooltipY = $state(0);
	let fullDescription = $state('');
	let tooltipTimeout: ReturnType<typeof setTimeout> | null = null;
	let hideTimeout: ReturnType<typeof setTimeout> | null = null;

	let { oncancel } = $props();

	function handleClose() {
		oncancel?.();
	}

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
		if (now - lastRefresh < 6000) return;
		lastRefresh = now;
		loading = true;

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
			loading = false;
		}
	}

	onMount(() => {
		const unsubscribe = showHistoryModal.subscribe((visible) => {
			if (visible) refreshList();
		});
		return () => unsubscribe();
	});

	async function exportToCSV() {
		const txList = $transactions;
		if (txList.length === 0) return;

		const headers = ['Type', 'Émetteur', 'Récepteur', 'Description', 'Date', 'Montant'];
		const rows = txList.map((t) => [
			t.inorout === 'IN' ? 'Reçu' : 'Envoyé',
			t.emitter,
			t.receiver,
			`"${t.description.replace(/"/g, '""')}"`,
			t.date,
			t.amount
		]);

		const csvContent = [headers.join(';'), ...rows.map((row) => row.join(';'))].join('\n');

		const path = await save({
			filters: [{ name: 'CSV', extensions: ['csv'] }],
			defaultPath: `nexium_transactions_${new Date().toISOString().slice(0, 10)}.csv`
		});

		if (path) {
			try {
				await writeTextFile(path, csvContent);
			} catch (e) {
				console.error('Error exporting CSV:', e);
			}
		}
	}
</script>

<div class="modal-backdrop" transition:fade={{ duration: 200 }}>
	<div class="modal-container modal-container-xl" transition:fly={{ y: 30, duration: 200 }}>
		<div class="modal-header">
			<h3 class="modal-title">Historique des transactions</h3>
			<div class="flex items-center gap-2">
				<button class="btn btn-sm btn-ghost" onclick={refreshList} disabled={loading}>
					<RefreshCw size={14} class={loading ? 'spinner' : ''} />
				</button>
				<button class="btn btn-sm btn-ghost" onclick={exportToCSV} disabled={$transactions.length === 0}>
					<Download size={14} />
				</button>
				<button class="modal-close" onclick={handleClose}>
					<X size={18} />
				</button>
			</div>
		</div>

		<div class="modal-body" style="padding: 0;">
			{#if loading && $transactions.length === 0}
				<div class="loading-state">
					<Spinner />
					<p>Chargement...</p>
				</div>
			{:else if $transactions.length === 0}
				<div class="empty-state">
					<p>Aucune transaction</p>
				</div>
			{:else}
				<div class="table-container" style="border: none; border-radius: 0;">
					<table class="data-table">
						<thead>
							<tr>
								<th style="width: 50px;"></th>
								<th>Émetteur</th>
								<th>Récepteur</th>
								<th>Description</th>
								<th>Date</th>
								<th style="text-align: right;">Montant</th>
							</tr>
						</thead>
						<tbody class={loading ? 'opacity-50' : ''}>
							{#each $transactions as t}
								<tr>
									<td style="text-align: center;">
										{#if t.inorout === 'IN'}
											<span class="tx-icon-in"><Plus size={16} strokeWidth={3} /></span>
										{:else}
											<span class="tx-icon-out"><Minus size={16} strokeWidth={3} /></span>
										{/if}
									</td>
									<td class="login">{t.emitter}</td>
									<td class="login">{t.receiver}</td>
									<td
										style="max-width: 200px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;"
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
											}, 500);
										}}
										onmouseleave={() => {
											if (tooltipTimeout) {
												clearTimeout(tooltipTimeout);
												tooltipTimeout = null;
											}
											if (hideTimeout) clearTimeout(hideTimeout);
											hideTimeout = setTimeout(() => {
												fullDescription = '';
											}, 300);
										}}
										onmousemove={(e) => {
											tooltipX = e.clientX;
											tooltipY = e.clientY;
										}}
									>
										{t.description || '—'}
									</td>
									<td>{t.date}</td>
									<td class="amount text-right">{t.amount}</td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>
			{/if}
		</div>

		<div class="modal-footer">
			<button class="btn btn-ghost" onclick={handleClose}>
				Fermer
			</button>
		</div>
	</div>
</div>

{#if fullDescription}
	<div class="tooltip" style="top: {tooltipY}px; left: {tooltipX}px;">
		{fullDescription}
	</div>
{/if}
