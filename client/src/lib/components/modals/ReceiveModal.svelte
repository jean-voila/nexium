<script>
	import { fly, fade } from 'svelte/transition';
	import { open } from '@tauri-apps/plugin-dialog';
	import { invoke } from '@tauri-apps/api/core';
	let { oncancel } = $props();

	let montant = $state('');
	let description = $state('');
	let errorMessage = $state('');
	let invoice = $state(null);
	function handleCancel() {
		oncancel?.();
	}

	async function handleExport() {
		errorMessage = '';
		const invoice = {
			montant: parseFloat(montant),
			description: description
		};
		const validationError = await invoke('check_invoice_values', { invoice });
		if (validationError !== '') {
			errorMessage = validationError;
			return;
		}
		const path = await open({
			title: "Choisir l'emplacement de sauvegarde",
			multiple: false,
			directory: false,
			save: true,
			defaultPath: 'facture.json',
			filters: [{ name: 'JSON', extensions: ['json'] }]
		});
		if (!path) {
			errorMessage = 'Aucun chemin sélectionné.';
			return;
		}
		try {
			const result = await invoke('save_facture_to_file', {
				invoice,
				pathString: path
			});
			if (result !== '') {
				errorMessage = result;
				return;
			}
			oncancel?.();
		} catch (e) {
			errorMessage = `${e}`;
		}
	}
</script>

<div class="transaction-modal" transition:fade={{ duration: 200 }}>
	<div class="transaction-modal-content" transition:fly={{ y: 30, duration: 200 }}>
		<h3 class="password-titre">Nouvelle facture</h3>
		<p class="password-texte"></p>

		<div class="settings-item">
			<div class="flex flex-wrap gap-4 md:flex-nowrap">
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
		</div>
		<div class="mt-6 flex justify-end">
			<div class="flex gap-2">
				<button
					class="pillule-bouton-settings pillule-bouton-password-blanc bouton-noir-settings flex items-center transition"
					onclick={handleCancel}
					><span class="texte-bouton-password texte-bouton-password-blanc">Annuler</span>
				</button>
				<button
					onclick={handleExport}
					class="pillule-bouton-settings bouton-noir-settings flex items-center transition"
				>
					<span class="texte-bouton-settings">Exporter</span>
				</button>
			</div>
		</div>
	</div>
</div>
