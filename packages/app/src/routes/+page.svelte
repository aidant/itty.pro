<script lang="ts">
	import { random, shorten } from '$lib';
	import IconCopy from '$lib/icon-copy.svelte';
	import { onMount } from 'svelte';

	let clipboard = $state('');
	let placeholder = $state('8badf00d');

	let link = $state('');
	let key = $state('');

	const setPlaceholder = () => {
		const alphabet = [
			'_',
			'-',
			'0',
			'1',
			'2',
			'3',
			'4',
			'5',
			'6',
			'7',
			'8',
			'9',
			'a',
			'b',
			'c',
			'd',
			'e',
			'f',
			'g',
			'h',
			'i',
			'j',
			'k',
			'l',
			'm',
			'n',
			'o',
			'p',
			'q',
			'r',
			's',
			't',
			'u',
			'v',
			'w',
			'x',
			'y',
			'z',
			'A',
			'B',
			'C',
			'D',
			'E',
			'F',
			'G',
			'H',
			'I',
			'J',
			'K',
			'L',
			'M',
			'N',
			'O',
			'P',
			'Q',
			'R',
			'S',
			'T',
			'U',
			'V',
			'W',
			'X',
			'Y',
			'Z'
		];
		const indexToReplace = random(0, placeholder.length - 1);
		const indexForChar = random(0, alphabet.length - 1);

		const start = placeholder.substring(0, indexToReplace);
		const end = placeholder.substring(indexToReplace + 1, placeholder.length);

		placeholder = start + alphabet[indexForChar] + end;
	};

	onMount(() => {
		const handleFocus = async () => {
			const text = await navigator.clipboard.readText();

			try {
				const url = new URL(text);
				clipboard = url.href;
			} catch {}
		};

		handleFocus().catch(() => {});

		const handlePaste = (event: ClipboardEvent) => {
			const value = event.clipboardData?.getData('text/plain');

			if (value) {
				link = value;
				key = '';
			}
		};

		const interval = setInterval(setPlaceholder, 100);

		addEventListener('focus', handleFocus);
		addEventListener('paste', handlePaste);
		addEventListener('copy', handleCopy);

		return () => {
			removeEventListener('focus', handleFocus);
			removeEventListener('paste', handlePaste);
			removeEventListener('copy', handleCopy);
			clearInterval(interval);
		};
	});

	const handleCopy = async () => {
		if (!link) {
			link = clipboard;
			key = '';
		}

		try {
			const short = await shorten(link);
			const url = new URL(short);
			key = url.pathname.replace(/^\//, '');
			await navigator.clipboard.writeText(url.href);
		} catch (error) {
			const fieldset = document.getElementById('link') as HTMLFieldSetElement;
			fieldset.setCustomValidity(error instanceof Error ? error.message : '');
			fieldset.checkValidity();
		}
	};
</script>

<header>
	<nav
		class="mx-auto flex w-full max-w-xl flex-row items-center justify-between py-4 md:max-w-2xl lg:max-w-4xl"
	>
		<h1 class="text-2xl">https://{globalThis?.window?.location.hostname || ''}/</h1>

		<ul class="flex flex-row gap-6">
			<li>Features</li>
			<li>Pricing</li>
			<li>Login</li>
			<li class="rounded-lg px-2 text-fuchsia-700 ring-2 ring-fuchsia-700">Get started</li>
		</ul>
	</nav>
</header>

<main class="flex flex-col">
	<section
		class="mx-auto flex h-96 max-w-xl flex-col justify-center gap-4 text-left md:max-w-2xl md:text-center lg:max-w-4xl lg:gap-6"
	>
		<h2
			class="font-serif text-4xl leading-10 font-bold text-balance md:text-6xl md:leading-14 lg:leading-18"
		>
			A bring your own domain link shortener, built for everyone
		</h2>
		<p class="text-balance">
			Transform any long or short link into a branded, trackable, and secure snippet on your domain.
			Streamline your user experience with our API by dynamically generating sharable trackable
			links on the fly.
		</p>
	</section>

	<section class="mx-auto flex h-96 w-full max-w-md flex-col items-center justify-center">
		<fieldset class="x-outline flex w-full flex-col gap-3 bg-fuchsia-50 px-6 py-3">
			<label>
				Link
				<textarea
					id="link"
					name="link"
					class="x-input field-sizing-content w-full resize-none"
					placeholder={clipboard || globalThis?.window?.location.href || 'https://example.com/'}
					bind:value={link}
					oninput={() => (key = '')}
					onchange={() => (key = '')}
				></textarea>
			</label>
			<form-error-message for="link" class="text-red-900 underline decoration-wavy">
			</form-error-message>
			<div>
				Shortened link
				<div class="x-input x-focus-within flex w-full flex-row justify-baseline">
					<span class="inline-block select-none">
						https://{globalThis?.window?.location.hostname}/
					</span>
					<textarea
						class="field-sizing-content w-full resize-none border-0 p-0 placeholder:text-fuchsia-950/50 focus:ring-0"
						oninput={(event) =>
							(event.currentTarget.value = encodeURIComponent(
								decodeURIComponent(event.currentTarget.value)
							))}
						bind:value={key}
						{placeholder}
					></textarea>
					<button class="cursor-pointer" onclick={handleCopy}>
						<IconCopy
							class="ml-auto h-4 w-4 shrink-0 overflow-visible text-fuchsia-700 hover:text-fuchsia-800"
						/>
					</button>
				</div>
			</div>
		</fieldset>
	</section>
</main>

<style>
	@import 'tailwindcss/theme' theme(reference);

	.x-input,
	.x-outline {
		@apply rounded-lg border-1 border-fuchsia-300;
	}

	.x-input,
	.x-focus {
		@apply transition focus:border-fuchsia-400 focus:ring-3 focus:ring-fuchsia-400/25;
	}

	.x-focus-within {
		@apply transition focus-within:border-fuchsia-400 focus-within:ring-3 focus-within:ring-fuchsia-400/25;
	}

	.x-input {
		@apply bg-white px-3 py-2 shadow-sm shadow-fuchsia-300/25 placeholder:text-fuchsia-950/50;
	}

	.x-button {
		@apply rounded-lg border-1 border-fuchsia-700/75 bg-fuchsia-700 bg-clip-padding px-3 py-2 text-white transition hover:bg-fuchsia-800 hover:bg-clip-border hover:shadow-inner hover:shadow-fuchsia-900 focus:border-fuchsia-800 focus:bg-fuchsia-800 focus:bg-clip-border focus:ring-3 focus:ring-fuchsia-400/25 focus-visible:outline-none;
	}
</style>
