<script>
	export let showSettingsModal = false;


	import { blur } from 'svelte/transition';




    import { serverPort, serverUrl } from '$lib/stores/settings.js'

    let url = '';
    let port = '';

    function saveSettings() {
        // Save the settings to local storage or a store
        serverPort.set(port);
        serverUrl.set(url);
    }


</script>
{#if showSettingsModal}

<div class="settings-modal" transition:blur={{ amount: 5, duration: 300 }}>
	
	<div class="settings-modal-content">
		
        <!-- Titre -->
        <h2 class="settings-titre">Paramètres</h2>

        <!-- Liste des paramètres -->
        <div class="settings-item mb-4">
            <label for="server-url" class="nom-parametre">URL du serveur</label>
            <input
                id="server-url"   
                type="text"
                bind:value={url}
                class="input-field"
                placeholder="https://server.nexium.com"
            />
        </div>
        <div class="settings-item mb-4">
            <label for="server-port" class="nom-parametre">Port</label>
            <input
                id="server-port"   
                type="number"
                bind:value={port}
                class="input-field"
                placeholder=3000
            />
        </div>




        <!-- Bouton Terminé -->
        <button onclick={() => { showSettingsModal = false; saveSettings(); }}
        class="flex items-center transition pillule-bouton-settings bouton"
    >
    
        <span class="texte-bouton-settings">Terminé</span>
        </button>

	</div>
</div>
{/if}