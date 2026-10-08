<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import { onMount } from "svelte";
import * as m from "$lib/paraglide/messages";
import { toast } from "@orbis/ui";
import { api, type SubjectDto, type ClassDto, type AcademicPeriodDto } from "$lib/api";
import { systemStore } from "$lib/stores";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Badge } from "@orbis/ui/badge";
import OnboardingStepHeader from "../onboarding-step-header.svelte";
import OnboardingSection from "../onboarding-section.svelte";
import { getTasksForStep } from "../onboarding-steps";
import { refreshOnboardingState } from "../refresh-onboarding-state";
import { BilingualNameFields } from "@orbis/ui/bilingual-name-fields";
import { suggestLocalTranslation } from "$lib/shared/translation/local-engine";

let { orgId }: { orgId: string } = $props();

let isLoading = $state(true);
let isSubmittingSubject = $state(false);
let isSubmittingClass = $state(false);
let successMessage = $state<string | null>(null);

// Existing Data
let subjects = $state<SubjectDto[]>([]);
let classes = $state<ClassDto[]>([]);
let periods = $state<AcademicPeriodDto[]>([]);

// Subject Creation Form
let subjectName = $state("Mathematics");
let subjectNameAr = $state("الرياضيات");
let subjectCode = $state("MATH-101");
let subjectCreditHours = $state("3");

// Class Creation Form
let className = $state("Grade 1 - Section A");
let classNameAr = $state("الصف الأول - شعبة (أ)");
let classCode = $state("G1-A");
let classGrade = $state("Grade 1");
let classCapacity = $state<number>(30);
let selectedPeriodId = $state<string>("");

const STANDARD_SUBJECTS = [
  { code: "MATH-01", name: "Mathematics", nameAr: "الرياضيات", credits: "4" },
  { code: "ARAB-01", name: "Arabic Language", nameAr: "اللغة العربية", credits: "4" },
  { code: "ENG-01", name: "English Language", nameAr: "اللغة الإنجليزية", credits: "4" },
  { code: "ISLM-01", name: "Islamic Studies", nameAr: "الدراسات الإسلامية", credits: "3" },
  { code: "SCI-01", name: "Science", nameAr: "العلوم العامة", credits: "3" },
  { code: "SOC-01", name: "Social Studies", nameAr: "الدراسات الاجتماعية", credits: "2" },
  { code: "CS-01", name: "Computer Science", nameAr: "الحاسب الآلي والتقنية", credits: "2" },
  { code: "ART-01", name: "Art Education", nameAr: "التربية الفنية", credits: "1" },
  { code: "PE-01", name: "Physical Education", nameAr: "التربية البدنية", credits: "1" },
];

const STANDARD_CLASSES = [
  { code: "G1-A", name: "Grade 1 - A", nameAr: "الصف الأول (أ)", grade: "Grade 1" },
  { code: "G2-A", name: "Grade 2 - A", nameAr: "الصف الثاني (أ)", grade: "Grade 2" },
  { code: "G3-A", name: "Grade 3 - A", nameAr: "الصف الثالث (أ)", grade: "Grade 3" },
  { code: "G4-A", name: "Grade 4 - A", nameAr: "الصف الرابع (أ)", grade: "Grade 4" },
  { code: "G5-A", name: "Grade 5 - A", nameAr: "الصف الخامس (أ)", grade: "Grade 5" },
  { code: "G6-A", name: "Grade 6 - A", nameAr: "الصف السادس (أ)", grade: "Grade 6" },
];

const stepTasks = $derived(getTasksForStep("academics", systemStore.onboardingTasks));
const isTaskComplete = $derived(stepTasks.length > 0 && stepTasks.every((t) => t.status === "completed"));

onMount(async () => {
  await loadData();
});

async function loadData() {
  try {
    isLoading = true;
    const [subRes, classRes, periodRes] = await Promise.all([
      api.listSubjects(),
      api.listClasses(),
      api.listAcademicPeriods(),
    ]);
    subjects = subRes;
    classes = classRes;
    periods = periodRes;
    if (periods.length > 0 && !selectedPeriodId) {
      selectedPeriodId = String(periods[0].id);
    }
  } catch (err: any) {
    console.error("Failed to load academic data:", err);
    toast.error(err?.message || m.common_action_failed());
  } finally {
    isLoading = false;
  }
}

async function handleCreateSubject() {
  if (!subjectName.trim() && !subjectNameAr.trim()) {
    toast.error(m.onboarding_step4_subject_err_name());
    return;
  }

  isSubmittingSubject = true;
  successMessage = null;

  try {
    const newSubject = await api.createSubject({
      code: subjectCode.trim() || `SUB-${subjects.length + 1}`,
      name: subjectName.trim() || subjectNameAr.trim(),
      name_ar: subjectNameAr.trim() || undefined,
      credit_hours: subjectCreditHours,
      is_core: true,
    });

    subjects = [...subjects, newSubject];
    subjectName = "";
    subjectNameAr = "";
    subjectCode = `SUB-0${subjects.length + 1}`;
    await refreshOnboardingState(orgId, m.common_action_failed());
    successMessage = m.onboarding_step4_subject_created_success();
  } catch (err: any) {
    console.error("Failed to create subject:", err);
    toast.error(err?.message || m.common_action_failed());
  } finally {
    isSubmittingSubject = false;
  }
}

async function handleBatchImportSubjects() {
  isSubmittingSubject = true;
  successMessage = null;

  try {
    for (const sub of STANDARD_SUBJECTS) {
      const created = await api.createSubject({
        code: sub.code,
        name: sub.name,
        name_ar: sub.nameAr,
        credit_hours: sub.credits,
        is_core: true,
      });
      subjects = [...subjects, created];
    }

    await refreshOnboardingState(orgId, m.common_action_failed());
    successMessage = m.onboarding_step4_subjects_imported_success();
  } catch (err: any) {
    console.error("Failed to import subjects:", err);
    toast.error(err?.message || m.common_action_failed());
  } finally {
    isSubmittingSubject = false;
  }
}

async function handleCreateClass() {
  if (!className.trim() && !classNameAr.trim()) {
    toast.error(m.onboarding_step4_class_err_name());
    return;
  }

  isSubmittingClass = true;
  successMessage = null;

  try {
    const newClass = await api.createClass({
      name: className.trim() || classNameAr.trim(),
      name_ar: classNameAr.trim() || undefined,
      code: classCode.trim() || `CLS-${classes.length + 1}`,
      period_id: selectedPeriodId || undefined,
      grade_level: classGrade.trim() || undefined,
      capacity: classCapacity > 0 ? classCapacity : 30,
    });

    classes = [...classes, newClass];
    className = "";
    classNameAr = "";
    classCode = `CLS-0${classes.length + 1}`;
    await refreshOnboardingState(orgId, m.common_action_failed());
    successMessage = m.onboarding_step4_class_created_success();
  } catch (err: any) {
    console.error("Failed to create class:", err);
    toast.error(err?.message || m.common_action_failed());
  } finally {
    isSubmittingClass = false;
  }
}

async function handleBatchImportClasses() {
  isSubmittingClass = true;
  successMessage = null;

  try {
    for (const cls of STANDARD_CLASSES) {
      const created = await api.createClass({
        name: cls.name,
        name_ar: cls.nameAr,
        code: cls.code,
        period_id: selectedPeriodId || undefined,
        grade_level: cls.grade,
        capacity: 30,
      });
      classes = [...classes, created];
    }

    await refreshOnboardingState(orgId, m.common_action_failed());
    successMessage = m.onboarding_step4_classes_imported_success();
  } catch (err: any) {
    console.error("Failed to import classes:", err);
    toast.error(err?.message || m.common_action_failed());
  } finally {
    isSubmittingClass = false;
  }
}

</script>

<div class="space-y-6">
  <OnboardingStepHeader
    stepNumber={4}
    title={m.onboarding_step4_title()}
    description={m.onboarding_step4_subtitle()}
    isRequired={true}
    status={isTaskComplete ? "completed" : subjects.length > 0 || classes.length > 0 ? "in_progress" : "not_started"}
  />


  {#if successMessage}
    <div class="rounded-lg border border-emerald-500/30 bg-emerald-500/10 p-4 text-sm text-emerald-600 dark:text-emerald-400 font-medium flex items-center justify-between">
      <span>{successMessage}</span>
      <Button variant="ghost" size="sm" onclick={() => successMessage = null}>✕</Button>
    </div>
  {/if}

  {#if isLoading}
    <div class="flex items-center justify-center p-12 text-muted-foreground text-sm gap-2">
      <LoadingIndicator size={16} class="size-4 text-primary" />
      <span>{m.common_control_loading()}</span>
    </div>
  {:else}
    <!-- Section 1: Subjects & Courses -->
    <OnboardingSection
      title={m.onboarding_step4_subjects_title()}
      description={m.onboarding_step4_subjects_desc()}
      badge={subjects.length > 0 ? `${subjects.length}` : undefined}
      isComplete={subjects.length > 0}
    >
      <div class="space-y-4">
        <!-- Quick Batch Import -->
        {#if subjects.length === 0}
          <div class="rounded-lg border border-primary/20 bg-primary/5 p-4 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-3">
            <div>
              <div class="text-xs font-semibold text-foreground">
                📚 {m.onboarding_step4_quick_subjects()}
              </div>
              <p class="text-xs text-muted-foreground">
                {m.onboarding_step4_quick_subjects_desc()}
              </p>
            </div>
            <Button
              variant="outline"
              size="sm"
              disabled={isSubmittingSubject}
              onclick={handleBatchImportSubjects}
              class="text-xs bg-background shrink-0 font-semibold"
            >
              {m.onboarding_step4_import_subjects_btn()}
            </Button>
          </div>
        {/if}

        <!-- Subject Form -->
        <div class="rounded-lg border border-border/40 bg-muted/20 p-4 space-y-4">
          <div class="grid gap-3 sm:grid-cols-2">
            <div class="sm:col-span-2">
              <BilingualNameFields
                arId="onboarding-subject-name-ar"
                enId="onboarding-subject-name-en"
                arLabel={m.onboarding_step4_subject_name_ar()}
                enLabel={m.onboarding_step4_subject_name_en()}
                bind:arValue={subjectNameAr}
                bind:enValue={subjectName}
                arPlaceholder={m.onboarding_step4_subject_name_ar_placeholder()}
                enPlaceholder={m.onboarding_step4_subject_name_en_placeholder()}
                kind="label"
                requestSuggestion={suggestLocalTranslation}
                replaceUneditedInitialValues
                disabled={isSubmittingSubject}
                loadingMessage={m.bilingual_entry_loading()}
                failureMessage={m.bilingual_entry_failure()}
              />
            </div>
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {m.onboarding_step4_subject_code()}
              </Label>
              <Input
                bind:value={subjectCode}
                placeholder="MATH-101"
                class="font-mono text-xs"
              />
            </div>
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {m.onboarding_step4_subject_credits()}
              </Label>
              <Input
                type="number"
                bind:value={subjectCreditHours}
                placeholder="3"
              />
            </div>
          </div>

          <div class="flex justify-end">
            <Button
              variant="default"
              size="sm"
              disabled={isSubmittingSubject || (!subjectName.trim() && !subjectNameAr.trim())}
              onclick={handleCreateSubject}
              class="text-xs font-semibold"
            >
              {isSubmittingSubject ? m.common_control_loading() : m.onboarding_step4_add_subject_btn()}
            </Button>
          </div>
        </div>

        <!-- Subjects Grid -->
        {#if subjects.length > 0}
          <div class="space-y-2">
            <span class="text-xs font-semibold text-muted-foreground uppercase">
              {m.onboarding_step4_configured_subjects()}
            </span>
            <div class="grid gap-2 sm:grid-cols-3">
              {#each subjects as sub (sub.id)}
                <div class="flex items-center justify-between rounded-lg border border-border/50 bg-card p-2.5 text-xs">
                  <div class="min-w-0 flex-1">
                    <div class="font-medium text-foreground truncate">
                      {sub.name_ar || sub.name}
                    </div>
                    <div class="text-[10px] font-mono text-muted-foreground">
                      {sub.code} | {sub.credit_hours || 3}h
                    </div>
                  </div>
                  <Badge variant="secondary" class="text-[10px]">
                    {m.onboarding_step4_subject_core_badge()}
                  </Badge>
                </div>
              {/each}
            </div>
          </div>
        {/if}
      </div>
    </OnboardingSection>

    <!-- Section 2: Classes & Sections -->
    <OnboardingSection
      title={m.onboarding_step4_classes_title()}
      description={m.onboarding_step4_classes_desc()}
      badge={classes.length > 0 ? `${classes.length}` : undefined}
      isComplete={classes.length > 0}
    >
      <div class="space-y-4">
        <!-- Quick Batch Import Classes -->
        {#if classes.length === 0}
          <div class="rounded-lg border border-primary/20 bg-primary/5 p-4 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-3">
            <div>
              <div class="text-xs font-semibold text-foreground">
                👥 {m.onboarding_step4_quick_classes()}
              </div>
              <p class="text-xs text-muted-foreground">
                {m.onboarding_step4_quick_classes_desc()}
              </p>
            </div>
            <Button
              variant="outline"
              size="sm"
              disabled={isSubmittingClass}
              onclick={handleBatchImportClasses}
              class="text-xs bg-background shrink-0 font-semibold"
            >
              {m.onboarding_step4_import_classes_btn()}
            </Button>
          </div>
        {/if}

        <!-- Class Form -->
        <div class="rounded-lg border border-border/40 bg-muted/20 p-4 space-y-4">
          <div class="grid gap-3 sm:grid-cols-2">
            <div class="sm:col-span-2">
              <BilingualNameFields
                arId="onboarding-class-name-ar"
                enId="onboarding-class-name-en"
                arLabel={m.onboarding_step4_class_name_ar()}
                enLabel={m.onboarding_step4_class_name_en()}
                bind:arValue={classNameAr}
                bind:enValue={className}
                arPlaceholder={m.onboarding_step4_class_name_ar_placeholder()}
                enPlaceholder={m.onboarding_step4_class_name_en_placeholder()}
                kind="label"
                requestSuggestion={suggestLocalTranslation}
                replaceUneditedInitialValues
                disabled={isSubmittingClass}
                loadingMessage={m.bilingual_entry_loading()}
                failureMessage={m.bilingual_entry_failure()}
              />
            </div>
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {m.onboarding_step4_class_grade()}
              </Label>
              <Input
                bind:value={classGrade}
                placeholder="Grade 1"
              />
            </div>
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {m.onboarding_step4_class_capacity()}
              </Label>
              <Input
                type="number"
                bind:value={classCapacity}
                placeholder="30"
              />
            </div>
          </div>

          <div class="flex justify-end">
            <Button
              variant="default"
              size="sm"
              disabled={isSubmittingClass || (!className.trim() && !classNameAr.trim())}
              onclick={handleCreateClass}
              class="text-xs font-semibold"
            >
              {isSubmittingClass ? m.common_control_loading() : m.onboarding_step4_add_class_btn()}
            </Button>
          </div>
        </div>

        <!-- Classes Grid -->
        {#if classes.length > 0}
          <div class="space-y-2">
            <span class="text-xs font-semibold text-muted-foreground uppercase">
              {m.onboarding_step4_configured_classes()}
            </span>
            <div class="grid gap-2 sm:grid-cols-3">
              {#each classes as cls (cls.id)}
                <div class="flex items-center justify-between rounded-lg border border-border/50 bg-card p-2.5 text-xs">
                  <div>
                    <div class="font-medium text-foreground">
                      {cls.name_ar || cls.name}
                    </div>
                    <div class="text-[10px] text-muted-foreground">
                      {cls.grade_level || "Grade"} | {cls.capacity || 30}
                    </div>
                  </div>
                  <Badge variant="outline" class="border-emerald-500/30 text-emerald-600 dark:text-emerald-400 text-[10px]">
                    {m.onboarding_active_badge()}
                  </Badge>
                </div>
              {/each}
            </div>
          </div>
        {/if}
      </div>
    </OnboardingSection>

  {/if}
</div>
