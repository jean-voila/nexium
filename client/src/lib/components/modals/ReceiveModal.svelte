<script>
	import { fly, fade } from 'svelte/transition';
	import { save } from '@tauri-apps/plugin-dialog';
	import { invoke } from '@tauri-apps/api/core';
	import { X, Download } from 'lucide-svelte';
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

	function handleClose() {
		oncancel?.();
	}

	function handleMontantChange() {
		montant = montant.trim();
		validateInvoice();
	}

	function handleDescriptionChange() {
		validateInvoice();
	}

	async function validateInvoice() {
		const invoice = {
			amount: montant,
			description: description,
			sender_login: $globalConfig.user_login
		};

		try {
			await invoke('check_invoice_values', { invoice });
			validationError.set(false);
		} catch (e) {
			validationError.set(true);
		}
	}

	async function handleExport() {
		if ($validationError !== false) return;
		
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
				await invoke('save_facture_to_file', { pathString: path, invoice });
				handleClose();
			} catch (e) {}
		}
	}
</script>

<div class="modal-backdrop" transition:fade={{ duration: 200 }}>
	<div class="modal-container" transition:fly={{ y: 30, duration: 200 }}>
		<div class="modal-header">
			<h3 class="modal-title">Nouvelle facture</h3>
			<button class="modal-close" onclick={handleClose}>
				<X size={18} />
			</button>
		</div>

		<div class="modal-body">
			<!-- Amount -->
			<div class="form-group">
				<label for="montant" class="form-label">Montant</label>
				<div class="input-with-suffix">
					<input
						id="montant"
						type="text"
						inputmode="decimal"
						pattern="[0-9]*"
						bind:value={montant}
						oninput={handleMontantChange}
						class="form-input text-right"
						placeholder="0.00"
					/>
					<span class="input-suffix">NXM</span>
				</div>
			</div>

			<!-- Description -->
			<div class="form-group">
				<label for="description" class="form-label">Description</label>
				<textarea
					id="description"
					bind:value={description}
					oninput={handleDescriptionChange}
					class="form-input form-textarea"
					placeholder="Description (facultative)"
					maxlength="256"
					rows="3"
				></textarea>
			</div>
		</div>

		<div class="modal-footer">
			<button class="btn btn-ghost" onclick={handleClose}>
				Annuler
			</button>
			<button
				class="btn btn-filled"
				onclick={handleExport}
				disabled={$validationError}
			>
				<Download size={16} />
				Exporter
			</button>
		</div>
	</div>
</div>
