<script lang="ts">
	import { page } from '$app/stores';
	import '@fontsource-variable/playfair-display';
	import '@fontsource/atkinson-hyperlegible';
	import '../app.css';
	let { children } = $props();
</script>

<header>
	<nav
		class="mx-auto flex w-full sm:max-w-xl flex-row items-center justify-between py-4 md:max-w-2xl lg:max-w-4xl"
	>
		<h1 class="text-2xl"><a href="/app/">itty</a></h1>

		<ul class="flex flex-row gap-6">
			<li class="line-through">Features</li>
			<li class="line-through">Pricing</li>
			{#if $page.url.pathname.includes('/sign-in')}
				<li><a href="/app/sign-up">Sign up</a></li>
			{:else}
				<li><a href="/app/sign-in">Sign in</a></li>
			{/if}
		</ul>
	</nav>
</header>

{@render children()}
