<script lang="ts">
	import Button from '$lib/button.svelte';
	import Container from '$lib/container.svelte';
	import Input from '$lib/input.svelte';
</script>

<main class="flex w-full grow flex-row items-center">
	<section class="mx-auto max-w-md grow">
		<Container>
			{#snippet title()}
				<h2>Sign up for an account</h2>
			{/snippet}
			<Input title="Display name" autocomplete="name" />
			<Input title="Email address" type="email" autocomplete="email" />
			<Input title="Password" type="password" autocomplete="new-password" />
			<Button>Sign up</Button>
			<a href="/app/sign-in" class="block w-full py-1 text-center text-rose-950"
				>Already have an account? <span class="underline decoration-dotted">Sign in</span></a
			>
		</Container>
	</section>
</main>
