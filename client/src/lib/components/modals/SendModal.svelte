<script lang="ts">
    import { fly, fade } from "svelte/transition";
    import { open } from "@tauri-apps/plugin-dialog";
    import { X, Star, Upload, Send } from "lucide-svelte";
    import { globalConfig, serverPublicKey } from "@stores/settings.js";
    import { selectedContact } from "@stores/contacts.js";
    import { writable, get } from "svelte/store";
    import { onMount } from "svelte";
    import Spinner from "@components/Spinner.svelte";
    import { constants } from "@stores/constants";
    import {
        calculateTransactionFee,
        checkSendTransaction,
        contactGet,
        contactMarkUsed,
        contactSearch,
        loadInvoiceFromFile,
        searchFirstUsers,
        sendTransaction
    } from "@invoke";
    import type { ClassicTransactionSent } from "@bindings";

    let { oncancel } = $props();

    type ContactDisplay = {
        login: string;
        nickname: string;
        favorite: boolean;
    };

    let receiver = $state("");
    let searchResults = $state<string[]>([]);
    let favoriteContacts = $state<ContactDisplay[]>([]);
    let showSuggestions = $state(false);
    let amount = $state("");
    let description = $state("");
    let fees = $state("0");
    let estimatedFee = $state("0");
    let totalCost = $state("0");

    let tooltipX = $state(0);
    let tooltipY = $state(0);
    let showTooltip = $state("");

    let invoice_file_extension = constants.nexium_invoice_extension;

    function handleClose() {
        selectedContact.set("");
        oncancel?.();
    }

    let validationError = writable(true);
    let isSending = $state(false);

    async function updateFeeCost(): Promise<void> {
        const hasDescription = description.trim().length > 0;

        await calculateTransactionFee(parseInt(fees), hasDescription).match(
            (fee) => {
                estimatedFee = fee;

                const amountNum = parseFloat(amount) || 0;
                const feeNum = parseFloat(fee) || 0;
                totalCost = (amountNum + feeNum).toFixed(6);
            },
            (err) => {
                console.error(err);
                estimatedFee = "0";
                totalCost = amount || "0";
            }
        );
    }

    function handleMontantChange(): void {
        amount = amount.trim();
        checkTransaction();
        updateFeeCost();
    }

    function handleFeesChange(): void {
        updateFeeCost();
        checkTransaction();
    }

    function handleDescriptionChange(): void {
        updateFeeCost();
        checkTransaction();
    }

    async function loadFavoriteContacts(): Promise<void> {
        const contacts = await contactGet(true);
        favoriteContacts = contacts.map((c) => ({
            login: c.login,
            nickname: c.nickname,
            favorite: c.favorite
        }));
    }

    async function handleReceiverChange(): Promise<void> {
        receiver = receiver.trim();
        checkTransaction();

        if (receiver.length > 0) {
            const [firstUsers, contactSrch] = await Promise.all([
                searchFirstUsers($globalConfig, receiver),
                contactSearch(receiver)
            ]);

            firstUsers.match(
                (results) => {
                    const contactLogins = contactSrch.map((c) => c.login);
                    const allResults = [...new Set([...contactLogins, ...results])];
                    searchResults = allResults;
                    showSuggestions = allResults.length > 0;
                },
                (err) => {
                    console.error(err);
                    searchResults = [];
                    showSuggestions = false;
                }
            );
        } else {
            searchResults = [];
            showSuggestions = false;
        }
    }

    async function handleLoadFile(): Promise<void> {
        const path = await open({
            title: "Choisir le fichier de la facture",
            multiple: false,
            directory: false,
            save: false,
            filters: [{ name: "Nexium Invoice", extensions: [invoice_file_extension] }]
        });
        if (!path) return;

        await loadInvoiceFromFile(path).match(
            (invoice) => {
                receiver = invoice.sender_login;
                amount = invoice.amount;
                description = invoice.description;
                checkTransaction();
            },
            (err) => {
                console.error("Erreur lors du chargement de la facture:", err);
            }
        );
    }

    async function checkTransaction(): Promise<void> {
        const classic_transaction_sent = {
            receiver: receiver,
            amount: amount,
            description: description,
            fees: fees
        };

        const hasErr = await checkSendTransaction(classic_transaction_sent, $globalConfig).match(
            () => false,
            (err) => {
                console.error("Erreur de validation de la transaction:", err);
                return true;
            }
        );

        validationError.set(hasErr);
    }

    async function handleSend(): Promise<void> {
        const classic_transaction_sent: ClassicTransactionSent = {
            receiver: receiver,
            amount: amount,
            description: description,
            fees: fees
        };

        isSending = true;

        const [markUsed, sendTr] = await Promise.all([
            contactMarkUsed(receiver),
            sendTransaction($serverPublicKey, $globalConfig, classic_transaction_sent)
        ]);

        if (markUsed.isOk() && sendTr.isOk()) {
            handleClose();
        } else {
            if (markUsed.isErr()) {
                console.error("Erreur lors de la mise à jour du contact:", markUsed.error);
            }
            if (sendTr.isErr()) {
                console.error("Erreur lors de l'envoi de la transaction:", sendTr.error);
            }
        }

        isSending = false;
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
                        <span class="cost-value">{amount || "0"} NXM</span>
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
