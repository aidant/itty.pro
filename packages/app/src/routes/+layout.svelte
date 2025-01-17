<script lang="ts">
	import { page } from '$app/stores';
	import '@fontsource-variable/playfair-display';
	import '@fontsource/atkinson-hyperlegible';
	import '../app.css';
	import type { PageData } from './$types';

	let { data, children }: { data: PageData; children: Function } = $props();
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
				{#await data.user}
					{@render signIn()}
				{:then user}
					{#if user.data}
						<form method="POST" action="/api/sign-out">
							<button type="submit">Sign out</button>
						</form>
					{:else}
						{@render signIn()}
					{/if}
				{:catch}
					{@render signIn()}
				{/await}
			</li>
		</ul>
	</nav>
</header>

{@render children()}
