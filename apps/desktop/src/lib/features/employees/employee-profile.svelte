<!--
  @component EmployeeProfile
  360-degree Employee Profile Dossier.
  Integrates full demographic data, job hierarchy, linked account lifecycle,
  group memberships, and effective permission inspection.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { goto } from "$app/navigation";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@orbis/ui/dialog";
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetDescription,
  SheetFooter,
} from "@orbis/ui/sheet";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import {
  ArrowPath,
  BookOpen,
  BuildingOffice,
  Calendar,
  CheckCircle,
  ClockRotateRight,
  Eye,
  Keyframes,
  LockClosed,
  Pencil,
  Phone,
  Plus,
  ShieldCheck,
  Trash,
  User,
  UserBadgeCheck,
  UserGroup,
  Users,
  XMark,
} from "@orbis/icons";
import { toast } from "@orbis/ui";
import { localizeHref } from "$lib/paraglide/runtime";
import { formatDate, l } from "$lib/core/i18n";
import { extractErrorMessage } from "$lib/api";
import type {
  EmployeeDetailDto,
  AccountProvisioningResultDto,
  ProvisionEmployeeAccountInput,
} from "$lib/api/types";
import {
  createEmployeeQuery,
  createResetEmployeePasswordMutation,
  createUpdateEmployeeStatusMutation,
} from "./queries";
import ProvisionAccountSheet from "./components/provision-account-sheet.svelte";
import ManageEmployeeGroupsSheet from "./components/manage-employee-groups-sheet.svelte";
import CredentialDeliveryModal from "./components/credential-delivery-modal.svelte";
import UserAvatar from "$lib/shared/components/user-avatar.svelte";
import ConfirmDialog from "$lib/shared/components/confirm-dialog.svelte";
import { IAM_DOMAINS } from "$lib/core/iam";

interface Props {
  orgId: string;
  employeeId: string;
}

let { orgId, employeeId }: Props = $props();

let activeTab = $state<"overview" | "employment" | "security">("overview");

// Employee detail query
const employeeQuery = createEmployeeQuery(() => employeeId);
const employee = $derived(employeeQuery.data);

// Mutations
const resetPasswordMutation = createResetEmployeePasswordMutation();
const statusMutation = createUpdateEmployeeStatusMutation();

// Modals & Sheets
let credentialModalOpen = $state(false);
let provisioningResult = $state<AccountProvisioningResultDto | null>(null);
let provisionDialogOpen = $state(false);
let groupsDialogOpen = $state(false);
let statusDialogOpen = $state(false);
let nextStatus = $state<string>("");

// Reset Password Handler
async function handleResetPassword() {
  if (!employee) return;
  try {
    const result = await resetPasswordMutation.mutateAsync(employee.id);
    provisioningResult = result;
    credentialModalOpen = true;
    toast.success(m.employees_password_reset_success?.() || "تمت إعادة ضبط كلمة المرور بنجاح");
  } catch (err) {
    toast.error(extractErrorMessage(err));
  }
}

// Status Toggle Handler
function promptStatusChange(status: string) {
  nextStatus = status;
  statusDialogOpen = true;
}

async function confirmStatusChange() {
  if (!employee || !nextStatus) return;
  try {
    await statusMutation.mutateAsync({ id: employee.id, status: nextStatus });
    toast.success(m.employees_status_updated?.() || "تم تحديث حالة الموظف بنجاح");
  } catch (err) {
    toast.error(extractErrorMessage(err));
  } finally {
    statusDialogOpen = false;
  }
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
</script>

<div class="h-full w-full flex flex-col overflow-hidden bg-background">
  {#if employeeQuery.isLoading}
    <div class="h-full w-full flex flex-col items-center justify-center gap-3 text-muted-foreground">
      <ArrowPath size={24} class="animate-spin text-primary" />
      <span class="text-xs">{l("جاري تحميل ملف الموظف...", "Loading employee dossier...")}</span>
    </div>
  {:else if employeeQuery.error || !employee}
    <div class="h-full w-full flex flex-col items-center justify-center p-6 text-center gap-3">
      <p class="text-xs text-destructive font-semibold">{extractErrorMessage(employeeQuery.error) || "Employee not found"}</p>
      <Button variant="outline" size="sm" class="text-xs" onclick={() => goto(localizeHref(`/${orgId}/employees`))}>
        {l("العودة لدليل الموظفين", "Back to Directory")}
      </Button>
    </div>
  {:else}
    <!-- Dossier Header Card -->
    <div class="border-b border-border bg-card px-6 py-5 shrink-0 shadow-2xs">
      <div class="max-w-5xl mx-auto flex items-center justify-between gap-4">
        <!-- Identity Summary -->
        <div class="flex items-center gap-4 min-w-0">
          <UserAvatar
            name={employee.full_name || employee.full_name_ar || employee.first_name}
            avatarUrl={employee.avatar_url ?? undefined}
            size="lg"
          />
          <div class="flex flex-col min-w-0">
            <div class="flex items-center gap-2.5">
              <h1 class="text-lg font-bold text-foreground truncate">
                {employee.full_name || employee.full_name_ar || `${employee.first_name} ${employee.last_name}`}
              </h1>
              <Badge variant={getStatusBadgeVariant(employee.status)} class="text-[10px]">
                {employee.status}
              </Badge>
              <Badge variant="outline" class="text-[10px] uppercase font-mono">
                {employee.employee_type}
              </Badge>
            </div>
            <div class="flex items-center gap-4 text-xs text-muted-foreground pt-1">
              {#if employee.job_title}
                <span class="flex items-center gap-1">
                  <BuildingOffice size={13} />
                  {employee.job_title}
                </span>
              {/if}
              {#if employee.department || employee.org_unit_name}
                <span class="flex items-center gap-1">
                  <BuildingOffice size={13} />
                  {employee.department || employee.org_unit_name}
                </span>
              {/if}
              {#if employee.employee_number}
                <span class="font-mono text-muted-foreground/80">
                  #{employee.employee_number}
                </span>
              {/if}
            </div>
          </div>
        </div>

        <!-- Quick Action Buttons -->
        <div class="flex items-center gap-2 shrink-0">
          <Button
            variant="outline"
            size="sm"
            class="text-xs cursor-pointer gap-1.5"
            onclick={() => goto(localizeHref(`/${orgId}/employees/${employee.id}/edit`))}
          >
            <Pencil size={13} />
            <span>{m.employees_edit_title?.() || "تعديل الملف"}</span>
          </Button>

          {#if employee.username}
            <Button
              variant="outline"
              size="sm"
              class="text-xs cursor-pointer gap-1.5 text-amber-700 dark:text-amber-400"
              onclick={handleResetPassword}
            >
              <LockClosed size={13} />
              <span>{m.employees_reset_password?.() || "إعادة تعيين كلمة المرور"}</span>
            </Button>
          {:else}
            <Button
              variant="default"
              size="sm"
              class="text-xs cursor-pointer gap-1.5"
              onclick={() => (provisionDialogOpen = true)}
            >
              <ShieldCheck size={13} />
              <span>{m.employees_provision_account?.() || "إنشاء حساب مستخدم"}</span>
            </Button>
          {/if}
        </div>
      </div>

      <!-- Dossier Tabs -->
      <div class="max-w-5xl mx-auto flex items-center gap-2 pt-5 border-t border-border/50 mt-4">
        <button
          type="button"
          class="px-3 py-1.5 text-xs font-medium rounded-md transition-all cursor-pointer flex items-center gap-1.5 {activeTab === 'overview' ? 'bg-primary/10 text-primary font-bold' : 'text-muted-foreground hover:text-foreground'}"
          onclick={() => (activeTab = "overview")}
        >
          <User size={14} />
          <span>{m.employees_tab_personal?.() || "البيانات الشخصية"}</span>
        </button>

        <button
          type="button"
          class="px-3 py-1.5 text-xs font-medium rounded-md transition-all cursor-pointer flex items-center gap-1.5 {activeTab === 'employment' ? 'bg-primary/10 text-primary font-bold' : 'text-muted-foreground hover:text-foreground'}"
          onclick={() => (activeTab = "employment")}
        >
          <BuildingOffice size={14} />
          <span>{m.employees_tab_employment?.() || "العقد والوظيفة"}</span>
        </button>

        <button
          type="button"
          class="px-3 py-1.5 text-xs font-medium rounded-md transition-all cursor-pointer flex items-center gap-1.5 {activeTab === 'security' ? 'bg-primary/10 text-primary font-bold' : 'text-muted-foreground hover:text-foreground'}"
          onclick={() => (activeTab = "security")}
        >
          <ShieldCheck size={14} />
          <span>{m.employees_tab_account?.() || "الأمان والصلاحيات"}</span>
        </button>
      </div>
    </div>

    <!-- Dossier Content Body -->
    <div class="flex-1 min-h-0 overflow-y-auto p-6">
      <div class="max-w-5xl mx-auto flex flex-col gap-6">

        <!-- TAB 1: OVERVIEW & PERSONAL DEMOGRAPHICS -->
        {#if activeTab === "overview"}
          <div class="grid grid-cols-2 gap-6">
            <!-- Personal Information Box -->
            <div class="flex flex-col gap-4 rounded-xl border border-border bg-card p-5 shadow-xs">
              <h2 class="text-xs font-bold text-foreground uppercase tracking-wider text-muted-foreground border-b border-border/60 pb-2">
                {l("الهوية والبيانات الشخصية", "Identity & Personal Details")}
              </h2>
              <div class="grid grid-cols-2 gap-y-3 text-xs">
                <div class="flex flex-col gap-0.5">
                  <span class="text-muted-foreground">{m.employees_first_name?.() || "الاسم بالإنجليزية"}:</span>
                  <span class="font-medium text-foreground">{employee.first_name} {employee.last_name}</span>
                </div>
                <div class="flex flex-col gap-0.5">
                  <span class="text-muted-foreground">{m.employees_first_name_ar?.() || "الاسم بالعربية"}:</span>
                  <span class="font-medium text-foreground">{employee.full_name_ar || "—"}</span>
                </div>
                <div class="flex flex-col gap-0.5">
                  <span class="text-muted-foreground">{m.employees_national_id?.() || "الهوية الوطنية"}:</span>
                  <span class="font-mono font-medium text-foreground">{employee.national_id || "—"}</span>
                </div>
                <div class="flex flex-col gap-0.5">
                  <span class="text-muted-foreground">{m.employees_passport_number?.() || "جواز السفر"}:</span>
                  <span class="font-mono font-medium text-foreground">{employee.passport_number || "—"}</span>
                </div>
                <div class="flex flex-col gap-0.5">
                  <span class="text-muted-foreground">{m.employees_gender?.() || "الجنس"}:</span>
                  <span class="font-medium text-foreground">{employee.gender || "—"}</span>
                </div>
                <div class="flex flex-col gap-0.5">
                  <span class="text-muted-foreground">{m.employees_birth_date?.() || "تاريخ الميلاد"}:</span>
                  <span class="font-mono text-foreground">{employee.birth_date ? formatDate(employee.birth_date, "date") : "—"}</span>
                </div>
                <div class="flex flex-col gap-0.5">
                  <span class="text-muted-foreground">{m.employees_nationality?.() || "الجنسية"}:</span>
                  <span class="font-medium text-foreground">{employee.nationality || "—"}</span>
                </div>
                <div class="flex flex-col gap-0.5">
                  <span class="text-muted-foreground">{m.employees_blood_type?.() || "فصيلة الدم"}:</span>
                  <span class="font-mono font-medium text-foreground">{employee.blood_type || "—"}</span>
                </div>
              </div>
            </div>

            <!-- Contact & Address Box -->
            <div class="flex flex-col gap-4 rounded-xl border border-border bg-card p-5 shadow-xs">
              <h2 class="text-xs font-bold text-foreground uppercase tracking-wider text-muted-foreground border-b border-border/60 pb-2">
                {l("بيانات الاتصال والعنوان", "Contact & Location")}
              </h2>
              <div class="flex flex-col gap-3 text-xs">
                <div class="flex flex-col gap-0.5">
                  <span class="text-muted-foreground">{m.employees_phone?.() || "رقم الهاتف"}:</span>
                  <span class="font-mono font-medium text-foreground">{employee.phone || "—"}</span>
                </div>
                <div class="flex flex-col gap-0.5">
                  <span class="text-muted-foreground">{m.employees_email?.() || "البريد الإلكتروني"}:</span>
                  <span class="font-mono font-medium text-foreground">{employee.email || "—"}</span>
                </div>
                <div class="flex flex-col gap-0.5">
                  <span class="text-muted-foreground">{m.employees_address?.() || "العنوان السكني"}:</span>
                  <span class="font-medium text-foreground">{employee.address || "—"}</span>
                </div>
              </div>
            </div>
          </div>
        {/if}

        <!-- TAB 2: EMPLOYMENT DETAILS -->
        {#if activeTab === "employment"}
          <div class="flex flex-col gap-5 rounded-xl border border-border bg-card p-6 shadow-xs">
            <h2 class="text-xs font-bold text-foreground uppercase tracking-wider text-muted-foreground border-b border-border/60 pb-2">
              {m.employees_tab_employment?.() || "البيانات الوظيفية والتعاقدية"}
            </h2>
            <div class="grid grid-cols-3 gap-y-4 text-xs">
              <div class="flex flex-col gap-0.5">
                <span class="text-muted-foreground">{m.employees_employee_number?.() || "الرقم الوظيفي"}:</span>
                <span class="font-mono font-bold text-foreground">{employee.employee_number || "—"}</span>
              </div>
              <div class="flex flex-col gap-0.5">
                <span class="text-muted-foreground">{m.employees_job_title?.() || "المسمى الوظيفي"}:</span>
                <span class="font-medium text-foreground">{employee.job_title || "—"}</span>
              </div>
              <div class="flex flex-col gap-0.5">
                <span class="text-muted-foreground">{m.employees_department?.() || "القسم / الإدارة"}:</span>
                <span class="font-medium text-foreground">{employee.department || "—"}</span>
              </div>
              <div class="flex flex-col gap-0.5">
                <span class="text-muted-foreground">{m.employees_hire_date?.() || "تاريخ التعيين"}:</span>
                <span class="font-mono text-foreground">{employee.hire_date ? formatDate(employee.hire_date, "date") : "—"}</span>
              </div>
              <div class="flex flex-col gap-0.5">
                <span class="text-muted-foreground">{m.employees_employment_type?.() || "نوع العقد"}:</span>
                <span class="font-medium text-foreground">{employee.employment_type}</span>
              </div>
              <div class="flex flex-col gap-0.5">
                <span class="text-muted-foreground">{m.employees_work_shift?.() || "فترة العمل"}:</span>
                <span class="font-medium text-foreground">{employee.work_shift || "—"}</span>
              </div>
            </div>

            {#if employee.notes}
              <div class="flex flex-col gap-1.5 border-t border-border/60 pt-4 text-xs">
                <span class="font-semibold text-muted-foreground">{m.employees_tab_notes?.() || "ملاحظات إدارية"}:</span>
                <p class="text-foreground leading-relaxed bg-muted/30 p-3 rounded-lg border border-border/40 whitespace-pre-wrap">{employee.notes}</p>
              </div>
            {/if}
          </div>
        {/if}

        <!-- TAB 3: SECURITY, USER GROUPS & PERMISSIONS -->
        {#if activeTab === "security"}
          <div class="flex flex-col gap-6">
            <!-- User Account Box -->
            <div class="flex flex-col gap-4 rounded-xl border border-border bg-card p-5 shadow-xs">
              <div class="flex items-center justify-between border-b border-border/60 pb-3">
                <div class="flex items-center gap-2">
                  <ShieldCheck size={16} class="text-primary" />
                  <h2 class="text-xs font-bold text-foreground uppercase tracking-wider">{l("حساب النظام الموحد", "System User Account")}</h2>
                </div>
                {#if employee.username}
                  <Badge variant="default" class="font-mono text-[10px]">
                    {m.employees_account_active?.() || "الحساب نشط ومربوط"}
                  </Badge>
                {:else}
                  <Badge variant="outline" class="text-[10px] text-muted-foreground">
                    {m.employees_no_account?.() || "غير مربوط بحساب مستخدم"}
                  </Badge>
                {/if}
              </div>

              {#if employee.username}
                <div class="grid grid-cols-3 gap-4 text-xs">
                  <div class="flex flex-col gap-0.5">
                    <span class="text-muted-foreground">{m.credentials_username?.() || "اسم المستخدم"}:</span>
                    <span class="font-mono font-bold text-foreground">{employee.username}</span>
                  </div>
                  <div class="flex flex-col gap-0.5">
                    <span class="text-muted-foreground">{l("حالة الحساب", "Account Status")}:</span>
                    <span class="font-medium text-foreground">{employee.user_status || "Active"}</span>
                  </div>
                  <div class="flex items-center justify-end gap-2">
                    <Button
                      variant="outline"
                      size="sm"
                      class="text-xs cursor-pointer gap-1.5 text-amber-700 dark:text-amber-400"
                      onclick={handleResetPassword}
                    >
                      <LockClosed size={13} />
                      <span>{m.employees_reset_password?.() || "إعادة ضبط كلمة المرور"}</span>
                    </Button>
                  </div>
                </div>
              {:else}
                <div class="flex items-center justify-between p-4 rounded-lg bg-muted/40 text-xs">
                  <div class="flex flex-col gap-0.5">
                    <span class="font-semibold text-foreground">{l("لم يتم إنشاء حساب مستخدم لهذا الموظف بعد", "No account provisioned yet")}</span>
                    <span class="text-[11px] text-muted-foreground">{l("يمكنك تهيئة حساب مستخدم الآن لمنحه الوصول للنظام والصلاحيات.", "Provision an account to grant system access.")}</span>
                  </div>
                  <Button
                    variant="default"
                    size="sm"
                    class="text-xs cursor-pointer gap-1.5"
                    onclick={() => (provisionDialogOpen = true)}
                  >
                    <Plus size={13} />
                    <span>{m.employees_provision_account?.() || "إنشاء الحساب الآن"}</span>
                  </Button>
                </div>
              {/if}
            </div>

            <!-- Assigned User Groups -->
            <div class="flex flex-col gap-4 rounded-xl border border-border bg-card p-5 shadow-xs">
              <div class="flex items-center justify-between border-b border-border/60 pb-3">
                <div class="flex items-center gap-2">
                  <UserGroup size={16} class="text-primary" />
                  <h2 class="text-xs font-bold text-foreground uppercase tracking-wider">{m.employees_user_groups?.() || "مجموعات المستخدمين"}</h2>
                </div>
                {#if employee.username}
                  <Button
                    variant="outline"
                    size="sm"
                    class="text-xs cursor-pointer gap-1.5"
                    onclick={openManageGroups}
                  >
                    <Pencil size={13} />
                    <span>{l("إدارة المجموعات", "Manage Groups")}</span>
                  </Button>
                {/if}
              </div>

              {#if employee.groups.length === 0}
                <span class="text-xs text-muted-foreground italic">{l("لا توجد مجموعات مسندة لهذا الموظف.", "No groups assigned.")}</span>
              {:else}
                <div class="flex flex-wrap gap-2">
                  {#each employee.groups as grp}
                    <Badge variant="secondary" class="text-xs py-1 px-2.5 font-medium flex items-center gap-1.5">
                      <UserGroup size={12} />
                      <span>{grp}</span>
                    </Badge>
                  {/each}
                </div>
              {/if}
            </div>

            <!-- Effective Permissions -->
            <div class="flex flex-col gap-4 rounded-xl border border-border bg-card p-5 shadow-xs">
              <div class="flex items-center justify-between border-b border-border/60 pb-3">
                <div class="flex items-center gap-2">
                  <Keyframes size={16} class="text-primary" />
                  <h2 class="text-xs font-bold text-foreground uppercase tracking-wider">{m.employees_tab_permissions?.() || "الصلاحيات الفعلية المحسوبة"}</h2>
                </div>
                <span class="text-xs font-mono text-muted-foreground">
                  {formatDate(new Date().toISOString(), "time")} ({employee.effective_permissions.length} {l("صلاحية", "permissions")})
                </span>
              </div>

              {#if employee.effective_permissions.length === 0}
                <span class="text-xs text-muted-foreground italic">{l("لا توجد صلاحيات فعالة ممنوحة.", "No effective permissions.")}</span>
              {:else}
                <div class="flex flex-wrap gap-1.5 max-h-60 overflow-y-auto p-1">
                  {#each employee.effective_permissions as perm}
                    <span class="font-mono text-[11px] bg-muted/60 text-foreground px-2 py-0.5 rounded-md border border-border/50">
                      {perm}
                    </span>
                  {/each}
                </div>
              {/if}
            </div>
          </div>
        {/if}

      </div>
    </div>
  {/if}

  <!-- Provision Account Sheet -->
  <ProvisionAccountSheet
    bind:open={provisionDialogOpen}
    {orgId}
    employeeId={employee.id}
    employeeName={employee.full_name || employee.first_name}
    onprovisioned={(result) => {
      provisioningResult = result;
      credentialModalOpen = true;
    }}
  />

  <!-- Manage User Groups Sheet -->
  <ManageEmployeeGroupsSheet
    bind:open={groupsDialogOpen}
    {orgId}
    employeeId={employee.id}
    employeeName={employee.full_name || employee.first_name}
    currentGroupIds={employee.groups}
    onsaved={() => employeeQuery.refetch()}
  />

  <!-- Credential Delivery Modal -->
  <CredentialDeliveryModal
    bind:open={credentialModalOpen}
    provisioning={provisioningResult}
    employeeName={employee.full_name || employee.first_name}
  />
</div>
