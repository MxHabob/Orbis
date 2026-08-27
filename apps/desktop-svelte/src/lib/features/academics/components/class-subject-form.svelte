<!--
  @component ClassSubjectForm
  Form for assigning a subject to a teacher for a specific class and academic period.
  Used in both create and edit modes via a Sheet panel.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import { commands, extractErrorMessage } from "$lib/core/ipc";
import type { ClassSubject, JsonRecord } from "@orbis/sdk-core";
import { toast } from "svelte-sonner";

import {
  createPeriodsQuery,
  createClassesQuery,
  createSubjectsQuery,
} from "$lib/features/academics/queries";
import ClassSelect from "$lib/features/academics/components/class-select.svelte";
import SubjectSelect from "$lib/features/academics/components/subject-select.svelte";
import PeriodSelect from "$lib/features/academics/components/period-select.svelte";
import TeacherSelect from "$lib/features/academics/components/teacher-select.svelte";
import { createUsersQuery } from "$lib/features/settings/queries";
import { createAssessmentSchemesQuery } from "$lib/features/academics/queries-assessment";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { classSubjectFormInputSchema } from "@orbis/schemas";
import { FormController } from "$lib/shared/forms/form-controller.svelte";

let {
  orgId,
  fixedClassId = "",
  editData,
  onSuccess,
  onCancel,
}: {
  orgId: string;
  fixedClassId?: string;
  editData?: ClassSubject | null;
  onSuccess?: () => void;
  onCancel?: () => void;
} = $props();

const isEditMode = $derived(!!editData);

// Form state
let classId = $state("");
let subjectId = $state("");
let teacherId = $state("");
let weeklyHours = $state(4);
let periodId = $state("");
let schemeId = $state("");

$effect(() => {
  classId = fixedClassId || (editData ? String(editData.class_id ?? "") : "");
  subjectId = editData ? String(editData.subject_id ?? "") : "";
  teacherId = editData ? String(editData.primary_teacher_id ?? "") : "";
  weeklyHours = editData ? Number(editData.weekly_hours ?? 4) : 4;
  periodId = editData ? String(editData.period_id ?? "") : "";
  schemeId = editData ? String((editData as any).scheme_id ?? "") : "";
});

// TanStack Queries
const periodsQuery = createPeriodsQuery(() => orgId);
const classesQuery = createClassesQuery(() => orgId);
const subjectsQuery = createSubjectsQuery(() => orgId);
const usersQuery = createUsersQuery(() => orgId);
const schemesQuery = createAssessmentSchemesQuery(() => orgId);

const periods = $derived((periodsQuery.data as JsonRecord[]) ?? []);
const classes = $derived((classesQuery.data as JsonRecord[]) ?? []);
const subjects = $derived((subjectsQuery.data as JsonRecord[]) ?? []);
const teachers = $derived(
  ((usersQuery.data as JsonRecord[]) ?? []).filter(
    (u) => String(u.primary_role ?? u.role) === "teacher",
  ),
);
const schemes = $derived((schemesQuery.data as JsonRecord[]) ?? []);

const isLoading = $derived(
  periodsQuery.isLoading ||
    classesQuery.isLoading ||
    subjectsQuery.isLoading ||
    usersQuery.isLoading,
);
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

// Derived label helpers
const selectedClass = $derived(classes.find((c) => String(c.id) === classId));
const selectedSubject = $derived(
  subjects.find((s) => String(s.id) === subjectId),
);
const selectedTeacher = $derived(
  teachers.find((t) => String(t.id) === teacherId),
);
const selectedPeriod = $derived(periods.find((c) => String(c.id) === periodId));
const availablePeriods = $derived(
  fixedClassId && selectedClass?.period_id
    ? periods.filter((period) => String(period.id) === String(selectedClass.period_id))
    : periods,
);

async function handleSubmit() {
  form.values = {
    class_id: classId,
    subject_id: subjectId,
    teacher_id: teacherId,
    period_id: periodId,
    weekly_hours: Number(weeklyHours),
  };
  const submitted = await form.submit(async (values) => {
    const request = {
      class_id: values.class_id,
      subject_id: values.subject_id,
      period_id: values.period_id,
      primary_teacher_id: values.teacher_id,
      weekly_hours: values.weekly_hours,
      scheme_id: schemeId || null,
    };

    if (isEditMode && editData?.id) {
      await commands.academic.updateClassSubject(
        String(editData.id),
        request as any,
      );
      toast.success(m.common_cs_update_success());
    } else {
      await commands.academic.createClassSubject(request as any);
      toast.success(m.common_cs_create_success());
    }

    onSuccess?.();
  });
  if (!submitted && form.formError) toast.error(form.formError);
  else if (!submitted) toast.error(m.common_form_error_required_fields());
}
</script>

<form class="flex flex-col gap-5" onsubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
  {#if isLoading}
    <div class="flex flex-col gap-5 animate-pulse">
      <div class="flex flex-col gap-1.5">
        <Skeleton class="h-3.5 w-24" />
        <Skeleton class="h-9 w-full rounded-md" />
      </div>
      <div class="flex flex-col gap-1.5">
        <Skeleton class="h-3.5 w-20" />
        <Skeleton class="h-9 w-full rounded-md" />
      </div>
      <div class="flex flex-col gap-1.5">
        <Skeleton class="h-3.5 w-28" />
        <Skeleton class="h-9 w-full rounded-md" />
      </div>
      <div class="flex flex-col gap-1.5">
        <Skeleton class="h-3.5 w-24" />
        <Skeleton class="h-9 w-full rounded-md" />
      </div>
    </div>
  {:else}
    <!-- Period -->
    <div class="flex flex-col gap-1.5">
      <Label class="text-xs font-semibold text-muted-foreground">{m.common_metrics_period()} *</Label>
      <PeriodSelect
        bind:value={periodId}
        periods={availablePeriods}
        placeholder={m.academics_studio_select_period()}
        class="h-9 text-sm"
      />
    </div>

    {#if !fixedClassId}
      <!-- Class -->
      <div class="flex flex-col gap-1.5">
        <Label class="text-xs font-semibold text-muted-foreground">{m.common_word_semester()} *</Label>
        <ClassSelect
          bind:value={classId}
          {classes}
          placeholder={m.common_cs_form_select_class()}
          class="h-9 text-sm"
        />
      </div>
    {/if}

    <!-- Subject -->
    <div class="flex flex-col gap-1.5">
      <Label class="text-xs font-semibold text-muted-foreground">{m.common_word_subject()} *</Label>
      <SubjectSelect
        bind:value={subjectId}
        {subjects}
        placeholder={m.common_cs_form_select_subject()}
        class="h-9 text-sm"
      />
    </div>

    <!-- Primary Teacher -->
    <div class="flex flex-col gap-1.5">
      <Label class="text-xs font-semibold text-muted-foreground">{m.common_cs_col_teacher()} *</Label>
      <TeacherSelect
        bind:value={teacherId}
        {teachers}
        placeholder={m.common_cs_form_select_teacher()}
        class="h-9 text-sm"
      />
    </div>

    <!-- Weekly Hours -->
    <div class="flex flex-col gap-1.5">
      <Label class="text-xs font-semibold text-muted-foreground">{m.common_cs_col_hours()}</Label>
      <Input
        type="number"
        min="1"
        max="30"
        value={weeklyHours}
        oninput={(e) => (weeklyHours = parseInt((e.target as HTMLInputElement).value) || 4)}
        class="h-9 text-sm"
      />
    </div>

    <!-- Assessment scheme (optional; falls back to period/org default) -->
    <div class="flex flex-col gap-1.5">
      <Label class="text-xs font-semibold text-muted-foreground">{m.common_cs_assessment_scheme()}</Label>
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
    </div>

    <!-- Actions -->
    <div class="flex gap-2 pt-2 border-t border-border/40">
      <Button
        type="button"
        variant="outline"
        class="flex-1 h-9 text-sm"
        onclick={onCancel}
      >
        {m.common_action_cancel()}
      </Button>
      <Button
        type="submit"
        class="flex-1 h-9 text-sm"
        disabled={isSaving || isLoading || !classId || !subjectId || !teacherId || !periodId}
      >
        {isSaving ? m.common_state_saving() : m.common_action_save()}
      </Button>
    </div>
  {/if}
</form>
