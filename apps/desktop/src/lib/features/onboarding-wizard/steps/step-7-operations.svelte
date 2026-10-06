<script lang="ts">
import { onMount } from "svelte";
import { goto } from "$app/navigation";
import { getLocale } from "$lib/paraglide/runtime";
import { api, type DocumentRequirementDto, type OrganizationSettingsDto } from "$lib/api";
import { systemStore } from "$lib/stores";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Badge } from "@orbis/ui/badge";
import { Checkbox } from "@orbis/ui/checkbox";
import OnboardingStepHeader from "../onboarding-step-header.svelte";
import OnboardingSection from "../onboarding-section.svelte";
import { getTasksForStep } from "../onboarding-steps";

let { orgId }: { orgId: string } = $props();

const isArabic = $derived(getLocale().startsWith("ar"));

let isLoading = $state(true);
let isSaving = $state(false);
let isSubmittingDoc = $state(false);
let errorMessage = $state<string | null>(null);
let successMessage = $state<string | null>(null);

// Registration Policies State
let studentNumberPattern = $state("{YEAR}-{SEQ:5}");
let autoGenerateStudentId = $state(true);
let preventDuplicateNationalId = $state(true);
let preventDuplicatePhone = $state(false);
let requireGuardianApproval = $state(true);

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
      api.getSettings().catch(() => null),
      api.listDocumentRequirements().catch(() => []),
    ]);

    docRequirements = docsRes;
    if (settingsRes?.student_number_pattern) {
      studentNumberPattern = settingsRes.student_number_pattern;
    }
  } catch (err: any) {
    console.error("Failed to load operations data:", err);
    errorMessage = err?.message || "Failed to load operations settings";
  } finally {
    isLoading = false;
  }
}

async function handleSaveRegistrationPolicy() {
  isSaving = true;
  errorMessage = null;
  successMessage = null;

  try {
    const currentSettings = (await api.getSettings().catch(() => ({}))) || {};
    await api.updateSettings({
      ...currentSettings,
      student_number_pattern: studentNumberPattern.trim() || "{YEAR}-{SEQ:5}",
    });

    await systemStore.loadOnboardingState(orgId);
    successMessage = isArabic ? "تم حفظ سياسات التسجيل والقبول بنجاح" : "Registration policies saved successfully";
  } catch (err: any) {
    console.error("Failed to save registration policies:", err);
    errorMessage = err?.message || "Failed to save registration policies";
  } finally {
    isSaving = false;
  }
}

async function handleCreateDocRequirement() {
  if (!docNameEn.trim() && !docNameAr.trim()) {
    errorMessage = isArabic ? "يرجى كتابة اسم الوثيقة" : "Document name is required";
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
    await systemStore.loadOnboardingState(orgId);
    successMessage = isArabic ? "تمت إضافة متطلب الوثيقة بنجاح" : "Document requirement created successfully";
  } catch (err: any) {
    console.error("Failed to create document requirement:", err);
    errorMessage = err?.message || "Failed to create document requirement";
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
    await systemStore.loadOnboardingState(orgId);
    successMessage = isArabic ? `تمت إضافة ${preset.nameAr}` : `Added ${preset.nameEn}`;
  } catch (err: any) {
    console.error("Failed to add preset document:", err);
    errorMessage = err?.message || "Failed to add preset document";
  } finally {
    isSubmittingDoc = false;
  }
}

async function handleContinue() {
  await handleSaveRegistrationPolicy();
  await goto(`/${orgId}/onboarding/review`);
}
</script>

<div class="space-y-6">
  <OnboardingStepHeader
    stepNumber={7}
    title="Operations, Admissions & Document Requirements"
    titleAr="العمليات التشغيلية وسياسات القبول والوثائق"
    description="Configure student numbering formats, mandatory admission document requirements, and verification rules."
    descriptionAr="حدد قواعد الترقيم الأكاديمي للطلاب، الوثائق الإلزامية للتسجيل، وضوابط التحقق."
    isRequired={true}
    status={isTaskComplete ? "completed" : docRequirements.length > 0 ? "in_progress" : "not_started"}
  />

  {#if errorMessage}
    <div class="rounded-lg border border-destructive/30 bg-destructive/10 p-4 text-sm text-destructive font-medium flex items-center justify-between">
      <span>{errorMessage}</span>
      <Button variant="ghost" size="sm" onclick={() => errorMessage = null}>✕</Button>
    </div>
  {/if}

  {#if successMessage}
    <div class="rounded-lg border border-emerald-500/30 bg-emerald-500/10 p-4 text-sm text-emerald-600 dark:text-emerald-400 font-medium flex items-center justify-between">
      <span>{successMessage}</span>
      <Button variant="ghost" size="sm" onclick={() => successMessage = null}>✕</Button>
    </div>
  {/if}

  {#if isLoading}
    <div class="flex items-center justify-center p-12 text-muted-foreground text-sm gap-2">
      <span class="size-4 animate-spin rounded-full border-2 border-primary border-t-transparent"></span>
      <span>{isArabic ? "جاري تحميل إعدادات العمليات..." : "Loading operations settings..."}</span>
    </div>
  {:else}
    <!-- Section 1: Student Numbering & Registration Policy -->
    <OnboardingSection
      title="Student Numbering & ID Generation"
      titleAr="الترقيم الأكاديمي والرقم التعريفي للطالب"
      description="Define the automated sequence formula for generating new student IDs."
      descriptionAr="حدد صيغة الترقيم الآلي التي تولد الأرقام الأكاديمية للطلاب عند قبولهم."
      isComplete={Boolean(studentNumberPattern.trim())}
    >
      <div class="space-y-4">
        <div class="grid gap-3 sm:grid-cols-2">
          <div class="space-y-1.5 sm:col-span-2">
            <Label class="text-xs font-medium">
              {isArabic ? "صيغة الترقيم الأكاديمي (Student Number Pattern) *" : "Student ID Numbering Pattern *"}
            </Label>
            <Input
              bind:value={studentNumberPattern}
              placeholder={'\{YEAR\}-\{SEQ:5\}'}
              dir="ltr"
              class="font-mono text-xs"
            />
            <p class="text-[11px] text-muted-foreground">
              {isArabic ? "أمثلة: {YEAR}-{SEQ:5} (مثل 2026-00001) أو {STAGE}-{YEAR}-{SEQ:4}" : "Templates: {YEAR}-{SEQ:5} (e.g. 2026-00001) or {STAGE}-{YEAR}-{SEQ:4}"}
            </p>
          </div>
        </div>

        <div class="space-y-2 rounded-lg border border-border/40 bg-muted/20 p-3">
          <div class="flex items-center space-x-2 space-x-reverse">
            <Checkbox id="auto-gen" bind:checked={autoGenerateStudentId} />
            <Label for="auto-gen" class="text-xs font-medium cursor-pointer">
              {isArabic ? "توليد الرقم الأكاديمي تلقائياً فور حفظ استمارة التسجيل" : "Automatically generate student ID upon admission form submission"}
            </Label>
          </div>
          <div class="flex items-center space-x-2 space-x-reverse">
            <Checkbox id="prevent-dup" bind:checked={preventDuplicateNationalId} />
            <Label for="prevent-dup" class="text-xs font-medium cursor-pointer">
              {isArabic ? "منع تكرار رقم الهوية الوطنية / الإقامة على مستوى المنشأة" : "Strictly prevent duplicate National ID / Resident Iqama across institution"}
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
            {isSaving ? (isArabic ? "جاري الحفظ..." : "Saving...") : (isArabic ? "حفظ سياسة التسجيل" : "Save Registration Policy")}
          </Button>
        </div>
      </div>
    </OnboardingSection>

    <!-- Section 2: Required Admission Documents -->
    <OnboardingSection
      title="Mandatory Admission Documents"
      titleAr="الوثائق والمستندات المطلوبة للقبول"
      description="Define the checklist of documents guardians must provide during student registration."
      descriptionAr="حدد قائمة المستندات التي يجب على ولي الأمر إرفاقها لإنهاء القبول."
      badge={docRequirements.length > 0 ? `${docRequirements.length} ${isArabic ? "وثائق محددة" : "Documents"}` : undefined}
      isComplete={docRequirements.length > 0}
    >
      <div class="space-y-4">
        <!-- Quick Preset Document Badges -->
        {#if docRequirements.length === 0}
          <div class="rounded-lg border border-primary/20 bg-primary/5 p-4 space-y-3">
            <div class="text-xs font-semibold text-foreground">
              📄 {isArabic ? "إضافة الوثائق الرسمية الشائعة بنقرة واحدة" : "Quick Add Official Required Documents"}
            </div>
            <div class="flex flex-wrap gap-2">
              {#each PRESET_DOCUMENTS as preset}
                <Button
                  variant="outline"
                  size="sm"
                  disabled={isSubmittingDoc}
                  onclick={() => handleAddPresetDoc(preset)}
                  class="text-xs bg-background hover:bg-muted"
                >
                  + {isArabic ? preset.nameAr : preset.nameEn}
                </Button>
              {/each}
            </div>
          </div>
        {/if}

        <!-- Document Form -->
        <div class="rounded-lg border border-border/40 bg-muted/20 p-4 space-y-4">
          <div class="grid gap-3 sm:grid-cols-2">
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {isArabic ? "اسم الوثيقة (بالإنكليزية) *" : "Document Name (English) *"}
              </Label>
              <Input
                bind:value={docNameEn}
                placeholder="e.g. Birth Certificate"
                dir="ltr"
              />
            </div>
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {isArabic ? "اسم الوثيقة بالعربية" : "Document Name (Arabic)"}
              </Label>
              <Input
                bind:value={docNameAr}
                placeholder="مثال: شهادة الميلاد الرسمية"
                dir="rtl"
              />
            </div>
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {isArabic ? "رمز الوثيقة" : "Document Code"}
              </Label>
              <Input
                bind:value={docCode}
                placeholder="DOC_BIRTH_CERT"
                dir="ltr"
                class="font-mono text-xs"
              />
            </div>
            <div class="flex items-center gap-4 self-end pb-2">
              <div class="flex items-center space-x-2 space-x-reverse">
                <Checkbox id="doc-mand" bind:checked={docMandatory} />
                <Label for="doc-mand" class="text-xs font-medium cursor-pointer">
                  {isArabic ? "إلزامية" : "Mandatory"}
                </Label>
              </div>
              <div class="flex items-center space-x-2 space-x-reverse">
                <Checkbox id="doc-block" bind:checked={docBlocksEnrollment} />
                <Label for="doc-block" class="text-xs font-medium cursor-pointer">
                  {isArabic ? "حظر القبول حتى التسليم" : "Blocks Enrollment"}
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
              {isSubmittingDoc ? (isArabic ? "جاري الإضافة..." : "Adding...") : (isArabic ? "+ إضافة متطلب الوثيقة" : "+ Add Requirement")}
            </Button>
          </div>
        </div>

        <!-- Documents List -->
        {#if docRequirements.length > 0}
          <div class="space-y-2">
            <span class="text-xs font-semibold text-muted-foreground uppercase">
              {isArabic ? "الوثائق المعتمدة للتسجيل" : "Configured Admission Documents"}
            </span>
            <div class="grid gap-2 sm:grid-cols-2">
              {#each docRequirements as doc (doc.id)}
                <div class="flex items-center justify-between rounded-lg border border-border/50 bg-card p-3 text-xs">
                  <div>
                    <div class="font-medium text-foreground">
                      {isArabic && doc.name_ar ? doc.name_ar : doc.name_en || doc.code}
                    </div>
                    <div class="text-[10px] font-mono text-muted-foreground">
                      {doc.code}
                    </div>
                  </div>
                  <div class="flex items-center gap-1.5">
                    {#if doc.is_mandatory}
                      <Badge variant="outline" class="border-amber-500/30 text-amber-600 dark:text-amber-400 text-[10px]">
                        {isArabic ? "إلزامي" : "Required"}
                      </Badge>
                    {/if}
                    {#if doc.blocks_enrollment}
                      <Badge variant="destructive" class="text-[10px]">
                        {isArabic ? "شرط قبول" : "Blocking"}
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

    <!-- Bottom Actions Bar -->
    <div class="flex items-center justify-between border-t border-border/50 pt-6">
      <Button
        variant="outline"
        onclick={() => goto(`/${orgId}/onboarding/assessment`)}
        class="text-xs"
      >
        {isArabic ? "← الخطوة السابقة (التقييم)" : "← Back to Assessment"}
      </Button>

      <div class="flex items-center gap-3">
        <Button
          variant="default"
          onclick={handleContinue}
          class="text-xs font-semibold shadow-xs"
        >
          {isArabic ? "متابعة إلى المراجعة النهائية والإكمال →" : "Continue to Review & Launch →"}
        </Button>
      </div>
    </div>
  {/if}
</div>
