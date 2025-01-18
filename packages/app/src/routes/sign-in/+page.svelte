<script lang="ts">
	import { goto } from '$app/navigation';
	import { signIn } from '$lib/api.svelte';
	import Button from '$lib/button.svelte';
	import Container from '$lib/container.svelte';
	import Input from '$lib/input.svelte';
	import type { EventHandler } from 'svelte/elements';

	let disableSignIn = $state(false);

	const handleSubmit: EventHandler<SubmitEvent, HTMLFormElement> = async (event) => {
		event.preventDefault();
		disableSignIn = true;
		await signIn(new FormData(event.currentTarget));
		disableSignIn = false;
		goto('/app/');
	};
</script>

<main class="flex w-full grow flex-row items-center">
	<section class="mx-auto max-w-md grow">
		<form onsubmit={handleSubmit}>
			<Container>
				{#snippet title()}
					<h2>Sign in to your account</h2>
				{/snippet}
				<Input name="email" title="Email address" type="email" autocomplete="email" />
				<Input name="password" title="Password" type="password" autocomplete="password" />
				<Button disabled={disableSignIn}>Sign in</Button>
				<a href="/app/sign-up" class="block w-full py-1 text-center text-rose-950"
					>Need an account? <span class="underline decoration-dotted">Sign up</span></a
				>
			</Container>
		</form>
	</section>
</main>
