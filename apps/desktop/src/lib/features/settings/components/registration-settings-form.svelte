<!--
  @component RegistrationSettingsForm
  Comprehensive, robust configuration hub for Admission & Registration policies,
  tailored specifically for educational institutions (K-12 schools, universities, institutes, Quranic centers).
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Switch } from "@orbis/ui/switch";
import { Badge } from "@orbis/ui/badge";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@orbis/ui/table";
import { Tabs, TabsList, TabsTrigger, TabsContent } from "@orbis/ui/tabs";
import {
  Plus,
  Trash2,
  Sparkles,
  ShieldCheck,
  FileCheck,
  SlidersHorizontal,
  GraduationCap,
  School,
  Building2,
  BookOpen,
  CheckCircle2,
} from "@lucide/svelte";
import { toast } from "@orbis/ui";
import { l } from "$lib/core/i18n";
import SettingsBottomBar from "$lib/shared/components/settings-bottom-bar.svelte";

let { org, isSaving, onsave } = $props<{
  org: any;
  isSaving: boolean;
  onsave: (registrationConfig: any) => void;
}>();

// Active tab
let activeTab = $state<"policies" | "documents" | "fields">("policies");

// Institution Type Presets
let institutionType = $state("school");

// Student Number Pattern
let studentNumberPrefix = $state("STU");
let studentNumberYearFormat = $state("YEAR"); // "YEAR" (2026) or "YEAR2" (26) or "NONE"
let studentNumberDigits = $state(4); // 4 -> 0001, 5 -> 00001, 6 -> 000001

// Admission Policies
let requiresApproval = $state(false);
let allowNoGuardian = $state(false);
let allowDuplicateNationalId = $state(false);
let autoAssignFees = $state(true);
let maxClassCapacity = $state<number | null>(30);
let ageCalculationDate = $state("start_of_year"); // "start_of_year" | "custom"

// Required Documents
interface DocItem {
  id: string;
  nameAr: string;
  nameEn: string;
  isRequired: boolean;
}

let requiredDocuments = $state<DocItem[]>([
  { id: "birth_cert", nameAr: "شهادة الميلاد الأصلية المعتمدة", nameEn: "Certified Birth Certificate", isRequired: true },
  { id: "national_id", nameAr: "بطاقة الهوية الوطنية / جواز السفر", nameEn: "National ID / Passport", isRequired: true },
  { id: "academic_transcript", nameAr: "كشف درجات / شهادة آخر مؤهل دراسي", nameEn: "Previous Academic Transcript", isRequired: true },
  { id: "health_record", nameAr: "كشف اللياقة الطبية وسجل التطعيمات", nameEn: "Medical Fitness & Immunization", isRequired: false },
  { id: "photos", nameAr: "صور شخصية حديثة (4x6)", nameEn: "Recent Personal Photographs", isRequired: true },
  { id: "guardian_id", nameAr: "صورة هوية ولي الأمر", nameEn: "Guardian National ID Copy", isRequired: true },
  { id: "transfer_cert", nameAr: "استمارة نقل أو إخلاء طرف للطلاب المحولين", nameEn: "Transfer / Clearance Certificate", isRequired: false },
]);

let newDocNameAr = $state("");
let newDocNameEn = $state("");

// Application Form Fields Configuration
interface FieldConfig {
  key: string;
  nameAr: string;
  nameEn: string;
  isActive: boolean;
  isRequired: boolean;
  preventDuplicate: boolean;
  supportsDuplicate: boolean;
}

let formFields = $state<FieldConfig[]>([
  { key: "national_id", nameAr: "الرقم الوطني / رقم الهوية", nameEn: "National ID Number", isActive: true, isRequired: true, preventDuplicate: true, supportsDuplicate: true },
  { key: "phone", nameAr: "رقم هاتف الطالب / التواصل", nameEn: "Student Phone Number", isActive: true, isRequired: true, preventDuplicate: false, supportsDuplicate: true },
  { key: "email", nameAr: "البريد الإلكتروني", nameEn: "Email Address", isActive: true, isRequired: false, preventDuplicate: false, supportsDuplicate: true },
  { key: "birth_date", nameAr: "تاريخ الميلاد", nameEn: "Date of Birth", isActive: true, isRequired: true, preventDuplicate: false, supportsDuplicate: false },
  { key: "sex", nameAr: "الجنس", nameEn: "Gender", isActive: true, isRequired: true, preventDuplicate: false, supportsDuplicate: false },
  { key: "nationality", nameAr: "الجنسية الرسمية", nameEn: "Nationality", isActive: true, isRequired: true, preventDuplicate: false, supportsDuplicate: false },
  { key: "address", nameAr: "عنوان السكن والحي", nameEn: "Residential Address", isActive: true, isRequired: false, preventDuplicate: false, supportsDuplicate: false },
  { key: "place_of_birth", nameAr: "مكان الميلاد", nameEn: "Place of Birth", isActive: true, isRequired: false, preventDuplicate: false, supportsDuplicate: false },
  { key: "blood_type", nameAr: "فصيلة الدم", nameEn: "Blood Type", isActive: true, isRequired: false, preventDuplicate: false, supportsDuplicate: false },
  { key: "emergency_contact", nameAr: "جهة الاتصال للطوارئ", nameEn: "Emergency Contact", isActive: true, isRequired: true, preventDuplicate: false, supportsDuplicate: false },
  { key: "medical_notes", nameAr: "الملاحظات الطبية والحساسية", nameEn: "Medical Notes & Allergies", isActive: true, isRequired: false, preventDuplicate: false, supportsDuplicate: false },
  { key: "previous_school", nameAr: "المدرسة / المؤسسة السابقة", nameEn: "Previous Institution", isActive: true, isRequired: false, preventDuplicate: false, supportsDuplicate: false },
]);

let isInitialized = $state(false);

// Load existing settings
$effect(() => {
  if (org && !isInitialized) {
    isInitialized = true;
    let settingsObj: any = {};
    try {
      settingsObj = typeof org.settings === "string" ? JSON.parse(org.settings) : org.settings || {};
    } catch {}

    const reg = settingsObj.registration || settingsObj.registration_config || {};

    if (reg.institution_type) institutionType = reg.institution_type;
    if (reg.student_number_prefix) studentNumberPrefix = reg.student_number_prefix;
    if (reg.student_number_digits) studentNumberDigits = Number(reg.student_number_digits);
    if (reg.student_number_year_format) studentNumberYearFormat = reg.student_number_year_format;

    const pol = reg.policy_settings || reg;
    if (pol.requires_approval !== undefined) requiresApproval = Boolean(pol.requires_approval);
    if (pol.allow_no_guardian !== undefined) allowNoGuardian = Boolean(pol.allow_no_guardian);
    if (pol.allow_duplicate_national_id !== undefined) allowDuplicateNationalId = Boolean(pol.allow_duplicate_national_id);
    if (pol.auto_assign_fees !== undefined) autoAssignFees = Boolean(pol.auto_assign_fees);
    if (pol.max_class_capacity !== undefined) maxClassCapacity = pol.max_class_capacity == null ? null : Number(pol.max_class_capacity);

    // Documents
    if (Array.isArray(reg.required_documents_list) && reg.required_documents_list.length > 0) {
      requiredDocuments = reg.required_documents_list;
    }

    // Fields
    if (Array.isArray(reg.form_fields_list) && reg.form_fields_list.length > 0) {
      formFields = reg.form_fields_list;
    }
  }
});

// Generated ID Preview
const studentNumberPreview = $derived.by(() => {
  const year = new Date().getFullYear();
  const yearStr = studentNumberYearFormat === "YEAR" ? `${year}-` : studentNumberYearFormat === "YEAR2" ? `${String(year).slice(2)}-` : "";
  const seq = "1".padStart(studentNumberDigits, "0");
  return `${studentNumberPrefix}-${yearStr}${seq}`;
});

// Apply presets based on institution type
function applyPreset(type: string) {
  institutionType = type;
  if (type === "school") {
    studentNumberPrefix = "SCH";
    studentNumberDigits = 4;
    allowNoGuardian = false;
    requiresApproval = true;
    maxClassCapacity = 30;
  } else if (type === "university") {
    studentNumberPrefix = "UNIV";
    studentNumberDigits = 5;
    allowNoGuardian = true;
    requiresApproval = true;
    maxClassCapacity = 60;
  } else if (type === "institute") {
    studentNumberPrefix = "INST";
    studentNumberDigits = 4;
    allowNoGuardian = true;
    requiresApproval = false;
    maxClassCapacity = 25;
  } else if (type === "quran_center") {
    studentNumberPrefix = "QRN";
    studentNumberDigits = 4;
    allowNoGuardian = false;
    requiresApproval = false;
    maxClassCapacity = 20;
  }
  toast.success(l("تم تطبيق إعدادات النموذج الافتراضي للمؤسسة", "Applied institution profile preset"));
}

// Add new document
function addDocument() {
  if (!newDocNameAr.trim() && !newDocNameEn.trim()) {
    toast.error(l("الرجاء إدخال اسم المستند", "Document name is required"));
    return;
  }
  const id = `doc_${Date.now()}`;
  requiredDocuments = [
    ...requiredDocuments,
    {
      id,
      nameAr: newDocNameAr.trim() || newDocNameEn.trim(),
      nameEn: newDocNameEn.trim() || newDocNameAr.trim(),
      isRequired: true,
    },
  ];
  newDocNameAr = "";
  newDocNameEn = "";
  toast.success(l("تمت إضافة المستند المطلوب", "Document added"));
}

// Remove document
function removeDocument(id: string) {
  requiredDocuments = requiredDocuments.filter((d) => d.id !== id);
}

// Save form
function handleFormSave() {
  const activeFieldsList = formFields.filter((f) => f.isActive).map((f) => f.key);
  const requiredFieldsList = formFields.filter((f) => f.isActive && f.isRequired).map((f) => f.key);
  const preventDuplicatesList = formFields.filter((f) => f.isActive && f.preventDuplicate).map((f) => f.key);

  const payload = {
    registration: {
      institution_type: institutionType,
      student_number_pattern: `${studentNumberPrefix}-{YEAR}-{SEQ:${studentNumberDigits}}`,
      student_number_prefix: studentNumberPrefix,
      student_number_digits: studentNumberDigits,
      student_number_year_format: studentNumberYearFormat,
      policy_settings: {
        requires_approval: requiresApproval,
        allow_no_guardian: allowNoGuardian,
        allow_duplicate_national_id: allowDuplicateNationalId,
        auto_assign_fees: autoAssignFees,
        max_class_capacity: maxClassCapacity,
      },
      active_fields: activeFieldsList,
      required_fields: requiredFieldsList,
      prevent_duplicates: preventDuplicatesList,
      required_documents: requiredDocuments.map((d) => d.nameAr),
      required_documents_list: requiredDocuments,
      form_fields_list: formFields,
    },
  };

  onsave(payload);
}
</script>

<div class="flex flex-col gap-6 text-start pb-20">
  <!-- Top Profile Selector Banner -->
  <div class="p-4 rounded-xl border border-border/70 bg-card shadow-2xs flex flex-col sm:flex-row items-center justify-between gap-4">
    <div class="flex items-center gap-3">
      <div class="w-10 h-10 rounded-xl bg-primary/10 border border-primary/20 flex items-center justify-center text-primary shrink-0">
        <GraduationCap class="w-5 h-5" />
      </div>
      <div>
        <h3 class="text-sm font-bold text-foreground">
          {l("نوع المؤسسة التعليمية والنمط الافتراضي", "Educational Institution Type")}
        </h3>
        <p class="text-xs text-muted-foreground mt-0.5">
          {l("تخصيص قواعد القبول والترقيم والمستندات وفق طبيعة المؤسسة", "Customize admission and numbering schemas by institution profile")}
        </p>
      </div>
    </div>

    <!-- Quick Preset Select -->
    <div class="w-full sm:w-56 shrink-0">
      <Select
        type="single"
        value={institutionType}
        onValueChange={(v) => {
          if (v) applyPreset(v);
        }}
      >
        <SelectTrigger class="h-9 w-full text-xs font-semibold">
          {#if institutionType === "school"}
            {l("مدرسة تعليم عام (K-12)", "K-12 School")}
          {:else if institutionType === "university"}
            {l("جامعة / كلية أكاديمية", "University / College")}
          {:else if institutionType === "institute"}
            {l("معهد تدريبي / مهني", "Training Institute")}
          {:else if institutionType === "quran_center"}
            {l("مركز تحفيظ قرآن كريم", "Quranic Center")}
          {:else}
            {institutionType}
          {/if}
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="school" class="text-xs">{l("مدرسة تعليم عام (K-12)", "K-12 School")}</SelectItem>
          <SelectItem value="university" class="text-xs">{l("جامعة / كلية أكاديمية", "University / College")}</SelectItem>
          <SelectItem value="institute" class="text-xs">{l("معهد تدريبي / مهني", "Training Institute")}</SelectItem>
          <SelectItem value="quran_center" class="text-xs">{l("مركز تحفيظ قرآن كريم", "Quranic Center")}</SelectItem>
        </SelectContent>
      </Select>
    </div>
  </div>

  <!-- Tabs Navigation -->
  <Tabs value={activeTab} onValueChange={(v) => (activeTab = v as any)}>
    <TabsList class="h-9 p-1 bg-muted/50 rounded-lg border border-border/40">
      <TabsTrigger value="policies" class="text-xs font-semibold gap-1.5 px-4 h-7">
        <ShieldCheck class="w-3.5 h-3.5" />
        <span>{l("سياسات القبول والترقيم", "Admission & Numbering")}</span>
      </TabsTrigger>
      <TabsTrigger value="documents" class="text-xs font-semibold gap-1.5 px-4 h-7">
        <FileCheck class="w-3.5 h-3.5" />
        <span>{l("المستندات المطلوبة", "Required Documents")}</span>
        <Badge variant="secondary" class="ms-1 px-1.5 py-0 text-[10px] font-mono">
          {requiredDocuments.length}
        </Badge>
      </TabsTrigger>
      <TabsTrigger value="fields" class="text-xs font-semibold gap-1.5 px-4 h-7">
        <SlidersHorizontal class="w-3.5 h-3.5" />
        <span>{l("حقول استمارة التسجيل", "Form Fields Matrix")}</span>
      </TabsTrigger>
    </TabsList>

    <!-- TAB 1: Policies & Student Numbering -->
    <TabsContent value="policies" class="space-y-6 pt-4">
      <!-- Student Numbering Card -->
      <div class="p-5 rounded-xl border border-border/70 bg-card shadow-2xs space-y-4">
        <div class="flex items-center justify-between border-b border-border/40 pb-3">
          <div>
            <h4 class="text-xs font-bold text-foreground">
              {l("نمط توليد الرقم الأكاديمي للطالب", "Student ID Numbering Format")}
            </h4>
            <p class="text-[11px] text-muted-foreground mt-0.5">
              {l("التركيبة التلقائية المعتمدة لتوليد الرقم الجامعي أو المدرسي للطلاب الجدد", "Automated sequence schema for new admissions")}
            </p>
          </div>

          <!-- Live Preview Badge -->
          <div class="flex items-center gap-2">
            <span class="text-xs text-muted-foreground">{l("معاينة الرقم:", "Preview:")}</span>
            <Badge variant="lime" class="font-mono text-xs px-2.5 py-0.5 font-bold">
              {studentNumberPreview}
            </Badge>
          </div>
        </div>

        <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
          <!-- Prefix -->
          <div class="flex flex-col gap-1.5">
            <Label class="text-xs font-semibold text-foreground">
              {l("البادئة (Prefix)", "Prefix")}
            </Label>
            <Input
              bind:value={studentNumberPrefix}
              placeholder="e.g. STU"
              class="h-8 text-xs font-mono uppercase"
            />
          </div>

          <!-- Year Token Format -->
          <div class="flex flex-col gap-1.5">
            <Label class="text-xs font-semibold text-foreground">
              {l("تضمين العام الدراسي", "Include Year")}
            </Label>
            <Select
              type="single"
              value={studentNumberYearFormat}
              onValueChange={(v) => (studentNumberYearFormat = v ?? "YEAR")}
            >
              <SelectTrigger class="h-8 text-xs">
                {studentNumberYearFormat === "YEAR"
                  ? l("العام كاملاً (مثال: 2026)", "Full 4-digit Year (2026)")
                  : studentNumberYearFormat === "YEAR2"
                  ? l("آخر خانتين (مثال: 26)", "Short 2-digit Year (26)")
                  : l("بدون العام", "No Year")}
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="YEAR" class="text-xs">{l("العام كاملاً (مثال: 2026)", "Full 4-digit Year (2026)")}</SelectItem>
                <SelectItem value="YEAR2" class="text-xs">{l("آخر خانتين (مثال: 26)", "Short 2-digit Year (26)")}</SelectItem>
                <SelectItem value="NONE" class="text-xs">{l("بدون العام", "No Year")}</SelectItem>
              </SelectContent>
            </Select>
          </div>

          <!-- Digits Sequence -->
          <div class="flex flex-col gap-1.5">
            <Label class="text-xs font-semibold text-foreground">
              {l("خانة التسلسل الرقمي", "Sequence Digits")}
            </Label>
            <Select
              type="single"
              value={String(studentNumberDigits)}
              onValueChange={(v) => (studentNumberDigits = Number(v || 4))}
            >
              <SelectTrigger class="h-8 text-xs font-mono">
                {studentNumberDigits} {l("خانات", "digits")} (مثال: {"1".padStart(studentNumberDigits, "0")})
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="3" class="text-xs font-mono">3 خانات (001)</SelectItem>
                <SelectItem value="4" class="text-xs font-mono">4 خانات (0001)</SelectItem>
                <SelectItem value="5" class="text-xs font-mono">5 خانات (00001)</SelectItem>
                <SelectItem value="6" class="text-xs font-mono">6 خانات (000001)</SelectItem>
              </SelectContent>
            </Select>
          </div>
        </div>
      </div>

      <!-- Admission Policies Toggles Card -->
      <div class="p-5 rounded-xl border border-border/70 bg-card shadow-2xs space-y-4">
        <h4 class="text-xs font-bold text-foreground border-b border-border/40 pb-2">
          {l("ضوابط وإجراءات القبول والتسجيل", "Admission Governance Rules")}
        </h4>

        <div class="flex flex-col divide-y divide-border/40">
          <!-- Approval Workflow -->
          <div class="py-3 flex items-center justify-between gap-4">
            <div class="flex flex-col gap-0.5">
              <span class="text-xs font-bold text-foreground">
                {l("اشتراط موافقة المسجل الأكاديمي / الإدارة", "Require Academic Registrar Approval")}
              </span>
              <span class="text-[11px] text-muted-foreground">
                {l("إخضاع طلبات التسجيل الجديدة لمرحلة المراجعة والاعتماد قبل التسكين في الشعب", "New applicants remain pending approval before enrollment")}
              </span>
            </div>
            <Switch bind:checked={requiresApproval} />
          </div>

          <!-- Allow No Guardian -->
          <div class="py-3 flex items-center justify-between gap-4">
            <div class="flex flex-col gap-0.5">
              <span class="text-xs font-bold text-foreground">
                {l("السماح بالتسجيل بدون ولي أمر (تسجيل مستقل)", "Allow Independent Enrollment (No Guardian)")}
              </span>
              <span class="text-[11px] text-muted-foreground">
                {l("مناسب لطلاب الجامعات والمعاهد والدراسات العليا", "Applicable for adult university and vocational students")}
              </span>
            </div>
            <Switch bind:checked={allowNoGuardian} />
          </div>

          <!-- Prevent Duplicate National ID -->
          <div class="py-3 flex items-center justify-between gap-4">
            <div class="flex flex-col gap-0.5">
              <span class="text-xs font-bold text-foreground">
                {l("التحقق الصارم من عدم تكرار الرقم الوطني / الهوية", "Strict Unique National ID Enforcement")}
              </span>
              <span class="text-[11px] text-muted-foreground">
                {l("منع تسجيل أي طالب برقم هوية مسجل مسبقاً في النظام", "Rejects duplicate national ID entries across the institution")}
              </span>
            </div>
            <Switch
              checked={!allowDuplicateNationalId}
              onCheckedChange={(v) => (allowDuplicateNationalId = !v)}
            />
          </div>

          <!-- Auto Assign Tuition Fees -->
          <div class="py-3 flex items-center justify-between gap-4">
            <div class="flex flex-col gap-0.5">
              <span class="text-xs font-bold text-foreground">
                {l("إسناد الرسوم الدراسية تلقائياً فور اعتماد القيد", "Auto-Assign Tuition Fee Template on Enrollment")}
              </span>
              <span class="text-[11px] text-muted-foreground">
                {l("إنشاء قيود وفواتير الرسوم تلقائياً للطالب بناءً على الصف والشعبة المسكن بها", "Automatically maps default fee templates based on grade level")}
              </span>
            </div>
            <Switch bind:checked={autoAssignFees} />
          </div>

          <!-- Max Capacity -->
          <div class="py-3 flex items-center justify-between gap-4">
            <div class="flex flex-col gap-0.5">
              <span class="text-xs font-bold text-foreground">
                {l("السعة القصوى الافتراضية للفصل / الشعبة", "Default Class Capacity Limit")}
              </span>
              <span class="text-[11px] text-muted-foreground">
                {l("الحد الأقصى لعدد الطلاب المسموح بتسكينهم في الشعبة الدراسية الواحدة", "Maximum enrollment limit per classroom section")}
              </span>
            </div>
            <Input
              type="number"
              bind:value={maxClassCapacity}
              min={5}
              max={200}
              class="h-8 w-24 text-xs font-mono text-center"
            />
          </div>
        </div>
      </div>
    </TabsContent>

    <!-- TAB 2: Required Documents -->
    <TabsContent value="documents" class="space-y-4 pt-4">
      <!-- Add Document Row -->
      <div class="p-4 rounded-xl border border-border/70 bg-card shadow-2xs space-y-3">
        <span class="text-xs font-bold text-foreground">
          {l("إضافة مستند أو وثيقة مطلوبة جديدة", "Add New Required Admission Document")}
        </span>

        <div class="grid grid-cols-1 sm:grid-cols-3 gap-3">
          <Input
            placeholder={l("اسم المستند بالعربية (مثال: كشف درجات)", "Arabic Document Name")}
            bind:value={newDocNameAr}
            class="h-8 text-xs text-end"
          />
          <Input
            placeholder={l("English Document Name", "English Document Name")}
            bind:value={newDocNameEn}
            class="h-8 text-xs"
          />
          <Button
            variant="lime"
            size="sm"
            class="h-8 text-xs font-bold gap-1.5"
            onclick={addDocument}
          >
            <Plus class="w-3.5 h-3.5" />
            <span>{l("إضافة المستند", "Add Document")}</span>
          </Button>
        </div>
      </div>

      <!-- Documents Table -->
      <div class="rounded-xl border border-border/70 bg-card overflow-hidden shadow-2xs">
        <Table>
          <TableHeader class="bg-muted/40">
            <TableRow>
              <TableHead class="text-start font-bold text-xs">{l("اسم الوثيقة والمستند المطلوب", "Document Title")}</TableHead>
              <TableHead class="text-start font-bold text-xs w-48">{l("الحالة الإلزامية", "Requirement")}</TableHead>
              <TableHead class="text-end font-bold text-xs w-20">{l("حذف", "Action")}</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {#each requiredDocuments as doc (doc.id)}
              <TableRow class="hover:bg-muted/10 transition-colors">
                <TableCell class="py-3">
                  <div class="flex flex-col gap-0.5">
                    <span class="text-xs font-bold text-foreground">{l(doc.nameAr, doc.nameEn)}</span>
                    <span class="text-[10px] text-muted-foreground">{doc.nameEn}</span>
                  </div>
                </TableCell>

                <TableCell class="py-3">
                  <button
                    type="button"
                    onclick={() => (doc.isRequired = !doc.isRequired)}
                    class="cursor-pointer select-none"
                  >
                    {#if doc.isRequired}
                      <Badge variant="lime" class="text-[10px] font-bold">
                        {l("إلزامي للقبول", "Mandatory")}
                      </Badge>
                    {:else}
                      <Badge variant="secondary" class="text-[10px] font-semibold">
                        {l("اختياري / تكميلي", "Optional")}
                      </Badge>
                    {/if}
                  </button>
                </TableCell>

                <TableCell class="py-3 text-end">
                  <Button
                    variant="ghost"
                    size="sm"
                    class="h-7 w-7 p-0 text-muted-foreground hover:text-destructive cursor-pointer"
                    onclick={() => removeDocument(doc.id)}
                  >
                    <Trash2 class="w-3.5 h-3.5" />
                  </Button>
                </TableCell>
              </TableRow>
            {/each}
          </TableBody>
        </Table>
      </div>
    </TabsContent>

    <!-- TAB 3: Form Fields Matrix -->
    <TabsContent value="fields" class="space-y-4 pt-4">
      <div class="rounded-xl border border-border/70 bg-card overflow-hidden shadow-2xs">
        <Table>
          <TableHeader class="bg-muted/40">
            <TableRow>
              <TableHead class="text-start font-bold text-xs">{l("حقل استمارة التسجيل", "Field Name")}</TableHead>
              <TableHead class="text-center font-bold text-xs w-36">{l("مفعل بالاستمارة", "Active")}</TableHead>
              <TableHead class="text-center font-bold text-xs w-36">{l("إلزامي التعبئة", "Required")}</TableHead>
              <TableHead class="text-center font-bold text-xs w-36">{l("منع التكرار", "Unique")}</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {#each formFields as field (field.key)}
              <TableRow class="hover:bg-muted/10 transition-colors">
                <!-- Field Title -->
                <TableCell class="py-3">
                  <div class="flex flex-col gap-0.5">
                    <span class="text-xs font-bold text-foreground">{l(field.nameAr, field.nameEn)}</span>
                    <span class="text-[10px] font-mono text-muted-foreground">{field.key}</span>
                  </div>
                </TableCell>

                <!-- Active Toggle -->
                <TableCell class="py-3 text-center">
                  <Switch bind:checked={field.isActive} />
                </TableCell>

                <!-- Required Toggle -->
                <TableCell class="py-3 text-center">
                  <Switch
                    bind:checked={field.isRequired}
                    disabled={!field.isActive}
                  />
                </TableCell>

                <!-- Prevent Duplicate Toggle -->
                <TableCell class="py-3 text-center">
                  {#if field.supportsDuplicate}
                    <Switch
                      bind:checked={field.preventDuplicate}
                      disabled={!field.isActive}
                    />
                  {:else}
                    <span class="text-[10px] text-muted-foreground/50">—</span>
                  {/if}
                </TableCell>
              </TableRow>
            {/each}
          </TableBody>
        </Table>
      </div>
    </TabsContent>
  </Tabs>

  <!-- Bottom Save Bar -->
  <SettingsBottomBar
    isDirty={true}
    {isSaving}
    onsave={handleFormSave}
    onreset={() => {
      isInitialized = false;
    }}
  />
</div>
