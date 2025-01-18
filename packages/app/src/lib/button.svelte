<script lang="ts">
	let { children, ...props } = $props();
</script>

<button
	class="not-disabled:inset-shadow-sm not-disabled:inset-shadow-rose-800/25 focus:ring-3 not-disabled:focus:inset-shadow-rose-800/0 not-disabled:hover:bg-[color-mix(in_oklab,var(--color-rose-700),var(--color-rose-800))] w-full rounded-lg border-2 border-rose-200 bg-rose-700 px-3 py-2 text-white shadow-md shadow-rose-600/10 transition focus:border-rose-400 focus:bg-[color-mix(in_oklab,var(--color-rose-700),var(--color-rose-800))] focus:ring-rose-400/25 focus-visible:outline-none disabled:cursor-not-allowed disabled:bg-rose-300"
	{...props}
>
	{@render children()}
</button>
