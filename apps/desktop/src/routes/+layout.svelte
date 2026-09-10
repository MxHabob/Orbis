<script lang="ts">
import type { Pathname } from "$app/types";
import { resolve } from "$app/paths";
import { page } from "$app/state";
import { locales, localizeHref } from "$lib/paraglide/runtime";
import { themeStore } from "$lib/stores";
import { isDesktopApp } from "$lib/api";
import { onMount } from "svelte";
import { QueryClientProvider } from "@tanstack/svelte-query";
import { queryClient } from "$lib/api";
import "./layout.css";
import favicon from "$lib/assets/favicon.svg";
import { Toaster } from "@orbis/ui";

let { children } = $props();

// Apply theme on mount
$effect(() => {
  themeStore.applyToDOM();
});

// Add desktop class to html element for Tauri shell styling
onMount(() => {
  if (isDesktopApp) {
    document.documentElement.classList.add("desktop");
  }
});
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
	<title>Orbis</title>
</svelte:head>

<QueryClientProvider client={queryClient}>
	{@render children()}
</QueryClientProvider>

<Toaster/>

<!-- Hidden locale links for paraglide -->
<div style="display:none">
	{#each locales as locale (locale)}
		<a href={resolve(localizeHref(page.url.pathname, { locale }) as Pathname)}>{locale}</a>
	{/each}
</div>
