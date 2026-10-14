<!--
  @component EmployeeForm
  Slide-over panel form for Creating and Editing Employee dossiers in HR.
  Designed to fit perfectly within Sheet layouts with Accordion sections,
  and live payroll & compensation calculations.
-->
<script lang="ts">
import { untrack } from "svelte";
import * as m from "$lib/paraglide/messages";
import { Input } from "@orbis/ui/input";
import { Textarea } from "@orbis/ui/textarea";
import { Button } from "@orbis/ui/button";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui/select";
import * as Sheet from "@orbis/ui/sheet";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@orbis/ui/accordion";
import { FormField, toast } from "@orbis/ui";
import { Checkbox } from "@orbis/ui/checkbox";
import {
  Banknotes,
  BuildingOffice,
  User,
  UserGroup,
} from "@orbis/icons";
import { extractErrorMessage } from "$lib/api";
import type {
  EmployeeDetailDto,
  RegisterEmployeeRequest,
  UpdateEmployeeRequest,
} from "../../../api/types";
import {
  createRegisterEmployeeMutation,
  createUpdateEmployeeMutation,
} from "../queries";
import NationalitySelect from "$lib/shared/components/nationality-select.svelte";
import { formatNumber } from "$lib/core/i18n";
import { BilingualNameFields } from "@orbis/ui/bilingual-name-fields";
import { suggestLocalTranslation } from "$lib/shared/translation/local-engine";
import { splitPersonName } from "$lib/shared/utils/person-name";
import { FormController } from "$lib/shared/forms/form-controller.svelte";
import { z } from "zod";

interface Props {
  orgId: string;
  mode?: "create" | "edit";
  employee?: EmployeeDetailDto | null;
  onSuccess?: (emp: EmployeeDetailDto) => void;
  onCancel?: () => void;
}

let { 
  orgId, 
  mode = "create", 
  employee = null,
  onSuccess,
  onCancel,
}: Props = $props();

const initialEmployee = untrack(() => employee);

const form = new FormController({
  initialValues: {
    fullNameEn: [initialEmployee?.first_name, initialEmployee?.middle_name, initialEmployee?.last_name]
      .filter(Boolean)
      .join(" "),
    fullNameAr: [initialEmployee?.first_name_ar, initialEmployee?.middle_name_ar, initialEmployee?.last_name_ar]
      .filter(Boolean)
      .join(" "),
  },
  schema: z.object({
    fullNameEn: z.string(),
    fullNameAr: z.string(),
  }).refine(
    ({ fullNameAr, fullNameEn }) => Boolean(fullNameAr.trim() || fullNameEn.trim()),
    { path: ["fullNameAr"], message: m.employees_error_name_required() },
  ),
});

let createLoginAccount = $state(false);
let nationalId = $state(initialEmployee?.national_id || "");
let passportNumber = $state(initialEmployee?.passport_number || "");
let gender = $state(initialEmployee?.gender || "male");
let maritalStatus = $state(initialEmployee?.marital_status || "");
let birthDate = $state(initialEmployee?.birth_date || "");
let nationality = $state(initialEmployee?.nationality || "Saudi");
let bloodType = $state(initialEmployee?.blood_type || "");
let phone = $state(initialEmployee?.phone || "");
let email = $state(initialEmployee?.email || "");
let address = $state(initialEmployee?.address || "");

let employeeNumber = $state(initialEmployee?.employee_number || "");
let employeeType = $state(initialEmployee?.employee_type?.toLowerCase() || "teacher");
let jobTitle = $state(initialEmployee?.job_title || "");
let department = $state(initialEmployee?.department || "");
let orgUnitId = $state(initialEmployee?.org_unit_id || "");
let hireDate = $state(initialEmployee?.hire_date || new Date().toISOString().split("T")[0]);
let employmentType = $state(initialEmployee?.employment_type?.toLowerCase() || "full_time");
let workShift = $state(initialEmployee?.work_shift?.toLowerCase() || "morning");
let status = $state(initialEmployee?.status?.toLowerCase() || "active");

// Payroll & Compensation State
let rawPayroll: Record<string, any> = {};
try {
  if (initialEmployee?.notes && initialEmployee.notes.startsWith("{")) {
    rawPayroll = JSON.parse(initialEmployee.notes);
  }
} catch {}

let basicSalary = $state<number>(rawPayroll.basic_salary ?? 0);
let housingAllowance = $state<number>(rawPayroll.housing_allowance ?? 0);
let transportAllowance = $state<number>(rawPayroll.transport_allowance ?? 0);
let otherAllowances = $state<number>(rawPayroll.other_allowances ?? 0);
let gosiDeduction = $state<number>(rawPayroll.gosi_deduction ?? 0);
let otherDeductions = $state<number>(rawPayroll.other_deductions ?? 0);
let bankName = $state<string>(rawPayroll.bank_name || "");
let bankIban = $state<string>(rawPayroll.bank_iban || "");
let paymentMethod = $state<string>(rawPayroll.payment_method || "bank_transfer");
let textNotes = $state<string>(
  rawPayroll.text_notes || (initialEmployee?.notes && !initialEmployee.notes.startsWith("{") ? initialEmployee.notes : "")
);

// Derived Payroll Totals
const totalGrossSalary = $derived(
  (Number(basicSalary) || 0) +
  (Number(housingAllowance) || 0) +
  (Number(transportAllowance) || 0) +
  (Number(otherAllowances) || 0)
);

const totalDeductions = $derived(
  (Number(gosiDeduction) || 0) +
  (Number(otherDeductions) || 0)
);

const netPayableSalary = $derived(
  totalGrossSalary - totalDeductions
);

// Queries & Mutations
const registerMutation = createRegisterEmployeeMutation();
const updateMutation = createUpdateEmployeeMutation();

const isSaving = $derived(
  form.isSubmitting || registerMutation.isPending || updateMutation.isPending,
);

async function handleSubmit() {
  const submitted = await form.submit(async () => {
    const { fullNameAr, fullNameEn } = form.values;
    const namePartsAr = splitPersonName(fullNameAr || fullNameEn, "موظف");
    const namePartsEn = splitPersonName(fullNameEn || fullNameAr, "Employee");

    const payrollPayload = {
      basic_salary: Number(basicSalary) || 0,
      housing_allowance: Number(housingAllowance) || 0,
      transport_allowance: Number(transportAllowance) || 0,
      other_allowances: Number(otherAllowances) || 0,
      gosi_deduction: Number(gosiDeduction) || 0,
      other_deductions: Number(otherDeductions) || 0,
      gross_salary: totalGrossSalary,
      net_salary: netPayableSalary,
      bank_name: bankName.trim(),
      bank_iban: bankIban.trim(),
      payment_method: paymentMethod,
      text_notes: textNotes.trim(),
    };
    const serializedNotes = JSON.stringify(payrollPayload);

    if (mode === "create") {
      const payload: RegisterEmployeeRequest = {
        first_name: namePartsEn.firstName,
        middle_name: namePartsEn.middleName,
        last_name: namePartsEn.lastName,
        first_name_ar: namePartsAr.firstName,
        middle_name_ar: namePartsAr.middleName,
        last_name_ar: namePartsAr.lastName,
        display_name: fullNameAr.trim() || fullNameEn.trim(),
        national_id: nationalId.trim() || undefined,
        passport_number: passportNumber.trim() || undefined,
        gender: gender || "male",
        marital_status: maritalStatus || undefined,
        birth_date: birthDate || undefined,
        nationality: nationality.trim() || "Saudi",
        blood_type: bloodType || undefined,
        phone: phone.trim() || undefined,
        email: email.trim() || undefined,
        address: address.trim() || undefined,

        employee_number: employeeNumber.trim() || undefined,
        employee_type: (employeeType || "teacher").toLowerCase(),
        job_title: jobTitle.trim() || undefined,
        department: department.trim() || undefined,
        org_unit_id: orgUnitId.trim() || undefined,
        hire_date: hireDate || undefined,
        status: (status || "active").toLowerCase(),
        employment_type: (employmentType || "full_time").toLowerCase(),
        work_shift: (workShift || "morning").toLowerCase(),
        notes: serializedNotes,
        account: createLoginAccount
          ? {
              enabled: true,
              preferred_username: email.trim() || employeeNumber.trim() || undefined,
              role: (employeeType || "teacher").toLowerCase() === "teacher" ? "teacher" : "staff",
              delivery_channel: email.trim() ? "email" : phone.trim() ? "sms" : "manual",
            }
          : null,
      };

      try {
        const response = await registerMutation.mutateAsync(payload);
        toast.success(m.employees_save_success());
        if (response.provisioning) {
          toast.success(
            m.employees_login_account_created({
              username: response.provisioning.username,
            }),
          );
        }

        if (onSuccess) {
          onSuccess(response.employee);
        }
      } catch (err) {
        toast.error(extractErrorMessage(err));
        throw err;
      }
    } else if (mode === "edit" && employee) {
      const payload: UpdateEmployeeRequest = {
        first_name: namePartsEn.firstName,
        middle_name: namePartsEn.middleName,
        last_name: namePartsEn.lastName,
        first_name_ar: namePartsAr.firstName,
        middle_name_ar: namePartsAr.middleName,
        last_name_ar: namePartsAr.lastName,
        display_name: fullNameAr.trim() || fullNameEn.trim(),
        national_id: nationalId.trim() || undefined,
        passport_number: passportNumber.trim() || undefined,
        gender: gender || undefined,
        marital_status: maritalStatus || undefined,
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
        notes: serializedNotes,
      };

      try {
        const res = await updateMutation.mutateAsync({ id: employee.id, req: payload });
        toast.success(m.employees_save_success());
        if (onSuccess) {
          onSuccess(res.employee);
        }
      } catch (err) {
        toast.error(extractErrorMessage(err));
        throw err;
      }
    }
  });

  if (!submitted && !form.formError && form.errors.fullNameAr) {
    toast.error(m.employees_error_name_required());
  }
}

function handleCancelClick() {
  if (onCancel) {
    onCancel();
  } else {
    history.back();
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

<form
  onsubmit={(e) => {
    e.preventDefault();
    handleSubmit();
  }}
  class="flex flex-col h-full overflow-hidden gap-0 bg-background text-foreground"
>
  <!-- Header matching sheet standards -->
  <Sheet.Header class="px-6 py-4 border-b border-border/40 shrink-0 text-start flex flex-row items-center justify-between">
    <div class="flex items-center gap-3">
      <div class="flex size-9 shrink-0 items-center justify-center rounded-xl bg-primary/10 text-primary">
        <UserGroup size={18} />
      </div>
      <div class="flex flex-col">
        <Sheet.Title class="text-base font-bold text-foreground">
          {mode === "create" 
            ? (m.employees_new_title()) 
            : (m.employees_edit_title())}
        </Sheet.Title>
        <span class="text-xs text-muted-foreground">
          {mode === "create" 
            ? m.ui_copy_enter_personal_employment_and_payroll_reco_f1caab5e() 
            : (employee?.full_name || employee?.first_name || "")}
        </span>
      </div>
    </div>
  </Sheet.Header>

  <!-- Scrollable Form Body with Accordion Layout -->
  <div class="flex-1 min-h-0 overflow-y-auto px-4 py-4 sm:px-5 scrollbar-hide">
    <Accordion
      type="multiple"
      value={["personal", "employment", "payroll"]}
      class="flex flex-col"
    >
      
      <!-- 1. PERSONAL INFORMATION -->
      <AccordionItem value="personal" class="border-b border-border/50">
        <AccordionTrigger class="px-1 py-3 text-sm font-semibold hover:no-underline text-foreground">
          <div class="flex items-center gap-2">
            <User size={16} class="text-primary" />
            <span>{m.employees_tab_personal()}</span>
          </div>
        </AccordionTrigger>
        <AccordionContent class="px-1 pb-5 pt-2 flex flex-col gap-4 [&_label]:text-[13px]">
          
          <BilingualNameFields
            arId="employee-full-name-ar"
            enId="employee-full-name-en"
            arLabel={m.common_form_full_name_ar()}
            enLabel={m.common_form_full_name_en()}
            bind:arValue={form.values.fullNameAr}
            bind:enValue={form.values.fullNameEn}
            arPlaceholder={m.common_form_full_name_ar_placeholder()}
            enPlaceholder={m.common_form_full_name_en_placeholder()}
            kind="person-name"
            requestSuggestion={suggestLocalTranslation}
            arRequired
            disabled={isSaving}
            loadingMessage={m.bilingual_entry_loading()}
            failureMessage={m.bilingual_entry_failure()}
          />

          <!-- ID & Demographic Grid -->
          <div class="grid grid-cols-2 gap-3 sm:gap-4">
            <FormField label={m.employees_national_id()}>
              {#snippet children()}
              <Input
                type="text"
                placeholder="10XXXXXXXX"
                class="text-xs font-mono h-9"
                bind:value={nationalId}
                disabled={isSaving}
              />
              {/snippet}
            </FormField>

            <FormField label={m.employees_passport_number()}>
              {#snippet children()}
              <Input
                type="text"
                placeholder="A1234567"
                class="text-xs font-mono h-9"
                bind:value={passportNumber}
                disabled={isSaving}
              />
              {/snippet}
            </FormField>

            <FormField label={m.employees_gender()}>
              {#snippet children()}
              <Select type="single" value={gender} onValueChange={(v) => { if (v) gender = v; }} disabled={isSaving}>
                <SelectTrigger class="text-xs h-9">
                  {gender === "male" ? (m.employees_gender_male()) : (m.employees_gender_female())}
                </SelectTrigger>
                <SelectContent class="text-xs font-sans">
                  <SelectItem value="male">{m.employees_gender_male()}</SelectItem>
                  <SelectItem value="female">{m.employees_gender_female()}</SelectItem>
                </SelectContent>
              </Select>
              {/snippet}
            </FormField>

            <FormField label={m.employees_marital_status()}>
              {#snippet children()}
              <Select type="single" value={maritalStatus} onValueChange={(v) => { if (v) maritalStatus = v; }} disabled={isSaving}>
                <SelectTrigger class="text-xs h-9">
                  {maritalStatus === "single" ? m.employees_marital_status_single() :
                   maritalStatus === "married" ? m.employees_marital_status_married() :
                   maritalStatus === "divorced" ? m.employees_marital_status_divorced() :
                   maritalStatus === "widowed" ? m.employees_marital_status_widowed() :
                   maritalStatus === "separated" ? m.employees_marital_status_separated() : "—"}
                </SelectTrigger>
                <SelectContent class="text-xs font-sans">
                  <SelectItem value="single">{m.employees_marital_status_single()}</SelectItem>
                  <SelectItem value="married">{m.employees_marital_status_married()}</SelectItem>
                  <SelectItem value="divorced">{m.employees_marital_status_divorced()}</SelectItem>
                  <SelectItem value="widowed">{m.employees_marital_status_widowed()}</SelectItem>
                  <SelectItem value="separated">{m.employees_marital_status_separated()}</SelectItem>
                </SelectContent>
              </Select>
              {/snippet}
            </FormField>

            <FormField label={m.employees_birth_date()}>
              {#snippet children()}
              <Input
                type="date"
                class="text-xs font-mono h-9"
                bind:value={birthDate}
                disabled={isSaving}
              />
              {/snippet}
            </FormField>
          </div>

          <!-- Contact & Nationality -->
          <div class="grid grid-cols-2 gap-3 sm:gap-4">
            <FormField label={m.employees_phone()}>
              {#snippet children()}
              <Input
                type="tel"
                placeholder="+966 5X XXX XXXX"
                class="text-xs font-mono h-9"
                bind:value={phone}
                disabled={isSaving}
              />
              {/snippet}
            </FormField>

            <FormField label={m.employees_email()}>
              {#snippet children()}
              <Input
                type="email"
                placeholder="employee@school.edu.sa"
                class="text-xs font-mono h-9"
                bind:value={email}
                disabled={isSaving}
              />
              {/snippet}
            </FormField>

            <FormField label={m.employees_nationality()}>
              {#snippet children()}
              <NationalitySelect bind:value={nationality} disabled={isSaving} />
              {/snippet}
            </FormField>

            <FormField label={m.employees_blood_type()}>
              {#snippet children()}
              <Select type="single" value={bloodType} onValueChange={(v) => { if (v) bloodType = v; }} disabled={isSaving}>
                <SelectTrigger class="text-xs h-9">
                  {bloodType || "—"}
                </SelectTrigger>
                <SelectContent class="text-xs font-mono">
                  <SelectItem value="A+">A+</SelectItem>
                  <SelectItem value="A-">A-</SelectItem>
                  <SelectItem value="B+">B+</SelectItem>
                  <SelectItem value="B-">B-</SelectItem>
                  <SelectItem value="O+">O+</SelectItem>
                  <SelectItem value="O-">O-</SelectItem>
                  <SelectItem value="AB+">AB+</SelectItem>
                  <SelectItem value="AB-">AB-</SelectItem>
                </SelectContent>
              </Select>
              {/snippet}
            </FormField>
          </div>

          <!-- Address -->
          <FormField label={m.employees_address()}>
            {#snippet children()}
            <Input
              type="text"
              placeholder={m.operations_ui_employee_address_example()}
              class="text-xs h-9"
              bind:value={address}
              disabled={isSaving}
            />
            {/snippet}
          </FormField>

          {#if mode === "create"}
            <div class="flex items-start gap-3 rounded-lg border border-border/50 bg-muted/20 p-3 sm:p-4">
              <Checkbox
                id="employee-create-login-account"
                checked={createLoginAccount}
                onCheckedChange={(value) => (createLoginAccount = value === true)}
              />
              <label for="employee-create-login-account" class="flex flex-col gap-1 text-start cursor-pointer">
                <span class="text-xs font-semibold text-foreground">{m.bulk_entry_employee_header_provision_user()}</span>
                <span class="text-[11px] text-muted-foreground">{m.employees_create_login_account_description()}</span>
              </label>
            </div>
          {/if}

        </AccordionContent>
      </AccordionItem>

      <!-- 2. EMPLOYMENT DETAILS -->
      <AccordionItem value="employment" class="border-b border-border/50">
        <AccordionTrigger class="px-1 py-3 text-sm font-semibold hover:no-underline text-foreground">
          <div class="flex items-center gap-2">
            <BuildingOffice size={16} class="text-primary" />
            <span>{m.employees_tab_employment()}</span>
          </div>
        </AccordionTrigger>
        <AccordionContent class="px-1 pb-5 pt-2 flex flex-col gap-4 [&_label]:text-[13px]">
          
          <div class="grid grid-cols-2 gap-3">
            <FormField label={m.employees_employee_number()}>
              {#snippet children()}
              <Input
                type="text"
                placeholder="EMP-00123"
                class="text-xs font-mono h-9"
                bind:value={employeeNumber}
                disabled={isSaving}
              />
              {/snippet}
            </FormField>

            <FormField label={m.employees_job_title()}>
              {#snippet children()}
              <Input
                type="text"
                placeholder={m.operations_ui_employee_job_title_example()}
                class="text-xs h-9"
                bind:value={jobTitle}
                disabled={isSaving}
              />
              {/snippet}
            </FormField>
          </div>

          <div class="grid grid-cols-2 gap-3">
            <FormField label={m.employees_department()}>
              {#snippet children()}
              <Input
                type="text"
                placeholder={m.operations_ui_employee_department_example()}
                class="text-xs h-9"
                bind:value={department}
                disabled={isSaving}
              />
              {/snippet}
            </FormField>

            <FormField label={m.employees_type()}>
              {#snippet children()}
              <Select type="single" value={employeeType} onValueChange={(v) => { if (v) employeeType = v; }} disabled={isSaving}>
                <SelectTrigger class="text-xs h-9">
                  {employeeType === "teacher" ? (m.employees_type_teacher()) :
                   employeeType === "admin" ? (m.employees_type_admin()) :
                   employeeType === "staff" ? (m.employees_type_staff()) : employeeType}
                </SelectTrigger>
                <SelectContent class="text-xs font-sans">
                  <SelectItem value="teacher">{m.employees_type_teacher()}</SelectItem>
                  <SelectItem value="admin">{m.employees_type_admin()}</SelectItem>
                  <SelectItem value="staff">{m.employees_type_staff()}</SelectItem>
                  <SelectItem value="other">{m.common_guardians_relationship_other()}</SelectItem>
                </SelectContent>
              </Select>
              {/snippet}
            </FormField>
          </div>

          <div class="grid grid-cols-2 gap-3 sm:gap-4">
            <FormField label={m.employees_employment_type()}>
              {#snippet children()}
              <Select type="single" value={employmentType} onValueChange={(v) => { if (v) employmentType = v; }} disabled={isSaving}>
                <SelectTrigger class="text-xs h-9">
                  {employmentType === "full_time" ? (m.employees_contract_fulltime()) :
                   employmentType === "part_time" ? (m.employees_contract_parttime()) :
                   employmentType === "volunteer" ? (m.employees_contract_volunteer()) :
                   employmentType === "hourly" ? (m.employees_contract_hourly()) : (m.employees_contract_contract())}
                </SelectTrigger>
                <SelectContent class="text-xs font-sans">
                  <SelectItem value="full_time">{m.employees_contract_fulltime()}</SelectItem>
                  <SelectItem value="part_time">{m.employees_contract_parttime()}</SelectItem>
                  <SelectItem value="contract">{m.employees_contract_contract()}</SelectItem>
                  <SelectItem value="hourly">{m.employees_contract_hourly()}</SelectItem>
                  <SelectItem value="volunteer">{m.employees_contract_volunteer()}</SelectItem>
                </SelectContent>
              </Select>
              {/snippet}
            </FormField>

            <FormField label={m.employees_work_shift()}>
              {#snippet children()}
              <Select type="single" value={workShift} onValueChange={(v) => { if (v) workShift = v; }} disabled={isSaving}>
                <SelectTrigger class="text-xs h-9">
                  {workShift === "morning" ? (m.employees_shift_morning()) :
                   workShift === "evening" ? (m.employees_shift_evening()) : (m.employees_shift_flexible())}
                </SelectTrigger>
                <SelectContent class="text-xs font-sans">
                  <SelectItem value="morning">{m.employees_shift_morning()}</SelectItem>
                  <SelectItem value="evening">{m.employees_shift_evening()}</SelectItem>
                  <SelectItem value="flexible">{m.employees_shift_flexible()}</SelectItem>
                </SelectContent>
              </Select>
              {/snippet}
            </FormField>

            <FormField label={m.employees_hire_date()}>
              {#snippet children()}
              <Input
                type="date"
                class="text-xs font-mono h-9"
                bind:value={hireDate}
                disabled={isSaving}
              />
              {/snippet}
            </FormField>

            <FormField label={m.common_word_status()}>
              {#snippet children()}
              <Select type="single" value={status} onValueChange={(v) => { if (v) status = v; }} disabled={isSaving}>
                <SelectTrigger class="text-xs h-9">
                  {status === "active" ? (m.employees_status_active()) :
                   status === "suspended" ? (m.employees_status_suspended()) : (m.employees_status_terminated())}
                </SelectTrigger>
                <SelectContent class="text-xs font-sans">
                  <SelectItem value="active">{m.employees_status_active()}</SelectItem>
                  <SelectItem value="suspended">{m.employees_status_suspended()}</SelectItem>
                  <SelectItem value="terminated">{m.employees_status_terminated()}</SelectItem>
                </SelectContent>
              </Select>
              {/snippet}
            </FormField>
          </div>

        </AccordionContent>
      </AccordionItem>

      <!-- 3. PAYROLL & COMPENSATION -->
      <AccordionItem value="payroll" class="border-b border-border/50">
        <AccordionTrigger class="px-1 py-3 text-sm font-semibold hover:no-underline text-foreground">
          <div class="flex items-center gap-2">
            <Banknotes size={16} class="text-emerald-500" />
            <span>{m.ui_copy_payroll_compensation_93b1d7a4()}</span>
          </div>
        </AccordionTrigger>
        <AccordionContent class="px-1 pb-5 pt-2 flex flex-col gap-4 [&_label]:text-[13px]">
          
          <!-- Salary & Allowances Grid -->
          <div class="grid grid-cols-2 gap-3 sm:gap-4">
            <FormField label={m.ui_copy_basic_salary_0385806d()}>
              {#snippet children()}
              <Input
                type="number"
                min="0"
                step="0.01"
                placeholder="0.00"
                class="text-xs font-mono h-9"
                bind:value={basicSalary}
                disabled={isSaving}
              />
              {/snippet}
            </FormField>

            <FormField label={m.ui_copy_housing_allowance_5244c34d()}>
              {#snippet children()}
              <Input
                type="number"
                min="0"
                step="0.01"
                placeholder="0.00"
                class="text-xs font-mono h-9"
                bind:value={housingAllowance}
                disabled={isSaving}
              />
              {/snippet}
            </FormField>

            <FormField label={m.ui_copy_transport_allowance_28c7b865()}>
              {#snippet children()}
              <Input
                type="number"
                min="0"
                step="0.01"
                placeholder="0.00"
                class="text-xs font-mono h-9"
                bind:value={transportAllowance}
                disabled={isSaving}
              />
              {/snippet}
            </FormField>

            <FormField label={m.ui_copy_other_allowances_865ee31f()}>
              {#snippet children()}
              <Input
                type="number"
                min="0"
                step="0.01"
                placeholder="0.00"
                class="text-xs font-mono h-9"
                bind:value={otherAllowances}
                disabled={isSaving}
              />
              {/snippet}
            </FormField>
          </div>

          <!-- Deductions Grid -->
          <div class="grid grid-cols-2 gap-3">
            <FormField label={m.ui_copy_gosi_social_insurance_03abbd70()}>
              {#snippet children()}
              <Input
                type="number"
                min="0"
                step="0.01"
                placeholder="0.00"
                class="text-xs font-mono h-9 text-rose-500"
                bind:value={gosiDeduction}
                disabled={isSaving}
              />
              {/snippet}
            </FormField>

            <FormField label={m.ui_copy_other_deductions_8d08c919()}>
              {#snippet children()}
              <Input
                type="number"
                min="0"
                step="0.01"
                placeholder="0.00"
                class="text-xs font-mono h-9 text-rose-500"
                bind:value={otherDeductions}
                disabled={isSaving}
              />
              {/snippet}
            </FormField>
          </div>

          <!-- Summary KPI Banner -->
          <div class="grid grid-cols-3 gap-3 p-3.5 bg-muted/30 border border-border/40 rounded-lg">
            <div class="flex flex-col text-start">
              <span class="text-[11px] text-muted-foreground">{m.ui_copy_total_gross_49972225()}</span>
              <span class="text-sm font-bold font-mono text-foreground">{formatNumber(totalGrossSalary)} YER</span>
            </div>
            <div class="flex flex-col text-start">
              <span class="text-[11px] text-muted-foreground">{m.ui_copy_total_deductions_045a8067()}</span>
              <span class="text-sm font-bold font-mono text-rose-500">-{formatNumber(totalDeductions)} YER</span>
            </div>
            <div class="flex flex-col text-start border-s border-border/40 ps-3">
              <span class="text-[11px] font-semibold text-emerald-600 dark:text-emerald-400">{m.ui_copy_net_payable_23b86c48()}</span>
              <span class="text-base font-black font-mono text-emerald-600 dark:text-emerald-400">{formatNumber(netPayableSalary)} YER</span>
            </div>
          </div>

          <!-- Bank & Routing Details -->
          <div class="grid grid-cols-1 gap-3 sm:grid-cols-2 sm:gap-4">
            <FormField label={m.ui_copy_payment_method_f140647a()}>
              {#snippet children()}
              <Select type="single" value={paymentMethod} onValueChange={(v) => { if (v) paymentMethod = v; }} disabled={isSaving}>
                <SelectTrigger class="text-xs h-9">
                  {paymentMethod === "bank_transfer" ? m.finance_pos_bank_transfer() :
                   paymentMethod === "cheque" ? m.ui_copy_cheque_4f37d4c3() : m.ui_copy_cash_ec480ad1()}
                </SelectTrigger>
                <SelectContent class="text-xs font-sans">
                  <SelectItem value="bank_transfer">{m.finance_pos_bank_transfer()}</SelectItem>
                  <SelectItem value="cheque">{m.ui_copy_cheque_4f37d4c3()}</SelectItem>
                  <SelectItem value="cash">{m.ui_copy_cash_ec480ad1()}</SelectItem>
                </SelectContent>
              </Select>
              {/snippet}
            </FormField>

            <FormField label={m.ui_copy_bank_name_7c740997()}>
              {#snippet children()}
              <Input
                type="text"
                placeholder={m.ui_copy_e_g_al_rajhi_bank_5c1a77cc()}
                class="text-xs h-9"
                bind:value={bankName}
                disabled={isSaving}
              />
              {/snippet}
            </FormField>

            <FormField label={m.ui_copy_iban_account_number_7ea55e25()}>
              {#snippet children()}
              <Input
                type="text"
                placeholder="SA0000000000000000000000"
                class="text-xs font-mono h-9 uppercase"
                bind:value={bankIban}
                disabled={isSaving}
              />
              {/snippet}
            </FormField>
          </div>

          <!-- Notes Textarea -->
          <FormField label={m.employees_notes()}>
            {#snippet children()}
            <Textarea
              rows={2}
              placeholder={m.employees_notes_placeholder()}
              class="text-xs resize-none"
              bind:value={textNotes}
              disabled={isSaving}
            />
            {/snippet}
          </FormField>

        </AccordionContent>
      </AccordionItem>

    </Accordion>
  </div>

  <!-- Sticky Bottom Actions -->
  <div class="px-4 py-3 sm:px-5 border-t border-border/40 flex gap-3 shrink-0 bg-background/95">
    <Button
      type="button"
      variant="ghost"
      onclick={handleCancelClick}
      disabled={isSaving}
      class="flex-1 h-9 text-sm"
    >
      {m.common_action_cancel()}
    </Button>
    <Button
      type="submit"
      disabled={isSaving}
      class="flex-1 h-9 text-sm font-semibold"
    >
      {#if isSaving}
        {m.common_state_saving()}
      {:else}
        {m.common_action_save()}
      {/if}
    </Button>
  </div>
</form>
