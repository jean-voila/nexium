<script>
	import { fly, fade } from 'svelte/transition';
	import { open } from '@tauri-apps/plugin-dialog';
	import { invoke } from '@tauri-apps/api/core';
	import { X, Star, Upload, Send } from 'lucide-svelte';
	import { globalConfig, serverPublicKey } from '$lib/stores/settings.js';
	import { selectedContact } from '$lib/stores/contacts.js';
	import { writable, get } from 'svelte/store';
	import { onMount } from 'svelte';
	import Spinner from '$lib/components/Spinner.svelte';

	let { oncancel } = $props();

	let receiver = $state('');
	let searchResults = $state([]);
	let favoriteContacts = $state([]);
	let showSuggestions = $state(false);
	let amount = $state('');
	let description = $state('');
	let fees = $state('0');
	let estimatedFee = $state('0');
	let totalCost = $state('0');

	let tooltipX = $state(0);
	let tooltipY = $state(0);
	let showTooltip = $state('');

	let invoice_file_extension = '';
	invoke('get_invoice_extension').then((ext) => {
		invoice_file_extension = ext;
	});

	function handleClose() {
		selectedContact.set('');
		oncancel?.();
	}

	let validationError = writable(true);
	let isSending = $state(false);

	async function updateFeeCost() {
		try {
			const hasDescription = description.trim().length > 0;
			const feeResult = await invoke('calculate_transaction_fee', {
				fees: fees,
				hasDescription: hasDescription
			});
			estimatedFee = feeResult;
			
			const amountNum = parseFloat(amount) || 0;
			const feeNum = parseFloat(feeResult) || 0;
			totalCost = (amountNum + feeNum).toFixed(6);
		} catch (e) {
			estimatedFee = '0';
			totalCost = amount || '0';
		}
	}

	function handleMontantChange() {
		amount = amount.trim();
		checkTransaction();
		updateFeeCost();
	}

	function handleFeesChange() {
		updateFeeCost();
		checkTransaction();
	}

	function handleDescriptionChange() {
		updateFeeCost();
		checkTransaction();
	}

	async function loadFavoriteContacts() {
		try {
			const contacts = await invoke('get_favorite_contacts');
			favoriteContacts = contacts.map((c) => ({
				login: c.login,
				nickname: c.nickname,
				favorite: c.favorite
			}));
		} catch (e) {
			favoriteContacts = [];
		}
	}

	async function handleReceiverChange() {
		receiver = receiver.trim();
		checkTransaction();

		if (receiver.length > 0) {
			try {
				const results = await invoke('search_first_users', {
					config: $globalConfig,
					search: receiver
				});
				const contactResults = await invoke('search_contacts', { query: receiver });
				const contactLogins = contactResults.map((c) => c.login);
				const allResults = [...new Set([...contactLogins, ...results])];
				searchResults = allResults;
				showSuggestions = allResults.length > 0;
			} catch (e) {
				searchResults = [];
				showSuggestions = false;
			}
		} else {
			searchResults = [];
			showSuggestions = false;
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
		if (!path) return;
		
		try {
			const result = await invoke('load_invoice_from_file', { pathString: path });
			receiver = result.sender_login;
			amount = result.amount;
			description = result.description;
			checkTransaction();
		} catch (e) {}
	}

	async function checkTransaction() {
		const classic_transaction_sent = {
			receiver: receiver,
			amount: amount,
			description: description,
			fees: fees
		};

		try {
			await invoke('check_send_transaction', {
				config: $globalConfig,
				transaction: classic_transaction_sent
			});
			validationError.set(false);
		} catch (e) {
			validationError.set(true);
		}
	}

	async function handleSend() {
		const classic_transaction_sent = {
			receiver: receiver,
			amount: amount,
			description: description,
			fees: fees
		};

		isSending = true;
		try {
			await invoke('mark_contact_used', { login: receiver }).catch(() => {});
			await invoke('send_transaction', {
				serverPubkey: $serverPublicKey,
				config: $globalConfig,
				transaction: classic_transaction_sent
			});
			handleClose();
		} catch (e) {
			console.error("Erreur lors de l'envoi de la transaction:", e);
		} finally {
			isSending = false;
		}
	}

	onMount(() => {
		loadFavoriteContacts();
		const preselected = get(selectedContact);
		if (preselected) {
			receiver = preselected;
			checkTransaction();
		}
	});
</script>

<div class="modal-backdrop" transition:fade={{ duration: 200 }}>
	<div class="modal-container" transition:fly={{ y: 30, duration: 200 }}>
		<div class="modal-header">
			<h3 class="modal-title">Nouvelle transaction</h3>
			<button class="modal-close" onclick={handleClose}>
				<X size={18} />
			</button>
		</div>

		<div class="modal-body">
			<!-- Favorite Contacts -->
			{#if favoriteContacts.length > 0}
				<div class="chip-section">
					<div class="chip-section-header">
						<Star size={14} fill="currentColor" />
						<span>Contacts favoris</span>
					</div>
					<div class="chip-list">
						{#each favoriteContacts.slice(0, 4) as contact}
							<button
								class="chip"
								class:active={receiver === contact.login}
								onclick={() => {
									receiver = contact.login;
									checkTransaction();
								}}
							>
								{contact.nickname || contact.login}
							</button>
						{/each}
					</div>
				</div>
			{/if}

			<!-- Receiver Input -->
			<div class="form-group relative">
				<label for="destinataire" class="form-label">Destinataire</label>
				<input
					id="destinataire"
					type="text"
					bind:value={receiver}
					oninput={handleReceiverChange}
					class="form-input form-input-mono"
					placeholder="Login du destinataire"
				/>
				{#if showSuggestions}
					<div class="suggestions-dropdown">
						{#each searchResults as user}
							<button
								type="button"
								class="suggestion-item"
								onclick={() => {
									receiver = user;
									showSuggestions = false;
									checkTransaction();
								}}
							>
								{user}
							</button>
						{/each}
					</div>
				{/if}
			</div>

			<!-- Amount & Fees Row -->
			<div class="form-row">
				<div class="form-group">
					<label for="montant" class="form-label">Montant</label>
					<div class="input-with-suffix">
						<input
							id="montant"
							type="text"
							inputmode="decimal"
							pattern="[0-9]*"
							bind:value={amount}
							oninput={handleMontantChange}
							class="form-input text-right"
							placeholder="0.00"
						/>
						<span class="input-suffix">NXM</span>
					</div>
				</div>
				<div class="form-group">
					<label for="fees" class="form-label">Frais</label>
					<div class="input-with-suffix">
						<input
							id="fees"
							type="text"
							bind:value={fees}
							oninput={handleFeesChange}
							class="form-input text-right"
							placeholder="0"
						/>
						<span class="input-suffix">µNXM/o</span>
					</div>
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
					rows="2"
				></textarea>
			</div>

			<!-- Total Cost Summary -->
			{#if parseFloat(amount) > 0 || parseFloat(fees) > 0}
				<div class="cost-summary">
					<div class="cost-row">
						<span class="cost-label">Montant:</span>
						<span class="cost-value">{amount || '0'} NXM</span>
					</div>
					{#if parseFloat(fees) > 0}
						<div class="cost-row">
							<span class="cost-label">Frais estimés:</span>
							<span class="cost-value">{estimatedFee} NXM</span>
						</div>
					{/if}
					<div class="cost-row cost-total">
						<span class="cost-label">Coût total:</span>
						<span class="cost-value">{totalCost} NXM</span>
					</div>
				</div>
			{/if}
		</div>

		<div class="modal-footer">
			<button class="btn btn-ghost" onclick={handleLoadFile} disabled={isSending}>
				<Upload size={16} />
				Importer
			</button>
			<div class="flex-1"></div>
			<button class="btn btn-ghost" onclick={handleClose} disabled={isSending}>
				Annuler
			</button>
			<button
				class="btn btn-filled"
				onclick={handleSend}
				disabled={isSending || $validationError}
			>
				{#if isSending}
					<Spinner />
				{:else}
					<Send size={16} />
				{/if}
				Envoyer
			</button>
		</div>
	</div>
</div>

{#if showTooltip}
	<div class="tooltip" style="top: {tooltipY}px; left: {tooltipX}px;">
		{showTooltip}
	</div>
{/if}
