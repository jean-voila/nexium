<script>
	import { globalConfig, isConfigSet } from '$lib/stores/settings.js';
	import { invoke } from '@tauri-apps/api/core';
	import { userBalance } from '$lib/stores/settings.js';
	let userBalanceDecimals = '';
	let userBalanceInteger = '';

	$: if ($globalConfig?.user_login) {
		invoke('get_balance', { login: $globalConfig.user_login })
			.then((balance) => {
				userBalance.set(balance);
			})
			.catch(() => {});
	}
	$: userBalance.subscribe((val) => {
		if (val) {
			[userBalanceInteger, userBalanceDecimals] = splitBalance(val);
		}
	});

	/**
	 * @param {string} userBalance
	 * @returns {[string, string]}
	 */
	function splitBalance(userBalance) {
		if (userBalance) {
			const parts = userBalance.toString().split('.');
			return [parts[0], parts[1] ? parts[1].slice(0, 2) : '00'];
		} else {
			return ['0', '00'];
		}
	}
</script>

<div class="relative h-80 w-80">
	<div
		class="absolute inset-0 rounded-full border-4"
		class:cercle={$isConfigSet}
		class:cercle-disabled={!$isConfigSet}
	></div>
	<div
		class="absolute inset-0 left-1/2 top-1/2 h-60 w-60 -translate-x-1/2 -translate-y-1/2 transform rounded-full bg-black"
	></div>
	<div class="cercle-gris border-19 absolute inset-0 rounded-full"></div>

	<div class="absolute inset-0 flex flex flex-col items-center justify-center">
		{#if $isConfigSet}
			<div class="text-4xl text-white">
				<span class="solde-unit solde">{userBalanceInteger}</span>.<span class="solde-deci solde"
					>{userBalanceDecimals}</span
				>
			</div>
			<span class="sous-titre">NXM</span>
		{/if}
	</div>
</div>
