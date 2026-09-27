<!--
  @component EmployeeForm
  Slide-over panel form for Creating and Editing Employee dossiers in HR.
  Designed to fit perfectly within Sheet layouts with Accordion sections,
  and live payroll & compensation calculations.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { goto } from "$app/navigation";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
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
import {
  Banknotes,
  BuildingOffice,
  User,
  UserGroup,
} from "@orbis/icons";
import { localizeHref } from "$lib/paraglide/runtime";
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
import { formatNumber, l } from "$lib/core/i18n";

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

// Form State (Initialized directly from employee prop without $effect)
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
let employeeType = $state(employee?.employee_type?.toLowerCase() || "teacher");
let jobTitle = $state(employee?.job_title || "");
let department = $state(employee?.department || "");
let orgUnitId = $state(employee?.org_unit_id || "");
let hireDate = $state(employee?.hire_date || new Date().toISOString().split("T")[0]);
let employmentType = $state(employee?.employment_type?.toLowerCase() || "full_time");
let workShift = $state(employee?.work_shift?.toLowerCase() || "morning");
let status = $state(employee?.status?.toLowerCase() || "active");

// Payroll & Compensation State
let rawPayroll: Record<string, any> = {};
try {
  if (employee?.notes && employee.notes.startsWith("{")) {
    rawPayroll = JSON.parse(employee.notes);
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
  rawPayroll.text_notes || (employee?.notes && !employee.notes.startsWith("{") ? employee.notes : "")
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

const isSaving = $derived(registerMutation.isPending || updateMutation.isPending);

async function handleSubmit() {
  if (!firstNameAr.trim() && !firstName.trim()) {
    toast.error(m.employees_error_name_required?.() || "يرجى إدخال اسم الموظف");
    return;
  }

  // Serialize payroll + text notes into notes JSON
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
      first_name: firstName.trim() || firstNameAr.trim(),
      middle_name: middleName.trim() || undefined,
      last_name: lastName.trim() || lastNameAr.trim() || undefined,
      first_name_ar: firstNameAr.trim() || undefined,
      middle_name_ar: middleNameAr.trim() || undefined,
      last_name_ar: lastNameAr.trim() || undefined,
      national_id: nationalId.trim() || undefined,
      passport_number: passportNumber.trim() || undefined,
      gender: gender || "male",
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
      account: null,
    };

    try {
      const response = await registerMutation.mutateAsync(payload);
      toast.success(m.employees_save_success?.() || "تم تسجيل الموظف بنجاح");

      if (onSuccess) {
        onSuccess(response.employee);
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
      notes: serializedNotes,
    };

    try {
      const res = await updateMutation.mutateAsync({ id: employee.id, req: payload });
      toast.success(m.employees_save_success?.() || "تم حفظ بيانات الموظف بنجاح");
      if (onSuccess) {
        onSuccess(res.employee);
      } else {
        goto(localizeHref(`/${orgId}/employees/${employee.id}`));
      }
    } catch (err) {
      toast.error(extractErrorMessage(err));
    }
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
            ? (m.employees_new_title?.() || "تسجيل موظف جديد") 
            : (m.employees_edit_title?.() || "تعديل ملف الموظف")}
        </Sheet.Title>
        <span class="text-xs text-muted-foreground">
          {mode === "create" 
            ? l("إدخال السجل الشخصي والوظيفي والمالي", "Enter personal, employment, and payroll records.") 
            : (employee?.full_name || employee?.first_name || "")}
        </span>
      </div>
    </div>
  </Sheet.Header>

  <!-- Scrollable Form Body with Accordion Layout -->
  <div class="flex-1 overflow-y-auto p-6 scrollbar-hide">
    <Accordion type="multiple" value={["personal", "employment", "payroll"]} class="flex flex-col gap-4">
      
      <!-- 1. PERSONAL INFORMATION -->
      <AccordionItem value="personal" class="rounded-lg border border-border/40 bg-card/60">
        <AccordionTrigger class="px-4 py-3 text-sm font-semibold hover:no-underline text-foreground">
          <div class="flex items-center gap-2">
            <User size={16} class="text-primary" />
            <span>{m.employees_tab_personal?.() || "البيانات الشخصية والهوية"}</span>
          </div>
        </AccordionTrigger>
        <AccordionContent class="px-4 pb-4 pt-1 flex flex-col gap-4">
          
          <!-- Arabic Names -->
          <div class="grid grid-cols-3 gap-3">
            <FormField label={m.employees_first_name_ar?.() || "الاسم الأول (بالعربية)"} required>
              {#snippet children()}
              <Input
                type="text"
                placeholder="محمد"
                class="text-xs h-9"
                bind:value={firstNameAr}
                disabled={isSaving}
                required
              />
              {/snippet}
            </FormField>
            <FormField label={m.employees_middle_name_ar?.() || "اسم الأب (بالعربية)"}>
              {#snippet children()}
              <Input
                type="text"
                placeholder="عبدالله"
                class="text-xs h-9"
                bind:value={middleNameAr}
                disabled={isSaving}
              />
              {/snippet}
            </FormField>
            <FormField label={m.employees_last_name_ar?.() || "اسم العائلة (بالعربية)"}>
              {#snippet children()}
              <Input
                type="text"
                placeholder="الغامدي"
                class="text-xs h-9"
                bind:value={lastNameAr}
                disabled={isSaving}
              />
              {/snippet}
            </FormField>
          </div>

          <!-- Latin / English Names -->
          <div class="grid grid-cols-3 gap-3">
            <FormField label={m.employees_first_name?.() || "First Name (Latin)"}>
              {#snippet children()}
              <Input
                type="text"
                placeholder="Mohammed"
                class="text-xs h-9"
                bind:value={firstName}
                disabled={isSaving}
              />
              {/snippet}
            </FormField>
            <FormField label={m.employees_middle_name?.() || "Middle Name"}>
              {#snippet children()}
              <Input
                type="text"
                placeholder="Abdullah"
                class="text-xs h-9"
                bind:value={middleName}
                disabled={isSaving}
              />
              {/snippet}
            </FormField>
            <FormField label={m.employees_last_name?.() || "Last Name"}>
              {#snippet children()}
              <Input
                type="text"
                placeholder="Al-Ghamdi"
                class="text-xs h-9"
                bind:value={lastName}
                disabled={isSaving}
              />
              {/snippet}
            </FormField>
          </div>

          <!-- ID & Demographic Grid -->
          <div class="grid grid-cols-2 sm:grid-cols-4 gap-3">
            <FormField label={m.employees_national_id?.() || "الهوية الوطنية / الإقامة"}>
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

            <FormField label={m.employees_passport_number?.() || "رقم جواز السفر"}>
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

            <FormField label={m.employees_gender?.() || "الجنس"}>
              {#snippet children()}
              <Select type="single" value={gender} onValueChange={(v) => { if (v) gender = v; }} disabled={isSaving}>
                <SelectTrigger class="text-xs h-9">
                  {gender === "male" ? (m.employees_gender_male?.() || "ذكر") : (m.employees_gender_female?.() || "أنثى")}
                </SelectTrigger>
                <SelectContent class="text-xs font-sans">
                  <SelectItem value="male">{m.employees_gender_male?.() || "ذكر"}</SelectItem>
                  <SelectItem value="female">{m.employees_gender_female?.() || "أنثى"}</SelectItem>
                </SelectContent>
              </Select>
              {/snippet}
            </FormField>

            <FormField label={m.employees_birth_date?.() || "تاريخ الميلاد"}>
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
          <div class="grid grid-cols-2 sm:grid-cols-4 gap-3">
            <FormField label={m.employees_phone?.() || "رقم الهاتف"}>
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

            <FormField label={m.employees_email?.() || "البريد الإلكتروني"}>
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

            <FormField label={m.employees_nationality?.() || "الجنسية"}>
              {#snippet children()}
              <NationalitySelect bind:value={nationality} disabled={isSaving} />
              {/snippet}
            </FormField>

            <FormField label={m.employees_blood_type?.() || "فصيلة الدم"}>
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
          <FormField label={m.employees_address?.() || "العنوان الوطني ومحل الإقامة"}>
            {#snippet children()}
            <Input
              type="text"
              placeholder="الرياض، حي النخيل، شارع الملك فهد"
              class="text-xs h-9"
              bind:value={address}
              disabled={isSaving}
            />
            {/snippet}
          </FormField>

        </AccordionContent>
      </AccordionItem>

      <!-- 2. EMPLOYMENT DETAILS -->
      <AccordionItem value="employment" class="rounded-lg border border-border/40 bg-card/60">
        <AccordionTrigger class="px-4 py-3 text-sm font-semibold hover:no-underline text-foreground">
          <div class="flex items-center gap-2">
            <BuildingOffice size={16} class="text-primary" />
            <span>{m.employees_tab_employment?.() || "البيانات الوظيفية والتعاقدية"}</span>
          </div>
        </AccordionTrigger>
        <AccordionContent class="px-4 pb-4 pt-1 flex flex-col gap-4">
          
          <div class="grid grid-cols-2 gap-3">
            <FormField label={m.employees_employee_number?.() || "الرقم الوظيفي"}>
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

            <FormField label={m.employees_job_title?.() || "المسمى الوظيفي"}>
              {#snippet children()}
              <Input
                type="text"
                placeholder="معلم رياضيات أول"
                class="text-xs h-9"
                bind:value={jobTitle}
                disabled={isSaving}
              />
              {/snippet}
            </FormField>
          </div>

          <div class="grid grid-cols-2 gap-3">
            <FormField label={m.employees_department?.() || "القسم / الإدارة"}>
              {#snippet children()}
              <Input
                type="text"
                placeholder="قسم العلوم والرياضيات"
                class="text-xs h-9"
                bind:value={department}
                disabled={isSaving}
              />
              {/snippet}
            </FormField>

            <FormField label={m.employees_type?.() || "الفئة الوظيفية"}>
              {#snippet children()}
              <Select type="single" value={employeeType} onValueChange={(v) => { if (v) employeeType = v; }} disabled={isSaving}>
                <SelectTrigger class="text-xs h-9">
                  {employeeType === "teacher" ? (m.employees_type_teacher?.() || "معلم / كادر تعليمي") :
                   employeeType === "admin" ? (m.employees_type_admin?.() || "إداري") :
                   employeeType === "staff" ? (m.employees_type_staff?.() || "موظف خدمات") : employeeType}
                </SelectTrigger>
                <SelectContent class="text-xs font-sans">
                  <SelectItem value="teacher">{m.employees_type_teacher?.() || "معلم / كادر تعليمي"}</SelectItem>
                  <SelectItem value="admin">{m.employees_type_admin?.() || "إداري"}</SelectItem>
                  <SelectItem value="staff">{m.employees_type_staff?.() || "موظف خدمات"}</SelectItem>
                  <SelectItem value="other">{m.common_guardians_relationship_other?.() || "أخرى"}</SelectItem>
                </SelectContent>
              </Select>
              {/snippet}
            </FormField>
          </div>

          <div class="grid grid-cols-2 sm:grid-cols-4 gap-3">
            <FormField label={m.employees_employment_type?.() || "نوع العقد"}>
              {#snippet children()}
              <Select type="single" value={employmentType} onValueChange={(v) => { if (v) employmentType = v; }} disabled={isSaving}>
                <SelectTrigger class="text-xs h-9">
                  {employmentType === "full_time" ? (m.employees_contract_fulltime?.() || "دوام كامل") :
                   employmentType === "part_time" ? (m.employees_contract_parttime?.() || "دوام جزئي") : (m.employees_contract_contract?.() || "عقد محدد")}
                </SelectTrigger>
                <SelectContent class="text-xs font-sans">
                  <SelectItem value="full_time">{m.employees_contract_fulltime?.() || "دوام كامل"}</SelectItem>
                  <SelectItem value="part_time">{m.employees_contract_parttime?.() || "دوام جزئي"}</SelectItem>
                  <SelectItem value="contract">{m.employees_contract_contract?.() || "عقد محدد"}</SelectItem>
                </SelectContent>
              </Select>
              {/snippet}
            </FormField>

            <FormField label={m.employees_work_shift?.() || "فترة العمل"}>
              {#snippet children()}
              <Select type="single" value={workShift} onValueChange={(v) => { if (v) workShift = v; }} disabled={isSaving}>
                <SelectTrigger class="text-xs h-9">
                  {workShift === "morning" ? (m.employees_shift_morning?.() || "صباحي") :
                   workShift === "evening" ? (m.employees_shift_evening?.() || "مسائي") : (m.employees_shift_flexible?.() || "مرن")}
                </SelectTrigger>
                <SelectContent class="text-xs font-sans">
                  <SelectItem value="morning">{m.employees_shift_morning?.() || "صباحي"}</SelectItem>
                  <SelectItem value="evening">{m.employees_shift_evening?.() || "مسائي"}</SelectItem>
                  <SelectItem value="flexible">{m.employees_shift_flexible?.() || "مرن"}</SelectItem>
                </SelectContent>
              </Select>
              {/snippet}
            </FormField>

            <FormField label={m.employees_hire_date?.() || "تاريخ التعيين"}>
              {#snippet children()}
              <Input
                type="date"
                class="text-xs font-mono h-9"
                bind:value={hireDate}
                disabled={isSaving}
              />
              {/snippet}
            </FormField>

            <FormField label={m.common_word_status?.() || "حالة الملف"}>
              {#snippet children()}
              <Select type="single" value={status} onValueChange={(v) => { if (v) status = v; }} disabled={isSaving}>
                <SelectTrigger class="text-xs h-9">
                  {status === "active" ? (m.employees_status_active?.() || "على رأس العمل") :
                   status === "suspended" ? (m.employees_status_suspended?.() || "موقوف") : (m.employees_status_terminated?.() || "منتهي عقده")}
                </SelectTrigger>
                <SelectContent class="text-xs font-sans">
                  <SelectItem value="active">{m.employees_status_active?.() || "على رأس العمل"}</SelectItem>
                  <SelectItem value="suspended">{m.employees_status_suspended?.() || "موقوف"}</SelectItem>
                  <SelectItem value="terminated">{m.employees_status_terminated?.() || "منتهي عقده"}</SelectItem>
                </SelectContent>
              </Select>
              {/snippet}
            </FormField>
          </div>

        </AccordionContent>
      </AccordionItem>

      <!-- 3. PAYROLL & COMPENSATION -->
      <AccordionItem value="payroll" class="rounded-lg border border-border/40 bg-card/60">
        <AccordionTrigger class="px-4 py-3 text-sm font-semibold hover:no-underline text-foreground">
          <div class="flex items-center gap-2">
            <Banknotes size={16} class="text-emerald-500" />
            <span>{l("الرواتب والبدلات والتعويضات", "Payroll & Compensation")}</span>
          </div>
        </AccordionTrigger>
        <AccordionContent class="px-4 pb-4 pt-1 flex flex-col gap-4">
          
          <!-- Salary & Allowances Grid -->
          <div class="grid grid-cols-2 sm:grid-cols-4 gap-3">
            <FormField label={l("الراتب الأساسي", "Basic Salary")}>
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

            <FormField label={l("بدل السكن", "Housing Allowance")}>
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

            <FormField label={l("بدل النقل", "Transport Allowance")}>
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

            <FormField label={l("بدلات ومكافآت أخرى", "Other Allowances")}>
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
            <FormField label={l("استقطاع التأمينات (GOSI)", "GOSI / Social Insurance")}>
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

            <FormField label={l("استقطاعات أخرى", "Other Deductions")}>
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
              <span class="text-[11px] text-muted-foreground">{l("إجمالي الراتب (Gross)", "Total Gross")}</span>
              <span class="text-sm font-bold font-mono text-foreground">{formatNumber(totalGrossSalary)} YER</span>
            </div>
            <div class="flex flex-col text-start">
              <span class="text-[11px] text-muted-foreground">{l("إجمالي الاستقطاعات", "Total Deductions")}</span>
              <span class="text-sm font-bold font-mono text-rose-500">-{formatNumber(totalDeductions)} YER</span>
            </div>
            <div class="flex flex-col text-start border-s border-border/40 ps-3">
              <span class="text-[11px] font-semibold text-emerald-600 dark:text-emerald-400">{l("صافي الراتب المستحق (Net)", "Net Payable")}</span>
              <span class="text-base font-black font-mono text-emerald-600 dark:text-emerald-400">{formatNumber(netPayableSalary)} YER</span>
            </div>
          </div>

          <!-- Bank & Routing Details -->
          <div class="grid grid-cols-1 sm:grid-cols-3 gap-3">
            <FormField label={l("طريقة الدفع", "Payment Method")}>
              {#snippet children()}
              <Select type="single" value={paymentMethod} onValueChange={(v) => { if (v) paymentMethod = v; }} disabled={isSaving}>
                <SelectTrigger class="text-xs h-9">
                  {paymentMethod === "bank_transfer" ? l("تحويل بنكي", "Bank Transfer") :
                   paymentMethod === "cheque" ? l("شيك", "Cheque") : l("نقداً / صندوق", "Cash")}
                </SelectTrigger>
                <SelectContent class="text-xs font-sans">
                  <SelectItem value="bank_transfer">{l("تحويل بنكي", "Bank Transfer")}</SelectItem>
                  <SelectItem value="cheque">{l("شيك", "Cheque")}</SelectItem>
                  <SelectItem value="cash">{l("نقداً / صندوق", "Cash")}</SelectItem>
                </SelectContent>
              </Select>
              {/snippet}
            </FormField>

            <FormField label={l("اسم البنك", "Bank Name")}>
              {#snippet children()}
              <Input
                type="text"
                placeholder={l("مثال: بنك الراجحي / بنك التضامن", "e.g. Al Rajhi Bank")}
                class="text-xs h-9"
                bind:value={bankName}
                disabled={isSaving}
              />
              {/snippet}
            </FormField>

            <FormField label={l("رقم الآيبان (IBAN)", "IBAN Account Number")}>
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
          <FormField label={m.employees_notes?.() || "ملاحظات إضافية"}>
            {#snippet children()}
            <Textarea
              rows={2}
              placeholder={m.employees_notes_placeholder?.() || "أدخل أي ملاحظات مهنية أو تعاقدية إضافية..."}
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
  <div class="px-6 py-3 border-t border-border/40 flex gap-2 shrink-0 bg-background/80 backdrop-blur-xs">
    <Button
      type="button"
      variant="ghost"
      onclick={handleCancelClick}
      disabled={isSaving}
      class="flex-1 h-9 text-sm"
    >
      {m.common_action_cancel?.() || "إلغاء"}
    </Button>
    <Button
      type="submit"
      disabled={isSaving}
      class="flex-1 h-9 text-sm font-semibold"
    >
      {#if isSaving}
        {m.common_state_saving?.() || "جاري الحفظ..."}
      {:else}
        {m.common_action_save?.() || "حفظ"}
      {/if}
    </Button>
  </div>
</form>
