<script>
	import { fly, fade } from 'svelte/transition';
	import { open } from '@tauri-apps/plugin-dialog';
	import { invoke } from '@tauri-apps/api/core';
	let { oncancel } = $props();
	let destinataire = $state('');
	let montant = $state('');
	let description = $state('');
	let balance = $state(0);

	let invoice = $state(null);

	let errorMessage = $state('');

	function handleCancel() {
		oncancel?.();
	}

	function checkGitlabBalance() {
		return true;
	}

	async function checkDestinataireLogin() {
		// appel a la fonction qui verifie si le login est valide
	}

	async function handleLoadFile() {
		errorMessage = '';
		const path = await open({
			title: 'Choisir le fichier de la facture',
			multiple: false,
			directory: false,
			save: false,
			filters: [{ name: 'JSON', extensions: ['json'] }]
		});
		if (!path) {
			errorMessage = 'Aucun chemin sélectionné.';
			return;
		}
		try {
			const result = await invoke('load_facture_from_file', { pathString: path });
			if (result !== '') {
				errorMessage = result;
				return;
			}
			invoice = result;
			destinataire = result.destinataire;
			montant = result.montant;
			description = result.description;
		} catch (e) {
			errorMessage = `${e}`;
		}
	}
	async function handleSend() {
		errorMessage = '';
		if (montant === '') {
			errorMessage = 'Montant invalide.';
			return;
		}
		if (destinataire === '') {
			errorMessage = 'Login du destinataire invalide.';
			return;
		}
		if (!checkGitlabBalance()) {
			errorMessage = 'Solde insuffisant.';
			return;
		}

		try {
			const result = await invoke('send_facture', {
				destinataire,
				montant,
				description
			});
			if (result !== '') {
				errorMessage = result;
				return;
			}
			handleCancel();
		} catch (e) {
			errorMessage = `${e}`;
		}
	}
</script>

<div class="transaction-modal" transition:fade={{ duration: 200 }}>
	<div class="transaction-modal-content" transition:fly={{ y: 30, duration: 200 }}>
		<h3 class="password-titre">Nouvelle transaction</h3>
		<p class="password-texte"></p>

		<div class="settings-item">
			<div class="flex flex-wrap gap-4 md:flex-nowrap">
				<div class="flex-1">
					<label for="destinataire" class="nom-parametre block">Destinataire</label>
					<input
						id="destinataire"
						type="text"
						bind:value={destinataire}
						oninput={checkDestinataireLogin}
						class="input-field w-full"
						placeholder="Login du destinataire"
					/>
				</div>
				<div class="w-full md:w-1/2">
					<label for="montant" class="nom-parametre mb-1 block">Montant</label>
					<div class="flex items-center gap-2">
						<input
							id="montant"
							type="text"
							inputmode="decimal"
							pattern="[0-9]*"
							bind:value={montant}
							class="input-field flex-1"
							placeholder="0.00"
						/>
						<span class="text-sm text-gray-500">NXM</span>
					</div>
				</div>
			</div>
			<div class="mt-4">
				<label for="description" class="nom-parametre block">Description</label>
				<textarea
					id="description"
					bind:value={description}
					class="input-field w-full resize-none"
					placeholder="Description (facultative)"
					maxlength="256"
					rows="5"
				></textarea>
			</div>
		</div>
		<div class="mt-6 flex items-center justify-between">
			<div class="flex">
				<button
					class="pillule-bouton-sauvercharger flex items-center justify-center transition"
					onclick={handleLoadFile}
				>
					<span class="texte-bouton-settings">Importer</span>
				</button>
			</div>
			<div class="flex gap-2">
				<button
					class="pillule-bouton-settings pillule-bouton-password-blanc bouton-noir-settings flex items-center transition"
					onclick={handleCancel}
					><span class="texte-bouton-password texte-bouton-password-blanc">Annuler</span>
				</button>
				<button
					onclick={handleSend}
					class="pillule-bouton-settings bouton-noir-settings flex items-center transition"
				>
					<span class="texte-bouton-settings">Envoyer</span>
				</button>
			</div>
		</div>
	</div>
</div>
