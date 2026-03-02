<script lang="ts">
    import { Settings, Sun, Moon, BarChart3, Users, CircleAlert, X, Globe } from "lucide-svelte";
    import { fade } from "svelte/transition";
    import { getCurrentWindow } from "@tauri-apps/api/window";
    import { showContactsModal } from "@stores/contacts.js";
    import {
        isConfigSet,
        showHistoryModal,
        showReceiveModal,
        showSendModal,
        showStatsModal,
        showServerListModal,
        globalErrorMessage,
        theme
    } from "@stores/settings.js";
    import { TRANSITION_FAST } from "@consts";

    import MenuGauche from "@components/MenuGauche.svelte";
    import Solde from "@components/Solde.svelte";
    import SendModal from "@modals/SendModal.svelte";
    import ReceiveModal from "@modals/ReceiveModal.svelte";
    import ContactsModal from "@modals/ContactsModal.svelte";
    import StatsModal from "@modals/StatsModal.svelte";
    import SettingsModal from "@modals/SettingsModal.svelte";
    import HistoryModal from "@modals/HistoryModal.svelte";
    import ServerListModal from "@modals/ServerListModal.svelte";

    let showSettingsModal = false;

    function toggleTheme() {
        theme.update((t) => (t === "dark" ? "light" : "dark"));
    }

    async function minimizeWindow() {
        await getCurrentWindow().minimize();
    }

    async function toggleMaximize() {
        const win = getCurrentWindow();
        if (await win.isMaximized()) {
            await win.unmaximize();
        } else {
            await win.maximize();
        }
    }

    async function closeWindow() {
        await getCurrentWindow().close();
    }

    function handleTitlebarMouseDown(e: MouseEvent) {
        if (e.buttons === 1) {
            if (e.detail === 2) {
                toggleMaximize();
            } else {
                getCurrentWindow().startDragging();
            }
        }
    }
</script>

<div class="main-container">
    <!-- Custom Titlebar -->
    <div class="custom-titlebar">
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div class="titlebar-drag-region" onmousedown={handleTitlebarMouseDown}></div>
        <div class="titlebar-left">
            <button class="icon-btn" onclick={closeWindow} title="Fermer">
                <X strokeWidth={2} size={20} />
            </button>
        </div>
        <div class="titlebar-actions">
            <button
                onclick={toggleTheme}
                class="icon-btn"
                title="Changer le thème"
                style="position: relative;"
            >
                {#if $theme === "dark"}
                    <div
                        style="position: absolute;"
                        in:fade={{ duration: TRANSITION_FAST }}
                        out:fade={{ duration: TRANSITION_FAST }}
                    >
                        <Sun strokeWidth={2} size={20} />
                    </div>
                {:else}
                    <div
                        style="position: absolute;"
                        in:fade={{ duration: TRANSITION_FAST }}
                        out:fade={{ duration: TRANSITION_FAST }}
                    >
                        <Moon strokeWidth={2} size={20} />
                    </div>
                {/if}
            </button>
            <button
                onclick={() => showServerListModal.set(true)}
                disabled={!$isConfigSet}
                class="icon-btn"
                title="Serveurs du réseau"
            >
                <Globe strokeWidth={2} size={20} />
            </button>
            <button
                onclick={() => showStatsModal.set(true)}
                disabled={!$isConfigSet}
                class="icon-btn"
                title="Statistiques"
            >
                <BarChart3 strokeWidth={2} size={20} />
            </button>
            <button
                onclick={() => showContactsModal.set(true)}
                disabled={!$isConfigSet}
                class="icon-btn"
                title="Contacts"
            >
                <Users strokeWidth={2} size={20} />
            </button>
            <button onclick={() => (showSettingsModal = true)} class="icon-btn" title="Paramètres">
                <Settings strokeWidth={2} size={20} />
            </button>
        </div>
    </div>

    <!-- Main Content -->
    <div class="content-wrapper">
        <MenuGauche />
        <Solde />
    </div>

    <!-- Status Bar -->
    {#if !$isConfigSet}
        <div class="status-bar">
            <button class="status-message" onclick={() => (showSettingsModal = true)}>
                <CircleAlert strokeWidth={2.5} size={18} class="icon" />
                Aucune configuration
            </button>
        </div>
    {:else if $globalErrorMessage}
        <div class="status-bar">
            <div class="status-message">
                <CircleAlert strokeWidth={2.5} size={18} class="icon" />
                {$globalErrorMessage}
            </div>
        </div>
    {/if}
</div>

<!-- Modals -->
<SettingsModal bind:showSettingsModal />

{#if $showSendModal}
    <SendModal oncancel={() => showSendModal.set(false)} />
{/if}

{#if $showReceiveModal}
    <ReceiveModal oncancel={() => showReceiveModal.set(false)} />
{/if}

{#if $showHistoryModal}
    <HistoryModal oncancel={() => showHistoryModal.set(false)} />
{/if}

{#if $showContactsModal}
    <ContactsModal oncancel={() => showContactsModal.set(false)} />
{/if}

{#if $showStatsModal}
    <StatsModal oncancel={() => showStatsModal.set(false)} />
{/if}

{#if $showServerListModal}
    <ServerListModal oncancel={() => showServerListModal.set(false)} />
{/if}
