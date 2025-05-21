<script>
	import { HelpCircle, Weight } from 'lucide-svelte';
	export let label = '';
	export let type = 'principal';
	export let Icon = HelpCircle;

	$: buttonClass = type === 'principal' ? 'pillule-bouton-principal' : 'pillule-bouton-secondaire';
</script>

<div
	class="w-47 flex h-11 items-center justify-center gap-2 rounded-full transition {buttonClass} boutons-menu"
>
	<Icon strokeWidth={2.3} class="icone-bouton" />

	<span class="texte-bouton">{label}</span>
</div>
