<script>
	import { fly, fade } from 'svelte/transition';
	import { save } from '@tauri-apps/plugin-dialog';
	import { invoke } from '@tauri-apps/api/core';
	import { globalConfig } from '$lib/stores/settings';
	import { writable } from 'svelte/store';
	let { oncancel } = $props();

	let montant = $state('');
	let description = $state('');
	let validationError = writable(true);

	let invoice_file_extension = '';
	invoke('get_invoice_extension').then((ext) => {
		invoice_file_extension = ext;
	});

	let invoice = $state(null);

	function handleClose() {
		oncancel?.();
	}

	function handleMontantChange() {
		montant = montant.trim();
		validateInvoice();
	}

	function handleDescriptionChange() {
		description = description.trim();
		validateInvoice();
	}

	async function validateInvoice() {
		const invoice = {
			amount: montant,
			description: description,
			sender_login: $globalConfig.user_login
		};

		try {
			const result = await invoke('check_invoice_values', { invoice });
			validationError.set(false);
		} catch (e) {
			validationError.set(true);
		}
	}

	async function handleExport() {
		if ($validationError !== false) {
			return;
		}
		const invoice = {
			amount: montant,
			description: description,
			sender_login: $globalConfig.user_login
		};
		const path = await save({
			filters: [{ name: 'Nexium Invoice', extensions: [invoice_file_extension] }]
		});
		if (path) {
			try {
				const result = await invoke('save_facture_to_file', {
					pathString: path,
					invoice
				});
				handleClose();
			} catch (e) {}
		}
	}
</script>

<div class="transaction-modal" transition:fade={{ duration: 200 }}>
	<div class="transaction-modal-content" transition:fly={{ y: 30, duration: 200 }}>
		<h3 class="transaction-titre mb-1">Nouvelle facture</h3>

		<div class="settings-item">
			<div class="flex flex-wrap gap-4 md:flex-nowrap">
				<div class="md:w-1/ w-1/3">
					<label for="montant" class="nom-parametre block">Montant</label>
					<div class="flex items-center gap-2">
						<input
							id="montant"
							type="text"
							inputmode="decimal"
							pattern="[0-9]*"
							bind:value={montant}
							oninput={handleMontantChange}
							class="input-field input- input-field-montant flex-1"
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
					oninput={handleDescriptionChange}
					class="input-field w-full resize-none"
					placeholder="Description (facultative)"
					maxlength="256"
					rows="2"
				></textarea>
			</div>
		</div>
		<div class=" flex justify-end">
			<div class="flex gap-2">
				<button
					class="pillule-bouton-password pillule-bouton-password-blanc bouton-noir-settings flex items-center transition"
					onclick={handleClose}
					><span class="texte-bouton-password texte-bouton-password-blanc">Annuler</span>
				</button>
				<button
					class="pillule-bouton-password pillule-bouton-password-noir bouton-noir-settings flex items-center transition"
					onclick={handleExport}
					disabled={$validationError}
				>
					<span class="texte-bouton-password texte-bouton-password-noir">Exporter</span>
				</button>
			</div>
		</div>
	</div>
</div>
