import { me } from '$lib/api.svelte.js';

export const prerender = true;
export const ssr = false;

export const load = () => {
	me();
};
