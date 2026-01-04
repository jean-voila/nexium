<script>
	import { HelpCircle } from 'lucide-svelte';
	
	export let label = '';
	export let disabled = false;
	export let type = 'primary';
	export let Icon = HelpCircle;
</script>

<div class="flex items-center justify-center gap-3 transition-all" 
	 class:btn-primary={type === 'primary'}
	 class:btn-secondary={type === 'secondary'}
	 class:opacity-50={disabled}
	 class:cursor-not-allowed={disabled}>
	<Icon strokeWidth={2.5} size={20} />
	<span class="font-display font-bold">{label}</span>
</div>
