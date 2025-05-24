<script>
	let password = $state('');
	let showPassword = $state(false);
	import { fly, fade } from 'svelte/transition';
	let { onsubmit, oncancel } = $props();

	function submit() {
		onsubmit?.(password);
	}

	function cancel() {
		oncancel?.();
	}

	/**
	 * @param {{ key: string; }} event
	 */
	function handleKeydown(event) {
		if (event.key === 'Enter') {
			submit();
		}
	}
</script>

<div class="password-modal" transition:fade={{ duration: 200 }}>
	<div class="password-modal-content" transition:fly={{ y: 30, duration: 200 }}>
		<h3 class="password-titre">Mot de passe</h3>
		<p class="password-texte">Veuillez entrer le mot de passe de votre clé.</p>
		<div class="p-2">
			<div class=" flex flex-col gap-2">
				<input
					type={showPassword ? 'text' : 'password'}
					bind:value={password}
					class="input-field-password"
					placeholder="Mot de passe"
					onkeydown={handleKeydown}
				/>
			</div>
			<div class="checkbox-wrapper-13 mt-2 flex items-center">
				<input bind:checked={showPassword} id="c1-13" type="checkbox" />
				<label for="c1-13" class="password-label">Afficher le mot de passe</label>
			</div>
		</div>

		<div class="mt-4 flex justify-end gap-2">
			<button
				class="pillule-bouton-password pillule-bouton-password-noir bouton-noir-settings flex items-center transition"
				onclick={submit}
			>
				<span class="texte-bouton-password texte-bouton-password-noir">Valider</span>
			</button>
		</div>
	</div>
</div>
