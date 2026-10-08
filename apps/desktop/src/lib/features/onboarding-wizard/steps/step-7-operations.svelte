<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import { onMount } from "svelte";
import * as m from "$lib/paraglide/messages";
import { api, type DocumentRequirementDto } from "$lib/api";
import { systemStore } from "$lib/stores";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Badge } from "@orbis/ui/badge";
import { Checkbox } from "@orbis/ui/checkbox";
import OnboardingStepHeader from "../onboarding-step-header.svelte";
import OnboardingSection from "../onboarding-section.svelte";
import { getTasksForStep } from "../onboarding-steps";
import { refreshOnboardingState } from "../refresh-onboarding-state";
import { BilingualNameFields } from "@orbis/ui/bilingual-name-fields";
import { suggestLocalTranslation } from "$lib/shared/translation/local-engine";

let { orgId }: { orgId: string } = $props();

let isLoading = $state(true);
let isSaving = $state(false);
let isSubmittingDoc = $state(false);
let errorMessage = $state<string | null>(null);
let successMessage = $state<string | null>(null);

// Registration Policies State
let studentNumberPattern = $state("{YEAR}-{SEQ:5}");
let autoGenerateStudentId = $state(true);
let preventDuplicateNationalId = $state(true);

// Document Requirements Data
let docRequirements = $state<DocumentRequirementDto[]>([]);
let docNameEn = $state("Birth Certificate");
let docNameAr = $state("شهادة الميلاد");
let docCode = $state("DOC_BIRTH_CERT");
let docMandatory = $state(true);
let docBlocksEnrollment = $state(true);

const PRESET_DOCUMENTS = [
  { code: "DOC_BIRTH_CERT", nameEn: "Birth Certificate", nameAr: "شهادة الميلاد الرسمية", mandatory: true, blocks: true },
  { code: "DOC_NATIONAL_ID", nameEn: "National ID / Iqama Copy", nameAr: "صورة الهوية الوطنية أو الإقامة", mandatory: true, blocks: true },
  { code: "DOC_PREV_RECORD", nameEn: "Previous Academic Transcripts", nameAr: "الشهادات والسجلات الدراسية السابقة", mandatory: true, blocks: false },
  { code: "DOC_VACCINE_CARD", nameEn: "Vaccination & Health Card", nameAr: "كارت التطعيمات والسجل الصحي", mandatory: false, blocks: false },
  { code: "DOC_PHOTO", nameEn: "Recent Passport Photographs (4x6)", nameAr: "الصور الشخصية الحديثة (4×6)", mandatory: true, blocks: false },
];

const stepTasks = $derived(getTasksForStep("operations", systemStore.onboardingTasks));
const isTaskComplete = $derived(stepTasks.length > 0 && stepTasks.every((t) => t.status === "completed"));

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
    if (settingsRes?.student_number_pattern) {
      studentNumberPattern = settingsRes.student_number_pattern;
    }
  } catch (err: any) {
    console.error("Failed to load operations data:", err);
    errorMessage = err?.message || m.onboarding_step7_err_load();
  } finally {
    isLoading = false;
  }
}

async function handleSaveRegistrationPolicy() {
  isSaving = true;
  errorMessage = null;
  successMessage = null;

  try {
    const currentSettings = (await api.getSettings()) || {};
    await api.updateSettings({
      ...currentSettings,
      student_number_pattern: studentNumberPattern.trim() || "{YEAR}-{SEQ:5}",
    });

    await refreshOnboardingState(orgId, m.common_action_failed());
    successMessage = m.onboarding_step7_policy_saved_success();
  } catch (err: any) {
    console.error("Failed to save registration policies:", err);
    errorMessage = err?.message || m.onboarding_step7_policy_err();
  } finally {
    isSaving = false;
  }
}

async function handleCreateDocRequirement() {
  if (!docNameEn.trim() && !docNameAr.trim()) {
    errorMessage = m.onboarding_step7_doc_err_name();
    return;
  }

  isSubmittingDoc = true;
  errorMessage = null;
  successMessage = null;

  try {
    const created = await api.createDocumentRequirement({
      code: docCode.trim() || `DOC_${Date.now()}`,
      name_en: docNameEn.trim() || docNameAr.trim(),
      name_ar: docNameAr.trim() || docNameEn.trim(),
      is_mandatory: docMandatory,
      blocks_enrollment: docBlocksEnrollment,
      target_type: "global",
    });

    docRequirements = [...docRequirements, created];
    docNameEn = "";
    docNameAr = "";
    docCode = `DOC_${docRequirements.length + 1}`;
    await refreshOnboardingState(orgId, m.common_action_failed());
    successMessage = m.onboarding_step7_doc_created_success();
  } catch (err: any) {
    console.error("Failed to create document requirement:", err);
    errorMessage = err?.message || m.onboarding_step7_err_create_doc();
  } finally {
    isSubmittingDoc = false;
  }
}

async function handleAddPresetDoc(preset: typeof PRESET_DOCUMENTS[0]) {
  isSubmittingDoc = true;
  errorMessage = null;
  successMessage = null;

  try {
    const created = await api.createDocumentRequirement({
      code: preset.code,
      name_en: preset.nameEn,
      name_ar: preset.nameAr,
      is_mandatory: preset.mandatory,
      blocks_enrollment: preset.blocks,
      target_type: "global",
    });

    docRequirements = [...docRequirements, created];
    await refreshOnboardingState(orgId, m.common_action_failed());
    successMessage = m.onboarding_step7_preset_added({ name: preset.nameEn });
  } catch (err: any) {
    console.error("Failed to add preset document:", err);
    errorMessage = err?.message || m.onboarding_step7_err_create_doc();
  } finally {
    isSubmittingDoc = false;
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

  {#if errorMessage}
    <div class="rounded-lg border border-destructive/30 bg-destructive/10 p-4 text-sm text-destructive font-medium flex items-center justify-between">
      <span>{errorMessage}</span>
      <Button variant="ghost" size="sm" onclick={() => (errorMessage = null)}>✕</Button>
    </div>
  {/if}

  {#if successMessage}
    <div class="rounded-lg border border-emerald-500/30 bg-emerald-500/10 p-4 text-sm text-emerald-600 dark:text-emerald-400 font-medium flex items-center justify-between">
      <span>{successMessage}</span>
      <Button variant="ghost" size="sm" onclick={() => (successMessage = null)}>✕</Button>
    </div>
  {/if}

  {#if isLoading}
    <div class="flex items-center justify-center p-12 text-muted-foreground text-sm gap-2">
      <LoadingIndicator size={16} class="size-4 text-primary" />
      <span>{m.onboarding_step7_loading()}</span>
    </div>
  {:else}
    <!-- Section 1: Student Numbering & Registration Policy -->
    <OnboardingSection
      title={m.onboarding_step7_numbering_title()}
      description={m.onboarding_step7_numbering_desc()}
      isComplete={Boolean(studentNumberPattern.trim())}
    >
      <div class="space-y-4">
        <div class="grid gap-3 sm:grid-cols-2">
          <div class="space-y-1.5 sm:col-span-2">
            <Label class="text-xs font-medium">
              {m.onboarding_step7_pattern_label()}
            </Label>
            <Input
              bind:value={studentNumberPattern}
              placeholder={'\\{YEAR\\}-\\{SEQ:5\\}'}
              class="font-mono text-xs"
            />
            <p class="text-[11px] text-muted-foreground">
              {m.onboarding_step7_pattern_hint()}
            </p>
          </div>
        </div>

        <div class="space-y-2 rounded-lg border border-border/40 bg-muted/20 p-3">
          <div class="flex items-center space-x-2 space-x-reverse">
            <Checkbox id="auto-gen" bind:checked={autoGenerateStudentId} />
            <Label for="auto-gen" class="text-xs font-medium cursor-pointer">
              {m.onboarding_step7_auto_gen()}
            </Label>
          </div>
          <div class="flex items-center space-x-2 space-x-reverse">
            <Checkbox id="prevent-dup" bind:checked={preventDuplicateNationalId} />
            <Label for="prevent-dup" class="text-xs font-medium cursor-pointer">
              {m.onboarding_step7_prevent_dup()}
            </Label>
          </div>
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
              <div class="flex items-center space-x-2 space-x-reverse">
                <Checkbox id="doc-mand" bind:checked={docMandatory} />
                <Label for="doc-mand" class="text-xs font-medium cursor-pointer">
                  {m.onboarding_step7_doc_mandatory()}
                </Label>
              </div>
              <div class="flex items-center space-x-2 space-x-reverse">
                <Checkbox id="doc-block" bind:checked={docBlocksEnrollment} />
                <Label for="doc-block" class="text-xs font-medium cursor-pointer">
                  {m.onboarding_step7_doc_blocking()}
                </Label>
              </div>
            </div>
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
