import { signUpVerifyEmail } from '$lib/api.svelte.js';
import { redirect } from '@sveltejs/kit';

export const prerender = false;
export const ssr = false;

export const load = async ({ params: { token } }) => {
	await signUpVerifyEmail(token);
	throw redirect(307, '/app/');
};
