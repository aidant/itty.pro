<script lang="ts">
	import { page } from '$app/stores';
	import { signOut, user$ } from '$lib/api.svelte';
	import '@fontsource-variable/playfair-display';
	import '@fontsource/atkinson-hyperlegible';
	import '../app.css';

	let { children }: { children: Function } = $props();

	let disableSignOut = $state(false);
	const handleSignOut = async () => {
		disableSignOut = true;
		await signOut();
		disableSignOut = false;
	};
</script>

{#snippet signIn()}
	{#if $page.url.pathname.includes('/sign-in')}
		<a href="/app/sign-up">Sign up</a>
	{:else}
		<a href="/app/sign-in">Sign in</a>
	{/if}
{/snippet}

<header>
	<nav
		class="mx-auto flex w-full flex-row items-center justify-between py-4 sm:max-w-xl md:max-w-2xl lg:max-w-4xl"
	>
		<h1 class="text-2xl"><a href="/app/">itty</a></h1>

		<ul class="flex flex-row gap-6">
			<li class="line-through">Features</li>
			<li class="line-through">Pricing</li>
			<li>
				{#if user$()}
					<button onclick={handleSignOut} class="cursor-pointer">Sign out</button>
				{:else}
					{@render signIn()}
				{/if}
			</li>
		</ul>
	</nav>
</header>

{@render children()}
