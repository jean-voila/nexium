<script>
	import Bouton from '$lib/components/Bouton.svelte';
	import { SendHorizontal } from 'lucide-svelte';
	import { HandCoins } from 'lucide-svelte';
	import { QrCode } from 'lucide-svelte';
	import { Share2 } from 'lucide-svelte';
	import { Copy } from 'lucide-svelte';
</script>

<div class="flex flex-col justify-center gap-14">
	<div class="flex flex-col gap-2">
		<div class="flex flex-col">
			<div class="prenom">Jean</div>
			<div class="nom">Herail</div>
		</div>
		<div class="flex gap-1">
			<button>
				<QrCode strokeWidth={2.4} size={25} class="bouton-action" />
			</button>

			<button>
				<Share2 strokeWidth={2.4} size={25} class="bouton-action" />
			</button>

			<button>
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
