<script lang="ts">
    import { SendHorizontal, HandCoins, History, Copy, Check } from "lucide-svelte";
    import { onMount } from "svelte";
    import { writeText } from "@tauri-apps/plugin-clipboard-manager";
    import { get } from "svelte/store";
    import { wait } from "@darco2903/web-common";
    import {
        globalConfig,
        isConfigSet,
        showReceiveModal,
        showSendModal,
        userBalanceInt,
        userBalanceDec,
        showHistoryModal,
        globalErrorMessage
    } from "@stores/settings.js";
    import { getNamesFromLogin } from "@invoke";
    import { getBalance } from "@invoke/getBalance";

    let firstName = ""; // TODO: verify type (string or null)
    let lastName = ""; // TODO: verify type (string or null)
    let copied: boolean = false;

    async function copyLogin(): Promise<void> {
        if (!$globalConfig?.user_login) return;

        await writeText($globalConfig.user_login);

        copied = true;
        await wait(2000);
        copied = false;
    }

    // React to user login changes
    $: if ($globalConfig?.user_login) {
        balanceUpdate();
        getNamesFromLogin($globalConfig.user_login).match(
            (names) => {
                firstName = names.first_name;
                lastName = names.last_name;
            },
            (err) => {
                console.error("Error fetching names from login:", err);
                firstName = "";
                lastName = "";
            }
        );
    }

    async function balanceUpdate(): Promise<void> {
        if (get(showHistoryModal) || get(showSendModal) || get(showReceiveModal)) return;
        if ($isConfigSet === false) return;

        await getBalance($globalConfig.user_login, $globalConfig).match(
            (balance) => {
                userBalanceInt.set(balance.integer_part);
                userBalanceDec.set(balance.decimal_part);
                globalErrorMessage.set("");
            },
            (err) => {
                globalErrorMessage.set("Erreur lors de la récupération du solde.");
                console.error("Error fetching balance:", err);
            }
        );
    }

    onMount(() => {
        const interval = setInterval(balanceUpdate, 5000);
        return () => clearInterval(interval);
    });
</script>

<div class="user-panel">
    <!-- User Identity -->
    <div class="user-identity">
        {#if firstName || lastName}
            <div class="user-firstname">{firstName}</div>
            <div class="user-lastname">{lastName}</div>
        {:else if $isConfigSet}
            <div class="skeleton skeleton-text-lg"></div>
            <div class="skeleton skeleton-text-sm mt-2"></div>
        {/if}

        <!-- Quick Actions -->
        {#if $isConfigSet}
            <div class="user-actions mt-4">
                <button onclick={copyLogin} class="action-link" title="Copier le login">
                    {#if copied}
                        <Check strokeWidth={2} size={18} />
                    {:else}
                        <Copy strokeWidth={2} size={18} />
                    {/if}
                </button>
                <button
                    onclick={() => showHistoryModal.set(true)}
                    class="action-link"
                    title="Historique"
                >
                    <History strokeWidth={2} size={18} />
                </button>
            </div>
        {/if}
    </div>

    <!-- Main Action Buttons -->
    {#if $isConfigSet}
        <div class="action-buttons">
            <button onclick={() => showSendModal.set(true)} class="btn-primary">
                <SendHorizontal strokeWidth={2.5} size={20} />
                Envoyer
            </button>

            <button onclick={() => showReceiveModal.set(true)} class="btn-secondary">
                <HandCoins strokeWidth={2.5} size={20} />
                Recevoir
            </button>
        </div>
    {/if}
</div>
