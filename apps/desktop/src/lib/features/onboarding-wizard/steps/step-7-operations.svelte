<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import { onMount } from "svelte";
import * as m from "$lib/paraglide/messages";
import { toast } from "@orbis/ui";
import {
  api,
  type DocumentRequirementDto,
  type OrganizationSettingsDto,
} from "$lib/api";
import { systemStore } from "$lib/stores";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Badge } from "@orbis/ui/badge";
import { Checkbox } from "@orbis/ui/checkbox";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui/select";
import OnboardingStepHeader from "../onboarding-step-header.svelte";
import OnboardingSection from "../onboarding-section.svelte";
import { getTasksForStep } from "../onboarding-steps";
import { refreshOnboardingState } from "../refresh-onboarding-state";
import { BilingualNameFields } from "@orbis/ui/bilingual-name-fields";
import { suggestLocalTranslation } from "$lib/shared/translation/local-engine";
import DeleteRecordButton from "../components/delete-record-button.svelte";

let { orgId }: { orgId: string } = $props();

let isLoading = $state(true);
let isSaving = $state(false);
let isSubmittingDoc = $state(false);

// Registration Policies State
let studentNumberPrefix = $state("STU");
let studentNumberYearFormat = $state<"full" | "short" | "none">("full");
let studentNumberDigits = $state(4);
let studentNumberSeparator = $state("-");
let preventDuplicateNationalId = $state(true);

// Document Requirements Data
let docRequirements = $state<DocumentRequirementDto[]>([]);
let docNameEn = $state("Birth Certificate");
let docNameAr = $state("شهادة الميلاد");
let docCode = $state("DOC_BIRTH_CERT");
let docMandatory = $state(true);
let docBlocksEnrollment = $state(true);
let docCondition = $state("all_students");

const PRESET_DOCUMENTS = [
  { code: "DOC_BIRTH_CERT", nameEn: "Birth Certificate", nameAr: "شهادة الميلاد الرسمية", mandatory: true, blocks: true },
  { code: "DOC_NATIONAL_ID", nameEn: "National ID / Iqama Copy", nameAr: "صورة الهوية الوطنية أو الإقامة", mandatory: true, blocks: true },
  { code: "DOC_PREV_RECORD", nameEn: "Previous Academic Transcripts", nameAr: "الشهادات والسجلات الدراسية السابقة", mandatory: true, blocks: false },
  { code: "DOC_VACCINE_CARD", nameEn: "Vaccination & Health Card", nameAr: "كارت التطعيمات والسجل الصحي", mandatory: false, blocks: false },
  { code: "DOC_PHOTO", nameEn: "Recent Passport Photographs (4x6)", nameAr: "الصور الشخصية الحديثة (4×6)", mandatory: true, blocks: false },
];

const stepTasks = $derived(getTasksForStep("operations", systemStore.onboardingTasks));
const isTaskComplete = $derived(stepTasks.length > 0 && stepTasks.every((t) => t.status === "completed"));
const studentNumberPreview = $derived.by(() => {
  const parts: string[] = [];
  if (studentNumberPrefix.trim()) parts.push(studentNumberPrefix.trim().toUpperCase());
  if (studentNumberYearFormat === "full") parts.push(String(new Date().getFullYear()));
  if (studentNumberYearFormat === "short") {
    parts.push(String(new Date().getFullYear()).slice(-2));
  }
  parts.push("1".padStart(studentNumberDigits, "0"));
  return parts.join(studentNumberSeparator);
});

onMount(async () => {
  await loadData();
});

async function loadData() {
  try {
    isLoading = true;
    const [settingsRes, docsRes] = await Promise.all([
      api.getSettings(),
      api.listDocumentRequirements(),
    ]);

    docRequirements = docsRes;
    studentNumberPrefix = settingsRes.admission?.numbering?.prefix ?? "STU";
    studentNumberSeparator =
      settingsRes.admission?.numbering?.separator || "-";
    const savedYearFormat = settingsRes.admission?.numbering?.year_format;
    studentNumberYearFormat = savedYearFormat === "short" || savedYearFormat === "none"
      ? savedYearFormat
      : "full";
    studentNumberDigits = Math.min(
      6,
      Math.max(3, settingsRes.admission?.numbering?.sequence_digits ?? 4),
    );
    preventDuplicateNationalId =
      !(settingsRes.admission?.policy?.allow_duplicate_national_id ?? false);
  } catch (err: any) {
    console.error("Failed to load operations data:", err);
    toast.error(err?.message || m.onboarding_step7_err_load());
  } finally {
    isLoading = false;
  }
}

async function handleSaveRegistrationPolicy() {
  isSaving = true;

  try {
    const currentSettings: OrganizationSettingsDto = await api.getSettings();
    await api.updateSettings({
      ...currentSettings,
      admission: {
        ...currentSettings.admission,
        numbering: {
          prefix: studentNumberPrefix.trim().toUpperCase(),
          year_format: studentNumberYearFormat,
          sequence_digits: studentNumberDigits,
          separator: studentNumberSeparator,
        },
        policy: {
          requires_approval:
            currentSettings.admission?.policy?.requires_approval ?? false,
          allow_no_guardian:
            currentSettings.admission?.policy?.allow_no_guardian ?? false,
          allow_duplicate_national_id: !preventDuplicateNationalId,
          auto_assign_fees:
            currentSettings.admission?.policy?.auto_assign_fees ?? true,
          max_class_capacity:
            currentSettings.admission?.policy?.max_class_capacity ?? 30,
          allow_late_enrollment:
            currentSettings.admission?.policy?.allow_late_enrollment ?? false,
          allow_waitlist:
            currentSettings.admission?.policy?.allow_waitlist ?? true,
          sibling_priority_enabled:
            currentSettings.admission?.policy?.sibling_priority_enabled ?? true,
        },
      },
      enrollment: {
        is_configured: true,
        registration_enabled:
          currentSettings.enrollment?.registration_enabled ?? true,
        require_admin_approval:
          currentSettings.enrollment?.require_admin_approval ?? true,
        allow_waitlist: currentSettings.enrollment?.allow_waitlist ?? true,
        sibling_priority_enabled:
          currentSettings.enrollment?.sibling_priority_enabled ?? true,
        max_class_size_default:
          currentSettings.enrollment?.max_class_size_default ?? 25,
        allow_late_enrollment:
          currentSettings.enrollment?.allow_late_enrollment ?? false,
      },
    });

    await refreshOnboardingState(orgId, m.common_action_failed());
    toast.success(m.onboarding_step7_policy_saved_success());
  } catch (err: any) {
    console.error("Failed to save registration policies:", err);
    toast.error(err?.message || m.onboarding_step7_policy_err());
  } finally {
    isSaving = false;
  }
}

async function handleCreateDocRequirement() {
  if (!docNameEn.trim() && !docNameAr.trim()) {
    toast.error(m.onboarding_step7_doc_err_name());
    return;
  }

  isSubmittingDoc = true;

  try {
    const created = await api.createDocumentRequirement({
      code: docCode.trim() || `DOC_${Date.now()}`,
      name_en: docNameEn.trim() || docNameAr.trim(),
      name_ar: docNameAr.trim() || docNameEn.trim(),
      is_mandatory: docMandatory,
      blocks_enrollment: docBlocksEnrollment,
      target_type: "global",
      condition: docCondition,
    });

    docRequirements = [...docRequirements, created];
    docNameEn = "";
    docNameAr = "";
    docCode = `DOC_${docRequirements.length + 1}`;
    docCondition = "all_students";
    await refreshOnboardingState(orgId, m.common_action_failed());
    toast.success(m.onboarding_step7_doc_created_success());
  } catch (err: any) {
    console.error("Failed to create document requirement:", err);
    toast.error(err?.message || m.onboarding_step7_err_create_doc());
  } finally {
    isSubmittingDoc = false;
  }
}

async function handleAddPresetDoc(preset: typeof PRESET_DOCUMENTS[0]) {
  isSubmittingDoc = true;

  try {
    const created = await api.createDocumentRequirement({
      code: preset.code,
      name_en: preset.nameEn,
      name_ar: preset.nameAr,
      is_mandatory: preset.mandatory,
      blocks_enrollment: preset.blocks,
      target_type: "global",
      condition: "all_students",
    });

    docRequirements = [...docRequirements, created];
    await refreshOnboardingState(orgId, m.common_action_failed());
    toast.success(m.onboarding_step7_preset_added({ name: preset.nameEn }));
  } catch (err: any) {
    console.error("Failed to add preset document:", err);
    toast.error(err?.message || m.onboarding_step7_err_create_doc());
  } finally {
    isSubmittingDoc = false;
  }
}

async function deleteDocumentRequirement(doc: DocumentRequirementDto) {
  await api.deleteDocumentRequirement(String(doc.id));
  docRequirements = docRequirements.filter((item) => item.id !== doc.id);
  await refreshOnboardingState(orgId, m.common_action_failed());
  toast.success(m.common_docs_delete_success());
}

function conditionLabel(condition: string): string {
  switch (condition) {
    case "non_national_only":
      return m.ui_copy_foreign_expatriate_students_only_93492941();
    case "transfer_students_only":
      return m.ui_copy_transfer_students_only_bf6acf94();
    case "special_needs_only":
      return m.ui_copy_special_needs_5315c628();
    default:
      return m.ui_copy_all_new_students_b14cd3e5();
  }
}

</script>

<div class="space-y-6">
  <OnboardingStepHeader
    stepNumber={8}
    title={m.onboarding_step7_title()}
    description={m.onboarding_step7_subtitle()}
    isRequired={true}
    status={isTaskComplete ? "completed" : docRequirements.length > 0 ? "in_progress" : "not_started"}
  />

  {#if isLoading}
    <div class="flex items-center justify-center p-12 text-muted-foreground text-sm gap-2">
      <LoadingIndicator size={16} class="size-4 text-primary" />
      <span>{m.onboarding_step7_loading()}</span>
    </div>
  {:else}
    <!-- Section 1: Student Numbering & Registration Policy -->
    <OnboardingSection
      title={m.onboarding_step7_numbering_title()}
      isComplete={Boolean(studentNumberPrefix.trim())}
    >
      <div class="space-y-4">
        <div class="grid gap-4 sm:grid-cols-2">
          <div class="space-y-1.5">
            <Label for="student-number-prefix" class="text-xs font-medium">
              {m.ui_copy_prefix_9af3f0d2()}
            </Label>
            <Input
              id="student-number-prefix"
              bind:value={studentNumberPrefix}
              placeholder="STU"
              maxlength={12}
              class="font-mono text-xs uppercase"
              disabled={isSaving}
            />
          </div>
          <div class="space-y-1.5">
            <Label class="text-xs font-medium">
              {m.onboarding_step7_pattern_hint()}
            </Label>
            <Select
              type="single"
              value={studentNumberYearFormat}
              onValueChange={(value) => {
                if (value === "full" || value === "short" || value === "none") {
                  studentNumberYearFormat = value;
                }
              }}
              disabled={isSaving}
            >
              <SelectTrigger class="h-9 text-start text-xs">
                {studentNumberYearFormat === "full"
                  ? m.ui_copy_full_4_digit_year_2026_3a1eaa6a()
                  : studentNumberYearFormat === "short"
                    ? m.ui_copy_short_2_digit_year_26_5f63ba6d()
                    : m.ui_copy_no_year_92c0ace0()}
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="full">{m.ui_copy_full_4_digit_year_2026_3a1eaa6a()}</SelectItem>
                <SelectItem value="short">{m.ui_copy_short_2_digit_year_26_5f63ba6d()}</SelectItem>
                <SelectItem value="none">{m.ui_copy_no_year_92c0ace0()}</SelectItem>
              </SelectContent>
            </Select>
          </div>
          <div class="space-y-1.5">
            <Label class="text-xs font-medium">
              {m.onboarding_step7_pattern_label()}
            </Label>
            <Select
              type="single"
              value={String(studentNumberDigits)}
              onValueChange={(value) => {
                const digits = Number(value);
                if (digits >= 3 && digits <= 6) studentNumberDigits = digits;
              }}
              disabled={isSaving}
            >
              <SelectTrigger class="h-9 text-start text-xs">
                {studentNumberDigits} {m.ui_copy_digits_2a437f20()}
              </SelectTrigger>
              <SelectContent>
                {#each [3, 4, 5, 6] as digits (digits)}
                  <SelectItem value={String(digits)}>
                    {digits} {m.ui_copy_digits_2a437f20()} ({String(1).padStart(digits, "0")})
                  </SelectItem>
                {/each}
              </SelectContent>
            </Select>
          </div>
          <div class="flex items-center gap-2 self-end pb-2">
            <Checkbox id="prevent-dup" bind:checked={preventDuplicateNationalId} disabled={isSaving} />
            <Label for="prevent-dup" class="cursor-pointer text-xs font-medium">
              {m.onboarding_step7_prevent_dup()}
            </Label>
          </div>
        </div>

        <div class="rounded-md bg-muted/40 px-3 py-2" aria-live="polite">
          <p class="font-mono text-xs font-semibold text-foreground">{studentNumberPreview}</p>
          <p class="mt-1 text-[11px] text-muted-foreground">{m.onboarding_step7_auto_gen()}</p>
        </div>

        <div class="flex justify-end">
          <Button
            variant="secondary"
            size="sm"
            disabled={isSaving}
            onclick={handleSaveRegistrationPolicy}
            class="text-xs font-semibold"
          >
            {isSaving ? m.onboarding_step7_saving_btn() : m.onboarding_step7_save_policy_btn()}
          </Button>
        </div>
      </div>
    </OnboardingSection>

    <!-- Section 2: Required Admission Documents -->
    <OnboardingSection
      title={m.onboarding_step7_docs_title()}
      description={m.onboarding_step7_docs_desc()}
      badge={docRequirements.length > 0 ? m.onboarding_step7_docs_count({ count: docRequirements.length }) : undefined}
      isComplete={docRequirements.length > 0}
    >
      <div class="space-y-4">
        <!-- Quick Preset Document Badges -->
        {#if docRequirements.length === 0}
          <div class="rounded-lg border border-primary/20 bg-primary/5 p-4 space-y-3">
            <div class="text-xs font-semibold text-foreground">
              📄 {m.onboarding_step7_quick_docs()}
            </div>
            <div class="flex flex-wrap gap-2">
              {#each PRESET_DOCUMENTS as preset (preset.code)}
                <Button
                  variant="outline"
                  size="sm"
                  disabled={isSubmittingDoc}
                  onclick={() => handleAddPresetDoc(preset)}
                  class="text-xs bg-background hover:bg-muted"
                >
                  + {preset.nameEn}
                </Button>
              {/each}
            </div>
          </div>
        {/if}

        <!-- Document Form -->
        <div class="rounded-lg border border-border/40 bg-muted/20 p-4 space-y-4">
          <div class="grid gap-3 sm:grid-cols-2">
            <div class="sm:col-span-2">
              <BilingualNameFields
                arId="onboarding-document-name-ar"
                enId="onboarding-document-name-en"
                arLabel={m.onboarding_step7_doc_name_ar()}
                enLabel={m.onboarding_step7_doc_name_en()}
                bind:arValue={docNameAr}
                bind:enValue={docNameEn}
                arPlaceholder={m.onboarding_step7_doc_name_ar_placeholder()}
                enPlaceholder={m.onboarding_step7_doc_name_en_placeholder()}
                kind="label"
                requestSuggestion={suggestLocalTranslation}
                replaceUneditedInitialValues
                disabled={isSubmittingDoc}
                loadingMessage={m.bilingual_entry_loading()}
                failureMessage={m.bilingual_entry_failure()}
              />
            </div>
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {m.onboarding_step7_doc_code()}
              </Label>
              <Input
                bind:value={docCode}
                placeholder="DOC_BIRTH_CERT"
                class="font-mono text-xs"
              />
            </div>
            <div class="flex items-center gap-4 self-end pb-2">
              <div class="flex items-center gap-2">
                <Checkbox id="doc-mand" bind:checked={docMandatory} />
                <Label for="doc-mand" class="text-xs font-medium cursor-pointer">
                  {m.onboarding_step7_doc_mandatory()}
                </Label>
              </div>
              <div class="flex items-center gap-2">
                <Checkbox id="doc-block" bind:checked={docBlocksEnrollment} />
                <Label for="doc-block" class="text-xs font-medium cursor-pointer">
                  {m.onboarding_step7_doc_blocking()}
                </Label>
              </div>
            </div>
          </div>

          <div class="space-y-1.5">
            <Label class="text-xs font-medium">
              {m.ui_copy_applicability_condition_edba6012()}
            </Label>
            <Select
              type="single"
              bind:value={docCondition}
              disabled={isSubmittingDoc}
            >
              <SelectTrigger class="h-9 text-start text-xs">
                {conditionLabel(docCondition)}
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="all_students">{m.ui_copy_all_new_students_b14cd3e5()}</SelectItem>
                <SelectItem value="non_national_only">{m.ui_copy_foreign_expatriate_students_only_93492941()}</SelectItem>
                <SelectItem value="transfer_students_only">{m.ui_copy_transfer_students_only_bf6acf94()}</SelectItem>
                <SelectItem value="special_needs_only">{m.ui_copy_special_needs_5315c628()}</SelectItem>
              </SelectContent>
            </Select>
          </div>

          <div class="flex justify-end">
            <Button
              variant="default"
              size="sm"
              disabled={isSubmittingDoc || (!docNameEn.trim() && !docNameAr.trim())}
              onclick={handleCreateDocRequirement}
              class="text-xs font-semibold"
            >
              {isSubmittingDoc ? m.onboarding_step7_adding_btn() : m.onboarding_step7_add_doc_btn()}
            </Button>
          </div>
        </div>

        <!-- Documents List -->
        {#if docRequirements.length > 0}
          <div class="space-y-2">
            <span class="text-xs font-semibold text-muted-foreground uppercase">
              {m.onboarding_step7_configured_docs()}
            </span>
            <div class="grid gap-2 sm:grid-cols-2">
              {#each docRequirements as doc (doc.id)}
                <div class="flex items-center justify-between rounded-lg border border-border/50 bg-card p-3 text-xs">
                  <div>
                    <div class="font-medium text-foreground">
                      {doc.name_ar || doc.name_en || doc.code}
                    </div>
                    <div class="text-[10px] font-mono text-muted-foreground">
                      {doc.code}
                    </div>
                    <div class="mt-1 flex flex-wrap gap-1.5">
                      <Badge variant="secondary" class="text-[10px]">
                        {doc.target_type === "global" ? m.ui_copy_all_classes_grade_levels_cd709249() : doc.target_type}
                      </Badge>
                      <Badge variant="outline" class="text-[10px]">
                        {conditionLabel(doc.condition)}
                      </Badge>
                    </div>
                  </div>
                  <div class="flex items-center gap-1.5">
                    {#if doc.is_mandatory}
                      <Badge variant="outline" class="border-amber-500/30 text-amber-600 dark:text-amber-400 text-[10px]">
                        {m.onboarding_step7_required_badge()}
                      </Badge>
                    {/if}
                    {#if doc.blocks_enrollment}
                      <Badge variant="destructive" class="text-[10px]">
                        {m.onboarding_step7_blocking_badge()}
                      </Badge>
                    {/if}
                    <DeleteRecordButton label={doc.name_ar || doc.name_en || doc.code} onDelete={() => deleteDocumentRequirement(doc)} />
                  </div>
                </div>
              {/each}
            </div>
          </div>
        {/if}
      </div>
    </OnboardingSection>

    <!-- Inline Actions Bar -->
  {/if}
</div>
