<!--
@component BranchesOverviewPage
Main root overview page for branches management.
Matches the FinanceOverview layout and design aesthetic with top KPI cards,
toolbar controls, and an advanced OrbisGrid / cards directory.
-->
<script lang="ts">
import { page } from "$app/state";
import { goto } from "$app/navigation";
import { onMount } from "svelte";
import * as m from "$lib/paraglide/messages";
import { api, type OrgUnitDto, type EmployeeDto, extractErrorMessage } from "$lib/api";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Badge } from "@orbis/ui/badge";
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from "@orbis/ui/card";
import { Select, SelectTrigger, SelectContent, SelectItem } from "@orbis/ui/select";
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
import { OrbisGrid, type OrbisColumnDef, type ActionItem, type GridActionEvent } from "@orbis/svelte-grid";
import { l, isAr, formatDate } from "$lib/core/i18n";
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
let isolationFilter = $state<"all" | "shared" | "isolated" | "read_only">("all");

// Sheet State
let isBranchSheetOpen = $state(false);
let editingBranch = $state<OrgUnitDto | null>(null);

function formatEmployeeName(e?: EmployeeDto | null): string {
  if (!e) return l("غير محدد", "Unassigned");
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
    toast.success(l("تم تحديث بيانات الفروع", "Branches refreshed"));
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
const branches = $derived(
  allUnits.filter((u) => u.unit_type === "branch")
);

// Read default branch ID from settings
const defaultBranchId = $derived(() => {
  const s = currentOrg?.settings;
  const parsed = typeof s === "string" ? (() => { try { return JSON.parse(s); } catch { return {}; } })() : s || {};
  return parsed?.branch_policy?.default_branch_id || null;
});

// Derived KPI Computations (Matching FinanceOverview)
const totalBranchesCount = $derived(branches.length);
const activeBranchesCount = $derived(branches.filter((b) => b.is_active).length);
const totalCapacity = $derived(
  branches.reduce((sum, b) => sum + (b.capacity || 0), 0)
);
const branchPolicyIsolation = $derived(() => {
  const s = currentOrg?.settings;
  const parsed = typeof s === "string" ? (() => { try { return JSON.parse(s); } catch { return {}; } })() : s || {};
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
      const nameMatch = (b.name_ar || "").toLowerCase().includes(q) || (b.name || "").toLowerCase().includes(q);
      const codeMatch = (b.code || "").toLowerCase().includes(q);
      const manager = b.head_id ? employeeMap.get(b.head_id) : null;
      const managerMatch = manager ? formatEmployeeName(manager).toLowerCase().includes(q) : false;
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
  { id: "details", label: l("عرض التفاصيل", "Details"), variant: "default" },
  { id: "edit", label: l("تعديل", "Edit"), variant: "default" },
  { id: "delete", label: l("حذف", "Delete"), variant: "destructive" },
];

const gridSource = $derived(
  filteredBranches.map((b, i) => {
    const mgr = b.head_id ? employeeMap.get(b.head_id) : null;
    return {
      ...b,
      __idx: i + 1,
      displayName: b.name_ar || b.name,
      displayCode: b.code || "—",
      managerName: mgr ? formatEmployeeName(mgr) : l("غير محدد", "Unassigned"),
      displayCapacity: b.capacity ? b.capacity.toLocaleString() : l("غير محددة", "Uncapped"),
      subUnitsCount: b.children_count || 0,
      isolationLabel: b.data_isolation === "isolated" ? l("معزول", "Isolated") : b.data_isolation === "read_only" ? l("قراءة فقط", "Read-Only") : l("مشترك", "Shared"),
      statusLabel: b.is_active ? l("نشط", "Active") : l("معطل", "Inactive"),
      created_at_display: b.created_at ? formatDate(b.created_at, "datetime") : "—",
    };
  })
);

const columns: OrbisColumnDef[] = [
  { id: "__idx", prop: "__idx", header: "#", name: "#", width: 50, size: 50, readonly: true, pin: "left", align: "center" },
  { id: "displayName", prop: "displayName", header: l("اسم الفرع", "Branch Name"), name: "Branch Name", width: 220, size: 220, readonly: true, pin: "left", cellClass: "font-semibold text-foreground cursor-pointer hover:underline" },
  { id: "displayCode", prop: "displayCode", header: l("الكود", "Code"), name: "Code", width: 110, size: 110, readonly: true, cellClass: "font-mono text-xs uppercase" },
  { id: "managerName", prop: "managerName", header: l("مدير الفرع", "Manager"), name: "Manager", width: 180, size: 180, readonly: true },
  { id: "displayCapacity", prop: "displayCapacity", header: l("السعة الطلابية", "Capacity"), name: "Capacity", width: 120, size: 120, readonly: true, align: "center", type: "number" },
  { id: "subUnitsCount", prop: "subUnitsCount", header: l("الأقسام التابعة", "Sub-units"), name: "Sub-units", width: 110, size: 110, readonly: true, align: "center", type: "number" },
  { id: "isolationLabel", prop: "isolationLabel", header: l("العزل الأمني", "Isolation"), name: "Isolation", width: 120, size: 120, readonly: true, align: "center" },
  { id: "statusLabel", prop: "statusLabel", header: l("الحالة", "Status"), name: "Status", width: 100, size: 100, readonly: true, align: "center", type: "badge" },
  { id: "created_at_display", prop: "created_at_display", header: l("تاريخ الإنشاء", "Created At"), name: "Created At", width: 140, size: 140, readonly: true, cellClass: "font-mono text-xs text-muted-foreground" },
  { id: "actions", prop: "actions", header: l("إجراءات", "Actions"), name: "Actions", width: 140, size: 140, type: "action", align: "center", readonly: true, meta: { items: gridActions } },
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
      l(
        `لا يمكن حذف الفرع لوجود (${children.length}) وحدات وأقسام تابعة له. يرجى نقلها أو حذفها أولاً.`,
        `Cannot delete branch because it contains (${children.length}) child units.`
      )
    );
    return;
  }

  confirmStore.prompt({
    title: l("حذف الفرع التشغيلي", "Delete Branch"),
    description: l(
      `هل أنت متأكد من رغبتك في حذف فرع "${branch.name_ar || branch.name}" بشكل نهائي؟`,
      `Are you sure you want to permanently delete branch "${branch.name_ar || branch.name}"?`
    ),
    confirmText: l("تأكيد الحذف", "Confirm Delete"),
    variant: "destructive",
    onConfirm: async () => {
      try {
        await api.deleteOrgUnit(branch.id);
        toast.success(l("تم حذف الفرع بنجاح", "Branch deleted successfully"));
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
    toast.success(l(`تم تعيين "${branch.name_ar || branch.name}" كفرع افتراضي للمؤسسة`, `Set "${branch.name_ar || branch.name}" as default branch`));
    loadData();
  } catch (err) {
    toast.error(extractErrorMessage(err));
  }
}
</script>

<svelte:head>
  <title>{l("دليل الفروع والعمليات", "Branches Directory")} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto p-6 scrollbar-hide text-start font-sans flex flex-col gap-6">
  <!-- 1. Top KPI Summary Metric Cards (Matching FinanceOverview Style) -->
  <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
    <!-- Total Branches -->
    <Card class="border border-border/50 bg-card/60 shadow-xs">
      <CardContent class="p-4 flex items-center justify-between">
        <div class="space-y-1">
          <span class="text-xs font-medium text-muted-foreground">{l("إجمالي الفروع", "Total Branches")}</span>
          <div class="text-2xl font-bold tabular-nums text-foreground">{totalBranchesCount}</div>
          <span class="text-[11px] text-muted-foreground">{l("مراكز تشغيلية مسجلة", "Registered campuses")}</span>
        </div>
      </CardContent>
    </Card>

    <!-- Active Branches -->
    <Card class="border border-border/50 bg-card/60 shadow-xs">
      <CardContent class="p-4 flex items-center justify-between">
        <div class="space-y-1">
          <span class="text-xs font-medium text-muted-foreground">{l("الفروع النشطة", "Active Branches")}</span>
          <div class="text-2xl font-bold tabular-nums text-emerald-600 dark:text-emerald-400">{activeBranchesCount}</div>
          <span class="text-[11px] text-muted-foreground">
            {totalBranchesCount > 0 ? `${Math.round((activeBranchesCount / totalBranchesCount) * 100)}% ${l("جاهزية تشغيلية", "readiness")}` : l("لا توجد فروع", "No branches")}
          </span>
        </div>
      </CardContent>
    </Card>

    <!-- Total Student Capacity -->
    <Card class="border border-border/50 bg-card/60 shadow-xs">
      <CardContent class="p-4 flex items-center justify-between">
        <div class="space-y-1">
          <span class="text-xs font-medium text-muted-foreground">{l("السعة الطلابية الإجمالية", "Total Capacity")}</span>
          <div class="text-2xl font-bold tabular-nums text-foreground">{totalCapacity.toLocaleString()}</div>
          <span class="text-[11px] text-muted-foreground">{l("مقعد دراسي عبر الفروع", "Available seats")}</span>
        </div>
      </CardContent>
    </Card>

    <!-- Isolation Policy Scope -->
    <Card class="border border-border/50 bg-card/60 shadow-xs">
      <CardContent class="p-4 flex items-center justify-between">
        <div class="space-y-1">
          <span class="text-xs font-medium text-muted-foreground">{l("سياسة العزل المؤسسي", "Governance Scope")}</span>
          <div class="text-base font-bold text-foreground truncate mt-0.5">
            {branchPolicyIsolation() === "isolated" ? l("نطاق معزول", "Strict Isolated") : l("نطاق مشترك", "Shared Domain")}
          </div>
          <span class="text-[11px] text-muted-foreground">
            {branchPolicyIsolation() === "isolated" ? l("سجلات منفصلة للفرع", "Per-branch silos") : l("سجلات مشتركة للمؤسسة", "Multi-branch")}
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
          placeholder={l("البحث في الفروع...", "Search branches...")}
          class="h-9 text-xs pr-9 pl-3 bg-card border-border/60"
        />
      </div>

      <!-- Status Filter -->
      <Select type="single" value={statusFilter} onValueChange={(val: any) => (statusFilter = val || "all")}>
        <SelectTrigger class="h-9 text-xs w-36 bg-card border-border/60">
          {statusFilter === "active" ? l("نشط فقط", "Active Only") : statusFilter === "inactive" ? l("معطل فقط", "Inactive Only") : l("جميع الحالات", "All Statuses")}
        </SelectTrigger>
        <SelectContent class="text-xs">
          <SelectItem value="all">{l("جميع الحالات", "All Statuses")}</SelectItem>
          <SelectItem value="active">{l("نشط فقط", "Active Only")}</SelectItem>
          <SelectItem value="inactive">{l("معطل فقط", "Inactive Only")}</SelectItem>
        </SelectContent>
      </Select>

      <!-- Isolation Filter -->
      <Select type="single" value={isolationFilter} onValueChange={(val: any) => (isolationFilter = val || "all")}>
        <SelectTrigger class="h-9 text-xs w-36 bg-card border-border/60">
          {isolationFilter === "isolated" ? l("معزول", "Isolated") : isolationFilter === "read_only" ? l("قراءة فقط", "Read-Only") : isolationFilter === "shared" ? l("مشترك", "Shared") : l("كل نطاقات العزل", "All Isolations")}
        </SelectTrigger>
        <SelectContent class="text-xs">
          <SelectItem value="all">{l("كل نطاقات العزل", "All Isolations")}</SelectItem>
          <SelectItem value="shared">{l("مشترك", "Shared")}</SelectItem>
          <SelectItem value="isolated">{l("معزول", "Isolated")}</SelectItem>
          <SelectItem value="read_only">{l("قراءة فقط", "Read-Only")}</SelectItem>
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
        <Refresh size={13} class={isRefreshing ? "animate-spin" : ""} />
        <span>{l("تحديث", "Refresh")}</span>
      </Button>

      <Button
        variant="default"
        size="sm"
        onclick={openCreateBranch}
        class="h-9 px-4 text-xs font-semibold gap-1.5 shadow-sm cursor-pointer"
      >
        <Plus size={15} />
        <span>{l("إضافة فرع جديد", "Add Branch")}</span>
      </Button>
    </div>
  </div>

  <!-- 3. Main Content (OrbisGrid / Cards) -->
  {#if isLoading}
    <div class="h-[520px] w-full rounded-xl border border-border/50 bg-muted/10 flex items-center justify-center">
      <span class="text-xs text-muted-foreground">{l("جاري تحميل بيانات الفروع...", "Loading branches...")}</span>
    </div>
  {:else if filteredBranches.length === 0}
    <div class="flex flex-col items-center justify-center py-16 px-4 text-center rounded-2xl border border-dashed border-border/60 bg-card/30">
      <div class="size-16 rounded-2xl bg-muted/40 flex items-center justify-center text-muted-foreground mb-4">
        <BuildingOffice size={32} />
      </div>
      <h3 class="text-base font-bold text-foreground">
        {searchQuery ? l("لا توجد فروع مطابقة للبحث", "No matching branches found") : l("لم يتم إضافة فروع تشغيلية بعد", "No branches added yet")}
      </h3>
      <p class="text-xs text-muted-foreground max-w-sm mt-1 mb-5">
        {searchQuery ? l("جرب تغيير كلمات البحث أو إعادة تعيين الفلاتر المحددة", "Try adjusting your search query or reset active filters") : l("ابدأ بإضافة أول فرع أو مجمع تعليمي لإدارة المنظومة بشكل منظم", "Add your first campus to organize students and classes")}
      </p>
      {#if !searchQuery}
        <Button size="sm" onclick={openCreateBranch} class="gap-1.5 text-xs font-semibold">
          <Plus size={14} />
          <span>{l("إضافة أول فرع الآن", "Add First Branch")}</span>
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
