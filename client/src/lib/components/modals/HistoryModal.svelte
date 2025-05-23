<script>
	import { Minus } from 'lucide-svelte';
	import { Plus } from 'lucide-svelte';

	import { onMount } from 'svelte';
	import { fade, fly } from 'svelte/transition';
	import { writable } from 'svelte/store';

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
</script>

<div class="history-modal" transition:fade={{ duration: 200 }}>
	<div class="history-modal-content" transition:fly={{ y: 30, duration: 200 }}>
		<h3 class="history-titre">Historique des transactions</h3>

		<!-- Tableau avec les transactions-->
		<!--
            type (moins ou plus en rouge ou vert) | emetteur | recepteur | description | date | montant
        -->

		<div class="mt-4 flex justify-end gap-2">
			<button
				class="pillule-bouton-history pillule-bouton-history-noir bouton-noir-settings flex items-center transition"
			>
				<span class="texte-bouton-history texte-bouton-history-noir">Fermer</span>
			</button>
		</div>
	</div>
</div>
