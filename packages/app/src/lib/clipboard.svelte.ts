import { onMount } from 'svelte';

export const createClipboard = () => {
	let clipboard = $state('');

	const reflect = async () => {
		try {
			const text = await navigator.clipboard.readText();

			const url = new URL(text);
			clipboard = url.href;
		} catch {}
	};

	onMount(() => {
		reflect();

		addEventListener('focus', reflect);

		return () => {
			removeEventListener('focus', reflect);
		};
	});

	return () => clipboard;
};
