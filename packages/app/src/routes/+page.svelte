<script lang="ts">
	import { shorten } from '$lib';

	const handleSubmit = async (event: SubmitEvent) => {
		event.preventDefault();

		const data = new FormData(event.target as HTMLFormElement);

		const long = data.get('url')?.toString();

		const short = await shorten(long!);

		await navigator.clipboard.writeText(short);
	};
</script>

<main class="flex h-full w-full flex-col items-center justify-center font-sans text-fuchsia-950">
	<form class="flex w-full max-w-md flex-col gap-3" on:submit={handleSubmit}>
		<label>
			Url
			<input
				type="url"
				name="url"
				class="mt-1 block w-full rounded-lg border-fuchsia-100 px-3 py-2 shadow-md shadow-fuchsia-600/5 transition focus:border-fuchsia-400 focus:ring-3 focus:ring-fuchsia-200/75"
			/>
		</label>
		<button
			class="blocktext-white fosuc:ring-fuchsia-200/75 rounded-lg border-1 border-fuchsia-200 bg-fuchsia-700 px-3 py-2 shadow-md shadow-fuchsia-600/10 transition hover:bg-fuchsia-800 focus:border-fuchsia-400 focus:ring-3 focus-visible:outline-none"
			>Shorten</button
		>
	</form>
</main>
