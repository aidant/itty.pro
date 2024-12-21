<script lang="ts">
	import { shorten } from '$lib';
	import { createClipboard } from '$lib/clipboard.svelte';
	import Container from '$lib/container.svelte';
	import IconCopy from '$lib/icon-copy.svelte';
	import Input from '$lib/input.svelte';
	import { createRandomKey } from '$lib/random-key.svelte';
	import { onMount } from 'svelte';

	const getClipboard = createClipboard();
	const getRandomKey = createRandomKey();

	let link = $state('');
	let key = $state('');
	let errorMessage = $state('');

	onMount(() => {
		const handlePaste = (event: ClipboardEvent) => {
			if ((event.target as HTMLInputElement)?.id === 'link') {
				return;
			}

			const value = event.clipboardData?.getData('text/plain');

			if (value) {
				link = value;
				key = '';
				errorMessage = '';
			}
		};

		addEventListener('paste', handlePaste);
		addEventListener('copy', handleCopy);

		return () => {
			removeEventListener('paste', handlePaste);
			removeEventListener('copy', handleCopy);
		};
	});

	const handleCopy = async () => {
		if (!link) {
			link = getClipboard();
			errorMessage = '';
		}

		try {
			const short = await shorten(new URL(link).href, key);
			const url = new URL(short);
			key = url.pathname.replace(/^\//, '');
			await navigator.clipboard.writeText(url.href);
		} catch (error) {
			errorMessage = 'Please enter a valid URL';
		}
	};
</script>

<main class="flex flex-col">
	<section
		class="mx-auto flex h-96 max-w-xl flex-col justify-center gap-4 text-left md:max-w-2xl md:text-center"
	>
		<h2
			class="font-serif text-4xl sm:text-5xl sm:leading-13 leading-10 font-bold text-balance md:text-6xl md:leading-18"
		>
			A link shortener, built for everyone
		</h2>
		<p class="text-balance max-w-sm sm:max-w-md md:mx-auto md:max-w-xl lg:max-w-2xl">
			Transform any long or short link into a branded, trackable, and secure snippet on your domain.
			Streamline your user experience with our API by dynamically generating sharable trackable
			links on the fly.
		</p>
	</section>

	<section class="mx-auto flex h-96 w-full max-w-md flex-col items-center justify-center">
		<Container>
			<Input
				id="link"
				type="url"
				title="Link"
				placeholder={getClipboard() || globalThis?.window?.location.href || 'https://itty.pro/'}
				bind:value={link}
				oninput={() => {
					key = '';
					errorMessage = '';
				}}
				onchange={() => {
					key = '';
					errorMessage = '';
				}}
				error={errorMessage}
			/>

			<Input
				title="Shortened link"
				prefix={`https://${globalThis?.window?.location.host}/`}
				placeholder={getRandomKey()}
				bind:value={key}
				onaction={handleCopy}
			>
				{#snippet action()}
					<IconCopy class="ml-auto h-4 w-4 shrink-0 overflow-visible" />
				{/snippet}
			</Input>
		</Container>
	</section>
</main>
