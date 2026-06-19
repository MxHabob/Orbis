<script lang="ts">
	import type { Pathname } from '$app/types';
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { locales, localizeHref } from '$lib/paraglide/runtime';
	import { themeStore } from '$lib/stores';
	import { Toaster } from 'svelte-sonner';
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';

	let { children } = $props();

	// Apply theme on mount
	$effect(() => {
		themeStore.applyToDOM();
	});
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
	<title>Orbis</title>
</svelte:head>

{@render children()}
<Toaster richColors position="top-center" />

<!-- Hidden locale links for paraglide -->
<div style="display:none">
	{#each locales as locale (locale)}
		<a href={resolve(localizeHref(page.url.pathname, { locale }) as Pathname)}>{locale}</a>
	{/each}
</div>
