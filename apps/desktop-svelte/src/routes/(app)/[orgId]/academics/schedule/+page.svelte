<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { readable } from "svelte/store";
import { page } from "$app/state";
import { confirmStore } from "$lib/stores";
import { commands } from "$lib/core/ipc";
import { toast } from "svelte-sonner";
import { getLocale, localizeHref } from "$lib/paraglide/runtime";
import { goto } from "$app/navigation";
import { WandSparkles } from "@lucide/svelte";
import { Button } from "@orbis/ui-svelte/button";
import { createQuery, useQueryClient } from "@tanstack/svelte-query";
import LessonCreateSheet from "$lib/features/academics/components/lesson-create-sheet.svelte";

// Component imports
import {
  ScheduleFilterControls,
  ScheduleBoard,
} from "$lib/features/academics";

let { data }: { data: { orgId: string } } = $props();

const queryClient = useQueryClient();

// Sync state selections
let activePeriodId = $state<string>("");
let selectedClassId = $state<string>("");
let selectedTeacherId = $state<string>("");
let viewMode = $state<"class" | "teacher">("class");

// Scheduling Sheet Trigger State
let isModalOpen = $state(false);
let activeDay = $state<number>(1); // 1 = Sunday
let activeSlotId = $state<string>("");
let activeLesson = $state<any | null>(null);

// Solver & AI Scheduling State
let activeJob = $state<any>(null);
let previewDraft = $state(false);

const periodsQuery = createQuery(() => ({
  queryKey: ["periods", data.orgId],
  queryFn: () => commands.calendar.listAcademicPeriods(),
  enabled: !!data.orgId,
  staleTime: 10 * 60 * 1000,
}));

const classesQuery = createQuery(() => ({
  queryKey: ["classes", data.orgId],
  queryFn: () => commands.academic.listClasses(),
  enabled: !!data.orgId,
  staleTime: 5 * 60 * 1000,
}));

const subjectsQuery = createQuery(() => ({
  queryKey: ["subjects", data.orgId],
  queryFn: () => commands.academic.listSubjects(),
  enabled: !!data.orgId,
  staleTime: 10 * 60 * 1000,
}));

const usersQuery = createQuery(() => ({
  queryKey: ["users", data.orgId],
  queryFn: () => commands.iam.listUsers(null),
  enabled: !!data.orgId,
  staleTime: 5 * 60 * 1000,
}));

const periods = $derived(periodsQuery.data ?? []);
const classes = $derived(classesQuery.data ?? []);
const subjects = $derived(subjectsQuery.data ?? []);
const users = $derived(usersQuery.data ?? []);

// Filter teachers list - strictly ONLY users with teacher role
const teachers = $derived(
  users.filter(
    (u: any) =>
      u.primary_role === "teacher" ||
      u.role === "teacher" ||
      (Array.isArray(u.roles) && u.roles.includes("teacher")),
  ),
);

// Track periods and activePeriodId selection
$effect(() => {
  if (periods.length > 0 && !activePeriodId) {
    const active =
      periods.find(
        (c: any) => c.status === "active" || c.status === "active",
      ) || periods[0];
    if (active) {
      activePeriodId = String(active.id);
    }
  }
});

// Track selections Sync
$effect(() => {
  if (classes.length > 0 && !selectedClassId) {
    const paramClassId =
      page.url.searchParams.get("classId") ||
      page.url.searchParams.get("class");
    if (
      paramClassId &&
      classes.some((c: any) => String(c.id) === paramClassId)
    ) {
      selectedClassId = paramClassId;
      viewMode = "class";
    } else {
      selectedClassId = String(classes[0].id);
    }
  }
});

$effect(() => {
  if (teachers.length > 0 && !selectedTeacherId) {
    const paramTeacherId =
      page.url.searchParams.get("teacherId") ||
      page.url.searchParams.get("teacher");
    if (
      paramTeacherId &&
      teachers.some((u: any) => String(u.id) === paramTeacherId)
    ) {
      selectedTeacherId = paramTeacherId;
      viewMode = "teacher";
    } else {
      selectedTeacherId = String(teachers[0].id);
    }
  }
});

// Time Slots logic
const timeSlotsQuery = createQuery(() => ({
  queryKey: ["time-slots", data.orgId, activePeriodId],
  queryFn: () => commands.scheduling.listTimeSlots(),
  enabled: !!data.orgId && !!activePeriodId,
  staleTime: 10 * 60 * 1000,
}));

const timeSlots = $derived(
  (timeSlotsQuery.data ?? []).filter(
    (slot: any) => String(slot.period_id) === String(activePeriodId),
  ),
);

// Schedules logic
const classScheduleQuery = createQuery(() => ({
  queryKey: ["class-schedule", data.orgId, activePeriodId, selectedClassId],
  queryFn: () =>
    commands.scheduling.listLessonsByClass(activePeriodId, selectedClassId),
  enabled:
    !!data.orgId &&
    !!activePeriodId &&
    !!selectedClassId &&
    viewMode === "class",
  staleTime: 2 * 60 * 1000,
}));

const teacherScheduleQuery = createQuery(() => ({
  queryKey: [
    "teacher-schedule",
    data.orgId,
    activePeriodId,
    selectedTeacherId,
  ],
  queryFn: () =>
    commands.scheduling.listLessonsByTeacher(
      activePeriodId,
      selectedTeacherId,
    ),
  enabled:
    !!data.orgId &&
    !!activePeriodId &&
    !!selectedTeacherId &&
    viewMode === "teacher",
  staleTime: 2 * 60 * 1000,
}));

const lessons = $derived.by(() => {
  if (viewMode === "class") {
    return classScheduleQuery.data ?? [];
  } else {
    return teacherScheduleQuery.data ?? [];
  }
});

const weekdays = $derived.by(() => {
  const configuredDays = new Set<number>();

  for (const slot of timeSlots) {
    if (!slot.applicable_days) continue;
    for (const value of String(slot.applicable_days).split(",")) {
      const day = Number(value.trim());
      if (Number.isInteger(day) && day >= 1 && day <= 7) {
        configuredDays.add(day);
      }
    }
  }

  for (const lesson of lessons) {
    const day = Number(lesson.day_of_week);
    if (Number.isInteger(day) && day >= 1 && day <= 7) {
      configuredDays.add(day);
    }
  }

  const days = configuredDays.size > 0 ? [...configuredDays] : [1, 2, 3, 4, 5];
  const locale = getLocale();

  const weekOrder = [7, 1, 2, 3, 4, 5, 6];
  return days.sort((a, b) => weekOrder.indexOf(a) - weekOrder.indexOf(b)).map((day) => ({
    value: day,
    label: new Intl.DateTimeFormat(locale, { weekday: "short" }).format(
      new Date(Date.UTC(2023, 0, day)),
    ),
  }));
});

const isLessonsLoading = $derived(
  viewMode === "class"
    ? classScheduleQuery.isLoading
    : teacherScheduleQuery.isLoading,
);

const isLoading = $derived(
  periodsQuery.isLoading ||
    classesQuery.isLoading ||
    subjectsQuery.isLoading ||
    usersQuery.isLoading ||
    timeSlotsQuery.isLoading ||
    isLessonsLoading,
);

const error = $derived(
  periodsQuery.error ||
    classesQuery.error ||
    subjectsQuery.error ||
    usersQuery.error ||
    timeSlotsQuery.error ||
    (viewMode === "class"
      ? classScheduleQuery.error
      : teacherScheduleQuery.error),
);

// Derived state to map and filter draft assignments for preview
const draftLessonsInGrid = $derived.by(() => {
  if (!previewDraft || !activeJob || !activeJob.best_solution) return [];
  try {
    const solution = JSON.parse(activeJob.best_solution);
    const assignments = solution.assignments || [];
    return assignments
      .filter((a: any) => {
        if (viewMode === "class") {
          return String(a.class_id) === String(selectedClassId);
        } else {
          return String(a.teacher_id) === String(selectedTeacherId);
        }
      })
      .map((a: any) => ({
        id: a.id,
        class_id: a.class_id,
        subject_id: a.subject_id,
        teacher_id: a.teacher_id,
        time_slot_id: a.time_slot_id,
        day_of_week: a.day_of_week,
        room: a.room_id || null,
        isDraft: true,
      }));
  } catch (e) {
    console.error("Error parsing draft solution:", e);
    return [];
  }
});

async function handleMoveLesson(
  lesson: any,
  targetDay: number,
  targetSlotId: string,
) {
  try {
    toast.loading(m.academics_schedule_checking_for_conflicts());
    const conflicts = await commands.scheduling.checkConflicts(
      activePeriodId,
      lesson.teacher_id,
      targetSlotId,
      targetDay,
      lesson.room_id || undefined,
    );

    toast.dismiss();

    if (conflicts.length > 0) {
      const conflictMsgs = conflicts.map((c: any) => c.message).join("\n");
      toast.error(m.academics_schedule_conflicts_detected_for_this_schedule(), {
        description: conflictMsgs,
      });
      return;
    }

    await commands.scheduling.updateLesson(lesson.id, {
      day_of_week: targetDay,
      time_slot_id: targetSlotId,
    } as any);
    toast.success(m.academics_schedule_lesson_moved_successfully());
    refetchLessons();
  } catch (e: any) {
    toast.dismiss();
    toast.error((e as any)?.message || "Failed to move lesson");
  }
}

async function handleDeleteLesson(lessonId: string) {
  const confirmed = await confirmStore.prompt({
    title: m.common_action_delete() + "؟",
    description: m.common_schedule_page_confirm_delete(),
    confirmText: m.common_action_delete(),
    variant: "destructive",
  });
  if (!confirmed) return;
  try {
    await commands.scheduling.deleteLesson(lessonId);
    toast.success(m.common_schedule_page_success_delete());
    refetchLessons();
  } catch (e) {
    toast.error(m.common_schedule_page_error_delete());
  }
}

function refetchLessons() {
  queryClient.invalidateQueries({
    queryKey: ["class-schedule", data.orgId, activePeriodId, selectedClassId],
  });
  queryClient.invalidateQueries({
    queryKey: [
      "teacher-schedule",
      data.orgId,
      activePeriodId,
      selectedTeacherId,
    ],
  });
}

function refetchTimeSlots() {
  queryClient.invalidateQueries({
    queryKey: ["time-slots", data.orgId, activePeriodId],
  });
}

async function deleteTimeSlot(slot: any) {
  const confirmed = await confirmStore.prompt({
    title: m.common_action_delete() + "؟",
    description: "سيتم حذف الفترة من الجدول.",
    confirmText: m.common_action_delete(),
    variant: "destructive",
  });
  if (!confirmed) return;
  try {
    await commands.scheduling.deleteTimeSlot(String(slot.id));
    toast.success("تم حذف الفترة");
    refetchTimeSlots();
  } catch (error) {
    toast.error(error instanceof Error ? error.message : "تعذر حذف الفترة");
  }
}

async function saveTimeSlot(slot: any | null, slotData: any) {
  if (slotData.duration_minutes <= 0) {
    toast.error("يجب أن يكون وقت النهاية بعد وقت البداية");
    return;
  }
  try {
    if (slot) {
      await commands.scheduling.updateTimeSlot(String(slot.id), slotData);
      toast.success("تم تعديل الفترة");
    } else {
      const slotNumber = timeSlots.reduce(
        (highest, item) => Math.max(highest, Number(item.slot_number) || 0),
        0,
      ) + 1;
      await commands.scheduling.createTimeSlot({
        period_id: activePeriodId,
        slot_number: slotNumber,
        ...slotData,
        is_break: false,
        applicable_days: "1,2,3,4,5",
      });
      toast.success("تمت إضافة الحصة");
    }
    refetchTimeSlots();
  } catch (error) {
    toast.error(error instanceof Error ? error.message : "تعذر حفظ الفترة");
  }
}

function getSubjectName(subjectId: string) {
  const sub = subjects.find((s) => String(s.id) === String(subjectId));
  return sub
    ? sub.name_ar || sub.name
    : m.common_schedule_page_unknown_subject();
}

function getTeacherName(teacherId: string) {
  const teach = teachers.find((t) => String(t.id) === String(teacherId));
  return teach
    ? getLocale() === "ar"
      ? teach.name_ar || teach.full_name
      : teach.full_name
    : m.common_schedule_page_no_teacher();
}

function getClassName(classId: string) {
  const cls = classes.find((c) => String(c.id) === String(classId));
  return cls ? cls.name_ar || cls.name : m.common_schedule_page_unknown_class();
}

function getSubjectColor(subjectId: string) {
  const sub = subjects.find((s) => String(s.id) === String(subjectId)) as any;
  return sub ? sub.color : "#9ca3af";
}

function openScheduler(day: number, slotId: string) {
  if (viewMode !== "class") {
    toast.error(m.common_schedule_page_error_class_only());
    return;
  }
  activeDay = day;
  activeSlotId = slotId;
  activeLesson = null;
  isModalOpen = true;
}

function openLessonEditor(lesson: any) {
  activeLesson = lesson;
  activeDay = Number(lesson.day_of_week);
  activeSlotId = String(lesson.time_slot_id);
  isModalOpen = true;
}
</script>


<div class="h-full w-full flex flex-col items-stretch overflow-hidden">
  <div class="flex-1 min-h-0 min-w-0 w-full flex flex-col p-4 sm:p-6 gap-4 sm:gap-6 overflow-y-auto">
    <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3 shrink-0">
      <ScheduleFilterControls
        bind:viewMode
        bind:selectedClassId
        bind:selectedTeacherId
        {classes}
        {teachers}
      />
      <Button
        variant="default"
        class="w-full sm:w-auto h-9 text-xs font-semibold gap-2 cursor-pointer"
        onclick={() => goto(localizeHref(`/${data.orgId}/academics/schedule/prepare`))}
      >
        <WandSparkles size={14} />
        {m.common_schedule_ai_auto_scheduler()}
      </Button>
    </div>

    <!-- Timetable Grid Content -->
    <ScheduleBoard
      {isLoading}
      error={error?.message || null}
      {timeSlots}
      {weekdays}
      {lessons}
      draftLessons={draftLessonsInGrid}
      {previewDraft}
      {viewMode}
      {getSubjectName}
      {getTeacherName}
      {getClassName}
      {getSubjectColor}
      onCellClick={openScheduler}
      onDeleteLesson={handleDeleteLesson}
      onEditLesson={openLessonEditor}
      onMoveLesson={handleMoveLesson}
      onAddTimeSlot={() => {}}
      onEditTimeSlot={() => {}}
      onDeleteTimeSlot={(slot) => deleteTimeSlot(slot)}
      onSaveTimeSlot={saveTimeSlot}
      onRetry={refetchLessons}
    />
  </div>
</div>

<!-- Lesson Scheduling Side Sheet -->
<LessonCreateSheet
  bind:open={isModalOpen}
  orgId={data.orgId}
  periodId={activePeriodId}
  classId={selectedClassId}
  dayOfWeek={activeDay}
  timeSlotId={activeSlotId}
  timeSlots={timeSlots}
  lesson={activeLesson}
  {subjects}
  {teachers}
  onSuccess={refetchLessons}
/>
