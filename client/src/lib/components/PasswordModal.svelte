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
		<h3 class="settings-titre mb-4">Mot de passe</h3>
		<input type="password" bind:value={password} class="input-field" />
		<div class="mt-4 flex justify-end gap-4">
			<button
				class="pillule-bouton-settings bouton-noir-settings flex items-center transition"
				on:click={cancel}
				><span class="texte-bouton-settings">Annuler</span>
			</button>
			<button
				class="pillule-bouton-settings bouton-noir-settings flex items-center transition"
				on:click={submit}><span class="texte-bouton-settings">Valider</span></button
			>
		</div>
	</div>
</div>
