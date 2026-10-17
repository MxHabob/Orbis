<script lang="ts">
  import { LoadingIndicator } from "@orbis/ui";
  import * as m from "$lib/paraglide/messages";
  import { onMount, untrack } from "svelte";
  import { goto } from "$app/navigation";
  import { Button } from "@orbis/ui/button";
  import { Badge } from "@orbis/ui/badge";
  import { ArrowLeft, Check, ChevronDown, Printer } from "@orbis/icons";
  import {
    Card,
    CardContent,
    CardHeader,
    CardTitle,
  } from "@orbis/ui/card";
  import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuTrigger,
  } from "@orbis/ui/dropdown-menu";
  import {
    Accordion,
    AccordionContent,
    AccordionItem,
    AccordionTrigger,
  } from "@orbis/ui/accordion";

  import { localizeHref } from "$lib/paraglide/runtime";
  import { StudentFormState } from "./form-state.svelte";
  import { createClassesQuery } from "$lib/features/academics";
  import { useQueryClient } from "@tanstack/svelte-query";
  import { academicContext } from "$lib/stores/academic-context.svelte";
  import { effectiveContext } from "$lib/stores/effective-context.svelte";

  import ProfileSection from "./sections/profile-section.svelte";
  import FamilySection from "./sections/family-section.svelte";
  import LocationSection from "./sections/location-section.svelte";
  import FinancialsSection from "./sections/financials-section.svelte";
  import DocumentsSection from "./sections/documents-section.svelte";

  let { studentId, orgId }: { studentId?: string; orgId: string } = $props();

  const state = untrack(() => new StudentFormState(orgId || "", studentId));
  const queryClient = useQueryClient();
  const classesQuery = createClassesQuery(() => orgId);
  const currentPeriod = $derived(
    academicContext.periods.find(
      (period) => period.id === effectiveContext.currentPeriodId,
    ) ?? null,
  );

  $effect(() => {
    const classes = classesQuery.data;
    if (!classes) return;

    state.classes = classes;
    if (!state.isEditMode && !state.personal.classId) {
      const contextualClassId =
        effectiveContext.activeClassId || academicContext.selectedClassId;
      if (classes.some((classItem) => classItem.id === contextualClassId)) {
        state.personal.classId = contextualClassId;
      }
    }
  });

  onMount(async () => {
    if (orgId) state.orgId = orgId;
    if (studentId) {
      state.studentId = studentId;
      state.isEditMode = true;
    }
    await state.init();
  });

  const documentStats = $derived.by(() => {
    const fulfillments = state.document.fulfillments;
    if (fulfillments.length > 0) {
      const fulfilled = fulfillments.filter(
        (item) =>
          item.is_fulfilled
          || state.document.submittedDocs.includes(item.requirement.code)
          || !!state.document.attachments[item.requirement.code],
      ).length;
      return { total: fulfillments.length, fulfilled };
    }

    const requirements = state.document.requirements.filter((item) => {
      const selectedClassId = state.personal.classId;
      const selectedClassLevel = state.selectedClass?.level;
      if (!item.target_id || !selectedClassId) return true;
      return item.target_id === selectedClassId || item.target_id === selectedClassLevel;
    });
    if (requirements.length > 0) {
      const fulfilled = requirements.filter(
        (item) =>
          state.document.submittedDocs.includes(item.code)
          || !!state.document.attachments[item.code],
      ).length;
      return { total: requirements.length, fulfilled };
    }

    const requiredDocs = state.document.orgRequiredDocuments;
    return {
      total: requiredDocs.length,
      fulfilled: requiredDocs.filter(
        (code) =>
          state.document.submittedDocs.includes(code)
          || !!state.document.attachments[code],
      ).length,
    };
  });

  const isFormDirty = $derived.by(
    () =>
      !!state.personal.nameAr
      || !!state.personal.nameEn
      || !!state.personal.classId
      || !!state.personal.nationalId
      || !!state.personal.phone
      || !!state.personal.email
      || state.guardian.linkedGuardians.some(
        (guardian) => !!guardian.guardian_id || !!guardian.full_name || !!guardian.phone,
      )
      || state.document.submittedDocs.length > 0
      || Object.keys(state.document.attachments).length > 0
      || !!state.avatarFile,
  );

  const canSubmit = $derived(
    !state.isInitializing
    && !state.initializationFailed
    && !classesQuery.isLoading
    && !classesQuery.isError
    && (
      (state.isEditMode && state.originalStudent?.student?.status !== "inactive")
      || (
        state.admissionPolicy.registrationEnabled
        && !!currentPeriod
        && (
          currentPeriod.is_enrollment_open
          || state.admissionPolicy.allowLateEnrollment
        )
      )
    ),
  );
  const canSaveDraft = $derived(
    !state.isInitializing && !state.initializationFailed,
  );
  const isSavedDraft = $derived(
    state.isEditMode && state.originalStudent?.student?.status === "inactive",
  );

  async function handleSave(saveAsDraft = false) {
    if (!saveAsDraft && (!state.isEditMode || isSavedDraft)) {
      if (!state.admissionPolicy.registrationEnabled) {
        return;
      }
      if (!currentPeriod) {
        return;
      }
      if (
        !currentPeriod.is_enrollment_open
        && !state.admissionPolicy.allowLateEnrollment
      ) {
        return;
      }
    }
    await state.handleSave(queryClient, saveAsDraft);
  }

  function cancel() {
    goto(localizeHref(`/${orgId}/students`));
  }
</script>

<div class="h-full w-full overflow-y-auto text-foreground text-start pb-24">
  <div class="mx-auto flex w-full max-w-7xl flex-col gap-5 px-4 py-5 sm:px-6 lg:px-8">
    <header class="flex flex-wrap items-start justify-between gap-4 border-b border-border/70 pb-4">
      <div class="flex min-w-0 items-start gap-3">
        <Button
          variant="ghost"
          size="icon-sm"
          class="mt-0.5 shrink-0"
          onclick={cancel}
          aria-label={m.common_action_cancel()}
        >
          <ArrowLeft class="h-4 w-4 rtl:rotate-180" />
        </Button>
        <div class="min-w-0">
          <div class="flex flex-wrap items-center gap-2">
            <h1 class="truncate text-xl font-semibold">
              {state.personal.nameAr
                || state.personal.nameEn
                || (state.isEditMode
                  ? m.ui_copy_edit_student_8d69c746()
                  : m.ui_copy_new_student_admission_37fb2c3c())}
            </h1>
            {#if state.isEditMode}
              <Badge
                variant={state.registrationStatus === "approved"
                  ? "lime"
                  : state.registrationStatus === "pending_review"
                    ? "warning"
                    : "secondary"}
              >
                {state.registrationStatus === "approved"
                  ? m.ui_copy_approved_39578cc4()
                  : state.registrationStatus === "pending_review"
                    ? m.ui_copy_under_review_46b36239()
                    : state.registrationStatus === "active"
                      ? m.common_phase_active()
                    : state.registrationStatus === "inactive"
                      ? m.common_status_draft()
                      : state.registrationStatus === "withdrawn"
                      ? m.common_status_withdrawn()
                      : state.registrationStatus === "graduated"
                        ? m.ui_copy_graduated_dc58b020()
                        : m.ui_copy_draft_873425fa()}
              </Badge>
            {/if}
          </div>
          <p class="mt-1 text-sm text-muted-foreground">
            {state.personal.studentNumber
              || m.ui_copy_auto_generated_academic_id_a05c74f0()}
          </p>
        </div>
      </div>

      {#if state.availableTemplates.length > 0}
        <DropdownMenu>
          <DropdownMenuTrigger>
            <Button variant="outline" size="sm" class="gap-2">
              <Printer class="h-4 w-4" />
              <span>
                {state.availableTemplates.find(
                  (template) => template.id === state.selectedTemplateId,
                )?.name_ar
                  || state.availableTemplates.find(
                    (template) => template.id === state.selectedTemplateId,
                  )?.name
                  || m.ui_copy_print_form_ce69ef6c()}
              </span>
              <ChevronDown class="h-3.5 w-3.5" />
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end" class="min-w-48">
            {#each state.availableTemplates as template (template.id)}
              <DropdownMenuItem
                onclick={async () => {
                  state.selectedTemplateId = template.id;
                  await state.handleDownloadRegistrationForm();
                }}
              >
                <span>{template.name_ar || template.name}</span>
                {#if state.selectedTemplateId === template.id}
                  <Check class="ms-auto h-4 w-4" />
                {/if}
              </DropdownMenuItem>
            {/each}
          </DropdownMenuContent>
        </DropdownMenu>
      {/if}
    </header>

    <Card class="border-border/70 shadow-xs">
      <CardHeader class="pb-2">
        <CardTitle class="text-sm font-semibold">
          {m.student_registration_policy_title()}
        </CardTitle>
      </CardHeader>
      <CardContent class="pt-0">
        <Accordion
          type="multiple"
          value={["enrollment"]}
          class="flex flex-col divide-y divide-border"
        >
          <AccordionItem value="enrollment" class="border-0">
            <AccordionTrigger class="py-3 text-sm font-medium hover:no-underline">
              {m.student_registration_enrollment_rules()}
            </AccordionTrigger>
            <AccordionContent>
              <div class="grid gap-4 pb-3 sm:grid-cols-2 xl:grid-cols-3">
                <div class="min-w-0">
                  <p class="text-xs text-muted-foreground">{m.student_registration_active_period()}</p>
                  <p class="truncate text-sm font-medium">
                    {currentPeriod?.name_ar || currentPeriod?.name || "—"}
                  </p>
                </div>
                <div class="min-w-0">
                  <p class="text-xs text-muted-foreground">
                    {m.ui_copy_enable_registration_portal_dbaf8a8e()}
                  </p>
                  <p class="text-sm font-medium">
                    {state.admissionPolicy.registrationEnabled
                      ? m.student_registration_policy_enabled()
                      : m.student_registration_policy_disabled()}
                  </p>
                </div>
                <div class="min-w-0">
                  <p class="text-xs text-muted-foreground">
                    {m.ui_copy_require_academic_registrar_admin_approval_b147c1b8()}
                  </p>
                  <p class="text-sm font-medium">
                    {state.admissionPolicy.requiresApproval
                      ? m.ui_copy_under_review_46b36239()
                      : m.ui_copy_approved_39578cc4()}
                  </p>
                </div>
                <div class="min-w-0">
                  <p class="text-xs text-muted-foreground">
                    {m.ui_copy_allow_late_enrollment_06114ddb()}
                  </p>
                  <p class="text-sm font-medium">
                    {state.admissionPolicy.allowLateEnrollment
                      ? m.student_registration_policy_enabled()
                      : m.student_registration_policy_disabled()}
                  </p>
                </div>
              </div>
            </AccordionContent>
          </AccordionItem>

          <AccordionItem value="capacity" class="border-0">
            <AccordionTrigger class="py-3 text-sm font-medium hover:no-underline">
              {m.student_registration_capacity_rules()}
            </AccordionTrigger>
            <AccordionContent>
              <div class="grid gap-4 pb-3 sm:grid-cols-2 xl:grid-cols-3">
                <div class="min-w-0">
                  <p class="text-xs text-muted-foreground">
                    {m.ui_copy_default_class_capacity_limit_700d9cac()}
                  </p>
                  <p class="text-sm font-medium">
                    {state.selectedClass?.capacity ?? state.admissionPolicy.maxClassCapacity}
                  </p>
                </div>
                <div class="min-w-0">
                  <p class="text-xs text-muted-foreground">
                    {m.ui_copy_enable_waitlist_on_capacity_189ed812()}
                  </p>
                  <p class="text-sm font-medium">
                    {state.admissionPolicy.allowWaitlist
                      ? m.student_registration_policy_enabled()
                      : m.student_registration_policy_disabled()}
                  </p>
                </div>
                <div class="min-w-0">
                  <p class="text-xs text-muted-foreground">
                    {m.ui_copy_sibling_priority_enrollment_b6d49ba5()}
                  </p>
                  <p class="text-sm font-medium">
                    {state.admissionPolicy.siblingPriorityEnabled
                      ? m.student_registration_policy_enabled()
                      : m.student_registration_policy_disabled()}
                  </p>
                </div>
              </div>
            </AccordionContent>
          </AccordionItem>

          <AccordionItem value="identity" class="border-0">
            <AccordionTrigger class="py-3 text-sm font-medium hover:no-underline">
              {m.student_registration_identity_rules()}
            </AccordionTrigger>
            <AccordionContent>
              <div class="grid gap-4 pb-3 sm:grid-cols-2">
                <div class="min-w-0">
                  <p class="text-xs text-muted-foreground">
                    {m.ui_copy_allow_independent_enrollment_no_guardian_7ce5a41b()}
                  </p>
                  <p class="text-sm font-medium">
                    {state.admissionPolicy.allowNoGuardian
                      ? m.bulk_entry_workbook_optional()
                      : m.common_validation_guardian_name_required()}
                  </p>
                </div>
                <div class="min-w-0">
                  <p class="text-xs text-muted-foreground">
                    {m.ui_copy_national_id_c94d5a1a()}
                  </p>
                  <p class="text-sm font-medium">
                    {state.admissionPolicy.allowDuplicateNationalId
                      ? m.student_registration_duplicate_nid_allowed()
                      : m.student_registration_duplicate_nid_blocked()}
                  </p>
                </div>
              </div>
            </AccordionContent>
          </AccordionItem>

          <AccordionItem value="finance" class="border-0">
            <AccordionTrigger class="py-3 text-sm font-medium hover:no-underline">
              {m.student_registration_finance_rules()}
            </AccordionTrigger>
            <AccordionContent>
              <div class="pb-3">
                <p class="text-xs text-muted-foreground">
                  {m.ui_copy_auto_assign_tuition_fee_template_on_enroll_5ed6c3b1()}
                </p>
                <p class="text-sm font-medium">
                  {state.admissionPolicy.autoAssignFees
                    ? m.student_registration_policy_enabled()
                    : m.student_registration_policy_disabled()}
                </p>
              </div>
            </AccordionContent>
          </AccordionItem>
        </Accordion>
      </CardContent>
    </Card>

    {#if state.initializationFailed}
      <div
        role="alert"
        class="rounded-lg border border-destructive/30 bg-destructive/5 px-4 py-3 text-sm text-destructive"
      >
        {m.student_registration_initialization_error()}
      </div>
    {:else if classesQuery.isError}
      <div
        role="alert"
        class="rounded-lg border border-destructive/30 bg-destructive/5 px-4 py-3 text-sm text-destructive"
      >
        {m.student_registration_classes_load_error()}
      </div>
    {:else if academicContext.isInitialized && !currentPeriod && (!state.isEditMode || isSavedDraft)}
      <div
        role="alert"
        class="rounded-lg border border-amber-500/30 bg-amber-500/5 px-4 py-3 text-sm text-amber-700 dark:text-amber-300"
      >
        {m.student_registration_period_required()}
      </div>
    {:else if !state.admissionPolicy.registrationEnabled && (!state.isEditMode || isSavedDraft)}
      <div
        role="alert"
        class="rounded-lg border border-amber-500/30 bg-amber-500/5 px-4 py-3 text-sm text-amber-700 dark:text-amber-300"
      >
        {m.student_registration_disabled()}
      </div>
    {:else if currentPeriod && !currentPeriod.is_enrollment_open && !state.admissionPolicy.allowLateEnrollment && (!state.isEditMode || isSavedDraft)}
      <div
        role="alert"
        class="rounded-lg border border-amber-500/30 bg-amber-500/5 px-4 py-3 text-sm text-amber-700 dark:text-amber-300"
      >
        {m.student_registration_period_closed()}
      </div>
    {/if}

    {#if documentStats.total > 0}
      <div class="flex flex-wrap items-center gap-2 text-sm text-muted-foreground">
        <span>{m.common_form_documents_title()}</span>
        <Badge variant={documentStats.fulfilled === documentStats.total ? "lime" : "outline"}>
          {documentStats.fulfilled} / {documentStats.total}
        </Badge>
      </div>
    {/if}

    <div class="flex flex-col gap-4">
      <ProfileSection {state} />
      <FamilySection {state} />
      <LocationSection {state} />
      <FinancialsSection {state} />
      <DocumentsSection {state} />
    </div>
  </div>

  <div class="fixed inset-x-0 bottom-0 z-40 border-t border-border bg-background/95 px-4 py-3 backdrop-blur-sm">
    <div class="mx-auto flex max-w-7xl items-center justify-between gap-3">
      <p class="hidden min-w-0 truncate text-sm text-muted-foreground sm:block">
        {#if state.isEditMode}
          {state.personal.nameAr || state.personal.nameEn || m.settings_action_save_changes()}
        {:else if state.admissionPolicy.requiresApproval}
          {m.student_registration_approval_policy()}
        {:else}
          {m.student_registration_direct_policy()}
        {/if}
      </p>
      <div class="ms-auto flex flex-wrap items-center justify-end gap-2">
        <Button variant="ghost" onclick={cancel} disabled={state.isSaving}>
          {m.common_action_cancel()}
        </Button>
        {#if !state.isEditMode || isSavedDraft}
          <Button
            variant="outline"
            onclick={() => handleSave(true)}
            disabled={state.isSaving || !canSaveDraft || !isFormDirty}
          >
            {m.student_registration_save_draft()}
          </Button>
        {/if}
        <Button
          variant="lime"
          class="min-w-36"
          onclick={() => handleSave(false)}
          disabled={state.isSaving || !canSubmit || (!isFormDirty && !state.isEditMode)}
        >
          {#if state.isSaving}
            <LoadingIndicator class="me-2" size={15} />
            {m.common_state_saving()}
          {:else if state.isEditMode}
            {#if isSavedDraft}
              {m.student_registration_complete_draft()}
            {:else}
              {m.settings_action_save_changes()}
            {/if}
          {:else if state.admissionPolicy.requiresApproval}
            {m.ui_copy_submit_for_approval_4cea7b71()}
          {:else}
            {m.ui_copy_save_direct_enroll_ed9ff0dc()}
          {/if}
        </Button>
      </div>
    </div>
  </div>
</div>
