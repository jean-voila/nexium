<script>
	import { fly, fade } from 'svelte/transition';
	import { open } from '@tauri-apps/plugin-dialog';
	import { invoke } from '@tauri-apps/api/core';
	import { globalConfig } from '$lib/stores/settings.js';

	let { oncancel } = $props();

	let receiver = $state('');
	let amount = $state('');
	let description = $state('');
	let fees = $state('');
	let copied = false;

	let tooltipX = $state(0);
	let tooltipY = $state(0);
	let showMessage = $state('');

	let invoice_file_extension = '';
	invoke('get_invoice_extension').then((ext) => {
		invoice_file_extension = ext;
	});
	function handleClose() {
		oncancel?.();
	}

	let totalFees = '';
	let isUserLoginValid = $state(false);

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

			receiver = result.sender_login;
			amount = result.amount;
			description = result.description;
		} catch (e) {}
	}
	async function handleSend() {
		const transaction = {
			receiver: receiver,
			amount: amount,
			description: description,
			fees: fees
		};

		try {
			let send_transaction_result = await invoke('send_transaction', {
				transaction: transaction,
				globalConfig: $globalConfig
			});
			handleClose();
		} catch (e) {
			console.error('Error sending transaction:', e);
		}
	}
</script>

<div class="transaction-modal" transition:fade={{ duration: 200 }}>
	<div class="transaction-modal-content" transition:fly={{ y: 30, duration: 200 }}>
		<h3 class="transaction-titre">Nouvelle transaction</h3>

		<div class="settings-item">
			<div class="flex flex-col gap-4">
				<div class="w-full">
					<label for="destinataire" class="nom-parametre block">Destinataire</label>
					<input
						id="destinataire"
						type="text"
						bind:value={receiver}
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
								class="input-field-fees flex-1"
								placeholder="0.00"
								disabled={$globalConfig.is_testnet}
								onmouseenter={() => (showMessage = 'Le serveur est en testnet')}
								onmouseleave={() => (showMessage = '')}
								onmousemove={(e) => {
									tooltipX = e.clientX;
									tooltipY = e.clientY;
								}}
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
			<div class="flex w-full">
				<div class="flex flex-1 justify-start">
					<button
						class="pillule-bouton-password pillule-bouton-password-blanc bouton-noir-settings flex items-center transition"
						onclick={handleLoadFile}
					>
						<span class="texte-bouton-password texte-bouton-password-blanc">Importer</span>
					</button>
				</div>
				<div class="flex justify-end gap-2">
					<button
						class="pillule-bouton-password pillule-bouton-password-blanc bouton-noir-settings flex items-center transition"
						onclick={handleClose}
					>
						<span class="texte-bouton-password texte-bouton-password-blanc">Annuler</span>
					</button>
					<button
						class="pillule-bouton-password pillule-bouton-password-noir bouton-noir-settings flex items-center transition"
						onclick={handleSend}
					>
						<span class="texte-bouton-password texte-bouton-password-noir">Envoyer</span>
					</button>
				</div>
			</div>
		</div>
	</div>
</div>

{#if showMessage}
	<div class="testnet-tooltip" style="top: {tooltipY}px; left: {tooltipX}px;">
		<span
			class="absolute"
			class:translate-y-0={!copied}
			class:-translate-y-3={copied}
			class:opacity-100={!copied}
			class:opacity-0={copied}
		>
			{showMessage}
		</span>
	</div>
{/if}
