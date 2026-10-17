<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import { onMount } from "svelte";
import * as m from "$lib/paraglide/messages";
import { toast } from "@orbis/ui";
import { api, type SubjectDto, type ClassDto, type OrgUnitDto } from "$lib/api";
import { systemStore } from "$lib/stores";
import { l } from "$lib/core/i18n";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Badge } from "@orbis/ui/badge";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import OnboardingStepHeader from "../onboarding-step-header.svelte";
import OnboardingSection from "../onboarding-section.svelte";
import OnboardingPresetBar from "../components/onboarding-preset-bar.svelte";
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
let orgUnits = $state<OrgUnitDto[]>([]);

// Subject Creation Form
let subjectName = $state("");
let subjectNameAr = $state("");
let subjectCode = $state("");
let subjectCreditHours = $state("3");

// Class Creation Form
let className = $state("");
let classNameAr = $state("");
let classGrade = $state("");
let classTrack = $state("");
let classCapacity = $state<number>(30);
let selectedUnitId = $state<string>("");

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
  { name: "Grade 1 - A", nameAr: "الصف الأول (أ)", grade: "Grade 1" },
  { name: "Grade 2 - A", nameAr: "الصف الثاني (أ)", grade: "Grade 2" },
  { name: "Grade 3 - A", nameAr: "الصف الثالث (أ)", grade: "Grade 3" },
  { name: "Grade 4 - A", nameAr: "الصف الرابع (أ)", grade: "Grade 4" },
  { name: "Grade 5 - A", nameAr: "الصف الخامس (أ)", grade: "Grade 5" },
  { name: "Grade 6 - A", nameAr: "الصف السادس (أ)", grade: "Grade 6" },
];

const stepTasks = $derived(getTasksForStep("academics", systemStore.onboardingTasks));
const isTaskComplete = $derived(stepTasks.length > 0 && stepTasks.every((t) => t.status === "completed"));
const remainingStandardSubjects = $derived(
  STANDARD_SUBJECTS.filter(
    (preset) => !subjects.some((subject) => subject.code.toLowerCase() === preset.code.toLowerCase()),
  ),
);
const remainingStandardClasses = $derived(
  STANDARD_CLASSES.filter(
    (preset) =>
      !classes.some(
        (classItem) =>
          String(classItem.unit_id) === selectedUnitId &&
          (classItem.name.trim().toLowerCase() === preset.name.toLowerCase() ||
            classItem.name_ar?.trim().toLowerCase() === preset.nameAr.toLowerCase()) &&
          classItem.level.trim().toLowerCase() === preset.grade.toLowerCase(),
      ),
  ),
);

function applySubjectPreset(subject: typeof STANDARD_SUBJECTS[number]) {
  subjectName = subject.name;
  subjectNameAr = subject.nameAr;
  subjectCode = subject.code;
  subjectCreditHours = subject.credits;
}

function applyClassPreset(classPreset: typeof STANDARD_CLASSES[number]) {
  className = classPreset.name;
  classNameAr = classPreset.nameAr;
  classGrade = classPreset.grade;
  classCapacity = 30;
}

onMount(async () => {
  await loadData();
});

async function loadData() {
  try {
    isLoading = true;
    const [subRes, classRes, unitRes] = await Promise.all([
      api.listSubjects(),
      api.listSections(),
      api.listOrgUnits(),
    ]);
    subjects = subRes;
    classes = classRes;
    orgUnits = unitRes;
    if (orgUnits.length > 0 && !selectedUnitId) {
      selectedUnitId = String(orgUnits[0].id);
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
  const resolvedCode =
    subjectCode.trim() || `SUB-${String(subjects.length + 1).padStart(2, "0")}`;
  if (subjects.some((subject) => subject.code.toLowerCase() === resolvedCode.toLowerCase())) {
    toast.error(m.onboarding_step4_subject_duplicate_code());
    return;
  }

  isSubmittingSubject = true;
  successMessage = null;

  try {
    const newSubject = await api.createSubject({
      code: resolvedCode,
      name: subjectName.trim() || subjectNameAr.trim(),
      name_ar: subjectNameAr.trim() || undefined,
      credit_hours: subjectCreditHours,
      is_core: true,
    });

    subjects = [...subjects, newSubject];
    subjectName = "";
    subjectNameAr = "";
    subjectCode = "";
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
  if (remainingStandardSubjects.length === 0) return;
  isSubmittingSubject = true;
  successMessage = null;

  try {
    for (const sub of remainingStandardSubjects) {
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
  if (!selectedUnitId) {
    toast.error(m.shared_ui_organizational_unit_required());
    return;
  }
  if (!Number.isInteger(classCapacity) || classCapacity <= 0) {
    toast.error(m.ui_copy_settings_contain_invalid_values_fae2c24c());
    return;
  }
  const resolvedName = className.trim() || classNameAr.trim();
  const resolvedLevel = classGrade.trim() || "1";
  if (
    classes.some(
      (classItem) =>
        String(classItem.unit_id) === selectedUnitId &&
        (classItem.name.trim().toLowerCase() === resolvedName.toLowerCase() ||
          (!!classNameAr.trim() &&
            classItem.name_ar?.trim().toLowerCase() === classNameAr.trim().toLowerCase())) &&
        classItem.level.trim().toLowerCase() === resolvedLevel.toLowerCase(),
    )
  ) {
    toast.error(m.onboarding_step4_class_duplicate());
    return;
  }

  isSubmittingClass = true;
  successMessage = null;

  try {
    const newClass = await api.createSection({
      unit_id: selectedUnitId,
      name: resolvedName,
      name_ar: classNameAr.trim() || undefined,
      level: resolvedLevel,
      track: classTrack.trim() || undefined,
      capacity: classCapacity,
    });

    classes = [...classes, newClass];
    className = "";
    classNameAr = "";
    classGrade = "";
    classTrack = "";
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
  if (!selectedUnitId) {
    toast.error(m.shared_ui_organizational_unit_required());
    return;
  }
  const pendingClasses = remainingStandardClasses;
  if (pendingClasses.length === 0) return;
  isSubmittingClass = true;
  successMessage = null;

  try {
    for (const cls of pendingClasses) {
      const created = await api.createSection({
        unit_id: selectedUnitId,
        name: cls.name,
        name_ar: cls.nameAr,
        level: cls.grade,
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
        <OnboardingPresetBar
          title={m.onboarding_step4_quick_subjects()}
          description={m.onboarding_step4_quick_subjects_desc()}
        >
          {#each STANDARD_SUBJECTS as subject (subject.code)}
            <Button
              variant="outline"
              size="sm"
              disabled={isSubmittingSubject}
              onclick={() => applySubjectPreset(subject)}
              class="text-xs"
            >
              {l(subject.nameAr, subject.name)}
            </Button>
          {/each}
          {#if remainingStandardSubjects.length > 0}
            <Button
              variant="ghost"
              size="sm"
              disabled={isSubmittingSubject}
              onclick={handleBatchImportSubjects}
              class="text-xs"
            >
              {m.onboarding_step4_import_subjects_btn()}
            </Button>
          {/if}
        </OnboardingPresetBar>

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
                placeholder="MATH-01"
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
        <OnboardingPresetBar
          title={m.onboarding_step4_quick_classes()}
          description={m.onboarding_step4_quick_classes_desc()}
        >
          {#each STANDARD_CLASSES as classPreset (classPreset.name)}
            <Button
              variant="outline"
              size="sm"
              disabled={isSubmittingClass}
              onclick={() => applyClassPreset(classPreset)}
              class="text-xs"
            >
              {l(classPreset.nameAr, classPreset.name)}
            </Button>
          {/each}
          {#if remainingStandardClasses.length > 0}
            <Button
              variant="ghost"
              size="sm"
              disabled={isSubmittingClass}
              onclick={handleBatchImportClasses}
              class="text-xs"
            >
              {m.onboarding_step4_import_classes_btn()}
            </Button>
          {/if}
        </OnboardingPresetBar>

        <!-- Class Form -->
        <div class="rounded-lg border border-border/40 bg-muted/20 p-4 space-y-4">
          <div class="grid gap-3 sm:grid-cols-2">
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">{m.common_word_unit()}</Label>
              <Select
                type="single"
                bind:value={selectedUnitId}
                disabled={orgUnits.length <= 1 || isSubmittingClass}
              >
                <SelectTrigger class="w-full">
                  {orgUnits.find((unit) => String(unit.id) === selectedUnitId)?.name_ar ||
                    orgUnits.find((unit) => String(unit.id) === selectedUnitId)?.name ||
                    m.common_word_unit()}
                </SelectTrigger>
                <SelectContent>
                  {#each orgUnits as unit (unit.id)}
                    <SelectItem value={String(unit.id)}>
                      {unit.name_ar || unit.name}
                    </SelectItem>
                  {/each}
                </SelectContent>
              </Select>
            </div>
          </div>
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
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {m.settings_general_educational_track()}
              </Label>
              <Input bind:value={classTrack} disabled={isSubmittingClass} />
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
                      {cls.level} | {cls.capacity || 30}
                      {#if orgUnits.find((unit) => String(unit.id) === String(cls.unit_id))}
                        · {orgUnits.find((unit) => String(unit.id) === String(cls.unit_id))?.name_ar ||
                          orgUnits.find((unit) => String(unit.id) === String(cls.unit_id))?.name}
                      {/if}
                      {#if periods.find((period) => String(period.id) === String(cls.period_id))}
                        · {periods.find((period) => String(period.id) === String(cls.period_id))?.name}
                      {/if}
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
