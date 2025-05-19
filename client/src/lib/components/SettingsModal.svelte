<script>
	export let showSettingsModal = false;

	import { blur } from 'svelte/transition';
	import { Download } from 'lucide-svelte';
	import { Upload } from 'lucide-svelte';
	import {
		globalPort,
		globalUrl,
		globalLogin,
		globalGitlabToken,
		globalPubKey,
		globalPrivKey
	} from '$lib/stores/settings.js';

	import { invoke } from '@tauri-apps/api/core';
	import { error } from '@sveltejs/kit';

	let port = 0;
	let url = '';
	let login = '';
	let gitlabtoken = '';
	let pub_key = '';
	let priv_key = '';

	let errorMessage = '';
	let isSettingsValid = false;

	async function validateSettings() {
		//if (pub_key === '' || priv_key === '') {
		//	errorMessage = 'KeyPairError';
		//	return false;
		//}

		try {
			const response = await invoke('check_config_values', {
				port,
				url,
				login,
				gitlabtoken
			});
			errorMessage = '';
			return true;
		} catch (error) {
			errorMessage = String(error);
			return false;
		}
	}

	$: validateOnChange();
	async function validateOnChange() {
		console.log('Validating settings...');
		isSettingsValid = await validateSettings();
	}

	function saveSettings() {
		globalPort.set(port);
		globalUrl.set(url);
		globalLogin.set(login);
		globalGitlabToken.set(gitlabtoken);
		globalPubKey.set(pub_key);
		globalPrivKey.set(priv_key);
	}
</script>

{#if showSettingsModal}
	<div class="settings-modal" transition:blur={{ amount: 5, duration: 300 }}>
		<div class="settings-modal-content">
			<!-- Titre -->
			<h2 class="settings-titre">Paramètres</h2>

			<!-- Liste des paramètres -->
			<div class=" settings-item settings-row flex gap-4">
				<div class=" flex-1">
					<label for="server-port" class="nom-parametre">Port</label>
					<input
						id="server-port"
						type="number"
						bind:value={port}
						class="input-field"
						placeholder="3000"
					/>
				</div>
				<div class="flex-[4]">
					<label for="server-url" class="nom-parametre">URL du serveur</label>
					<input
						id="server-url"
						type="text"
						bind:value={url}
						class="input-field"
						placeholder="https://server.nexium.com"
					/>
				</div>
			</div>

			<div class="settings-item flex-1">
				<label for="user-login" class="nom-parametre">Login</label>
				<input
					id="user-login"
					type="text"
					bind:value={login}
					class="input-field"
					placeholder="prenom.nom"
				/>
			</div>

			<div class="settings-item flex-1">
				<label for="gitlab-token" class="nom-parametre">Token Gitlab</label>
				<input
					id="gitlab-token"
					type="text"
					bind:value={gitlabtoken}
					class="input-field"
					placeholder="•••••"
				/>
			</div>

			<div class="settings-item flex-1">
				<label for="key_pair" class="nom-parametre">Paire de clés</label>
				<!-- If key_pair defined, green check and "Clé définie" + button for "Générer une clé"
                    else, red cross and "Clé non définie" + button for "Changer la clé" -->
				{#if pub_key !== '' && priv_key !== ''}
					<div class="flex items-center gap-2">
						<span class="keypair-status text-green-500">✔️</span>
						<span class="keypair-status text-green-500">Clé définie</span>
						<button class="pillule-bouton-keypair bouton-keypair flex items-center transition">
							<span class="texte-bouton-keypair">Changer la clé</span>
						</button>
					</div>
				{:else}
					<div class="flex items-center gap-2">
						<span class="keypair-status text-red-500">❌</span>
						<span class="keypair-status text-red-500">Clé non définie</span>
						<button class="pillule-bouton-keypair bouton-keypair flex items-center transition">
							<span class="texte-bouton-keypair">Générer une clé</span>
						</button>
					</div>
				{/if}
			</div>

			<!-- Bouton Terminé -->
			<!-- Bouton Terminé et autres actions -->
			<div class="mt-6 flex items-center justify-between">
				<div class="flex gap-2">
					<button
						class="pillule-bouton-sauvercharger bouton bouton-settings flex items-center justify-center gap-2 p-2 transition"
						on:click={() => {
							saveSettings();
						}}
					>
						<Upload strokeWidth={3} class="icone-bouton-sauvercharger m-1" />
						<span class="texte-bouton-sauvercharger">Charger</span>
					</button>

					<button
						class="pillule-bouton-sauvercharger bouton bouton-settings flex items-center justify-center gap-2 p-2 transition"
						on:click={() => {
							saveSettings();
						}}
					>
						<Download strokeWidth={3} class="icone-bouton-sauvercharger m-1" />
					</button>
				</div>
				<div class="flex flex-col items-end">
					<button
						on:click={() => {
							validateOnChange();
						}}
						class="pillule-bouton-settings bouton bouton-settings flex items-center transition"
					>
						<span class="texte-bouton-settings">Check</span>
					</button>
				</div>

				<div class="flex flex-col items-end">
					<button
						on:click={() => {
							saveSettings();
							showSettingsModal = false;
						}}
						disabled={!isSettingsValid}
						class="pillule-bouton-settings bouton bouton-settings flex items-center transition"
					>
						<span class="texte-bouton-settings">Terminé</span>
					</button>
					{#if errorMessage !== ''}
						<div class="error-message">{errorMessage}</div>
					{/if}
				</div>
			</div>
		</div>
	</div>
{/if}
