<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { api, extractErrorMessage } from "$lib/api";
import { l } from "$lib/core/i18n";
import { getLocale } from "$lib/paraglide/runtime";
import { createClassSubjectsQuery } from "$lib/features/academics/queries";
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
  Clock,
  MapPin,
  ShieldAlert,
  User,
  Users,
} from "@lucide/svelte";
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

let selectedClassId = $state("");
let selectedSubjectId = $state("");
let selectedTeacherId = $state("");
let selectedRoomId = $state("");
let selectedDay = $state(1);
let selectedSlotId = $state("");
let topic = $state("");
let homework = $state("");
let notes = $state("");
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
const classSubjectsQuery = createClassSubjectsQuery(
  () => selectedClassId,
  () => periodId,
);
const classSubjects = $derived(classSubjectsQuery.data ?? []);
const availableSubjects = $derived.by(() => {
  const linkedIds = new Set(
    classSubjects.map((item: any) => String(item.subject_id)),
  );
  return subjects.filter((subject: any) => linkedIds.has(String(subject.id)));
});
const selectedClassSubject = $derived(
  classSubjects.find(
    (item: any) => String(item.subject_id) === selectedSubjectId,
  ),
);
const availableTeachers = $derived.by(() => {
  const linkedIds = classSubjects
    .filter(
      (item: any) =>
        !selectedSubjectId || String(item.subject_id) === selectedSubjectId,
    )
    .map((item: any) => String(item.primary_teacher_id || item.teacher_id))
    .filter(Boolean);
  return teachers.filter((teacher: any) =>
    linkedIds.includes(String(teacher.id)),
  );
});
const weekdays = $derived(
  Array.from({ length: 7 }, (_, index) => {
    const value = index + 1;
    return {
      value,
      label: new Intl.DateTimeFormat(getLocale(), { weekday: "long" }).format(
        new Date(Date.UTC(2023, 0, value)),
      ),
    };
  }),
);

$effect(() => {
  selectedClassId = String(lesson?.class_id || classId || "");
  selectedSubjectId = String(lesson?.subject_id || "");
  selectedTeacherId = String(lesson?.teacher_id || "");
  selectedRoomId = String(lesson?.room_id || "");
  selectedDay = Number(lesson?.day_of_week || dayOfWeek || 1);
  selectedSlotId = String(lesson?.time_slot_id || timeSlotId || "");
  topic = lesson?.topic || "";
  homework = lesson?.homework || "";
  notes = lesson?.notes || "";
});

$effect(() => {
  if (selectedSubjectId && selectedClassSubject?.primary_teacher_id) {
    selectedTeacherId = String(selectedClassSubject.primary_teacher_id);
  }
});

$effect(() => {
  if (selectedClassId && selectedSubjectId && selectedSlotId) checkConflict();
});

async function checkConflict() {
  isCheckingConflict = false;
  conflictMessage = null;
}

async function handleSubmit(event: SubmitEvent) {
  event.preventDefault();
  form.values = {
    period_id: periodId,
    class_id: selectedClassId,
    subject_id: selectedSubjectId,
    teacher_id: selectedTeacherId,
    time_slot_id: selectedSlotId,
    day_of_week: selectedDay,
  };
  if (!form.validate()) {
    toast.error(m.common_lesson_required_fields());
    return;
  }
  if (!selectedClassSubject) {
    toast.error(m.common_lesson_error_subject_link());
    return;
  }
  if (conflictMessage) {
    toast.error(conflictMessage);
    return;
  }

  const submitted = await form.submit(async (values) => {
    const content = {
      topic: topic.trim() || undefined,
      homework: homework.trim() || undefined,
      notes: notes.trim() || undefined,
    };
    await api.scheduleLesson({
      ...values,
      class_subject_id: selectedClassSubject.id,
      room_id: selectedRoomId || undefined,
      ...content,
    } as any);
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
        <ShieldAlert size={16} class="mt-0.5 shrink-0 text-amber-500" />
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
            <Select type="single" value={selectedSubjectId} onValueChange={(value: string | undefined) => (selectedSubjectId = value ?? "")} disabled={isEditing || availableSubjects.length === 0}>
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
              <SelectTrigger id="lesson-room" class="h-9 w-full text-sm"><div class="flex items-center gap-1.5 truncate"><MapPin size={13} class="text-muted-foreground" /><span>{rooms.find((item) => String(item.id) === selectedRoomId) ? l(rooms.find((item) => String(item.id) === selectedRoomId).name_ar, rooms.find((item) => String(item.id) === selectedRoomId).name) : m.common_lesson_option_select_room()}</span></div></SelectTrigger>
              <SelectContent><SelectItem value="">{m.common_lesson_option_no_room()}</SelectItem>{#each rooms as item (item.id)}<SelectItem value={String(item.id)}>{l(item.name_ar, item.name)}</SelectItem>{/each}</SelectContent>
            </Select>
            {/snippet}
          </FormField>
          <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
            <div class="flex flex-col gap-2 text-start"><label for="lesson-day" class="text-sm font-medium text-foreground">{m.common_lesson_label_day()} *</label><Select type="single" value={String(selectedDay)} onValueChange={(value: string | undefined) => (selectedDay = Number(value) || 1)}><SelectTrigger id="lesson-day" class="h-9 w-full text-sm"><span class="truncate">{weekdays.find((item) => item.value === selectedDay)?.label}</span></SelectTrigger><SelectContent>{#each weekdays as item}<SelectItem value={String(item.value)}>{item.label}</SelectItem>{/each}</SelectContent></Select></div>
            <div class="flex flex-col gap-2 text-start"><label for="lesson-slot" class="text-sm font-medium text-foreground">{m.common_lesson_label_period()} *</label><Select type="single" value={selectedSlotId} onValueChange={(value: string | undefined) => (selectedSlotId = value ?? "")}><SelectTrigger id="lesson-slot" class="h-9 w-full text-sm"><div class="flex items-center gap-1.5 truncate"><Clock size={13} class="text-muted-foreground" /><span>{timeSlots.find((item) => String(item.id) === selectedSlotId)?.name_ar || m.common_lesson_select_slot()}</span></div></SelectTrigger><SelectContent>{#each timeSlots.filter((item) => !item.is_break) as item (item.id)}<SelectItem value={String(item.id)}>{l(item.name_ar, item.name)} ({item.start_time?.slice(0, 5)} - {item.end_time?.slice(0, 5)})</SelectItem>{/each}</SelectContent></Select></div>
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
    <Button type="submit" variant="lime" class="font-semibold" disabled={isSubmitting || isCheckingConflict || Boolean(conflictMessage)}>{isSubmitting ? m.common_state_saving() : m.common_lesson_save()}</Button>
  </div>
</form>