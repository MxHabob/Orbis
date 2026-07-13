<script lang="ts">
import { onMount } from "svelte";
import { readable } from "svelte/store";
import { page } from "$app/state";
import { commands } from "$lib/core/ipc";
import { toast } from "svelte-sonner";
import { getLocale } from "$lib/paraglide/runtime";
import { createQuery, useQueryClient } from "@tanstack/svelte-query";
import LessonCreateSheet from "$lib/features/academics/components/lesson-create-sheet.svelte";
import SchedulerAiPanel from "$lib/features/academics/components/scheduler-ai-panel.svelte";
import * as m from "$lib/paraglide/messages";

// Component imports
import {
  ScheduleFilterControls,
  ScheduleTimetableWrapper,
} from "$lib/features/academics";

let { data }: { data: { orgId: string } } = $props();

const queryClient = useQueryClient();

// Sync state selections
let activeYearId = $state<string>("");
let selectedClassId = $state<string>("");
let selectedTeacherId = $state<string>("");
let viewMode = $state<"class" | "teacher">("class");

// Scheduling Sheet Trigger State
let isModalOpen = $state(false);
let activeDay = $state<number>(1); // 1 = Sunday
let activeSlotId = $state<string>("");

// Solver & AI Scheduling State
let activeJob = $state<any>(null);
let previewDraft = $state(false);

const weekdays = $derived([
  { value: 1, label: m.schedule_page_day_sun() },
  { value: 2, label: m.schedule_page_day_mon() },
  { value: 3, label: m.schedule_page_day_tue() },
  { value: 4, label: m.schedule_page_day_wed() },
  { value: 5, label: m.schedule_page_day_thu() },
]);

// Helper to convert Svelte 5 reactive getters into Svelte 4/5 Stores
function toStore<T>(getter: () => T) {
  return readable<T>(getter(), (set) => {
    const cleanup = $effect.root(() => {
      $effect(() => {
        set(getter());
      });
    });
    return cleanup;
  });
}

// Reactive Queries using toStore for Svelte 5 runes responsiveness
const periodsQuery = createQuery(
  toStore(() => ({
    queryKey: ["periods", data.orgId],
    queryFn: () => commands.academic.listPeriods(data.orgId),
    enabled: !!data.orgId,
    staleTime: 10 * 60 * 1000,
  })),
);

const classesQuery = createQuery(
  toStore(() => ({
    queryKey: ["classes", data.orgId],
    queryFn: () => commands.academic.listClasses(data.orgId),
    enabled: !!data.orgId,
    staleTime: 5 * 60 * 1000,
  })),
);

const subjectsQuery = createQuery(
  toStore(() => ({
    queryKey: ["subjects", data.orgId],
    queryFn: () => commands.academic.listSubjects(data.orgId),
    enabled: !!data.orgId,
    staleTime: 10 * 60 * 1000,
  })),
);

const usersQuery = createQuery(
  toStore(() => ({
    queryKey: ["users", data.orgId],
    queryFn: () => commands.iam.listUsers(data.orgId),
    enabled: !!data.orgId,
    staleTime: 5 * 60 * 1000,
  })),
);

const periods = $derived($periodsQuery.data ?? []);
const classes = $derived($classesQuery.data ?? []);
const subjects = $derived($subjectsQuery.data ?? []);
const users = $derived($usersQuery.data ?? []);

// Filter teachers list
const teachers = $derived(
  users.filter(
    (u: any) =>
      u.primary_role === "teacher" ||
      u.primary_role === "admin" ||
      u.primary_role === "super_admin",
  ),
);

// Track periods and activeYearId selection
$effect(() => {
  if (periods.length > 0 && !activeYearId) {
    const active =
      periods.find((c: any) => c.is_current === 1 || c.is_current === true) ||
      periods[0];
    if (active) {
      activeYearId = String(active.id);
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
const timeSlotsQuery = createQuery(
  toStore(() => ({
    queryKey: ["time-slots", data.orgId, activeYearId],
    queryFn: () => commands.scheduling.listTimeSlots(data.orgId, activeYearId),
    enabled: !!data.orgId && !!activeYearId,
    staleTime: 10 * 60 * 1000,
  })),
);

const timeSlots = $derived($timeSlotsQuery.data ?? []);

// Schedules logic
const classScheduleQuery = createQuery(
  toStore(() => ({
    queryKey: ["class-schedule", data.orgId, activeYearId, selectedClassId],
    queryFn: () =>
      commands.scheduling.listLessonsByClass(
        data.orgId,
        activeYearId,
        selectedClassId,
      ),
    enabled:
      !!data.orgId &&
      !!activeYearId &&
      !!selectedClassId &&
      viewMode === "class",
    staleTime: 2 * 60 * 1000,
  })),
);

const teacherScheduleQuery = createQuery(
  toStore(() => ({
    queryKey: ["teacher-schedule", data.orgId, activeYearId, selectedTeacherId],
    queryFn: () =>
      commands.scheduling.listLessonsByTeacher(
        data.orgId,
        activeYearId,
        selectedTeacherId,
      ),
    enabled:
      !!data.orgId &&
      !!activeYearId &&
      !!selectedTeacherId &&
      viewMode === "teacher",
    staleTime: 2 * 60 * 1000,
  })),
);

const lessons = $derived.by(() => {
  if (viewMode === "class") {
    return $classScheduleQuery.data ?? [];
  } else {
    return $teacherScheduleQuery.data ?? [];
  }
});

const isLessonsLoading = $derived(
  viewMode === "class"
    ? $classScheduleQuery.isLoading
    : $teacherScheduleQuery.isLoading,
);

const isLoading = $derived(
  $periodsQuery.isLoading ||
    $classesQuery.isLoading ||
    $subjectsQuery.isLoading ||
    $usersQuery.isLoading ||
    $timeSlotsQuery.isLoading ||
    isLessonsLoading,
);

const error = $derived(
  $periodsQuery.error ||
    $classesQuery.error ||
    $subjectsQuery.error ||
    $usersQuery.error ||
    $timeSlotsQuery.error ||
    (viewMode === "class"
      ? $classScheduleQuery.error
      : $teacherScheduleQuery.error),
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
    toast.loading(
      getLocale() === "ar"
        ? "جاري التحقق من وجود تضاربات..."
        : "Checking for conflicts...",
    );
    const conflicts = await commands.scheduling.checkConflicts({
      orgId: data.orgId,
      yearId: activeYearId,
      timeSlotId: targetSlotId,
      dayOfWeek: targetDay,
      teacherId: lesson.teacher_id,
      classId: lesson.class_id,
      roomId: lesson.room_id || undefined,
      excludeId: lesson.id,
    });

    toast.dismiss();

    if (conflicts.length > 0) {
      const conflictMsgs = conflicts.map((c: any) => c.message).join("\n");
      toast.error(
        getLocale() === "ar"
          ? "توجد تضاربات في هذا الموعد!"
          : "Conflicts detected for this schedule!",
        {
          description: conflictMsgs,
        },
      );
      return;
    }

    await commands.scheduling.updateLesson(data.orgId, lesson.id, {
      day_of_week: targetDay,
      time_slot_id: targetSlotId,
    });
    toast.success(
      getLocale() === "ar" ? "تم نقل الحصة بنجاح" : "Lesson moved successfully",
    );
    refetchLessons();
  } catch (e: any) {
    toast.dismiss();
    toast.error(e?.message || "Failed to move lesson");
  }
}

async function handleDeleteLesson(lessonId: string) {
  if (!confirm(m.schedule_page_confirm_delete())) return;
  try {
    await commands.scheduling.deleteLesson(lessonId);
    toast.success(m.schedule_page_success_delete());
    refetchLessons();
  } catch (e) {
    toast.error(m.schedule_page_error_delete());
  }
}

function refetchLessons() {
  queryClient.invalidateQueries({
    queryKey: ["class-schedule", data.orgId, activeYearId, selectedClassId],
  });
  queryClient.invalidateQueries({
    queryKey: ["teacher-schedule", data.orgId, activeYearId, selectedTeacherId],
  });
}

function getSubjectName(subjectId: string) {
  const sub = subjects.find((s) => String(s.id) === String(subjectId));
  return sub ? sub.name_ar || sub.name : m.schedule_page_unknown_subject();
}

function getTeacherName(teacherId: string) {
  const teach = teachers.find((t) => String(t.id) === String(teacherId));
  return teach
    ? getLocale() === "ar"
      ? teach.name_ar || teach.full_name
      : teach.full_name
    : m.schedule_page_no_teacher();
}

function getClassName(classId: string) {
  const cls = classes.find((c) => String(c.id) === String(classId));
  return cls ? cls.name_ar || cls.name : m.schedule_page_unknown_class();
}

function getSubjectColor(subjectId: string) {
  const sub = subjects.find((s) => String(s.id) === String(subjectId));
  return sub ? sub.color : "#9ca3af";
}

function openScheduler(day: number, slotId: string) {
  if (viewMode !== "class") {
    toast.error(m.schedule_page_error_class_only());
    return;
  }
  activeDay = day;
  activeSlotId = slotId;
  isModalOpen = true;
}
</script>

<div class="h-full w-full flex flex-col lg:flex-row items-stretch overflow-hidden">
  <!-- Main Schedule Panel Column -->
  <div class="flex-1 min-w-0 w-full flex flex-col p-6 gap-6 overflow-y-auto">
    <!-- Timetable Header Filter Controls -->
    <ScheduleFilterControls
      bind:viewMode
      bind:selectedClassId
      bind:selectedTeacherId
      {classes}
      {teachers}
    />

    <!-- Timetable Grid Content -->
    <ScheduleTimetableWrapper
      {isLoading}
      {error}
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
      onMoveLesson={handleMoveLesson}
      onRetry={refetchLessons}
    />
  </div>

  <!-- AI Auto-Scheduler Sidebar Panel (Fixed Side) -->
  {#if !isLoading && !error}
    <SchedulerAiPanel
      orgId={data.orgId}
      yearId={activeYearId}
      selectedClassId={selectedClassId}
      selectedTeacherId={selectedTeacherId}
      viewMode={viewMode}
      {subjects}
      bind:previewDraft={previewDraft}
      bind:activeJob={activeJob}
      onApply={refetchLessons}
    />
  {/if}
</div>

<!-- Lesson Scheduling Side Sheet -->
<LessonCreateSheet
  bind:open={isModalOpen}
  orgId={data.orgId}
  yearId={activeYearId}
  classId={selectedClassId}
  dayOfWeek={activeDay}
  timeSlotId={activeSlotId}
  {subjects}
  {teachers}
  onSuccess={refetchLessons}
/>
