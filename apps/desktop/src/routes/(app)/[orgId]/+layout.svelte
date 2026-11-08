<!--
  [orgId] Grid — wraps all authenticated pages with the Chrome Shell.
  This is the equivalent of GitButler's [projectId]/+layout.svelte.
-->
<script lang="ts">
import AppChrome from "$lib/shell/app-chrome.svelte";
import {
  authStore,
  systemStore,
  licenseStore,
  uiStore,
  academicContext,
} from "$lib/stores";
import { afterNavigate, goto, replaceState } from "$app/navigation";
import { page } from "$app/state";
import { onMount } from "svelte";
import type { Snippet } from "svelte";
import type { LayoutData } from "./$types";
import { localeState } from "$lib/core/i18n";
import { LoadingIndicator } from "@orbis/ui";
import LicenseFeatureGate from "$lib/shell/license-feature-gate.svelte";
import {
  isStandaloneWindow,
  STANDALONE_WINDOW_PARAM,
} from "$lib/shell/window-mode";

let { data, children }: { data: LayoutData; children: Snippet } = $props();
let authReady = $state(false);
let licenseReady = $state(false);

const isOnboarding = $derived(page.url.pathname.includes("/onboarding"));
const standaloneWindow = isStandaloneWindow(page.url);

afterNavigate(({ to }) => {
  if (!standaloneWindow || !to?.url || isStandaloneWindow(to.url)) return;

  const url = new URL(to.url);
  url.searchParams.set(STANDALONE_WINDOW_PARAM, "1");
  replaceState(`${url.pathname}${url.search}${url.hash}`, page.state);
});

onMount(() => {
  const initialize = async () => {
    await authStore.restore();

    if (!authStore.isAuthenticated) {
      await goto("/login", { replaceState: true });
      return;
    }

    await Promise.all([
      systemStore.orgReadiness ?? systemStore.loadReadiness(),
      licenseStore.loadState(),
      academicContext.resolveContext(data.orgId),
    ]);
    licenseReady = true;
    authReady = true;
  };

  void initialize();
});

$effect(() => {
  if (data.orgId && data.orgId !== academicContext.orgId && authReady) {
    void academicContext.resolveContext(data.orgId);
  }
});
</script>

{#if authReady}
  {#key localeState.revision}
    {#if isOnboarding}
      <LicenseFeatureGate requireOperational {licenseReady}>
        {@render children()}
      </LicenseFeatureGate>
      {#if uiStore.showActivationDialog}
        {#await import("$lib/shell/activation-dialog.svelte") then { default: ActivationDialog }}
          <ActivationDialog bind:open={uiStore.showActivationDialog} />
        {/await}
      {/if}
    {:else}
      <AppChrome
        orgId={data.orgId}
        {licenseReady}
        standalone={standaloneWindow}
      >
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
