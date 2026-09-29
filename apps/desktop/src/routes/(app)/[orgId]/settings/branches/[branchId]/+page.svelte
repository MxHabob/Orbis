<!--
@component BranchDetailsPage
Dedicated page for viewing a specific branch with inner tabs (overview, classes, rooms, sub-units, staff, stats).
-->
<script lang="ts">
import { page } from "$app/state";
import { goto } from "$app/navigation";
import { onMount } from "svelte";
import { api, type OrgUnitDto, extractErrorMessage } from "$lib/api";
import { toast } from "@orbis/ui";
import { l } from "$lib/core/i18n";
import BranchDetailsView from "$lib/features/settings/components/branch-details-view.svelte";
import BranchSheet from "$lib/features/settings/components/branch-sheet.svelte";

let orgId = $derived((page.params.orgId as string) || "");
let branchId = $derived((page.params.branchId as string) || "");

let branch = $state<OrgUnitDto | null>(null);
let isLoading = $state(true);
let isEditSheetOpen = $state(false);

async function loadBranch() {
  if (!branchId) return;
  isLoading = true;
  try {
    branch = await api.getOrgUnit(branchId);
  } catch (err) {
    toast.error(extractErrorMessage(err));
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  loadBranch();
});

function handleBack() {
  goto(`/${orgId}/settings/branches`);
}
</script>

<svelte:head>
  <title>{branch?.name_ar || branch?.name || l("تفاصيل الفرع", "Branch Details")} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto p-6 scrollbar-hide text-start font-sans">
  {#if isLoading}
    <div class="h-96 rounded-xl border border-border/40 bg-muted/20 animate-pulse"></div>
  {:else if !branch}
    <div class="py-16 text-center text-xs text-muted-foreground">
      {l("لم يتم العثور على الفرع المطلوب", "Branch not found")}
    </div>
  {:else}
    <BranchDetailsView
      {orgId}
      {branch}
      onBack={handleBack}
      onEdit={() => (isEditSheetOpen = true)}
    />

    <BranchSheet
      bind:open={isEditSheetOpen}
      {orgId}
      {branch}
      onSuccess={() => {
        loadBranch();
      }}
    />
  {/if}
</div>
