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
import { onMount, untrack } from "svelte";
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
import { OrbisGrid, type OrbisColumnDef, type GridActionEvent, type ActionItem } from "@orbis/svelte-grid";
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
  nameAr?: string;
  nameEn?: string;
  label?: () => string;
  isActive: boolean;
  isRequired: boolean;
  preventDuplicate: boolean;
  supportsDuplicate: boolean;
}

const defaultFormFields: FieldConfig[] = [
  { key: "national_id", label: m.common_form_doc_national_id, isActive: true, isRequired: true, preventDuplicate: true, supportsDuplicate: true },
  { key: "phone", label: m.common_form_student_phone, isActive: true, isRequired: true, preventDuplicate: false, supportsDuplicate: true },
  { key: "email", label: m.common_form_student_email, isActive: true, isRequired: false, preventDuplicate: false, supportsDuplicate: true },
  { key: "birth_date", label: m.registration_field_birth_date, isActive: true, isRequired: true, preventDuplicate: false, supportsDuplicate: false },
  { key: "sex", label: m.common_form_sex, isActive: true, isRequired: true, preventDuplicate: false, supportsDuplicate: false },
  { key: "nationality", label: m.common_form_nationality, isActive: true, isRequired: true, preventDuplicate: false, supportsDuplicate: false },
  { key: "address", label: m.common_form_address, isActive: true, isRequired: false, preventDuplicate: false, supportsDuplicate: false },
  { key: "place_of_birth", label: m.common_form_place_of_birth, isActive: true, isRequired: false, preventDuplicate: false, supportsDuplicate: false },
  { key: "blood_type", label: m.registration_field_blood_type, isActive: true, isRequired: false, preventDuplicate: false, supportsDuplicate: false },
  { key: "emergency_contact", label: m.common_form_guardian_role_emergency, isActive: true, isRequired: true, preventDuplicate: false, supportsDuplicate: false },
  { key: "medical_notes", label: m.registration_field_medical_notes, isActive: true, isRequired: false, preventDuplicate: false, supportsDuplicate: false },
  { key: "previous_school", label: m.common_form_previous_school, isActive: true, isRequired: false, preventDuplicate: false, supportsDuplicate: false },
];

function parseOrgRegistration(rawOrg: any) {
  let settingsObj: any = {};
  try {
    settingsObj = typeof rawOrg?.settings === "string" ? JSON.parse(rawOrg.settings) : rawOrg?.settings || {};
  } catch {
    settingsObj = {};
  }

  const adm = settingsObj.admission || settingsObj.registration || settingsObj.registration_config || {};
  const enr = settingsObj.enrollment || {};
  const num = adm.numbering || {};
  const pol = adm.policy || adm.policy_settings || adm;
  const fields = adm.fields || adm.form_fields_list || [];

  return {
    institutionType: adm.institution_type || "school",
    studentNumberPrefix: num.prefix || adm.student_number_prefix || "STU",
    studentNumberYearFormat: num.year_format === "full" ? "YEAR" : num.year_format === "short" ? "YEAR2" : num.year_format === "none" ? "NONE" : (adm.student_number_year_format || "YEAR"),
    studentNumberDigits: num.sequence_digits ? Number(num.sequence_digits) : (adm.student_number_digits ? Number(adm.student_number_digits) : 4),
    registrationEnabled: enr.registration_enabled !== undefined ? Boolean(enr.registration_enabled) : true,
    requiresApproval: enr.require_admin_approval !== undefined ? Boolean(enr.require_admin_approval) : (pol.requires_approval !== undefined ? Boolean(pol.requires_approval) : true),
    allowWaitlist: enr.allow_waitlist !== undefined ? Boolean(enr.allow_waitlist) : (pol.allow_waitlist !== undefined ? Boolean(pol.allow_waitlist) : true),
    siblingPriorityEnabled: enr.sibling_priority_enabled !== undefined ? Boolean(enr.sibling_priority_enabled) : (pol.sibling_priority_enabled !== undefined ? Boolean(pol.sibling_priority_enabled) : true),
    allowLateEnrollment: enr.allow_late_enrollment !== undefined ? Boolean(enr.allow_late_enrollment) : (pol.allow_late_enrollment !== undefined ? Boolean(pol.allow_late_enrollment) : false),
    allowNoGuardian: pol.allow_no_guardian !== undefined ? Boolean(pol.allow_no_guardian) : false,
    allowDuplicateNationalId: pol.allow_duplicate_national_id !== undefined ? Boolean(pol.allow_duplicate_national_id) : false,
    autoAssignFees: pol.auto_assign_fees !== undefined ? Boolean(pol.auto_assign_fees) : true,
    maxClassCapacity: enr.max_class_size_default ? Number(enr.max_class_size_default) : (pol.max_class_capacity ? Number(pol.max_class_capacity) : 30),
    requiredDocuments: Array.isArray(adm.required_documents_list) ? adm.required_documents_list : [],
    formFields: Array.isArray(fields) && fields.length > 0
      ? fields.map((f: any) => ({
          key: f.key,
          nameAr: f.name_ar || f.nameAr || f.name,
          nameEn: f.name_en || f.nameEn || f.key,
          isActive: f.is_active !== undefined ? Boolean(f.is_active) : (f.isActive ?? true),
          isRequired: f.is_required !== undefined ? Boolean(f.is_required) : (f.isRequired ?? false),
          preventDuplicate: f.prevent_duplicate !== undefined ? Boolean(f.prevent_duplicate) : (f.preventDuplicate ?? false),
          supportsDuplicate: f.supports_duplicate !== undefined ? Boolean(f.supports_duplicate) : (f.supportsDuplicate ?? true),
        }))
      : defaultFormFields,
  };
}

const parsed = untrack(() => parseOrgRegistration(org));

// Editable Form State
let institutionType = $state<string>(parsed.institutionType);
let studentNumberPrefix = $state<string>(parsed.studentNumberPrefix);
let studentNumberYearFormat = $state<string>(parsed.studentNumberYearFormat);
let studentNumberDigits = $state<number>(parsed.studentNumberDigits);
let registrationEnabled = $state<boolean>(parsed.registrationEnabled);
let requiresApproval = $state<boolean>(parsed.requiresApproval);
let allowWaitlist = $state<boolean>(parsed.allowWaitlist);
let siblingPriorityEnabled = $state<boolean>(parsed.siblingPriorityEnabled);
let allowLateEnrollment = $state<boolean>(parsed.allowLateEnrollment);
let allowNoGuardian = $state<boolean>(parsed.allowNoGuardian);
let allowDuplicateNationalId = $state<boolean>(parsed.allowDuplicateNationalId);
let autoAssignFees = $state<boolean>(parsed.autoAssignFees);
let maxClassCapacity = $state<number>(parsed.maxClassCapacity);
let requiredDocuments = $state<DocItem[]>(JSON.parse(JSON.stringify(parsed.requiredDocuments)));
let formFields = $state<FieldConfig[]>(parsed.formFields.map((field) => ({ ...field })));

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
  registrationEnabled: parsed.registrationEnabled,
  requiresApproval: parsed.requiresApproval,
  allowWaitlist: parsed.allowWaitlist,
  siblingPriorityEnabled: parsed.siblingPriorityEnabled,
  allowLateEnrollment: parsed.allowLateEnrollment,
  allowNoGuardian: parsed.allowNoGuardian,
  allowDuplicateNationalId: parsed.allowDuplicateNationalId,
  autoAssignFees: parsed.autoAssignFees,
  maxClassCapacity: parsed.maxClassCapacity,
  requiredDocuments: JSON.parse(JSON.stringify(parsed.requiredDocuments)),
  formFields: parsed.formFields.map((field) => ({ ...field })),
};

const isDirty = $derived.by(() => {
  return (
    institutionType !== initialState.institutionType ||
    studentNumberPrefix !== initialState.studentNumberPrefix ||
    studentNumberYearFormat !== initialState.studentNumberYearFormat ||
    studentNumberDigits !== initialState.studentNumberDigits ||
    registrationEnabled !== initialState.registrationEnabled ||
    requiresApproval !== initialState.requiresApproval ||
    allowWaitlist !== initialState.allowWaitlist ||
    siblingPriorityEnabled !== initialState.siblingPriorityEnabled ||
    allowLateEnrollment !== initialState.allowLateEnrollment ||
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
    const [backendReqs, classesList, currentSettings] = await Promise.all([
      api.listDocumentRequirements().catch(() => []),
      api.listClasses().catch(() => []),
      api.getSettings().catch(() => null),
    ]);

    if (classesList && Array.isArray(classesList)) {
      dbClasses = classesList;
    }

    if (currentSettings) {
      if (currentSettings.enrollment) {
        const e = currentSettings.enrollment;
        if (e.registration_enabled !== undefined) registrationEnabled = Boolean(e.registration_enabled);
        if (e.require_admin_approval !== undefined) requiresApproval = Boolean(e.require_admin_approval);
        if (e.allow_waitlist !== undefined) allowWaitlist = Boolean(e.allow_waitlist);
        if (e.sibling_priority_enabled !== undefined) siblingPriorityEnabled = Boolean(e.sibling_priority_enabled);
        if (e.max_class_size_default !== undefined) maxClassCapacity = Number(e.max_class_size_default);
        if (e.allow_late_enrollment !== undefined) allowLateEnrollment = Boolean(e.allow_late_enrollment);
      }
      if (currentSettings.admission) {
        const a = currentSettings.admission;
        if (a.institution_type) institutionType = a.institution_type;
        if (a.numbering?.prefix) studentNumberPrefix = a.numbering.prefix;
        if (a.numbering?.sequence_digits) studentNumberDigits = Number(a.numbering.sequence_digits);
        if (a.numbering?.year_format) {
          studentNumberYearFormat = a.numbering.year_format === "full" ? "YEAR" : a.numbering.year_format === "short" ? "YEAR2" : "NONE";
        }
        if (a.policy) {
          if (a.policy.requires_approval !== undefined) requiresApproval = Boolean(a.policy.requires_approval);
          if (a.policy.allow_no_guardian !== undefined) allowNoGuardian = Boolean(a.policy.allow_no_guardian);
          if (a.policy.allow_duplicate_national_id !== undefined) allowDuplicateNationalId = Boolean(a.policy.allow_duplicate_national_id);
          if (a.policy.auto_assign_fees !== undefined) autoAssignFees = Boolean(a.policy.auto_assign_fees);
          if (a.policy.max_class_capacity !== undefined) maxClassCapacity = Number(a.policy.max_class_capacity);
          if (a.policy.allow_waitlist !== undefined) allowWaitlist = Boolean(a.policy.allow_waitlist);
          if (a.policy.sibling_priority_enabled !== undefined) siblingPriorityEnabled = Boolean(a.policy.sibling_priority_enabled);
          if (a.policy.allow_late_enrollment !== undefined) allowLateEnrollment = Boolean(a.policy.allow_late_enrollment);
        }
      }
      // Re-synchronize initial state
      initialState.institutionType = institutionType;
      initialState.studentNumberPrefix = studentNumberPrefix;
      initialState.studentNumberYearFormat = studentNumberYearFormat;
      initialState.studentNumberDigits = studentNumberDigits;
      initialState.registrationEnabled = registrationEnabled;
      initialState.requiresApproval = requiresApproval;
      initialState.allowWaitlist = allowWaitlist;
      initialState.siblingPriorityEnabled = siblingPriorityEnabled;
      initialState.allowLateEnrollment = allowLateEnrollment;
      initialState.allowNoGuardian = allowNoGuardian;
      initialState.allowDuplicateNationalId = allowDuplicateNationalId;
      initialState.autoAssignFees = autoAssignFees;
      initialState.maxClassCapacity = maxClassCapacity;
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
      initialState.requiredDocuments = JSON.parse(JSON.stringify(mapped));
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
  registrationEnabled = initialState.registrationEnabled;
  requiresApproval = initialState.requiresApproval;
  allowWaitlist = initialState.allowWaitlist;
  siblingPriorityEnabled = initialState.siblingPriorityEnabled;
  allowLateEnrollment = initialState.allowLateEnrollment;
  allowNoGuardian = initialState.allowNoGuardian;
  allowDuplicateNationalId = initialState.allowDuplicateNationalId;
  autoAssignFees = initialState.autoAssignFees;
  maxClassCapacity = initialState.maxClassCapacity;
  requiredDocuments = JSON.parse(JSON.stringify(initialState.requiredDocuments));
  formFields = initialState.formFields.map((field) => ({ ...field }));
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
  if (selectedClassFilter === "all" || !selectedClassFilter) return requiredDocuments;
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

// ── OrbisGrid Source & Columns for Document Requirements Studio ──
const documentActions: ActionItem[] = [
  { id: "toggle-mandatory", label: m.ui_copy_toggle_mandatory_5b28c7c2(), icon: "CheckCircle" },
  { id: "toggle-blocks", label: m.ui_copy_toggle_blocks_9f3655f1(), icon: "Lock" },
  { id: "delete", label: m.common_action_delete(), icon: "Trash2", variant: "destructive" },
];

const docGridSource = $derived(
  filteredDocuments.map((doc, idx) => {
    const mappedClass = dbClasses.find(c => String(c.id) === doc.gradeLevelId);
    return {
      ...doc,
      __idx: idx + 1,
      doc_name: l(doc.nameAr, doc.nameEn),
      class_name: mappedClass ? (mappedClass.name || mappedClass.level) : m.ui_copy_all_classes_c890049c(),
      condition_label: doc.condition === "all_students"
        ? m.ui_copy_all_students_be94120b()
        : doc.condition === "national_only"
        ? m.ui_copy_nationals_only_f7853fd7()
        : doc.condition === "foreign_only"
        ? m.ui_copy_expats_only_714a26ad()
        : doc.condition === "transfer_students"
        ? m.ui_copy_transfers_452455ee()
        : doc.condition,
      is_required_display: doc.isRequired ? m.ui_copy_mandatory_5ceed842() : m.ui_onboarding_progress_optional(),
      blocks_display: doc.blocksEnrollment ? m.ui_copy_blocks_1362af09() : m.ui_copy_no_block_8cdb8971(),
    };
  })
);

const docGridColumns = $derived<OrbisColumnDef[]>([
  { id: "__idx", prop: "__idx", header: "#", name: "#", width: 48, size: 48, readonly: true, align: "center" },
  { id: "doc_name", prop: "doc_name", header: m.ui_copy_document_title_12e6b59a(), name: m.ui_copy_document_title_12e6b59a(), width: 220, size: 220, readonly: true, cellClass: "font-semibold" },
  { id: "code", prop: "code", header: m.settings_units_column_code(), name: m.settings_units_column_code(), width: 110, size: 110, readonly: true, cellClass: "font-mono text-xs text-muted-foreground" },
  { id: "class_name", prop: "class_name", header: m.ui_copy_class_grade_fe0bb7cd(), name: m.ui_copy_class_grade_fe0bb7cd(), width: 150, size: 150, readonly: true },
  { id: "condition_label", prop: "condition_label", header: m.ui_copy_condition_401a3f12(), name: m.ui_copy_condition_401a3f12(), width: 130, size: 130, readonly: true },
  { id: "is_required_display", prop: "is_required_display", header: m.ui_copy_mandatory_d1edb458(), name: m.ui_copy_mandatory_d1edb458(), width: 110, size: 110, type: "badge", align: "center", readonly: true },
  { id: "blocks_display", prop: "blocks_display", header: m.ui_copy_blocks_d807c44d(), name: m.ui_copy_blocks_d807c44d(), width: 110, size: 110, type: "badge", align: "center", readonly: true },
  {
    id: "actions",
    prop: "actions",
    header: m.ui_copy_actions_c48127a5(),
    name: m.ui_copy_actions_c48127a5(),
    width: 140,
    size: 140,
    type: "action",
    align: "center",
    readonly: true,
    meta: { items: documentActions },
  },
]);

function handleDocGridAction(event: GridActionEvent) {
  const row = event.row as unknown as DocItem;
  if (!row) return;

  if (event.actionId === "toggle-mandatory") {
    requiredDocuments = requiredDocuments.map(d => d.id === row.id || d.code === row.code ? { ...d, isRequired: !d.isRequired } : d);
  } else if (event.actionId === "toggle-blocks") {
    requiredDocuments = requiredDocuments.map(d => d.id === row.id || d.code === row.code ? { ...d, blocksEnrollment: !d.blocksEnrollment } : d);
  } else if (event.actionId === "delete") {
    removeDocument(row);
  }
}

// ── OrbisGrid Source & Columns for Form Fields Matrix ──
const fieldActions: ActionItem[] = [
  { id: "toggle-active", label: m.ui_copy_toggle_active_af75a006() },
  { id: "toggle-required", label: m.ui_copy_toggle_required_a6afbcac() },
  { id: "toggle-duplicate", label: m.ui_copy_toggle_unique_aa5c4014() },
];

const fieldGridSource = $derived(
  formFields.map((field, idx) => ({
    ...field,
    __idx: idx + 1,
    field_name: field.label?.() ?? l(field.nameAr ?? "", field.nameEn ?? ""),
    active_display: field.isActive ? m.ui_copy_active_60ced915() : m.academics_exams_center_disabled(),
    required_display: field.isActive ? (field.isRequired ? m.ui_copy_required_1c464044() : m.ui_onboarding_progress_optional()) : "—",
    duplicate_display: field.supportsDuplicate ? (field.preventDuplicate ? m.ui_copy_strict_unique_be0bf346() : m.ui_copy_allow_duplicate_9bdc69d7()) : "—",
  }))
);

const fieldGridColumns = $derived<OrbisColumnDef[]>([
  { id: "__idx", prop: "__idx", header: "#", name: "#", width: 48, size: 48, readonly: true, align: "center" },
  { id: "field_name", prop: "field_name", header: m.ui_copy_field_name_b04bb75b(), name: m.ui_copy_field_name_b04bb75b(), width: 220, size: 220, readonly: true, cellClass: "font-semibold" },
  { id: "key", prop: "key", header: m.ui_copy_key_813ba219(), name: m.ui_copy_key_813ba219(), width: 160, size: 160, readonly: true, cellClass: "font-mono text-xs text-muted-foreground" },
  { id: "active_display", prop: "active_display", header: m.common_status(), name: m.common_status(), width: 130, size: 130, type: "badge", align: "center", readonly: true },
  { id: "required_display", prop: "required_display", header: m.ui_onboarding_progress_required_2(), name: m.ui_onboarding_progress_required_2(), width: 130, size: 130, type: "badge", align: "center", readonly: true },
  { id: "duplicate_display", prop: "duplicate_display", header: m.ui_copy_unique_c4f61bab(), name: m.ui_copy_unique_c4f61bab(), width: 140, size: 140, type: "badge", align: "center", readonly: true },
  {
    id: "actions",
    prop: "actions",
    header: m.ui_copy_settings_actions_fc61aaf9(),
    name: m.ui_copy_settings_actions_fc61aaf9(),
    width: 150,
    size: 150,
    type: "action",
    align: "center",
    readonly: true,
    meta: { items: fieldActions },
  },
]);

function handleFieldGridAction(event: GridActionEvent) {
  const row = event.row as unknown as FieldConfig;
  if (!row) return;

  if (event.actionId === "toggle-active") {
    formFields = formFields.map(f => f.key === row.key ? { ...f, isActive: !f.isActive } : f);
  } else if (event.actionId === "toggle-required") {
    formFields = formFields.map(f => f.key === row.key ? { ...f, isRequired: !f.isRequired } : f);
  } else if (event.actionId === "toggle-duplicate") {
    if (row.supportsDuplicate) {
      formFields = formFields.map(f => f.key === row.key ? { ...f, preventDuplicate: !f.preventDuplicate } : f);
    }
  }
}

// Save form
function handleFormSave() {
  const payload = {
    admission: {
      institution_type: institutionType,
      numbering: {
        prefix: studentNumberPrefix,
        year_format: studentNumberYearFormat === "YEAR" ? "full" : studentNumberYearFormat === "YEAR2" ? "short" : "none",
        sequence_digits: studentNumberDigits,
        separator: "-",
      },
      policy: {
        requires_approval: requiresApproval,
        allow_no_guardian: allowNoGuardian,
        allow_duplicate_national_id: allowDuplicateNationalId,
        auto_assign_fees: autoAssignFees,
        max_class_capacity: maxClassCapacity,
        allow_late_enrollment: allowLateEnrollment,
        allow_waitlist: allowWaitlist,
        sibling_priority_enabled: siblingPriorityEnabled,
      },
      fields: formFields.map((f) => ({
        key: f.key,
        name_ar: f.nameAr,
        name_en: f.nameEn,
        is_active: f.isActive,
        is_required: f.isRequired,
        prevent_duplicate: f.preventDuplicate,
        supports_duplicate: f.supportsDuplicate,
      })),
    },
    enrollment: {
      registration_enabled: registrationEnabled,
      require_admin_approval: requiresApproval,
      allow_waitlist: allowWaitlist,
      sibling_priority_enabled: siblingPriorityEnabled,
      max_class_size_default: maxClassCapacity,
      allow_late_enrollment: allowLateEnrollment,
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
            {m.ui_copy_educational_institution_type_044c6d34()}
          </CardTitle>
          <CardDescription>
            {m.ui_copy_customize_admission_and_numbering_schemas__6e581efd()}
          </CardDescription>
        </div>
      </div>
      <CardAction>
        <Badge variant="outline" class="px-3 py-1 h-7 text-xs font-bold gap-1.5 bg-muted/40 border-border text-foreground">
          <Lock class="w-3 h-3 text-muted-foreground" />
          <span>
            {#if institutionType === "school"}
              {m.ui_copy_k_12_school_cd02a44e()}
            {:else if institutionType === "university"}
              {m.ui_copy_university_college_5c444bdf()}
            {:else if institutionType === "institute"}
              {m.ui_copy_training_institute_b9a51493()}
            {:else if institutionType === "quran_center"}
              {m.ui_copy_quranic_center_4c9e12cc()}
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
        <span>{m.ui_copy_admission_numbering_eca11847()}</span>
      </TabsTrigger>
      <TabsTrigger value="documents" class="text-xs font-semibold gap-1.5 px-4 h-7">
        <DocumentCheck class="w-3.5 h-3.5" />
        <span>{m.ui_copy_class_requirements_studio_f2d57d31()}</span>
        <Badge variant="secondary" class="ms-1 px-1.5 py-0 text-[10px] font-mono">
          {requiredDocuments.length}
        </Badge>
      </TabsTrigger>
      <TabsTrigger value="fields" class="text-xs font-semibold gap-1.5 px-4 h-7">
        <AdjustmentsHorizontal class="w-3.5 h-3.5" />
        <span>{m.ui_copy_form_fields_matrix_960721da()}</span>
      </TabsTrigger>
    </TabsList>

    <!-- TAB 1: Policies & Student Numbering -->
    <TabsContent value="policies" class="space-y-6 pt-4">
      <!-- Student Numbering Card -->
      <Card class="border-border/80 shadow-xs">
        <CardHeader>
          <div class="flex flex-col text-start">
            <CardTitle>
              {m.ui_copy_student_id_numbering_format_347fd4b4()}
            </CardTitle>
            <CardDescription>
              {m.ui_copy_automated_sequence_schema_for_new_admissio_62932f38()}
            </CardDescription>
          </div>
          <CardAction>
            <div class="flex items-center gap-2">
              <span class="text-xs text-muted-foreground">{m.ui_copy_preview_e0036935()}</span>
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
              {m.ui_copy_prefix_9af3f0d2()}
            </Label>
            <Input
              bind:value={studentNumberPrefix}
              placeholder={m.settings_ui_student_number_prefix_example()}
              class="h-9 text-xs font-mono uppercase"
            />
          </div>

          <!-- Year Token Format -->
          <div class="flex flex-col gap-1.5">
            <Label class="text-xs font-semibold text-foreground">
              {m.ui_copy_include_year_fa9e283c()}
            </Label>
            <Select
              type="single"
              value={studentNumberYearFormat}
              onValueChange={(v) => (studentNumberYearFormat = v ?? "YEAR")}
            >
              <SelectTrigger class="h-9 text-xs">
                {studentNumberYearFormat === "YEAR"
                  ? m.ui_copy_full_4_digit_year_2026_3a1eaa6a()
                  : studentNumberYearFormat === "YEAR2"
                  ? m.ui_copy_short_2_digit_year_26_5f63ba6d()
                  : m.ui_copy_no_year_92c0ace0()}
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="YEAR" class="text-xs">{m.ui_copy_full_4_digit_year_2026_3a1eaa6a()}</SelectItem>
                <SelectItem value="YEAR2" class="text-xs">{m.ui_copy_short_2_digit_year_26_5f63ba6d()}</SelectItem>
                <SelectItem value="NONE" class="text-xs">{m.ui_copy_no_year_92c0ace0()}</SelectItem>
              </SelectContent>
            </Select>
          </div>

          <!-- Digits Sequence -->
          <div class="flex flex-col gap-1.5">
            <Label class="text-xs font-semibold text-foreground">
              {m.ui_copy_sequence_digits_8d44beda()}
            </Label>
            <Select
              type="single"
              value={String(studentNumberDigits)}
              onValueChange={(v) => (studentNumberDigits = Number(v || 4))}
            >
              <SelectTrigger class="h-9 text-xs font-mono">
                {studentNumberDigits} {m.ui_copy_digits_2a437f20()} (مثال: {"1".padStart(studentNumberDigits, "0")})
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="3" class="text-xs font-mono">3 {m.ui_copy_digits_2a437f20()} (001)</SelectItem>
                <SelectItem value="4" class="text-xs font-mono">4 {m.ui_copy_digits_2a437f20()} (0001)</SelectItem>
                <SelectItem value="5" class="text-xs font-mono">5 {m.ui_copy_digits_2a437f20()} (00001)</SelectItem>
                <SelectItem value="6" class="text-xs font-mono">6 {m.ui_copy_digits_2a437f20()} (000001)</SelectItem>
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
              {m.ui_copy_admission_enrollment_governance_rules_71d20dd7()}
            </CardTitle>
            <CardDescription>
              {m.ui_copy_manage_registration_portals_approval_workf_a333aae0()}
            </CardDescription>
          </div>
        </CardHeader>

        <CardContent class="p-6 flex flex-col divide-y divide-border">
          <!-- Enable Registration Portal -->
          <div class="py-3 flex items-center justify-between gap-4 first:pt-0">
            <div class="flex flex-col gap-0.5 text-start">
              <span class="text-xs font-bold text-foreground">
                {m.ui_copy_enable_registration_portal_dbaf8a8e()}
              </span>
              <span class="text-[11px] text-muted-foreground">
                {m.ui_copy_allow_incoming_student_admission_requests__216e66ac()}
              </span>
            </div>
            <Switch bind:checked={registrationEnabled} />
          </div>

          <!-- Approval Workflow -->
          <div class="py-3 flex items-center justify-between gap-4">
            <div class="flex flex-col gap-0.5 text-start">
              <span class="text-xs font-bold text-foreground">
                {m.ui_copy_require_academic_registrar_admin_approval_b147c1b8()}
              </span>
              <span class="text-[11px] text-muted-foreground">
                {m.ui_copy_new_applicants_remain_pending_approval_bef_beccd042()}
              </span>
            </div>
            <Switch bind:checked={requiresApproval} />
          </div>

          <!-- Allow Waitlist -->
          <div class="py-3 flex items-center justify-between gap-4">
            <div class="flex flex-col gap-0.5 text-start">
              <span class="text-xs font-bold text-foreground">
                {m.ui_copy_enable_waitlist_on_capacity_189ed812()}
              </span>
              <span class="text-[11px] text-muted-foreground">
                {m.ui_copy_place_students_on_waitlist_when_classes_re_fb2f377c()}
              </span>
            </div>
            <Switch bind:checked={allowWaitlist} />
          </div>

          <!-- Sibling Priority -->
          <div class="py-3 flex items-center justify-between gap-4">
            <div class="flex flex-col gap-0.5 text-start">
              <span class="text-xs font-bold text-foreground">
                {m.ui_copy_sibling_priority_enrollment_b6d49ba5()}
              </span>
              <span class="text-[11px] text-muted-foreground">
                {m.ui_copy_prioritize_siblings_of_already_enrolled_st_29bdbb54()}
              </span>
            </div>
            <Switch bind:checked={siblingPriorityEnabled} />
          </div>

          <!-- Allow Late Enrollment -->
          <div class="py-3 flex items-center justify-between gap-4">
            <div class="flex flex-col gap-0.5 text-start">
              <span class="text-xs font-bold text-foreground">
                {m.ui_copy_allow_late_enrollment_06114ddb()}
              </span>
              <span class="text-[11px] text-muted-foreground">
                {m.ui_copy_accept_enrollment_applications_after_perio_59f54c88()}
              </span>
            </div>
            <Switch bind:checked={allowLateEnrollment} />
          </div>

          <!-- Allow No Guardian -->
          <div class="py-3 flex items-center justify-between gap-4">
            <div class="flex flex-col gap-0.5 text-start">
              <span class="text-xs font-bold text-foreground">
                {m.ui_copy_allow_independent_enrollment_no_guardian_7ce5a41b()}
              </span>
              <span class="text-[11px] text-muted-foreground">
                {m.ui_copy_applicable_for_adult_university_and_vocati_310c0145()}
              </span>
            </div>
            <Switch bind:checked={allowNoGuardian} />
          </div>

          <!-- Prevent Duplicate National ID -->
          <div class="py-3 flex items-center justify-between gap-4">
            <div class="flex flex-col gap-0.5 text-start">
              <span class="text-xs font-bold text-foreground">
                {m.ui_copy_strict_unique_national_id_enforcement_7efa5e55()}
              </span>
              <span class="text-[11px] text-muted-foreground">
                {m.ui_copy_rejects_duplicate_national_id_entries_acro_410471c5()}
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
                {m.ui_copy_auto_assign_tuition_fee_template_on_enroll_5ed6c3b1()}
              </span>
              <span class="text-[11px] text-muted-foreground">
                {m.ui_copy_automatically_maps_default_fee_templates_b_4e4adc5f()}
              </span>
            </div>
            <Switch bind:checked={autoAssignFees} />
          </div>

          <!-- Max Capacity -->
          <div class="py-3 flex items-center justify-between gap-4 last:pb-0">
            <div class="flex flex-col gap-0.5 text-start">
              <span class="text-xs font-bold text-foreground">
                {m.ui_copy_default_class_capacity_limit_700d9cac()}
              </span>
              <span class="text-[11px] text-muted-foreground">
                {m.ui_copy_maximum_enrollment_limit_per_classroom_sec_a7d094ee()}
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

    <!-- TAB 2: Class/Grade Requirements Studio with OrbisGrid -->
    <TabsContent value="documents" class="space-y-4 pt-4">
      <!-- Studio Header Actions Bar -->
      <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
        <div class="flex items-center gap-2">
          <Funnel class="w-4 h-4 text-muted-foreground" />
          <span class="text-xs font-semibold text-muted-foreground">{m.ui_copy_filter_class_grade_a161d4ac()}</span>
          <div class="w-56">
            <ClassSelect
              bind:value={selectedClassFilter}
              showAllOption={true}
              allOptionLabel={m.ui_copy_all_classes_grades_b92c9849()}
              placeholder={m.ui_copy_all_classes_grades_b92c9849()}
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
          <span>{m.ui_copy_add_document_requirement_d6a40d4d()}</span>
        </Button>
      </div>

      <!-- Requirements OrbisGrid Container -->
      <div class="rounded-xl border border-border/80 bg-background overflow-hidden">
        <OrbisGrid
          columns={docGridColumns}
          source={docGridSource}
          height="min(55vh, 480px)"
          rowSize={44}
          rtl={true}
          onaction={handleDocGridAction}
          emptyTitle={m.ui_copy_no_document_requirements_configured_07292a9d()}
        />
      </div>
    </TabsContent>

    <!-- TAB 3: Form Fields Matrix with OrbisGrid -->
    <TabsContent value="fields" class="space-y-4 pt-4">
      <div class="rounded-xl border border-border/80 bg-background overflow-hidden">
        <OrbisGrid
          columns={fieldGridColumns}
          source={fieldGridSource}
          height="min(60vh, 520px)"
          rowSize={44}
          rtl={true}
          onaction={handleFieldGridAction}
          emptyTitle={m.ui_copy_no_form_fields_79d7bef7()}
        />
      </div>
    </TabsContent>
  </Tabs>

  <!-- Add Document Requirement Slide-over Sheet -->
  <AddDocumentRequirementSheet
    bind:open={isAddDocSheetOpen}
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
