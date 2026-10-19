<!--
  @component SectionSubjectForm
  Form for assigning a subject to a teacher for a specific class and academic period.
  Used in both create and edit modes via a Sheet panel.
  Utilizes self-fetching Searchable Select components for periods, classes, subjects, and teachers.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { untrack } from "svelte";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { SheetHeader, SheetTitle } from "@orbis/ui/sheet";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import { api } from "$lib/api";
import type {
  SectionSubjectDto,
  AssessmentSchemeDto,
  AssignSectionSubjectRequest,
  UpdateSectionSubjectRequest,
} from "$lib/api";
import { toast } from "@orbis/ui";

import ClassSelect from "$lib/features/academics/components/class-select.svelte";
import SubjectSelect from "$lib/features/academics/components/subject-select.svelte";
import PeriodSelect from "$lib/features/academics/components/period-select.svelte";
import TeacherSelect from "$lib/features/academics/components/teacher-select.svelte";
import { createAssessmentSchemesQuery } from "$lib/features/academics/queries-assessment";
import { classSubjectFormInputSchema } from "@orbis/schemas";
import { FormController } from "$lib/shared/forms/form-controller.svelte";
import { FormField } from "@orbis/ui";

let {
  orgId,
  fixedClassId = "",
  fixedPeriodId = "",
  editData,
  onSuccess,
  onCancel,
}: {
  orgId: string;
  fixedClassId?: string;
  fixedPeriodId?: string;
  editData?: SectionSubjectDto | null;
  onSuccess?: () => void;
  onCancel?: () => void;
} = $props();

const isEditMode = $derived(!!editData);

// Form state
let classId = $state(
  untrack(() => fixedClassId || (editData ? String(editData.section_id ?? "") : "")),
);
let subjectId = $state(untrack(() => editData ? String(editData.subject_id ?? "") : ""));
let teacherId = $state(
  untrack(() => editData ? String(editData.instructor_id ?? "") : ""),
);
let weeklyHours = $state(untrack(() => editData ? Number(editData.weekly_hours ?? 4) : 4));
let periodId = $state(
  untrack(() => fixedPeriodId || (editData ? String(editData.period_id ?? "") : "")),
);
let schemeId = $state(untrack(() => String(editData?.scheme_id ?? "")));

// Assessment Schemes Query
const schemesQuery = createAssessmentSchemesQuery(() => orgId);
const schemes = $derived((schemesQuery.data as AssessmentSchemeDto[]) ?? []);

const form = new FormController({
  initialValues: {
    class_id: "",
    subject_id: "",
    teacher_id: "",
    period_id: "",
    weekly_hours: 4,
  },
  schema: classSubjectFormInputSchema,
});
const isSaving = $derived(form.isSubmitting);

async function handleSubmit() {
  form.values = {
    class_id: classId,
    subject_id: subjectId,
    teacher_id: teacherId,
    period_id: periodId,
    weekly_hours: Number(weeklyHours),
  };
  const submitted = await form.submit(async (values) => {
    if (isEditMode && editData?.id) {
      const request: UpdateSectionSubjectRequest = {
        instructor_id: values.teacher_id,
        scheme_id: schemeId || undefined,
        weekly_hours: values.weekly_hours,
      };
      await api.updateSectionSubject(editData.id, request);
      toast.success(m.common_cs_update_success());
    } else {
      const request: AssignSectionSubjectRequest = {
        section_id: values.class_id,
        subject_id: values.subject_id,
        period_id: values.period_id,
        instructor_id: values.teacher_id,
        scheme_id: schemeId || undefined,
        weekly_hours: values.weekly_hours,
      };
      await api.assignSectionSubject(request);
      toast.success(m.common_cs_create_success());
    }

    onSuccess?.();
  });
  if (!submitted && form.formError) toast.error(form.formError);
  else if (!submitted) toast.error(m.common_form_error_required_fields());
}
</script>

<form class="flex h-full flex-col overflow-hidden gap-0" onsubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
  <SheetHeader class="shrink-0 border-b border-border/40 px-6 py-4 text-start select-none">
    <SheetTitle class="text-xl font-bold text-foreground">
      {isEditMode ? m.common_action_edit() : m.common_cs_sheet_create_title()}
    </SheetTitle>
  </SheetHeader>

  <div class="flex-1 overflow-y-auto p-6 scrollbar-hide">
    <div class="flex flex-col gap-4">
      <!-- Period -->
      <FormField label={m.common_metrics_period()} error={form.errors.period_id?.[0]} required>
        {#snippet children()}
        <PeriodSelect
          {orgId}
          bind:value={periodId}
          disabled={!!fixedPeriodId}
          placeholder={m.academics_studio_select_period()}
          class="h-9 text-sm"
        />
        {/snippet}
      </FormField>

      {#if !fixedClassId}
        <!-- Class -->
        <FormField label={m.common_word_semester()} error={form.errors.class_id?.[0]} required>
          {#snippet children()}
          <ClassSelect
            {orgId}
            bind:value={classId}
            placeholder={m.common_cs_form_select_class()}
            class="h-9 text-sm"
          />
          {/snippet}
        </FormField>
      {/if}

      <!-- Subject -->
      <FormField label={m.common_word_subject()} error={form.errors.subject_id?.[0]} required>
        {#snippet children()}
        <SubjectSelect
          {orgId}
          bind:value={subjectId}
          placeholder={m.common_cs_form_select_subject()}
          class="h-9 text-sm"
        />
        {/snippet}
      </FormField>

      <!-- Primary Teacher -->
      <FormField label={m.common_cs_col_teacher()} error={form.errors.teacher_id?.[0]} required>
        {#snippet children()}
        <TeacherSelect
          {orgId}
          bind:value={teacherId}
          placeholder={m.common_cs_form_select_teacher()}
          class="h-9 text-sm"
        />
        {/snippet}
      </FormField>

      <!-- Weekly Hours -->
      <FormField label={m.common_cs_col_hours()} error={form.errors.weekly_hours?.[0]} required>
        {#snippet children()}
        <Input
          type="number"
          min="1"
          max="30"
          value={weeklyHours}
          oninput={(e) => (weeklyHours = parseInt((e.target as HTMLInputElement).value) || 4)}
          class="h-9 text-sm"
        />
        {/snippet}
      </FormField>

      <!-- Assessment scheme (optional; falls back to period/org default) -->
      <FormField label={m.common_cs_assessment_scheme()}>
        {#snippet children()}
        <Select
          type="single"
          value={schemeId || "__default__"}
          onValueChange={(v) => (schemeId = v === "__default__" ? "" : (v ?? ""))}
        >
          <SelectTrigger class="h-9 text-sm">
            {schemeId
              ? String(
                  schemes.find((s) => String(s.id) === schemeId)?.name ?? schemeId,
                )
              : m.common_cs_assessment_default()}
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="__default__">{m.common_cs_assessment_default()}</SelectItem>
            {#each schemes as scheme (scheme.id)}
              <SelectItem value={String(scheme.id)}>
                {scheme.name || scheme.id}
              </SelectItem>
            {/each}
          </SelectContent>
        </Select>
        {/snippet}
      </FormField>
    </div>
  </div>

  <!-- Actions -->
  <div class="flex shrink-0 justify-end gap-2 border-t border-border/40 bg-background/50 px-6 py-3">
    <Button
      type="button"
      variant="ghost"
      class="h-9 text-sm"
      onclick={onCancel}
    >
      {m.common_action_cancel()}
    </Button>
    <Button
      type="submit"
      variant="lime"
      class="h-9 text-sm font-semibold"
      disabled={isSaving || !classId || !subjectId || !teacherId || !periodId}
    >
      {isSaving ? m.common_state_saving() : m.common_action_save()}
    </Button>
  </div>
</form>
