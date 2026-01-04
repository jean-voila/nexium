<script>
	import { fly, fade } from 'svelte/transition';
	import { X, Eye, EyeOff } from 'lucide-svelte';
	
	let password = $state('');
	let confirmPassword = $state('');
	let showPassword = $state(false);
	
	let { onsubmit, oncancel } = $props();

	function submit() {
		onsubmit?.(password);
	}

	function cancel() {
		oncancel?.();
	}

	function handleKeydown(event) {
		if (event.key === 'Enter' && password === confirmPassword) {
			submit();
		}
	}
</script>

<div class="modal-backdrop" style="z-index: 120;" transition:fade={{ duration: 200 }}>
	<div class="modal-container" style="max-width: 400px;" transition:fly={{ y: 30, duration: 200 }}>
		<div class="modal-header">
			<h3 class="modal-title">Nouveau mot de passe</h3>
			<button class="modal-close" onclick={cancel}>
				<X size={18} />
			</button>
		</div>

		<div class="modal-body">
			<p class="text-muted text-sm mb-4">Créez un mot de passe sécurisé pour votre nouvelle clé.</p>
			
			<div class="form-group">
				<label for="password" class="form-label">Mot de passe</label>
				<div class="relative">
					<input
						id="password"
						type={showPassword ? 'text' : 'password'}
						bind:value={password}
						class="form-input"
						placeholder="Votre mot de passe"
						onkeydown={handleKeydown}
					/>
				</div>
			</div>
			
			<div class="form-group">
				<label for="confirm_password" class="form-label">Confirmation</label>
				<input
					id="confirm_password"
					type="password"
					bind:value={confirmPassword}
					class="form-input"
					placeholder="Confirmez le mot de passe"
					onkeydown={handleKeydown}
				/>
			</div>

			<div class="toggle-wrapper mt-2">
				<button
					class="toggle"
					class:active={showPassword}
					onclick={() => (showPassword = !showPassword)}
					aria-label="Afficher le mot de passe"
				>
					<div class="toggle-knob"></div>
				</button>
				<span class="toggle-label">Afficher le mot de passe</span>
			</div>
		</div>

		<div class="modal-footer">
			<button class="btn btn-ghost" onclick={cancel}>Annuler</button>
			<button
				class="btn btn-filled"
				onclick={submit}
				disabled={password !== confirmPassword || !password}
			>
				Valider
			</button>
		</div>
	</div>
</div>
