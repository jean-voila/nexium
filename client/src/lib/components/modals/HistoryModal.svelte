<script>
	import { Minus } from 'lucide-svelte';
	import { Plus } from 'lucide-svelte';

	import { onMount } from 'svelte';
	import { fade, fly } from 'svelte/transition';
	import { writable } from 'svelte/store';

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
</script>

<div class="history-modal" transition:fade={{ duration: 200 }}>
	<div class="history-modal-content" transition:fly={{ y: 30, duration: 200 }}>
		<h3 class="history-titre">Historique des transactions</h3>

		<!-- Tableau avec les transactions-->
		<!--
            type (moins ou plus en rouge ou vert) | emetteur | recepteur | description | date | montant
        -->

		<Table>
			<TableHead>
				<TableHeadCell>Product name</TableHeadCell>
				<TableHeadCell>Color</TableHeadCell>
				<TableHeadCell>Category</TableHeadCell>
				<TableHeadCell>Price</TableHeadCell>
			</TableHead>
			<TableBody>
				<TableBodyRow>
					<TableBodyCell>Apple MacBook Pro 17"</TableBodyCell>
					<TableBodyCell>Sliver</TableBodyCell>
					<TableBodyCell>Laptop</TableBodyCell>
					<TableBodyCell>$2999</TableBodyCell>
				</TableBodyRow>
				<TableBodyRow>
					<TableBodyCell>Microsoft Surface Pro</TableBodyCell>
					<TableBodyCell>White</TableBodyCell>
					<TableBodyCell>Laptop PC</TableBodyCell>
					<TableBodyCell>$1999</TableBodyCell>
				</TableBodyRow>
				<TableBodyRow>
					<TableBodyCell>Magic Mouse 2</TableBodyCell>
					<TableBodyCell>Black</TableBodyCell>
					<TableBodyCell>Accessories</TableBodyCell>
					<TableBodyCell>$99</TableBodyCell>
				</TableBodyRow>
			</TableBody>
		</Table>

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
