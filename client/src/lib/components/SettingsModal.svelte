<script>
	export let showSettingsModal = false;

	import Spinner from '$lib/components/Spinner.svelte';

	import { blur } from 'svelte/transition';
	import { Circle, Download } from 'lucide-svelte';
	import { Upload } from 'lucide-svelte';
	import { CheckCheck } from 'lucide-svelte';
	import { CircleOff } from 'lucide-svelte';
	import { CircleAlert } from 'lucide-svelte';
	import { open } from '@tauri-apps/plugin-dialog';
	import { save } from '@tauri-apps/plugin-dialog';
	import { Unplug } from 'lucide-svelte';
	import { globalConfig } from '$lib/stores/settings.js';

	import { invoke } from '@tauri-apps/api/core';
	import { error } from '@sveltejs/kit';
	import { get } from 'svelte/store';

	// create a copy of the global config store
	let config = get(globalConfig);

	let errorMessage = '';
	let isValidating = false;

	let isGenerating = false;
	let generationMessage = '';

	// (use new modal to ask for password)
	let password = '1234';

	async function getGitlabOauthToken() {
		try {
			const response = await invoke('get_gitlab_oauth_token');
			config.gitlab_token = response.token;
			config.gitlab_token_type = 'OAuth';
		} catch (error) {
			errorMessage = String(error);
		}
	}

	async function validateSettings() {
		try {
			const response = await invoke('check_config_values', {
				port: String(port),
				url,
				login,
				gitlabTokenType: gitlab_token,
				tokenType: gitlab_token_type,
				pubKey: pub_key,
				privKey: priv_key
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
				gitlab_token = response.gitlab_token;
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
					gitlabToken: gitlab_token,
					gitlabTokenType: gitlab_token_type
				});
			} catch (error) {
				errorMessage = String(error);
			}
		}
	}

	async function generateKeyPair() {
		try {
			const [pubKey, privKey] = await invoke('keypair_generation', { login, password });
			pub_key = pubKey;
			priv_key = privKey;
		} catch (error) {
			errorMessage = String(error);
		}
	}
	async function sendGpgKey() {
		try {
			await invoke('send_gpg_key', {
				gitlabTokenType: gitlab_token_type,
				gitlabToken: gitlab_token,
				pubKey: pub_key
			});
		} catch (error) {
			errorMessage = String(error);
		}
	}

	async function handleKeyGeneration() {
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

	function disconnectGitlabOauth() {
		gitlab_token = '';
		gitlab_token_type = 'classic';
	}

	async function getLoginFromToken() {
		try {
			const response = await invoke('get_login', {
				gitlabToken: gitlab_token,
				tokentypestring: gitlab_token_type
			});
			login = response.login;
		} catch (error) {
			errorMessage = String(error);
		}
	}

	let lastCheckedToken = '';
	$: if (gitlab_token && gitlab_token.length > 15 && gitlab_token !== lastCheckedToken) {
		lastCheckedToken = gitlab_token;
		getLoginFromToken();
	}

	function saveGlobalSettings() {
		globalPort.set(port);
		globalUrl.set(url);
		globalLogin.set(login);
		globalGitlabToken.set(gitlab_token);
		globalPubKey.set(pub_key);
		globalPrivKey.set(priv_key);
		globalGitlabTokenType.set(gitlab_token_type);
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
				<input
					id="gitlab-token"
					type="text"
					bind:value={gitlab_token}
					class="input-field"
					placeholder="Token Gitlab"
					disabled={gitlab_token_type === 'oauth'}
				/>

				<!-- "ou" -->
				<span class="ou">ou</span>

				<div class="flex flex-col items-center">
					<button
						on:click={() => {
							if (gitlab_token_type === 'oauth') {
								disconnectGitlabOauth();
							} else {
								getGitlabOauthToken();
								getLoginFromToken();
							}
						}}
						class=" bouton-gitlab group flex items-center gap-1 px-4 py-2 pl-3 transition {gitlab_token_type ===
						'oauth'
							? 'pillule-bouton-gitlab-checked'
							: 'pillule-bouton-gitlab'}"
					>
						{#if gitlab_token_type === 'oauth'}
							<span
								class="relative flex h-[24px] w-[140px] items-center justify-center transition-all duration-300"
							>
								<span
									class="absolute inset-0 flex items-center justify-center gap-2 opacity-100 transition-opacity duration-300 group-hover:opacity-0"
								>
									<CheckCheck strokeWidth={3} class="icone-gitlab" />
									<span class="texte-bouton-gitlab">Connecté</span>
								</span>
								<span
									class="absolute inset-0 flex items-center justify-center gap-2 text-black opacity-0 transition-opacity duration-300 group-hover:opacity-100"
								>
									<Unplug strokeWidth={3} class="icone-gitlab-deco" />
									<span class="texte-bouton-gitlab">Déconnexion</span>
								</span>
							</span>
						{:else}
							<img src="/gitlab.png" alt="GitLab" class="icone-gitlab" />
							<span class="texte-bouton-gitlab">Connexion</span>
						{/if}
					</button>
				</div>
			</div>

			<!-- Affichage du login -->
			<div class="settings-item flex-1">
				<span class="nom-parametre">Login: </span>
				<span class="login">{login}</span>
			</div>

			<div class="settings-item flex-1">
				<label for="key_pair" class="nom-parametre">Paire de clés</label>

				<div class="flex items-center gap-4">
					<div class="flex items-center gap-1">
						{#if pub_key !== '' && priv_key !== ''}
							<CheckCheck strokeWidth={3.7} class="blue-icon " />
							<span class="keypair-status keypair-status-green">Clé définie</span>
						{:else}
							<CircleOff strokeWidth={3.7} class="orange-icon " />
							<span class="keypair-status keypair-status-red">Clé non définie</span>
						{/if}

						<button
							class="pillule-bouton-keypair bouton-keypair flex items-center transition"
							on:click={handleKeyGeneration}
							disabled={gitlab_token === '' ||
								isGenerating ||
								(pub_key !== '' && priv_key !== '') ||
								login.trim() === ''}
						>
							<span class="texte-bouton-keypair">Générer une clé</span>
						</button>
					</div>

					{#if isGenerating}
						<div class="flex items-center gap-2 text-sm text-gray-700">
							<Spinner />
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
						class="pillule-bouton-sauvercharger bouton-settings flex items-center justify-center gap-2 p-2 transition"
						on:click={() => {
							loadConfigFromFile();
						}}
					>
						<Upload strokeWidth={3} class="icone-bouton-sauvercharger m-1" />
						<span class="texte-bouton-sauvercharger">Charger</span>
					</button>

					<button
						class="pillule-bouton-sauvercharger bouton-settings flex items-center justify-center gap-2 p-2 transition"
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
						class="pillule-bouton-settings bouton-settings flex items-center transition"
						disabled={isValidating}
					>
						<span class="texte-bouton-settings">Terminé</span>
					</button>
					{#if isValidating}
						<div class="mt-1 flex items-center">
							<Spinner />
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
