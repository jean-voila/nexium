<script lang="ts">
    import { fly, fade } from "svelte/transition";
    import { open, save } from "@tauri-apps/plugin-dialog";
    import { get } from "svelte/store";
    import {
        X,
        FolderOpen,
        Save,
        CheckCircle,
        AlertCircle,
        Key,
        Upload,
        Download,
        Loader2,
        Unplug
    } from "lucide-svelte";
    import { globalConfig, isConfigSet, serverPublicKey } from "@stores/settings.js";

    import Spinner from "@components/Spinner.svelte";
    import PasswordModal from "./PasswordModal.svelte";
    import AskPasswordModal from "./AskPasswordModal.svelte";
    import { getConstants } from "@stores/constants";
    import {
        checkConfigValues,
        getGitlabOauthToken,
        getLogin,
        getServerInfos,
        keypairGeneration,
        loadConfigFromFile,
        readKeyFromFile,
        saveConfig,
        saveConfigToFile,
        sendGpgKey,
        writeKeyToFile
    } from "@invoke";
    import { onMount } from "svelte";

    export let showSettingsModal = false;

    let config = get(globalConfig);
    let errorMessage = "";
    let isValidating = false;
    let isGenerating = false;
    let generationMessage = "";
    let sentKeys = false;
    let isValidatingAndDone = false;
    let password = "";

    let showNewPasswordModal = false;
    let showAskPasswordModal = false;
    let resolveNewPassword: ((value: string) => void) | null = null;
    let resolveAskPassword: ((value: string) => void) | null = null;

    async function promptNewPassword(): Promise<string> {
        showNewPasswordModal = true;

        return new Promise<string>((resolve) => {
            resolveNewPassword = resolve;
        });
    }

    async function askPassword(): Promise<string> {
        showAskPasswordModal = true;

        return new Promise<string>((resolve) => {
            resolveAskPassword = resolve;
        });
    }

    async function handleGitlabOAuth(): Promise<void> {
        if (config.gitlab_token_type === "OAuth") {
            // TODO: nullify instead of just emptying
            config.gitlab_token = "";
            config.gitlab_token_type = "Classic";
            config.user_login = "";
            config.pub_key = "";
            config.priv_key = "";
            sentKeys = false;
            isValidating = false;
            return;
        }

        await getGitlabOauthToken().match(
            (token) => {
                config.gitlab_token = token;
                config.gitlab_token_type = "OAuth";
                setLoginFromToken();
            },
            (err) => {
                console.error(err);
                errorMessage = err;
            }
        );

        isValidating = false;
    }

    async function handleLoadFile(): Promise<void> {
        const path = await open({
            multiple: false,
            directory: false,
            title: "Charger la configuration",
            filters: [{ name: "JSON", extensions: ["json"] }]
        });

        if (path) {
            await loadConfigFromFile(path).match(
                (cfg) => {
                    config = cfg;
                    isValidating = false;
                },
                (err) => {
                    console.error(err);
                    errorMessage = err;
                }
            );
        }
    }

    async function handleSaveFile(): Promise<void> {
        isValidating = true;

        const path = await save({
            filters: [{ name: "config", extensions: ["json"] }]
        });

        if (path) {
            const parsedPort = Number(config.port);
            if (!Number.isInteger(parsedPort) || parsedPort < 0 || parsedPort > 65535) {
                console.error("Invalid port number.");
                errorMessage = "Numéro de port invalide.";
            }

            config.port = parsedPort.toString();

            await saveConfigToFile(config, path).orTee((err) => {
                console.error(err);
                errorMessage = err;
            });
        }

        isValidating = false;
    }

    async function handleKeyGeneration(): Promise<void> {
        isValidating = true;
        password = await promptNewPassword();
        config.password = password;

        isGenerating = true;
        generationMessage = "Génération de vos clés...";

        const keyPairRes = await keypairGeneration(config.user_login, password);
        if (keyPairRes.isOk()) {
            const keyPair = keyPairRes.value;
            config.pub_key = keyPair.pub_key;
            config.priv_key = keyPair.priv_key;

            generationMessage = "Envoi des clés sur GitLab...";

            await sendGpgKey(config.gitlab_token_type, config.gitlab_token, config.pub_key).match(
                () => {
                    sentKeys = true;
                },
                (err) => {
                    console.error(err);
                    errorMessage = err;
                }
            );
        } else {
            console.error(keyPairRes.error);
            errorMessage = keyPairRes.error;
        }

        generationMessage = "";

        isGenerating = false;
        isValidating = false;
    }

    function handleNewPasswordSubmit(pw: string): void {
        showNewPasswordModal = false;
        resolveNewPassword?.(pw);
    }

    function handleAskedPasswordSubmit(pw: string): void {
        showAskPasswordModal = false;
        resolveAskPassword?.(pw);
    }

    function handleNewPasswordCancel(): void {
        showNewPasswordModal = false;
    }

    function handleAskedPasswordCancel(): void {
        showAskPasswordModal = false;
    }

    async function handleDone(): Promise<void> {
        isValidatingAndDone = true;
        errorMessage = "";
        isValidating = true;

        const parsedPort = Number(config.port);
        if (Number.isInteger(parsedPort) && parsedPort >= 0 && parsedPort <= 65535) {
            config.port = parsedPort.toString();

            if (await checkConfigValues(config)) {
                await getServerInfos(config).match(
                    async (serverInfos) => {
                        serverPublicKey.set(serverInfos.pub_key);
                        config.server_login = serverInfos.login;

                        // Save config to default path
                        if (!(await saveConfig(config))) {
                            console.error("Failed to save config.");
                            errorMessage = "Erreur lors de la sauvegarde de la configuration.";
                            return;
                        }

                        globalConfig.set(config);
                        isConfigSet.set(true);
                        showSettingsModal = false;
                    },
                    (err) => {
                        console.error(err);
                        errorMessage = "Erreur de récupération des informations du serveur.";
                    }
                );
            } else {
                errorMessage = "Configuration invalide.";
            }
        } else {
            errorMessage = "Numéro de port invalide.";
        }

        isValidating = false;
        isValidatingAndDone = false;
    }

    function closeAndCancel(): void {
        config = get(globalConfig);
        showSettingsModal = false;
    }

    async function setLoginFromToken(): Promise<void> {
        config.pub_key = "";
        config.priv_key = "";
        sentKeys = false;

        await getLogin(config.gitlab_token_type, config.gitlab_token).match(
            (login) => {
                config.user_login = login;
                errorMessage = "";
            },
            (err) => {
                console.error(err);
                config.user_login = "";
                errorMessage = err;
            }
        );
    }

    async function handleKeyImport(): Promise<void> {
        const pubKeyPath = await open({
            multiple: false,
            directory: false,
            title: "Sélectionner la clé publique",
            filters: [{ name: "Clé publique", extensions: ["pub"] }]
        });

        if (pubKeyPath === null) {
            errorMessage = "Les clés n'ont pas été importées correctement.";
            return;
        }

        const pubKeyRes = await readKeyFromFile(pubKeyPath);
        if (pubKeyRes.isOk()) {
            config.pub_key = pubKeyRes.value;
        } else {
            console.error(pubKeyRes.error);
            errorMessage = pubKeyRes.error;
            return;
        }

        const privKeyPath = await open({
            multiple: false,
            directory: false,
            title: "Sélectionner la clé privée",
            filters: [{ name: "Clé privée", extensions: ["priv"] }]
        });

        if (privKeyPath === null) {
            errorMessage = "Les clés n'ont pas été importées correctement.";
            return;
        }

        const privKeyRes = await readKeyFromFile(privKeyPath);
        if (privKeyRes.isErr()) {
            console.error(privKeyRes.error);
            errorMessage = privKeyRes.error;
            return;
        }

        config.priv_key = privKeyRes.value;

        if (!config.pub_key || !config.priv_key) {
            errorMessage = "Les clés n'ont pas été importées correctement.";
            return;
        }

        config.password = await askPassword();
    }

    async function handleKeyExport(): Promise<void> {
        const pubPath = await save({
            title: "Exporter la clé publique",
            defaultPath: "public_key.pub",
            filters: [{ name: "Clé publique", extensions: ["pub"] }]
        });

        if (pubPath !== null) {
            const writePubRes = await writeKeyToFile(pubPath, config.pub_key);
            if (writePubRes.isErr()) {
                console.error(writePubRes.error);
                errorMessage = "Erreur lors de l'export de la clé publique.";
                return;
            }
        }

        const privPath = await save({
            title: "Exporter la clé privée",
            defaultPath: "private_key.priv",
            filters: [{ name: "Clé privée", extensions: ["priv"] }]
        });

        if (privPath !== null) {
            const writePrivRes = await writeKeyToFile(privPath, config.priv_key);
            if (writePrivRes.isErr()) {
                console.error(writePrivRes.error);
                errorMessage = "Erreur lors de l'export de la clé privée.";
                return;
            }
        }
    }

    onMount(async () => {
        config.is_testnet = (await getConstants()).is_testnet;
    });
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
                            disabled={config.gitlab_token_type === "OAuth"}
                        />
                        <span class="gitlab-divider">ou</span>
                        <button
                            onclick={handleGitlabOAuth}
                            class="btn {config.gitlab_token_type === 'OAuth'
                                ? 'btn-danger'
                                : 'btn-filled'}"
                            disabled={isValidating}
                        >
                            {#if config.gitlab_token_type === "OAuth"}
                                <Unplug size={16} />
                                Déconnecter
                            {:else}
                                <img
                                    src="https://gitlab.com/favicon.ico"
                                    alt=""
                                    style="width: 16px; height: 16px;"
                                />
                                OAuth
                            {/if}
                        </button>
                    </div>

                    {#if config.user_login}
                        <div class="mt-3 flex items-center gap-2">
                            <CheckCircle size={14} class="text-success" />
                            <span class="text-sm">Connecté en tant que</span>
                            <code class="font-mono text-sm bg-surface px-2 py-0.5 rounded"
                                >{config.user_login}</code
                            >
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
                                    <span class="text-muted text-sm ml-2"
                                        >• Envoyées sur GitLab</span
                                    >
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
        font-family: "Inconsolata", monospace;
    }
</style>
