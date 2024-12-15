import { random } from '$lib';
import { onMount } from 'svelte';

// prettier-ignore
const alphabet = ['_','-','0','1','2','3','4','5','6','7','8','9','a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];

export const createRandomKey = () => {
	let key = $state('8badf00d');

	const replace = () => {
		const indexToReplace = random(0, key.length - 1);
		const indexForChar = random(0, alphabet.length - 1);

		const start = key.substring(0, indexToReplace);
		const end = key.substring(indexToReplace + 1, key.length);

		key = start + alphabet[indexForChar] + end;
	};

	onMount(() => {
		const interval = setInterval(replace, 100);

		return () => {
			clearInterval(interval);
		};
	});

	return () => key;
};
