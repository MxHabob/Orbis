<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { onMount, tick, untrack } from "svelte";
import { goto } from "$app/navigation";
import { Button } from "@orbis/ui/button";
import { Card } from "@orbis/ui/card";
import { Badge } from "@orbis/ui/badge";
import { ArrowLeft, Check, ChevronDown, Printer } from "@orbis/icons";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@orbis/ui/dropdown-menu";

import { l } from "$lib/core/i18n";
import { localizeHref } from "$lib/paraglide/runtime";
import { StudentFormState } from "./form-state.svelte";
import { createClassesQuery } from "$lib/features/academics";
import { useQueryClient } from "@tanstack/svelte-query";
import { academicContext } from "$lib/stores/academic-context.svelte";
import { effectiveContext } from "$lib/stores/effective-context.svelte";

import AdmissionRules from "./components/admission-rules.svelte";
import ProfileSection from "./sections/profile-section.svelte";
import FamilySection from "./sections/family-section.svelte";
import LocationSection from "./sections/location-section.svelte";
import EnrollmentSection from "./sections/enrollment-section.svelte";
import FinancialsSection from "./sections/financials-section.svelte";
import DocumentsSection from "./sections/documents-section.svelte";

let { studentId, orgId }: { studentId?: string; orgId: string } = $props();

const form = untrack(() => new StudentFormState(orgId, studentId));
const queryClient = useQueryClient();
const classesQuery = createClassesQuery(() => orgId);
let initialFormSnapshot = $state<string | null>(null);
let initialClassId = $state("");
const currentPeriod = $derived(
  academicContext.periods.find(
    (period) => period.id === effectiveContext.currentPeriodId,
  ) ?? null,
);

$effect(() => {
  const classes = classesQuery.data;
  if (!classes) return;

  const contextualIds = [
    effectiveContext.activeClassId,
    academicContext.selectedClassId,
  ];
  const defaultClassId = untrack(() => form.applyClassOptions(classes, contextualIds));
  if (defaultClassId) initialClassId = defaultClassId;
});

async function initializeForm() {
  await form.init();
  if (!form.initializationFailed) {
    initialClassId = form.personal.classId;
    initialFormSnapshot = getFormSnapshot();
  }
}

onMount(() => {
  void initializeForm();
});

const steps = [
  { id: "profile", label: m.common_form_student_info, sections: [ProfileSection] },
  { id: "contacts", label: m.student_registration_step_contacts, sections: [FamilySection, LocationSection] },
  { id: "enrollment", label: m.student_registration_step_enrollment, sections: [EnrollmentSection, FinancialsSection] },
  { id: "documents", label: m.student_registration_step_documents, sections: [DocumentsSection] },
  { id: "review", label: m.student_registration_step_review, sections: [] },
] as const;

type StepId = (typeof steps)[number]["id"];
let activeStep = $state<StepId>("profile");
let returnToReview = $state(false);
let stepPanels = $state<HTMLDivElement>();
let scrollArea = $state<HTMLDivElement>();
const stepIndex = $derived(steps.findIndex((step) => step.id === activeStep));
const isReview = $derived(activeStep === "review");

async function openStep(step: StepId, fromReview = false) {
  if (form.isSaving) return;
  if (step !== activeStep) attemptedSave = null;
  activeStep = step;
  returnToReview = fromReview;
  await tick();
  scrollArea?.scrollTo({ top: 0 });
  stepPanels?.querySelector<HTMLElement>(`#registration-${step}-heading`)?.focus({ preventScroll: true });
}

function findField(id: string) {
  return stepPanels?.querySelector<HTMLElement>(`#${CSS.escape(id)}`);
}

function isFieldInStep(id: string, step: StepId) {
  return findField(id)?.closest("[data-registration-step]")?.getAttribute("data-registration-step") === step;
}

const documentStats = $derived.by(() => {
  const fulfillments = form.document.fulfillments;
  if (fulfillments.length > 0) {
    const fulfilled = fulfillments.filter(
      (item) =>
        item.is_fulfilled ||
        form.document.submittedDocs.includes(item.requirement.code) ||
        !!form.document.attachments[item.requirement.code],
    ).length;
    return { total: fulfillments.length, fulfilled };
  }

  const requirements = form.document.requirements.filter((item) => {
    const selectedClassId = form.personal.classId;
    const selectedClassLevel = form.selectedClass?.level;
    if (!item.target_id || !selectedClassId) return true;
    return (
      item.target_id === selectedClassId ||
      item.target_id === selectedClassLevel
    );
  });
  if (requirements.length > 0) {
    const fulfilled = requirements.filter(
      (item) =>
        form.document.submittedDocs.includes(item.code) ||
        !!form.document.attachments[item.code],
    ).length;
    return { total: requirements.length, fulfilled };
  }

  const requiredDocs = form.document.orgRequiredDocuments;
  return {
    total: requiredDocs.length,
    fulfilled: requiredDocs.filter(
      (code) =>
        form.document.submittedDocs.includes(code) ||
        !!form.document.attachments[code],
    ).length,
  };
});

function getFormSnapshot(): string {
  return JSON.stringify({
    personal: {
      nameAr: form.personal.nameAr,
      nameEn: form.personal.nameEn,
      nationalId: form.personal.nationalId,
      sex: form.personal.sex,
      birthDate: form.personal.birthDate,
      studentNumber: form.personal.studentNumber,
      email: form.personal.email,
      phone: form.personal.phone,
      address: form.personal.address,
      previousSchool: form.personal.previousSchool,
      district: form.personal.district,
      governorate: form.personal.governorate,
      nationality: form.personal.nationality,
      placeOfBirth: form.personal.placeOfBirth,
      passportNumber: form.personal.passportNumber,
      bloodType: form.personal.bloodType,
      emergencyContact: form.personal.emergencyContact,
      medicalNotes: form.personal.medicalNotes,
      specialNeedsNotes: form.personal.specialNeedsNotes,
    },
    guardians: form.guardian.linkedGuardians,
    studyMode: form.studyMode,
    createLoginAccount: form.createLoginAccount,
    financial: {
      applyDiscount: form.financial.applyDiscount,
      discountType: form.financial.discountType,
      selectedDiscountTemplateId: form.financial.selectedDiscountTemplateId,
      customDiscountAmount: form.financial.customDiscountAmount,
      selectedOptionalFees: form.financial.selectedOptionalFees,
    },
    documents: {
      submittedDocs: [...form.document.submittedDocs].sort(),
      attachments: Object.entries(form.document.attachments)
        .map(([code, attachment]) => [
          code,
          attachment.fileName,
          attachment.fileSize,
          attachment.file?.lastModified,
        ])
        .sort(([left], [right]) => String(left).localeCompare(String(right))),
    },
    avatar: form.avatarFile
      ? [
          form.avatarFile.name,
          form.avatarFile.size,
          form.avatarFile.lastModified,
        ]
      : null,
  });
}

const isFormDirty = $derived(
  initialFormSnapshot !== null &&
    (form.personal.classId !== initialClassId || getFormSnapshot() !== initialFormSnapshot),
);

const canSubmit = $derived(
  initialFormSnapshot !== null &&
    !form.isInitializing &&
    !form.initializationFailed &&
    !classesQuery.isLoading &&
    !classesQuery.isError &&
    ((form.isEditMode &&
      form.originalStudent?.student?.status !== "inactive") ||
      (form.admissionPolicy.registrationEnabled && !!currentPeriod)),
);
const canSaveDraft = $derived(
  initialFormSnapshot !== null && !form.isInitializing && !form.initializationFailed,
);
const isSavedDraft = $derived(
  form.isEditMode && form.originalStudent?.student?.status === "inactive",
);

type ValidationAttempt = { draft: boolean; step?: StepId };
let attemptedSave = $state<ValidationAttempt | null>(null);

function getValidationIssue({ draft, step }: ValidationAttempt) {
  return form.getRequiredFieldIssue(draft, (id) => !step || isFieldInStep(id, step)) ??
    form.getAdmissionIssue(draft, step);
}
const requiredFieldIssue = $derived.by(() => {
  const attempt = attemptedSave;
  if (!attempt) return null;
  return getValidationIssue(attempt);
});

async function focusFieldIssue(
  issue: { id: string; step?: StepId },
  attempt: ValidationAttempt,
) {
  const panel = findField(issue.id)?.closest("[data-registration-step]");
  const step = issue.step ??
    steps.find((item) => item.id === panel?.getAttribute("data-registration-step"))?.id;
  if (step && step !== activeStep) await openStep(step, isReview);
  attemptedSave = attempt;
  const field = findField(issue.id) ?? findField(`registration-${activeStep}-heading`);
  field?.focus({ preventScroll: true });
  field?.scrollIntoView({ block: "center", inline: "nearest" });
}

async function advance() {
  const issue = getValidationIssue({ draft: false, step: activeStep });
  if (issue) {
    await focusFieldIssue(issue, { draft: false, step: activeStep });
    return;
  }

  const next = returnToReview ? "review" : steps[stepIndex + 1]?.id;
  if (next) await openStep(next);
}

async function handleSave(saveAsDraft = false) {
  if (form.isSaving || (saveAsDraft ? !canSaveDraft : !canSubmit)) return;
  const issue = getValidationIssue({ draft: saveAsDraft });
  if (issue) {
    await focusFieldIssue(issue, { draft: saveAsDraft });
    return;
  }
  attemptedSave = null;
  await form.handleSave(queryClient, saveAsDraft);
}

const reviewItems = $derived([
  { step: steps[0], value: (form.personal.nameAr.trim() || form.personal.nameEn.trim()) },
  { step: steps[1], value: [
      form.guardian.linkedGuardians.map((guardian) => guardian.full_name.trim()).filter(Boolean).join(" ? "),
      [form.personal.governorate, form.personal.district, form.personal.address].filter(Boolean).join(" ? "),
    ].filter(Boolean).join("\n") },
  { step: steps[2], value: [
      form.selectedClassName,
      l(currentPeriod?.name_ar, currentPeriod?.name),
    ].filter(Boolean).join(" ? ") },
  { step: steps[3], value: documentStats.total
      ? `${documentStats.fulfilled} / ${documentStats.total}`
      : m.ui_copy_no_document_requirements_configured_07292a9d() },
]);

function cancel() {
  goto(localizeHref(`/${orgId}/students`));
}
</script>

<div class="flex h-full min-h-0 w-full min-w-0 flex-col overflow-hidden bg-background text-start text-foreground">
  <header class="shrink-0 border-b border-border bg-background">
    <div class="mx-auto flex w-full max-w-7xl flex-wrap items-center justify-between gap-4 px-4 py-3 sm:px-6 lg:px-8">
      <div class="flex min-w-0 items-start gap-3">
        <Button
          variant="ghost"
          size="icon-sm"
          class="mt-0.5 shrink-0"
          onclick={cancel}
          aria-label={m.common_action_back()}
          disabled={form.isSaving}
        >
          <ArrowLeft class="h-4 w-4 rtl:rotate-180" />
        </Button>
        <div class="min-w-0">
          <div class="flex flex-wrap items-center gap-2">
            <h1 class="truncate text-xl font-semibold">
              {form.isEditMode
                ? form.personal.nameAr || form.personal.nameEn || m.ui_copy_edit_student_8d69c746()
                : m.ui_copy_new_student_admission_37fb2c3c()}
            </h1>
            {#if form.isEditMode}
              <Badge
                variant={form.registrationStatus === "approved"
                  ? "lime"
                  : form.registrationStatus === "pending_review"
                    ? "warning"
                    : "secondary"}
              >
                {form.registrationStatus === "approved"
                  ? m.ui_copy_approved_39578cc4()
                  : form.registrationStatus === "pending_review"
                    ? m.ui_copy_under_review_46b36239()
                    : form.registrationStatus === "active"
                      ? m.common_phase_active()
                    : form.registrationStatus === "inactive"
                      ? m.common_status_draft()
                      : form.registrationStatus === "withdrawn"
                      ? m.common_status_withdrawn()
                      : form.registrationStatus === "graduated"
                        ? m.ui_copy_graduated_dc58b020()
                        : m.ui_copy_draft_873425fa()}
              </Badge>
            {/if}
          </div>
          {#if form.personal.studentNumber}
            <p class="mt-1 text-sm text-muted-foreground">{form.personal.studentNumber}</p>
          {/if}
        </div>
      </div>

      {#if form.availableTemplates.length > 0}
        <DropdownMenu>
          <DropdownMenuTrigger>
            {#snippet child({ props })}
            <Button {...props} variant="outline" size="sm" class="gap-2">
              <Printer class="h-4 w-4" />
              <span>{m.ui_copy_print_form_ce69ef6c()}</span>
              <ChevronDown class="h-3.5 w-3.5" />
            </Button>
            {/snippet}
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end" class="min-w-48">
            {#each form.availableTemplates as template (template.id)}
              <DropdownMenuItem
                onclick={async () => {
                  form.selectedTemplateId = template.id;
                  await form.handleDownloadRegistrationForm();
                }}
              >
                <span>{template.name_ar || template.name}</span>
                {#if form.selectedTemplateId === template.id}
                  <Check class="ms-auto h-4 w-4" />
                {/if}
              </DropdownMenuItem>
            {/each}
          </DropdownMenuContent>
        </DropdownMenu>
      {/if}
    </div>
  </header>

  <div bind:this={scrollArea} class="min-h-0 flex-1 overflow-y-auto overscroll-contain">
    <div class="mx-auto w-full max-w-7xl space-y-4 px-4 py-4 sm:px-6 lg:px-8">

      {#if form.isInitializing || (initialFormSnapshot === null && !form.initializationFailed)}
        <div role="status" class="flex items-center justify-center gap-2 py-12 text-sm text-muted-foreground">
          <LoadingIndicator size={18} />
          {m.common_loading()}
        </div>
      {:else if form.initializationFailed}
        <div
          role="alert"
          class="rounded-lg border border-destructive/30 bg-destructive/5 px-4 py-3 text-sm text-destructive"
        >
          {m.student_registration_initialization_error()}
          <Button variant="outline" size="sm" class="ms-2" onclick={initializeForm}>{m.common_action_retry()}</Button>
        </div>
      {:else if classesQuery.isError}
        <div
          role="alert"
          class="rounded-lg border border-destructive/30 bg-destructive/5 px-4 py-3 text-sm text-destructive"
        >
          {m.student_registration_classes_load_error()}
          <Button variant="outline" size="sm" class="ms-2" onclick={() => classesQuery.refetch()} disabled={classesQuery.isFetching}>{m.common_action_retry()}</Button>
        </div>
      {:else if academicContext.isInitialized && !currentPeriod && (!form.isEditMode || isSavedDraft)}
        <div
          role="alert"
          class="rounded-lg border border-amber-500/30 bg-amber-500/5 px-4 py-3 text-sm text-amber-700 dark:text-amber-300"
        >
          {m.student_registration_period_required()}
        </div>
      {:else if !form.admissionPolicy.registrationEnabled && (!form.isEditMode || isSavedDraft)}
        <div
          role="alert"
          class="rounded-lg border border-amber-500/30 bg-amber-500/5 px-4 py-3 text-sm text-amber-700 dark:text-amber-300"
        >
          {m.student_registration_disabled()}
        </div>
      {:else if currentPeriod && !currentPeriod.is_enrollment_open && !form.admissionPolicy.allowLateEnrollment && (!form.isEditMode || isSavedDraft)}
        <div
          role="alert"
          class="rounded-lg border border-amber-500/30 bg-amber-500/5 px-4 py-3 text-sm text-amber-700 dark:text-amber-300"
        >
          {m.student_registration_period_closed()}
        </div>
      {/if}

      {#if initialFormSnapshot !== null && !form.isInitializing && !form.initializationFailed}
        <nav aria-label={m.student_registration_step_navigation()} class="rounded-xl border border-border bg-card p-2">
          <ol class="flex flex-wrap gap-1">
            {#each steps as step, index (step.id)}
              <li class="flex-1">
                <Button
                  variant={step.id === activeStep ? "secondary" : "ghost"}
                  size="sm"
                  class="w-full justify-center gap-2 text-start"
                  aria-current={step.id === activeStep ? "step" : undefined}
                  disabled={form.isSaving}
                  onclick={() => openStep(step.id)}
                >
                  <span
                    class="grid size-5 shrink-0 place-items-center rounded-full text-xs tabular-nums {step.id === activeStep ? 'bg-primary text-primary-foreground' : 'bg-muted text-muted-foreground'}"
                    aria-hidden="true"
                  >{index + 1}</span>
                  <span>{step.label()}</span>
                </Button>
              </li>
            {/each}
          </ol>
        </nav>

        <div bind:this={stepPanels} class="w-full min-w-0">
            {#each steps as step, index (step.id)}
              <section
                data-registration-step={step.id}
                aria-labelledby={`registration-${step.id}-heading`}
                hidden={activeStep !== step.id}
                class="@container/form-section min-w-0"
              >
                <h2 id={`registration-${step.id}-heading`} tabindex="-1" class="sr-only">
                  {step.label()} ? {m.student_registration_step_position({ current: index + 1, total: steps.length })}
                </h2>

                {#if step.id === "review"}
                  <Card class="gap-0">
                    <dl class="divide-y divide-border">
                      {#each reviewItems as item (item.step.id)}
                        <div class="grid grid-cols-[minmax(0,1fr)_auto] items-center gap-x-3 gap-y-1 px-4 py-3 sm:px-5">
                          <dt class="text-sm font-medium">{item.step.label()}</dt>
                          <dd class="col-start-1 row-start-2 text-sm whitespace-pre-line text-muted-foreground [overflow-wrap:anywhere]" dir="auto">
                            {item.value || m.student_registration_review_empty()}
                          </dd>
                          <dd class="col-start-2 row-span-2 row-start-1">
                            <Button
                              variant="ghost"
                              size="sm"
                              aria-label={m.student_registration_review_edit({ section: item.step.label() })}
                              disabled={form.isSaving}
                              onclick={() => openStep(item.step.id, true)}
                            >{m.common_action_edit()}</Button>
                          </dd>
                        </div>
                      {/each}
                    </dl>
                  </Card>
                  <div class="mt-4">
                    <AdmissionRules state={form} period={currentPeriod} />
                  </div>
                {:else}
                  <div class="grid items-start gap-4 {step.sections.length > 1 ? '@min-[54rem]/form-section:grid-cols-2' : ''}">
                    {#each step.sections as Section (Section)}
                      <div class="@container/registration-panel min-w-0">
                        <Section state={form} />
                      </div>
                    {/each}
                  </div>
                {/if}
              </section>
            {/each}
        </div>
      {/if}
    </div>
  </div>

  <footer class="shrink-0 border-t border-border bg-background">
    <div class="mx-auto flex w-full max-w-7xl flex-wrap items-center justify-between gap-3 px-4 py-3 sm:px-6 lg:px-8">
      {#if requiredFieldIssue}
        <p role="alert" class="text-sm font-medium text-destructive">{requiredFieldIssue.message}</p>
      {/if}
      <div class="ms-auto flex flex-wrap items-center justify-end gap-2">
        {#if !form.isEditMode || isSavedDraft}
          <Button
            variant="outline"
            onclick={() => handleSave(true)}
            disabled={form.isSaving || !canSaveDraft || !isFormDirty}
          >
            {m.student_registration_save_draft_short()}
          </Button>
        {/if}
        {#if stepIndex > 0}
          <Button variant="ghost" onclick={() => openStep(steps[stepIndex - 1].id)} disabled={form.isSaving}>
            {m.common_action_back()}
          </Button>
        {/if}
        {#if !isReview && (!form.isEditMode || isSavedDraft)}
          <Button class="min-w-32" onclick={advance} disabled={!canSaveDraft || form.isSaving}>
            {returnToReview ? m.student_registration_review_return() : m.ui_onboarding_onboarding_footer_continue()}
          </Button>
        {:else}
        <Button
          class="min-w-36"
          onclick={() => handleSave(false)}
          disabled={form.isSaving || !canSubmit || (!isFormDirty && !isSavedDraft)}
        >
          {#if form.isSaving}
            <LoadingIndicator class="me-2" size={15} />
            {m.common_state_saving()}
          {:else if form.isEditMode}
            {#if isSavedDraft}
              {m.student_registration_complete_draft()}
            {:else}
              {m.settings_action_save_changes()}
            {/if}
          {:else if form.admissionPolicy.requiresApproval}
            {m.ui_copy_submit_for_approval_4cea7b71()}
          {:else}
            {m.ui_copy_save_direct_enroll_ed9ff0dc()}
          {/if}
        </Button>
        {/if}
      </div>
    </div>
  </footer>
</div>
