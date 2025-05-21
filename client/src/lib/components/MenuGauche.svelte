<script>
	import Bouton from '$lib/components/Bouton.svelte';
	import { SendHorizontal } from 'lucide-svelte';
	import { HandCoins } from 'lucide-svelte';
	import { QrCode } from 'lucide-svelte';
	import { Share2 } from 'lucide-svelte';
	import { Copy } from 'lucide-svelte';
	import { writeText, readText } from '@tauri-apps/plugin-clipboard-manager';
	import { globalConfig } from '$lib/stores/settings.js';
	import { on } from 'svelte/events';
	let firstName = '';
	let lastName = '';

	$: {
		const login = $globalConfig.user_login || '';
		const parts = login.split('.');
		firstName = parts[0] || '';
		lastName = parts[1] || '';

		if (firstName && lastName) {
			firstName = firstName.charAt(0).toUpperCase() + firstName.slice(1);
			lastName = lastName.charAt(0).toUpperCase() + lastName.slice(1);
		}
	}
</script>

<div class="flex flex-col justify-center gap-14">
	<div class="flex flex-col gap-2">
		<div class="flex flex-col">
			<div class="prenom">{firstName}</div>
			<div class="nom">{lastName}</div>
		</div>
		<div class="flex gap-1">
			<button onclick={() => writeText($globalConfig.user_login)}>
				<Copy strokeWidth={2.4} size={25} class="bouton-action" />
			</button>
		</div>
	</div>

	<div class="flex flex-col gap-2">
		<button>
			<Bouton label="Envoyer" Icon={SendHorizontal} />
		</button>

		<button>
			<Bouton label="Recevoir" type="secondaire" Icon={HandCoins} />
		</button>
	</div>
</div>
