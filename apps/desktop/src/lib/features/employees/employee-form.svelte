<!--
  @component EmployeeForm
  Comprehensive multi-tab form for Registering and Editing Employee dossiers.
  Integrates Unified Identity Provisioning, User Group assignment, and CSPRNG credential delivery.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { goto } from "$app/navigation";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Textarea } from "@orbis/ui/textarea";
import { Badge } from "@orbis/ui/badge";
import { Switch } from "@orbis/ui/switch";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui/select";
import {
  ArrowPath,
  BookOpen,
  BuildingOffice,
  CheckCircle,
  Eye,
  EyeSlash,
  Keyframes,
  LockClosed,
  Plus,
  ShieldCheck,
  User,
  UserBadgeCheck,
  UserGroup,
  Users,
  XMark,
} from "@orbis/icons";
import { toast } from "@orbis/ui";
import { localizeHref } from "$lib/paraglide/runtime";
import { formatNumber, l } from "$lib/core/i18n";
import { extractErrorMessage } from "$lib/api";
import type {
  EmployeeDetailDto,
  RegisterEmployeeRequest,
  UpdateEmployeeRequest,
  AccountProvisioningResultDto,
} from "$lib/api/types";
import {
  createRegisterEmployeeMutation,
  createUpdateEmployeeMutation,
} from "./queries";
import { createGroupsQuery } from "$lib/features/groups/queries";
import CredentialDeliveryModal from "./components/credential-delivery-modal.svelte";
import SettingsBottomBar from "$lib/shared/components/settings-bottom-bar.svelte";

interface Props {
  orgId: string;
  mode?: "create" | "edit";
  employee?: EmployeeDetailDto | null;
}

let { orgId, mode = "create", employee = null }: Props = $props();

// Active Form Tab
let activeTab = $state<"personal" | "employment" | "account">("personal");

// Form State
let firstName = $state(employee?.first_name || "");
let middleName = $state(employee?.middle_name || "");
let lastName = $state(employee?.last_name || "");
let firstNameAr = $state(employee?.first_name_ar || "");
let middleNameAr = $state(employee?.middle_name_ar || "");
let lastNameAr = $state(employee?.last_name_ar || "");
let nationalId = $state(employee?.national_id || "");
let passportNumber = $state(employee?.passport_number || "");
let gender = $state(employee?.gender || "male");
let birthDate = $state(employee?.birth_date || "");
let nationality = $state(employee?.nationality || "Saudi");
let bloodType = $state(employee?.blood_type || "");
let phone = $state(employee?.phone || "");
let email = $state(employee?.email || "");
let address = $state(employee?.address || "");

let employeeNumber = $state(employee?.employee_number || "");
let employeeType = $state(employee?.employee_type || "Teacher");
let jobTitle = $state(employee?.job_title || "");
let department = $state(employee?.department || "");
let orgUnitId = $state(employee?.org_unit_id || "");
let hireDate = $state(employee?.hire_date || new Date().toISOString().split("T")[0]);
let employmentType = $state(employee?.employment_type || "FullTime");
let workShift = $state(employee?.work_shift || "Morning");
let status = $state(employee?.status || "active");
let notes = $state(employee?.notes || "");

// Identity Provisioning State (Create Mode)
let enableAccount = $state(mode === "create");
let preferredUsername = $state("");
let deliveryChannel = $state<"manual" | "email" | "sms" | "whatsapp" | "auto">("manual");
let selectedGroupIds = $state<string[]>([]);
let customPassword = $state("");
let showCustomPassword = $state(false);

// Queries & Mutations
const groupsQuery = createGroupsQuery(() => orgId);
const availableGroups = $derived(groupsQuery.data ?? []);

const registerMutation = createRegisterEmployeeMutation();
const updateMutation = createUpdateEmployeeMutation();

const isSaving = $derived(registerMutation.isPending || updateMutation.isPending);

// Credential modal state
let credentialModalOpen = $state(false);
let provisioningResult = $state<AccountProvisioningResultDto | null>(null);
let registeredEmployeeName = $state("");
let registeredEmployeeId = $state<string | null>(null);

let isDirty = $state(false);

function markDirty() {
  isDirty = true;
}

function toggleGroupSelection(groupId: string) {
  markDirty();
  if (selectedGroupIds.includes(groupId)) {
    selectedGroupIds = selectedGroupIds.filter((id) => id !== groupId);
  } else {
    selectedGroupIds = [...selectedGroupIds, groupId];
  }
}

async function handleSubmit() {
  if (!firstName.trim() && !firstNameAr.trim()) {
    toast.error(l("يرجى إدخال الاسم الأول على الأقل", "Please enter at least the first name"));
    activeTab = "personal";
    return;
  }

  if (mode === "create") {
    const payload: RegisterEmployeeRequest = {
      first_name: firstName.trim() || firstNameAr.trim(),
      middle_name: middleName.trim() || undefined,
      last_name: lastName.trim() || lastNameAr.trim() || "Staff",
      first_name_ar: firstNameAr.trim() || undefined,
      middle_name_ar: middleNameAr.trim() || undefined,
      last_name_ar: lastNameAr.trim() || undefined,
      national_id: nationalId.trim() || undefined,
      passport_number: passportNumber.trim() || undefined,
      gender: gender || undefined,
      birth_date: birthDate || undefined,
      nationality: nationality.trim() || undefined,
      blood_type: bloodType || undefined,
      phone: phone.trim() || undefined,
      email: email.trim() || undefined,
      address: address.trim() || undefined,

      employee_number: employeeNumber.trim() || undefined,
      employee_type: employeeType,
      job_title: jobTitle.trim() || undefined,
      department: department.trim() || undefined,
      org_unit_id: orgUnitId.trim() || undefined,
      hire_date: hireDate || undefined,
      status: status || "active",
      employment_type: employmentType,
      work_shift: workShift,
      notes: notes.trim() || undefined,

      account: enableAccount
        ? {
            enabled: true,
            preferred_username: preferredUsername.trim() || undefined,
            delivery_channel: deliveryChannel,
            group_ids: selectedGroupIds,
            custom_password: customPassword.trim() || undefined,
          }
        : null,
    };

    try {
      const response = await registerMutation.mutateAsync(payload);
      toast.success(m.employees_save_success?.() || "تم تسجيل الموظف بنجاح");
      isDirty = false;

      registeredEmployeeName = response.employee.full_name || response.employee.first_name;
      registeredEmployeeId = response.employee.id;

      if (response.provisioning) {
        provisioningResult = response.provisioning;
        credentialModalOpen = true;
      } else {
        goto(localizeHref(`/${orgId}/employees/${response.employee.id}`));
      }
    } catch (err) {
      toast.error(extractErrorMessage(err));
    }
  } else if (mode === "edit" && employee) {
    const payload: UpdateEmployeeRequest = {
      first_name: firstName.trim() || undefined,
      middle_name: middleName.trim() || undefined,
      last_name: lastName.trim() || undefined,
      first_name_ar: firstNameAr.trim() || undefined,
      middle_name_ar: middleNameAr.trim() || undefined,
      last_name_ar: lastNameAr.trim() || undefined,
      national_id: nationalId.trim() || undefined,
      passport_number: passportNumber.trim() || undefined,
      gender: gender || undefined,
      birth_date: birthDate || undefined,
      nationality: nationality.trim() || undefined,
      blood_type: bloodType || undefined,
      phone: phone.trim() || undefined,
      email: email.trim() || undefined,
      address: address.trim() || undefined,

      employee_type: employeeType || undefined,
      job_title: jobTitle.trim() || undefined,
      department: department.trim() || undefined,
      org_unit_id: orgUnitId.trim() || undefined,
      hire_date: hireDate || undefined,
      status: status || undefined,
      employment_type: employmentType || undefined,
      work_shift: workShift || undefined,
      notes: notes.trim() || undefined,
    };

    try {
      await updateMutation.mutateAsync({ id: employee.id, req: payload });
      toast.success(m.employees_save_success?.() || "تم حفظ بيانات الموظف بنجاح");
      isDirty = false;
      goto(localizeHref(`/${orgId}/employees/${employee.id}`));
    } catch (err) {
      toast.error(extractErrorMessage(err));
    }
  }
}

function handleCredentialModalClose() {
  credentialModalOpen = false;
  if (registeredEmployeeId) {
    goto(localizeHref(`/${orgId}/employees/${registeredEmployeeId}`));
  } else {
    goto(localizeHref(`/${orgId}/employees`));
  }
}
</script>

<svelte:window
  onkeydown={(e) => {
    if ((e.ctrlKey || e.metaKey) && e.key === "s") {
      e.preventDefault();
      handleSubmit();
    }
  }}
/>

<div class="h-full w-full flex flex-col overflow-hidden bg-background">
  <!-- Top Navigation Ribbon -->
  <div class="border-b border-border/80 bg-card px-6 py-4 flex items-center justify-between shrink-0 shadow-2xs">
    <div class="flex items-center gap-3">
      <div class="flex size-10 shrink-0 items-center justify-center rounded-xl bg-primary/10 text-primary">
        <UserGroup size={20} />
      </div>
      <div class="flex flex-col">
        <h1 class="text-base font-bold text-foreground">
          {mode === "create" ? (m.employees_new_title?.() || "تسجيل موظف جديد") : (m.employees_edit_title?.() || "تعديل ملف الموظف")}
        </h1>
        <span class="text-xs text-muted-foreground">
          {mode === "create" ? l("إدخال السجل الشخصي والوظيفي وتهيئة بيانات الدخول الموحدة", "Enter personal, employment, and unified identity records.") : (employee?.full_name || employee?.first_name || "")}
        </span>
      </div>
    </div>

    <!-- Navigation Tabs -->
    <div class="flex items-center gap-1 bg-muted/60 p-1 rounded-lg border border-border/60">
      <button
        type="button"
        class="px-3 py-1.5 text-xs font-medium rounded-md transition-all cursor-pointer flex items-center gap-1.5 {activeTab === 'personal' ? 'bg-card text-foreground shadow-2xs font-semibold' : 'text-muted-foreground hover:text-foreground'}"
        onclick={() => (activeTab = "personal")}
      >
        <User size={13} />
        <span>{m.employees_tab_personal?.() || "البيانات الشخصية"}</span>
      </button>

      <button
        type="button"
        class="px-3 py-1.5 text-xs font-medium rounded-md transition-all cursor-pointer flex items-center gap-1.5 {activeTab === 'employment' ? 'bg-card text-foreground shadow-2xs font-semibold' : 'text-muted-foreground hover:text-foreground'}"
        onclick={() => (activeTab = "employment")}
      >
        <BuildingOffice size={13} />
        <span>{m.employees_tab_employment?.() || "البيانات الوظيفية"}</span>
      </button>

      {#if mode === "create"}
        <button
          type="button"
          class="px-3 py-1.5 text-xs font-medium rounded-md transition-all cursor-pointer flex items-center gap-1.5 {activeTab === 'account' ? 'bg-card text-foreground shadow-2xs font-semibold' : 'text-muted-foreground hover:text-foreground'}"
          onclick={() => (activeTab = "account")}
        >
          <ShieldCheck size={13} />
          <span>{m.employees_tab_account?.() || "حساب النظام والأمان"}</span>
          {#if enableAccount}
            <span class="size-1.5 rounded-full bg-emerald-500"></span>
          {/if}
        </button>
      {/if}
    </div>
  </div>

  <!-- Form Body -->
  <div class="flex-1 min-h-0 overflow-y-auto p-6">
    <div class="max-w-4xl mx-auto flex flex-col gap-6 pb-20">

      <!-- TAB 1: PERSONAL INFORMATION -->
      {#if activeTab === "personal"}
        <div class="flex flex-col gap-5 rounded-xl border border-border bg-card p-6 shadow-xs">
          <div class="flex items-center gap-2 border-b border-border/60 pb-3">
            <User size={16} class="text-primary" />
            <h2 class="text-sm font-bold text-foreground">{m.employees_tab_personal?.() || "البيانات الشخصية والهوية"}</h2>
          </div>

          <!-- Names Grid (Bilingual) -->
          <div class="grid grid-cols-3 gap-4">
            <div class="flex flex-col gap-1.5">
              <Label class="text-xs font-medium">{m.employees_first_name_ar?.() || "الاسم الأول (بالعربية)"}</Label>
              <Input
                type="text"
                placeholder="محمد"
                class="text-xs"
                bind:value={firstNameAr}
                oninput={markDirty}
              />
            </div>
            <div class="flex flex-col gap-1.5">
              <Label class="text-xs font-medium">{m.employees_middle_name_ar?.() || "اسم الأب (بالعربية)"}</Label>
              <Input
                type="text"
                placeholder="عبدالله"
                class="text-xs"
                bind:value={middleNameAr}
                oninput={markDirty}
              />
            </div>
            <div class="flex flex-col gap-1.5">
              <Label class="text-xs font-medium">{m.employees_last_name_ar?.() || "اسم العائلة (بالعربية)"}</Label>
              <Input
                type="text"
                placeholder="الغامدي"
                class="text-xs"
                bind:value={lastNameAr}
                oninput={markDirty}
              />
            </div>
          </div>

          <div class="grid grid-cols-3 gap-4">
            <div class="flex flex-col gap-1.5">
              <Label class="text-xs font-medium">{m.employees_first_name?.() || "First Name (Latin)"}</Label>
              <Input
                type="text"
                placeholder="Mohammed"
                class="text-xs"
                bind:value={firstName}
                oninput={markDirty}
              />
            </div>
            <div class="flex flex-col gap-1.5">
              <Label class="text-xs font-medium">{m.employees_middle_name?.() || "Middle Name"}</Label>
              <Input
                type="text"
                placeholder="Abdullah"
                class="text-xs"
                bind:value={middleName}
                oninput={markDirty}
              />
            </div>
            <div class="flex flex-col gap-1.5">
              <Label class="text-xs font-medium">{m.employees_last_name?.() || "Last Name"}</Label>
              <Input
                type="text"
                placeholder="Al-Ghamdi"
                class="text-xs"
                bind:value={lastName}
                oninput={markDirty}
              />
            </div>
          </div>

          <!-- Identification & Demographics -->
          <div class="grid grid-cols-4 gap-4 pt-2">
            <div class="flex flex-col gap-1.5">
              <Label class="text-xs font-medium">{m.employees_national_id?.() || "الهوية الوطنية / الإقامة"}</Label>
              <Input
                type="text"
                placeholder="10XXXXXXXX"
                class="text-xs font-mono"
                bind:value={nationalId}
                oninput={markDirty}
              />
            </div>

            <div class="flex flex-col gap-1.5">
              <Label class="text-xs font-medium">{m.employees_passport_number?.() || "رقم جواز السفر"}</Label>
              <Input
                type="text"
                placeholder="A1234567"
                class="text-xs font-mono"
                bind:value={passportNumber}
                oninput={markDirty}
              />
            </div>

            <div class="flex flex-col gap-1.5">
              <Label class="text-xs font-medium">{m.employees_gender?.() || "الجنس"}</Label>
              <Select type="single" value={gender} onValueChange={(v) => { if (v) { gender = v; markDirty(); } }}>
                <SelectTrigger class="text-xs h-9">
                  {gender === "male" ? (m.employees_gender_male?.() || "ذكر") : (m.employees_gender_female?.() || "أنثى")}
                </SelectTrigger>
                <SelectContent class="text-xs font-sans">
                  <SelectItem value="male">{m.employees_gender_male?.() || "ذكر"}</SelectItem>
                  <SelectItem value="female">{m.employees_gender_female?.() || "أنثى"}</SelectItem>
                </SelectContent>
              </Select>
            </div>

            <div class="flex flex-col gap-1.5">
              <Label class="text-xs font-medium">{m.employees_birth_date?.() || "تاريخ الميلاد"}</Label>
              <Input
                type="date"
                class="text-xs font-mono"
                bind:value={birthDate}
                oninput={markDirty}
              />
            </div>
          </div>

          <!-- Contact & Extra Info -->
          <div class="grid grid-cols-3 gap-4 pt-2">
            <div class="flex flex-col gap-1.5">
              <Label class="text-xs font-medium">{m.employees_phone?.() || "رقم الهاتف"}</Label>
              <Input
                type="tel"
                placeholder="+966 5X XXX XXXX"
                class="text-xs font-mono"
                bind:value={phone}
                oninput={markDirty}
              />
            </div>

            <div class="flex flex-col gap-1.5">
              <Label class="text-xs font-medium">{m.employees_email?.() || "البريد الإلكتروني"}</Label>
              <Input
                type="email"
                placeholder="staff@orbis.edu"
                class="text-xs font-mono"
                bind:value={email}
                oninput={markDirty}
              />
            </div>

            <div class="flex flex-col gap-1.5">
              <Label class="text-xs font-medium">{m.employees_nationality?.() || "الجنسية"}</Label>
              <Input
                type="text"
                placeholder="Saudi"
                class="text-xs"
                bind:value={nationality}
                oninput={markDirty}
              />
            </div>
          </div>

          <div class="flex flex-col gap-1.5 pt-2">
            <Label class="text-xs font-medium">{m.employees_address?.() || "العنوان السكني"}</Label>
            <Input
              type="text"
              placeholder="الرياض، حي النخيل..."
              class="text-xs"
              bind:value={address}
              oninput={markDirty}
            />
          </div>
        </div>
      {/if}

      <!-- TAB 2: EMPLOYMENT & JOB -->
      {#if activeTab === "employment"}
        <div class="flex flex-col gap-5 rounded-xl border border-border bg-card p-6 shadow-xs">
          <div class="flex items-center gap-2 border-b border-border/60 pb-3">
            <BuildingOffice size={16} class="text-primary" />
            <h2 class="text-sm font-bold text-foreground">{m.employees_tab_employment?.() || "البيانات والتعيينات الوظيفية"}</h2>
          </div>

          <div class="grid grid-cols-3 gap-4">
            <div class="flex flex-col gap-1.5">
              <Label class="text-xs font-medium">{m.employees_employee_number?.() || "الرقم الوظيفي"}</Label>
              <Input
                type="text"
                placeholder="EMP-1001"
                class="text-xs font-mono"
                bind:value={employeeNumber}
                oninput={markDirty}
              />
            </div>

            <div class="flex flex-col gap-1.5">
              <Label class="text-xs font-medium">{m.employees_filter_type?.() || "فئة الموظف"}</Label>
              <Select type="single" value={employeeType} onValueChange={(v) => { if (v) { employeeType = v; markDirty(); } }}>
                <SelectTrigger class="text-xs h-9">
                  {employeeType}
                </SelectTrigger>
                <SelectContent class="text-xs font-sans">
                  <SelectItem value="Teacher">{m.employees_type_teacher?.() || "معلم"}</SelectItem>
                  <SelectItem value="Admin">{m.employees_type_admin?.() || "إداري"}</SelectItem>
                  <SelectItem value="Specialist">{m.employees_type_specialist?.() || "أخصائي"}</SelectItem>
                  <SelectItem value="Support">{m.employees_type_support?.() || "خدمات مساندة"}</SelectItem>
                </SelectContent>
              </Select>
            </div>

            <div class="flex flex-col gap-1.5">
              <Label class="text-xs font-medium">{m.employees_job_title?.() || "المسمى الوظيفي"}</Label>
              <Input
                type="text"
                placeholder="معلم رياضيات أول"
                class="text-xs"
                bind:value={jobTitle}
                oninput={markDirty}
              />
            </div>
          </div>

          <div class="grid grid-cols-3 gap-4">
            <div class="flex flex-col gap-1.5">
              <Label class="text-xs font-medium">{m.employees_department?.() || "القسم / الإدارة"}</Label>
              <Input
                type="text"
                placeholder="قسم العلوم والرياضيات"
                class="text-xs"
                bind:value={department}
                oninput={markDirty}
              />
            </div>

            <div class="flex flex-col gap-1.5">
              <Label class="text-xs font-medium">{m.employees_hire_date?.() || "تاريخ التعيين"}</Label>
              <Input
                type="date"
                class="text-xs font-mono"
                bind:value={hireDate}
                oninput={markDirty}
              />
            </div>

            <div class="flex flex-col gap-1.5">
              <Label class="text-xs font-medium">{m.employees_filter_status?.() || "الحالة الوظيفية"}</Label>
              <Select type="single" value={status} onValueChange={(v) => { if (v) { status = v; markDirty(); } }}>
                <SelectTrigger class="text-xs h-9">
                  {status}
                </SelectTrigger>
                <SelectContent class="text-xs font-sans">
                  <SelectItem value="active">{m.employees_status_active?.() || "نشط"}</SelectItem>
                  <SelectItem value="on_leave">{m.employees_status_on_leave?.() || "إجازة"}</SelectItem>
                  <SelectItem value="suspended">{m.employees_status_suspended?.() || "موقوف"}</SelectItem>
                  <SelectItem value="terminated">{m.employees_status_terminated?.() || "منتهي الخدمة"}</SelectItem>
                </SelectContent>
              </Select>
            </div>
          </div>

          <div class="grid grid-cols-2 gap-4">
            <div class="flex flex-col gap-1.5">
              <Label class="text-xs font-medium">{m.employees_employment_type?.() || "نوع العقد"}</Label>
              <Select type="single" value={employmentType} onValueChange={(v) => { if (v) { employmentType = v; markDirty(); } }}>
                <SelectTrigger class="text-xs h-9">
                  {employmentType}
                </SelectTrigger>
                <SelectContent class="text-xs font-sans">
                  <SelectItem value="FullTime">{l("دوام كامل (Full Time)", "Full Time")}</SelectItem>
                  <SelectItem value="PartTime">{l("دوام جزئي (Part Time)", "Part Time")}</SelectItem>
                  <SelectItem value="Contract">{l("عقد محدد (Contract)", "Contract")}</SelectItem>
                  <SelectItem value="Temporary">{l("مؤقت (Temporary)", "Temporary")}</SelectItem>
                </SelectContent>
              </Select>
            </div>

            <div class="flex flex-col gap-1.5">
              <Label class="text-xs font-medium">{m.employees_work_shift?.() || "فترة العمل"}</Label>
              <Select type="single" value={workShift} onValueChange={(v) => { if (v) { workShift = v; markDirty(); } }}>
                <SelectTrigger class="text-xs h-9">
                  {workShift}
                </SelectTrigger>
                <SelectContent class="text-xs font-sans">
                  <SelectItem value="Morning">{l("فترة صباحية", "Morning")}</SelectItem>
                  <SelectItem value="Evening">{l("فترة مسائية", "Evening")}</SelectItem>
                  <SelectItem value="Rotating">{l("فترات مناوبة", "Rotating")}</SelectItem>
                </SelectContent>
              </Select>
            </div>
          </div>

          <div class="flex flex-col gap-1.5 pt-2">
            <Label class="text-xs font-medium">{m.employees_tab_notes?.() || "ملاحظات إدارية"}</Label>
            <Textarea
              placeholder={l("ملاحظات إضافية حول الموظف أو العقد...", "Additional notes...")}
              class="text-xs min-h-20"
              bind:value={notes}
              oninput={markDirty}
            />
          </div>
        </div>
      {/if}

      <!-- TAB 3: IDENTITY PROVISIONING & USER GROUPS (CREATE MODE) -->
      {#if activeTab === "account" && mode === "create"}
        <div class="flex flex-col gap-5 rounded-xl border border-border bg-card p-6 shadow-xs">
          <div class="flex items-center justify-between border-b border-border/60 pb-3">
            <div class="flex items-center gap-2">
              <ShieldCheck size={16} class="text-primary" />
              <div class="flex flex-col">
                <h2 class="text-sm font-bold text-foreground">{m.employees_account_provision_switch?.() || "إنشاء حساب مستخدم وربطه تلقائياً"}</h2>
                <span class="text-xs text-muted-foreground">{m.employees_account_provision_desc?.() || "ينشئ حساب مستخدم مع بيانات اعتماد آمنة لمرة واحدة"}</span>
              </div>
            </div>

            <Switch
              checked={enableAccount}
              onCheckedChange={(c) => {
                enableAccount = c;
                markDirty();
              }}
            />
          </div>

          {#if enableAccount}
            <div class="flex flex-col gap-5 pt-2">
              <div class="grid grid-cols-2 gap-4">
                <div class="flex flex-col gap-1.5">
                  <Label class="text-xs font-medium">{l("اسم المستخدم المفضل (اختياري)", "Preferred Username (Optional)")}</Label>
                  <Input
                    type="text"
                    placeholder="e.g. m.ghamdi"
                    class="text-xs font-mono"
                    bind:value={preferredUsername}
                    oninput={markDirty}
                  />
                  <span class="text-[10px] text-muted-foreground">{l("في حال تركه فارغاً، سيتم توليده تلقائياً من البريد أو الاسم والرقم الوظيفي.", "Auto-generated from email/name if left empty.")}</span>
                </div>

                <div class="flex flex-col gap-1.5">
                  <Label class="text-xs font-medium">{m.employees_delivery_channel?.() || "طريقة تسليم بيانات الدخول"}</Label>
                  <Select type="single" value={deliveryChannel} onValueChange={(v) => { if (v) { deliveryChannel = v as any; markDirty(); } }}>
                    <SelectTrigger class="text-xs h-9">
                      {deliveryChannel}
                    </SelectTrigger>
                    <SelectContent class="text-xs font-sans">
                      <SelectItem value="manual">{m.employees_delivery_manual?.() || "عرض على الشاشة لمرة واحدة (يدوي)"}</SelectItem>
                      <SelectItem value="email">{m.employees_delivery_email?.() || "إرسال عبر البريد الإلكتروني"}</SelectItem>
                      <SelectItem value="sms">{m.employees_delivery_sms?.() || "إرسال عبر الرسائل القصيرة"}</SelectItem>
                      <SelectItem value="whatsapp">{m.employees_delivery_whatsapp?.() || "إرسال عبر واتساب"}</SelectItem>
                      <SelectItem value="auto">{m.employees_delivery_auto?.() || "الوضع التلقائي للنظام"}</SelectItem>
                    </SelectContent>
                  </Select>
                </div>
              </div>

              <!-- Assigned User Groups Selection -->
              <div class="flex flex-col gap-2.5 pt-2">
                <div class="flex items-center justify-between">
                  <Label class="text-xs font-bold text-foreground flex items-center gap-1.5">
                    <UserGroup size={14} class="text-primary" />
                    {m.employees_user_groups?.() || "مجموعات المستخدمين المسندة"}
                  </Label>
                  <span class="text-xs text-muted-foreground">
                    {formatNumber(selectedGroupIds.length)} {l("مجموعة محددة", "selected")}
                  </span>
                </div>
                <span class="text-[11px] text-muted-foreground">
                  {m.employees_user_groups_desc?.() || "حدد المجموعات لمنح الصلاحيات والأدوار للموظف تلقائياً."}
                </span>

                {#if groupsQuery.isLoading}
                  <div class="p-4 text-center text-xs text-muted-foreground flex items-center justify-center gap-2">
                    <ArrowPath size={14} class="animate-spin text-primary" />
                    <span>{l("جاري تحميل مجموعات المستخدمين...", "Loading groups...")}</span>
                  </div>
                {:else if availableGroups.length === 0}
                  <div class="p-4 rounded-lg bg-muted/40 text-center text-xs text-muted-foreground">
                    {l("لا توجد مجموعات معرفة حالياً. يمكنك إنشاء مجموعات من إعدادات النظام.", "No user groups available.")}
                  </div>
                {:else}
                  <div class="grid grid-cols-2 gap-3 max-h-56 overflow-y-auto p-1 border border-border/60 rounded-lg bg-muted/20">
                    {#each availableGroups as group (group.id)}
                      {@const isSelected = selectedGroupIds.includes(group.id)}
                      <button
                        type="button"
                        class="flex items-start gap-2.5 p-3 rounded-lg border text-start transition-all cursor-pointer {isSelected ? 'border-primary bg-primary/10 text-foreground shadow-2xs' : 'border-border/60 bg-card text-muted-foreground hover:border-border'}"
                        onclick={() => toggleGroupSelection(group.id)}
                      >
                        <div class="flex size-6 shrink-0 items-center justify-center rounded-md {isSelected ? 'bg-primary text-primary-foreground' : 'bg-muted text-muted-foreground'}">
                          <CheckCircle size={13} />
                        </div>
                        <div class="flex flex-col min-w-0">
                          <span class="text-xs font-semibold text-foreground truncate">
                            {group.name_ar || group.name}
                          </span>
                          {#if group.description}
                            <span class="text-[10px] text-muted-foreground line-clamp-1">
                              {group.description}
                            </span>
                          {/if}
                        </div>
                      </button>
                    {/each}
                  </div>
                {/if}
              </div>
            </div>
          {/if}
        </div>
      {/if}

    </div>
  </div>

  <!-- Bottom Floating Action Bar -->
  <SettingsBottomBar
    visible={true}
    isSaving={isSaving}
    isDirty={isDirty}
    saveLabel={mode === "create" ? (m.employees_new_title?.() || "تسجيل الموظف") : (m.settings_action_save_changes?.() || "حفظ التعديلات")}
    onsave={handleSubmit}
    onreset={() => goto(localizeHref(`/${orgId}/employees`))}
    resetLabel={m.common_action_cancel?.() || "إلغاء"}
  />

  <!-- Credential Delivery Modal (CSPRNG temporary password presentation) -->
  <CredentialDeliveryModal
    bind:open={credentialModalOpen}
    provisioning={provisioningResult}
    employeeName={registeredEmployeeName}
    onclose={handleCredentialModalClose}
  />
</div>
