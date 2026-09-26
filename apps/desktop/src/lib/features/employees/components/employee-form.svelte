<!--
  @component EmployeeForm
  Canonical Slide-over Form for Creating, Editing, and Previewing Employee dossiers.
  Follows the exact FeeTemplateForm architecture with FormController and zero wrapper divs.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { l } from "$lib/core/i18n";
import { api, extractErrorMessage } from "$lib/api";
import { FormController } from "$lib/shared/forms/form-controller.svelte";
import { FormField } from "@orbis/ui";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Textarea } from "@orbis/ui/textarea";
import { Badge } from "@orbis/ui/badge";
import { Switch } from "@orbis/ui/switch";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import * as Sheet from "@orbis/ui/sheet";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@orbis/ui/accordion";
import { toast } from "@orbis/ui";
import {
  ArrowPath,
  BuildingOffice,
  ShieldCheck,
  User,
} from "@orbis/icons";
import { z } from "zod";
import type {
  EmployeeDetailDto,
  RegisterEmployeeRequest,
  UpdateEmployeeRequest,
  AccountProvisioningResultDto,
} from "$lib/api/types";
import NationalitySelect from "$lib/shared/components/nationality-select.svelte";
import { GroupSelector } from "$lib/features/groups";

interface Props {
  orgId: string;
  mode?: "preview" | "create" | "edit";
  employee?: EmployeeDetailDto | null;
  onSuccess: (emp: EmployeeDetailDto, prov?: AccountProvisioningResultDto | null) => void;
  onCancel: () => void;
}

let {
  orgId,
  mode = "create",
  employee = null,
  onSuccess,
  onCancel,
}: Props = $props();

const employeeFormSchema = z.object({
  first_name: z.string().trim().min(1, "الاسم الأول مطلوب / First name is required"),
  middle_name: z.string().trim().nullable().optional(),
  last_name: z.string().trim().min(1, "الاسم الأخير مطلوب / Last name is required"),
  first_name_ar: z.string().trim().nullable().optional(),
  middle_name_ar: z.string().trim().nullable().optional(),
  last_name_ar: z.string().trim().nullable().optional(),
  national_id: z.string().trim().nullable().optional(),
  passport_number: z.string().trim().nullable().optional(),
  gender: z.string().nullable().optional(),
  birth_date: z.string().nullable().optional(),
  nationality: z.string().nullable().optional(),
  blood_type: z.string().nullable().optional(),
  phone: z.string().trim().nullable().optional(),
  email: z.string().trim().email("بريد إلكتروني غير صالح").nullable().optional().or(z.literal("")),
  address: z.string().trim().nullable().optional(),

  employee_number: z.string().trim().nullable().optional(),
  employee_type: z.string().default("Teacher"),
  job_title: z.string().trim().nullable().optional(),
  department: z.string().trim().nullable().optional(),
  org_unit_id: z.string().trim().nullable().optional(),
  hire_date: z.string().nullable().optional(),
  status: z.string().default("active"),
  employment_type: z.string().default("FullTime"),
  work_shift: z.string().default("Morning"),
  notes: z.string().trim().nullable().optional(),

  enable_account: z.boolean().default(false),
  preferred_username: z.string().trim().nullable().optional(),
  selected_group_ids: z.array(z.string()).default([]),
  custom_password: z.string().nullable().optional(),
});

type EmployeeFormValues = z.infer<typeof employeeFormSchema>;

// Form state using canonical FormController
const form = new FormController<EmployeeFormValues>({
  initialValues: {
    first_name: employee?.first_name || "",
    middle_name: employee?.middle_name || "",
    last_name: employee?.last_name || "",
    first_name_ar: employee?.first_name_ar || "",
    middle_name_ar: employee?.middle_name_ar || "",
    last_name_ar: employee?.last_name_ar || "",
    national_id: employee?.national_id || "",
    passport_number: employee?.passport_number || "",
    gender: employee?.gender || "male",
    birth_date: employee?.birth_date || "",
    nationality: employee?.nationality || "Saudi",
    blood_type: employee?.blood_type || "",
    phone: employee?.phone || "",
    email: employee?.email || "",
    address: employee?.address || "",

    employee_number: employee?.employee_number || "",
    employee_type: employee?.employee_type || "Teacher",
    job_title: employee?.job_title || "",
    department: employee?.department || "",
    org_unit_id: employee?.org_unit_id || "",
    hire_date: employee?.hire_date || new Date().toISOString().split("T")[0],
    status: employee?.status || "active",
    employment_type: employee?.employment_type || "FullTime",
    work_shift: employee?.work_shift || "Morning",
    notes: employee?.notes || "",

    enable_account: mode === "create",
    preferred_username: "",
    selected_group_ids: [],
    custom_password: "",
  },
  schema: employeeFormSchema,
});

function getEmployeeTypeLabel(type: string): string {
  switch (type) {
    case "Teacher":
      return m.employees_type_teacher?.() || "معلم / كادر تعليمي";
    case "Admin":
      return m.employees_type_admin?.() || "إداري";
    case "Support":
      return m.employees_type_support?.() || "خدمات مساندة";
    case "Management":
      return l("إدارة عليا", "Management");
    case "Worker":
      return l("عامل / خدمات", "Worker");
    default:
      return type;
  }
}

function getContractTypeLabel(type: string): string {
  switch (type) {
    case "FullTime":
      return l("دوام كامل", "Full-time");
    case "PartTime":
      return l("دوام جزئي", "Part-time");
    case "Contractor":
      return l("عقد محدد", "Contractor");
    case "Temporary":
      return l("مؤقت / موسمي", "Temporary");
    default:
      return type;
  }
}

function getStatusLabel(status: string): string {
  switch (status) {
    case "active":
      return m.employees_status_active?.() || "على رأس العمل";
    case "on_leave":
      return m.employees_status_on_leave?.() || "في إجازة";
    case "suspended":
      return m.employees_status_suspended?.() || "موقوف مؤقتاً";
    case "terminated":
      return m.employees_status_terminated?.() || "منتهي الخدمة";
    default:
      return status;
  }
}

async function handleSubmit() {
  if (mode === "preview") {
    onCancel();
    return;
  }

  const success = await form.submit(async (values) => {
    if (mode === "create") {
      const payload: RegisterEmployeeRequest = {
        first_name: values.first_name.trim(),
        middle_name: values.middle_name?.trim() || undefined,
        last_name: values.last_name.trim(),
        first_name_ar: values.first_name_ar?.trim() || undefined,
        middle_name_ar: values.middle_name_ar?.trim() || undefined,
        last_name_ar: values.last_name_ar?.trim() || undefined,
        national_id: values.national_id?.trim() || undefined,
        passport_number: values.passport_number?.trim() || undefined,
        gender: values.gender || undefined,
        birth_date: values.birth_date || undefined,
        nationality: values.nationality?.trim() || undefined,
        blood_type: values.blood_type?.trim() || undefined,
        phone: values.phone?.trim() || undefined,
        email: values.email?.trim() || undefined,
        address: values.address?.trim() || undefined,

        employee_number: values.employee_number?.trim() || undefined,
        employee_type: values.employee_type,
        job_title: values.job_title?.trim() || undefined,
        department: values.department?.trim() || undefined,
        org_unit_id: values.org_unit_id?.trim() || undefined,
        hire_date: values.hire_date || undefined,
        status: values.status || "active",
        employment_type: values.employment_type,
        work_shift: values.work_shift,
        notes: values.notes?.trim() || undefined,

        account: values.enable_account
          ? {
              enabled: true,
              preferred_username: values.preferred_username?.trim() || undefined,
              delivery_channel: "auto",
              group_ids: values.selected_group_ids,
              custom_password: values.custom_password?.trim() || undefined,
            }
          : null,
      };

      const res = await api.registerEmployee(payload);
      toast.success(m.employees_save_success?.() || "تم تسجيل الموظف بنجاح");
      onSuccess(res.employee, res.provisioning);
    } else if (mode === "edit" && employee) {
      const payload: UpdateEmployeeRequest = {
        first_name: values.first_name.trim() || undefined,
        middle_name: values.middle_name?.trim() || undefined,
        last_name: values.last_name.trim() || undefined,
        first_name_ar: values.first_name_ar?.trim() || undefined,
        middle_name_ar: values.middle_name_ar?.trim() || undefined,
        last_name_ar: values.last_name_ar?.trim() || undefined,
        national_id: values.national_id?.trim() || undefined,
        passport_number: values.passport_number?.trim() || undefined,
        gender: values.gender || undefined,
        birth_date: values.birth_date || undefined,
        nationality: values.nationality?.trim() || undefined,
        blood_type: values.blood_type?.trim() || undefined,
        phone: values.phone?.trim() || undefined,
        email: values.email?.trim() || undefined,
        address: values.address?.trim() || undefined,

        employee_type: values.employee_type || undefined,
        job_title: values.job_title?.trim() || undefined,
        department: values.department?.trim() || undefined,
        org_unit_id: values.org_unit_id?.trim() || undefined,
        hire_date: values.hire_date || undefined,
        status: values.status || undefined,
        employment_type: values.employment_type || undefined,
        work_shift: values.work_shift || undefined,
        notes: values.notes?.trim() || undefined,
      };

      const res = await api.updateEmployee(employee.id, payload);
      toast.success(m.employees_save_success?.() || "تم حفظ بيانات الموظف بنجاح");
      onSuccess(res, null);
    }
  });

  if (!success && form.formError) {
    toast.error(form.formError);
  }
}
</script>

<form
  onsubmit={(e) => {
    e.preventDefault();
    handleSubmit();
  }}
  class="flex flex-col h-full overflow-hidden gap-0"
>
  <Sheet.Header class="px-6 py-4 border-b border-border/40 shrink-0 text-start">
    <Sheet.Title class="text-base font-bold text-foreground">
      {#if mode === "create"}
        {m.employees_new_title?.() || "تسجيل موظف جديد"}
      {:else if mode === "edit"}
        {m.employees_edit_title?.() || "تعديل ملف الموظف"}
      {:else}
        {m.employees_preview_title?.() || "عرض ملف الموظف"}
      {/if}
    </Sheet.Title>
    <Sheet.Description class="text-xs text-muted-foreground mt-0.5">
      {#if mode === "create"}
        {l("إدخال السجل الشخصي والوظيفي وتهيئة بيانات الدخول الموحدة", "Enter personal, employment, and unified identity records.")}
      {:else}
        {employee?.full_name || employee?.first_name || ""}
      {/if}
    </Sheet.Description>
  </Sheet.Header>

  <div class="flex-1 overflow-y-auto p-6 scrollbar-hide">
    <Accordion
      type="multiple"
      value={["personal", "employment", ...(mode === "create" ? ["account"] : [])]}
      class="flex flex-col gap-4"
    >
      <!-- Accordion Item 1: Personal Information -->
      <AccordionItem value="personal" class="rounded-lg border border-border/60 bg-card px-4 py-1">
        <AccordionTrigger class="text-sm font-semibold hover:no-underline py-3">
          <div class="flex items-center gap-2">
            <User size={16} class="text-primary" />
            <span>{m.employees_tab_personal?.() || "البيانات الشخصية والهوية"}</span>
          </div>
        </AccordionTrigger>
        <AccordionContent class="pt-2 pb-4 flex flex-col gap-4">
          <!-- Arabic Names -->
          <div class="grid grid-cols-1 sm:grid-cols-3 gap-3">
            <FormField id="emp_fn_ar" label={m.employees_first_name_ar?.() || "الاسم الأول (عربي)"} error={form.errors.first_name_ar?.[0]}>
              {#snippet children()}
                <Input
                  id="emp_fn_ar"
                  type="text"
                  placeholder="محمد"
                  bind:value={form.values.first_name_ar}
                  disabled={mode === "preview" || form.isSubmitting}
                />
              {/snippet}
            </FormField>

            <FormField id="emp_mn_ar" label={m.employees_middle_name_ar?.() || "اسم الأب (عربي)"} error={form.errors.middle_name_ar?.[0]}>
              {#snippet children()}
                <Input
                  id="emp_mn_ar"
                  type="text"
                  placeholder="عبدالله"
                  bind:value={form.values.middle_name_ar}
                  disabled={mode === "preview" || form.isSubmitting}
                />
              {/snippet}
            </FormField>

            <FormField id="emp_ln_ar" label={m.employees_last_name_ar?.() || "اسم العائلة (عربي)"} error={form.errors.last_name_ar?.[0]}>
              {#snippet children()}
                <Input
                  id="emp_ln_ar"
                  type="text"
                  placeholder="الغامدي"
                  bind:value={form.values.last_name_ar}
                  disabled={mode === "preview" || form.isSubmitting}
                />
              {/snippet}
            </FormField>
          </div>

          <!-- English/Latin Names -->
          <div class="grid grid-cols-1 sm:grid-cols-3 gap-3">
            <FormField id="emp_fn_en" label={m.common_first_name?.() || "الاسم الأول (إنجليزي)"} error={form.errors.first_name?.[0]} required>
              {#snippet children()}
                <Input
                  id="emp_fn_en"
                  type="text"
                  placeholder="Mohammed"
                  bind:value={form.values.first_name}
                  onblur={() => form.markTouched("first_name")}
                  required
                  disabled={mode === "preview" || form.isSubmitting}
                />
              {/snippet}
            </FormField>

            <FormField id="emp_mn_en" label={m.common_middle_name?.() || "اسم الأب (إنجليزي)"} error={form.errors.middle_name?.[0]}>
              {#snippet children()}
                <Input
                  id="emp_mn_en"
                  type="text"
                  placeholder="Abdullah"
                  bind:value={form.values.middle_name}
                  disabled={mode === "preview" || form.isSubmitting}
                />
              {/snippet}
            </FormField>

            <FormField id="emp_ln_en" label={m.common_last_name?.() || "اسم العائلة (إنجليزي)"} error={form.errors.last_name?.[0]} required>
              {#snippet children()}
                <Input
                  id="emp_ln_en"
                  type="text"
                  placeholder="Al-Ghamdi"
                  bind:value={form.values.last_name}
                  onblur={() => form.markTouched("last_name")}
                  required
                  disabled={mode === "preview" || form.isSubmitting}
                />
              {/snippet}
            </FormField>
          </div>

          <!-- IDs & Demographics Grid -->
          <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-3">
            <FormField id="emp_nat_id" label={m.employees_national_id?.() || "رقم الهوية الوطنية / الإقامة"} error={form.errors.national_id?.[0]}>
              {#snippet children()}
                <Input
                  id="emp_nat_id"
                  type="text"
                  placeholder="10XXXXXXXX"
                  class="font-mono"
                  bind:value={form.values.national_id}
                  disabled={mode === "preview" || form.isSubmitting}
                />
              {/snippet}
            </FormField>

            <FormField id="emp_passport" label={m.employees_passport_number?.() || "رقم جواز السفر"} error={form.errors.passport_number?.[0]}>
              {#snippet children()}
                <Input
                  id="emp_passport"
                  type="text"
                  placeholder="P0XXXXXX"
                  class="font-mono"
                  bind:value={form.values.passport_number}
                  disabled={mode === "preview" || form.isSubmitting}
                />
              {/snippet}
            </FormField>

            <FormField id="emp_gender" label={m.employees_gender?.() || "الجنس"}>
              {#snippet children()}
                <Select
                  type="single"
                  value={form.values.gender ?? "male"}
                  onValueChange={(val) => {
                    if (val) form.values.gender = val;
                  }}
                  disabled={mode === "preview" || form.isSubmitting}
                >
                  <SelectTrigger class="h-9 text-xs">
                    {form.values.gender === "female" ? (m.common_gender_female?.() || "أنثى") : (m.common_gender_male?.() || "ذكر")}
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="male">{m.common_gender_male?.() || "ذكر"}</SelectItem>
                    <SelectItem value="female">{m.common_gender_female?.() || "أنثى"}</SelectItem>
                  </SelectContent>
                </Select>
              {/snippet}
            </FormField>

            <FormField id="emp_dob" label={m.employees_birth_date?.() || "تاريخ الميلاد"}>
              {#snippet children()}
                <Input
                  id="emp_dob"
                  type="date"
                  bind:value={form.values.birth_date}
                  disabled={mode === "preview" || form.isSubmitting}
                />
              {/snippet}
            </FormField>
          </div>

          <!-- Nationality & Contact Grid -->
          <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
            <FormField id="emp_nat" label={m.employees_nationality?.() || "الجنسية"}>
              {#snippet children()}
                <NationalitySelect
                  bind:value={form.values.nationality}
                  disabled={mode === "preview" || form.isSubmitting}
                />
              {/snippet}
            </FormField>

            <FormField id="emp_phone" label={m.common_phone?.() || "رقم الهاتف"} error={form.errors.phone?.[0]}>
              {#snippet children()}
                <Input
                  id="emp_phone"
                  type="tel"
                  placeholder="+966 5X XXX XXXX"
                  class="font-mono"
                  bind:value={form.values.phone}
                  disabled={mode === "preview" || form.isSubmitting}
                />
              {/snippet}
            </FormField>

            <FormField id="emp_email" label={m.common_email?.() || "البريد الإلكتروني"} error={form.errors.email?.[0]}>
              {#snippet children()}
                <Input
                  id="emp_email"
                  type="email"
                  placeholder="teacher@school.edu.sa"
                  class="font-mono"
                  bind:value={form.values.email}
                  disabled={mode === "preview" || form.isSubmitting}
                />
              {/snippet}
            </FormField>
          </div>

          <FormField id="emp_addr" label={m.common_address?.() || "العنوان السكني"} error={form.errors.address?.[0]}>
            {#snippet children()}
              <Input
                id="emp_addr"
                type="text"
                placeholder="الرياض، حي النخيل"
                bind:value={form.values.address}
                disabled={mode === "preview" || form.isSubmitting}
              />
            {/snippet}
          </FormField>
        </AccordionContent>
      </AccordionItem>

      <!-- Accordion Item 2: Employment Information -->
      <AccordionItem value="employment" class="rounded-lg border border-border/60 bg-card px-4 py-1">
        <AccordionTrigger class="text-sm font-semibold hover:no-underline py-3">
          <div class="flex items-center gap-2">
            <BuildingOffice size={16} class="text-primary" />
            <span>{m.employees_tab_employment?.() || "البيانات الوظيفية والتعاقدية"}</span>
          </div>
        </AccordionTrigger>
        <AccordionContent class="pt-2 pb-4 flex flex-col gap-4">
          <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
            <FormField id="emp_num" label={m.employees_employee_number?.() || "الرقم الوظيفي"} error={form.errors.employee_number?.[0]}>
              {#snippet children()}
                <Input
                  id="emp_num"
                  type="text"
                  placeholder="EMP-00123"
                  class="font-mono"
                  bind:value={form.values.employee_number}
                  disabled={mode === "preview" || form.isSubmitting}
                />
              {/snippet}
            </FormField>

            <FormField id="emp_type" label={m.employees_filter_type?.() || "فئة الموظف"}>
              {#snippet children()}
                <Select
                  type="single"
                  value={form.values.employee_type}
                  onValueChange={(val) => {
                    if (val) form.values.employee_type = val;
                  }}
                  disabled={mode === "preview" || form.isSubmitting}
                >
                  <SelectTrigger class="h-9 text-xs">
                    {getEmployeeTypeLabel(form.values.employee_type)}
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="Teacher">{m.employees_type_teacher?.() || "معلم / كادر تعليمي"}</SelectItem>
                    <SelectItem value="Admin">{m.employees_type_admin?.() || "إداري"}</SelectItem>
                    <SelectItem value="Support">{m.employees_type_support?.() || "خدمات مساندة"}</SelectItem>
                    <SelectItem value="Management">{l("إدارة عليا", "Management")}</SelectItem>
                    <SelectItem value="Worker">{l("عامل / خدمات", "Worker")}</SelectItem>
                  </SelectContent>
                </Select>
              {/snippet}
            </FormField>

            <FormField id="emp_title" label={m.employees_job_title?.() || "المسمى الوظيفي"} error={form.errors.job_title?.[0]}>
              {#snippet children()}
                <Input
                  id="emp_title"
                  type="text"
                  placeholder="معلم رياضيات أول"
                  bind:value={form.values.job_title}
                  disabled={mode === "preview" || form.isSubmitting}
                />
              {/snippet}
            </FormField>
          </div>

          <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-3">
            <FormField id="emp_dept" label={m.employees_department?.() || "القسم / الإدارة"} error={form.errors.department?.[0]}>
              {#snippet children()}
                <Input
                  id="emp_dept"
                  type="text"
                  placeholder="قسم العلوم والرياضيات"
                  bind:value={form.values.department}
                  disabled={mode === "preview" || form.isSubmitting}
                />
              {/snippet}
            </FormField>

            <FormField id="emp_hire_date" label={m.employees_hire_date?.() || "تاريخ التعيين"}>
              {#snippet children()}
                <Input
                  id="emp_hire_date"
                  type="date"
                  bind:value={form.values.hire_date}
                  disabled={mode === "preview" || form.isSubmitting}
                />
              {/snippet}
            </FormField>

            <FormField id="emp_contract" label={m.employees_employment_type?.() || "نوع العقد"}>
              {#snippet children()}
                <Select
                  type="single"
                  value={form.values.employment_type}
                  onValueChange={(val) => {
                    if (val) form.values.employment_type = val;
                  }}
                  disabled={mode === "preview" || form.isSubmitting}
                >
                  <SelectTrigger class="h-9 text-xs">
                    {getContractTypeLabel(form.values.employment_type)}
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="FullTime">{l("دوام كامل", "Full-time")}</SelectItem>
                    <SelectItem value="PartTime">{l("دوام جزئي", "Part-time")}</SelectItem>
                    <SelectItem value="Contractor">{l("عقد محدد", "Contractor")}</SelectItem>
                    <SelectItem value="Temporary">{l("مؤقت / موسمي", "Temporary")}</SelectItem>
                  </SelectContent>
                </Select>
              {/snippet}
            </FormField>

            <FormField id="emp_status" label={m.employees_filter_status?.() || "الحالة"}>
              {#snippet children()}
                <Select
                  type="single"
                  value={form.values.status}
                  onValueChange={(val) => {
                    if (val) form.values.status = val;
                  }}
                  disabled={mode === "preview" || form.isSubmitting}
                >
                  <SelectTrigger class="h-9 text-xs">
                    {getStatusLabel(form.values.status)}
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="active">{m.employees_status_active?.() || "على رأس العمل"}</SelectItem>
                    <SelectItem value="on_leave">{m.employees_status_on_leave?.() || "في إجازة"}</SelectItem>
                    <SelectItem value="suspended">{m.employees_status_suspended?.() || "موقوف مؤقتاً"}</SelectItem>
                    <SelectItem value="terminated">{m.employees_status_terminated?.() || "منتهي الخدمة"}</SelectItem>
                  </SelectContent>
                </Select>
              {/snippet}
            </FormField>
          </div>

          <FormField id="emp_notes" label={m.common_notes?.() || "ملاحظات إضافية"} error={form.errors.notes?.[0]}>
            {#snippet children()}
              <Textarea
                id="emp_notes"
                placeholder={l("أي ملاحظات وظيفية أو متطلبات خاصة...", "Additional notes...")}
                class="min-h-[70px] text-xs resize-none"
                bind:value={form.values.notes}
                disabled={mode === "preview" || form.isSubmitting}
              />
            {/snippet}
          </FormField>
        </AccordionContent>
      </AccordionItem>

      <!-- Accordion Item 3: Account & Identity (Only on Create) -->
      {#if mode === "create"}
        <AccordionItem value="account" class="rounded-lg border border-border/60 bg-card px-4 py-1">
          <AccordionTrigger class="text-sm font-semibold hover:no-underline py-3">
            <div class="flex items-center gap-2">
              <ShieldCheck size={16} class="text-primary" />
              <span>{m.employees_tab_account?.() || "حساب النظام والأمان الموحد"}</span>
              {#if form.values.enable_account}
                <Badge variant="secondary" class="ms-2 text-[10px] bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 border-emerald-500/20">
                  {l("مفعل", "Enabled")}
                </Badge>
              {/if}
            </div>
          </AccordionTrigger>
          <AccordionContent class="pt-2 pb-4 flex flex-col gap-4">
            <div class="flex items-center justify-between p-3.5 rounded-lg border border-border/70 bg-muted/20">
              <div class="flex flex-col gap-0.5">
                <span class="text-xs font-bold text-foreground">{l("إنشاء حساب مستخدم وتهيئة الدخول", "Provision User Account & Unified Login")}</span>
                <span class="text-[11px] text-muted-foreground">{l("إنشاء حساب تسجيل دخول تلقائي بكلمة مرور آمنة وتعيين المجموعات والصلاحيات", "Automatically generate credentials and configure group permissions.")}</span>
              </div>
              <Switch
                checked={form.values.enable_account}
                onCheckedChange={(v) => (form.values.enable_account = v)}
                disabled={form.isSubmitting}
              />
            </div>

            {#if form.values.enable_account}
              <div class="flex flex-col gap-4 p-4 rounded-lg border border-border/60 bg-background/50">
                <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                  <FormField id="emp_pref_user" label={l("اسم المستخدم المفضل (اختياري)", "Preferred Username (Optional)")}>
                    {#snippet children()}
                      <Input
                        id="emp_pref_user"
                        type="text"
                        placeholder="m.alghamdi"
                        class="font-mono"
                        bind:value={form.values.preferred_username}
                        disabled={form.isSubmitting}
                      />
                    {/snippet}
                  </FormField>

                  <FormField id="emp_custom_pwd" label={l("كلمة مرور مخصصة (اختياري - يترك فارغاً للتوليد الآمن)", "Custom Password (Optional - Leave empty for CSPRNG)")}>
                    {#snippet children()}
                      <Input
                        id="emp_custom_pwd"
                        type="password"
                        placeholder="••••••••••••"
                        class="font-mono"
                        bind:value={form.values.custom_password}
                        disabled={form.isSubmitting}
                      />
                    {/snippet}
                  </FormField>
                </div>

                <div class="flex flex-col gap-1.5">
                  <Label class="text-xs font-medium">{l("تعيين مجموعات المستخدمين والصلاحيات", "Assign User Groups & Roles")}</Label>
                  <GroupSelector
                    {orgId}
                    bind:selectedGroupIds={form.values.selected_group_ids}
                  />
                </div>
              </div>
            {/if}
          </AccordionContent>
        </AccordionItem>
      {/if}
    </Accordion>
  </div>

  <!-- Bottom Actions Bar -->
  <div class="px-6 py-3 border-t border-border/40 flex gap-2 shrink-0 bg-background/50">
    <Button
      type="button"
      variant="ghost"
      onclick={onCancel}
      disabled={form.isSubmitting}
      class="flex-1 h-9 text-sm cursor-pointer"
    >
      {mode === "preview" ? (m.common_action_close?.() || "إغلاق") : (m.common_action_cancel?.() || "إلغاء")}
    </Button>

    {#if mode !== "preview"}
      <Button
        type="submit"
        disabled={form.isSubmitting}
        class="flex-1 h-9 text-sm font-semibold cursor-pointer"
      >
        {#if form.isSubmitting}
          <ArrowPath size={14} class="me-1.5 animate-spin" />
          {m.common_state_saving?.() || "جاري الحفظ..."}
        {:else}
          {mode === "create" ? (m.employees_new_title?.() || "تسجيل الموظف") : (m.common_action_save?.() || "حفظ التعديلات")}
        {/if}
      </Button>
    {/if}
  </div>
</form>
