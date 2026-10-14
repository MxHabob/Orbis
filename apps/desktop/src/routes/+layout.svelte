<script lang="ts">
import type { Pathname } from "$app/types";
import { resolve } from "$app/paths";
import { page } from "$app/state";
import { locales, localizeHref, getLocale, getTextDirection } from "$lib/paraglide/runtime";
import { appearanceStore } from "$lib/stores/appearance.svelte";
import "$lib/stores/theme.svelte";
import { isDesktopApp } from "$lib/api/runtime";
import { onMount } from "svelte";
import { QueryClientProvider } from "@tanstack/svelte-query";
import { queryClient } from "$lib/api/query-client";
import "./layout.css";
import favicon from "$lib/assets/favicon.svg";
import { Toaster } from "@orbis/ui";
import { localeState } from "$lib/core/i18n/locale-state.svelte";

let { children } = $props();

// Keep document metadata in sync before the next DOM update after locale changes.
$effect.pre(() => {
  localeState.revision;
  const currentLocale = getLocale();
  const dir = getTextDirection(currentLocale);
  if (typeof document !== "undefined") {
    document.documentElement.dir = dir;
    document.documentElement.lang = currentLocale;
  }
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

<Toaster position={appearanceStore.toastPosition} />

<!-- Hidden locale links for paraglide -->
<div style="display:none">
	{#each locales as locale (locale)}
		<a href={resolve(localizeHref(page.url.pathname, { locale }) as Pathname)}>{locale}</a>
	{/each}
</div>
