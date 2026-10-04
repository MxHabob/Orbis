<!--
  @component CreateUserSheet
  Enterprise IAM User Provisioning Sheet.
  Adheres strictly to the canonical Orbis Sheet structure (matching DiscountTemplateSheet / Form):
  - Accordion-based multi-section layout
  - FormField wrapper with typed validation & snippets
  - Integrations-driven dynamic delivery channel detection
  - Zero duplicate Person records (reusing authoritative employee, student & guardian person_id)
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { l } from "$lib/core/i18n";
import {
  api,
  queryKeys,
  type ProvisionUserRequest,
  type AccountProvisioningResultDto,
  type EmployeeDetailDto,
  type StudentDetailDto,
  type GuardianDto,
  type IntegrationDto,
} from "$lib/api";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Badge } from "@orbis/ui/badge";
import { Checkbox } from "@orbis/ui/checkbox";
import { FormField } from "@orbis/ui";
import * as Sheet from "@orbis/ui/sheet";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@orbis/ui/accordion";
import { toast } from "@orbis/ui";
import {
  AcademicCap,
  BuildingOffice,
  Check,
  CheckCircle,
  Eye,
  EyeSlash,
  InfoCircle,
  LockClosed,
  Phone,
  Printer,
  Refresh,
  ShieldCheck,
  Sparkles,
  User,
  UserGroup,
  UserPlus,
  Users,
} from "@orbis/icons";
import { createQuery, useQueryClient } from "@tanstack/svelte-query";
import { createEmployeesQuery } from "$lib/features/employees/queries";
import { createStudentsQuery } from "$lib/features/students/queries";
import { GroupSelector } from "$lib/features/groups";
import RoleSelect from "$lib/shared/components/role-select.svelte";
import SearchableSelect from "$lib/shared/components/searchable-select.svelte";

interface Props {
  open?: boolean;
  orgId: string;
  onsuccess?: (result: AccountProvisioningResultDto, userName: string) => void;
}

let {
  open = $bindable(false),
  orgId,
  onsuccess,
}: Props = $props();

const queryClient = useQueryClient();

// ── Persona & Entity State ───────────────────────────────────────────
type PersonaType = "employee" | "student" | "guardian" | "custom";
let personaType = $state<PersonaType>("employee");

let selectedEmployeeId = $state<string | null>(null);
let selectedStudentId = $state<string | null>(null);

// Guardian Linkage State
let selectedGuardianStudentId = $state<string | null>(null);
let selectedGuardianPersonId = $state<string | null>(null);
let guardianList = $state<GuardianDto[]>([]);
let isLoadingGuardians = $state(false);

// ── Account & Identity Fields ────────────────────────────────────────
let fullName = $state("");
let fullNameAr = $state("");
let email = $state("");
let phone = $state("");
let username = $state("");
let selectedRole = $state("staff");
let selectedGroupIds = $state<string[]>([]);

// ── Password & Security ──────────────────────────────────────────────
type PasswordMode = "auto" | "custom";
let passwordMode = $state<PasswordMode>("auto");
let customPassword = $state("");
let confirmPassword = $state("");
let showPassword = $state(false);

// ── Delivery Channel ─────────────────────────────────────────────────
let deliveryChannel = $state<string>("manual");
let isSubmitting = $state(false);
let showGroupsSection = $state(false);

// ── TanStack Queries ─────────────────────────────────────────────────
const employeesQuery = createEmployeesQuery(() => orgId, () => ({ limit: 300 }));
const studentsQuery = createStudentsQuery(() => orgId, () => ({ pageSize: 300 }));

// Dynamic Integrations Query (determines active SMS, WhatsApp, Email gateways)
const integrationsQuery = createQuery<IntegrationDto[]>(() => ({
  queryKey: ["integrations", orgId],
  queryFn: () => api.listIntegrations(),
  enabled: !!orgId,
  staleTime: 60 * 1000,
}));

const rawEmployees = $derived<EmployeeDetailDto[]>(employeesQuery.data?.items ?? []);
const rawStudents = $derived<StudentDetailDto[]>(studentsQuery.data?.items ?? []);
const integrations = $derived<IntegrationDto[]>(integrationsQuery.data ?? []);

// Dynamic Gateway Capabilities
const hasEmailIntegration = $derived(
  integrations.some(
    (i) =>
      i.is_enabled &&
      (i.category?.toLowerCase() === "email" ||
        ["smtp", "resend", "sendgrid", "mailgun", "postmark", "email"].includes(
          i.provider_name?.toLowerCase(),
        )),
  ),
);

const hasSmsIntegration = $derived(
  integrations.some(
    (i) =>
      i.is_enabled &&
      (i.category?.toLowerCase() === "sms" ||
        ["twilio", "unifonic", "yamamah", "sms"].includes(
          i.provider_name?.toLowerCase(),
        )),
  ),
);

const hasWhatsAppIntegration = $derived(
  integrations.some(
    (i) =>
      i.is_enabled &&
      (i.category?.toLowerCase() === "whatsapp" ||
        ["whatsapp", "meta", "twilio_whatsapp"].includes(
          i.provider_name?.toLowerCase(),
        )),
  ),
);

// Dynamic Available Delivery Channels
interface DeliveryOption {
  id: string;
  name: string;
  desc: string;
  icon: string;
  isAvailable: boolean;
  requiresIntegration?: string;
  badge?: string;
}

const availableChannels = $derived<DeliveryOption[]>([
  {
    id: "manual",
    name: m.settings_ui_delivery_manual_name(),
    desc: m.settings_ui_delivery_manual_description(),
    icon: "📋",
    isAvailable: true,
    badge: m.settings_ui_always_available(),
  },
  {
    id: "email",
    name: m.settings_ui_delivery_email_name(),
    desc: email ? m.settings_ui_delivery_send_to({ destination: email }) : m.settings_ui_delivery_email_description(),
    icon: "✉️",
    isAvailable: hasEmailIntegration,
    requiresIntegration: "SMTP / Email Gateway",
    badge: hasEmailIntegration ? m.settings_ui_integration_connected() : m.settings_ui_email_integration_required(),
  },
  {
    id: "sms",
    name: m.settings_ui_delivery_sms_name(),
    desc: phone ? m.settings_ui_delivery_send_to({ destination: phone }) : m.settings_ui_delivery_sms_description(),
    icon: "💬",
    isAvailable: hasSmsIntegration,
    requiresIntegration: "SMS Gateway (Twilio / Unifonic)",
    badge: hasSmsIntegration ? m.settings_ui_integration_connected() : m.settings_ui_sms_gateway_required(),
  },
  {
    id: "whatsapp",
    name: m.settings_ui_delivery_whatsapp_name(),
    desc: phone ? m.settings_ui_delivery_send_to({ destination: phone }) : m.settings_ui_delivery_whatsapp_description(),
    icon: "📱",
    isAvailable: hasWhatsAppIntegration,
    requiresIntegration: "Meta / WhatsApp Business API",
    badge: hasWhatsAppIntegration ? m.settings_ui_integration_connected() : m.settings_ui_whatsapp_integration_required(),
  },
]);

// Entity lookups
const unprovisionedEmployees = $derived(rawEmployees.filter((e) => !e.user_id));
const unprovisionedStudents = $derived(rawStudents.filter((s) => !s.student?.user_id));

const currentEmployee = $derived(
  rawEmployees.find((e) => e.id === selectedEmployeeId) ?? null,
);
const currentStudent = $derived(
  rawStudents.find((s) => s.student?.id === selectedStudentId) ?? null,
);
const currentGuardianStudent = $derived(
  rawStudents.find((s) => s.student?.id === selectedGuardianStudentId) ?? null,
);

// ── Helper: Username Generation ──────────────────────────────────────
function cleanUsernameSegment(str: string): string {
  return str
    .trim()
    .toLowerCase()
    .replace(/[^\w\s-]/g, "")
    .replace(/\s+/g, ".")
    .replace(/_{2,}/g, "_")
    .replace(/\.{2,}/g, ".");
}

function generateSuggestedUsername() {
  if (personaType === "employee" && currentEmployee) {
    const fn = currentEmployee.first_name ? cleanUsernameSegment(currentEmployee.first_name) : "";
    const ln = currentEmployee.last_name ? cleanUsernameSegment(currentEmployee.last_name) : "";
    const num = currentEmployee.employee_number ? cleanUsernameSegment(currentEmployee.employee_number) : "";
    if (fn && ln) {
      username = `${fn}.${ln}`;
    } else if (fn && num) {
      username = `${fn}.${num}`;
    } else if (num) {
      username = `emp.${num}`;
    }
  } else if (personaType === "student" && currentStudent) {
    const fn = currentStudent.person?.first_name ? cleanUsernameSegment(currentStudent.person.first_name) : "";
    const num = currentStudent.student?.student_number ? cleanUsernameSegment(currentStudent.student.student_number) : "";
    if (fn && num) {
      username = `${fn}.${num}`;
    } else if (num) {
      username = `std.${num}`;
    } else if (fn) {
      username = `std.${fn}`;
    }
  } else if (personaType === "guardian" && fullName) {
    const seg = cleanUsernameSegment(fullName);
    if (seg) username = `p.${seg}`;
  } else if (fullName) {
    const seg = cleanUsernameSegment(fullName);
    if (seg) username = seg;
  }
}

// Reset form
function resetForm() {
  selectedEmployeeId = null;
  selectedStudentId = null;
  selectedGuardianStudentId = null;
  selectedGuardianPersonId = null;
  guardianList = [];
  isLoadingGuardians = false;
  fullName = "";
  fullNameAr = "";
  email = "";
  phone = "";
  username = "";
  customPassword = "";
  confirmPassword = "";
  passwordMode = "auto";
  deliveryChannel = "manual";
  selectedGroupIds = [];
  isSubmitting = false;
  showGroupsSection = false;
}

$effect(() => {
  if (!open) {
    resetForm();
  }
});

function handlePersonaChange(type: PersonaType) {
  personaType = type;
  resetForm();

  if (type === "employee") {
    selectedRole = "staff";
  } else if (type === "student") {
    selectedRole = "student";
  } else if (type === "guardian") {
    selectedRole = "parent";
  } else {
    selectedRole = "staff";
  }
}

function handleSelectEmployee(empId: string | null) {
  selectedEmployeeId = empId;
  const emp = rawEmployees.find((e) => e.id === empId);
  if (!emp) return;

  fullName = `${emp.first_name || ""} ${emp.last_name || ""}`.trim();
  fullNameAr = `${emp.first_name_ar || ""} ${emp.last_name_ar || ""}`.trim();
  email = emp.email || "";
  phone = emp.phone || "";
  selectedRole = emp.employee_type === "teacher" ? "teacher" : "staff";
  generateSuggestedUsername();
}

function handleSelectStudent(stdId: string | null) {
  selectedStudentId = stdId;
  const std = rawStudents.find((s) => s.student?.id === stdId);
  if (!std) return;

  fullName = `${std.person?.first_name || ""} ${std.person?.last_name || ""}`.trim();
  fullNameAr = `${std.person?.first_name_ar || ""} ${std.person?.last_name_ar || ""}`.trim();
  email = std.person?.email || "";
  phone = std.person?.phone || "";
  selectedRole = "student";
  generateSuggestedUsername();
}

async function handleSelectGuardianStudent(stdId: string | null) {
  selectedGuardianStudentId = stdId;
  selectedGuardianPersonId = null;
  guardianList = [];
  if (!stdId) return;

  isLoadingGuardians = true;
  try {
    const list = await api.listGuardians(stdId);
    guardianList = list ?? [];
  } catch (err) {
    console.error("Failed to fetch student guardians:", err);
    guardianList = [];
  } finally {
    isLoadingGuardians = false;
  }
}

function handleSelectGuardianRecord(g: GuardianDto) {
  selectedGuardianPersonId = g.guardian_id;
  const rel = l(g.relationship_ar, g.relationship) || g.relationship || "Guardian";
  const stdName = currentGuardianStudent
    ? l(
        `${currentGuardianStudent.person?.first_name_ar || ""} ${currentGuardianStudent.person?.last_name_ar || ""}`.trim(),
        `${currentGuardianStudent.person?.first_name || ""} ${currentGuardianStudent.person?.last_name || ""}`.trim(),
      )
    : "";
  fullName = `${rel} - ${stdName}`.trim();
  fullNameAr = `${g.relationship_ar || rel} - ${stdName}`.trim();
  phone = g.work_phone || "";
  selectedRole = "parent";
  generateSuggestedUsername();
}

// ── Submit Provisioning ──────────────────────────────────────────────
async function handleSubmit() {
  const finalUsername = username.trim().toLowerCase();
  if (!finalUsername) {
    toast.error(m.ui_copy_please_enter_a_valid_username_5bef8f34());
    return;
  }

  if (finalUsername.length < 3) {
    toast.error(m.settings_ui_username_min_length_error());
    return;
  }

  if (!selectedRole) {
    toast.error(m.ui_copy_please_select_a_user_role_e59e6c02());
    return;
  }

  if (passwordMode === "custom") {
    if (!customPassword || customPassword.length < 6) {
      toast.error(
        m.ui_copy_password_must_be_at_least_6_characters_921652ea(),
      );
      return;
    }
    if (customPassword !== confirmPassword) {
      toast.error(m.setup_bootstrap_err_password_mismatch());
      return;
    }
  }

  // Validate channel availability
  const selectedChannelObj = availableChannels.find((c) => c.id === deliveryChannel);
  if (selectedChannelObj && !selectedChannelObj.isAvailable && deliveryChannel !== "manual") {
    toast.error(m.settings_ui_delivery_channel_not_enabled({ channel: selectedChannelObj.name }));
    return;
  }

  isSubmitting = true;
  try {
    const payload: ProvisionUserRequest = {
      actor_type: personaType,
      roles: [selectedRole],
      preferred_username: finalUsername,
      delivery_channel: deliveryChannel,
      group_ids: selectedGroupIds.length > 0 ? selectedGroupIds : undefined,
    };

    // Strict Persona Linkage: reusing authoritative person_id
    if (personaType === "employee" && selectedEmployeeId) {
      payload.employee_id = selectedEmployeeId;
      if (currentEmployee?.person_id) {
        payload.person_id = currentEmployee.person_id;
      }
    } else if (personaType === "student" && selectedStudentId) {
      payload.student_id = selectedStudentId;
      if (currentStudent?.person?.id) {
        payload.person_id = currentStudent.person.id;
      }
    } else if (personaType === "guardian" && selectedGuardianPersonId) {
      payload.guardian_id = selectedGuardianPersonId;
      payload.person_id = selectedGuardianPersonId;
    }

    payload.full_name = fullName || undefined;
    payload.full_name_ar = fullNameAr || undefined;
    payload.email = email || undefined;
    payload.phone = phone || undefined;

    if (passwordMode === "custom") {
      payload.custom_password = customPassword;
    }

    const result = await api.provisionUser(payload);

    // Invalidate caches
    await Promise.all([
      queryClient.invalidateQueries({ queryKey: queryKeys.auth.users() }),
      queryClient.invalidateQueries({ queryKey: ["users_paginated", orgId] }),
      queryClient.invalidateQueries({ queryKey: queryKeys.employee.all }),
      queryClient.invalidateQueries({ queryKey: queryKeys.student.all }),
    ]);

    toast.success(m.settings_ui_user_account_provisioned({ username: finalUsername }));

    const displayName = l(fullNameAr, fullName) || fullName || finalUsername;
    open = false;
    onsuccess?.(result, displayName);
  } catch (err: any) {
    console.error("User provisioning error:", err);
    toast.error(err?.message || m.ui_copy_failed_to_provision_user_account_a5de6e16());
  } finally {
    isSubmitting = false;
  }
}
</script>

<Sheet.Root bind:open>
  <Sheet.Content side="right" class="w-105 sm:w-125 md:w-140 h-full p-0 flex flex-col overflow-hidden">
    <form
      onsubmit={(e) => {
        e.preventDefault();
        handleSubmit();
      }}
      class="flex flex-col h-full overflow-hidden gap-0"
    >
      <!-- Sheet Header (DiscountTemplate style) -->
      <Sheet.Header class="px-6 py-4 border-b border-border/40 shrink-0 text-start">
        <Sheet.Title class="text-base font-bold text-foreground flex items-center gap-2">
          <UserPlus size={18} class="text-primary" />
          <span>{m.ui_copy_provision_user_account_796336f2()}</span>
        </Sheet.Title>
        <p class="text-xs text-muted-foreground mt-0.5">
          {m.settings_ui_user_provisioning_description()}
        </p>
      </Sheet.Header>

      <!-- Scrollable Accordion Content Area -->
      <div class="flex-1 overflow-y-auto p-6 scrollbar-hide space-y-4">
        <Accordion
          type="multiple"
          value={["persona", "identity", "security", "channels"]}
          class="flex flex-col gap-4"
        >
          <!-- 1. Persona & Target Entity Selection -->
          <AccordionItem value="persona" class="rounded-lg border border-border/60 overflow-hidden bg-card/40">
            <AccordionTrigger class="px-4 py-3 text-sm font-semibold hover:no-underline">
              <div class="flex items-center gap-2">
                <User size={15} class="text-primary" />
                <span>{m.ui_copy_account_persona_target_entity_a82adc96()}</span>
              </div>
            </AccordionTrigger>
            <AccordionContent class="px-4 pb-4 pt-1 space-y-4">
              <!-- Persona Grid -->
              <div class="grid grid-cols-2 sm:grid-cols-4 gap-2">
                <!-- Employee Card -->
                <button
                  type="button"
                  class="flex flex-col items-start gap-1 p-2.5 rounded-lg border text-start transition-all cursor-pointer {personaType === 'employee' ? 'border-primary bg-primary/10 ring-1 ring-primary/30 font-bold' : 'border-border/70 hover:bg-muted/30'}"
                  onclick={() => handlePersonaChange("employee")}
                >
                  <div class="flex items-center justify-between w-full">
                    <BuildingOffice size={14} class={personaType === 'employee' ? 'text-primary' : 'text-muted-foreground'} />
                    {#if personaType === "employee"}
                      <CheckCircle size={12} class="text-primary" />
                    {/if}
                  </div>
                  <span class="text-xs text-foreground mt-0.5">{m.ui_copy_employee_d0311cb2()}</span>
                </button>

                <!-- Student Card -->
                <button
                  type="button"
                  class="flex flex-col items-start gap-1 p-2.5 rounded-lg border text-start transition-all cursor-pointer {personaType === 'student' ? 'border-primary bg-primary/10 ring-1 ring-primary/30 font-bold' : 'border-border/70 hover:bg-muted/30'}"
                  onclick={() => handlePersonaChange("student")}
                >
                  <div class="flex items-center justify-between w-full">
                    <AcademicCap size={14} class={personaType === 'student' ? 'text-primary' : 'text-muted-foreground'} />
                    {#if personaType === "student"}
                      <CheckCircle size={12} class="text-primary" />
                    {/if}
                  </div>
                  <span class="text-xs text-foreground mt-0.5">{m.common_ledger_entity_student()}</span>
                </button>

                <!-- Guardian Card -->
                <button
                  type="button"
                  class="flex flex-col items-start gap-1 p-2.5 rounded-lg border text-start transition-all cursor-pointer {personaType === 'guardian' ? 'border-primary bg-primary/10 ring-1 ring-primary/30 font-bold' : 'border-border/70 hover:bg-muted/30'}"
                  onclick={() => handlePersonaChange("guardian")}
                >
                  <div class="flex items-center justify-between w-full">
                    <Users size={14} class={personaType === 'guardian' ? 'text-primary' : 'text-muted-foreground'} />
                    {#if personaType === "guardian"}
                      <CheckCircle size={12} class="text-primary" />
                    {/if}
                  </div>
                  <span class="text-xs text-foreground mt-0.5">{m.common_users_filter_guardian()}</span>
                </button>

                <!-- Custom Card -->
                <button
                  type="button"
                  class="flex flex-col items-start gap-1 p-2.5 rounded-lg border text-start transition-all cursor-pointer {personaType === 'custom' ? 'border-primary bg-primary/10 ring-1 ring-primary/30 font-bold' : 'border-border/70 hover:bg-muted/30'}"
                  onclick={() => handlePersonaChange("custom")}
                >
                  <div class="flex items-center justify-between w-full">
                    <ShieldCheck size={14} class={personaType === 'custom' ? 'text-primary' : 'text-muted-foreground'} />
                    {#if personaType === "custom"}
                      <CheckCircle size={12} class="text-primary" />
                    {/if}
                  </div>
                  <span class="text-xs text-foreground mt-0.5">{m.common_onboarding_break_custom()}</span>
                </button>
              </div>

              <!-- Persona Picker (Employee) -->
              {#if personaType === "employee"}
                <FormField id="emp_picker" label={m.ui_copy_linked_employee_profile_2de2ffb8()} required>
                  {#snippet children()}
                  <SearchableSelect
                    id="emp_picker"
                    items={rawEmployees}
                    value={selectedEmployeeId}
                    placeholder={m.ui_copy_search_by_name_id_or_department_3f1dc5aa()}
                    emptyText={m.ui_copy_no_employees_found_3814cebc()}
                    getItemValue={(emp: EmployeeDetailDto) => emp.id}
                    getItemLabel={(emp: EmployeeDetailDto) =>
                      l(
                        `${emp.first_name_ar || ""} ${emp.last_name_ar || ""}`.trim(),
                        `${emp.first_name} ${emp.last_name}`.trim(),
                      ) || `${emp.first_name} ${emp.last_name}`}
                    getItemSubtitle={(emp: EmployeeDetailDto) =>
                      `${emp.employee_number || "—"} • ${l(emp.department, emp.department) || emp.employee_type}`}
                    getItemBadge={(emp: EmployeeDetailDto) =>
                      emp.user_id ? m.ui_copy_has_account_ffd704f7() : m.ui_copy_available_eebdcd8c()}
                    onchange={handleSelectEmployee}
                  />
                  {/snippet}
                </FormField>

                {#if currentEmployee}
                  <div class="flex items-center gap-2 text-xs {currentEmployee.user_id ? 'text-amber-600 dark:text-amber-400 bg-amber-500/10' : 'text-emerald-600 dark:text-emerald-400 bg-emerald-500/10'} p-2 rounded-lg border border-border/40">
                    <InfoCircle size={13} class="shrink-0" />
                    <span>
                      {currentEmployee.user_id
                        ? l(
                            m.settings_ui_employee_already_has_account(),
                            "Employee already has an account. Credentials will update their original Person identity.",
                          )
                        : l(
                            m.settings_ui_employee_account_link_description(),
                            "Account will link directly to employee record without duplicating person details.",
                          )}
                    </span>
                  </div>
                {/if}
              {:else if personaType === "student"}
                <FormField id="std_picker" label={m.ui_copy_linked_student_profile_72144e3a()} required>
                  {#snippet children()}
                  <SearchableSelect
                    id="std_picker"
                    items={rawStudents}
                    value={selectedStudentId}
                    placeholder={m.ui_copy_search_by_name_student_or_class_7958c831()}
                    emptyText={m.common_table_no_students()}
                    getItemValue={(std: StudentDetailDto) => std.student?.id || ""}
                    getItemLabel={(std: StudentDetailDto) =>
                      l(
                        `${std.person?.first_name_ar || ""} ${std.person?.last_name_ar || ""}`.trim(),
                        `${std.person?.first_name || ""} ${std.person?.last_name || ""}`.trim(),
                      ) || m.common_ledger_entity_student()}
                    getItemSubtitle={(std: StudentDetailDto) =>
                      `${std.student?.student_number || "—"} • ${std.enrollment?.class_name || m.ui_copy_unassigned_df62ef28()}`}
                    getItemBadge={(std: StudentDetailDto) =>
                      std.student?.user_id ? m.ui_copy_has_account_ffd704f7() : m.ui_copy_available_eebdcd8c()}
                    onchange={handleSelectStudent}
                  />
                  {/snippet}
                </FormField>

                {#if currentStudent}
                  <div class="flex items-center gap-2 text-xs {currentStudent.student?.user_id ? 'text-amber-600 dark:text-amber-400 bg-amber-500/10' : 'text-emerald-600 dark:text-emerald-400 bg-emerald-500/10'} p-2 rounded-lg border border-border/40">
                    <InfoCircle size={13} class="shrink-0" />
                    <span>
                      {currentStudent.student?.user_id
                        ? l(
                            m.settings_ui_student_already_has_account(),
                            "This student already has an active user account.",
                          )
                        : l(
                            m.settings_ui_student_account_link_description(),
                            "Account will link directly to the student record without duplicate Person row.",
                          )}
                    </span>
                  </div>
                {/if}
              {:else if personaType === "guardian"}
                <div class="space-y-3">
                  <FormField id="g_std_picker" label={m.ui_copy_select_student_to_find_linked_guardians_0f20f702()}>
                    {#snippet children()}
                    <SearchableSelect
                      id="g_std_picker"
                      items={rawStudents}
                      value={selectedGuardianStudentId}
                      placeholder={m.ui_copy_search_student_by_name_or_number_494f6958()}
                      emptyText={m.common_table_no_students()}
                      getItemValue={(std: StudentDetailDto) => std.student?.id || ""}
                      getItemLabel={(std: StudentDetailDto) =>
                        l(
                          `${std.person?.first_name_ar || ""} ${std.person?.last_name_ar || ""}`.trim(),
                          `${std.person?.first_name || ""} ${std.person?.last_name || ""}`.trim(),
                        ) || m.common_ledger_entity_student()}
                      getItemSubtitle={(std: StudentDetailDto) =>
                        `${std.student?.student_number || "—"} • ${std.enrollment?.class_name || ""}`}
                      onchange={handleSelectGuardianStudent}
                    />
                    {/snippet}
                  </FormField>

                  {#if isLoadingGuardians}
                    <div class="flex items-center gap-2 text-xs text-muted-foreground p-2">
                      <Refresh size={13} class="animate-spin text-primary" />
                      <span>{m.ui_copy_loading_registered_guardians_16d08b44()}</span>
                    </div>
                  {:else if selectedGuardianStudentId}
                    {#if guardianList.length > 0}
                      <div class="space-y-1.5">
                        <Label class="text-xs font-semibold">{m.ui_copy_select_guardian_to_link_account_700d1ac5()}</Label>
                        <div class="grid grid-cols-1 sm:grid-cols-2 gap-2">
                          {#each guardianList as g (g.id)}
                            <button
                              type="button"
                              class="flex flex-col items-start gap-1 p-2.5 rounded-lg border text-start transition-all cursor-pointer {selectedGuardianPersonId === g.guardian_id ? 'border-primary bg-primary/10 ring-1 ring-primary/40 font-bold' : 'border-border/70 hover:bg-muted/40'}"
                              onclick={() => handleSelectGuardianRecord(g)}
                            >
                              <div class="flex items-center justify-between w-full">
                                <span class="text-xs text-foreground">{l(g.relationship_ar, g.relationship) || g.relationship}</span>
                                {#if g.is_primary}
                                  <Badge variant="outline" class="text-[9px] bg-primary/10 text-primary">{m.ui_copy_primary_13a3febb()}</Badge>
                                {/if}
                              </div>
                              {#if g.work_phone}
                                <span class="text-[10px] font-mono text-muted-foreground flex items-center gap-1">
                                  <Phone size={10} /> {g.work_phone}
                                </span>
                              {/if}
                            </button>
                          {/each}
                        </div>
                      </div>
                    {:else}
                      <div class="text-xs text-muted-foreground p-2 bg-muted/30 rounded-lg">
                        {m.ui_copy_no_registered_guardians_found_you_may_fill_0fe73d2f()}
                      </div>
                    {/if}
                  {/if}
                </div>
              {/if}
            </AccordionContent>
          </AccordionItem>

          <!-- 2. Account & Demographic Details -->
          <AccordionItem value="identity" class="rounded-lg border border-border/60 overflow-hidden bg-card/40">
            <AccordionTrigger class="px-4 py-3 text-sm font-semibold hover:no-underline">
              <div class="flex items-center gap-2">
                <ShieldCheck size={15} class="text-primary" />
                <span>{m.ui_copy_identity_account_details_b5154e18()}</span>
              </div>
            </AccordionTrigger>
            <AccordionContent class="px-4 pb-4 pt-1 space-y-3">
              <!-- Full Name Grid -->
              <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                <FormField id="dt_user_name_en" label={m.ui_copy_full_name_en_84cacaf7()}>
                  {#snippet children()}
                  <Input
                    id="dt_user_name_en"
                    type="text"
                    placeholder={m.settings_ui_user_name_english_example()}
                    bind:value={fullName}
                    oninput={generateSuggestedUsername}
                    disabled={isSubmitting}
                    class="h-9 text-xs"
                  />
                  {/snippet}
                </FormField>

                <FormField id="dt_user_name_ar" label={m.ui_copy_full_name_ar_9214cac5()}>
                  {#snippet children()}
                  <Input
                    id="dt_user_name_ar"
                    type="text"
                    placeholder={m.settings_ui_user_name_arabic_example()}
                    bind:value={fullNameAr}
                    disabled={isSubmitting}
                    class="h-9 text-xs"
                  />
                  {/snippet}
                </FormField>
              </div>

              <!-- Username & Role Grid -->
              <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                <!-- Username -->
                <FormField id="dt_username" label={m.ui_copy_username_handle_914729f0()} required>
                  {#snippet children()}
                  <div class="relative">
                    <Input
                      id="dt_username"
                      type="text"
                      placeholder={m.settings_ui_username_example()}
                      bind:value={username}
                      disabled={isSubmitting}
                      class="h-9 text-xs font-mono lowercase pe-16"
                      required
                    />
                    <button
                      type="button"
                      class="absolute inset-e-2 top-1/2 -translate-y-1/2 text-[10px] text-primary hover:underline cursor-pointer flex items-center gap-0.5"
                      onclick={generateSuggestedUsername}
                    >
                      <Sparkles size={11} />
                      <span>{m.ui_copy_suggest_5645a4c7()}</span>
                    </button>
                  </div>
                  {/snippet}
                </FormField>

                <!-- Role -->
                <FormField id="dt_role" label={m.ui_copy_system_role_ae5b071c()} required>
                  {#snippet children()}
                  <RoleSelect
                    bind:value={selectedRole}
                    disabled={isSubmitting}
                    size="sm"
                  />
                  {/snippet}
                </FormField>
              </div>

              <!-- Contact Details Grid -->
              <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                <FormField id="dt_email" label={m.employees_email()}>
                  {#snippet children()}
                  <Input
                    id="dt_email"
                    type="email"
                    placeholder={m.settings_ui_email_example()}
                    bind:value={email}
                    disabled={isSubmitting}
                    class="h-9 text-xs"
                  />
                  {/snippet}
                </FormField>

                <FormField id="dt_phone" label={m.ui_copy_phone_number_d163a03d()}>
                  {#snippet children()}
                  <Input
                    id="dt_phone"
                    type="tel"
                    placeholder="+967 770 000 000"
                    bind:value={phone}
                    disabled={isSubmitting}
                    class="h-9 text-xs font-mono"
                  />
                  {/snippet}
                </FormField>
              </div>
            </AccordionContent>
          </AccordionItem>

          <!-- 3. Password & Security -->
          <AccordionItem value="security" class="rounded-lg border border-border/60 overflow-hidden bg-card/40">
            <AccordionTrigger class="px-4 py-3 text-sm font-semibold hover:no-underline">
              <div class="flex items-center gap-2">
                <LockClosed size={15} class="text-primary" />
                <span>{m.ui_copy_password_security_0d1cc84e()}</span>
              </div>
            </AccordionTrigger>
            <AccordionContent class="px-4 pb-4 pt-1 space-y-3">
              <!-- Password Mode Grid -->
              <div class="grid grid-cols-1 sm:grid-cols-2 gap-2.5">
                <button
                  type="button"
                  class="flex items-start gap-2.5 p-3 rounded-lg border text-start transition-all cursor-pointer {passwordMode === 'auto' ? 'border-primary bg-primary/10 ring-1 ring-primary/30 font-semibold' : 'border-border/70 hover:bg-muted/30'}"
                  onclick={() => (passwordMode = "auto")}
                >
                  <div class="mt-0.5 flex size-3.5 shrink-0 items-center justify-center rounded-full border border-primary {passwordMode === 'auto' ? 'bg-primary' : ''}">
                    {#if passwordMode === "auto"}
                      <div class="size-1 rounded-full bg-white"></div>
                    {/if}
                  </div>
                  <div class="flex flex-col gap-0.5">
                    <span class="text-xs text-foreground flex items-center gap-1.5">
                      <span>{m.ui_copy_auto_generate_password_b761f0fd()}</span>
                      <Badge variant="outline" class="text-[9px] px-1 py-0 bg-primary/10 text-primary">{m.ui_copy_recommended_4917dbf2()}</Badge>
                    </span>
                    <span class="text-[10px] text-muted-foreground leading-normal">
                      {m.ui_copy_secure_12_char_temp_password_shown_once_on_75af1dae()}
                    </span>
                  </div>
                </button>

                <button
                  type="button"
                  class="flex items-start gap-2.5 p-3 rounded-lg border text-start transition-all cursor-pointer {passwordMode === 'custom' ? 'border-primary bg-primary/10 ring-1 ring-primary/30 font-semibold' : 'border-border/70 hover:bg-muted/30'}"
                  onclick={() => (passwordMode = "custom")}
                >
                  <div class="mt-0.5 flex size-3.5 shrink-0 items-center justify-center rounded-full border border-primary {passwordMode === 'custom' ? 'bg-primary' : ''}">
                    {#if passwordMode === "custom"}
                      <div class="size-1 rounded-full bg-white"></div>
                    {/if}
                  </div>
                  <div class="flex flex-col gap-0.5">
                    <span class="text-xs text-foreground">{m.ui_copy_custom_password_c4ade6bf()}</span>
                    <span class="text-[10px] text-muted-foreground leading-normal">
                      {m.ui_copy_specify_a_direct_custom_password_748d3be2()}
                    </span>
                  </div>
                </button>
              </div>

              <!-- Custom Password Fields -->
              {#if passwordMode === "custom"}
                <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 pt-1">
                  <FormField id="dt_pass" label={m.settings_security_new_password()} required>
                    {#snippet children()}
                    <div class="relative">
                      <Input
                        id="dt_pass"
                        type={showPassword ? "text" : "password"}
                        placeholder="••••••••••••"
                        bind:value={customPassword}
                        disabled={isSubmitting}
                        class="h-9 pe-8 text-xs font-mono"
                      />
                      <button
                        type="button"
                        class="absolute inset-e-2 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground cursor-pointer"
                        onclick={() => (showPassword = !showPassword)}
                      >
                        {#if showPassword}
                          <EyeSlash size={13} />
                        {:else}
                          <Eye size={13} />
                        {/if}
                      </button>
                    </div>
                    {/snippet}
                  </FormField>

                  <FormField id="dt_pass_confirm" label={m.settings_security_confirm_password()} required>
                    {#snippet children()}
                    <Input
                      id="dt_pass_confirm"
                      type={showPassword ? "text" : "password"}
                      placeholder="••••••••••••"
                      bind:value={confirmPassword}
                      disabled={isSubmitting}
                      class="h-9 text-xs font-mono"
                    />
                    {/snippet}
                  </FormField>
                </div>
              {/if}
            </AccordionContent>
          </AccordionItem>

          <!-- 4. Dynamic Integrations-Based Delivery Channels -->
          <AccordionItem value="channels" class="rounded-lg border border-border/60 overflow-hidden bg-card/40">
            <AccordionTrigger class="px-4 py-3 text-sm font-semibold hover:no-underline">
              <div class="flex items-center gap-2">
                <Printer size={15} class="text-primary" />
                <span>{m.ui_copy_delivery_channels_connected_gateways_31a74a07()}</span>
              </div>
            </AccordionTrigger>
            <AccordionContent class="px-4 pb-4 pt-1 space-y-3">
              <p class="text-[11px] text-muted-foreground">
                {l(
                  m.settings_ui_delivery_channel_integrations_hint(),
                  "Delivery channels depend on active integration gateways (SMS, WhatsApp, SMTP) in workspace settings:",
                )}
              </p>

              <!-- Dynamic Channels Grid -->
              <div class="grid grid-cols-1 sm:grid-cols-2 gap-2.5">
                {#each availableChannels as channel (channel.id)}
                  <button
                    type="button"
                    class="flex items-start gap-2.5 p-3 rounded-lg border text-start transition-all cursor-pointer {deliveryChannel === channel.id ? 'border-primary bg-primary/10 ring-1 ring-primary/40' : (channel.isAvailable ? 'border-border/70 hover:bg-muted/30' : 'border-border/40 opacity-60 bg-muted/10')}"
                    onclick={() => {
                      if (channel.isAvailable) {
                        deliveryChannel = channel.id;
                      } else {
                        toast.info(m.ui_integration_not_configured({ channelName: channel.name }));
                      }
                    }}
                  >
                    <span class="text-base mt-0.5">{channel.icon}</span>
                    <div class="flex flex-col gap-0.5 flex-1 min-w-0">
                      <div class="flex items-center justify-between gap-1">
                        <span class="text-xs font-bold text-foreground truncate">{channel.name}</span>
                        {#if channel.badge}
                          <Badge
                            variant={channel.isAvailable ? "default" : "outline"}
                            class="text-[9px] px-1 py-0 shrink-0 font-normal {channel.isAvailable ? 'bg-primary/15 text-primary border-primary/20' : 'text-muted-foreground'}"
                          >
                            {channel.badge}
                          </Badge>
                        {/if}
                      </div>
                      <span class="text-[10px] text-muted-foreground line-clamp-1 leading-normal">
                        {channel.desc}
                      </span>
                    </div>
                  </button>
                {/each}
              </div>

              <!-- User Groups Toggle -->
              <div class="pt-2 border-t border-border/40">
                <div class="flex items-center justify-between">
                  <span class="text-xs font-semibold text-foreground flex items-center gap-1.5">
                    <UserGroup size={13} class="text-primary" />
                    <span>{m.ui_copy_user_groups_fa7b35be()}</span>
                  </span>
                  <button
                    type="button"
                    class="text-[11px] text-primary hover:underline cursor-pointer"
                    onclick={() => (showGroupsSection = !showGroupsSection)}
                  >
                    {showGroupsSection ? m.ui_copy_hide_5290eebc() : m.ui_copy_assign_groups_6067d0c9()}
                  </button>
                </div>

                {#if showGroupsSection}
                  <div class="mt-2 p-3 rounded-lg border border-border/60 bg-card">
                    <GroupSelector
                      {orgId}
                      bind:selectedGroupIds
                      maxHeight="max-h-40"
                    />
                  </div>
                {:else if selectedGroupIds.length > 0}
                  <div class="flex items-center gap-2 mt-1.5">
                    <Badge variant="secondary" class="text-[10px]">
                      {selectedGroupIds.length} {m.ui_copy_groups_selected_dded0212()}
                    </Badge>
                  </div>
                {/if}
              </div>
            </AccordionContent>
          </AccordionItem>
        </Accordion>
      </div>

      <!-- Form Actions Footer (DiscountTemplate style) -->
      <div class="px-6 py-3 border-t border-border/40 flex gap-2 shrink-0 bg-background/50">
        <Button
          type="button"
          variant="ghost"
          onclick={() => (open = false)}
          disabled={isSubmitting}
          class="flex-1 h-9 text-sm cursor-pointer"
        >
          {m.common_action_cancel()}
        </Button>
        <Button
          type="submit"
          disabled={isSubmitting || !username.trim()}
          class="flex-1 h-9 text-sm font-semibold cursor-pointer shadow-xs gap-1.5"
        >
          {#if isSubmitting}
            <Refresh size={14} class="animate-spin" />
            <span>{m.common_state_saving()}</span>
          {:else}
            <Check size={14} />
            <span>{m.ui_copy_provision_user_3502e985()}</span>
          {/if}
        </Button>
      </div>
    </form>
  </Sheet.Content>
</Sheet.Root>
