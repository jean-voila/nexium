<script>
	import { HelpCircle, Weight } from 'lucide-svelte';
	export let label = '';
	export let disabled = false;
	export let type = 'principal';
	export let Icon = HelpCircle;

	$: buttonClass = type === 'principal' ? 'pillule-bouton-principal' : 'pillule-bouton-secondaire';
	$: boutonsMenuClass = disabled ? 'boutons-menu-disabled' : 'boutons-menu';
</script>

<div
	class="w-47 flex h-11 items-center justify-center gap-2 rounded-full transition {buttonClass} {boutonsMenuClass}"
>
	<Icon strokeWidth={2.3} class="icone-bouton" />

	<span class="texte-bouton">{label}</span>
</div>
