<script lang="ts">
	import { isConfigSet, userBalanceInt, userBalanceDec, globalConfig, showHistoryModal, showSendModal, showReceiveModal } from '$lib/stores/settings.js';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import { get } from 'svelte/store';
	import { Plus, Minus } from 'lucide-svelte';
	import NumberFlow from '@number-flow/svelte';

	type Transaction = {
		receiver: string;
		emitter: string;
		description: string;
		amount: string;
		date: string;
		inorout: string;
	};

	let recentTransactions = $state<Transaction[]>([]);

	async function fetchRecentTransactions() {
		if (get(showHistoryModal) || get(showSendModal) || get(showReceiveModal)) return;
		if (get(isConfigSet) === false) return;
		try {
			const result = await invoke('get_transactions', {
				config: get(globalConfig),
				login: get(globalConfig).user_login,
				n: '2'
			});
			if (Array.isArray(result)) {
				recentTransactions = result.map((t: any) => ({
					receiver: t.receiver || '',
					emitter: t.emitter || '',
					description: t.description || '',
					amount: t.amount || '',
					date: t.date || '',
					inorout: t.inorout || 'IN'
				}));
			}
		} catch (e) {
			console.log(e);
		}
	}

	onMount(() => {
		fetchRecentTransactions();
		const interval = setInterval(fetchRecentTransactions, 5000);
		return () => clearInterval(interval);
	});
</script>

<div class="balance-container">
	{#if $isConfigSet}
		<div class="balance-card">
			<div class="balance-center">
				<div class="balance-amount">
					<NumberFlow
						value={$userBalanceInt}
						format={{ useGrouping: false }}
						transformTiming={{ duration: 750, easing: 'ease-out' }}
						spinTiming={{ duration: 750, easing: 'ease-out' }}
						opacityTiming={{ duration: 350, easing: 'ease-out' }}
					/>{#if $userBalanceDec !== '0'}<span class="balance-decimals">.<NumberFlow
						value={$userBalanceDec}
						format={{ useGrouping: false }}
						transformTiming={{ duration: 750, easing: 'ease-out' }}
						spinTiming={{ duration: 750, easing: 'ease-out' }}
						opacityTiming={{ duration: 350, easing: 'ease-out' }}
					/></span>{/if}
				</div>
				<span class="balance-unit">NXM</span>
			</div>
		</div>

		{#if recentTransactions.length > 0}
			<div class="recent-transactions">
				<div class="recent-transactions-header">Transactions récentes</div>
				<div class="recent-transactions-list">
					{#each recentTransactions as t}
						<div class="recent-tx-item">
							<div class="recent-tx-icon">
								{#if t.inorout === 'IN'}
									<span class="tx-icon-in"><Plus size={14} strokeWidth={3} /></span>
								{:else}
									<span class="tx-icon-out"><Minus size={14} strokeWidth={3} /></span>
								{/if}
							</div>
							<div class="recent-tx-details">
								<span class="recent-tx-party">{t.inorout === 'IN' ? t.emitter : t.receiver}</span>
								<span class="recent-tx-date">{t.date}</span>
							</div>
							<div class="recent-tx-amount {t.inorout === 'IN' ? 'amount-in' : 'amount-out'}">
								{t.inorout === 'IN' ? '+' : '-'}{t.amount}
							</div>
						</div>
					{/each}
				</div>
			</div>
		{/if}
	{/if}
</div>
