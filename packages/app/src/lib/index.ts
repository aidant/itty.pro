export const shorten = async (url: string): Promise<string> =>
	fetch('/', { method: 'POST', body: url }).then((response) => response.text());
