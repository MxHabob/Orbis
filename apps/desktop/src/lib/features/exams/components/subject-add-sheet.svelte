<script lang="ts">
import * as m from "$lib/paraglide/messages";
import {
  createMutation,
  createQuery,
  useQueryClient,
} from "@tanstack/svelte-query";
import { FormField } from "@orbis/ui";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Sheet, SheetContent, SheetHeader, SheetTitle } from "@orbis/ui/sheet";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui/select";
import { toast } from "@orbis/ui";
import { z } from "zod";

import { api, extractErrorMessage } from "$lib/api";
import { FormController } from "$lib/shared/forms/form-controller.svelte";

let {
  open = $bindable(false),
  orgId,
  sessionId,
  periodId,
  examStartDate,
  examEndDate,
  onSuccess,
}: {
  open: boolean;
  orgId: string;
  sessionId: string;
  periodId: string;
  examStartDate: string;
  examEndDate: string;
  onSuccess: () => void;
} = $props();

type ScheduleFormValues = {
  section_id: string;
  section_subject_id: string;
  teacher_id: string;
  exam_date: string;
  start_time: string;
  end_time: string;
  subject_name: string;
  subject_name_ar: string;
  subject_code: string;
};

const scheduleSchema = z
  .object({
    section_id: z
      .string()
      .trim()
      .min(1, m.operations_ui_subject_selection_required()),
    section_subject_id: z
      .string()
      .trim()
      .min(1, m.operations_ui_subject_selection_required()),
    teacher_id: z.string(),
    exam_date: z.string().date(m.common_period_form_error_dates_required()),
    start_time: z
      .string()
      .regex(/^\d{2}:\d{2}$/, m.academics_schedule_start_time()),
    end_time: z
      .string()
      .regex(/^\d{2}:\d{2}$/, m.academics_schedule_end_time()),
    subject_name: z.string(),
    subject_name_ar: z.string(),
    subject_code: z.string(),
  })
  .superRefine((values, context) => {
    if (values.end_time <= values.start_time) {
      context.addIssue({
        code: "custom",
        path: ["end_time"],
        message: m.academics_exams_session_invalid_time_range(),
      });
    }
    if (mode === "create" && !values.subject_name_ar.trim()) {
      context.addIssue({
        code: "custom",
        path: ["subject_name_ar"],
        message: m.operations_ui_subject_arabic_name_required(),
      });
    }
    if (mode === "create" && !values.subject_code.trim()) {
      context.addIssue({
        code: "custom",
        path: ["subject_code"],
        message: m.operations_ui_subject_code_required(),
      });
    }
    if (
      (examStartDate && values.exam_date < examStartDate) ||
      (examEndDate && values.exam_date > examEndDate)
    ) {
      context.addIssue({
        code: "custom",
        path: ["exam_date"],
        message: m.academics_exams_session_date_outside_range(),
      });
    }
  });

let mode = $state<"select" | "create">("select");
let sectionId = $state("");
let sectionSubjectId = $state("");
let teacherId = $state("");
let examDate = $state("");
let startTime = $state("08:00");
let endTime = $state("10:00");
let newSubjectName = $state("");
let newSubjectNameAr = $state("");
let newSubjectCode = $state("");

const queryClient = useQueryClient();
const sectionsQuery = createQuery(() => ({
  queryKey: ["exam-session-sections", orgId, periodId],
  queryFn: () => api.listSections(),
  enabled: open && !!orgId && !!periodId,
  staleTime: 5 * 60 * 1000,
}));
const subjectsQuery = createQuery(() => ({
  queryKey: ["subjects", orgId],
  queryFn: () => api.listSubjects(),
  enabled: open && !!orgId,
  staleTime: 10 * 60 * 1000,
}));
const teachersQuery = createQuery(() => ({
  queryKey: ["exam-session-teachers", orgId],
  queryFn: async () => {
    const employees = [];
    for (let page = 1; ; page += 1) {
      const result = await api.listEmployees({
        employee_type: "teacher",
        status: "active",
        page,
        page_size: 500,
      });
      employees.push(...result.items);
      if (page >= result.total_pages) break;
    }
    return employees;
  },
  enabled: open && !!orgId,
  staleTime: 5 * 60 * 1000,
}));
const offeringsQuery = createQuery(() => ({
  queryKey: ["exam-session-subject-offerings", orgId, periodId, sectionId],
  queryFn: () => api.listSectionSubjects(sectionId, periodId),
  enabled: open && mode === "select" && !!sectionId && !!periodId,
  staleTime: 5 * 60 * 1000,
}));

const sections = $derived(
  (sectionsQuery.data ?? []).filter((section) => section.is_active),
);
const offerings = $derived(offeringsQuery.data ?? []);
const subjects = $derived(subjectsQuery.data ?? []);
const teachers = $derived(teachersQuery.data ?? []);
const subjectOptions = $derived(
  subjects
    .filter((subject) => subject.is_active)
    .map((subject) => ({
      subject,
      offering: offerings.find(
        (offering) => String(offering.subject_id) === String(subject.id),
      ),
    })),
);
const selectedOffering = $derived(
  offerings.find((offering) => String(offering.id) === sectionSubjectId) ??
    null,
);
const isLoadingOptions = $derived(
  sectionsQuery.isLoading ||
    subjectsQuery.isLoading ||
    teachersQuery.isLoading ||
    (mode === "select" && !!sectionId && offeringsQuery.isLoading),
);
const optionsError = $derived(
  sectionsQuery.error ||
    subjectsQuery.error ||
    teachersQuery.error ||
    offeringsQuery.error,
);

function getTeacherName(teacherId: string | null | undefined) {
  const teacher = teachers.find(
    (item) => String(item.id) === String(teacherId),
  );
  return (
    teacher?.full_name_ar ||
    teacher?.full_name ||
    teacher?.display_name ||
    teacher?.employee_number ||
    ""
  );
}

function getSubjectOptionValue(subjectId: string) {
  const offering = offerings.find(
    (item) => String(item.subject_id) === String(subjectId),
  );
  return offering?.id ?? `subject:${subjectId}`;
}

const form = new FormController({
  initialValues: {
    section_id: "",
    section_subject_id: "",
    teacher_id: "",
    exam_date: "",
    start_time: "08:00",
    end_time: "10:00",
    subject_name: "",
    subject_name_ar: "",
    subject_code: "",
  },
  schema: scheduleSchema,
});

const scheduleMutation = createMutation(() => ({
  mutationFn: async (payload: ScheduleFormValues) => {
    let targetSectionSubjectId = payload.section_subject_id;

    if (mode === "create") {
      const newSubject = await api.createSubject({
        name: payload.subject_name.trim() || payload.subject_name_ar.trim(),
        name_ar: payload.subject_name_ar.trim(),
        code: payload.subject_code.trim().toUpperCase(),
      });
      await queryClient.invalidateQueries({ queryKey: ["subjects", orgId] });

      const offering = await api.assignSectionSubject({
        section_id: payload.section_id,
        subject_id: newSubject.id,
        period_id: periodId,
        instructor_id: payload.teacher_id || null,
        status: "active",
      });
      targetSectionSubjectId = offering.id;
    } else if (payload.section_subject_id.startsWith("subject:")) {
      const subjectId = payload.section_subject_id.slice("subject:".length);
      const currentOfferings = await api.listSectionSubjects(
        payload.section_id,
        periodId,
      );
      const existingOffering = currentOfferings.find(
        (offering) => String(offering.subject_id) === subjectId,
      );
      if (existingOffering) {
        targetSectionSubjectId = existingOffering.id;
      } else {
        const offering = await api.assignSectionSubject({
          section_id: payload.section_id,
          subject_id: subjectId,
          period_id: periodId,
          instructor_id: payload.teacher_id || null,
          status: "active",
        });
        targetSectionSubjectId = offering.id;
      }
    }

    return api.scheduleExam({
      exam_session_id: sessionId,
      section_subject_id: targetSectionSubjectId,
      exam_date: payload.exam_date,
      start_time: payload.start_time,
      end_time: payload.end_time,
    });
  },
}));

const isSaving = $derived(form.isSubmitting || scheduleMutation.isPending);

function resetForm() {
  sectionId = "";
  sectionSubjectId = "";
  teacherId = "";
  examDate = examStartDate;
  startTime = "08:00";
  endTime = "10:00";
  newSubjectName = "";
  newSubjectNameAr = "";
  newSubjectCode = "";
  mode = "select";
  form.values = {
    section_id: "",
    section_subject_id: "",
    teacher_id: "",
    exam_date: "",
    start_time: "08:00",
    end_time: "10:00",
    subject_name: "",
    subject_name_ar: "",
    subject_code: "",
  };
}

$effect(() => {
  if (open && !examDate) examDate = examStartDate;
});

async function handleSubmit() {
  if (!sectionId) {
    toast.error(m.operations_ui_subject_selection_required());
    return;
  }
  if (mode === "select" && !sectionSubjectId) {
    toast.error(m.operations_ui_subject_selection_required());
    return;
  }
  form.values = {
    section_id: sectionId,
    section_subject_id: mode === "create" ? "new-subject" : sectionSubjectId,
    teacher_id: teacherId,
    exam_date: examDate,
    start_time: startTime,
    end_time: endTime,
    subject_name: newSubjectName,
    subject_name_ar: newSubjectNameAr,
    subject_code: newSubjectCode,
  };

  const submitted = await form.submit(async (payload) => {
    await scheduleMutation.mutateAsync(payload);
  });

  if (submitted) {
    toast.success(m.operations_ui_subject_add_success());
    resetForm();
    open = false;
    onSuccess();
  } else {
    toast.error(
      form.formError ||
        Object.values(form.errors).flat()[0] ||
        m.operations_ui_subject_invalid(),
    );
  }
}
</script>

<Sheet
  bind:open
  onOpenChange={(value) => {
    if (!value) resetForm();
  }}
>
  <SheetContent
    contentClass="data-[side=end]:sm:!max-w-md"
    class="flex h-full flex-col overflow-hidden p-0"
  >
    <form
      class="flex h-full flex-col overflow-hidden"
      onsubmit={(event) => {
        event.preventDefault();
        handleSubmit();
      }}
    >
      <SheetHeader class="shrink-0 border-b border-border/40 px-6 py-4 text-start">
        <SheetTitle class="text-base font-semibold text-foreground">
          {m.common_exams_subjects_add()}
        </SheetTitle>
      </SheetHeader>

      <div class="flex-1 space-y-5 overflow-y-auto p-6">
        <div class="flex rounded-lg border border-border/60 bg-muted/30 p-1 text-xs">
          <button
            type="button"
            aria-pressed={mode === "select"}
            class="flex-1 rounded-md px-3 py-2 transition-colors {mode === 'select' ? 'bg-card font-medium text-foreground shadow-sm' : 'text-muted-foreground hover:text-foreground'}"
            onclick={() => {
              mode = "select";
              sectionSubjectId = "";
              teacherId = "";
            }}
            disabled={isSaving}
          >
            {m.operations_ui_existing_school_subject()}
          </button>
          <button
            type="button"
            aria-pressed={mode === "create"}
            class="flex-1 rounded-md px-3 py-2 transition-colors {mode === 'create' ? 'bg-card font-medium text-foreground shadow-sm' : 'text-muted-foreground hover:text-foreground'}"
            onclick={() => {
              mode = "create";
              sectionSubjectId = "";
              teacherId = "";
            }}
            disabled={isSaving}
          >
            {m.operations_ui_create_new_subject()}
          </button>
        </div>

        <FormField id="exam-section" label={m.common_table_class()} error={form.errors.section_id?.[0]} required>
          {#snippet children()}
            <Select
              type="single"
              value={sectionId}
              onValueChange={(value) => {
                sectionId = value ?? "";
                sectionSubjectId = "";
                teacherId = "";
              }}
              disabled={isSaving || isLoadingOptions}
            >
              <SelectTrigger id="exam-section" class="h-9 w-full text-sm">
                {sections.find((section) => String(section.id) === sectionId)?.name_ar ||
                  sections.find((section) => String(section.id) === sectionId)?.name ||
                  m.common_table_class()}
              </SelectTrigger>
              <SelectContent>
                {#each sections as section (section.id)}
                  <SelectItem value={String(section.id)}>
                    {section.name_ar || section.name}
                  </SelectItem>
                {/each}
              </SelectContent>
            </Select>
          {/snippet}
        </FormField>

        {#if mode === "select"}
          <FormField id="exam-subject" label={m.operations_ui_select_subject()} error={form.errors.section_subject_id?.[0]} required>
            {#snippet children()}
              <Select
                type="single"
                value={sectionSubjectId}
                onValueChange={(value) => {
                  sectionSubjectId = value ?? "";
                  const offering = offerings.find(
                    (item) => String(item.id) === sectionSubjectId,
                  );
                  teacherId = offering?.instructor_id ?? "";
                }}
                disabled={!sectionId || isSaving || offeringsQuery.isLoading}
              >
                <SelectTrigger id="exam-subject" class="h-9 w-full text-sm">
                  {#if sectionSubjectId}
                    {@const option = subjectOptions.find(
                      (item) =>
                        (item.offering?.id ??
                          `subject:${item.subject.id}`) === sectionSubjectId,
                    )}
                    {option?.subject.name_ar ||
                      option?.subject.name ||
                      m.operations_ui_select_course_subject()}
                  {:else}
                    {m.operations_ui_select_course_subject()}
                  {/if}
                </SelectTrigger>
                <SelectContent>
                  {#each subjectOptions as option (option.subject.id)}
                    <SelectItem
                      value={getSubjectOptionValue(String(option.subject.id))}
                    >
                      {option.subject.name_ar || option.subject.name}
                      {#if option.offering?.instructor_id}
                        <span class="ms-1 text-muted-foreground">
                          · {getTeacherName(option.offering.instructor_id)}
                        </span>
                      {/if}
                    </SelectItem>
                  {/each}
                </SelectContent>
              </Select>
            {/snippet}
          </FormField>
          {#if selectedOffering?.instructor_id}
            <p class="text-xs text-muted-foreground">
              {m.operations_ui_exam_subject_assigned_teacher()}:
              {getTeacherName(selectedOffering.instructor_id)}
            </p>
          {/if}
        {:else}
          <div class="flex flex-col gap-4">
            <FormField id="new-sub-name-ar" label={m.operations_ui_subject_name_arabic()} error={form.errors.subject_name_ar?.[0]} required>
              {#snippet children()}
                <Input
                  id="new-sub-name-ar"
                  bind:value={newSubjectNameAr}
                  placeholder={m.operations_ui_subject_name_example()}
                  class="h-9 text-sm"
                  disabled={isSaving}
                />
              {/snippet}
            </FormField>
            <FormField id="new-sub-name-en" label={m.operations_ui_subject_name_english()}>
              {#snippet children()}
                <Input
                  id="new-sub-name-en"
                  bind:value={newSubjectName}
                  placeholder="e.g. Mathematics"
                  class="h-9 text-sm"
                  disabled={isSaving}
                />
              {/snippet}
            </FormField>
            <FormField id="new-sub-code" label={m.operations_ui_subject_code()} error={form.errors.subject_code?.[0]} required>
              {#snippet children()}
                <Input
                  id="new-sub-code"
                  bind:value={newSubjectCode}
                  placeholder={m.operations_ui_subject_code_example()}
                  class="h-9 font-mono text-sm uppercase"
                  disabled={isSaving}
                />
              {/snippet}
            </FormField>
          </div>
        {/if}

        {#if mode === "create" ||
          (mode === "select" && sectionSubjectId.startsWith("subject:"))}
          <FormField
            id="exam-teacher"
            label={m.operations_ui_exam_subject_teacher()}
          >
            {#snippet children()}
              <Select
                type="single"
                value={teacherId}
                onValueChange={(value) => (teacherId = value ?? "")}
                disabled={isSaving || isLoadingOptions}
              >
                <SelectTrigger id="exam-teacher" class="h-9 w-full text-sm">
                  {getTeacherName(teacherId) ||
                    m.operations_ui_exam_subject_teacher_optional()}
                </SelectTrigger>
                <SelectContent>
                  {#each teachers as teacher (teacher.id)}
                    <SelectItem value={teacher.id}>
                      {teacher.full_name_ar ||
                        teacher.full_name ||
                        teacher.display_name ||
                        teacher.employee_number}
                    </SelectItem>
                  {/each}
                </SelectContent>
              </Select>
            {/snippet}
          </FormField>
        {/if}

        <FormField id="exam-date" label={m.common_overview_date()} error={form.errors.exam_date?.[0]} required>
          {#snippet children()}
            <Input
              id="exam-date"
              type="date"
              bind:value={examDate}
              min={examStartDate}
              max={examEndDate}
              class="h-9 text-sm"
              disabled={isSaving}
            />
          {/snippet}
        </FormField>

        <div class="grid grid-cols-2 gap-3">
          <FormField id="exam-start-time" label={m.academics_schedule_start_time()} error={form.errors.start_time?.[0]} required>
            {#snippet children()}
              <Input id="exam-start-time" type="time" bind:value={startTime} class="h-9 text-sm" disabled={isSaving} />
            {/snippet}
          </FormField>
          <FormField id="exam-end-time" label={m.academics_schedule_end_time()} error={form.errors.end_time?.[0]} required>
            {#snippet children()}
              <Input id="exam-end-time" type="time" bind:value={endTime} class="h-9 text-sm" disabled={isSaving} />
            {/snippet}
          </FormField>
        </div>

        {#if optionsError}
          <div class="flex items-center justify-between gap-3 rounded-lg border border-destructive/20 bg-destructive/5 p-3 text-xs">
            <span class="text-destructive">{extractErrorMessage(optionsError)}</span>
            <Button
              type="button"
              variant="outline"
              size="sm"
              onclick={() => {
                sectionsQuery.refetch();
                subjectsQuery.refetch();
                if (sectionId) offeringsQuery.refetch();
              }}
            >
              {m.common_retry()}
            </Button>
          </div>
        {:else if mode === "select" && sectionId && !offeringsQuery.isLoading && subjectOptions.length === 0}
          <p class="rounded-lg border border-border/50 bg-muted/20 p-3 text-xs text-muted-foreground">
            {m.academics_exams_no_subjects()}
          </p>
        {/if}
      </div>

      <div class="flex shrink-0 items-center justify-end gap-2 border-t border-border/40 bg-background px-6 py-4">
        <Button type="button" variant="outline" size="sm" onclick={() => open = false} disabled={isSaving}>
          {m.common_action_cancel()}
        </Button>
        <Button type="submit" variant="default" size="sm" class="font-semibold" disabled={isSaving || isLoadingOptions || !!optionsError}>
          {isSaving ? m.operations_ui_saving() : m.common_exams_subjects_add()}
        </Button>
      </div>
    </form>
  </SheetContent>
</Sheet>
