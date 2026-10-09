<!--
  [orgId] Grid — wraps all authenticated pages with the Chrome Shell.
  This is the equivalent of GitButler's [projectId]/+layout.svelte.
-->
<script lang="ts">
import AppChrome from "$lib/shell/app-chrome.svelte";
import { authStore, systemStore, licenseStore, uiStore } from "$lib/stores";
import { goto } from "$app/navigation";
import { page } from "$app/state";
import { onMount } from "svelte";
import type { Snippet } from "svelte";
import type { LayoutData } from "./$types";
import { localeState } from "$lib/core/i18n";
import { LoadingIndicator } from "@orbis/ui";

let { data, children }: { data: LayoutData; children: Snippet } = $props();
let authReady = $state(false);
let licenseReady = $state(false);

const isOnboarding = $derived(page.url.pathname.includes("/onboarding"));

onMount(() => {
  const initialize = async () => {
    await authStore.restore();

    if (!authStore.isAuthenticated) {
      await goto("/login", { replaceState: true });
      return;
    }

    await Promise.all([
      systemStore.loadReadiness(),
      licenseStore.loadState(),
    ]);
    licenseReady = true;
    authReady = true;
  };

  void initialize();
});
</script>

{#if authReady}
  {#key localeState.revision}
    {#if isOnboarding}
      {@render children()}
      {#if uiStore.showActivationDialog}
        {#await import("$lib/shell/activation-dialog.svelte") then { default: ActivationDialog }}
          <ActivationDialog bind:open={uiStore.showActivationDialog} />
        {/await}
      {/if}
    {:else}
      <AppChrome orgId={data.orgId} {licenseReady}>
        {@render children()}
      </AppChrome>
    {/if}
  {/key}
{:else}
  <div class="flex h-full w-full items-center justify-center bg-background">
    <div class="flex items-center gap-3 text-sm text-muted-foreground">
      <LoadingIndicator size="lg" label="Loading workspace" />
    </div>
  </div>
{/if}
