<script lang="ts">
	import '@fontsource-variable/playfair-display';
	import '@fontsource/atkinson-hyperlegible';
	import { onMount } from 'svelte';
	import '../app.css';
	let { children } = $props();

	onMount(async () => {
		await import('$lib/form-error-message');
	});
</script>

{@render children()}
