<!--
  [orgId] Grid — wraps all authenticated pages with the Chrome Shell.
  This is the equivalent of GitButler's [projectId]/+layout.svelte.
-->
<script lang="ts">
import { Grid } from "@orbis/icons";

import AppChrome from "$lib/shell/app-chrome.svelte";
import { authStore, systemStore } from "$lib/stores";
import { goto } from "$app/navigation";
import { onMount } from "svelte";
import type { Snippet } from "svelte";
import type { LayoutData } from "./$types";

let { data, children }: { data: LayoutData; children: Snippet } = $props();
let authReady = $state(false);

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
  <AppChrome orgId={data.orgId}>
    {@render children()}
  </AppChrome>
{:else}
  <div class="flex h-full w-full items-center justify-center bg-background">
    <div class="flex items-center gap-3 text-sm text-muted-foreground" role="status">
      <span class="size-4 animate-spin rounded-full border-2 border-muted-foreground/30 border-t-primary"></span>
      <span>Loading workspace...</span>
    </div>
  </div>
{/if}
