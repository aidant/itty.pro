class ApiError extends Error {
	constructor(
		public response: unknown,
		message: string,
		options?: ErrorOptions
	) {
		super(message, options);
	}
}
const unwrap = async (promise: Promise<Response>) => {
	const response = await promise;

	if (response.ok) {
		return await response.json();
	} else {
		throw new ApiError(await response.json(), '');
	}
};
const isBodyInit = (body: unknown): body is BodyInit =>
	body === undefined ||
	body === null ||
	body instanceof ReadableStream ||
	body instanceof Blob ||
	body instanceof ArrayBuffer ||
	body instanceof FormData ||
	body instanceof URLSearchParams;
const entries = (body: any) => {
	if ('entries' in body && typeof body.entries === 'function') {
		return body.entries();
	}
	return Object.entries(body);
};
const encodeWith = (body: any, encoder: any): any => {
	if (body instanceof encoder) {
		return body;
	}

	const data = new encoder();

	for (const [key, value] of entries(body)) {
		data.set(key, String(value));
	}

	return data;
};
const multipartFormData = (body: Record<PropertyKey, string> | FormData | URLSearchParams) =>
	encodeWith(body, FormData);
const formUrlEncoded = (body: Record<PropertyKey, string> | FormData | URLSearchParams) =>
	encodeWith(body, URLSearchParams);

const api = {
	get: async (path: string) => unwrap(fetch(path, { method: 'GET' })),
	post: async <T>(path: string, body?: T) =>
		unwrap(
			fetch(path, {
				method: 'POST',
				body: isBodyInit(body) ? (body ?? undefined) : JSON.stringify(body),
				headers: isBodyInit(body)
					? undefined
					: {
							'Content-Type': 'application/json; charset=utf-8'
						}
			})
		)
};

export type UserCredentials = {
	email: string;
	password: string;
};

export type NewUserCredentials = {
	display_name: string;
	email: string;
	password: string;
};

export type User = {
	display_name: string;
	email: string;
	email_verified: boolean;
};

export type EmailVerificationStatus = {
	email_verified: boolean;
};

let user = $state<User | null>(null);
export const user$ = () => user;

export const me = async (): Promise<User | null> => {
	user = await api.get('/api/@me');
	return user;
};

export const signIn = async (
	credentials: UserCredentials | FormData | URLSearchParams
): Promise<User | null> => {
	user = await api.post('/api/sign-in', formUrlEncoded(credentials));
	return user;
};

export const signUp = async (
	credentials: NewUserCredentials | FormData | URLSearchParams
): Promise<User> => {
	user = await api.post('/api/sign-up', formUrlEncoded(credentials));
	return user!;
};

export const signUpVerifyEmail = async (token: string): Promise<EmailVerificationStatus> => {
	const status = await api.post(`/api/sign-up/${token}`);
	return status;
};

export const signOut = async (): Promise<void> => {
	await api.post('/api/sign-out');
	user = null;
};
