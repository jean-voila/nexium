<script>
	import { fly, fade } from 'svelte/transition';
	import { open } from '@tauri-apps/plugin-dialog';
	import { invoke } from '@tauri-apps/api/core';
	import { userBalance, isConfigSet } from '$lib/stores/settings.js';

	let { oncancel } = $props();

	let sender_login = $state('');
	let amount = $state('');
	let description = $state('');
	let fees = $state('');
	let invoice_file_extension = '';
	invoke('get_invoice_extension').then((ext) => {
		invoice_file_extension = ext;
	});
	function handleClose() {
		oncancel?.();
	}

	let totalFees = '';
	let isUserLoginValid = $state(false);

	async function checkDestinataireLogin() {
		try {
			isUserLoginValid = await invoke('check_user_login', { login: sender_login });
		} catch (e) {
			isUserLoginValid = false;
		}
	}
	async function handleLoadFile() {
		const path = await open({
			title: 'Choisir le fichier de la facture',
			multiple: false,
			directory: false,
			save: false,
			filters: [{ name: 'Nexium Invoice', extensions: [invoice_file_extension] }]
		});
		if (!path) {
			return;
		}
		try {
			console.log('Path:', path);
			const result = await invoke('load_invoice_from_file', { pathString: path });

			sender_login = result.sender_login;
			amount = result.amount;
			description = result.description;
			console.log('Invoice loaded');
		} catch (e) {
			console.error('Error loading invoice:', e);
		}
	}
	async function handleSend() {
		if (!amount || !fees) {
			console.error('Montant ou frais manquant');
			return;
		}
		// ONLY ALLOW NUMERIC CHARACTERS
		if (!/^\d*\.?\d*$/.test(amount) || !/^\d*\.?\d*$/.test(fees)) {
			console.error('Montant ou frais contient des caractères non numériques');
			return;
		}
		const invoice = {
			sender_login,
			amount,
			description
		};
		handleClose();
	}
</script>

<div class="transaction-modal" transition:fade={{ duration: 200 }}>
	<div class="transaction-modal-content" transition:fly={{ y: 30, duration: 200 }}>
		<h3 class="password-titre">Nouvelle transaction</h3>
		<p class="password-texte"></p>

		<div class="settings-item">
			<div class="flex flex-col gap-4">
				<div class="w-full">
					<label for="destinataire" class="nom-parametre block">Destinataire</label>
					<input
						id="destinataire"
						type="text"
						bind:value={sender_login}
						oninput={checkDestinataireLogin}
						class="input-field w-full"
						placeholder="Login du destinataire"
					/>
				</div>
				<div class="flex flex-col gap-4 md:flex-row">
					<div class="w-full md:w-1/2">
						<label for="montant" class="nom-parametre block">Montant</label>
						<div class="flex items-center gap-2">
							<input
								id="montant"
								type="text"
								inputmode="decimal"
								pattern="[0-9]*"
								bind:value={amount}
								class="input-field flex-1"
								placeholder="0.00"
							/>
							<span class="text-sm text-gray-500">NXM</span>
						</div>
					</div>
					<div class="w-full md:w-1/2">
						<label for="fees" class="nom-parametre block">Frais</label>
						<div class="flex items-center gap-2">
							<input
								id="fees"
								type="text"
								bind:value={fees}
								class="input-field flex-1"
								placeholder="0.00"
							/>
							<span class="text-sm text-gray-500">µNXM / o</span>
						</div>
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
					rows="7"
				></textarea>
			</div>
		</div>
		<div class="mt-6 flex items-center justify-end">
			<div class="flex gap-2">
				<button
					class="pillule-bouton-settings pillule-bouton-password-blanc bouton-noir-settings flex items-center transition"
					onclick={handleLoadFile}
				>
					<span class="texte-bouton-settings texte-bouton-password-blanc">Importer</span>
				</button>
				<button
					class="pillule-bouton-settings pillule-bouton-password-blanc bouton-noir-settings flex items-center transition"
					onclick={handleClose}
					><span class="texte-bouton-settings texte-bouton-password-blanc">Annuler</span>
				</button>
				<button
					onclick={handleSend}
					class="pillule-bouton-settings bouton-noir-settings flex items-center transition"
					disabled={isUserLoginValid}
				>
					<span class="texte-bouton-settings">Envoyer</span>
				</button>
			</div>
		</div>
	</div>
</div>
