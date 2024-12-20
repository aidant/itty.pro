<script lang="ts">
	let { title = undefined, children } = $props();
</script>

<fieldset
	class="bg-roze-50/25 w-full rounded-lg ring-2 shadow-sm shadow-rose-300/10 ring-rose-200/75 backdrop-blur-3xl"
>
	{#if title}
		<div
			class="w-full rounded-t-lg border-b-2 border-rose-300/50 bg-white/50 px-6 py-3 text-center text-lg font-bold"
		>
			{@render title()}
		</div>
	{/if}
	<div class="flex flex-col gap-4 px-6 py-3">
		{@render children()}
	</div>
</fieldset>
