<script>
	import { globalConfig, isConfigSet } from '$lib/stores/settings.js';
	import { invoke } from '@tauri-apps/api/core';
	import { userBalanceInt, userBalanceDec } from '$lib/stores/settings.js';
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
				<span class="solde-unit solde">{$userBalanceInt}</span>{#if $userBalanceDec}.<span
						class="solde-deci solde">{$userBalanceDec}</span
					>{/if}
			</div>
			<span class="sous-titre">NXM</span>
		{/if}
	</div>
</div>
