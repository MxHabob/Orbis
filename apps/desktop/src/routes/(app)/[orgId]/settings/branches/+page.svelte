<!--
@component BranchesOverviewPage
Main root overview page for branches management.
Matches the FinanceOverview layout and design aesthetic with top KPI cards,
toolbar controls, and an advanced OrbisGrid / cards directory.
-->
<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import { page } from "$app/state";
import { goto } from "$app/navigation";
import { onMount } from "svelte";
import * as m from "$lib/paraglide/messages";
import {
  api,
  type OrgUnitDto,
  type EmployeeDto,
  extractErrorMessage,
} from "$lib/api";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Badge } from "@orbis/ui/badge";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
} from "@orbis/ui/card";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import { toast } from "@orbis/ui";
import {
  BuildingOffice,
  BuildingLibrary,
  Plus,
  Search,
  Refresh,
  Keyframes,
  ShieldCheck,
  Users,
  Star,
  CheckCircle,
  Pencil,
  Eye,
} from "@orbis/icons";
import {
  OrbisGrid,
  type OrbisColumnDef,
  type ActionItem,
  type GridActionEvent,
} from "@orbis/svelte-grid";
import { l, isAr, formatDate, formatNumber } from "$lib/core/i18n";
import { confirmStore } from "$lib/stores";
import BranchSheet from "$lib/features/settings/components/branch-sheet.svelte";

let orgId = $derived((page.params.orgId as string) || "");

// Data State
let allUnits = $state<OrgUnitDto[]>([]);
let employeesList = $state<EmployeeDto[]>([]);
let currentOrg = $state<any>(null);
let isLoading = $state(true);
let isRefreshing = $state(false);

// View & Filter State
let searchQuery = $state("");
let statusFilter = $state<"all" | "active" | "inactive">("all");
let isolationFilter = $state<"all" | "shared" | "isolated" | "read_only">(
  "all",
);

// Sheet State
let isBranchSheetOpen = $state(false);
let editingBranch = $state<OrgUnitDto | null>(null);

function formatEmployeeName(e?: EmployeeDto | null): string {
  if (!e) return m.common_unassigned();
  const ar = [e.first_name_ar, e.last_name_ar].filter(Boolean).join(" ");
  const en = [e.first_name, e.last_name].filter(Boolean).join(" ");
  const localized = l(ar, en);
  if (localized) return localized;
  return e.full_name || e.name || e.employee_number || "—";
}

async function loadData() {
  if (!orgId) return;
  isLoading = true;
  try {
    const [unitsRes, empRes, orgRes] = await Promise.all([
      api.listOrgUnits().catch(() => []),
      api.listEmployees().catch(() => []),
      api.getCurrentOrg().catch(() => null),
    ]);

    allUnits = Array.isArray(unitsRes) ? unitsRes : [];
    employeesList = Array.isArray(empRes) ? empRes : [];
    currentOrg = orgRes;
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
    toast.success(m.ui_copy_branches_refreshed_50ee51e2());
  } catch (err) {
    toast.error(extractErrorMessage(err));
  } finally {
    isRefreshing = false;
  }
}

onMount(() => {
  loadData();
});

// Derived Branches
const branches = $derived(allUnits.filter((u) => u.unit_type === "branch"));

// Read default branch ID from settings
const defaultBranchId = $derived(() => {
  const s = currentOrg?.settings;
  const parsed =
    typeof s === "string"
      ? (() => {
          try {
            return JSON.parse(s);
          } catch {
            return {};
          }
        })()
      : s || {};
  return parsed?.branch_policy?.default_branch_id || null;
});

// Derived KPI Computations (Matching FinanceOverview)
const totalBranchesCount = $derived(branches.length);
const activeBranchesCount = $derived(
  branches.filter((b) => b.is_active).length,
);
const totalCapacity = $derived(
  branches.reduce((sum, b) => sum + (b.capacity || 0), 0),
);
const branchPolicyIsolation = $derived(() => {
  const s = currentOrg?.settings;
  const parsed =
    typeof s === "string"
      ? (() => {
          try {
            return JSON.parse(s);
          } catch {
            return {};
          }
        })()
      : s || {};
  return parsed?.branch_policy?.data_isolation || "shared";
});

// Employee Map
const employeeMap = $derived(new Map(employeesList.map((e) => [e.id, e])));

// Filtered Branches
const filteredBranches = $derived.by(() => {
  let list = branches;

  if (searchQuery.trim()) {
    const q = searchQuery.toLowerCase().trim();
    list = list.filter((b) => {
      const nameMatch =
        (b.name_ar || "").toLowerCase().includes(q) ||
        (b.name || "").toLowerCase().includes(q);
      const codeMatch = (b.code || "").toLowerCase().includes(q);
      const manager = b.head_id ? employeeMap.get(b.head_id) : null;
      const managerMatch = manager
        ? formatEmployeeName(manager).toLowerCase().includes(q)
        : false;
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

// Actions for Grid View
const gridActions: ActionItem[] = [
  { id: "details", label: m.ui_copy_details_c4833503(), variant: "default" },
  { id: "edit", label: m.academics_schedule_edit(), variant: "default" },
  { id: "delete", label: m.common_action_delete(), variant: "destructive" },
];

const gridSource = $derived(
  filteredBranches.map((b, i) => {
    const mgr = b.head_id ? employeeMap.get(b.head_id) : null;
    return {
      ...b,
      __idx: i + 1,
      displayName: b.name_ar || b.name,
      displayCode: b.code || "—",
      managerName: mgr ? formatEmployeeName(mgr) : m.common_unassigned(),
      displayCapacity: b.capacity
        ? formatNumber(b.capacity)
        : m.ui_copy_uncapped_eb8a96f3(),
      subUnitsCount: b.children_count || 0,
      isolationLabel:
        b.data_isolation === "isolated"
          ? m.ui_copy_isolated_09d34bff()
          : b.data_isolation === "read_only"
            ? m.ui_copy_read_only_05cda70d()
            : m.ui_copy_shared_cdf39825(),
      statusLabel: b.is_active
        ? m.common_phase_active()
        : m.common_status_disabled(),
      created_at_display: b.created_at
        ? formatDate(b.created_at, "datetime")
        : "—",
    };
  }),
);

const columns: OrbisColumnDef[] = [
  {
    id: "__idx",
    prop: "__idx",
    header: "#",
    name: "#",
    width: 50,
    size: 50,
    readonly: true,
    pin: "left",
    align: "center",
  },
  {
    id: "displayName",
    prop: "displayName",
    header: m.ui_copy_branch_name_954ef523(),
    name: "Branch Name",
    width: 220,
    size: 220,
    readonly: true,
    pin: "left",
    cellClass: "font-semibold text-foreground cursor-pointer hover:underline",
  },
  {
    id: "displayCode",
    prop: "displayCode",
    header: m.ui_copy_code_3119a46b(),
    name: "Code",
    width: 110,
    size: 110,
    readonly: true,
    cellClass: "font-mono text-xs uppercase",
  },
  {
    id: "managerName",
    prop: "managerName",
    header: m.ui_copy_manager_a230935a(),
    name: "Manager",
    width: 180,
    size: 180,
    readonly: true,
  },
  {
    id: "displayCapacity",
    prop: "displayCapacity",
    header: m.ui_copy_capacity_6432d6d2(),
    name: "Capacity",
    width: 120,
    size: 120,
    readonly: true,
    align: "center",
    type: "number",
  },
  {
    id: "subUnitsCount",
    prop: "subUnitsCount",
    header: m.ui_copy_sub_units_93f58eca(),
    name: "Sub-units",
    width: 110,
    size: 110,
    readonly: true,
    align: "center",
    type: "number",
  },
  {
    id: "isolationLabel",
    prop: "isolationLabel",
    header: m.ui_copy_isolation_5fef6f7f(),
    name: "Isolation",
    width: 120,
    size: 120,
    readonly: true,
    align: "center",
  },
  {
    id: "statusLabel",
    prop: "statusLabel",
    header: m.common_status(),
    name: "Status",
    width: 100,
    size: 100,
    readonly: true,
    align: "center",
    type: "badge",
  },
  {
    id: "created_at_display",
    prop: "created_at_display",
    header: m.ui_copy_created_at_01f6ea5f(),
    name: "Created At",
    width: 140,
    size: 140,
    readonly: true,
    cellClass: "font-mono text-xs text-muted-foreground",
  },
  {
    id: "actions",
    prop: "actions",
    header: m.ui_copy_actions_c48127a5(),
    name: "Actions",
    width: 140,
    size: 140,
    type: "action",
    align: "center",
    readonly: true,
    meta: { items: gridActions },
  },
];

function handleGridAction(event: CustomEvent<GridActionEvent>) {
  const { actionId, row } = event.detail;
  const branch = branches.find((b) => b.id === row.id);
  if (!branch) return;

  if (actionId === "details" || actionId === "dblclick") {
    goto(`/${orgId}/settings/branches/${branch.id}`);
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
      m.settings_branch_delete_with_children({ count: children.length }),
    );
    return;
  }

  confirmStore.prompt({
    title: m.ui_copy_delete_branch_f22fe5e8(),
    description: l(
      `هل أنت متأكد من رغبتك في حذف فرع "${branch.name_ar || branch.name}" بشكل نهائي؟`,
      `Are you sure you want to permanently delete branch "${branch.name_ar || branch.name}"?`,
    ),
    confirmText: m.settings_currencies_btn_delete_confirm(),
    variant: "destructive",
    onConfirm: async () => {
      try {
        await api.deleteOrgUnit(branch.id);
        toast.success(m.ui_copy_branch_deleted_successfully_09d39e40());
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
    const currentSettings =
      typeof s === "string"
        ? (() => {
            try {
              return JSON.parse(s);
            } catch {
              return {};
            }
          })()
        : s || {};
    const updatedPolicy = {
      ...(currentSettings?.branch_policy || {}),
      default_branch_id: branch.id,
    };
    await api.updateSettings({ branch_policy: updatedPolicy });
    toast.success(
      m.ui_branch_set_default_success({
        branchName: branch.name_ar || branch.name,
      }),
    );
    loadData();
  } catch (err) {
    toast.error(extractErrorMessage(err));
  }
}
</script>

<svelte:head>
  <title>{m.ui_copy_branches_directory_cf4c7981()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto p-6 scrollbar-hide text-start font-sans flex flex-col gap-6">
  <!-- 1. Top KPI Summary Metric Cards (Matching FinanceOverview Style) -->
  <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
    <!-- Total Branches -->
    <Card class="border border-border/50 bg-card/60 shadow-xs">
      <CardContent class="p-4 flex items-center justify-between">
        <div class="space-y-1">
          <span class="text-xs font-medium text-muted-foreground">{m.ui_copy_total_branches_4da6c8fb()}</span>
          <div class="text-2xl font-bold tabular-nums text-foreground">{totalBranchesCount}</div>
          <span class="text-[11px] text-muted-foreground">{m.ui_copy_registered_campuses_d360c39e()}</span>
        </div>
      </CardContent>
    </Card>

    <!-- Active Branches -->
    <Card class="border border-border/50 bg-card/60 shadow-xs">
      <CardContent class="p-4 flex items-center justify-between">
        <div class="space-y-1">
          <span class="text-xs font-medium text-muted-foreground">{m.ui_copy_active_branches_b8974004()}</span>
          <div class="text-2xl font-bold tabular-nums text-emerald-600 dark:text-emerald-400">{activeBranchesCount}</div>
          <span class="text-[11px] text-muted-foreground">
            {totalBranchesCount > 0 ? `${Math.round((activeBranchesCount / totalBranchesCount) * 100)}% ${m.ui_copy_readiness_953368e7()}` : m.ui_copy_no_branches_30da1591()}
          </span>
        </div>
      </CardContent>
    </Card>

    <!-- Total Student Capacity -->
    <Card class="border border-border/50 bg-card/60 shadow-xs">
      <CardContent class="p-4 flex items-center justify-between">
        <div class="space-y-1">
          <span class="text-xs font-medium text-muted-foreground">{m.ui_copy_total_capacity_31f26962()}</span>
          <div class="text-2xl font-bold tabular-nums text-foreground">{formatNumber(totalCapacity)}</div>
          <span class="text-[11px] text-muted-foreground">{m.ui_copy_available_seats_1a55ae77()}</span>
        </div>
      </CardContent>
    </Card>

    <!-- Isolation Policy Scope -->
    <Card class="border border-border/50 bg-card/60 shadow-xs">
      <CardContent class="p-4 flex items-center justify-between">
        <div class="space-y-1">
          <span class="text-xs font-medium text-muted-foreground">{m.ui_copy_governance_scope_2634f5c1()}</span>
          <div class="text-base font-bold text-foreground truncate mt-0.5">
            {branchPolicyIsolation() === "isolated" ? m.ui_copy_strict_isolated_85a49f59() : m.ui_copy_shared_domain_398dfdba()}
          </div>
          <span class="text-[11px] text-muted-foreground">
            {branchPolicyIsolation() === "isolated" ? m.ui_copy_per_branch_silos_a3ad4f5c() : m.ui_copy_multi_branch_7bd0b46a()}
          </span>
        </div>
      </CardContent>
    </Card>
  </div>

  <!-- 2. Actions & Filters Toolbar -->
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

    <div class="flex items-center gap-2">
      <Button
        variant="outline"
        size="sm"
        onclick={refreshData}
        disabled={isRefreshing || isLoading}
        class="h-9 px-3 text-xs gap-1.5 cursor-pointer"
      >
        {#if isRefreshing}<LoadingIndicator  size={13} />{:else}<Refresh size={13}   />{/if}
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

  <!-- 3. Main Content (OrbisGrid / Cards) -->
  {#if isLoading}
    <div class="h-[520px] w-full rounded-xl border border-border/50 bg-muted/10 flex items-center justify-center">
      <span class="text-xs text-muted-foreground">{m.ui_copy_loading_branches_bf291b7e()}</span>
    </div>
  {:else if filteredBranches.length === 0}
    <div class="flex flex-col items-center justify-center py-16 px-4 text-center rounded-2xl border border-dashed border-border/60 bg-card/30">
      <div class="size-16 rounded-2xl bg-muted/40 flex items-center justify-center text-muted-foreground mb-4">
        <BuildingOffice size={32} />
      </div>
      <h3 class="text-base font-bold text-foreground">
        {searchQuery ? m.ui_copy_no_matching_branches_found_2924a375() : m.ui_copy_no_branches_added_yet_abb40289()}
      </h3>
      <p class="text-xs text-muted-foreground max-w-sm mt-1 mb-5">
        {searchQuery ? m.ui_copy_try_adjusting_your_search_query_or_reset_a_49e3d03a() : m.ui_copy_add_your_first_campus_to_organize_students_d05e68eb()}
      </p>
      {#if !searchQuery}
        <Button size="sm" onclick={openCreateBranch} class="gap-1.5 text-xs font-semibold">
          <Plus size={14} />
          <span>{m.ui_copy_add_first_branch_eb4f1ecb()}</span>
        </Button>
      {/if}
    </div>
  {:else}
    <div class="h-[560px] w-full overflow-hidden">
      <OrbisGrid
        source={gridSource}
        {columns}
        enableExport={true}
        on:action={handleGridAction}
      />
    </div>
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
