<!--
  @component EmployeeDirectory
  Main workspace for Employee Affairs directory.
  Integrates status stats, dual-panel filters, table view, search, export, and account lifecycle actions.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import MainViewport from "$lib/shell/main-viewport.svelte";
import SidebarNav from "$lib/shell/sidebar-nav.svelte";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Badge } from "@orbis/ui/badge";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@orbis/ui/dropdown-menu";
import {
  ArrowPath,
  BookOpen,
  BuildingOffice,
  CheckCircle,
  DocumentChartBar,
  EllipsisHorizontal,
  Eye,
  Keyframes,
  LockClosed,
  Pencil,
  Plus,
  Search,
  ShieldCheck,
  Trash,
  User,
  UserBadgeCheck,
  UserGroup,
  Users,
  XMark,
} from "@orbis/icons";
import { fly } from "svelte/transition";
import { quintOut } from "svelte/easing";
import { goto } from "$app/navigation";
import { toast } from "@orbis/ui";
import { localizeHref } from "$lib/paraglide/runtime";
import { formatNumber, formatDate, l } from "$lib/core/i18n";
import { extractErrorMessage } from "$lib/api";
import { debounce } from "$lib/shared/utils/debounce";
import { exportToCsv } from "$lib/shared/utils/export";
import type { EmployeeDetailDto, AccountProvisioningResultDto } from "$lib/api/types";
import {
  createEmployeesQuery,
  createEmployeeStatsQuery,
  createUpdateEmployeeStatusMutation,
  createResetEmployeePasswordMutation,
} from "./queries";
import CredentialDeliveryModal from "./components/credential-delivery-modal.svelte";
import UserAvatar from "$lib/shared/components/user-avatar.svelte";
import ConfirmDialog from "$lib/shared/components/confirm-dialog.svelte";

interface Props {
  orgId: string;
}

let { orgId }: Props = $props();

// Filter States
let searchQuery = $state("");
let statusFilter = $state<string>("all");
let typeFilter = $state<string>("all");
let selectedIds = $state<Set<string>>(new Set());

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
const resetPasswordMutation = createResetEmployeePasswordMutation();

// Derived Data
const employees = $derived(employeesQuery.data?.items ?? []);
const totalCount = $derived(employeesQuery.data?.total_count ?? 0);
const stats = $derived(statsQuery.data ?? { total: 0, active: 0, on_leave: 0, suspended: 0, terminated: 0 });
const isLoading = $derived(employeesQuery.isLoading || statsQuery.isLoading);
const error = $derived(employeesQuery.error || statsQuery.error);

// Credential modal state
let credentialModalOpen = $state(false);
let activeProvisioningResult = $state<AccountProvisioningResultDto | null>(null);
let activeEmployeeName = $state("");

// Status change dialog state
let statusDialogOpen = $state(false);
let targetEmployee = $state<EmployeeDetailDto | null>(null);
let nextStatus = $state<string>("");

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

function handleSelectAll(checked: boolean) {
  if (checked) {
    selectedIds = new Set(employees.map((e) => e.id));
  } else {
    selectedIds = new Set();
  }
}

function toggleSelect(id: string) {
  const next = new Set(selectedIds);
  if (next.has(id)) {
    next.delete(id);
  } else {
    next.add(id);
  }
  selectedIds = next;
}

// Reset Password Handler
async function handleResetPassword(employee: EmployeeDetailDto) {
  try {
    const result = await resetPasswordMutation.mutateAsync(employee.id);
    activeProvisioningResult = result;
    activeEmployeeName = employee.full_name || employee.full_name_ar || employee.first_name;
    credentialModalOpen = true;
    toast.success(m.employees_password_reset_success?.() || "تمت إعادة ضبط كلمة المرور بنجاح");
  } catch (err) {
    toast.error(extractErrorMessage(err));
  }
}

// Status Change Handler
function promptStatusChange(employee: EmployeeDetailDto, status: string) {
  targetEmployee = employee;
  nextStatus = status;
  statusDialogOpen = true;
}

async function confirmStatusChange() {
  if (!targetEmployee || !nextStatus) return;
  try {
    await statusMutation.mutateAsync({ id: targetEmployee.id, status: nextStatus });
    toast.success(m.employees_status_updated?.() || "تم تحديث حالة الموظف بنجاح");
  } catch (err) {
    toast.error(extractErrorMessage(err));
  } finally {
    statusDialogOpen = false;
    targetEmployee = null;
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
      { header: "الرقم الوظيفي", accessor: (e) => e.employee_number || "—" },
      { header: "الاسم الكامل", accessor: (e) => e.full_name || e.full_name_ar || `${e.first_name} ${e.last_name}` },
      { header: "الهوية الوطنية", accessor: (e) => e.national_id || "—" },
      { header: "المسمى الوظيفي", accessor: (e) => e.job_title || "—" },
      { header: "القسم", accessor: (e) => e.department || "—" },
      { header: "فئة الموظف", accessor: (e) => e.employee_type || "—" },
      { header: "الحالة", accessor: (e) => e.status || "active" },
      { header: "حساب المستخدم", accessor: (e) => e.username || "غير مربوط" },
      { header: "تاريخ التعيين", accessor: (e) => e.hire_date ? formatDate(e.hire_date, "date") : "—" },
      { header: "رقم الهاتف", accessor: (e) => e.phone || "—" },
      { header: "البريد الإلكتروني", accessor: (e) => e.email || "—" },
    ]
  );
  toast.success("تم تصدير قائمة الموظفين بنجاح (CSV)");
}

function getStatusBadgeVariant(status: string): "default" | "outline" | "secondary" | "destructive" {
  switch (status.toLowerCase()) {
    case "active":
      return "default";
    case "on_leave":
      return "secondary";
    case "suspended":
    case "terminated":
      return "destructive";
    default:
      return "outline";
  }
}

function getStatusLabel(status: string): string {
  switch (status.toLowerCase()) {
    case "active":
      return m.employees_status_active?.() || "نشط";
    case "on_leave":
      return m.employees_status_on_leave?.() || "إجازة";
    case "suspended":
      return m.employees_status_suspended?.() || "موقوف";
    case "terminated":
      return m.employees_status_terminated?.() || "منتهي الخدمة";
    default:
      return status;
  }
}

function getTypeLabel(type: string): string {
  switch (type.toLowerCase()) {
    case "teacher":
      return m.employees_type_teacher?.() || "معلم";
    case "admin":
    case "administrative":
      return m.employees_type_admin?.() || "إداري";
    case "support":
      return m.employees_type_support?.() || "خدمات مساندة";
    case "specialist":
      return m.employees_type_specialist?.() || "أخصائي";
    default:
      return type;
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
        title={m.employees_nav_title?.() || "شؤون الموظفين"}
        groups={[
          {
            title: m.employees_filter_status?.() || "الحالة الوظيفية",
            items: [
              {
                label: m.employees_filter_all?.() || "كافة الموظفين",
                badge: formatNumber(stats.total),
                isActive: statusFilter === "all",
                onclick: () => (statusFilter = "all"),
              },
              {
                label: m.employees_status_active?.() || "على رأس العمل",
                badge: formatNumber(stats.active),
                isActive: statusFilter === "active",
                onclick: () => (statusFilter = "active"),
              },
              {
                label: m.employees_status_on_leave?.() || "في إجازة",
                badge: formatNumber(stats.on_leave),
                isActive: statusFilter === "on_leave",
                onclick: () => (statusFilter = "on_leave"),
              },
              {
                label: m.employees_status_suspended?.() || "موقوف",
                badge: formatNumber(stats.suspended),
                isActive: statusFilter === "suspended",
                onclick: () => (statusFilter = "suspended"),
              },
              {
                label: m.employees_status_terminated?.() || "منتهي الخدمة",
                badge: formatNumber(stats.terminated),
                isActive: statusFilter === "terminated",
                onclick: () => (statusFilter = "terminated"),
              },
            ],
          },
          {
            title: m.employees_filter_type?.() || "فئة الموظف",
            items: [
              {
                label: l("الكل", "All Types"),
                badge: formatNumber(stats.total),
                isActive: typeFilter === "all",
                onclick: () => (typeFilter = "all"),
              },
              {
                label: m.employees_type_teacher?.() || "كادر تعليمي (معلم)",
                isActive: typeFilter === "teacher",
                onclick: () => (typeFilter = "teacher"),
              },
              {
                label: m.employees_type_admin?.() || "كادر إداري",
                isActive: typeFilter === "admin",
                onclick: () => (typeFilter = "admin"),
              },
              {
                label: m.employees_type_specialist?.() || "أخصائيين",
                isActive: typeFilter === "specialist",
                onclick: () => (typeFilter = "specialist"),
              },
              {
                label: m.employees_type_support?.() || "خدمات مساندة",
                isActive: typeFilter === "support",
                onclick: () => (typeFilter = "support"),
              },
            ],
          },
        ]}
        actions={[
          {
            label: m.common_filter_clear?.() || "إعادة تعيين التصفية",
            icon: XMark,
            onclick: clearFilters,
            disabled: !hasActiveFilters,
          },
        ]}
      />
    {/snippet}

    <!-- Middle Panel: Main Datagrid -->
    {#snippet middle()}
      <div class="h-full flex flex-col p-6 overflow-hidden gap-4">
        <!-- Top Stats Row -->
        <div class="grid grid-cols-4 gap-3 shrink-0">
          <div class="flex items-center gap-3 rounded-lg border border-border/70 bg-card p-3 shadow-xs">
            <div class="flex size-9 items-center justify-center rounded-lg bg-primary/10 text-primary">
              <Users size={18} />
            </div>
            <div class="flex flex-col min-w-0">
              <span class="text-[11px] font-medium text-muted-foreground truncate">{m.employees_total_count?.() || "إجمالي الموظفين"}</span>
              <span class="font-mono text-lg font-bold text-foreground">{formatNumber(stats.total)}</span>
            </div>
          </div>
          <div class="flex items-center gap-3 rounded-lg border border-border/70 bg-card p-3 shadow-xs">
            <div class="flex size-9 items-center justify-center rounded-lg bg-emerald-500/10 text-emerald-600 dark:text-emerald-400">
              <CheckCircle size={18} />
            </div>
            <div class="flex flex-col min-w-0">
              <span class="text-[11px] font-medium text-muted-foreground truncate">{m.employees_active_count?.() || "على رأس العمل"}</span>
              <span class="font-mono text-lg font-bold text-emerald-600 dark:text-emerald-400">{formatNumber(stats.active)}</span>
            </div>
          </div>
          <div class="flex items-center gap-3 rounded-lg border border-border/70 bg-card p-3 shadow-xs">
            <div class="flex size-9 items-center justify-center rounded-lg bg-amber-500/10 text-amber-600 dark:text-amber-400">
              <BuildingOffice size={18} />
            </div>
            <div class="flex flex-col min-w-0">
              <span class="text-[11px] font-medium text-muted-foreground truncate">{m.employees_on_leave_count?.() || "في إجازة"}</span>
              <span class="font-mono text-lg font-bold text-amber-600 dark:text-amber-400">{formatNumber(stats.on_leave)}</span>
            </div>
          </div>
          <div class="flex items-center gap-3 rounded-lg border border-border/70 bg-card p-3 shadow-xs">
            <div class="flex size-9 items-center justify-center rounded-lg bg-rose-500/10 text-rose-600 dark:text-rose-400">
              <ShieldCheck size={18} />
            </div>
            <div class="flex flex-col min-w-0">
              <span class="text-[11px] font-medium text-muted-foreground truncate">{m.employees_suspended_count?.() || "موقوف / منتهي"}</span>
              <span class="font-mono text-lg font-bold text-rose-600 dark:text-rose-400">{formatNumber(stats.suspended + stats.terminated)}</span>
            </div>
          </div>
        </div>

        <!-- Controls Bar: Search & Action Buttons -->
        <div class="flex items-center justify-between shrink-0 gap-3">
          <div class="relative w-full max-w-sm shrink-0">
            <Search class="absolute left-3 top-1/2 h-3.5 w-3.5 -translate-y-1/2 text-muted-foreground" />
            <Input
              type="text"
              placeholder={m.employees_search_placeholder?.() || "بحث بالاسم، الرقم الوظيفي، الهوية..."}
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
              {m.common_export?.() || "تصدير"}
            </Button>

            <Button
              variant="default"
              class="text-xs h-9 px-3 cursor-pointer"
              onclick={() => goto(localizeHref(`/${orgId}/employees/new`))}
            >
              <Plus size={14} class="me-1.5" />
              {m.employees_new_title?.() || "تسجيل موظف جديد"}
            </Button>
          </div>
        </div>

        <!-- Table View -->
        <div class="flex-1 min-h-0 flex flex-col bg-card border border-border rounded-lg overflow-hidden shadow-xs">
          {#if isLoading}
            <div class="h-full w-full flex flex-col items-center justify-center p-8 gap-3 text-muted-foreground">
              <ArrowPath size={24} class="animate-spin text-primary" />
              <span class="text-xs">{l("جاري تحميل سجلات الموظفين...", "Loading employees...")}</span>
            </div>
          {:else if error}
            <div class="h-full w-full flex flex-col items-center justify-center p-6 text-center gap-3">
              <p class="text-xs text-destructive font-semibold">{extractErrorMessage(error)}</p>
              <Button variant="outline" size="sm" class="text-xs" onclick={() => employeesQuery.refetch()}>
                {m.common_retry?.() || "إعادة المحاولة"}
              </Button>
            </div>
          {:else if employees.length === 0}
            <div class="h-full w-full flex flex-col items-center justify-center p-8 text-center gap-3">
              <div class="flex size-12 items-center justify-center rounded-full bg-muted text-muted-foreground">
                <Users size={24} />
              </div>
              <div class="flex flex-col gap-1">
                <h3 class="text-sm font-semibold text-foreground">{l("لا يوجد موظفين مطابقين", "No employees found")}</h3>
                <p class="text-xs text-muted-foreground max-w-xs">{l("لم يتم العثور على موظفين وفق معايير التصفية المحددة.", "Try adjusting your search or filters.")}</p>
              </div>
              {#if hasActiveFilters}
                <Button variant="outline" size="sm" class="text-xs mt-2" onclick={clearFilters}>
                  {m.common_filter_clear?.() || "مسح التصفية"}
                </Button>
              {/if}
            </div>
          {:else}
            <!-- Data Table Container -->
            <div class="flex-1 overflow-auto">
              <table class="w-full text-start text-xs border-collapse">
                <thead class="sticky top-0 bg-muted/60 backdrop-blur-xs border-b border-border z-10 select-none">
                  <tr>
                    <th class="p-3 w-10 text-center">
                      <input
                        type="checkbox"
                        class="rounded border-border"
                        checked={selectedIds.size === employees.length && employees.length > 0}
                        onchange={(e) => handleSelectAll(e.currentTarget.checked)}
                      />
                    </th>
                    <th class="p-3 text-start font-semibold text-muted-foreground">{l("الموظف", "Employee")}</th>
                    <th class="p-3 text-start font-semibold text-muted-foreground">{m.employees_employee_number?.() || "الرقم الوظيفي"}</th>
                    <th class="p-3 text-start font-semibold text-muted-foreground">{m.employees_job_title?.() || "المسمى والوظيفة"}</th>
                    <th class="p-3 text-start font-semibold text-muted-foreground">{m.employees_filter_type?.() || "الفئة"}</th>
                    <th class="p-3 text-start font-semibold text-muted-foreground">{m.employees_filter_status?.() || "الحالة"}</th>
                    <th class="p-3 text-start font-semibold text-muted-foreground">{l("حساب النظام", "User Account")}</th>
                    <th class="p-3 text-start font-semibold text-muted-foreground">{m.employees_hire_date?.() || "تاريخ التعيين"}</th>
                    <th class="p-3 w-12 text-center font-semibold text-muted-foreground">{l("إجراءات", "Actions")}</th>
                  </tr>
                </thead>
                <tbody class="divide-y divide-border/40">
                  {#each employees as employee (employee.id)}
                    {@const isSelected = selectedIds.has(employee.id)}
                    <tr class="hover:bg-muted/30 transition-colors {isSelected ? 'bg-primary/5' : ''}">
                      <!-- Checkbox -->
                      <td class="p-3 text-center">
                        <input
                          type="checkbox"
                          class="rounded border-border cursor-pointer"
                          checked={isSelected}
                          onchange={() => toggleSelect(employee.id)}
                        />
                      </td>

                      <!-- Name & Avatar -->
                      <td class="p-3">
                        <a
                          href={localizeHref(`/${orgId}/employees/${employee.id}`)}
                          class="flex items-center gap-2.5 hover:underline font-medium text-foreground"
                        >
                          <UserAvatar
                            name={employee.full_name || employee.full_name_ar || employee.first_name}
                            avatarUrl={employee.avatar_url ?? undefined}
                            size="sm"
                          />
                          <div class="flex flex-col min-w-0">
                            <span class="font-semibold text-foreground truncate">
                              {employee.full_name || employee.full_name_ar || `${employee.first_name} ${employee.last_name}`}
                            </span>
                            {#if employee.national_id}
                              <span class="text-[10px] font-mono text-muted-foreground">
                                {employee.national_id}
                              </span>
                            {/if}
                          </div>
                        </a>
                      </td>

                      <!-- Employee Number -->
                      <td class="p-3 font-mono font-medium text-foreground">
                        {employee.employee_number || "—"}
                      </td>

                      <!-- Job Title & Department -->
                      <td class="p-3">
                        <div class="flex flex-col min-w-0">
                          <span class="font-medium text-foreground truncate">{employee.job_title || "—"}</span>
                          <span class="text-[11px] text-muted-foreground truncate">{employee.department || employee.org_unit_name || "—"}</span>
                        </div>
                      </td>

                      <!-- Staff Type -->
                      <td class="p-3">
                        <Badge variant="outline" class="text-[10px] uppercase font-mono">
                          {getTypeLabel(employee.employee_type)}
                        </Badge>
                      </td>

                      <!-- Status -->
                      <td class="p-3">
                        <Badge variant={getStatusBadgeVariant(employee.status)} class="text-[10px]">
                          {getStatusLabel(employee.status)}
                        </Badge>
                      </td>

                      <!-- Linked Account -->
                      <td class="p-3">
                        {#if employee.username}
                          <div class="flex items-center gap-1.5 text-xs text-foreground font-mono">
                            <UserBadgeCheck size={14} class="text-primary shrink-0" />
                            <span class="truncate max-w-32">{employee.username}</span>
                          </div>
                        {:else}
                          <span class="text-[11px] text-muted-foreground/70 italic">
                            {m.employees_no_account?.() || "لا يوجد حساب"}
                          </span>
                        {/if}
                      </td>

                      <!-- Hire Date -->
                      <td class="p-3 font-mono text-muted-foreground text-[11px]">
                        {employee.hire_date ? formatDate(employee.hire_date, "date") : "—"}
                      </td>

                      <!-- Actions Dropdown -->
                      <td class="p-3 text-center">
                        <DropdownMenu>
                          <DropdownMenuTrigger>
                            <Button variant="ghost" size="icon" class="size-7 p-0 cursor-pointer">
                              <EllipsisHorizontal size={15} />
                            </Button>
                          </DropdownMenuTrigger>
                          <DropdownMenuContent align="end" class="w-48 text-start text-xs font-sans">
                            <DropdownMenuItem
                              class="cursor-pointer gap-2"
                              onclick={() => goto(localizeHref(`/${orgId}/employees/${employee.id}`))}
                            >
                              <Eye size={13} />
                              <span>{m.employees_profile_title?.() || "استعراض الملف الشامل"}</span>
                            </DropdownMenuItem>

                            <DropdownMenuItem
                              class="cursor-pointer gap-2"
                              onclick={() => goto(localizeHref(`/${orgId}/employees/${employee.id}/edit`))}
                            >
                              <Pencil size={13} />
                              <span>{m.employees_edit_title?.() || "تعديل البيانات"}</span>
                            </DropdownMenuItem>

                            <DropdownMenuSeparator />

                            {#if employee.username}
                              <DropdownMenuItem
                                class="cursor-pointer gap-2 text-amber-700 dark:text-amber-400"
                                onclick={() => handleResetPassword(employee)}
                              >
                                <LockClosed size={13} />
                                <span>{m.employees_reset_password?.() || "إعادة تعيين كلمة المرور"}</span>
                              </DropdownMenuItem>
                            {:else}
                              <DropdownMenuItem
                                class="cursor-pointer gap-2 text-primary"
                                onclick={() => goto(localizeHref(`/${orgId}/employees/${employee.id}?tab=account`))}
                              >
                                <ShieldCheck size={13} />
                                <span>{m.employees_provision_account?.() || "إنشاء حساب مستخدم"}</span>
                              </DropdownMenuItem>
                            {/if}

                            <DropdownMenuSeparator />

                            {#if employee.status !== "active"}
                              <DropdownMenuItem
                                class="cursor-pointer gap-2 text-emerald-600 dark:text-emerald-400"
                                onclick={() => promptStatusChange(employee, "active")}
                              >
                                <CheckCircle size={13} />
                                <span>{l("تفعيل الموظف", "Set Active")}</span>
                              </DropdownMenuItem>
                            {/if}

                            {#if employee.status !== "on_leave"}
                              <DropdownMenuItem
                                class="cursor-pointer gap-2 text-amber-600 dark:text-amber-400"
                                onclick={() => promptStatusChange(employee, "on_leave")}
                              >
                                <BuildingOffice size={13} />
                                <span>{l("تعيين في إجازة", "Set On Leave")}</span>
                              </DropdownMenuItem>
                            {/if}

                            {#if employee.status !== "suspended"}
                              <DropdownMenuItem
                                class="cursor-pointer gap-2 text-rose-600 dark:text-rose-400"
                                onclick={() => promptStatusChange(employee, "suspended")}
                              >
                                <ShieldCheck size={13} />
                                <span>{l("إيقاف مؤقت", "Suspend")}</span>
                              </DropdownMenuItem>
                            {/if}
                          </DropdownMenuContent>
                        </DropdownMenu>
                      </td>
                    </tr>
                  {/each}
                </tbody>
              </table>
            </div>

            <!-- Footer Counter -->
            <div class="p-3 border-t border-border flex items-center justify-between bg-muted/20 text-xs text-muted-foreground shrink-0">
              <span>{l("عرض", "Showing")} {formatNumber(employees.length)} {l("من إجمالي", "of")} {formatNumber(totalCount)} {l("موظف", "employees")}</span>
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
            {formatNumber(selectedIds.size)} {l("موظف محدد", "selected")}
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
                onclick={() => goto(localizeHref(`/${orgId}/employees/${single.id}`))}
              >
                <Eye size={13} />
                <span>{l("عرض الملف", "View")}</span>
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
            <span>{m.common_export?.() || "تصدير"}</span>
          </Button>
        </div>
      </div>
    </div>
  {/if}

  <!-- Password Reset / Credential Delivery Modal -->
  <CredentialDeliveryModal
    bind:open={credentialModalOpen}
    provisioning={activeProvisioningResult}
    employeeName={activeEmployeeName}
  />

  <!-- Status Change Confirm Dialog -->
  <ConfirmDialog
    bind:open={statusDialogOpen}
    title={l("تأكيد تغيير الحالة الوظيفية", "Confirm Status Change")}
    description={l(`هل أنت متأكد من تغيير حالة الموظف (${targetEmployee?.full_name || targetEmployee?.first_name || ''}) إلى "${getStatusLabel(nextStatus)}"?`, `Are you sure you want to change this employee status to ${getStatusLabel(nextStatus)}?`)}
    confirmText={l("تأكيد التغيير", "Confirm Change")}
    onConfirm={confirmStatusChange}
  />
</div>
