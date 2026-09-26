<!--
  @component RegistrationSettingsForm
  Next-Generation 2026 ERP Admission & Registration Policy Center.
  Features:
  - Interactive Student Number Token Pattern Builder with live sample chip preview.
  - Institutional Admission Governance & Capacity limits.
  - AddDocumentRequirementSheet integration for dynamic per-class/grade document requirements.
  - Dynamic Application Form Fields Matrix.
  - SettingsBottomBar with zero-latency dirty detection.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Switch } from "@orbis/ui/switch";
import { Badge } from "@orbis/ui/badge";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
  CardAction,
} from "@orbis/ui/card";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import {
  Grid,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@orbis/ui/table";
import { Tabs, TabsList, TabsTrigger, TabsContent } from "@orbis/ui/tabs";
import {
  AdjustmentsHorizontal,
  BookOpen,
  CheckCircle,
  DocumentCheck,
  FloppyDisk,
  Lock,
  Plus,
  Sparkles,
  Trash,
  UserBadgeCheck,
  Tag,
  Funnel,
} from "@orbis/icons";
import { toast } from "@orbis/ui";
import { l } from "$lib/core/i18n";
import { api, type ClassDto, type DocumentRequirementDto } from "$lib/api";
import { ClassSelect } from "$lib/features/academics";
import SettingsBottomBar from "$lib/shared/components/settings-bottom-bar.svelte";
import AddDocumentRequirementSheet from "./add-document-requirement-sheet.svelte";

let { org, isSaving, onsave } = $props<{
  org: any;
  isSaving: boolean;
  onsave: (registrationConfig: any) => void;
}>();

// Required Documents Interface
interface DocItem {
  id: string;
  code: string;
  nameAr: string;
  nameEn: string;
  isRequired: boolean;
  blocksEnrollment: boolean;
  gradeLevelId?: string; // specific class/grade ID or undefined for all
  condition: string; // all_students | national_only | foreign_only | transfer_students
  description?: string;
}

// Application Form Fields Configuration Interface & Defaults
interface FieldConfig {
  key: string;
  nameAr: string;
  nameEn: string;
  isActive: boolean;
  isRequired: boolean;
  preventDuplicate: boolean;
  supportsDuplicate: boolean;
}

const defaultFormFields: FieldConfig[] = [
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
];

function parseOrgRegistration(rawOrg: any) {
  let settingsObj: any = {};
  try {
    settingsObj = typeof rawOrg?.settings === "string" ? JSON.parse(rawOrg.settings) : rawOrg?.settings || {};
  } catch {
    settingsObj = {};
  }

  const reg = settingsObj.registration || settingsObj.registration_config || {};
  const pol = reg.policy_settings || reg;

  return {
    institutionType: reg.institution_type || "school",
    studentNumberPrefix: reg.student_number_prefix || "STU",
    studentNumberYearFormat: reg.student_number_year_format || "YEAR",
    studentNumberDigits: reg.student_number_digits ? Number(reg.student_number_digits) : 4,
    requiresApproval: pol.requires_approval !== undefined ? Boolean(pol.requires_approval) : false,
    allowNoGuardian: pol.allow_no_guardian !== undefined ? Boolean(pol.allow_no_guardian) : false,
    allowDuplicateNationalId: pol.allow_duplicate_national_id !== undefined ? Boolean(pol.allow_duplicate_national_id) : false,
    autoAssignFees: pol.auto_assign_fees !== undefined ? Boolean(pol.auto_assign_fees) : true,
    maxClassCapacity: pol.max_class_capacity ? Number(pol.max_class_capacity) : 30,
    requiredDocuments: Array.isArray(reg.required_documents_list) ? reg.required_documents_list : [],
    formFields: Array.isArray(reg.form_fields_list) && reg.form_fields_list.length > 0
      ? reg.form_fields_list
      : defaultFormFields,
  };
}

const parsed = parseOrgRegistration(org);

// Editable Form State
let institutionType = $state<string>(parsed.institutionType);
let studentNumberPrefix = $state<string>(parsed.studentNumberPrefix);
let studentNumberYearFormat = $state<string>(parsed.studentNumberYearFormat);
let studentNumberDigits = $state<number>(parsed.studentNumberDigits);
let requiresApproval = $state<boolean>(parsed.requiresApproval);
let allowNoGuardian = $state<boolean>(parsed.allowNoGuardian);
let allowDuplicateNationalId = $state<boolean>(parsed.allowDuplicateNationalId);
let autoAssignFees = $state<boolean>(parsed.autoAssignFees);
let maxClassCapacity = $state<number>(parsed.maxClassCapacity);
let requiredDocuments = $state<DocItem[]>(JSON.parse(JSON.stringify(parsed.requiredDocuments)));
let formFields = $state<FieldConfig[]>(JSON.parse(JSON.stringify(parsed.formFields)));

// Database Classes & Sheet State
let dbClasses = $state<ClassDto[]>([]);
let isAddDocSheetOpen = $state(false);
let selectedClassFilter = $state<string>("all");

// Active Tab
let activeTab = $state<"policies" | "documents" | "fields">("policies");

// Initial state snapshot for dirty checking
const initialState = {
  institutionType: parsed.institutionType,
  studentNumberPrefix: parsed.studentNumberPrefix,
  studentNumberYearFormat: parsed.studentNumberYearFormat,
  studentNumberDigits: parsed.studentNumberDigits,
  requiresApproval: parsed.requiresApproval,
  allowNoGuardian: parsed.allowNoGuardian,
  allowDuplicateNationalId: parsed.allowDuplicateNationalId,
  autoAssignFees: parsed.autoAssignFees,
  maxClassCapacity: parsed.maxClassCapacity,
  requiredDocuments: JSON.parse(JSON.stringify(parsed.requiredDocuments)),
  formFields: JSON.parse(JSON.stringify(parsed.formFields)),
};

const isDirty = $derived.by(() => {
  return (
    institutionType !== initialState.institutionType ||
    studentNumberPrefix !== initialState.studentNumberPrefix ||
    studentNumberYearFormat !== initialState.studentNumberYearFormat ||
    studentNumberDigits !== initialState.studentNumberDigits ||
    requiresApproval !== initialState.requiresApproval ||
    allowNoGuardian !== initialState.allowNoGuardian ||
    allowDuplicateNationalId !== initialState.allowDuplicateNationalId ||
    autoAssignFees !== initialState.autoAssignFees ||
    maxClassCapacity !== initialState.maxClassCapacity ||
    JSON.stringify(requiredDocuments) !== JSON.stringify(initialState.requiredDocuments) ||
    JSON.stringify(formFields) !== JSON.stringify(initialState.formFields)
  );
});

onMount(async () => {
  try {
    const [backendReqs, classesList] = await Promise.all([
      api.listDocumentRequirements().catch(() => []),
      api.listClasses().catch(() => []),
    ]);

    if (classesList && Array.isArray(classesList)) {
      dbClasses = classesList;
    }

    if (backendReqs && backendReqs.length > 0) {
      const mapped: DocItem[] = backendReqs.map((r: DocumentRequirementDto) => ({
        id: r.id,
        code: r.code,
        nameAr: r.name_ar || r.name_en || r.code,
        nameEn: r.name_en || r.name_ar || r.code,
        isRequired: r.is_mandatory,
        blocksEnrollment: r.blocks_enrollment,
        gradeLevelId: r.target_id || undefined,
        condition: r.condition || "all_students",
      }));
      requiredDocuments = mapped;
    }
  } catch (err) {
    console.error("Failed to load registration requirements data:", err);
  }
});

function resetToInitial() {
  institutionType = initialState.institutionType;
  studentNumberPrefix = initialState.studentNumberPrefix;
  studentNumberYearFormat = initialState.studentNumberYearFormat;
  studentNumberDigits = initialState.studentNumberDigits;
  requiresApproval = initialState.requiresApproval;
  allowNoGuardian = initialState.allowNoGuardian;
  allowDuplicateNationalId = initialState.allowDuplicateNationalId;
  autoAssignFees = initialState.autoAssignFees;
  maxClassCapacity = initialState.maxClassCapacity;
  requiredDocuments = JSON.parse(JSON.stringify(initialState.requiredDocuments));
  formFields = JSON.parse(JSON.stringify(initialState.formFields));
}

// Generated ID Preview
const studentNumberPreview = $derived.by(() => {
  const year = new Date().getFullYear();
  const yearStr = studentNumberYearFormat === "YEAR" ? `${year}-` : studentNumberYearFormat === "YEAR2" ? `${String(year).slice(2)}-` : "";
  const seq = "1".padStart(studentNumberDigits, "0");
  return `${studentNumberPrefix}-${yearStr}${seq}`;
});

// Filtered Documents
const filteredDocuments = $derived.by(() => {
  if (selectedClassFilter === "all") return requiredDocuments;
  return requiredDocuments.filter(d => d.gradeLevelId === selectedClassFilter || !d.gradeLevelId);
});

function handleRequirementCreated(created: DocumentRequirementDto) {
  const item: DocItem = {
    id: created.id,
    code: created.code,
    nameAr: created.name_ar || created.name_en || created.code,
    nameEn: created.name_en || created.name_ar || created.code,
    isRequired: created.is_mandatory,
    blocksEnrollment: created.blocks_enrollment,
    gradeLevelId: created.target_id || undefined,
    condition: created.condition || "all_students",
  };
  requiredDocuments = [...requiredDocuments, item];
}

// Remove document
async function removeDocument(doc: DocItem) {
  if (doc.id && !doc.id.startsWith("doc_")) {
    try {
      await api.deleteDocumentRequirement(doc.id).catch(() => null);
    } catch (err) {
      console.warn("Backend requirement deletion skipped:", err);
    }
  }
  requiredDocuments = requiredDocuments.filter((d) => d.id !== doc.id && d.code !== doc.code);
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

<div class="flex flex-col gap-6 text-start pb-24">
  <!-- Top Profile Selector Banner -->
  <Card class="border-border/80 shadow-xs">
    <CardHeader>
      <div class="flex items-center gap-3 text-start">
        <div class="w-8 h-8 rounded-lg bg-primary/10 border border-primary/20 flex items-center justify-center text-primary shrink-0">
          <BookOpen class="w-4 h-4" />
        </div>
        <div class="flex flex-col">
          <CardTitle>
            {l("نوع المؤسسة التعليمية والنمط الافتراضي", "Educational Institution Type")}
          </CardTitle>
          <CardDescription>
            {l("تخصيص قواعد القبول والترقيم والمستندات وفق طبيعة المؤسسة", "Customize admission and numbering schemas by institution profile")}
          </CardDescription>
        </div>
      </div>
      <CardAction>
        <Badge variant="outline" class="px-3 py-1 h-7 text-xs font-bold gap-1.5 bg-muted/40 border-border text-foreground">
          <Lock class="w-3 h-3 text-muted-foreground" />
          <span>
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
          </span>
        </Badge>
      </CardAction>
    </CardHeader>
  </Card>

  <!-- Tabs Navigation -->
  <Tabs value={activeTab} onValueChange={(v) => (activeTab = v as any)}>
    <TabsList class="h-9 p-1 bg-muted/50 rounded-lg border border-border/40">
      <TabsTrigger value="policies" class="text-xs font-semibold gap-1.5 px-4 h-7">
        <UserBadgeCheck class="w-3.5 h-3.5" />
        <span>{l("سياسات القبول والترقيم", "Admission & Numbering")}</span>
      </TabsTrigger>
      <TabsTrigger value="documents" class="text-xs font-semibold gap-1.5 px-4 h-7">
        <DocumentCheck class="w-3.5 h-3.5" />
        <span>{l("استوديو وثائق المراحل والصفوف", "Class Requirements Studio")}</span>
        <Badge variant="secondary" class="ms-1 px-1.5 py-0 text-[10px] font-mono">
          {requiredDocuments.length}
        </Badge>
      </TabsTrigger>
      <TabsTrigger value="fields" class="text-xs font-semibold gap-1.5 px-4 h-7">
        <AdjustmentsHorizontal class="w-3.5 h-3.5" />
        <span>{l("حقول استمارة التسجيل", "Form Fields Matrix")}</span>
      </TabsTrigger>
    </TabsList>

    <!-- TAB 1: Policies & Student Numbering -->
    <TabsContent value="policies" class="space-y-6 pt-4">
      <!-- Student Numbering Card -->
      <Card class="border-border/80 shadow-xs">
        <CardHeader>
          <div class="flex flex-col text-start">
            <CardTitle>
              {l("نمط توليد الرقم الأكاديمي للطالب", "Student ID Numbering Format")}
            </CardTitle>
            <CardDescription>
              {l("التركيبة التلقائية المعتمدة لتوليد الرقم الجامعي أو المدرسي للطلاب الجدد", "Automated sequence schema for new admissions")}
            </CardDescription>
          </div>
          <CardAction>
            <div class="flex items-center gap-2">
              <span class="text-xs text-muted-foreground">{l("معاينة الرقم:", "Preview:")}</span>
              <Badge variant="lime" class="font-mono text-xs px-2.5 py-0.5 font-bold">
                {studentNumberPreview}
              </Badge>
            </div>
          </CardAction>
        </CardHeader>

        <CardContent class="p-6 grid grid-cols-1 sm:grid-cols-3 gap-4 text-start">
          <!-- Prefix -->
          <div class="flex flex-col gap-1.5">
            <Label class="text-xs font-semibold text-foreground">
              {l("البادئة (Prefix)", "Prefix")}
            </Label>
            <Input
              bind:value={studentNumberPrefix}
              placeholder="e.g. STU"
              class="h-9 text-xs font-mono uppercase"
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
              <SelectTrigger class="h-9 text-xs">
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
              <SelectTrigger class="h-9 text-xs font-mono">
                {studentNumberDigits} {l("خانات", "digits")} (مثال: {"1".padStart(studentNumberDigits, "0")})
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="3" class="text-xs font-mono">3 {l("خانات", "digits")} (001)</SelectItem>
                <SelectItem value="4" class="text-xs font-mono">4 {l("خانات", "digits")} (0001)</SelectItem>
                <SelectItem value="5" class="text-xs font-mono">5 {l("خانات", "digits")} (00001)</SelectItem>
                <SelectItem value="6" class="text-xs font-mono">6 {l("خانات", "digits")} (000001)</SelectItem>
              </SelectContent>
            </Select>
          </div>
        </CardContent>
      </Card>

      <!-- Admission Policies Toggles Card -->
      <Card class="border-border/80 shadow-xs">
        <CardHeader>
          <div class="flex flex-col text-start">
            <CardTitle>
              {l("ضوابط وإجراءات القبول والتسجيل", "Admission Governance Rules")}
            </CardTitle>
          </div>
        </CardHeader>

        <CardContent class="p-6 flex flex-col divide-y divide-border">
          <!-- Approval Workflow -->
          <div class="py-3 flex items-center justify-between gap-4 first:pt-0">
            <div class="flex flex-col gap-0.5 text-start">
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
            <div class="flex flex-col gap-0.5 text-start">
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
            <div class="flex flex-col gap-0.5 text-start">
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
            <div class="flex flex-col gap-0.5 text-start">
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
          <div class="py-3 flex items-center justify-between gap-4 last:pb-0">
            <div class="flex flex-col gap-0.5 text-start">
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
              class="h-9 w-24 text-xs font-mono text-center"
            />
          </div>
        </CardContent>
      </Card>
    </TabsContent>

    <!-- TAB 2: Class/Grade Requirements Studio -->
    <TabsContent value="documents" class="space-y-4 pt-4">
      
      <!-- Studio Header Actions Bar -->
      <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
        <div class="flex items-center gap-2">
          <Funnel class="w-4 h-4 text-muted-foreground" />
          <span class="text-xs font-semibold text-muted-foreground">{l("تصفية حسب الصف / المرحلة:", "Filter Class / Grade:")}</span>
          <div class="w-52">
            <ClassSelect
              bind:value={selectedClassFilter}
              classes={dbClasses}
              showAllOption={true}
              allOptionLabel={l("كافة الصفوف والمراحل", "All Classes & Grades")}
              placeholder={l("كافة الصفوف والمراحل", "All Classes & Grades")}
            />
          </div>
        </div>

        <Button
          variant="lime"
          size="sm"
          class="h-8 px-4 text-xs font-bold gap-1.5 cursor-pointer shrink-0"
          onclick={() => (isAddDocSheetOpen = true)}
        >
          <Plus class="w-3.5 h-3.5" />
          <span>{l("إضافة متطلب وثيقة جديد", "Add Document Requirement")}</span>
        </Button>
      </div>

      <!-- Requirements Grid Card -->
      <Card class="border-border/80 shadow-xs">
        <CardContent class="p-0">
          <Grid>
            <TableHeader class="bg-muted/40">
              <TableRow>
                <TableHead class="text-start font-bold text-xs">{l("اسم الوثيقة والمستند المطلوب", "Document Title")}</TableHead>
                <TableHead class="text-start font-bold text-xs w-44">{l("الصف / المرحلة", "Class / Grade")}</TableHead>
                <TableHead class="text-start font-bold text-xs w-36">{l("شرط القبول", "Condition")}</TableHead>
                <TableHead class="text-center font-bold text-xs w-32">{l("الإلزامية", "Mandatory")}</TableHead>
                <TableHead class="text-center font-bold text-xs w-36">{l("أثر الحجب", "Blocks")}</TableHead>
                <TableHead class="text-end font-bold text-xs w-16">{l("حذف", "Action")}</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {#each filteredDocuments as doc (doc.id)}
                {@const mappedClass = dbClasses.find(c => String(c.id) === doc.gradeLevelId)}
                <TableRow class="hover:bg-muted/10 transition-colors">
                  <!-- Name & Code -->
                  <TableCell class="py-3 text-start">
                    <div class="flex flex-col gap-0.5">
                      <span class="text-xs font-bold text-foreground">{l(doc.nameAr, doc.nameEn)}</span>
                      <span class="text-[10px] font-mono text-muted-foreground">{doc.code || doc.nameEn}</span>
                    </div>
                  </TableCell>

                  <!-- Class / Grade Level -->
                  <TableCell class="py-3 text-start">
                    {#if mappedClass}
                      <Badge variant="outline" class="text-[10px] font-semibold">
                        {mappedClass.name || mappedClass.level }
                      </Badge>
                    {:else}
                      <Badge variant="secondary" class="text-[10px] font-semibold">
                        {l("كافة الصفوف", "All Classes")}
                      </Badge>
                    {/if}
                  </TableCell>

                  <!-- Condition Badge -->
                  <TableCell class="py-3 text-start">
                    <span class="text-[11px] text-muted-foreground">
                      {#if doc.condition === "all_students"}
                        {l("كافة الطلاب", "All")}
                      {:else if doc.condition === "national_only"}
                        {l("وطنيون فقط", "Nationals")}
                      {:else if doc.condition === "foreign_only"}
                        {l("وافدون فقط", "Expats")}
                      {:else if doc.condition === "transfer_students"}
                        {l("طلاب محولون", "Transfers")}
                      {:else}
                        {doc.condition}
                      {/if}
                    </span>
                  </TableCell>

                  <!-- Mandatory Toggle -->
                  <TableCell class="py-3 text-center">
                    <button
                      type="button"
                      onclick={() => (doc.isRequired = !doc.isRequired)}
                      class="cursor-pointer select-none"
                    >
                      {#if doc.isRequired}
                        <Badge variant="lime" class="text-[10px] font-bold">
                          {l("إلزامي", "Mandatory")}
                        </Badge>
                      {:else}
                        <Badge variant="secondary" class="text-[10px] font-semibold">
                          {l("اختياري", "Optional")}
                        </Badge>
                      {/if}
                    </button>
                  </TableCell>

                  <!-- Blocks Enrollment Toggle -->
                  <TableCell class="py-3 text-center">
                    <button
                      type="button"
                      onclick={() => (doc.blocksEnrollment = !doc.blocksEnrollment)}
                      class="cursor-pointer select-none"
                    >
                      {#if doc.blocksEnrollment}
                        <Badge variant="destructive" class="text-[10px] font-bold">
                          {l("يحجب القيد", "Blocks")}
                        </Badge>
                      {:else}
                        <Badge variant="outline" class="text-[10px] text-muted-foreground font-medium">
                          {l("لا يحجب", "No Block")}
                        </Badge>
                      {/if}
                    </button>
                  </TableCell>

                  <!-- Remove Action -->
                  <TableCell class="py-3 text-end">
                    <Button
                      variant="ghost"
                      size="sm"
                      class="h-7 w-7 p-0 text-muted-foreground hover:text-destructive cursor-pointer"
                      onclick={() => removeDocument(doc)}
                    >
                      <Trash class="w-3.5 h-3.5" />
                    </Button>
                  </TableCell>
                </TableRow>
              {/each}
              {#if filteredDocuments.length === 0}
                <TableRow>
                  <TableCell colspan={6} class="py-8 text-center text-xs text-muted-foreground">
                    {l("لا توجد متطلبات وثائق مسجلة لهذا الصف حالياً.", "No document requirements configured for this selection.")}
                  </TableCell>
                </TableRow>
              {/if}
            </TableBody>
          </Grid>
        </CardContent>
      </Card>
    </TabsContent>

    <!-- TAB 3: Form Fields Matrix -->
    <TabsContent value="fields" class="space-y-4 pt-4">
      <Card class="border-border/80 shadow-xs">
        <CardContent class="p-0">
          <Grid>
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
                  <TableCell class="py-3 text-start">
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
          </Grid>
        </CardContent>
      </Card>
    </TabsContent>
  </Tabs>

  <!-- Add Document Requirement Slide-over Sheet -->
  <AddDocumentRequirementSheet
    bind:open={isAddDocSheetOpen}
    classes={dbClasses}
    oncreated={handleRequirementCreated}
  />

  <!-- Bottom Save Bar -->
  <SettingsBottomBar
    {isDirty}
    {isSaving}
    onsave={handleFormSave}
    onreset={resetToInitial}
  />
</div>
