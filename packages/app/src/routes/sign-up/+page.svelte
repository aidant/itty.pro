<script lang="ts">
	import { goto } from '$app/navigation';
	import { signUp } from '$lib/api.svelte';
	import Button from '$lib/button.svelte';
	import Container from '$lib/container.svelte';
	import Input from '$lib/input.svelte';
	import type { EventHandler } from 'svelte/elements';

	let disableSignUp = $state(false);

	const handleSubmit: EventHandler<SubmitEvent, HTMLFormElement> = async (event) => {
		event.preventDefault();
		disableSignUp = true;
		try {
			await signUp(new FormData(event.currentTarget));
			goto('/app/');
		} catch (error) {
			console.error(error);
		} finally {
			disableSignUp = false;
		}
	};
</script>

<main class="flex w-full grow flex-row items-center">
	<section class="mx-auto max-w-md grow">
		<form onsubmit={handleSubmit}>
			<Container>
				{#snippet title()}
					<h2>Sign up for an account</h2>
				{/snippet}
				<Input name="display_name" title="Display name" autocomplete="name" />
				<Input name="email" title="Email address" type="email" autocomplete="email" />
				<Input name="password" title="Password" type="password" autocomplete="new-password" />
				<Button disabled={disableSignUp}>Sign up</Button>
				<a href="/app/sign-in" class="block w-full py-1 text-center text-rose-950"
					>Already have an account? <span class="underline decoration-dotted">Sign in</span></a
				>
			</Container>
		</form>
	</section>
</main>
