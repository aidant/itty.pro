<script lang="ts">
	import Button from '$lib/button.svelte';
	import Container from '$lib/container.svelte';
	import Input from '$lib/input.svelte';
</script>

<main class="flex w-full grow flex-row items-center">
	<section class="mx-auto max-w-md grow">
		<form method="POST" action="/api/sign-in">
			<Container>
				{#snippet title()}
					<h2>Sign in to your account</h2>
				{/snippet}
				<Input name="email" title="Email address" type="email" autocomplete="email" />
				<Input name="password" title="Password" type="password" autocomplete="password" />
				<Button>Sign in</Button>
				<a href="/app/sign-up" class="block w-full py-1 text-center text-rose-950"
					>Need an account? <span class="underline decoration-dotted">Sign up</span></a
				>
			</Container>
		</form>
	</section>
</main>
