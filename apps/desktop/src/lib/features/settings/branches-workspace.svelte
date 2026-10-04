<!--
@component BranchesWorkspace
Clean, comprehensive Multi-Branch Management & Institutional Governance Center.
Matches the Orbis Settings system design, with inner sub-pages for each branch (classes, rooms, sub-units, staff, stats).
-->
<script lang="ts">
import { onMount } from "svelte";
import * as m from "$lib/paraglide/messages";
import { api, type OrgUnitDto, type UserDto, extractErrorMessage } from "$lib/api";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Badge } from "@orbis/ui/badge";
import { Select, SelectTrigger, SelectContent, SelectItem } from "@orbis/ui/select";
import { Tabs, TabsList, TabsTrigger, TabsContent } from "@orbis/ui/tabs";
import { toast } from "@orbis/ui";
import {
  BuildingOffice,
  Plus,
  Search,
  Refresh,
  ShieldCheck,
} from "@orbis/icons";
import { OrbisGrid, type OrbisColumnDef, type ActionItem, type GridActionEvent } from "@orbis/svelte-grid";
import { l, isAr, formatDate } from "$lib/core/i18n";
import { confirmStore } from "$lib/stores";

import BranchSheet from "./components/branch-sheet.svelte";
import BranchSettingsCard from "./components/branch-settings-card.svelte";
import BranchDetailsView from "./components/branch-details-view.svelte";

let { orgId }: { orgId: string } = $props();

// Data State
let allUnits = $state<OrgUnitDto[]>([]);
let staffList = $state<UserDto[]>([]);
let currentOrg = $state<any>(null);
let isLoading = $state(true);
let isRefreshing = $state(false);

// View & Navigation State
let selectedBranch = $state<OrgUnitDto | null>(null);
let activeTab = $state<"branches" | "governance">("branches");
let viewMode = $state<"grid" | "cards">("grid");
let searchQuery = $state("");
let statusFilter = $state<"all" | "active" | "inactive">("all");
let isolationFilter = $state<"all" | "shared" | "isolated" | "read_only">("all");

// Sheets State
let isBranchSheetOpen = $state(false);
let editingBranch = $state<OrgUnitDto | null>(null);

async function loadData() {
  isLoading = true;
  try {
    const [unitsRes, usersRes, orgRes] = await Promise.all([
      api.listOrgUnits().catch(() => []),
      api.listUsers().catch(() => []),
      api.getCurrentOrg().catch(() => null),
    ]);

    allUnits = Array.isArray(unitsRes) ? unitsRes : [];
    staffList = Array.isArray(usersRes) ? usersRes : [];
    currentOrg = orgRes;

    // Update selectedBranch if it was open
    if (selectedBranch) {
      const found = allUnits.find((u) => u.id === selectedBranch?.id);
      if (found) selectedBranch = found;
    }
  } catch (err) {
    toast.error(extractErrorMessage(err));
  } finally {
    isLoading = false;
  }
}

async function refreshData() {
  isRefreshing = true;
  try {
    await loadData();
    toast.success(m.ui_copy_data_refreshed_362b604a());
  } catch (err) {
    toast.error(extractErrorMessage(err));
  } finally {
    isRefreshing = false;
  }
}

onMount(() => {
  if (orgId) loadData();
});

// Branches list
const branches = $derived(
  allUnits.filter((u) => u.unit_type === "branch")
);

// Read default branch ID from org settings
const defaultBranchId = $derived(() => {
  const s = currentOrg?.settings;
  const parsed = typeof s === "string" ? (() => { try { return JSON.parse(s); } catch { return {}; } })() : s || {};
  return parsed?.branch_policy?.default_branch_id || null;
});

// Filtered Branches
const filteredBranches = $derived.by(() => {
  let list = branches;

  if (searchQuery.trim()) {
    const q = searchQuery.toLowerCase().trim();
    list = list.filter((b) => {
      const nameMatch = (b.name_ar || "").toLowerCase().includes(q) || (b.name || "").toLowerCase().includes(q);
      const codeMatch = (b.code || "").toLowerCase().includes(q);
      const manager = staffList.find((s) => s.id === b.head_id);
      const managerMatch = manager ? (manager.name_ar || manager.full_name || manager.username).toLowerCase().includes(q) : false;
      return nameMatch || codeMatch || managerMatch;
    });
  }

  if (statusFilter === "active") {
    list = list.filter((b) => b.is_active);
  } else if (statusFilter === "inactive") {
    list = list.filter((b) => !b.is_active);
  }

  if (isolationFilter !== "all") {
    list = list.filter((b) => b.data_isolation === isolationFilter);
  }

  return list;
});

// Staff Map
const staffMap = $derived(new Map(staffList.map((s) => [s.id, s])));

// Actions for Grid View
const gridActions: ActionItem[] = [
  { id: "details", label: m.ui_copy_details_c4833503(), variant: "default" },
  { id: "edit", label: m.academics_schedule_edit(), variant: "default" },
  { id: "delete", label: m.common_action_delete(), variant: "destructive" },
];

const gridSource = $derived(
  filteredBranches.map((b, i) => {
    const mgr = b.head_id ? staffMap.get(b.head_id) : null;
    return {
      ...b,
      __idx: i + 1,
      displayName: b.name_ar || b.name,
      displayCode: b.code || "—",
      managerName: mgr ? (mgr.name_ar || mgr.full_name || mgr.username) : m.common_unassigned(),
      displayCapacity: b.capacity ? b.capacity.toLocaleString() : m.ui_copy_uncapped_eb8a96f3(),
      subUnitsCount: b.children_count || 0,
      isolationLabel: b.data_isolation === "isolated" ? m.ui_copy_isolated_09d34bff() : b.data_isolation === "read_only" ? m.ui_copy_read_only_05cda70d() : m.ui_copy_shared_cdf39825(),
      statusLabel: b.is_active ? m.common_phase_active() : m.common_status_disabled(),
      created_at_display: b.created_at ? formatDate(b.created_at, "datetime") : "—",
    };
  })
);

const columns: OrbisColumnDef[] = [
  { id: "__idx", prop: "__idx", header: "#", name: "#", width: 50, size: 50, readonly: true, pin: "left", align: "center" },
  { id: "displayName", prop: "displayName", header: m.ui_copy_branch_name_954ef523(), name: "Branch Name", width: 220, size: 220, readonly: true, pin: "left", cellClass: "font-semibold text-foreground cursor-pointer hover:underline" },
  { id: "displayCode", prop: "displayCode", header: m.ui_copy_code_3119a46b(), name: "Code", width: 110, size: 110, readonly: true, cellClass: "font-mono text-xs uppercase" },
  { id: "managerName", prop: "managerName", header: m.ui_copy_manager_a230935a(), name: "Manager", width: 180, size: 180, readonly: true },
  { id: "displayCapacity", prop: "displayCapacity", header: m.ui_copy_capacity_6432d6d2(), name: "Capacity", width: 120, size: 120, readonly: true, align: "center", type: "number" },
  { id: "subUnitsCount", prop: "subUnitsCount", header: m.ui_copy_sub_units_93f58eca(), name: "Sub-units", width: 110, size: 110, readonly: true, align: "center", type: "number" },
  { id: "isolationLabel", prop: "isolationLabel", header: m.ui_copy_isolation_5fef6f7f(), name: "Isolation", width: 120, size: 120, readonly: true, align: "center" },
  { id: "statusLabel", prop: "statusLabel", header: m.common_status(), name: "Status", width: 100, size: 100, readonly: true, align: "center", type: "badge" },
  { id: "created_at_display", prop: "created_at_display", header: m.ui_copy_created_at_01f6ea5f(), name: "Created At", width: 140, size: 140, readonly: true, cellClass: "font-mono text-xs text-muted-foreground" },
  { id: "actions", prop: "actions", header: m.ui_copy_actions_c48127a5(), name: "Actions", width: 140, size: 140, type: "action", align: "center", readonly: true, meta: { items: gridActions } },
];

function handleGridAction(event: CustomEvent<GridActionEvent>) {
  const { actionId, row } = event.detail;
  const branch = branches.find((b) => b.id === row.id);
  if (!branch) return;

  if (actionId === "details" || actionId === "dblclick") {
    selectedBranch = branch;
  } else if (actionId === "edit") {
    openEditBranch(branch);
  } else if (actionId === "delete") {
    promptDeleteBranch(branch);
  }
}

function openCreateBranch() {
  editingBranch = null;
  isBranchSheetOpen = true;
}

function openEditBranch(branch: OrgUnitDto) {
  editingBranch = branch;
  isBranchSheetOpen = true;
}

function promptDeleteBranch(branch: OrgUnitDto) {
  const children = allUnits.filter((u) => u.parent_id === branch.id);
  if (children.length > 0) {
    toast.error(
      l(
        m.settings_ui_branch_delete_has_children({ count: children.length }),
        `Cannot delete branch because it contains (${children.length}) child units.`
      )
    );
    return;
  }

  confirmStore.open({
    title: m.ui_copy_delete_branch_f22fe5e8(),
    message: l(
      m.settings_ui_branch_delete_confirm({ name: branch.name_ar || branch.name }),
      `Are you sure you want to permanently delete branch "${branch.name_ar || branch.name}"?`
    ),
    confirmLabel: m.settings_currencies_btn_delete_confirm(),
    variant: "destructive",
    onConfirm: async () => {
      try {
        await api.deleteOrgUnit(branch.id);
        toast.success(m.ui_copy_branch_deleted_successfully_09d39e40());
        if (selectedBranch?.id === branch.id) selectedBranch = null;
        loadData();
      } catch (err) {
        toast.error(extractErrorMessage(err));
      }
    },
  });
}

async function setDefaultBranch(branch: OrgUnitDto) {
  try {
    const s = currentOrg?.settings;
    const currentSettings = typeof s === "string" ? (() => { try { return JSON.parse(s); } catch { return {}; } })() : s || {};
    const updatedPolicy = {
      ...(currentSettings?.branch_policy || {}),
      default_branch_id: branch.id,
    };
    await api.updateSettings({ branch_policy: updatedPolicy });
    toast.success(m.ui_branch_set_default_success({ branchName: branch.name_ar || branch.name }));
    loadData();
  } catch (err) {
    toast.error(extractErrorMessage(err));
  }
}
</script>

<svelte:head>
  <title>{m.ui_copy_branch_management_db17ea37()} | Orbis</title>
</svelte:head>

<div class="h-full w-full flex flex-col gap-4 overflow-y-auto p-4 scrollbar-hide text-start font-sans">
  {#if selectedBranch}
    <!-- Inner Sub-page / Detailed View of the selected Branch -->
    <BranchDetailsView
      {orgId}
      branch={selectedBranch}
      onBack={() => (selectedBranch = null)}
      onEdit={openEditBranch}
    />
  {:else}
    <!-- Main Workspace -->
    <div class="flex items-center justify-between gap-3 border-b border-border/50 pb-3">
      <div class="flex items-center gap-3">
        <Tabs value={activeTab} onValueChange={(val: any) => (activeTab = val)} class="w-auto">
          <TabsList class="bg-muted/40 p-1 border border-border/50 rounded-xl">
            <TabsTrigger value="branches" class="gap-2 text-xs font-semibold px-4 py-2">
              <BuildingOffice size={15} />
              <span>{m.ui_copy_branches_8e6f7b91()} ({branches.length})</span>
            </TabsTrigger>
            <TabsTrigger value="governance" class="gap-2 text-xs font-semibold px-4 py-2">
              <ShieldCheck size={15} />
              <span>{m.ui_copy_branch_policies_6c418f2c()}</span>
            </TabsTrigger>
          </TabsList>
        </Tabs>
      </div>

      <div class="flex items-center gap-2">
        <Button
          variant="outline"
          size="sm"
          onclick={refreshData}
          disabled={isRefreshing || isLoading}
          class="h-9 px-3 text-xs gap-1.5 cursor-pointer"
        >
          <Refresh size={13} class={isRefreshing ? "animate-spin" : ""} />
          <span>{m.common_refresh()}</span>
        </Button>

        <Button
          variant="default"
          size="sm"
          onclick={openCreateBranch}
          class="h-9 px-4 text-xs font-semibold gap-1.5 shadow-sm cursor-pointer"
        >
          <Plus size={15} />
          <span>{m.ui_copy_add_branch_bbb91ff4()}</span>
        </Button>
      </div>
    </div>

    <!-- Tab 1: Branches Directory -->
    {#if activeTab === "branches"}
      <!-- Search & Filters Toolbar -->
      <div class="flex flex-col md:flex-row items-stretch md:items-center justify-between gap-3">
        <div class="flex flex-1 items-center gap-2.5">
          <div class="relative flex-1 max-w-sm">
            <Search size={14} class="absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
            <Input
              type="text"
              bind:value={searchQuery}
              placeholder={m.ui_copy_search_branches_24f7a872()}
              class="h-9 text-xs pr-9 pl-3 bg-card border-border/60"
            />
          </div>

          <!-- Status Filter -->
          <Select type="single" value={statusFilter} onValueChange={(val: any) => (statusFilter = val || "all")}>
            <SelectTrigger class="h-9 text-xs w-36 bg-card border-border/60">
              {statusFilter === "active" ? m.ui_copy_active_only_327db595() : statusFilter === "inactive" ? m.ui_copy_inactive_only_acc30677() : m.ui_copy_all_statuses_384fd23c()}
            </SelectTrigger>
            <SelectContent class="text-xs">
              <SelectItem value="all">{m.ui_copy_all_statuses_384fd23c()}</SelectItem>
              <SelectItem value="active">{m.ui_copy_active_only_327db595()}</SelectItem>
              <SelectItem value="inactive">{m.ui_copy_inactive_only_acc30677()}</SelectItem>
            </SelectContent>
          </Select>

          <!-- Isolation Filter -->
          <Select type="single" value={isolationFilter} onValueChange={(val: any) => (isolationFilter = val || "all")}>
            <SelectTrigger class="h-9 text-xs w-36 bg-card border-border/60">
              {isolationFilter === "isolated" ? m.ui_copy_isolated_09d34bff() : isolationFilter === "read_only" ? m.ui_copy_read_only_05cda70d() : isolationFilter === "shared" ? m.ui_copy_shared_cdf39825() : m.ui_copy_all_isolations_7849dd7e()}
            </SelectTrigger>
            <SelectContent class="text-xs">
              <SelectItem value="all">{m.ui_copy_all_isolations_7849dd7e()}</SelectItem>
              <SelectItem value="shared">{m.ui_copy_shared_cdf39825()}</SelectItem>
              <SelectItem value="isolated">{m.ui_copy_isolated_09d34bff()}</SelectItem>
              <SelectItem value="read_only">{m.ui_copy_read_only_05cda70d()}</SelectItem>
            </SelectContent>
          </Select>
        </div>

        <!-- View Switcher -->
        <div class="flex items-center gap-1 bg-muted/40 p-1 rounded-lg border border-border/40 shrink-0">
          <button
            type="button"
            onclick={() => (viewMode = "grid")}
            class="p-1.5 rounded-md transition-colors {viewMode === 'grid' ? 'bg-background shadow-xs text-primary' : 'text-muted-foreground hover:text-foreground'}"
            title={m.ui_copy_grid_view_eb56e45c()}
          >
          </button>
          <button
            type="button"
            onclick={() => (viewMode = "cards")}
            class="p-1.5 rounded-md transition-colors {viewMode === 'cards' ? 'bg-background shadow-xs text-primary' : 'text-muted-foreground hover:text-foreground'}"
            title={m.ui_copy_cards_view_9f9064c5()}
          >
          </button>
        </div>
      </div>

      <!-- Main Content -->
      {#if isLoading}
        <div class="h-[520px] w-full rounded-xl border border-border/50 bg-muted/10 flex items-center justify-center">
          <span class="text-xs text-muted-foreground">{m.ui_copy_loading_branches_bf291b7e()}</span>
        </div>
      {:else if viewMode === "grid"}
        <div class="h-[560px] w-full rounded-xl border border-border/50 overflow-hidden bg-card/50">
          <OrbisGrid
            source={gridSource}
            {columns}
            enableExport={true}
            on:action={handleGridAction}
          />
        </div>
      {:else}
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {#each filteredBranches as branch (branch.id)}
            <BranchCard
              {branch}
              isDefault={defaultBranchId() === branch.id}
              manager={branch.head_id ? staffMap.get(branch.head_id) : null}
              onEdit={openEditBranch}
              onDelete={promptDeleteBranch}
              onSetDefault={setDefaultBranch}
              onViewHierarchy={(b) => (selectedBranch = b)}
            />
          {/each}
        </div>
      {/if}
    {:else}
      <!-- Tab 2: Branch Governance & Policies -->
      <div class="max-w-4xl pt-2">
        <BranchSettingsCard org={currentOrg} />
      </div>
    {/if}
  {/if}
</div>

<!-- Slide-over Sheet for Branch Create / Edit -->
<BranchSheet
  bind:open={isBranchSheetOpen}
  {orgId}
  branch={editingBranch}
  onSuccess={() => {
    loadData();
  }}
/>
