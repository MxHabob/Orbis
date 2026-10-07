<!--
  [orgId] Grid — wraps all authenticated pages with the Chrome Shell.
  This is the equivalent of GitButler's [projectId]/+layout.svelte.
-->
<script lang="ts">
import AppChrome from "$lib/shell/app-chrome.svelte";
import { authStore, systemStore } from "$lib/stores";
import { goto } from "$app/navigation";
import { page } from "$app/state";
import { onMount } from "svelte";
import type { Snippet } from "svelte";
import type { LayoutData } from "./$types";
import { localeState } from "$lib/core/i18n";
import { LoadingIndicator } from "@orbis/ui";

let { data, children }: { data: LayoutData; children: Snippet } = $props();
let authReady = $state(false);

const isOnboarding = $derived(page.url.pathname.includes("/onboarding"));

onMount(async () => {
  await authStore.restore();

  if (!authStore.isAuthenticated) {
    await goto("/login", { replaceState: true });
    return;
  }

  authReady = true;
  await systemStore.loadReadiness();
});
</script>

{#if authReady}
  {#key localeState.revision}
    {#if isOnboarding}
      {@render children()}
    {:else}
      <AppChrome orgId={data.orgId}>
        {@render children()}
      </AppChrome>
    {/if}
  {/key}
{:else}
  <div class="flex h-full w-full items-center justify-center bg-background">
    <div class="flex items-center gap-3 text-sm text-muted-foreground">
      <LoadingIndicator size="lg" label="Loading workspace" />
      <span aria-hidden="true">Loading workspace...</span>
    </div>
  </div>
{/if}
