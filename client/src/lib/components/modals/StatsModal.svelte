<script lang="ts">
	import { fade, fly } from 'svelte/transition';
	import { invoke } from '@tauri-apps/api/core';
	import { X, TrendingUp, TrendingDown, Activity, ArrowUpRight, ArrowDownLeft, Wallet } from 'lucide-svelte';
	import { globalConfig } from '$lib/stores/settings.js';
	import { onMount } from 'svelte';
	import Spinner from '$lib/components/Spinner.svelte';

	let { oncancel } = $props();

	type Transaction = {
		receiver: string;
		emitter: string;
		description: string;
		amount: string;
		date: string;
		inorout: string;
	};

	let transactions = $state<Transaction[]>([]);
	let loading = $state(true);

	// Stats
	let totalSent = $state(0);
	let totalReceived = $state(0);
	let transactionCount = $state(0);
	let sentCount = $state(0);
	let receivedCount = $state(0);
	let avgTransaction = $state(0);
	let topRecipients = $state<{ login: string; total: number; count: number }[]>([]);
	let topSenders = $state<{ login: string; total: number; count: number }[]>([]);

	function handleClose() {
		oncancel?.();
	}

	async function loadStats() {
		loading = true;
		try {
			const result = await invoke('get_transactions', {
				config: $globalConfig,
				login: $globalConfig.user_login,
				n: '100'
			});

			if (Array.isArray(result)) {
				transactions = result.map((t: any) => ({
					receiver: t.receiver || '',
					emitter: t.emitter || '',
					description: t.description || '',
					amount: t.amount || '0',
					date: t.date || '',
					inorout: t.inorout || 'IN'
				}));
				calculateStats();
			}
		} catch (e) {
			console.error(e);
		} finally {
			loading = false;
		}
	}

	function calculateStats() {
		let sent = 0;
		let received = 0;
		let sCount = 0;
		let rCount = 0;
		const recipientMap = new Map<string, { total: number; count: number }>();
		const senderMap = new Map<string, { total: number; count: number }>();

		for (const tx of transactions) {
			const amount = parseFloat(tx.amount) || 0;

			if (tx.inorout === 'OUT') {
				sent += amount;
				sCount++;
				const existing = recipientMap.get(tx.receiver) || { total: 0, count: 0 };
				recipientMap.set(tx.receiver, {
					total: existing.total + amount,
					count: existing.count + 1
				});
			} else {
				received += amount;
				rCount++;
				const existing = senderMap.get(tx.emitter) || { total: 0, count: 0 };
				senderMap.set(tx.emitter, {
					total: existing.total + amount,
					count: existing.count + 1
				});
			}
		}

		totalSent = sent;
		totalReceived = received;
		sentCount = sCount;
		receivedCount = rCount;
		transactionCount = transactions.length;
		avgTransaction = transactionCount > 0 ? (sent + received) / transactionCount : 0;

		topRecipients = Array.from(recipientMap.entries())
			.map(([login, data]) => ({ login, ...data }))
			.sort((a, b) => b.total - a.total)
			.slice(0, 5);

		topSenders = Array.from(senderMap.entries())
			.map(([login, data]) => ({ login, ...data }))
			.sort((a, b) => b.total - a.total)
			.slice(0, 5);
	}

	function formatAmount(amount: number): string {
		return amount.toLocaleString('fr-FR', {
			minimumFractionDigits: 2,
			maximumFractionDigits: 2
		});
	}

	onMount(() => {
		loadStats();
	});
</script>

<div class="modal-backdrop" transition:fade={{ duration: 200 }}>
	<div class="modal-container modal-container-lg" transition:fly={{ y: 30, duration: 200 }}>
		<div class="modal-header">
			<h3 class="modal-title">Statistiques</h3>
			<button class="modal-close" onclick={handleClose}>
				<X size={18} />
			</button>
		</div>

		<div class="modal-body">
			{#if loading}
				<div class="loading-state">
					<Spinner />
					<p>Chargement des statistiques...</p>
				</div>
			{:else if transactions.length === 0}
				<div class="empty-state">
					<Activity size={48} strokeWidth={1} />
					<p>Aucune transaction à analyser</p>
				</div>
			{:else}
				<!-- Stats Grid -->
				<div class="grid grid-cols-2 gap-4 mb-6">
					<div class="stat-card">
						<div class="stat-card-icon blue">
							<Wallet size={20} />
						</div>
						<div class="stat-card-content">
							<span class="stat-card-label">Total transactions</span>
							<span class="stat-card-value">{transactionCount}</span>
						</div>
					</div>

					<div class="stat-card">
						<div class="stat-card-icon purple">
							<Activity size={20} />
						</div>
						<div class="stat-card-content">
							<span class="stat-card-label">Moyenne/transaction</span>
							<span class="stat-card-value">{formatAmount(avgTransaction)} NXM</span>
						</div>
					</div>

					<div class="stat-card">
						<div class="stat-card-icon red">
							<ArrowUpRight size={20} />
						</div>
						<div class="stat-card-content">
							<span class="stat-card-label">Total envoyé</span>
							<span class="stat-card-value">{formatAmount(totalSent)} NXM</span>
							<span class="stat-card-sub">{sentCount} transactions</span>
						</div>
					</div>

					<div class="stat-card">
						<div class="stat-card-icon green">
							<ArrowDownLeft size={20} />
						</div>
						<div class="stat-card-content">
							<span class="stat-card-label">Total reçu</span>
							<span class="stat-card-value">{formatAmount(totalReceived)} NXM</span>
							<span class="stat-card-sub">{receivedCount} transactions</span>
						</div>
					</div>
				</div>

				<!-- Balance Bar -->
				<div class="mb-6">
					<div class="progress-bar">
						<div
							class="progress-bar-fill"
							style="width: {totalReceived + totalSent > 0
								? (totalReceived / (totalReceived + totalSent)) * 100
								: 50}%"
						></div>
					</div>
					<div class="progress-labels">
						<span class="text-success">Reçu: {formatAmount(totalReceived)}</span>
						<span class="text-danger">Envoyé: {formatAmount(totalSent)}</span>
					</div>
				</div>

				<!-- Top Lists -->
				<div class="grid grid-cols-2 gap-4">
					{#if topRecipients.length > 0}
						<div class="card">
							<h4 class="flex items-center gap-2 text-sm font-semibold text-muted mb-3">
								<TrendingUp size={14} /> Top destinataires
							</h4>
							{#each topRecipients as recipient, i}
								<div class="flex items-center gap-3 py-2 border-b border-glass-border last:border-0">
									<span class="text-muted text-sm w-6">#{i + 1}</span>
									<span class="font-mono text-sm flex-1">{recipient.login}</span>
									<span class="text-sm font-medium">{formatAmount(recipient.total)}</span>
									<span class="text-muted text-xs">({recipient.count}x)</span>
								</div>
							{/each}
						</div>
					{/if}

					{#if topSenders.length > 0}
						<div class="card">
							<h4 class="flex items-center gap-2 text-sm font-semibold text-muted mb-3">
								<TrendingDown size={14} /> Top expéditeurs
							</h4>
							{#each topSenders as sender, i}
								<div class="flex items-center gap-3 py-2 border-b border-glass-border last:border-0">
									<span class="text-muted text-sm w-6">#{i + 1}</span>
									<span class="font-mono text-sm flex-1">{sender.login}</span>
									<span class="text-sm font-medium">{formatAmount(sender.total)}</span>
									<span class="text-muted text-xs">({sender.count}x)</span>
								</div>
							{/each}
						</div>
					{/if}
				</div>
			{/if}
		</div>

		<div class="modal-footer">
			<button class="btn btn-ghost" onclick={handleClose}>Fermer</button>
		</div>
	</div>
</div>

<style>
	.grid {
		display: grid;
	}
	.grid-cols-2 {
		grid-template-columns: repeat(2, 1fr);
	}
	.border-b {
		border-bottom-width: 1px;
	}
	.border-glass-border {
		border-color: var(--glass-border);
	}
	.last\:border-0:last-child {
		border-width: 0;
	}
</style>
