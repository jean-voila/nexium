<script>
	export let showSettingsModal = false;

	import { blur } from 'svelte/transition';
	import { Circle, Download } from 'lucide-svelte';
	import { Upload } from 'lucide-svelte';
	import { CheckCheck } from 'lucide-svelte';
	import { CircleOff } from 'lucide-svelte';
	import { CircleAlert } from 'lucide-svelte';
	import { open } from '@tauri-apps/plugin-dialog';
	import { save } from '@tauri-apps/plugin-dialog';
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

	let port = '';
	let url = '';
	let login = '';

	let oauth_connected = false;

	let pub_key = '';
	let priv_key = '';

	let gitlab_classic_token = '';
	let gitlab_oauth_token = '';

	let errorMessage = '';
	let isValidating = false;

	let isGenerating = false;
	let generationMessage = '';

	// Default password need to change it so the user can set it manually
	// (use new modal to ask for password)
	let password = '1234';

	async function getGitlabOauthToken() {
		try {
			gitlab_classic_token = '';
			// a supprimer apres avoir mis en place le code qui donne le login utilisateur en dessous
			const response = await invoke('get_gitlab_oauth_url');
			/*
			const response: { token: string; login?: string } = await invoke('get_gitlab_oauth_token');
			gitlab_oauth_token = response.token;
			if (response.login !== undefined) {
				login = response.login;
				oauth_connected = true;
			}
			*/
			oauth_connected = true; // a supprimer apres avoir mis en place le code qui donne le login utilisateur
		} catch (error) {
			errorMessage = String(error);
		}
	}

	async function validateSettings() {
		if (pub_key === '' || priv_key === '') {
			errorMessage = 'KeyPairError';
			return false;
		}

		let sentToken = '';
		let tokenType = '';

		if (gitlab_oauth_token === '' && gitlab_classic_token === '') {
			errorMessage = 'No Token.';
			return false;
		} else if (gitlab_oauth_token !== '' && gitlab_classic_token !== '') {
			sentToken = gitlab_classic_token;
			tokenType = 'classic';
			oauth_connected = false;
		} else if (gitlab_oauth_token !== '') {
			sentToken = gitlab_oauth_token;
			tokenType = 'oauth';
		} else {
			sentToken = gitlab_classic_token;
			tokenType = 'classic';
		}

		try {
			const response = await invoke('check_config_values', {
				port: String(port),
				url,
				login,
				gitlabtoken: sentToken,
				tokentypestring: tokenType
			});
			errorMessage = '';
			return true;
		} catch (error) {
			errorMessage = String(error);
			return false;
		}
	}

	async function loadConfigFromFile() {
		const result = await open({
			multiple: false,
			directory: false,
			title: 'Charger la configuration',
			filters: [
				{
					name: 'JSON',
					extensions: ['json']
				}
			]
		});
		if (result) {
			try {
				const response = await invoke('load_config_from_file', { pathString: result });

				port = response.port;
				url = response.url_server;
				login = response.user_login;
				pub_key = response.pub_key;
				priv_key = response.priv_key;
				gitlab_classic_token = response.gitlab_token;
			} catch (error) {
				errorMessage = String(error);
			}
		}
	}

	async function saveConfigToFile() {
		const path = await save({
			filters: [
				{
					name: 'config',
					extensions: ['json']
				}
			]
		});
		if (path) {
			try {
				await invoke('save_config_to_file', {
					pathString: path,
					port,
					url: url,
					login: login,
					pubKey: pub_key,
					privKey: priv_key,
					gitlabToken: gitlab_classic_token
				});
			} catch (error) {
				errorMessage = String(error);
			}
		}
	}

	async function generateKeyPair() {
		try {
			const [pubKey, privKey] = await invoke('keypair_generation', { login: login, password });
			pub_key = pubKey;
			priv_key = privKey;
		} catch (error) {
			errorMessage = String(error);
		}
	}
	async function sendGpgKey() {
		let sentToken = '';
		let tokenType = '';

		if (gitlab_oauth_token === '' && gitlab_classic_token === '') {
			errorMessage = 'No Token.';
			return false;
		} else if (gitlab_oauth_token !== '' && gitlab_classic_token !== '') {
			sentToken = gitlab_classic_token;
			tokenType = 'classic';
			oauth_connected = false;
		} else if (gitlab_oauth_token !== '') {
			sentToken = gitlab_oauth_token;
			tokenType = 'oauth';
		} else {
			sentToken = gitlab_classic_token;
			tokenType = 'classic';
		}
		try {
			await invoke('send_gpg_key', {
				tokentypestring: tokenType,
				gitlabToken: sentToken,
				pubKey: pub_key
			});
		} catch (error) {
			errorMessage = String(error);
		}
	}

	async function handleKeyGeneration() {
		if (gitlab_oauth_token === '' && gitlab_classic_token === '') {
			errorMessage = 'Aucun token disponible.';
			return;
		}

		isGenerating = true;
		generationMessage = 'Génération de vos clés...';

		try {
			await generateKeyPair();
			generationMessage = 'Communication de vos clés à GitLab...';
			await sendGpgKey();
			generationMessage = '';
		} catch (e) {
			errorMessage = String(e);
			generationMessage = '';
		} finally {
			isGenerating = false;
		}
	}

	function saveGlobalSettings() {
		globalPort.set(port);
		globalUrl.set(url);
		globalLogin.set(login);
		globalGitlabToken.set(gitlab_classic_token);
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

			<div class="settings-item flex items-center justify-between gap-4">
				<div class="flex w-full max-w-xl flex-col gap-2">
					<input
						id="user-login"
						type="text"
						bind:value={login}
						class="input-field"
						placeholder="Login (prenom.nom)"
					/>
					<input
						id="gitlab-token"
						type="text"
						bind:value={gitlab_classic_token}
						class="input-field"
						placeholder="Token Gitlab"
					/>
				</div>

				<div class="flex flex-col items-center">
					<span class="mb-1 text-sm text-gray-500">ou</span>
					<button
						on:click={() => getGitlabOauthToken()}
						class="bouton bouton-gitlab flex items-center gap-2 px-4 py-2 transition {oauth_connected
							? 'pillule-bouton-gitlab-checked'
							: 'pillule-bouton-gitlab'}"
						disabled={oauth_connected}
					>
						{#if oauth_connected}
							<CheckCheck strokeWidth={3} class="icone-gitlab m-1" />
							<span class="texte-bouton-gitlab">Connecté</span>
						{:else}
							<img src="/gitlab.png" alt="GitLab" class="icone-gitlab" />
							<span class="texte-bouton-gitlab">Connexion</span>
						{/if}
					</button>
				</div>
			</div>

			<div class="settings-item flex-1">
				<label for="key_pair" class="nom-parametre">Paire de clés</label>

				<div class="flex items-center gap-4">
					<div class="flex items-center gap-2">
						{#if pub_key !== '' && priv_key !== ''}
							<CheckCheck strokeWidth={3.5} class="green-icon m-1" />
							<span class="keypair-status text-green-500">Clé définie</span>
						{:else}
							<CircleOff strokeWidth={3.5} class="red-icon m-1" />
							<span class="keypair-status text-red-500">Clé non définie</span>
						{/if}

						<button
							class="pillule-bouton-keypair bouton-keypair flex items-center transition"
							on:click={handleKeyGeneration}
							disabled={(gitlab_oauth_token === '' && gitlab_classic_token === '') ||
								isGenerating ||
								(pub_key !== '' && priv_key !== '') ||
								login.trim() === ''}
						>
							<span class="texte-bouton-keypair">Générer une clé</span>
						</button>
					</div>

					{#if isGenerating}
						<div class="flex items-center gap-2 text-sm text-gray-700">
							<svg class="h-5 w-5 animate-spin text-gray-600" fill="none" viewBox="0 0 24 24">
								<circle
									class="opacity-25"
									cx="12"
									cy="12"
									r="10"
									stroke="currentColor"
									stroke-width="4"
								></circle>
								<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8v8z"></path>
							</svg>
							<span>{generationMessage}</span>
						</div>
					{/if}
				</div>
			</div>

			<!-- Bouton Terminé -->
			<!-- Bouton Terminé et autres actions -->
			<div class="mt-6 flex items-center justify-between">
				<div class="flex gap-2">
					<button
						class="pillule-bouton-sauvercharger bouton bouton-settings flex items-center justify-center gap-2 p-2 transition"
						on:click={() => {
							loadConfigFromFile();
						}}
					>
						<Upload strokeWidth={3} class="icone-bouton-sauvercharger m-1" />
						<span class="texte-bouton-sauvercharger">Charger</span>
					</button>

					<button
						class="pillule-bouton-sauvercharger bouton bouton-settings flex items-center justify-center gap-2 p-2 transition"
						on:click={() => {
							saveConfigToFile();
						}}
					>
						<Download strokeWidth={3} class="icone-bouton-sauvercharger m-1" />
					</button>
				</div>

				<div class="flex flex-col items-end">
					<button
						on:click={() => {
							isValidating = true;

							validateSettings().then((isValid) => {
								isValidating = false;
								if (isValid) {
									saveGlobalSettings();
									showSettingsModal = false;
								}
							});
						}}
						class="pillule-bouton-settings bouton bouton-settings flex items-center transition"
						disabled={isValidating}
					>
						<span class="texte-bouton-settings">Terminé</span>
					</button>
					{#if isValidating}
						<div class="mt-1 flex items-center">
							<svg class="m-1 h-5 w-5 animate-spin text-gray-600" fill="none" viewBox="0 0 24 24">
								<circle
									class="opacity-25"
									cx="12"
									cy="12"
									r="10"
									stroke="currentColor"
									stroke-width="4"
								></circle>
								<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8v8z"></path>
							</svg>
						</div>
					{:else if errorMessage !== ''}
						<div class="mt-1 flex items-center">
							<CircleAlert strokeWidth={3.5} class="red-icon m-1" />
							<span class="error-message centered-error">{errorMessage}</span>
						</div>
					{/if}
				</div>
			</div>
		</div>
	</div>
{/if}
