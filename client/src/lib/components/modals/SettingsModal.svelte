<script>
	export let showSettingsModal = false;

	import { fly, fade } from 'svelte/transition';
	import { open, save } from '@tauri-apps/plugin-dialog';
	import { invoke } from '@tauri-apps/api/core';
	import { get } from 'svelte/store';
	import { 
		X, FolderOpen, Save, CheckCircle, AlertCircle, 
		Key, Upload, Download, Loader2, Bell, BellOff, Unplug
	} from 'lucide-svelte';
	
	import Spinner from '$lib/components/Spinner.svelte';
	import PasswordModal from './PasswordModal.svelte';
	import AskPasswordModal from './AskPasswordModal.svelte';
	
	import { globalConfig, isConfigSet, serverPublicKey } from '$lib/stores/settings.js';
	import { notificationsEnabled } from '$lib/services/notifications';

	let config = get(globalConfig);
	let errorMessage = '';
	let isValidating = false;
	let isGenerating = false;
	let generationMessage = '';
	let sentKeys = false;
	let isValidatingAndDone = false;
	let password = '';

	let showNewPasswordModal = false;
	let showAskPasswordModal = false;
	let rejectPassword;
	let resolveNewPassword;
	let resolveAskPassword;

	invoke('is_testnet').then((result) => {
		config.is_testnet = result;
	});

	function promptNewPassword() {
		showNewPasswordModal = true;
		return new Promise((resolve, reject) => {
			resolveNewPassword = resolve;
			rejectPassword = reject;
		});
	}

	function askPassword() {
		showAskPasswordModal = true;
		return new Promise((resolve, reject) => {
			resolveAskPassword = resolve;
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
		}
		
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

	async function handleLoadFile() {
		const path = await open({
			multiple: false,
			directory: false,
			title: 'Charger la configuration',
			filters: [{ name: 'JSON', extensions: ['json'] }]
		});
		if (path) {
			try {
				const response = await invoke('load_config_from_file', { pathString: path });
				config = response;
				isValidating = false;
			} catch (error) {
				errorMessage = String(error);
			}
		}
	}

	async function handleSaveFile() {
		isValidating = true;
		const path = await save({
			filters: [{ name: 'config', extensions: ['json'] }]
		});
		if (path) {
			try {
				const parsedPort = Number(config.port);
				if (!Number.isInteger(parsedPort) || parsedPort < 0 || parsedPort > 65535) {
					throw new Error('Invalid port number.');
				}
				config.port = parsedPort.toString();
				await invoke('save_config_to_file', { pathString: path, config: config });
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
		password = await promptNewPassword();
		config.password = password;

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

	function handleNewPasswordSubmit(pw) {
		showNewPasswordModal = false;
		resolveNewPassword(pw);
	}

	function handleAskedPasswordSubmit(pw) {
		showAskPasswordModal = false;
		resolveAskPassword(pw);
	}

	function handleNewPasswordCancel() {
		showNewPasswordModal = false;
	}

	function handleAskedPasswordCancel() {
		showAskPasswordModal = false;
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

			config.port = parsedPort.toString();
			await invoke('check_config_values', { config });

			const server_pub_key_login = await invoke('get_server_infos', { config });
			if (server_pub_key_login) {
				serverPublicKey.set(server_pub_key_login[0]);
				config.server_login = server_pub_key_login[1];
			} else {
				throw new Error('Erreur de récupération des informations du serveur.');
			}

			// Save config to default path
			await invoke('save_config', { config });

			globalConfig.set(config);
			isConfigSet.set(true);
			showSettingsModal = false;
		} catch (error) {
			errorMessage = String(error);
		} finally {
			isValidating = false;
			isValidatingAndDone = false;
		}
	}

	function closeAndCancel() {
		config = get(globalConfig);
		showSettingsModal = false;
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

	async function handleKeyImport() {
		try {
			const pubKeyPath = await open({
				multiple: false,
				directory: false,
				title: 'Sélectionner la clé publique',
				filters: [{ name: 'Clé publique', extensions: ['pub'] }]
			});

			if (pubKeyPath) {
				const pubKey = await invoke('read_key_from_file', { path: pubKeyPath });
				config.pub_key = pubKey;
			}

			const privKeyPath = await open({
				multiple: false,
				directory: false,
				title: 'Sélectionner la clé privée',
				filters: [{ name: 'Clé privée', extensions: ['priv'] }]
			});

			if (privKeyPath) {
				const privKey = await invoke('read_key_from_file', { path: privKeyPath });
				config.priv_key = privKey;
			} else {
				errorMessage = "Les clés n'ont pas été importées correctement.";
			}

			if (config.pub_key && config.priv_key) {
				config.password = await askPassword();
			}
		} catch (error) {
			errorMessage = String(error);
		}
	}

	async function handleKeyExport() {
		try {
			const pubPath = await save({
				title: 'Exporter la clé publique',
				defaultPath: 'public_key.pub',
				filters: [{ name: 'Clé publique', extensions: ['pub'] }]
			});

			if (pubPath) {
				await invoke('write_key_to_file', { path: pubPath, key: config.pub_key });
			}

			const privPath = await save({
				title: 'Exporter la clé privée',
				defaultPath: 'private_key.priv',
				filters: [{ name: 'Clé privée', extensions: ['priv'] }]
			});

			if (privPath) {
				await invoke('write_key_to_file', { path: privPath, key: config.priv_key });
			}
		} catch (error) {
			errorMessage = String(error);
		}
	}
</script>

{#if showSettingsModal}
	<div class="modal-backdrop" transition:fade={{ duration: 200 }}>
		<div class="modal-container modal-container-lg" transition:fly={{ y: 30, duration: 200 }}>
			<div class="modal-header">
				<h3 class="modal-title">Paramètres</h3>
				<button class="modal-close" onclick={closeAndCancel}>
					<X size={18} />
				</button>
			</div>

			<div class="modal-body">
				{#if errorMessage}
					<div class="error-message">
						<AlertCircle size={16} />
						{errorMessage}
					</div>
				{/if}

				<!-- Server Settings -->
				<div class="settings-section">
					<div class="settings-section-title">Connexion au serveur</div>
					<div class="form-row">
						<div class="form-group" style="flex: 0 0 100px;">
							<label for="server-port" class="form-label">Port</label>
							<input
								id="server-port"
								type="number"
								bind:value={config.port}
								class="form-input text-center"
								placeholder="4242"
								min="0"
								max="65535"
							/>
						</div>
						<div class="form-group">
							<label for="server-url" class="form-label">Adresse du serveur</label>
							<input
								id="server-url"
								type="text"
								bind:value={config.server_address}
								class="form-input"
								placeholder="nexium.jeanflix.fr"
							/>
						</div>
					</div>
				</div>

				<!-- GitLab Auth -->
				<div class="settings-section">
					<div class="settings-section-title">Authentification GitLab</div>
					<div class="gitlab-auth-row">
						<input
							id="gitlab-token"
							type="text"
							bind:value={config.gitlab_token}
							oninput={setLoginFromToken}
							class="form-input form-input-mono"
							placeholder="Token GitLab"
							disabled={config.gitlab_token_type === 'OAuth'}
						/>
						<span class="gitlab-divider">ou</span>
						<button
							onclick={handleGitlabOAuth}
							class="btn {config.gitlab_token_type === 'OAuth' ? 'btn-danger' : 'btn-filled'}"
							disabled={isValidating}
						>
							{#if config.gitlab_token_type === 'OAuth'}
								<Unplug size={16} />
								Déconnecter
							{:else}
								<img src="https://gitlab.com/favicon.ico" alt="" style="width: 16px; height: 16px;" />
								OAuth
							{/if}
						</button>
					</div>
					
					{#if config.user_login}
						<div class="mt-3 flex items-center gap-2">
							<CheckCircle size={14} class="text-success" />
							<span class="text-sm">Connecté en tant que</span>
							<code class="font-mono text-sm bg-surface px-2 py-0.5 rounded">{config.user_login}</code>
						</div>
					{/if}
				</div>

				<!-- Keypair -->
				<div class="settings-section">
					<div class="settings-section-title">Paire de clés</div>
					<div class="keypair-section">
						{#if config.pub_key && config.priv_key}
							<div class="keypair-status valid">
								<CheckCircle size={14} />
								Clés configurées
								{#if sentKeys}
									<span class="text-muted text-sm ml-2">• Envoyées sur GitLab</span>
								{/if}
							</div>
						{:else if generationMessage}
							<div class="keypair-status pending">
								<Loader2 size={14} class="spinner" />
								{generationMessage}
							</div>
						{:else}
							<div class="keypair-status error">
								<AlertCircle size={14} />
								Aucune clé configurée
							</div>
						{/if}
						
						<div class="keypair-actions">
							<button
								class="btn btn-sm btn-filled"
								onclick={handleKeyGeneration}
								disabled={!config.user_login || isGenerating}
							>
								<Key size={14} />
								Générer
							</button>
							<button
								class="btn btn-sm btn-ghost"
								onclick={handleKeyImport}
								disabled={!config.user_login}
							>
								<Upload size={14} />
								Importer
							</button>
							<button
								class="btn btn-sm btn-ghost"
								onclick={handleKeyExport}
								disabled={!config.pub_key || !config.priv_key}
							>
								<Download size={14} />
								Exporter
							</button>
						</div>
					</div>
				</div>
			</div>

			<div class="modal-footer">
				<button class="btn btn-ghost" onclick={handleLoadFile} disabled={isValidating}>
					<FolderOpen size={16} />
					Charger
				</button>
				<button class="btn btn-ghost" onclick={handleSaveFile} disabled={isValidating}>
					<Save size={16} />
					Sauvegarder
				</button>
				<div class="flex-1"></div>
				<button class="btn btn-ghost" onclick={closeAndCancel}>Annuler</button>
				<button
					class="btn btn-filled"
					onclick={handleDone}
					disabled={isValidating || isValidatingAndDone}
				>
					{#if isValidatingAndDone}
						<Spinner />
					{/if}
					Valider
				</button>
			</div>
		</div>
	</div>
{/if}

{#if showNewPasswordModal}
	<PasswordModal onsubmit={handleNewPasswordSubmit} oncancel={handleNewPasswordCancel} />
{/if}

{#if showAskPasswordModal}
	<AskPasswordModal onsubmit={handleAskedPasswordSubmit} oncancel={handleAskedPasswordCancel} />
{/if}

<style>
	:global(.text-success) {
		color: var(--accent-green);
	}
	.text-muted {
		color: var(--text-muted);
	}
	.bg-surface {
		background: var(--bg-surface);
	}
	code {
		font-family: 'Inconsolata', monospace;
	}
</style>
