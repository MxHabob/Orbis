<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { untrack } from "svelte";
import { api, extractErrorMessage, type ScheduleLessonRequest } from "$lib/api";
import { getWeekdayOptions, l } from "$lib/core/i18n";
import { createSectionSubjectsQuery } from "$lib/features/academics/queries";
import { lessonFormInputSchema } from "@orbis/schemas";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@orbis/ui/accordion";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui/select";
import { SheetHeader, SheetTitle } from "@orbis/ui/sheet";
import { Textarea } from "@orbis/ui/textarea";
import {
  BookOpen,
  ClockRotateRight,
  Pin,
  User,
  UserBadgeCheck,
  Users,
} from "@orbis/icons";
import { toast } from "@orbis/ui";
import { FormController } from "$lib/shared/forms/form-controller.svelte";
import { FormField } from "@orbis/ui";

type Props = {
  orgId: string;
  periodId: string;
  classId: string;
  dayOfWeek: number;
  timeSlotId: string;
  timeSlots?: any[];
  lesson?: any | null;
  classes?: any[];
  subjects: any[];
  teachers: any[];
  rooms: any[];
  onSuccess: () => void;
  onCancel: () => void;
};

let {
  orgId: _orgId,
  periodId,
  classId,
  dayOfWeek,
  timeSlotId,
  timeSlots = [],
  lesson = null,
  classes = [],
  subjects = [],
  teachers = [],
  rooms = [],
  onSuccess,
  onCancel,
}: Props = $props();

let selectedClassId = $state(
  untrack(() => String(lesson?.class_id || classId || "")),
);
let selectedSubjectId = $state(untrack(() => String(lesson?.subject_id || "")));
let selectedTeacherId = $state(untrack(() => String(lesson?.teacher_id || "")));
let selectedRoomId = $state(untrack(() => String(lesson?.room_id || "")));
let selectedDay = $state(
  untrack(() => Number(lesson?.day_of_week || dayOfWeek || 0)),
);
let selectedSlotId = $state(
  untrack(() => String(lesson?.time_slot_id || timeSlotId || "")),
);
let topic = $state(untrack(() => lesson?.topic || ""));
let homework = $state(untrack(() => lesson?.homework || ""));
let notes = $state(untrack(() => lesson?.notes || ""));
let conflictMessage = $state<string | null>(null);
let isCheckingConflict = $state(false);

const form = new FormController({
  initialValues: {
    period_id: "",
    class_id: "",
    subject_id: "",
    teacher_id: "",
    time_slot_id: "",
    day_of_week: 1,
  },
  schema: lessonFormInputSchema,
});
const isEditing = $derived(Boolean(lesson?.id));
const isSubmitting = $derived(form.isSubmitting);
const SectionSubjectsQuery = createSectionSubjectsQuery(
  () => selectedClassId,
  () => periodId,
);
const SectionSubjects = $derived(SectionSubjectsQuery.data ?? []);
const availableSubjects = $derived.by(() => {
  const linkedIds = new Set(
    SectionSubjects.map((item) => String(item.subject_id)),
  );
  return subjects.filter((subject: any) => linkedIds.has(String(subject.id)));
});
const selectedSectionSubject = $derived(
  SectionSubjects.find((item) => String(item.subject_id) === selectedSubjectId),
);
const availableTeachers = $derived.by(() => {
  const linkedIds = SectionSubjects.filter(
    (item) =>
      !selectedSubjectId || String(item.subject_id) === selectedSubjectId,
  )
    .map((item) => item.instructor_id)
    .filter((id): id is string => Boolean(id));
  return teachers.filter((teacher: any) =>
    linkedIds.includes(String(teacher.id)),
  );
});
const weekdayOptions = getWeekdayOptions();
const weekdayOrder = new Map(
  weekdayOptions.map((weekday, index) => [weekday.value, index]),
);
const weekdays = $derived(
  [
    ...new Set(
      timeSlots
        .filter((slot) => !slot.is_break)
        .map((slot) => Number(slot.day_of_week)),
    ),
  ]
    .filter((day) => Number.isInteger(day) && day >= 1 && day <= 7)
    .sort(
      (left, right) =>
        (weekdayOrder.get(left) ?? 0) - (weekdayOrder.get(right) ?? 0),
    )
    .map((value) => ({
      value,
      label:
        weekdayOptions.find((weekday) => weekday.value === value)?.label ?? "",
    })),
);
const effectiveSelectedDay = $derived(
  weekdays.some((weekday) => weekday.value === selectedDay)
    ? selectedDay
    : (weekdays[0]?.value ?? 0),
);
const slotsForSelectedDay = $derived(
  timeSlots.filter(
    (slot) => !slot.is_break && slot.day_of_week === effectiveSelectedDay,
  ),
);
const effectiveSelectedSlotId = $derived(
  slotsForSelectedDay.some((slot) => String(slot.id) === selectedSlotId)
    ? selectedSlotId
    : String(slotsForSelectedDay[0]?.id ?? ""),
);

function handleSubjectChange(value: string | undefined) {
  selectedSubjectId = value ?? "";
  const cs = SectionSubjects.find(
    (item) => String(item.subject_id) === selectedSubjectId,
  );
  selectedTeacherId = cs?.instructor_id ?? "";
}

function handleDayChange(value: string | undefined) {
  selectedDay = Number(value) || 0;
  const slots = timeSlots.filter(
    (slot) => !slot.is_break && slot.day_of_week === selectedDay,
  );
  if (!slots.some((slot) => String(slot.id) === selectedSlotId)) {
    selectedSlotId = String(slots[0]?.id ?? "");
  }
}

async function handleSubmit(event: SubmitEvent) {
  event.preventDefault();
  form.values = {
    period_id: periodId,
    class_id: selectedClassId,
    subject_id: selectedSubjectId,
    teacher_id: selectedTeacherId,
    time_slot_id: effectiveSelectedSlotId,
    day_of_week: effectiveSelectedDay,
  };
  if (!form.validate()) {
    toast.error(m.common_lesson_required_fields());
    return;
  }
  if (!selectedSectionSubject) {
    toast.error(m.common_lesson_error_subject_link());
    return;
  }
  if (conflictMessage) {
    toast.error(conflictMessage);
    return;
  }

  const submitted = await form.submit(async (values) => {
    if (isEditing && lesson?.id) {
    }
    const request: ScheduleLessonRequest = {
      section_subject_id: selectedSectionSubject.id,
      time_slot_id: values.time_slot_id,
      room_id: selectedRoomId || undefined,
      teacher_id: values.teacher_id || undefined,
    };
    if (isEditing && lesson?.id) {
      await api.updateScheduledLesson(lesson.id, request);
    } else {
      await api.scheduleLesson(request);
    }
    toast.success(
      isEditing
        ? m.common_lesson_edit_success()
        : m.common_lesson_toast_success(),
    );
    onSuccess();
  });
  if (!submitted && form.formError) {
    toast.error(m.common_lesson_toast_error(), {
      description: extractErrorMessage(form.formError),
    });
  }
}
</script>

<form onsubmit={handleSubmit} class="flex h-full flex-col overflow-hidden gap-0">
  <SheetHeader class="shrink-0 border-b border-border/40 px-6 py-4 text-start select-none">
    <SheetTitle class="text-xl font-bold text-foreground">
      <span>{isEditing ? m.common_lesson_edit_title() : m.common_lesson_sheet_title()}</span>
    </SheetTitle>
  </SheetHeader>

  <div class="flex-1 overflow-y-auto p-6 scrollbar-hide">
    {#if conflictMessage}
      <div class="mb-6 flex items-start gap-2.5 rounded-lg border border-amber-500/20 bg-amber-500/10 p-3 text-start text-xs text-amber-600 dark:text-amber-400">
        <UserBadgeCheck size={16} class="mt-0.5 shrink-0 text-amber-500" />
        <span class="font-medium">{conflictMessage}</span>
      </div>
    {/if}

    <Accordion type="multiple" value={["schedule", "content"]} class="flex flex-col gap-2">
      <AccordionItem value="schedule" class="rounded-lg">
        <AccordionTrigger class="text-sm font-semibold hover:no-underline">{m.common_lesson_placement()}</AccordionTrigger>
        <AccordionContent class="space-y-3">
          <FormField id="lesson-class" label={m.common_lesson_class()} error={form.errors.class_id?.[0]} required>
            {#snippet children()}
            <Select type="single" value={selectedClassId} onValueChange={(value: string | undefined) => (selectedClassId = value ?? "")} disabled={isEditing}>
              <SelectTrigger id="lesson-class" class="h-9 w-full text-sm"><div class="flex items-center gap-1.5 truncate"><Users size={13} class="text-muted-foreground" /><span>{classes.find((item) => String(item.id) === selectedClassId) ? l(classes.find((item) => String(item.id) === selectedClassId).name_ar, classes.find((item) => String(item.id) === selectedClassId).name) : m.common_lesson_select_class()}</span></div></SelectTrigger>
              <SelectContent>{#each classes as item (item.id)}<SelectItem value={String(item.id)}>{l(item.name_ar, item.name)}</SelectItem>{/each}</SelectContent>
            </Select>
            {/snippet}
          </FormField>
          <FormField id="lesson-subject" label={m.common_lesson_subject()} error={form.errors.subject_id?.[0]} required>
            {#snippet children()}
            <Select type="single" value={selectedSubjectId} onValueChange={handleSubjectChange} disabled={isEditing || availableSubjects.length === 0}>
              <SelectTrigger id="lesson-subject" class="h-9 w-full text-sm"><div class="flex items-center gap-1.5 truncate"><BookOpen size={13} class="text-muted-foreground" /><span>{availableSubjects.find((item) => String(item.id) === selectedSubjectId) ? l(availableSubjects.find((item) => String(item.id) === selectedSubjectId).name_ar, availableSubjects.find((item) => String(item.id) === selectedSubjectId).name) : m.common_lesson_select_subject()}</span></div></SelectTrigger>
              <SelectContent>{#each availableSubjects as item (item.id)}<SelectItem value={String(item.id)}>{l(item.name_ar, item.name)}</SelectItem>{/each}</SelectContent>
            </Select>
            {/snippet}
          </FormField>
          <FormField id="lesson-teacher" label={m.common_lesson_label_teacher()} error={form.errors.teacher_id?.[0]}>
            {#snippet children()}
            <Select type="single" value={selectedTeacherId} onValueChange={(value: string | undefined) => (selectedTeacherId = value ?? "")} disabled={availableTeachers.length === 0}>
              <SelectTrigger id="lesson-teacher" class="h-9 w-full text-sm"><div class="flex items-center gap-1.5 truncate"><User size={13} class="text-muted-foreground" /><span>{teachers.find((item) => String(item.id) === selectedTeacherId) ? l(teachers.find((item) => String(item.id) === selectedTeacherId).name_ar, teachers.find((item) => String(item.id) === selectedTeacherId).full_name) : m.common_lesson_option_no_teacher()}</span></div></SelectTrigger>
              <SelectContent><SelectItem value="">{m.common_lesson_option_no_teacher()}</SelectItem>{#each availableTeachers as item (item.id)}<SelectItem value={String(item.id)}>{l(item.name_ar, item.full_name)}</SelectItem>{/each}</SelectContent>
            </Select>
            {/snippet}
          </FormField>
          <FormField id="lesson-room" label={m.common_lesson_label_room()}>
            {#snippet children()}
            <Select type="single" value={selectedRoomId} onValueChange={(value: string | undefined) => (selectedRoomId = value ?? "")}>
              <SelectTrigger id="lesson-room" class="h-9 w-full text-sm"><div class="flex items-center gap-1.5 truncate"><Pin size={13} class="text-muted-foreground" /><span>{rooms.find((item) => String(item.id) === selectedRoomId) ? l(rooms.find((item) => String(item.id) === selectedRoomId).name_ar, rooms.find((item) => String(item.id) === selectedRoomId).name) : m.common_lesson_option_select_room()}</span></div></SelectTrigger>
              <SelectContent><SelectItem value="">{m.common_lesson_option_no_room()}</SelectItem>{#each rooms as item (item.id)}<SelectItem value={String(item.id)}>{l(item.name_ar, item.name)}</SelectItem>{/each}</SelectContent>
            </Select>
            {/snippet}
          </FormField>
          <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
            <div class="flex flex-col gap-2 text-start"><label for="lesson-day" class="text-sm font-medium text-foreground">{m.common_lesson_label_day()} *</label><Select type="single" value={String(effectiveSelectedDay)} onValueChange={handleDayChange} disabled={weekdays.length === 0}><SelectTrigger id="lesson-day" class="h-9 w-full text-sm"><span class="truncate">{weekdays.find((item) => item.value === effectiveSelectedDay)?.label ?? m.common_lesson_label_day()}</span></SelectTrigger><SelectContent>{#each weekdays as item (item.value)}<SelectItem value={String(item.value)}>{item.label}</SelectItem>{/each}</SelectContent></Select></div>
            <div class="flex flex-col gap-2 text-start"><label for="lesson-slot" class="text-sm font-medium text-foreground">{m.common_lesson_label_period()} *</label><Select type="single" value={effectiveSelectedSlotId} onValueChange={(value: string | undefined) => (selectedSlotId = value ?? "")} disabled={slotsForSelectedDay.length === 0}><SelectTrigger id="lesson-slot" class="h-9 w-full text-sm"><div class="flex items-center gap-1.5 truncate"><ClockRotateRight size={13} class="text-muted-foreground" /><span>{slotsForSelectedDay.find((item) => String(item.id) === effectiveSelectedSlotId)?.name_ar || m.common_lesson_select_slot()}</span></div></SelectTrigger><SelectContent>{#each slotsForSelectedDay as item (item.id)}<SelectItem value={String(item.id)}>{l(item.name_ar, item.name)} ({item.start_time?.slice(0, 5)} - {item.end_time?.slice(0, 5)})</SelectItem>{/each}</SelectContent></Select></div>
          </div>
        </AccordionContent>
      </AccordionItem>
      <AccordionItem value="content" class="rounded-lg">
        <AccordionTrigger class="text-sm font-semibold hover:no-underline">{m.common_lesson_content()}</AccordionTrigger>
        <AccordionContent class="space-y-3">
          <div class="flex flex-col gap-2 text-start"><label for="lesson-topic" class="text-sm font-medium text-foreground">{m.common_lesson_topic()}</label><Input id="lesson-topic" bind:value={topic} placeholder={m.common_lesson_topic_placeholder()} /></div>
          <div class="flex flex-col gap-2 text-start"><label for="lesson-homework" class="text-sm font-medium text-foreground">{m.common_lesson_homework()}</label><Input id="lesson-homework" bind:value={homework} placeholder={m.common_lesson_homework_placeholder()} /></div>
          <div class="flex flex-col gap-2 text-start"><label for="lesson-notes" class="text-sm font-medium text-foreground">{m.common_lesson_notes()}</label><Textarea id="lesson-notes" bind:value={notes} rows={3} placeholder={m.common_lesson_notes_placeholder()} /></div>
        </AccordionContent>
      </AccordionItem>
    </Accordion>
  </div>

  <div class="flex shrink-0 justify-end gap-2 border-t border-border/40 bg-background/50 px-6 py-3">
    <Button type="button" variant="ghost" onclick={onCancel} disabled={isSubmitting}>{m.common_action_cancel()}</Button>
    <Button type="submit" variant="lime" class="font-semibold" disabled={isSubmitting || isCheckingConflict || Boolean(conflictMessage) || !effectiveSelectedDay || !effectiveSelectedSlotId}>{isSubmitting ? m.common_state_saving() : m.common_lesson_save()}</Button>
  </div>
</form>