<script lang="ts">
	import { slide } from 'svelte/transition';

	let {
		id = crypto.randomUUID(),
		type = 'text',
		title,
		value = $bindable(),
		prefix = '',
		action = undefined,
		onaction = () => {},
		error = '',
		...props
	} = $props();
</script>

<label for={id} class="max-w-prose">
	{title}
	<div
		class="mt-1 flex w-full flex-row justify-baseline rounded-lg border-2 border-rose-200/75 bg-white/75 px-3 py-2 inset-shadow-sm inset-shadow-rose-300/10 transition placeholder:text-rose-950/50 focus-within:border-rose-400 focus-within:ring-3 focus-within:ring-rose-400/25"
	>
		{#if prefix}
			<span class="inline-block">
				{prefix}
			</span>
		{/if}
		{#if type === 'textarea'}
			<textarea
				{id}
				class="field-sizing-content w-full resize-none border-0 bg-transparent p-0 placeholder:text-rose-950/50 focus:ring-0"
				bind:value
				{...props}
			></textarea>
		{:else}
			<input
				{type}
				{id}
				class="w-full border-0 bg-transparent p-0 placeholder:text-rose-950/50 focus:ring-0"
				bind:value
				{...props}
			/>
		{/if}
		{#if action}
			<button class="cursor-pointer text-rose-900 hover:text-rose-950" onclick={() => onaction()}>
				{@render action()}
			</button>
		{/if}
	</div>
	{#if error}
		<div transition:slide={{ duration: 250 }} class="text-rose-900 underline decoration-wavy">
			{error}
		</div>
	{/if}
</label>
