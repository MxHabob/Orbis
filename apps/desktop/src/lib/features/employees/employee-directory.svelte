<!--
  @component EmployeeDirectory
  Main workspace for Employee Affairs directory.
  Integrates status stats, dual-panel filters, high-performance OrbisGrid table view,
  search, CSV export, slide-over EmployeeSheet, and unified account lifecycle actions.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import MainViewport from "$lib/shell/main-viewport.svelte";
import SidebarNav from "$lib/shell/sidebar-nav.svelte";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Card } from "@orbis/ui/card";
import {
  ArrowPath,
  DocumentChartBar,
  Eye,
  Plus,
  Search,
  Users,
  XMark,
} from "@orbis/icons";
import { fly } from "svelte/transition";
import { quintOut } from "svelte/easing";
import { toast } from "@orbis/ui";
import { getLocale } from "$lib/paraglide/runtime";
import { formatNumber, formatDate, l } from "$lib/core/i18n";
import { extractErrorMessage } from "$lib/api";
import { debounce } from "$lib/shared/utils/debounce";
import { exportToCsv } from "$lib/shared/utils/export";
import type { EmployeeDetailDto } from "$lib/api/types";
import {
  createEmployeesQuery,
  createEmployeeStatsQuery,
  createUpdateEmployeeStatusMutation,
  createDeleteEmployeeMutation,
} from "./queries";
import EmployeeSheet from "./components/employee-sheet.svelte";
import { confirmStore } from "$lib/stores";
import {
  OrbisGrid,
  type OrbisColumnDef,
  type GridActionEvent,
  type ActionItem,
} from "@orbis/svelte-grid";

interface Props {
  orgId: string;
}

let { orgId }: Props = $props();

// Filter States
let searchQuery = $state("");
let statusFilter = $state<string>("all");
let typeFilter = $state<string>("all");
let selectedIds = $state<Set<string>>(new Set());

// Multi-mode Employee Sheet State
let sheetOpen = $state(false);
let sheetMode = $state<"preview" | "create" | "edit">("create");
let selectedEmployee = $state<EmployeeDetailDto | null>(null);

function openEmployeeSheet(emp: EmployeeDetailDto | null, mode: "preview" | "create" | "edit") {
  selectedEmployee = emp;
  sheetMode = mode;
  sheetOpen = true;
}

// Query filter payload
const filterInput = $derived({
  query: searchQuery.trim() || undefined,
  status: statusFilter !== "all" ? statusFilter : undefined,
  employee_type: typeFilter !== "all" ? typeFilter : undefined,
  limit: 200,
});

// Queries
const employeesQuery = createEmployeesQuery(() => orgId, () => filterInput);
const statsQuery = createEmployeeStatsQuery(() => orgId);

// Mutations
const statusMutation = createUpdateEmployeeStatusMutation();
const deleteMutation = createDeleteEmployeeMutation();

// Derived Data
const employees = $derived(employeesQuery.data?.items ?? []);
const totalCount = $derived(employeesQuery.data?.total_count ?? 0);
const stats = $derived(statsQuery.data ?? { total: 0, active: 0, on_leave: 0, suspended: 0, terminated: 0 });
const isLoading = $derived(employeesQuery.isLoading || statsQuery.isLoading);
const error = $derived(employeesQuery.error || statsQuery.error);

// Search with debounce
let localSearch = $state("");
const updateSearch = debounce((q: string) => {
  searchQuery = q;
}, 150);

function clearFilters() {
  searchQuery = "";
  localSearch = "";
  statusFilter = "all";
  typeFilter = "all";
}

const hasActiveFilters = $derived(
  searchQuery !== "" || statusFilter !== "all" || typeFilter !== "all"
);

function toggleSelect(id: string) {
  const next = new Set(selectedIds);
  if (next.has(id)) {
    next.delete(id);
  } else {
    next.add(id);
  }
  selectedIds = next;
}

// Status Change Handler via Global confirmStore
async function promptStatusChange(emp: EmployeeDetailDto, targetStatus: string) {
  const confirmed = await confirmStore.prompt({
    title: m.ui_copy_confirm_status_change_7af3a189(),
    description: m.operations_ui_employee_status_change_confirm({
      employee: emp.full_name || emp.first_name,
      status: getStatusLabel(targetStatus),
    }),
    confirmText: m.ui_copy_confirm_change_6cd511d4(),
    variant: targetStatus === "suspended" || targetStatus === "terminated" ? "destructive" : "default",
  });

  if (!confirmed) return;

  try {
    await statusMutation.mutateAsync({ id: emp.id, status: targetStatus });
    toast.success(m.employees_status_updated());
    if (selectedEmployee && selectedEmployee.id === emp.id) {
      selectedEmployee = { ...selectedEmployee, status: targetStatus };
    }
  } catch (err) {
    toast.error(extractErrorMessage(err));
  }
}

// Soft Delete Handler via Global confirmStore
async function promptDelete(emp: EmployeeDetailDto) {
  const confirmed = await confirmStore.prompt({
    title: m.ui_copy_delete_employee_record_0b2d4552(),
    description: m.operations_ui_employee_delete_confirm({
      employee: emp.full_name || emp.first_name,
    }),
    confirmText: m.ui_copy_delete_681d59c5(),
    variant: "destructive",
  });

  if (!confirmed) return;

  try {
    await deleteMutation.mutateAsync(emp.id);
    toast.success(m.ui_copy_employee_deleted_successfully_d6f925d3());
    selectedIds.delete(emp.id);
    if (selectedEmployee && selectedEmployee.id === emp.id) {
      sheetOpen = false;
      selectedEmployee = null;
    }
  } catch (err) {
    toast.error(extractErrorMessage(err));
  }
}


// Export CSV Handler
function handleExport() {
  const dataset = selectedIds.size > 0
    ? employees.filter((e) => selectedIds.has(e.id))
    : employees;

  exportToCsv(
    `Employees_Roster_${orgId}`,
    dataset,
    [
      { header: m.operations_ui_employee_number(), accessor: (e) => e.employee_number || "—" },
      { header: m.operations_ui_full_name(), accessor: (e) => e.full_name || e.full_name_ar || `${e.first_name} ${e.last_name}` },
      { header: m.operations_ui_national_id(), accessor: (e) => e.national_id || "—" },
      { header: m.operations_ui_job_title(), accessor: (e) => e.job_title || "—" },
      { header: m.operations_ui_department(), accessor: (e) => e.department || "—" },
      { header: m.operations_ui_employee_category(), accessor: (e) => e.employee_type || "—" },
      { header: "الحالة", accessor: (e) => e.status || "active" },
      { header: m.operations_ui_user_account(), accessor: (e) => e.username || m.operations_ui_not_linked() },
      { header: m.operations_ui_hire_date(), accessor: (e) => e.hire_date ? formatDate(e.hire_date, "date") : "—" },
      { header: m.operations_ui_phone_number(), accessor: (e) => e.phone || "—" },
      { header: m.operations_ui_email(), accessor: (e) => e.email || "—" },
    ]
  );
  toast.success(m.operations_ui_employee_export_success());
}

function getStatusLabel(s: string): string {
  switch (s.toLowerCase()) {
    case "active":
      return m.employees_status_active();
    case "on_leave":
      return m.employees_status_on_leave();
    case "suspended":
      return m.employees_status_suspended();
    case "terminated":
      return m.employees_status_terminated();
    default:
      return s;
  }
}

function getTypeLabel(t: string): string {
  switch (t.toLowerCase()) {
    case "teacher":
      return m.employees_type_teacher();
    case "admin":
    case "administrative":
      return m.employees_type_admin();
    case "support":
      return m.employees_type_support();
    case "specialist":
      return m.employees_type_specialist();
    default:
      return t;
  }
}

// ── OrbisGrid Actions & Columns (Icon-Only Pure Icons) ─────────────────
const employeeActions: ActionItem[] = [
  { id: "preview", label: m.ui_copy_view_dossier_ca7e9625(), icon: "Eye" },
  { id: "edit", label: m.ui_copy_edit_details_bb482293(), icon: "PencilSquare" },
  { id: "set_active", label: m.ui_copy_set_active_7c3b569e(), icon: "CheckCircle" },
  { id: "set_on_leave", label: m.ui_copy_set_on_leave_4946e4f1(), icon: "BuildingOffice" },
  { id: "set_suspended", label: m.ui_copy_suspend_50430438(), icon: "ShieldCheck", variant: "destructive" },
  { id: "delete", label: m.ui_copy_delete_employee_ef02f4a9(), icon: "Trash", variant: "destructive" },
];

const gridSource = $derived(
  employees.map((emp, i) => {
    const fullName = emp.full_name || emp.full_name_ar || `${emp.first_name} ${emp.last_name}`;
    const isSelected = selectedIds.has(emp.id);
    return {
      ...emp,
      __idx: i + 1,
      __selected: isSelected,
      full_name: fullName,
      job_dept: emp.department || emp.org_unit_name ? `${emp.job_title || ''} • ${emp.department || emp.org_unit_name || ''}` : (emp.job_title || "—"),
      type_display: getTypeLabel(emp.employee_type),
      status_display: getStatusLabel(emp.status),
      account_display: emp.username || m.ui_copy_not_linked_798eb72b(),
      hire_date_display: emp.hire_date ? formatDate(emp.hire_date, "date") : "—",
      created_at_display: emp.created_at ? formatDate(emp.created_at, "datetime") : "—",
      updated_at_display: emp.updated_at ? formatDate(emp.updated_at, "datetime") : "—",
    };
  })
);

const columns = $derived.by<OrbisColumnDef[]>(() => [
  {
    id: "__selected",
    prop: "__selected",
    header: "",
    name: "",
    width: 44,
    size: 44,
    type: "boolean",
    align: "center",
    sort: false,
    pin: "left",
  },
  {
    id: "__idx",
    prop: "__idx",
    header: "#",
    name: "#",
    width: 48,
    size: 48,
    readonly: true,
    align: "center",
    pin: "left",
  },
  {
    id: "full_name",
    prop: "full_name",
    header: m.ui_copy_employee_f6c1c741(),
    name: "Employee",
    width: 230,
    size: 230,
    type: "avatar",
    readonly: true,
    pin: "left",
  },
  {
    id: "employee_number",
    prop: "employee_number",
    header: m.employees_employee_number(),
    name: "Employee No",
    width: 120,
    size: 120,
    readonly: true,
    cellClass: "font-mono font-medium",
  },
  {
    id: "job_dept",
    prop: "job_dept",
    header: m.employees_job_title(),
    name: "Job / Department",
    width: 200,
    size: 200,
    readonly: true,
  },
  {
    id: "type_display",
    prop: "type_display",
    header: m.employees_filter_type(),
    name: "Category",
    width: 110,
    size: 110,
    readonly: true,
    align: "center",
    type: "badge",
  },
  {
    id: "status",
    prop: "status",
    header: m.employees_filter_status(),
    name: "Status",
    width: 110,
    size: 110,
    type: "badge",
    align: "center",
    readonly: true,
  },
  {
    id: "account_display",
    prop: "account_display",
    header: m.ui_copy_user_account_1f9dba18(),
    name: "Account",
    width: 140,
    size: 140,
    readonly: true,
    cellClass: "font-mono text-xs",
  },
  {
    id: "hire_date_display",
    prop: "hire_date_display",
    header: m.employees_hire_date(),
    name: "Hire Date",
    width: 120,
    size: 120,
    readonly: true,
    cellClass: "font-mono text-xs text-muted-foreground",
  },
  {
    id: "created_at_display",
    prop: "created_at_display",
    header: m.ui_copy_created_at_01f6ea5f(),
    name: "Created At",
    width: 130,
    size: 130,
    readonly: true,
    cellClass: "font-mono text-xs text-muted-foreground",
  },
  {
    id: "updated_at_display",
    prop: "updated_at_display",
    header: m.ui_copy_updated_at_ba0d0a9f(),
    name: "Updated At",
    width: 130,
    size: 130,
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
    meta: { items: employeeActions },
  },
]);

function handleGridAction(event: GridActionEvent) {
  const emp = event.row as unknown as EmployeeDetailDto;
  if (!emp) return;

  if (event.actionId === "toggle-select" || event.actionId === "select") {
    toggleSelect(emp.id);
  } else if (event.actionId === "preview" || event.actionId === "dblclick" || event.actionId === "profile") {
    openEmployeeSheet(emp, "preview");
  } else if (event.actionId === "edit") {
    openEmployeeSheet(emp, "edit");
  } else if (event.actionId === "set_active") {
    promptStatusChange(emp, "active");
  } else if (event.actionId === "set_on_leave") {
    promptStatusChange(emp, "on_leave");
  } else if (event.actionId === "set_suspended") {
    promptStatusChange(emp, "suspended");
  } else if (event.actionId === "delete") {
    promptDelete(emp);
  }
}

</script>

<div class="h-full w-full flex flex-col overflow-hidden bg-background">
  <MainViewport
    name="employees-workspace"
    leftWidth={{ default: 240, min: 200 }}
  >
    <!-- Left Panel: Filter Navigation -->
    {#snippet left()}
      <SidebarNav
        title={m.employees_nav_title()}
        groups={[
          {
            title: m.employees_filter_status(),
            items: [
              {
                label: m.employees_filter_all(),
                badge: formatNumber(stats.total),
                isActive: statusFilter === "all",
                onclick: () => (statusFilter = "all"),
              },
              {
                label: m.employees_status_active(),
                badge: formatNumber(stats.active),
                isActive: statusFilter === "active",
                onclick: () => (statusFilter = "active"),
              },
              {
                label: m.employees_status_on_leave(),
                badge: formatNumber(stats.on_leave),
                isActive: statusFilter === "on_leave",
                onclick: () => (statusFilter = "on_leave"),
              },
              {
                label: m.employees_status_suspended(),
                badge: formatNumber(stats.suspended),
                isActive: statusFilter === "suspended",
                onclick: () => (statusFilter = "suspended"),
              },
              {
                label: m.employees_status_terminated(),
                badge: formatNumber(stats.terminated),
                isActive: statusFilter === "terminated",
                onclick: () => (statusFilter = "terminated"),
              },
            ],
          },
          {
            title: m.employees_filter_type(),
            items: [
              {
                label: m.ui_copy_all_types_f376a2ae(),
                badge: formatNumber(stats.total),
                isActive: typeFilter === "all",
                onclick: () => (typeFilter = "all"),
              },
              {
                label: m.employees_type_teacher(),
                isActive: typeFilter === "teacher",
                onclick: () => (typeFilter = "teacher"),
              },
              {
                label: m.employees_type_admin(),
                isActive: typeFilter === "admin",
                onclick: () => (typeFilter = "admin"),
              },
              {
                label: m.employees_type_specialist(),
                isActive: typeFilter === "specialist",
                onclick: () => (typeFilter = "specialist"),
              },
              {
                label: m.employees_type_support(),
                isActive: typeFilter === "support",
                onclick: () => (typeFilter = "support"),
              },
            ],
          },
        ]}
      >
        {#snippet headerAction()}
          {#if hasActiveFilters}
            <Button
              variant="ghost"
              size="icon"
              class="size-6 text-muted-foreground hover:text-foreground cursor-pointer"
              onclick={clearFilters}
              title={m.common_filter_clear()}
            >
              <XMark size={14} />
            </Button>
          {/if}
        {/snippet}
      </SidebarNav>
    {/snippet}

    <!-- Middle Panel: Main Datagrid -->
    {#snippet middle()}
      <div class="h-full flex flex-col p-6 overflow-hidden gap-4">
        <!-- Top Stats Row using canonical Card primitive -->
        <div class="grid grid-cols-2 sm:grid-cols-4 gap-3 shrink-0 select-none">
          <Card class="flex items-center justify-between p-3.5 shadow-2xs">
            <div class="flex flex-col min-w-0 text-start">
              <span class="text-[11px] font-medium text-muted-foreground truncate">{m.employees_total_count()}</span>
              <span class="font-mono text-lg font-bold text-foreground mt-0.5">{formatNumber(stats.total)}</span>
            </div>
          </Card>

          <Card class="flex items-center justify-between p-3.5 shadow-2xs">
            <div class="flex flex-col min-w-0 text-start">
              <span class="text-[11px] font-medium text-muted-foreground truncate">{m.employees_active_count()}</span>
              <span class="font-mono text-lg font-bold text-emerald-600 dark:text-emerald-400 mt-0.5">{formatNumber(stats.active)}</span>
            </div>
          </Card>

          <Card class="flex items-center justify-between p-3.5 shadow-2xs">
            <div class="flex flex-col min-w-0 text-start">
              <span class="text-[11px] font-medium text-muted-foreground truncate">{m.employees_on_leave_count()}</span>
              <span class="font-mono text-lg font-bold text-amber-600 dark:text-amber-400 mt-0.5">{formatNumber(stats.on_leave)}</span>
            </div>
          </Card>

          <Card class="flex items-center justify-between p-3.5 shadow-2xs">
            <div class="flex flex-col min-w-0 text-start">
              <span class="text-[11px] font-medium text-muted-foreground truncate">{m.employees_suspended_count()}</span>
              <span class="font-mono text-lg font-bold text-rose-600 dark:text-rose-400 mt-0.5">{formatNumber(stats.suspended + stats.terminated)}</span>
            </div>
          </Card>
        </div>

        <!-- Controls Bar: Search & Action Buttons -->
        <div class="flex items-center justify-between shrink-0 gap-3">
          <div class="relative w-full max-w-sm shrink-0">
            <Search class="absolute left-3 top-1/2 h-3.5 w-3.5 -translate-y-1/2 text-muted-foreground" />
            <Input
              type="text"
              placeholder={m.employees_search_placeholder()}
              class="input-sunken ps-9 text-xs py-2"
              bind:value={localSearch}
              oninput={() => updateSearch(localSearch)}
            />
          </div>

          <div class="flex items-center gap-2">
            <Button
              variant="outline"
              class="text-xs h-9 px-3 cursor-pointer"
              onclick={handleExport}
            >
              <DocumentChartBar size={14} class="me-1.5" />
              {m.common_export()}
            </Button>

            <Button
              variant="default"
              class="text-xs h-9 px-3 cursor-pointer"
              onclick={() => openEmployeeSheet(null, "create")}
            >
              <Plus size={14} class="me-1.5" />
              {m.employees_new_title()}
            </Button>
          </div>
        </div>

        <!-- Table View with OrbisGrid -->
        <div class="flex-1 min-h-0 flex flex-col bg-card border border-border rounded-lg overflow-hidden shadow-xs">
          {#if isLoading}
            <div class="h-full w-full flex flex-col items-center justify-center p-8 gap-3 text-muted-foreground">
              <ArrowPath size={24} class="animate-spin text-primary" />
              <span class="text-xs">{m.ui_copy_loading_employees_3e3b45d1()}</span>
            </div>
          {:else if error}
            <div class="h-full w-full flex flex-col items-center justify-center p-6 text-center gap-3">
              <p class="text-xs text-destructive font-semibold">{extractErrorMessage(error)}</p>
              <Button variant="outline" size="sm" class="text-xs" onclick={() => employeesQuery.refetch()}>
                {m.common_retry()}
              </Button>
            </div>
          {:else if employees.length === 0}
            <div class="h-full w-full flex flex-col items-center justify-center p-8 text-center gap-3">
              <div class="flex size-12 items-center justify-center rounded-full bg-muted text-muted-foreground">
                <Users size={24} />
              </div>
              <div class="flex flex-col gap-1">
                <h3 class="text-sm font-semibold text-foreground">{m.ui_copy_no_employees_found_967f15bd()}</h3>
                <p class="text-xs text-muted-foreground max-w-xs">{m.ui_copy_try_adjusting_your_search_or_filters_2c9b3cb2()}</p>
              </div>
              {#if hasActiveFilters}
                <Button variant="outline" size="sm" class="text-xs mt-2" onclick={clearFilters}>
                  {m.common_filter_clear()}
                </Button>
              {/if}
            </div>
          {:else}
            <!-- OrbisGrid -->
            <div class="flex-1 min-h-0 flex flex-col">
              <OrbisGrid
                {columns}
                source={gridSource}
                height="100%"
                rowSize={44}
                rtl={getLocale() === "ar"}
                readonly
                onaction={handleGridAction}
              />
            </div>
          {/if}
        </div>
      </div>
    {/snippet}
  </MainViewport>

  <!-- Floating Bottom Action Bar for Selected Rows -->
  {#if selectedIds.size > 0}
    <div
      class="fixed bottom-6 left-1/2 -translate-x-1/2 z-50 w-[calc(100%-2rem)] max-w-md pointer-events-auto select-none"
      transition:fly={{ y: 35, duration: 250, easing: quintOut }}
    >
      <div class="bg-card/95 backdrop-blur-md border border-border shadow-2xl rounded-xl px-4 py-2.5 flex items-center justify-between gap-3 ring-1 ring-black/5 dark:ring-white/10">
        <div class="flex min-w-0 items-center gap-2.5 text-start">
          <span class="flex size-7 shrink-0 items-center justify-center rounded-full bg-primary/15 text-primary">
            <Users size={15} strokeWidth={2.5} />
          </span>
          <span class="truncate text-xs font-bold text-foreground">
            {formatNumber(selectedIds.size)} {m.ui_copy_selected_d87673e9()}
          </span>
        </div>

        <div class="flex shrink-0 items-center gap-2">
          {#if selectedIds.size === 1}
            {@const single = employees.find((e) => selectedIds.has(e.id))}
            {#if single}
              <Button
                type="button"
                variant="outline"
                size="sm"
                class="h-8 px-2.5 text-xs font-medium cursor-pointer gap-1.5"
                onclick={() => openEmployeeSheet(single, "preview")}
              >
                <Eye size={13} />
              </Button>
            {/if}
          {/if}

          <Button
            type="button"
            variant="default"
            size="sm"
            class="h-8 px-3 text-xs font-medium cursor-pointer gap-1.5"
            onclick={handleExport}
          >
            <DocumentChartBar size={13} />
            <span>{m.common_export()}</span>
          </Button>
        </div>
      </div>
    </div>
  {/if}

  <!-- Multi-mode Employee Sheet (Preview / Create / Edit) -->
  <EmployeeSheet
    bind:open={sheetOpen}
    {orgId}
    bind:mode={sheetMode}
    employee={selectedEmployee}
    onSuccess={() => {
      employeesQuery.refetch();
      statsQuery.refetch();
    }}
  />
</div>
