<script lang="ts">
	import { fade, fly } from 'svelte/transition';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import { X, RefreshCw, Server, Globe, Crown, ArrowRightLeft } from 'lucide-svelte';
	import { globalConfig, serverPublicKey } from '$lib/stores/settings.js';

	let { oncancel } = $props();

	type PeerInfo = {
		address: string;
		port: number;
	};

	type PeerWithStatus = PeerInfo & {
		status: 'waiting' | 'online' | 'offline';
		isCurrent?: boolean;
	};

	let currentServer = $state<PeerWithStatus | null>(null);
	let peers = $state<PeerWithStatus[]>([]);
	let loading = $state(false);
	let refreshing = $state(false);
	let switching = $state(false);
	let switchingIndex = $state<number | null>(null);

	function handleClose() {
		oncancel?.();
	}

	async function loadPeers() {
		loading = true;
		
		// Set current server from config
		const serverAddress = $globalConfig.server_address;
		const serverPort = parseInt($globalConfig.port) || 4242;
		currentServer = {
			address: serverAddress,
			port: serverPort,
			status: 'waiting',
			isCurrent: true
		};
		
		// Check current server status
		checkCurrentServerStatus();
		
		try {
			const result: PeerInfo[] = await invoke('get_peers', { config: $globalConfig });
			// Filter out the current server from peers list
			peers = result
				.filter(p => !(p.address === serverAddress && p.port === serverPort))
				.map((p) => ({ ...p, status: 'waiting' as const }));
			// Check status of each peer
			await checkAllPeerStatus();
		} catch (e) {
			console.error('Error loading peers:', e);
			peers = [];
		} finally {
			loading = false;
		}
	}

	async function checkCurrentServerStatus() {
		if (!currentServer) return;
		
		try {
			const isOnline: boolean = await invoke('check_peer_status', {
				address: currentServer.address,
				port: currentServer.port
			});
			currentServer.status = isOnline ? 'online' : 'offline';
		} catch (e) {
			currentServer.status = 'offline';
		}
	}

	async function checkPeerStatus(index: number) {
		const peer = peers[index];
		if (!peer) return;

		peers[index].status = 'waiting';
		try {
			const isOnline: boolean = await invoke('check_peer_status', {
				address: peer.address,
				port: peer.port
			});
			peers[index].status = isOnline ? 'online' : 'offline';
		} catch (e) {
			peers[index].status = 'offline';
		}
	}

	async function checkAllPeerStatus() {
		// Check all peers in parallel
		await Promise.all(peers.map((_, index) => checkPeerStatus(index)));
	}

	async function handleRefresh() {
		refreshing = true;
		await loadPeers();
		refreshing = false;
	}

	async function switchToServer(peer: PeerWithStatus, index: number) {
		if (switching || peer.status !== 'online') return;
		
		switching = true;
		switchingIndex = index;
		
		try {
			const result: [string, string, any] = await invoke('try_connect_to_server', {
				config: $globalConfig,
				address: peer.address,
				port: peer.port
			});
			
			if (result) {
				const [pubKey, login, updatedConfig] = result;
				serverPublicKey.set(pubKey);
				globalConfig.set(updatedConfig);
				// Save the updated config
				await invoke('save_config', { config: updatedConfig });
				// Reload the page to apply changes
				handleClose();
				window.location.reload();
			}
		} catch (e) {
			console.error('Failed to switch server:', e);
			peers[index].status = 'offline';
		} finally {
			switching = false;
			switchingIndex = null;
		}
	}

	function getStatusColor(status: 'waiting' | 'online' | 'offline'): string {
		switch (status) {
			case 'online':
				return 'var(--accent-green)';
			case 'offline':
				return 'var(--accent-red)';
			case 'waiting':
			default:
				return 'var(--accent-orange)';
		}
	}

	function getStatusLabel(status: 'waiting' | 'online' | 'offline'): string {
		switch (status) {
			case 'online':
				return 'En ligne';
			case 'offline':
				return 'Hors ligne';
			case 'waiting':
			default:
				return 'Vérification...';
		}
	}

	onMount(() => {
		loadPeers();
	});
</script>

<div class="modal-backdrop" transition:fade={{ duration: 200 }}>
	<div class="modal-container modal-md" transition:fly={{ y: 30, duration: 200 }}>
		<div class="modal-header">
			<div class="header-title">
				<Globe size={20} />
				<h3 class="modal-title">Serveurs du réseau</h3>
			</div>
			<div class="header-actions">
				<button
					class="btn btn-icon"
					onclick={handleRefresh}
					disabled={loading || refreshing}
					title="Rafraîchir la liste"
				>
					<RefreshCw size={18} class={refreshing ? 'spinning' : ''} />
				</button>
				<button class="modal-close" onclick={handleClose}>
					<X size={18} />
				</button>
			</div>
		</div>

		<div class="modal-body">
			{#if loading && peers.length === 0 && !currentServer}
				<div class="loading-state">
					<div class="spinner"></div>
					<p>Chargement des serveurs...</p>
				</div>
			{:else}
				<div class="peers-list">
					<!-- Current Server -->
					{#if currentServer}
						<div class="peer-item current-server">
							<div class="peer-icon current">
								<Crown size={20} />
							</div>
							<div class="peer-info">
								<span class="peer-address">{currentServer.address}:{currentServer.port}</span>
								<span class="peer-label">Serveur actuel</span>
							</div>
							<div class="peer-status">
								<span
									class="status-badge"
									style="--status-color: {getStatusColor(currentServer.status)}"
								>
									<span class="status-dot"></span>
									{getStatusLabel(currentServer.status)}
								</span>
							</div>
						</div>
					{/if}
					
					<!-- Other Peers -->
					{#if peers.length > 0}
						<div class="peers-divider">
							<span>Autres serveurs</span>
						</div>
						{#each peers as peer, index}
							<div class="peer-item">
								<div class="peer-icon">
									<Server size={20} />
								</div>
								<div class="peer-info">
									<span class="peer-address">{peer.address}:{peer.port}</span>
								</div>
								<div class="peer-actions">
									<div class="peer-status">
										<span
											class="status-badge"
											style="--status-color: {getStatusColor(peer.status)}"
										>
											<span class="status-dot"></span>
											{getStatusLabel(peer.status)}
										</span>
									</div>
									{#if peer.status === 'online'}
										<button
											class="btn btn-sm btn-switch"
											onclick={() => switchToServer(peer, index)}
											disabled={switching}
											title="Basculer vers ce serveur"
										>
											{#if switching && switchingIndex === index}
												<div class="spinner-small"></div>
											{:else}
												<ArrowRightLeft size={14} />
											{/if}
										</button>
									{/if}
								</div>
							</div>
						{/each}
					{/if}
				</div>
			{/if}
		</div>

		<div class="modal-footer">
			<div class="peer-count">
				{peers.length + 1} serveur{peers.length !== 0 ? 's' : ''} ({peers.length} pair{peers.length !== 1 ? 's' : ''})
			</div>
			<div class="flex-1"></div>
			<button class="btn btn-ghost" onclick={handleClose}>Fermer</button>
		</div>
	</div>
</div>

<style>
	.header-title {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.header-actions {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.btn-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		padding: 0;
		background: transparent;
		border: none;
		border-radius: var(--radius-sm);
		color: var(--text-secondary);
		cursor: pointer;
		transition: all var(--transition-fast);
	}

	.btn-icon:hover:not(:disabled) {
		background: var(--bg-hover);
		color: var(--text-primary);
	}

	.btn-icon:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	:global(.spinning) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	.loading-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 3rem;
		color: var(--text-muted);
		text-align: center;
	}

	.empty-state :global(svg) {
		margin-bottom: 1rem;
		opacity: 0.5;
	}

	.empty-hint {
		font-size: 0.875rem;
		margin-top: 0.5rem;
	}

	.spinner {
		width: 32px;
		height: 32px;
		border: 3px solid var(--glass-border);
		border-top-color: var(--accent-blue);
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
		margin-bottom: 1rem;
	}

	.peers-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.peer-item {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 0.875rem 1rem;
		background: var(--bg-surface);
		border: 1px solid var(--glass-border);
		border-radius: var(--radius-md);
		transition: all var(--transition-fast);
	}

	.peer-item:hover {
		background: var(--bg-hover);
	}

	.peer-item.current-server {
		background: color-mix(in srgb, var(--accent-blue) 10%, var(--bg-surface));
		border-color: color-mix(in srgb, var(--accent-blue) 30%, transparent);
	}

	.peer-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		background: var(--bg-elevated);
		border-radius: var(--radius-sm);
		color: var(--text-secondary);
	}

	.peer-icon.current {
		background: color-mix(in srgb, var(--accent-blue) 20%, var(--bg-elevated));
		color: var(--accent-blue);
	}

	.peer-info {
		flex: 1;
		min-width: 0;
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.peer-address {
		font-family: 'Inconsolata', monospace;
		font-size: 0.9375rem;
		color: var(--text-primary);
	}

	.peer-label {
		font-size: 0.75rem;
		color: var(--accent-blue);
		font-weight: 500;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.peers-divider {
		display: flex;
		align-items: center;
		gap: 1rem;
		margin: 0.75rem 0;
		color: var(--text-muted);
		font-size: 0.75rem;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.peers-divider::before,
	.peers-divider::after {
		content: '';
		flex: 1;
		height: 1px;
		background: var(--glass-border);
	}

	.peer-status {
		display: flex;
		align-items: center;
	}

	.peer-actions {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.btn-switch {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		padding: 0;
		background: var(--glass-bg);
		border: 1px solid var(--glass-border);
		border-radius: var(--radius-sm);
		color: var(--accent-green);
		cursor: pointer;
		transition: all var(--transition-fast);
	}

	.btn-switch:hover:not(:disabled) {
		background: color-mix(in srgb, var(--accent-green) 15%, transparent);
		border-color: var(--accent-green);
	}

	.btn-switch:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.spinner-small {
		width: 12px;
		height: 12px;
		border: 2px solid var(--glass-border);
		border-top-color: var(--accent-green);
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	.status-badge {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.375rem 0.75rem;
		background: color-mix(in srgb, var(--status-color) 15%, transparent);
		border: 1px solid color-mix(in srgb, var(--status-color) 30%, transparent);
		border-radius: var(--radius-full);
		font-size: 0.8125rem;
		font-weight: 500;
		color: var(--status-color);
	}

	.status-dot {
		width: 8px;
		height: 8px;
		background: var(--status-color);
		border-radius: 50%;
	}

	.peer-count {
		font-size: 0.875rem;
		color: var(--text-muted);
	}
</style>
