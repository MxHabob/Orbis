<script lang="ts">
  import {
    Sheet,
    SheetContent,
    SheetHeader,
    SheetTitle,
    SheetFooter,
  } from "@orbis/ui-svelte/sheet";
  import { Button } from "@orbis/ui-svelte/button";
  import { Input } from "@orbis/ui-svelte/input";
  import { Label } from "@orbis/ui-svelte/label";
  import { Textarea } from "@orbis/ui-svelte/textarea";
  import {
    Select,
    SelectTrigger,
    SelectContent,
    SelectItem,
  } from "@orbis/ui-svelte/select";
  import { AlertTriangle, Clock, MapPin, Trash2, User, Users, BookOpen } from "@lucide/svelte";
  import { commands, extractErrorMessage } from "$lib/core/ipc";
  import { toast } from "svelte-sonner";
  import * as m from "$lib/paraglide/messages";
  import { l } from "$lib/core/i18n";
  import { createClassSubjectsQuery } from "$lib/features/academics/queries";
  import {
    Accordion,
    AccordionContent,
    AccordionItem,
    AccordionTrigger,
  } from "@orbis/ui-svelte/accordion";

  interface Props {
    open: boolean;
    periodId: string;
    classId?: string;
    dayOfWeek?: number;
    timeSlotId?: string;
    lesson?: any | null;
    classes: any[];
    subjects: any[];
    teachers: any[];
    rooms: any[];
    timeSlots: any[];
    onSuccess?: () => void;
  }

  let {
    open = $bindable(false),
    periodId,
    classId = "",
    dayOfWeek = 1,
    timeSlotId = "",
    lesson = null,
    classes = [],
    subjects = [],
    teachers = [],
    rooms = [],
    timeSlots = [],
    onSuccess,
  }: Props = $props();

  // Form Fields
  let selectedClassId = $state("");
  let selectedSubjectId = $state("");
  let selectedTeacherId = $state("");
  let selectedRoomId = $state("");
  let selectedSlotId = $state("");
  let selectedDayOfWeek = $state(1);
  let topic = $state("");
  let homework = $state("");
  let notes = $state("");
  let lastClassId = $state("");

  let isSubmitting = $state(false);
  let isCheckingConflict = $state(false);
  let conflicts = $state<any[]>([]);

  // Fetch ClassSubjects for the selected class to validate assignments
  const classSubjectsQuery = createClassSubjectsQuery(
    () => selectedClassId,
    () => periodId
  );
  const classSubjects = $derived(classSubjectsQuery.data ?? []);
  const isLoadingClassSubjects = $derived(classSubjectsQuery.isFetching);

  // Filter available subjects to those taught in this class
  const classSubjectIds = $derived(
    new Set(classSubjects.map((cs) => String(cs.subject_id)))
  );
  const availableSubjects = $derived(
    !isLoadingClassSubjects && classSubjects.length > 0
      ? subjects.filter((s) => classSubjectIds.has(String(s.id)))
      : []
  );

  const selectedClassSubject = $derived(
    classSubjects.find((cs) => String(cs.subject_id) === String(selectedSubjectId))
  );

  const availableTeachers = $derived(
    selectedClassSubject?.primary_teacher_id
      ? teachers.filter((teacher) => String(teacher.id) === String(selectedClassSubject.primary_teacher_id))
      : []
  );

  // Sync form state when sheet opens or target lesson changes
  $effect(() => {
    if (open) {
      if (lesson) {
        selectedClassId = String(lesson.class_id || "");
        selectedSubjectId = String(lesson.subject_id || "");
        selectedTeacherId = String(lesson.teacher_id || "");
        selectedRoomId = lesson.room_id ? String(lesson.room_id) : "";
        selectedSlotId = String(lesson.time_slot_id || "");
        selectedDayOfWeek = Number(lesson.day_of_week) || 1;
        topic = lesson.topic || "";
        homework = lesson.homework || "";
        notes = lesson.notes || "";
      } else {
        selectedClassId = classId || (classes[0]?.id ? String(classes[0].id) : "");
        selectedSubjectId = "";
        selectedTeacherId = "";
        selectedRoomId = "";
        selectedSlotId = timeSlotId || (timeSlots[0]?.id ? String(timeSlots[0].id) : "");
        selectedDayOfWeek = dayOfWeek || 1;
        topic = "";
        homework = "";
        notes = "";
      }
      conflicts = [];
    }
  });

  // When subject changes, automatically pick the assigned primary teacher
  $effect(() => {
    if (open && !lesson?.id && selectedClassId && selectedClassId !== lastClassId) {
      lastClassId = selectedClassId;
      selectedSubjectId = "";
      selectedTeacherId = "";
    }

    if (!isLoadingClassSubjects && classSubjects.length === 1 && !selectedSubjectId && !lesson?.id) {
      selectedSubjectId = String(classSubjects[0].subject_id);
    }

    if (selectedSubjectId && selectedClassSubject?.primary_teacher_id) {
      const assignedTeacherId = String(selectedClassSubject.primary_teacher_id);
      if (selectedTeacherId !== assignedTeacherId) {
        selectedTeacherId = assignedTeacherId;
      }
    } else if (selectedSubjectId && selectedTeacherId) {
      selectedTeacherId = "";
    }
  });

  // Auto-run conflict checks on placement changes
  $effect(() => {
    if (open && periodId && selectedClassId && selectedTeacherId && selectedSlotId && selectedDayOfWeek) {
      checkLiveConflicts();
    }
  });

  async function checkLiveConflicts() {
    if (!periodId || !selectedClassId || !selectedTeacherId || !selectedSlotId) return;
    try {
      isCheckingConflict = true;
      const results = await commands.scheduling.checkConflicts(
        periodId,
        selectedClassId,
        selectedTeacherId,
        selectedSlotId,
        selectedDayOfWeek,
        selectedRoomId ? selectedRoomId : null
      );
      // Filter out self when editing
      conflicts = lesson?.id ? results.filter((c) => c.lesson_id !== lesson.id) : results;
    } catch (e) {
      console.error("Conflict check error:", e);
    } finally {
      isCheckingConflict = false;
    }
  }

  const weekdaysList = $derived([
    { val: 1, labelAr: "الأحد", labelEn: "Sunday" },
    { val: 2, labelAr: "الإثنين", labelEn: "Monday" },
    { val: 3, labelAr: "الثلاثاء", labelEn: "Tuesday" },
    { val: 4, labelAr: "الأربعاء", labelEn: "Wednesday" },
    { val: 5, labelAr: "الخميس", labelEn: "Thursday" },
    { val: 6, labelAr: "الجمعة", labelEn: "Friday" },
    { val: 7, labelAr: "السبت", labelEn: "Saturday" },
  ]);

  async function handleSubmit(e: SubmitEvent) {
    e.preventDefault();
    if (!selectedClassId || !selectedSubjectId || !selectedTeacherId || !selectedSlotId) {
      toast.error(m.common_lesson_required_fields());
      return;
    }

    if (conflicts.length > 0) {
      toast.error(m.common_lesson_schedule_conflict());
      return;
    }

    isSubmitting = true;
    try {
      // Find ClassSubject ID
      const cs = classSubjects.find((x) => String(x.subject_id) === String(selectedSubjectId));

      if (lesson?.id) {
        // Update Lesson
        await commands.scheduling.updateLesson(lesson.id, {
          teacher_id: selectedTeacherId,
          time_slot_id: selectedSlotId,
          day_of_week: selectedDayOfWeek,
          room_id: selectedRoomId || null,
          topic: topic.trim() || null,
          homework: homework.trim() || null,
          notes: notes.trim() || null,
        } as any);
        toast.success(m.common_lesson_updated());
      } else {
        // Create Lesson
        await commands.scheduling.createLesson({
          period_id: periodId,
          class_id: selectedClassId,
          class_subject_id: cs ? String(cs.id) : undefined,
          subject_id: selectedSubjectId,
          teacher_id: selectedTeacherId,
          time_slot_id: selectedSlotId,
          day_of_week: selectedDayOfWeek,
          room_id: selectedRoomId || null,
          topic: topic.trim() || null,
          homework: homework.trim() || null,
          notes: notes.trim() || null,
        } as any);
        toast.success(m.common_lesson_scheduled());
      }

      open = false;
      onSuccess?.();
    } catch (err) {
      toast.error(extractErrorMessage(err));
    } finally {
      isSubmitting = false;
    }
  }

  async function handleDelete() {
    if (!lesson?.id) return;
    try {
      await commands.scheduling.deleteLesson(lesson.id);
      toast.success(m.common_lesson_deleted());
      open = false;
      onSuccess?.();
    } catch (err) {
      toast.error(extractErrorMessage(err));
    }
  }
</script>

<Sheet bind:open>
  <SheetContent class="h-full w-full sm:max-w-lg overflow-hidden flex flex-col p-0 text-start">
    <SheetHeader class="border-b border-border/60 px-6 pb-4 pt-6 text-end">
      <SheetTitle>
        {lesson
          ? m.common_lesson_sheet_edit_title()
          : m.common_lesson_sheet_create_title()}
      </SheetTitle>
    </SheetHeader>

    <!-- Conflict Warning Banner -->
    {#if conflicts.length > 0}
      <div class="mt-4 rounded-lg border border-destructive/40 bg-destructive/10 p-3 text-xs text-destructive flex items-start gap-2">
        <AlertTriangle size={15} class="shrink-0 mt-0.5 text-destructive" />
        <div class="flex-1">
          <p class="font-bold">{m.common_lesson_conflict_title()}</p>
          <ul class="mt-1 list-disc list-inside space-y-0.5 text-[11px] opacity-90">
            {#each conflicts as c}
              <li>{c.message || m.common_lesson_conflict_detail()}</li>
            {/each}
          </ul>
        </div>
      </div>
    {/if}

    <form onsubmit={handleSubmit} class="mt-0 min-h-0 flex-1 overflow-y-auto scrollbar-hide px-6 py-5">
      <Accordion type="multiple" value={["placement", "content"]} class="flex flex-col gap-4">
        <AccordionItem value="placement" class="rounded-lg border border-border/50 px-4">
          <AccordionTrigger class="text-sm font-semibold hover:no-underline">{m.common_lesson_placement()}</AccordionTrigger>
          <AccordionContent>
      <!-- Class Selector -->
      <div class="space-y-1.5">
        <Label for="class-select" class="text-xs font-semibold">{m.common_lesson_class()} *</Label>
        <Select
          type="single"
          value={selectedClassId}
          onValueChange={(val) => (selectedClassId = val ?? "")}
          disabled={Boolean(lesson?.id)}
        >
          <SelectTrigger id="class-select" class="h-9 w-full min-w-0 text-xs">
            <div class="flex items-center gap-1.5 truncate">
              <Users size={13} class="text-muted-foreground" />
              <span>{classes.find((c) => String(c.id) === selectedClassId) ? l(classes.find((c) => String(c.id) === selectedClassId).name_ar, classes.find((c) => String(c.id) === selectedClassId).name) : m.common_lesson_select_class()}</span>
            </div>
          </SelectTrigger>
          <SelectContent class="max-h-56">
            {#each classes as c (c.id)}
              <SelectItem value={String(c.id)} class="text-xs">{c.name_ar || c.name}</SelectItem>
            {/each}
          </SelectContent>
        </Select>
      </div>

      <!-- Subject Selector -->
      <div class="space-y-1.5">
        <Label for="subject-select" class="text-xs font-semibold">{m.common_lesson_subject()} *</Label>
        <Select
          type="single"
          value={selectedSubjectId}
          onValueChange={(val) => (selectedSubjectId = val ?? "")}
          disabled={Boolean(lesson?.id)}
        >
          <SelectTrigger id="subject-select" class="h-9 w-full min-w-0 text-xs">
            <div class="flex items-center gap-1.5 truncate">
              <BookOpen size={13} class="text-muted-foreground" />
              <span>{availableSubjects.find((s) => String(s.id) === selectedSubjectId) ? l(availableSubjects.find((s) => String(s.id) === selectedSubjectId).name_ar, availableSubjects.find((s) => String(s.id) === selectedSubjectId).name) : m.common_lesson_select_subject()}</span>
            </div>
          </SelectTrigger>
          <SelectContent class="max-h-56">
            {#each availableSubjects as s (s.id)}
              <SelectItem value={String(s.id)} class="text-xs">
                <div class="flex items-center gap-2">
                  <span class="h-2 w-2 rounded-full" style={`background-color: ${s.color || 'var(--primary)'};`}></span>
                  <span>{s.name_ar || s.name}</span>
                </div>
              </SelectItem>
            {/each}
          </SelectContent>
        </Select>
      </div>

      <!-- Teacher Selector -->
      <div class="space-y-1.5">
        <Label for="teacher-select" class="text-xs font-semibold">{m.common_lesson_teacher()} *</Label>
        <Select
          type="single"
          value={selectedTeacherId}
          onValueChange={(val) => (selectedTeacherId = val ?? "")}
        >
          <SelectTrigger id="teacher-select" class="h-9 w-full min-w-0 text-xs">
            <div class="flex items-center gap-1.5 truncate">
              <User size={13} class="text-muted-foreground" />
              <span>{teachers.find((t) => String(t.id) === selectedTeacherId) ? l(teachers.find((t) => String(t.id) === selectedTeacherId).name_ar, teachers.find((t) => String(t.id) === selectedTeacherId).full_name) : m.common_lesson_select_teacher()}</span>
            </div>
          </SelectTrigger>
          <SelectContent class="max-h-56">
            {#each availableTeachers as t (t.id)}
              <SelectItem value={String(t.id)} class="text-xs">{t.name_ar || t.full_name}</SelectItem>
            {/each}
          </SelectContent>
        </Select>
      </div>

      <!-- Room Selector -->
      <div class="space-y-1.5">
        <Label for="room-select" class="text-xs font-semibold">{m.common_lesson_room_optional()}</Label>
        <Select
          type="single"
          value={selectedRoomId}
          onValueChange={(val) => (selectedRoomId = val ?? "")}
        >
          <SelectTrigger id="room-select" class="h-9 w-full min-w-0 text-xs">
            <div class="flex items-center gap-1.5 truncate">
              <MapPin size={13} class="text-muted-foreground" />
              <span>{rooms.find((r) => String(r.id) === selectedRoomId) ? l(rooms.find((r) => String(r.id) === selectedRoomId).name_ar, rooms.find((r) => String(r.id) === selectedRoomId).name) : m.common_lesson_no_room_selected()}</span>
            </div>
          </SelectTrigger>
          <SelectContent class="max-h-56">
            <SelectItem value="" class="text-xs text-muted-foreground">{m.common_lesson_no_room()}</SelectItem>
            {#each rooms as r (r.id)}
              <SelectItem value={String(r.id)} class="text-xs">
                {r.name_ar || r.name} {r.building ? `(${r.building})` : ""}
              </SelectItem>
            {/each}
          </SelectContent>
        </Select>
      </div>

      <!-- Day of Week & TimeSlot Grid -->
      <div class="grid grid-cols-2 gap-3">
        <div class="min-w-0 space-y-1.5">
          <Label class="text-xs font-semibold">{m.common_lesson_label_day()} *</Label>
          <Select
            type="single"
            value={String(selectedDayOfWeek)}
            onValueChange={(val) => (selectedDayOfWeek = Number(val) || 1)}
          >
            <SelectTrigger class="h-9 w-full min-w-0 text-xs">
              <span>{weekdaysList.find((w) => w.val === selectedDayOfWeek) ? l(weekdaysList.find((w) => w.val === selectedDayOfWeek).labelAr, weekdaysList.find((w) => w.val === selectedDayOfWeek).labelEn) : ""}</span>
            </SelectTrigger>
            <SelectContent>
              {#each weekdaysList as w}
                <SelectItem value={String(w.val)} class="text-xs">{l(w.labelAr, w.labelEn)}</SelectItem>
              {/each}
            </SelectContent>
          </Select>
        </div>

        <div class="min-w-0 space-y-1.5">
          <Label class="text-xs font-semibold">{m.common_lesson_time_slot()} *</Label>
          <Select
            type="single"
            value={selectedSlotId}
            onValueChange={(val) => (selectedSlotId = val ?? "")}
          >
            <SelectTrigger class="h-9 w-full min-w-0 text-xs">
              <div class="flex items-center gap-1.5 truncate">
                <Clock size={13} class="text-muted-foreground" />
                <span>{timeSlots.find((s) => String(s.id) === selectedSlotId) ? l(timeSlots.find((s) => String(s.id) === selectedSlotId).name_ar, timeSlots.find((s) => String(s.id) === selectedSlotId).name) : m.common_lesson_select_slot()}</span>
              </div>
            </SelectTrigger>
            <SelectContent class="max-h-56">
              {#each timeSlots.filter((s) => !s.is_break) as s (s.id)}
                <SelectItem value={String(s.id)} class="text-xs">
                  {s.name_ar || s.name || m.academics_schedule_placeholder_period_en({ number: s.slot_number })} ({s.start_time?.slice(0, 5)} - {s.end_time?.slice(0, 5)})
                </SelectItem>
              {/each}
            </SelectContent>
          </Select>
        </div>
      </div>

          </AccordionContent>
        </AccordionItem>

        <AccordionItem value="content" class="rounded-lg border border-border/50 px-4">
          <AccordionTrigger class="text-sm font-semibold hover:no-underline">{m.common_lesson_content()}</AccordionTrigger>
          <AccordionContent>

      <!-- Topic / Homework / Notes (Optional) -->
      <div class="space-y-1.5">
        <Label for="lesson-topic" class="text-xs font-semibold">{m.common_lesson_topic()}</Label>
        <Input id="lesson-topic" bind:value={topic} placeholder={m.common_lesson_topic_placeholder()} class="h-8 text-xs" />
      </div>

      <div class="space-y-1.5">
        <Label for="lesson-hw" class="text-xs font-semibold">{m.common_lesson_homework()}</Label>
        <Input id="lesson-hw" bind:value={homework} placeholder={m.common_lesson_homework_placeholder()} class="h-8 text-xs" />
      </div>

      <div class="space-y-1.5">
        <Label for="lesson-notes" class="text-xs font-semibold">{m.common_lesson_notes()}</Label>
        <Textarea id="lesson-notes" bind:value={notes} rows={2} placeholder={m.common_lesson_notes_placeholder()} class="text-xs" />
      </div>

          </AccordionContent>
        </AccordionItem>
      </Accordion>

      <SheetFooter class="sticky bottom-0 mt-0 flex items-center justify-between gap-2 border-t border-border/60 bg-background pt-4">
        {#if lesson?.id}
          <Button
            type="button"
            variant="destructive"
            size="sm"
            onclick={handleDelete}
            class="gap-1.5 text-xs"
          >
            <Trash2 size={13} />
            <span>{m.common_action_delete()}</span>
          </Button>
        {:else}
          <div></div>
        {/if}

        <div class="flex items-center gap-2">
          <Button
            type="button"
            variant="outline"
            size="sm"
            onclick={() => (open = false)}
            class="text-xs"
          >
            {m.common_action_cancel()}
          </Button>
          <Button
            type="submit"
            size="sm"
            disabled={isSubmitting || conflicts.length > 0}
            class="text-xs font-semibold"
          >
            {isSubmitting
              ? m.common_lesson_saving()
              : m.common_lesson_save()}
          </Button>
        </div>
      </SheetFooter>
    </form>
  </SheetContent>
</Sheet>
