<script lang="ts">
import * as m from "$lib/paraglide/messages";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuSub,
  DropdownMenuSubContent,
  DropdownMenuSubTrigger,
  DropdownMenuTrigger,
} from "@orbis/ui/dropdown-menu";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
  Avatar,
  AvatarFallback,
} from "@orbis/ui";
import { goto } from "$app/navigation";
import { resolve } from "$app/paths";
import type { Pathname } from "$app/types";
import { localizeHref } from "$lib/paraglide/runtime";
import { api } from "$lib/api/client";
import { l } from "$lib/core/i18n/locale";
import { authStore, effectiveContext } from "$lib/stores";
import { onMount } from "svelte";
import {
  createDashboardAccessQuery,
  dashboardBranch,
} from "$lib/features/dashboard/queries";

let {
  orgId,
  orgName = "",
  orgNameAr,
  primaryColor = "var(--primary)",
}: {
  orgId: string;
  orgName?: string | null;
  orgNameAr?: string | null;
  primaryColor?: string;
} = $props();

const access = createDashboardAccessQuery(
  () => orgId,
  () => authStore.userId,
);
const branches = $derived(access.data?.units ?? []);
const activeBranch = $derived(
  dashboardBranch(access.data, effectiveContext.activeOrgUnitId),
);
let organizations = $state<{ id: string; label: string }[]>([]);

const orgNameDisplay = $derived(l(orgNameAr, orgName) ?? "");
const orgInitials = $derived(orgNameDisplay.trim().charAt(0).toUpperCase());
const currentBranchLabel = $derived.by(() => {
  if (!activeBranch)
    return access.data?.organization_wide ? m.shared_ui_all_branches() : "";
  const currentBranch = branches.find((branch) => branch.id === activeBranch);
  return currentBranch
    ? l(currentBranch.name_ar, currentBranch.name)
    : m.shared_ui_selected_branch();
});

onMount(() => {
  try {
    const savedOrganizations = localStorage.getItem("orbis_custom_orgs");
    if (savedOrganizations) {
      const parsed: unknown = JSON.parse(savedOrganizations);
      if (Array.isArray(parsed)) {
        organizations = parsed.filter(
          (item): item is { id: string; label: string } => {
            if (typeof item !== "object" || item === null) return false;
            return (
              "id" in item &&
              typeof item.id === "string" &&
              item.id.length > 0 &&
              "label" in item &&
              typeof item.label === "string" &&
              item.label.length > 0
            );
          },
        );
      }
    }
  } catch (error) {
    console.error("Failed to load saved organizations:", error);
  }

  const currentOrgId = authStore.orgId || orgId;
  if (currentOrgId && orgNameDisplay) {
    const currentOrganization = { id: currentOrgId, label: orgNameDisplay };
    organizations = organizations.some((org) => org.id === currentOrgId)
      ? organizations.map((org) =>
          org.id === currentOrgId ? currentOrganization : org,
        )
      : [currentOrganization, ...organizations];
  }
});

function handleNavigate(path: string) {
  return goto(resolve(localizeHref(path) as Pathname));
}

function handleBranchSelect(id: string) {
  if (!branches.some((branch) => branch.id === id)) return;
  effectiveContext.setOrgUnit(id);
  effectiveContext.setClass(null);
  return handleNavigate(`/${orgId}`);
}

function handleOrganizationSelect(id: string) {
  effectiveContext.setOrgUnit(null);
  effectiveContext.setClass(null);
  return handleNavigate(`/${id}`);
}

function handleAllBranchesSelect() {
  if (!access.data?.organization_wide) return;
  effectiveContext.setOrgUnit(null);
  effectiveContext.setClass(null);
  return handleNavigate(`/${orgId}`);
}
</script>

<DropdownMenu>
  <DropdownMenuTrigger class="outline-none">
    <div class="flex">
      <Tooltip>
        <TooltipTrigger>
          <button
            class="w-8 h-8 rounded-md outline-none select-none cursor-pointer btn-tactile p-0 border-none flex items-center justify-center relative"
            style="--btn-shadow: color-mix(in srgb, {primaryColor} 55%, black); background-color: {primaryColor};"
            aria-label={orgNameDisplay}
          >
            <Avatar class="h-full w-full rounded-md">
              <AvatarFallback
                class="font-bold text-lg text-white"
                style="background-color: {primaryColor};"
              >
                {orgInitials}
              </AvatarFallback>
            </Avatar>
          </button>
        </TooltipTrigger>
        <TooltipContent side="inline-end" sideOffset={12}>
          {orgNameDisplay}
        </TooltipContent>
      </Tooltip>
    </div>
  </DropdownMenuTrigger>

  <DropdownMenuContent align="start" side="inline-end" sideOffset={12} class="w-56 p-1 z-50">
    <div class="flex items-center gap-3 px-3 py-2.5 border-b border-border/50">
      <Avatar class="w-8 h-8 rounded-md" style="background-color: {primaryColor};">
        <AvatarFallback class="font-bold text-md text-white" style="background-color: {primaryColor};">
          {orgInitials}
        </AvatarFallback>
      </Avatar>
      <div class="flex flex-col min-w-0 gap-2 leading-tight">
        <span class="text-xs font-bold uppercase tracking-wider text-foreground truncate">{orgNameDisplay}</span>
        <span class="text-[9px] text-muted-foreground uppercase font-semibold">{branches.length} {branches.length === 1 ? m.common_header_org_branch() : m.common_header_org_branches()}</span>
      </div>
    </div>

    <div class="py-1">
      {#if organizations.length > 1}
        <DropdownMenuSub>
          <DropdownMenuSubTrigger class="cursor-pointer px-3 py-1.5 text-xs rounded">
            {m.common_header_switch_org()}
          </DropdownMenuSubTrigger>
          <DropdownMenuSubContent class="w-52 p-1 z-50 shadow-md rounded-md">
            {#each organizations as organization (organization.id)}
              <DropdownMenuItem
                class="cursor-pointer px-3 py-1.5 text-xs rounded"
                onclick={() => handleOrganizationSelect(organization.id)}
              >
                {organization.label}
              </DropdownMenuItem>
            {/each}
          </DropdownMenuSubContent>
        </DropdownMenuSub>
      {/if}

      {#if branches.length > 1 || (access.data?.organization_wide && branches.length > 0)}
      <DropdownMenuSub>
        <DropdownMenuSubTrigger class="cursor-pointer px-3 py-1.5 text-xs rounded">
          <span>{m.dashboard_branch_filter()}</span>
          <span class="ms-auto max-w-24 truncate text-[10px] text-muted-foreground">
            {currentBranchLabel}
          </span>
        </DropdownMenuSubTrigger>
        <DropdownMenuSubContent class="w-52 p-1 z-50 shadow-md rounded-md">
          {#if access.data?.organization_wide}
          <DropdownMenuItem
            class="cursor-pointer px-3 py-1.5 text-xs rounded"
            onclick={handleAllBranchesSelect}
          >
            {m.shared_ui_all_branches()}
          </DropdownMenuItem>
          {/if}
          {#each branches as branch (branch.id)}
            <DropdownMenuItem
              class="cursor-pointer px-3 py-1.5 text-xs rounded"
              onclick={() => handleBranchSelect(branch.id)}
            >
              {l(branch.name_ar, branch.name)}
            </DropdownMenuItem>
          {/each}
        </DropdownMenuSubContent>
      </DropdownMenuSub>
      {/if}

      <DropdownMenuSeparator class="bg-border/50" />

      <DropdownMenuItem class="cursor-pointer px-3 py-1.5 text-xs rounded" onclick={() => handleNavigate(`/${orgId}/settings`)}>
        {m.common_header_org_settings()}
      </DropdownMenuItem>
      <DropdownMenuItem class="cursor-pointer px-3 py-1.5 text-xs rounded" onclick={() => handleNavigate(`/${orgId}/settings/branches`)}>
        {m.common_header_org_branches()}
      </DropdownMenuItem>
      <DropdownMenuItem class="cursor-pointer px-3 py-1.5 text-xs rounded" onclick={() => handleNavigate(`/${orgId}/settings/users`)}>
        {m.common_header_org_members()}
      </DropdownMenuItem>
      <DropdownMenuItem class="cursor-pointer px-3 py-1.5 text-xs rounded" onclick={() => handleNavigate(`/${orgId}/reports`)}>
        {m.common_nav_reports()}
      </DropdownMenuItem>
      <DropdownMenuItem class="cursor-pointer px-3 py-1.5 text-xs rounded" onclick={() => handleNavigate(`/${orgId}/billing`)}>
        {m.common_header_org_billing()}
      </DropdownMenuItem>
      <DropdownMenuItem class="cursor-pointer px-3 py-1.5 text-xs rounded" onclick={() => api.connectionOpenWindow()}>
        {m.settings_connection_settings()}
      </DropdownMenuItem>
    </div>
  </DropdownMenuContent>
</DropdownMenu>
