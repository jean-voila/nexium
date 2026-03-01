<script lang="ts">
    import { fade, fly } from "svelte/transition";
    import { onMount } from "svelte";
    import { Star, Trash2, Edit2, Plus, Search, X, User } from "lucide-svelte";
    import { showSendModal, globalConfig } from "$lib/stores/settings.js";
    import { selectedContact } from "$lib/stores/contacts.js";
    import type { Contact } from "@bindings";
    import {
        contactAdd,
        contactGet,
        contactMarkUsed,
        contactSearch,
        contactUpdate,
        searchFirstUsers
    } from "@invoke";
    import { contactRemove } from "@invoke/contactRemove";

    let { oncancel } = $props();

    let contacts = $state<Contact[]>([]);
    let searchQuery = $state("");
    let showAddModal = $state(false);
    let editingContact = $state<Contact | null>(null);

    // Form state
    let formLogin = $state("");
    let formNickname = $state("");
    let formNotes = $state("");
    let formFavorite = $state(false);
    let formError = $state("");
    let loading = $state(false);

    // Login autocomplete state
    let loginSearchResults = $state<string[]>([]);
    let showLoginSuggestions = $state(false);

    function handleClose() {
        oncancel?.();
    }

    async function loadContacts(): Promise<void> {
        loading = true;

        const query = searchQuery.trim();
        contacts = query ? await contactSearch(query) : await contactGet();

        loading = false;
    }

    function getFirstNameFromLogin(login: string): string {
        const firstName = login.split(".")[0] || "";
        return firstName.charAt(0).toUpperCase() + firstName.slice(1).toLowerCase();
    }

    function updateNicknameFromLogin(login: string): void {
        const firstName = getFirstNameFromLogin(login);
        if (firstName) {
            formNickname = firstName;
        }
    }

    async function handleLoginInput(): Promise<void> {
        const searchValue = formLogin.trim();

        // Pre-fill nickname with first name
        updateNicknameFromLogin(searchValue);

        if (searchValue.length > 0) {
            await searchFirstUsers($globalConfig, searchValue).match(
                (results) => {
                    loginSearchResults = results;
                    showLoginSuggestions = loginSearchResults.length > 0;
                },
                (e) => {
                    console.error("Error searching users:", e);
                    loginSearchResults = [];
                    showLoginSuggestions = false;
                }
            );
        } else {
            loginSearchResults = [];
            showLoginSuggestions = false;
        }
    }

    async function handleAddContact(): Promise<void> {
        formError = "";
        if (!formLogin.trim()) {
            formError = "Le login est requis";
            return;
        }

        await contactAdd(
            formLogin.trim(),
            formNickname.trim(),
            formNotes.trim(),
            formFavorite
        ).match(
            () => {
                showAddModal = false;
                resetForm();
            },
            (error) => {
                console.error("Error adding contact:", error);
                formError = error;
            }
        );

        // Load contacts outside try/catch to avoid error interference
        await loadContacts();
    }

    async function handleUpdateContact(): Promise<void> {
        if (!editingContact) return;
        formError = "";

        await contactUpdate(
            editingContact.login,
            formNickname?.trim(),
            formNotes.trim(),
            formFavorite
        ).match(
            () => {
                editingContact = null;
                resetForm();
            },
            (error) => {
                console.error("Error updating contact:", error);
                formError = error;
            }
        );
    }

    async function handleDeleteContact(login: string): Promise<void> {
        if (!confirm("Supprimer ce contact ?")) return;

        await contactRemove(login).match(
            () => loadContacts(),
            (error) => {
                console.error("Error removing contact:", error);
            }
        );
    }

    async function toggleFavorite(contact: Contact): Promise<void> {
        await contactUpdate(contact.login, null, null, !contact.favorite).match(
            () => {
                loadContacts();
            },
            (error) => {
                console.error("Error toggling favorite:", error);
            }
        );
    }

    function startEdit(contact: Contact): void {
        editingContact = contact;
        formLogin = contact.login;
        formNickname = contact.nickname;
        formNotes = contact.notes;
        formFavorite = contact.favorite;
        formError = "";
    }

    function resetForm(): void {
        formLogin = "";
        formNickname = "";
        formNotes = "";
        formFavorite = false;
        formError = "";
        loginSearchResults = [];
        showLoginSuggestions = false;
    }

    function openAddModal(): void {
        resetForm();
        showAddModal = true;
    }

    async function sendToContact(contact: Contact): Promise<void> {
        selectedContact.set(contact.login);

        await contactMarkUsed(contact.login).match(
            () => {},
            (error) => {
                console.error("Error marking contact as used:", error);
            }
        );

        handleClose();
        showSendModal.set(true);
    }

    onMount(() => {
        loadContacts();
    });
</script>

<div class="modal-backdrop" transition:fade={{ duration: 200 }}>
    <div class="modal-container modal-container-lg" transition:fly={{ y: 30, duration: 200 }}>
        <div class="modal-header">
            <h3 class="modal-title">Carnet d'adresses</h3>
            <button class="modal-close" onclick={handleClose}>
                <X size={18} />
            </button>
        </div>

        <!-- Toolbar -->
        <div class="toolbar">
            <div class="search-box">
                <span class="search-box-icon"><Search size={16} /></span>
                <input
                    type="text"
                    placeholder="Rechercher..."
                    bind:value={searchQuery}
                    oninput={loadContacts}
                    class="search-box-input"
                />
            </div>
            <button class="btn btn-accent" onclick={openAddModal}>
                <Plus size={16} />
                Ajouter
            </button>
        </div>

        <!-- Contacts List -->
        <div class="modal-body" style="padding-top: 0;">
            {#if loading}
                <div class="loading-state">Chargement...</div>
            {:else if contacts.length === 0}
                <div class="empty-state">
                    <User size={48} strokeWidth={1} />
                    <p>Aucun contact</p>
                </div>
            {:else}
                {#each contacts.toSorted((a, b) => (b.favorite ? 1 : 0) - (a.favorite ? 1 : 0)) as contact}
                    <div class="contact-card" class:favorite={contact.favorite}>
                        <button class="contact-info" onclick={() => sendToContact(contact)}>
                            <div class="contact-avatar">
                                {contact.nickname
                                    ? contact.nickname[0].toUpperCase()
                                    : contact.login[0].toUpperCase()}
                            </div>
                            <div class="contact-details">
                                <span class="contact-nickname"
                                    >{contact.nickname || contact.login}</span
                                >
                                {#if contact.nickname}
                                    <span class="contact-login">{contact.login}</span>
                                {/if}
                                {#if contact.notes}
                                    <span class="contact-notes">{contact.notes}</span>
                                {/if}
                            </div>
                        </button>
                        <div class="contact-actions">
                            <button
                                class="contact-action-btn star"
                                class:active={contact.favorite}
                                onclick={() => toggleFavorite(contact)}
                                title="Favori"
                            >
                                <Star size={16} fill={contact.favorite ? "currentColor" : "none"} />
                            </button>
                            <button
                                class="contact-action-btn"
                                onclick={() => startEdit(contact)}
                                title="Modifier"
                            >
                                <Edit2 size={16} />
                            </button>
                            <button
                                class="contact-action-btn delete"
                                onclick={() => handleDeleteContact(contact.login)}
                                title="Supprimer"
                            >
                                <Trash2 size={16} />
                            </button>
                        </div>
                    </div>
                {/each}
            {/if}
        </div>
    </div>
</div>

<!-- Add/Edit Modal -->
{#if showAddModal || editingContact}
    <div class="modal-backdrop" style="z-index: 110;" transition:fade={{ duration: 150 }}>
        <div class="modal-container" transition:fly={{ y: 20, duration: 200 }}>
            <div class="modal-header">
                <h3 class="modal-title">
                    {editingContact ? "Modifier le contact" : "Nouveau contact"}
                </h3>
                <button
                    class="modal-close"
                    onclick={() => {
                        showAddModal = false;
                        editingContact = null;
                    }}
                >
                    <X size={18} />
                </button>
            </div>

            <div class="modal-body">
                {#if formError}
                    <div class="error-message">{formError}</div>
                {/if}

                {#if !editingContact}
                    <div class="form-group relative">
                        <label for="login" class="form-label">Login *</label>
                        <input
                            id="login"
                            type="text"
                            bind:value={formLogin}
                            oninput={handleLoginInput}
                            placeholder="jean.dupont"
                            class="form-input form-input-mono"
                        />
                        {#if showLoginSuggestions}
                            <div class="suggestions-dropdown">
                                {#each loginSearchResults as user}
                                    <button
                                        type="button"
                                        class="suggestion-item"
                                        onclick={() => {
                                            formLogin = user;
                                            updateNicknameFromLogin(user);
                                            showLoginSuggestions = false;
                                        }}
                                    >
                                        {user}
                                    </button>
                                {/each}
                            </div>
                        {/if}
                    </div>
                {/if}

                <div class="form-group">
                    <label for="nickname" class="form-label">Surnom</label>
                    <input
                        id="nickname"
                        type="text"
                        bind:value={formNickname}
                        placeholder="Jean"
                        class="form-input"
                    />
                </div>

                <div class="form-group">
                    <label for="notes" class="form-label">Notes</label>
                    <textarea
                        id="notes"
                        bind:value={formNotes}
                        placeholder="Notes sur ce contact..."
                        class="form-input form-textarea"
                    ></textarea>
                </div>

                <div class="toggle-wrapper">
                    <button
                        class="toggle"
                        class:active={formFavorite}
                        onclick={() => (formFavorite = !formFavorite)}
                        aria-label="Marquer comme favori"
                    >
                        <div class="toggle-knob"></div>
                    </button>
                    <span class="toggle-label">Marquer comme favori</span>
                </div>
            </div>

            <div class="modal-footer">
                <button
                    class="btn btn-ghost"
                    onclick={() => {
                        showAddModal = false;
                        editingContact = null;
                    }}
                >
                    Annuler
                </button>
                <button
                    class="btn btn-filled"
                    onclick={editingContact ? handleUpdateContact : handleAddContact}
                >
                    {editingContact ? "Modifier" : "Ajouter"}
                </button>
            </div>
        </div>
    </div>
{/if}
