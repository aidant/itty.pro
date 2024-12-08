export const shorten = async (url: string, key?: string): Promise<string> => {
	const response = await fetch(`/${key || ''}`, { method: 'POST', body: url });

	if (response.ok) {
		return response.text();
	} else {
		const text = await response.text();

		let error: Error | undefined;
		try {
			const json = JSON.parse(text);
			error = new Error(json.error || text);
		} catch {}

		throw error || new Error(text);
	}
};

export const random = (min = 0, max = Number.MAX_SAFE_INTEGER) =>
	Math.floor(Math.random() * (max - min + 1) + min);
