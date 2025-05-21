<script>
	import { createEventDispatcher } from 'svelte';
	let password = '';
	const dispatch = createEventDispatcher();
	function submit() {
		dispatch('submit', password);
	}
	function cancel() {
		dispatch('cancel');
	}
</script>

<div class="settings-modal">
	<div class="settings-modal-content">
		<h3 class="settings-titre mb-4">Entrez votre mot de passe</h3>
		<input type="password" bind:value={password} class="input-field" />
		<div class="mt-4 flex justify-end gap-4">
			<button class="pillule-bouton-secondaire texte-bouton-settings transition" on:click={cancel}>
				Annuler
			</button>
			<button class="pillule-bouton-principal texte-bouton-settings transition" on:click={submit}>
				Valider
			</button>
		</div>
	</div>
</div>
