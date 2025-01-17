export const prerender = true;

export const ssr = false;

export const load = (event) => {
	return {
		user: event
			.fetch('/api/@me')
			.then((response) => response.json())
			.catch(() => ({ data: null }))
	};
};
