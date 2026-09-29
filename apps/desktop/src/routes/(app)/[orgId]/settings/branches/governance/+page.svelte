<!--
@component BranchGovernancePage
Dedicated page for institutional branch policies and data isolation governance.
-->
<script lang="ts">
import { page } from "$app/state";
import { onMount } from "svelte";
import { api, extractErrorMessage } from "$lib/api";
import { toast } from "@orbis/ui";
import { l } from "$lib/core/i18n";
import BranchSettingsCard from "$lib/features/settings/components/branch-settings-card.svelte";

let orgId = $derived((page.params.orgId as string) || "");
let currentOrg = $state<any>(null);
let isLoading = $state(true);

async function loadOrg() {
  isLoading = true;
  try {
    currentOrg = await api.getCurrentOrg();
  } catch (err) {
    toast.error(extractErrorMessage(err));
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (orgId) loadOrg();
});
</script>

<svelte:head>
  <title>{l("سياسات وحوكمة الفروع", "Branch Governance & Policies")} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto p-6 scrollbar-hide text-start font-sans">
  <div class="max-w-4xl space-y-4">
    {#if isLoading}
      <div class="h-64 rounded-xl border border-border/40 bg-muted/20 animate-pulse"></div>
    {:else}
      <BranchSettingsCard org={currentOrg} />
    {/if}
  </div>
</div>
