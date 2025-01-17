import { redirect } from '@sveltejs/kit';

export const ssr = false;

export const load = async ({ params: { token }, fetch }) => {
	await fetch(`/api/sign-up/${token}`, { method: 'POST' });
	throw redirect(307, '/app');
};
