<script>
	export let showSettingsModal = false;

	import Spinner from '$lib/components/Spinner.svelte';

	import { CloudUpload } from 'lucide-svelte';
	import { fly } from 'svelte/transition';
	import { Download } from 'lucide-svelte';
	import { Upload } from 'lucide-svelte';
	import { CheckCheck } from 'lucide-svelte';
	import { CircleOff } from 'lucide-svelte';
	import { CircleAlert } from 'lucide-svelte';
	import { open } from '@tauri-apps/plugin-dialog';
	import { save } from '@tauri-apps/plugin-dialog';
	import { Unplug } from 'lucide-svelte';
	import { globalConfig } from '$lib/stores/settings.js';

	import { invoke } from '@tauri-apps/api/core';

	import { get } from 'svelte/store';
	import PasswordModal from '$lib/components/PasswordModal.svelte';
	// create a copy of the global config store
	let config = get(globalConfig);

	let errorMessage = '';
	let isValidating = false;

	let isGenerating = false;
	let generationMessage = '';
	let sentKeys = false;

	let isValidatingAndDone = false;

	let password = '';
	let showPasswordModal = false;
	let rejectPassword;
	/** @type {(value: string) => void} */
	let resolvePassword;

	function promptPassword() {
		showPasswordModal = true;
		return new Promise((resolve, reject) => {
			resolvePassword = resolve;
			rejectPassword = reject;
		});
	}

	async function handleGitlabOAuth() {
		if (config.gitlab_token_type === 'OAuth') {
			config.gitlab_token = '';
			config.gitlab_token_type = 'Classic';
			config.user_login = '';
			config.pub_key = '';
			config.priv_key = '';
			sentKeys = false;
			isValidating = false;
			return;
		} else {
			try {
				isValidating = true;
				const response = await invoke('get_gitlab_oauth_token');
				config.gitlab_token = response.token;
				config.gitlab_token_type = 'OAuth';
				setLoginFromToken();
			} catch (error) {
				errorMessage = String(error);
			} finally {
				isValidating = false;
			}
		}
	}

	async function handleLoadFile() {
		const path = await open({
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
		if (path) {
			try {
				const response = await invoke('load_config_from_file', { pathString: path });
				config = response;
			} catch (error) {
				errorMessage = String(error);
			}
		}
	}

	async function handleSaveFile() {
		isValidating = true;
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
					config: config
				});
			} catch (error) {
				errorMessage = String(error);
			} finally {
				isValidating = false;
			}
		}
		isValidating = false;
	}

	async function handleKeyGeneration() {
		isValidating = true;
		password = await promptPassword();

		isGenerating = true;
		generationMessage = 'Génération de vos clés...';

		try {
			const [pubKey, privKey] = await invoke('keypair_generation', {
				login: config.user_login,
				password
			});
			config.pub_key = pubKey;
			config.priv_key = privKey;

			generationMessage = 'Envoi des clés sur GitLab...';
			await invoke('send_gpg_key', {
				gitlabTokenType: config.gitlab_token_type,
				gitlabToken: config.gitlab_token,
				pubKey: config.pub_key
			});
			generationMessage = '';
			sentKeys = true;
		} catch (e) {
			errorMessage = String(e);
			generationMessage = '';
		} finally {
			isGenerating = false;
			isValidating = false;
		}
	}
	/** @param {string} pw */
	function handlePasswordSubmit(pw) {
		showPasswordModal = false;
		resolvePassword(pw);
	}

	function handlePasswordCancel() {
		showPasswordModal = false;
	}

	async function handleDone() {
		isValidatingAndDone = true;
		try {
			errorMessage = '';
			isValidating = true;
			const parsedPort = Number(config.port);
			if (!Number.isInteger(parsedPort) || parsedPort < 0 || parsedPort > 65535) {
				throw new Error('Invalid port number.');
			}
			config.port = parsedPort;
			await invoke('check_config_values', { config });
			globalConfig.set(config);
			showSettingsModal = false;
			console.log(config.gitlab_token_type);
		} catch (error) {
			errorMessage = String(error);
		} finally {
			isValidating = false;
			isValidatingAndDone = false;
		}
	}

	async function setLoginFromToken() {
		try {
			config.pub_key = '';
			config.priv_key = '';
			sentKeys = false;
			const response = await invoke('get_login', {
				gitlabToken: config.gitlab_token,
				gitlabTokenType: config.gitlab_token_type
			});

			config.user_login = response;
			if (errorMessage !== '') {
				errorMessage = '';
			}
		} catch (error) {
			config.user_login = '';
			errorMessage = String(error);
		}
	}
</script>

{#if showSettingsModal}
	<div class="settings-modal" transition:fly={{ duration: 300 }}>
		<div class="settings-modal-content">
			<h2 class="settings-titre">Paramètres</h2>

			<div class="barre-separation"></div>

			<div class="settings-item settings-row flex gap-4">
				<div class=" flex-1">
					<label for="server-port" class="nom-parametre">Port</label>
					<input
						id="server-port"
						type="number"
						bind:value={config.port}
						class="input-field"
						placeholder="3000"
						min="0"
						max="65535"
					/>
				</div>
				<div class="flex-[4]">
					<label for="server-url" class="nom-parametre">URL du serveur</label>
					<input
						id="server-url"
						type="text"
						bind:value={config.url_server}
						class="input-field"
						placeholder="https://server.nexium.com"
					/>
				</div>
			</div>

			<div class="settings-item flex items-center justify-between gap-4">
				<input
					id="gitlab-token"
					type="text"
					bind:value={config.gitlab_token}
					on:input={setLoginFromToken}
					class="input-field"
					placeholder="Token Gitlab"
					disabled={config.gitlab_token_type === 'OAuth'}
				/>

				<!-- "ou" -->
				<span class="ou">ou</span>

				<div class="flex flex-col items-center">
					<button
						on:click={() => {
							handleGitlabOAuth();
						}}
						class="group flex items-center transition {config.gitlab_token_type === 'OAuth'
							? 'pillule-bouton-gitlab-checked'
							: 'pillule-bouton-gitlab bouton-noir-settings gap-1 px-4 py-2 pl-3 '}"
					>
						{#if config.gitlab_token_type === 'OAuth'}
							<span
								class=" relative flex h-[14px] w-[130px] items-center justify-center transition duration-300"
							>
								<span
									class="absolute flex items-center justify-center gap-2 opacity-100 group-hover:opacity-0"
								>
									<CheckCheck strokeWidth={3} class="icone-gitlab" />
									<span class="texte-bouton-gitlab">Connecté</span>
								</span>
								<span
									class="absolute flex items-center justify-center gap-2 opacity-0 group-hover:opacity-100"
								>
									<Unplug strokeWidth={3} class="icone-gitlab-deco" />
									<span class="texte-bouton-gitlab">Déconnexion</span>
								</span>
							</span>
						{:else}
							<img src="/gitlab.png" alt="GitLab" class="icone-gitlab" />
							<span class="texte-bouton-gitlab-noir">Connexion</span>
						{/if}
					</button>
				</div>
			</div>
			<div class="barre-separation"></div>
			<!-- Affichage du login -->
			<div class="settings-item flex-1">
				<span class="nom-parametre">Login: </span>
				{#if config.user_login !== ''}
					<span class="surligne transition">{config.user_login}</span>
				{/if}
			</div>

			<div class="settings-item flex-1 transition">
				<div class="flex items-center gap-4 transition">
					<div class="flex items-center gap-3 transition">
						<div class="flex items-center gap-1 transition">
							{#if config.pub_key !== '' && config.priv_key !== ''}
								<CheckCheck strokeWidth={3.7} class="blue-icon " />
								<span class="keypair-status keypair-status-blue transition">Clés définies!</span>
							{:else}
								<CircleOff strokeWidth={3.7} class="orange-icon " />
								<span class="keypair-status keypair-status-orange transition"
									>Clés non définies</span
								>
							{/if}
						</div>

						<button
							class="bouton-keypair flex items-center transition"
							on:click={handleKeyGeneration}
							hidden={config.gitlab_token === '' ||
								(config.pub_key !== '' && config.priv_key !== '') ||
								config.user_login.trim() === ''}
							disabled={isGenerating}
						>
							<span class="texte-bouton-keypair">Générer les clés</span>
						</button>
					</div>

					{#if isGenerating}
						<div class="flex items-center gap-1 text-sm text-gray-700 transition">
							<Spinner />
							<span class="generation-status transition">{generationMessage}</span>
						</div>
					{:else if sentKeys}
						<div class="sent-keys flex items-center gap-1 transition">
							<CloudUpload strokeWidth={3.2} />
							<span>Clés ajoutées sur Gitlab!</span>
						</div>
					{/if}
				</div>
			</div>

			<div class="barre-separation"></div>
			<!-- Barre du bas -->
			<div class="mt-6 flex items-center justify-between">
				<div class="flex gap-4">
					<button
						class="pillule-bouton-sauvercharger flex items-center justify-center transition"
						on:click={() => {
							handleLoadFile();
						}}
					>
						<Upload strokeWidth={3.4} class="icone-bouton-sauvercharger" />
					</button>

					<button
						class="pillule-bouton-sauvercharger flex items-center justify-center transition"
						on:click={() => {
							handleSaveFile();
						}}
					>
						<Download strokeWidth={3.4} class="icone-bouton-sauvercharger " />
					</button>
				</div>

				<div class="flex items-center gap-1">
					{#if errorMessage !== ''}
						<div class="mt-1 flex items-center gap-1">
							<CircleAlert strokeWidth={3.5} class="red-icon m" />
							<span class="centered-error">{errorMessage}</span>
						</div>
					{/if}
				</div>

				<div class="flex flex-col items-end">
					<button
						on:click={() => {
							handleDone();
						}}
						class="pillule-bouton-settings bouton-noir-settings flex items-center transition"
						disabled={isValidating}
					>
						<span class="texte-bouton-settings">Terminé</span>
					</button>
					{#if isValidatingAndDone}
						<div class="mt-1 flex items-center">
							<Spinner />
						</div>
					{/if}
				</div>
			</div>
		</div>
	</div>
{/if}
{#if showPasswordModal}
	<PasswordModal
		on:submit={(e) => handlePasswordSubmit(e.detail)}
		on:cancel={handlePasswordCancel}
	/>
{/if}
