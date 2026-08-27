<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import { Checkbox } from "@orbis/ui-svelte/checkbox";
import { Switch } from "@orbis/ui-svelte/switch";
import { Badge } from "@orbis/ui-svelte/badge";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@orbis/ui-svelte/table";
import {
  Tabs,
  TabsList,
  TabsTrigger,
  TabsContent,
} from "@orbis/ui-svelte/tabs";
import { Plus, Trash2, Sparkles, ShieldCheck, Layers, BookOpen } from "@lucide/svelte";
import { toast } from "svelte-sonner";

import { getLocale } from "$lib/paraglide/runtime";
import { registrationSettingsSchema } from "@orbis/schemas";
import { INSTITUTION_PROFILES } from "$lib/features/onboarding/templates";
import SettingsBottomBar from "$lib/shared/components/settings-bottom-bar.svelte";

let { org, isSaving, onsave } = $props<{
  org: any;
  isSaving: boolean;
  onsave: (registrationConfig: any) => void;
}>();

// Form state initialized once with props
let institutionType = $state("school");
let studentNumberPattern = $state("STU-{YEAR}-{SEQ}");
let rawStages = $state<string[]>(["primary", "middle", "high"]);
let stageInput = $state("");
let requiredDocuments = $state<string[]>([
  "Birth Certificate",
  "National ID or Passport",
]);
let documentInput = $state("");

// Policy settings state
let requiresApproval = $state(false);
let allowNoGuardian = $state(true);
let autoAssignFees = $state(true);
let allowDuplicateNationalId = $state(false);
let maxClassCapacity = $state<number | null>(null);

// Field configurations
let requiredFields = $state<Record<string, boolean>>({
  national_id: true,
  email: false,
  phone: true,
  birth_date: true,
  sex: true,
  address: false,
  nationality: false,
  place_of_birth: false,
  passport_number: false,
  blood_type: false,
  emergency_contact: true,
  medical_notes: false,
  previous_school: false,
  documents: true,
});

let activeFields = $state<Record<string, boolean>>({
  national_id: true,
  email: true,
  phone: true,
  birth_date: true,
  sex: true,
  address: true,
  nationality: true,
  place_of_birth: true,
  passport_number: true,
  blood_type: true,
  emergency_contact: true,
  medical_notes: true,
  previous_school: true,
  documents: true,
});

let preventDuplicates = $state<Record<string, boolean>>({
  national_id: true,
  email: false,
  phone: false,
});

const SYSTEM_FIELDS = [
  { key: "national_id", label: m.common_form_profile_nid_label(), supportsDuplicate: true },
  { key: "email", label: m.common_word_email(), supportsDuplicate: true },
  { key: "phone", label: m.common_word_phone(), supportsDuplicate: true },
  { key: "birth_date", label: m.common_bulk_col_dob(), supportsDuplicate: false },
  { key: "sex", label: m.common_form_sex(), supportsDuplicate: false },
  { key: "address", label: m.common_word_address(), supportsDuplicate: false },
  { key: "nationality", label: "الجنسية الرسمية", supportsDuplicate: false },
  { key: "place_of_birth", label: "مكان الميلاد", supportsDuplicate: false },
  { key: "passport_number", label: "رقم جواز السفر", supportsDuplicate: false },
  { key: "blood_type", label: "فصيلة الدم للطوارئ", supportsDuplicate: false },
  { key: "emergency_contact", label: "جهة الاتصال بالطوارئ", supportsDuplicate: false },
  { key: "medical_notes", label: "الملاحظات والبيانات الطبية", supportsDuplicate: false },
  { key: "previous_school", label: m.settings_registration_field_previous_school(), supportsDuplicate: false },
  { key: "documents", label: m.common_form_tab_documents(), supportsDuplicate: false },
];

let isInitialized = $state(false);
let initialSnapshot = $state("");

function getFormSnapshot() {
  return JSON.stringify({
    institutionType,
    studentNumberPattern,
    rawStages,
    requiredDocuments,
    requiresApproval,
    allowNoGuardian,
    autoAssignFees,
    allowDuplicateNationalId,
    maxClassCapacity,
    requiredFields,
    activeFields,
    preventDuplicates,
  });
}

const isDirty = $derived(
  isInitialized && initialSnapshot !== "" && getFormSnapshot() !== initialSnapshot,
);

function restoreSnapshot() {
  if (!initialSnapshot) return;
  const snapshot = JSON.parse(initialSnapshot);
  institutionType = snapshot.institutionType;
  studentNumberPattern = snapshot.studentNumberPattern;
  rawStages = snapshot.rawStages;
  requiredDocuments = snapshot.requiredDocuments;
  requiresApproval = snapshot.requiresApproval;
  allowNoGuardian = snapshot.allowNoGuardian;
  autoAssignFees = snapshot.autoAssignFees;
  allowDuplicateNationalId = snapshot.allowDuplicateNationalId;
  maxClassCapacity = snapshot.maxClassCapacity;
  requiredFields = snapshot.requiredFields;
  activeFields = snapshot.activeFields;
  preventDuplicates = snapshot.preventDuplicates;
}

$effect(() => {
  if (org && !isInitialized) {
    isInitialized = true;
    let settingsObj: any = {};
    try {
      settingsObj =
        typeof org.settings === "string"
          ? JSON.parse(org.settings)
          : org.settings || {};
    } catch {}
    
    // Single Source of Truth: org.settings.registration (fallback to registration_config if legacy data)
    const reg = settingsObj.registration || settingsObj.registration_config || {};

    if (reg.institution_type) institutionType = reg.institution_type;
    if (reg.student_number_pattern) studentNumberPattern = reg.student_number_pattern;
    if (Array.isArray(reg.stages)) rawStages = reg.stages;
    if (Array.isArray(reg.required_documents)) requiredDocuments = reg.required_documents;

    if (reg.active_fields) {
      if (Array.isArray(reg.active_fields)) {
        const actMap: Record<string, boolean> = {};
        for (const k of reg.active_fields) actMap[k] = true;
        activeFields = { ...activeFields, ...actMap };
      } else {
        activeFields = { ...activeFields, ...reg.active_fields };
      }
    }

    if (reg.required_fields) {
      if (Array.isArray(reg.required_fields)) {
        const reqMap: Record<string, boolean> = {};
        for (const k of reg.required_fields) reqMap[k] = true;
        requiredFields = { ...requiredFields, ...reqMap };
      } else {
        requiredFields = { ...requiredFields, ...reg.required_fields };
      }
    }

    if (reg.prevent_duplicates) {
      if (Array.isArray(reg.prevent_duplicates)) {
        const dupMap: Record<string, boolean> = {};
        for (const k of reg.prevent_duplicates) dupMap[k] = true;
        preventDuplicates = { ...preventDuplicates, ...dupMap };
      } else {
        preventDuplicates = { ...preventDuplicates, ...reg.prevent_duplicates };
      }
    }

    const pol = reg.policy_settings || reg;
    if (pol.requires_approval !== undefined) requiresApproval = !!pol.requires_approval;
    if (pol.allow_no_guardian !== undefined) allowNoGuardian = !!pol.allow_no_guardian;
    if (pol.auto_assign_fees !== undefined) autoAssignFees = pol.auto_assign_fees !== false;
    if (pol.allow_duplicate_national_id !== undefined) allowDuplicateNationalId = !!pol.allow_duplicate_national_id;
    if (pol.max_class_capacity !== undefined) maxClassCapacity = pol.max_class_capacity == null ? null : Number(pol.max_class_capacity);

    initialSnapshot = getFormSnapshot();
  }
});

const currentProfile = $derived(INSTITUTION_PROFILES[institutionType] || INSTITUTION_PROFILES.school);

function applySmartPreset(type?: string) {
  const targetType = type || institutionType;
  const profile = INSTITUTION_PROFILES[targetType];
  if (!profile) return;

  institutionType = targetType;
  studentNumberPattern = profile.numberingPattern || "STU-{YEAR}-{SEQ}";
  rawStages = profile.stages.map((s) => s.name_ar || s.name);
  requiredDocuments = [...profile.requiredDocuments];
  activeFields = { ...activeFields, ...profile.activeFields };
  requiredFields = { ...requiredFields, ...profile.requiredFields };
  if (profile.preventDuplicates) {
    preventDuplicates = { ...preventDuplicates, ...profile.preventDuplicates };
  }
  if (profile.policySettings) {
    requiresApproval = profile.policySettings.requires_approval;
    allowNoGuardian = profile.policySettings.allow_no_guardian;
    autoAssignFees = profile.policySettings.auto_assign_fees;
    allowDuplicateNationalId = profile.policySettings.allow_duplicate_national_id;
  }

  toast.success(`تم تطبيق الإعدادات الموصى بها لـ (${profile.name_ar}) بنجاح`);
}

function addStage() {
  const clean = stageInput.trim();
  if (clean && !rawStages.includes(clean)) {
    rawStages = [...rawStages, clean];
    stageInput = "";
  }
}

function removeStage(stage: string) {
  rawStages = rawStages.filter((s) => s !== stage);
}

function addDocument(docName?: string) {
  const clean = (docName || documentInput).trim();
  if (clean && !requiredDocuments.includes(clean)) {
    requiredDocuments = [...requiredDocuments, clean];
    if (!docName) documentInput = "";
  }
}

function removeDocument(doc: string) {
  requiredDocuments = requiredDocuments.filter((d) => d !== doc);
}

function handleSubmit(e?: Event) {
  e?.preventDefault();

  const activeFieldsArray = Object.keys(activeFields).filter((k) => activeFields[k]);
  const requiredFieldsArray = Object.keys(requiredFields).filter((k) => requiredFields[k] && activeFields[k]);
  const preventDuplicatesArray = Object.keys(preventDuplicates).filter((k) => preventDuplicates[k] && activeFields[k]);

  const registrationSettings = {
    institution_type: institutionType,
    student_number_pattern: studentNumberPattern,
    stages: rawStages,
    active_fields: activeFieldsArray,
    required_fields: requiredFieldsArray,
    prevent_duplicates: preventDuplicatesArray,
    required_documents: requiredDocuments,
    student_number_prefix: studentNumberPattern.split("-")[0] || "STU",
    student_number_digits: 6,
    requires_approval: requiresApproval,
    max_class_capacity: maxClassCapacity,
    allow_no_guardian: allowNoGuardian,
    allow_duplicate_national_id: allowDuplicateNationalId,
    auto_assign_fees: autoAssignFees,
    default_fee_template_ids: [],
  };

  const result = registrationSettingsSchema.safeParse(registrationSettings);
  if (!result.success) {
    toast.error("تحقق من قواعد التسجيل والحقول المطلوبة");
    return;
  }

  onsave({
    registration: result.data,
  });
}
</script>

<div class="h-full w-full overflow-y-auto scrollbar-hide text-start text-foreground">
  <div class="max-w-7xl mx-auto p-6 flex flex-col gap-6">
    <form onsubmit={handleSubmit} class="flex flex-col gap-6">
      <Tabs value="general" class="w-full">
        <TabsList class="flex justify-start border-b border-border bg-transparent gap-6 rounded-none p-0 h-auto mb-6">
          <TabsTrigger 
            value="general" 
            class="text-xs font-semibold py-2 px-1 border-b-2 border-transparent data-[state=active]:border-primary data-[state=active]:bg-transparent rounded-none shadow-none text-muted-foreground data-[state=active]:text-foreground bg-transparent transition-all duration-150 cursor-pointer"
          >
            {m.settings_registration_tab_general()}
          </TabsTrigger>
          <TabsTrigger 
            value="stages_docs" 
            class="text-xs font-semibold py-2 px-1 border-b-2 border-transparent data-[state=active]:border-primary data-[state=active]:bg-transparent rounded-none shadow-none text-muted-foreground data-[state=active]:text-foreground bg-transparent transition-all duration-150 cursor-pointer"
          >
            {m.settings_registration_tab_stages_docs()}
          </TabsTrigger>
          <TabsTrigger 
            value="fields" 
            class="text-xs font-semibold py-2 px-1 border-b-2 border-transparent data-[state=active]:border-primary data-[state=active]:bg-transparent rounded-none shadow-none text-muted-foreground data-[state=active]:text-foreground bg-transparent transition-all duration-150 cursor-pointer"
          >
            {m.settings_registration_tab_fields()}
          </TabsTrigger>
          <TabsTrigger 
            value="policies" 
            class="text-xs font-semibold py-2 px-1 border-b-2 border-transparent data-[state=active]:border-primary data-[state=active]:bg-transparent rounded-none shadow-none text-muted-foreground data-[state=active]:text-foreground bg-transparent transition-all duration-150 cursor-pointer"
          >
            سياسات وقواعد التسجيل
          </TabsTrigger>
        </TabsList>

        <!-- TAB 1: General Settings -->
        <TabsContent value="general" class="mt-0 flex flex-col gap-6">
          <div class="border border-border rounded-md overflow-hidden flex flex-col bg-card/10">
            <div class="bg-muted/50 px-6 py-3 border-b border-border text-[11px] font-bold text-muted-foreground tracking-wider uppercase text-start select-none">
              {m.settings_registration_type_and_numbering()}
            </div>

            <div class="flex flex-col">
              <!-- Institution Type -->
              <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
                <div class="flex flex-col gap-1 text-start">
                  <Label for="inst_type" class="text-xs font-semibold text-foreground">
                    {m.common_registration_inst_type()}
                  </Label>
                  <span class="text-[11px] text-muted-foreground leading-relaxed">
                    {m.settings_registration_inst_desc()}
                  </span>
                </div>
                <div class="w-full md:w-[320px] shrink-0">
                  <Select bind:value={institutionType} type="single" onValueChange={(val) => val && applySmartPreset(val)}>
                    <SelectTrigger id="inst_type" class="h-9 text-xs flex justify-between items-center border border-input bg-background w-full">
                      <span>
                        {institutionType === "school" ? m.settings_registration_school() : 
                         institutionType === "university" ? m.settings_registration_university() :
                         institutionType === "institute" ? m.settings_registration_institute() :
                         m.settings_registration_training_center()}
                      </span>
                    </SelectTrigger>
                    <SelectContent class="text-start">
                      <SelectItem value="school">{m.settings_registration_school()}</SelectItem>
                      <SelectItem value="university">{m.settings_registration_university()}</SelectItem>
                      <SelectItem value="institute">{m.settings_registration_institute()}</SelectItem>
                      <SelectItem value="training_center">{m.settings_registration_training_center()}</SelectItem>
                    </SelectContent>
                  </Select>
                </div>
              </div>

              <!-- Student Number Pattern -->
              <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b-0 gap-4">
                <div class="flex flex-col gap-1 text-start">
                  <Label for="num_pattern" class="text-xs font-semibold text-foreground">
                    {m.settings_registration_num_pattern()}
                  </Label>
                  <span class="text-[11px] text-muted-foreground leading-relaxed">
                    {m.settings_registration_pattern_desc()}
                  </span>
                </div>
                <div class="w-full md:w-[320px] shrink-0">
                  <Input id="num_pattern" bind:value={studentNumberPattern} class="h-9 text-xs border border-input bg-background w-full font-mono" />
                </div>
              </div>
            </div>
          </div>

          <!-- Terminology Live Preview Card -->
          <div class="border border-border rounded-md overflow-hidden flex flex-col bg-card/10">
            <div class="bg-muted/50 px-6 py-3 border-b border-border text-[11px] font-bold text-muted-foreground tracking-wider uppercase text-start select-none flex items-center gap-2">
              <Layers class="h-3.5 w-3.5 text-primary" />
              <span>معاينة مصطلحات النظام لمؤسسة ({currentProfile.name_ar})</span>
            </div>
            <div class="p-6 grid grid-cols-2 md:grid-cols-4 gap-4 text-start">
              <div class="flex flex-col p-3 rounded-lg border border-border/60 bg-background/50 gap-1">
                <span class="text-[10px] text-muted-foreground">المرحلة / المستوى:</span>
                <span class="text-xs font-bold text-foreground">{currentProfile.terminology.stage_ar}</span>
              </div>
              <div class="flex flex-col p-3 rounded-lg border border-border/60 bg-background/50 gap-1">
                <span class="text-[10px] text-muted-foreground">الصف / القاعة:</span>
                <span class="text-xs font-bold text-foreground">{currentProfile.terminology.classroom_ar}</span>
              </div>
              <div class="flex flex-col p-3 rounded-lg border border-border/60 bg-background/50 gap-1">
                <span class="text-[10px] text-muted-foreground">المادة / المساق:</span>
                <span class="text-xs font-bold text-foreground">{currentProfile.terminology.subject_ar}</span>
              </div>
              <div class="flex flex-col p-3 rounded-lg border border-border/60 bg-background/50 gap-1">
                <span class="text-[10px] text-muted-foreground">الدورة / العام:</span>
                <span class="text-xs font-bold text-foreground">{currentProfile.terminology.calendar_ar}</span>
              </div>
            </div>
          </div>
        </TabsContent>

        <!-- TAB 2: Academic Stages & Required Documents -->
        <TabsContent value="stages_docs" class="mt-0 flex flex-col gap-6">
          <!-- Academic Stages -->
          <div class="border border-border rounded-md overflow-hidden flex flex-col bg-card/10">
            <div class="bg-muted/50 px-6 py-3 border-b border-border text-[11px] font-bold text-muted-foreground tracking-wider uppercase text-start select-none">
              {m.settings_registration_stages_title()} ({currentProfile.terminology.stage_ar})
            </div>

            <div class="flex flex-col px-6 py-4 gap-4">
              <div class="flex flex-col md:flex-row md:items-center justify-between gap-4">
                <div class="flex flex-col gap-1 text-start">
                  <Label class="text-xs font-semibold text-foreground">
                    {m.settings_registration_add_stage_title()}
                  </Label>
                  <span class="text-[11px] text-muted-foreground leading-relaxed">
                    {m.settings_registration_add_stage_desc()}
                  </span>
                </div>
                <div class="w-full md:w-[320px] shrink-0 flex gap-2">
                  <Input 
                    placeholder={m.settings_registration_add_stage_placeholder()} 
                    bind:value={stageInput} 
                    class="h-9 text-xs flex-1 border border-input bg-background"
                    onkeydown={(e) => e.key === "Enter" && (e.preventDefault(), addStage())}
                  />
                  <Button type="button" variant="outline" size="sm" class="cursor-pointer font-semibold shrink-0" onclick={() => addStage()}>
                    <Plus size={14} class="me-1.5" />
                    {m.common_onboarding_add()}
                  </Button>
                </div>
              </div>

              <div class="flex flex-wrap gap-2 pt-2 border-t border-border/40">
                {#each rawStages as stage}
                  <div class="flex items-center gap-1.5 px-3 py-1 border border-border rounded-full bg-background text-xs text-foreground select-none">
                    <span class="font-medium">{stage}</span>
                    <Button 
                      type="button" 
                      variant="ghost"
                      size="icon-xs"
                      class="text-muted-foreground hover:text-destructive transition-colors cursor-pointer h-auto w-auto p-0"
                      onclick={() => removeStage(stage)}
                    >
                      <Trash2 size={12} />
                    </Button>
                  </div>
                {/each}
                {#if rawStages.length === 0}
                  <p class="text-xs text-muted-foreground py-1">
                    {m.settings_registration_no_stages()}
                  </p>
                {/if}
              </div>
            </div>
          </div>

          <!-- Required Documents Checklist & Tailored Presets -->
          <div class="border border-border rounded-md overflow-hidden flex flex-col bg-card/10">
            <div class="bg-muted/50 px-6 py-3 border-b border-border text-[11px] font-bold text-muted-foreground tracking-wider uppercase text-start select-none flex items-center justify-between">
              <span>{m.settings_registration_docs_title()}</span>
              <Badge variant="outline" class="text-[10px] font-normal">
                المقترح لـ {currentProfile.name_ar}
              </Badge>
            </div>

            <div class="flex flex-col px-6 py-4 gap-4">
              <!-- Preset Recommendations Bar -->
              {#if currentProfile.requiredDocuments.length > 0}
                <div class="flex flex-col gap-2 p-3 bg-muted/20 border border-border/60 rounded-lg">
                  <span class="text-[11px] font-semibold text-foreground text-start">
                    مقترحات مستندات تسجيل لـ ({currentProfile.name_ar}):
                  </span>
                  <div class="flex flex-wrap gap-2">
                    {#each currentProfile.requiredDocuments as recDoc}
                      {@const isAdded = requiredDocuments.includes(recDoc)}
                      <button
                        type="button"
                        class="text-xs px-2.5 py-1 rounded-md border text-start flex items-center gap-1.5 transition-all cursor-pointer {isAdded ? 'bg-primary/10 border-primary/30 text-primary font-semibold' : 'bg-background hover:bg-accent/40 border-border text-foreground'}"
                        onclick={() => addDocument(recDoc)}
                      >
                        <Plus size={12} class={isAdded ? "hidden" : "inline"} />
                        <span>{recDoc}</span>
                      </button>
                    {/each}
                  </div>
                </div>
              {/if}

              <div class="flex flex-col md:flex-row md:items-center justify-between gap-4 pt-2">
                <div class="flex flex-col gap-1 text-start">
                  <Label class="text-xs font-semibold text-foreground">
                    {m.settings_registration_add_doc_title()}
                  </Label>
                  <span class="text-[11px] text-muted-foreground leading-relaxed">
                    {m.settings_registration_add_doc_desc()}
                  </span>
                </div>
                <div class="w-full md:w-[320px] shrink-0 flex gap-2">
                  <Input 
                    placeholder={m.settings_registration_add_doc_placeholder()} 
                    bind:value={documentInput} 
                    class="h-9 text-xs flex-1 border border-input bg-background"
                    onkeydown={(e) => e.key === "Enter" && (e.preventDefault(), addDocument())}
                  />
                  <Button type="button" variant="outline" size="sm" class="cursor-pointer font-semibold shrink-0" onclick={() => addDocument()}>
                    <Plus size={14} class="me-1.5" />
                    {m.common_onboarding_add()}
                  </Button>
                </div>
              </div>

              <div class="flex flex-wrap gap-2 pt-2 border-t border-border/40">
                {#each requiredDocuments as doc}
                  <div class="flex items-center gap-1.5 px-3 py-1 border border-border rounded-full bg-background text-xs text-foreground select-none">
                    <span class="font-medium">{doc}</span>
                    <Button 
                      type="button" 
                      variant="ghost"
                      size="icon-xs"
                      class="text-muted-foreground hover:text-destructive transition-colors cursor-pointer h-auto w-auto p-0"
                      onclick={() => removeDocument(doc)}
                    >
                      <Trash2 size={12} />
                    </Button>
                  </div>
                {/each}
                {#if requiredDocuments.length === 0}
                  <p class="text-xs text-muted-foreground py-1">
                    {m.settings_registration_no_docs()}
                  </p>
                {/if}
              </div>
            </div>
          </div>
        </TabsContent>

        <!-- TAB 3: Registration Fields Table -->
        <TabsContent value="fields" class="mt-0">
          <div class="border border-border rounded-md overflow-hidden flex flex-col bg-card/10">
            <div class="bg-muted/50 px-6 py-3 border-b border-border text-[11px] font-bold text-muted-foreground tracking-wider uppercase text-start select-none">
              {m.settings_registration_custom_fields_title()}
            </div>

            <div class="p-6 overflow-x-auto">
              <Table class="w-full text-xs text-start">
                <TableHeader>
                  <TableRow class="border-b border-border/60 hover:bg-transparent">
                    <TableHead class="py-2 text-start font-semibold text-muted-foreground">{m.settings_registration_table_field()}</TableHead>
                    <TableHead class="py-2 text-center font-semibold text-muted-foreground">{m.common_phase_active()}</TableHead>
                    <TableHead class="py-2 text-center font-semibold text-muted-foreground">{m.settings_registration_table_required()}</TableHead>
                    <TableHead class="py-2 text-center font-semibold text-muted-foreground">{m.settings_registration_table_duplicate()}</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {#each SYSTEM_FIELDS as sysField (sysField.key)}
                    <TableRow class="border-b border-border/40 hover:bg-muted/5">
                      <TableCell class="py-3 font-semibold text-foreground text-start">
                        {sysField.label}
                      </TableCell>
                      <TableCell class="py-3 text-center">
                        <Checkbox 
                          id={`active_${sysField.key}`} 
                          checked={activeFields[sysField.key] ?? false} 
                          onCheckedChange={(val) => {
                            activeFields[sysField.key] = !!val;
                            if (!val) {
                              requiredFields[sysField.key] = false;
                              preventDuplicates[sysField.key] = false;
                            }
                          }} 
                          class="mx-auto"
                        />
                      </TableCell>
                      <TableCell class="py-3 text-center">
                        <Checkbox 
                          id={`required_${sysField.key}`} 
                          checked={requiredFields[sysField.key] ?? false} 
                          disabled={!activeFields[sysField.key]}
                          onCheckedChange={(val) => requiredFields[sysField.key] = !!val} 
                          class="mx-auto"
                        />
                      </TableCell>
                      <TableCell class="py-3 text-center">
                        {#if sysField.supportsDuplicate}
                          <Checkbox 
                            id={`duplicate_${sysField.key}`} 
                            checked={preventDuplicates[sysField.key] ?? false} 
                            disabled={!activeFields[sysField.key]}
                            onCheckedChange={(val) => preventDuplicates[sysField.key] = !!val} 
                            class="mx-auto"
                          />
                        {:else}
                          <span class="text-muted-foreground">-</span>
                        {/if}
                      </TableCell>
                    </TableRow>
                  {/each}
                </TableBody>
              </Table>
            </div>
          </div>
        </TabsContent>

        <!-- TAB 4: Registration Workflow & Policy Rules -->
        <TabsContent value="policies" class="mt-0">
          <div class="border border-border rounded-md overflow-hidden flex flex-col bg-card/10">
            <div class="bg-muted/50 px-6 py-3 border-b border-border text-[11px] font-bold text-muted-foreground tracking-wider uppercase text-start select-none flex items-center gap-2">
              <ShieldCheck class="h-3.5 w-3.5 text-primary" />
              <span>سياسات وقواعد قبول الطالب والتسجيل</span>
            </div>

            <div class="flex flex-col">
              <!-- Admin Approval Rule -->
              <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
                <div class="flex flex-col gap-1 text-start">
                  <Label for="req_approval" class="text-xs font-semibold text-foreground cursor-pointer">
                    اشتراط موافقة الإدارة قبل الاعتماد النشط للطالب
                  </Label>
                  <span class="text-[11px] text-muted-foreground leading-relaxed">
                    عند تفعيل هذا الخيار، يتم تقديم الطلبات بحالة (قيد المراجعة) بدلاً من القيد المباشر.
                  </span>
                </div>
                <div class="shrink-0 flex items-center gap-2">
                  <Switch
                    id="req_approval"
                    checked={requiresApproval}
                    onCheckedChange={(val) => requiresApproval = val}
                  />
                  <span class="text-xs font-medium text-muted-foreground">{requiresApproval ? "مفعل" : "معطل"}</span>
                </div>
              </div>

              <!-- Allow No Guardian Rule -->
              <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
                <div class="flex flex-col gap-1 text-start">
                  <Label for="max_class_capacity" class="text-xs font-semibold text-foreground">
                    الحد الأقصى لعدد الطلاب في الصف
                  </Label>
                  <span class="text-[11px] text-muted-foreground leading-relaxed">
                    اتركه فارغًا لاستخدام سعة الصف المحددة في البيانات الأكاديمية.
                  </span>
                </div>
                <Input
                  id="max_class_capacity"
                  type="number"
                  value={maxClassCapacity ?? ""}
                  oninput={(event) => {
                    const value = event.currentTarget.value.trim();
                    maxClassCapacity = value === "" ? null : Math.max(1, Number(value));
                  }}
                  min="1"
                  step="1"
                  class="h-9 w-full md:w-40"
                />
              </div>

              <!-- Allow No Guardian Rule -->
              <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
                <div class="flex flex-col gap-1 text-start">
                  <Label for="allow_no_g" class="text-xs font-semibold text-foreground cursor-pointer">
                    السماح بالتسجيل بدون ولي أمر (للبالغين / الجامعات والمراكز)
                  </Label>
                  <span class="text-[11px] text-muted-foreground leading-relaxed">
                    يتيح للطلاب البالغين التسجيل مباشرة دون الحاجة لربط ولي أمر.
                  </span>
                </div>
                <div class="shrink-0 flex items-center gap-2">
                  <Switch
                    id="allow_no_g"
                    checked={allowNoGuardian}
                    onCheckedChange={(val) => allowNoGuardian = val}
                  />
                  <span class="text-xs font-medium text-muted-foreground">{allowNoGuardian ? "مسموح" : "غير مسموح"}</span>
                </div>
              </div>

              <!-- Auto Assign Mandatory Fees -->
              <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
                <div class="flex flex-col gap-1 text-start">
                  <Label for="auto_fees" class="text-xs font-semibold text-foreground cursor-pointer">
                    الربط التلقائي بالرسوم الإلزامية للمرحلة
                  </Label>
                  <span class="text-[11px] text-muted-foreground leading-relaxed">
                    توليد الرسوم الإلزامية تلقائياً على الطالب المستجد بمجرد قيده في الصف أو المساق.
                  </span>
                </div>
                <div class="shrink-0 flex items-center gap-2">
                  <Switch
                    id="auto_fees"
                    checked={autoAssignFees}
                    onCheckedChange={(val) => autoAssignFees = val}
                  />
                  <span class="text-xs font-medium text-muted-foreground">{autoAssignFees ? "مفعل" : "معطل"}</span>
                </div>
              </div>

              <!-- Allow Duplicate National ID -->
              <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b-0 gap-4">
                <div class="flex flex-col gap-1 text-start">
                  <Label for="allow_dup_id" class="text-xs font-semibold text-foreground cursor-pointer">
                    السماح بتكرار الرقم القومي / رقم الهوية
                  </Label>
                  <span class="text-[11px] text-muted-foreground leading-relaxed">
                    السماح بتسجيل أكثر من طالب بنفس رقم الهوية الوطنية (مثل الإخوة في حال استخدام رقم هوية ولي الأمر).
                  </span>
                </div>
                <div class="shrink-0 flex items-center gap-2">
                  <Switch
                    id="allow_dup_id"
                    checked={allowDuplicateNationalId}
                    onCheckedChange={(val) => allowDuplicateNationalId = val}
                  />
                  <span class="text-xs font-medium text-muted-foreground">{allowDuplicateNationalId ? "مسموح" : "ممنوع"}</span>
                </div>
              </div>
            </div>
          </div>
        </TabsContent>
      </Tabs>

    </form>
  </div>
  <SettingsBottomBar
    {isSaving}
    {isDirty}
    onsave={handleSubmit}
    onreset={restoreSnapshot}
  />
</div>
