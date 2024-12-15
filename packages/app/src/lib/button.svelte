<script lang="ts">
	let { children, ...props } = $props();
</script>

<button
	class="w-full rounded-lg border-2 border-rose-200 bg-rose-700 px-3 py-2 text-white inset-shadow-sm shadow-md shadow-rose-600/10 inset-shadow-rose-800/25 transition hover:bg-[color-mix(in_oklab,var(--color-rose-700),var(--color-rose-800))] focus:border-rose-400 focus:bg-[color-mix(in_oklab,var(--color-rose-700),var(--color-rose-800))] focus:ring-3 focus:ring-rose-400/25 focus:inset-shadow-rose-800/0 focus-visible:outline-none"
	{...props}
>
	{@render children()}
</button>
