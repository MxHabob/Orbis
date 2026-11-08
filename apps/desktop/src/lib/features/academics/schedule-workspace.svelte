<!--
  @component ScheduleWorkspace
  Feature component for Timetable and Schedule workspace.
  Supports Class, Teacher, Room, Subject, and Matrix perspectives.
-->
<script lang="ts">
import { goto } from "$app/navigation";
import { resolve } from "$app/paths";
import {
  api,
  extractErrorMessage,
  queryKeys,
  type ScheduleLessonRequest,
  type OrgUnitDto,
} from "$lib/api";
import { getLocale } from "$lib/paraglide/runtime";
import { toast } from "@orbis/ui";
import { createQuery, useQueryClient } from "@tanstack/svelte-query";
import { TimetableGrid } from "@orbis/ui";
import { Skeleton } from "@orbis/ui/skeleton";
import { Badge } from "@orbis/ui/badge";
import {
  BookOpen,
  Calendar,
  ClockRotateRight,
  Pin,
  User,
  Users,
} from "@orbis/icons";
import * as m from "$lib/paraglide/messages";

import {
  ScheduleToolbar,
  ScheduleMatrix,
  LessonSheet,
  TimeSlotSheet,
  SolverDrawer,
} from "$lib/features/academics";
import ErrorState from "$lib/shared/components/error-state.svelte";
import EmptyState from "$lib/shared/components/empty-state.svelte";
import { l, formatDate, getWeekdayOptions } from "$lib/core/i18n";
import { openPrintDialog } from "$lib/stores/print.svelte";

let { orgId }: { orgId: string } = $props();

const queryClient = useQueryClient();

// Selection & View State
let activePeriodId = $state<string>("");
let viewMode = $state<"class" | "teacher" | "room" | "subject" | "matrix">(
  "class",
);
let selectedClassId = $state<string>("");
let selectedTeacherId = $state<string>("");
let selectedRoomId = $state<string>("");
let selectedSubjectId = $state<string>("");
let searchTerm = $state("");
let compactView = $state(false);
let selectedMatrixDay = $state(0);

// Dialog & Sheet States
let isLessonSheetOpen = $state(false);
let isTimeSlotsSheetOpen = $state(false);
let isSolverDrawerOpen = $state(false);

// Active Lesson for editing / creating
let targetLesson = $state<any | null>(null);
let activeCellDay = $state(0);
let activeCellSlotId = $state("");

// AI Solver Draft State
let previewDraft = $state(false);
let activeSolveJob = $state<any>(null);

// ── Queries ───────────────────────────────────────────────

const periodsQuery = createQuery(() => ({
  queryKey: ["periods", orgId],
  queryFn: () => api.listAcademicPeriods(),
  enabled: !!orgId,
  staleTime: 10 * 60 * 1000,
}));

const classesQuery = createQuery(() => ({
  queryKey: ["classes", orgId],
  queryFn: () => api.listSections(),
  enabled: !!orgId,
  staleTime: 5 * 60 * 1000,
}));

const subjectsQuery = createQuery(() => ({
  queryKey: ["subjects", orgId],
  queryFn: () => api.listSubjects(),
  enabled: !!orgId,
  staleTime: 10 * 60 * 1000,
}));

const teachersQuery = createQuery(() => ({
  queryKey: [
    "employees",
    orgId,
    { employee_type: "teacher", status: "active" },
  ],
  queryFn: () =>
    api.listEmployees({ employee_type: "teacher", status: "active" }),
  enabled: !!orgId,
  staleTime: 5 * 60 * 1000,
}));

const roomsQuery = createQuery(() => ({
  queryKey: ["rooms", orgId],
  queryFn: () => api.listRooms(),
  enabled: !!orgId,
  staleTime: 10 * 60 * 1000,
}));

const orgUnitsQuery = createQuery<OrgUnitDto[]>(() => ({
  queryKey: queryKeys.org.units(orgId),
  queryFn: () => api.listOrgUnits(),
  enabled: !!orgId,
  staleTime: 5 * 60 * 1000,
}));

const periods = $derived(periodsQuery.data ?? []);
const classes = $derived(classesQuery.data ?? []);
const subjects = $derived(subjectsQuery.data ?? []);
const rooms = $derived(roomsQuery.data ?? []);
const orgUnits = $derived(orgUnitsQuery.data ?? []);
const teachers = $derived(teachersQuery.data?.items ?? []);

const effectiveActivePeriodId = $derived(
  activePeriodId ||
    String(
      periods.find((p: any) => p.status === "active" || p.is_active)?.id ||
        periods[0]?.id ||
        "",
    ),
);
// Time Slots & Lessons Query for active period
const timeSlotsQuery = createQuery(() => ({
  queryKey: ["time-slots", orgId, effectiveActivePeriodId],
  queryFn: () => api.listTimeSlots(effectiveActivePeriodId),
  enabled: !!orgId && !!effectiveActivePeriodId,
  staleTime: 5 * 60 * 1000,
}));

const periodScheduleQuery = createQuery(() => ({
  queryKey: ["period-schedule", orgId, effectiveActivePeriodId],
  queryFn: () => api.listScheduledLessons(effectiveActivePeriodId),
  enabled: !!orgId && !!effectiveActivePeriodId,
  staleTime: 2 * 60 * 1000,
}));

const sectionSubjectsQuery = createQuery(() => ({
  queryKey: [
    "all-class-subjects-matrix",
    orgId,
    effectiveActivePeriodId,
    classes.map((section) => section.id).join(","),
  ],
  queryFn: async () => {
    const offerings = await Promise.all(
      classes.map((section) =>
        api.listSectionSubjects(String(section.id), effectiveActivePeriodId),
      ),
    );
    return offerings.flat();
  },
  enabled: !!orgId && !!effectiveActivePeriodId && classes.length > 0,
  staleTime: 2 * 60 * 1000,
}));

const timeSlots = $derived(timeSlotsQuery.data ?? []);
const allLessons = $derived(periodScheduleQuery.data ?? []);
const sectionSubjects = $derived(sectionSubjectsQuery.data ?? []);
const timeSlotsById = $derived(
  new Map(timeSlots.map((slot) => [String(slot.id), slot])),
);
const sectionSubjectsById = $derived(
  new Map(sectionSubjects.map((item) => [String(item.id), item])),
);

const weekdayOptions = getWeekdayOptions();
const weekdayOrder = new Map(
  weekdayOptions.map((weekday, index) => [weekday.value, index]),
);
const weekdays = $derived(
  [...new Set(timeSlots.map((slot) => Number(slot.day_of_week)))]
    .filter((day) => Number.isInteger(day) && day >= 1 && day <= 7)
    .sort(
      (left, right) =>
        (weekdayOrder.get(left) ?? 0) - (weekdayOrder.get(right) ?? 0),
    )
    .map((day) => ({
      value: day,
      day,
      label:
        weekdayOptions.find((weekday) => weekday.value === day)?.label ?? "",
    })),
);
const activeMatrixDay = $derived(
  weekdays.some((day) => day.value === selectedMatrixDay)
    ? selectedMatrixDay
    : (weekdays[0]?.value ?? 0),
);

const scheduledLessons = $derived.by(() =>
  allLessons.flatMap((lesson) => {
    const sectionSubject = sectionSubjectsById.get(
      String(lesson.section_subject_id),
    );
    const timeSlot = timeSlotsById.get(String(lesson.time_slot_id));
    if (!sectionSubject || !timeSlot) return [];

    return [
      {
        ...lesson,
        section_id: sectionSubject.section_id,
        class_id: sectionSubject.section_id,
        subject_id: sectionSubject.subject_id,
        day_of_week: timeSlot.day_of_week,
        teacher_id:
          lesson.teacher_id ?? sectionSubject.instructor_id ?? undefined,
        time_slot_start: timeSlot.start_time,
        time_slot_end: timeSlot.end_time,
      },
    ];
  }),
);

// Conflict Detection
const conflictLessonIds = $derived.by(() => {
  const ids = new Set<string>();
  const teacherOccupancy = new Map<string, string>();
  const roomOccupancy = new Map<string, string>();
  const classOccupancy = new Map<string, string>();

  for (const lesson of scheduledLessons) {
    const key = `${lesson.day_of_week}_${lesson.time_slot_id}`;
    if (lesson.teacher_id) {
      const tKey = `${key}_${lesson.teacher_id}`;
      if (teacherOccupancy.has(tKey)) {
        ids.add(lesson.id);
        ids.add(teacherOccupancy.get(tKey)!);
      } else {
        teacherOccupancy.set(tKey, lesson.id);
      }
    }

    if (lesson.room_id) {
      const rKey = `${key}_${lesson.room_id}`;
      if (roomOccupancy.has(rKey)) {
        ids.add(lesson.id);
        ids.add(roomOccupancy.get(rKey)!);
      } else {
        roomOccupancy.set(rKey, lesson.id);
      }
    }

    if (lesson.section_id) {
      const cKey = `${key}_${lesson.section_id}`;
      if (classOccupancy.has(cKey)) {
        ids.add(lesson.id);
        ids.add(classOccupancy.get(cKey)!);
      } else {
        classOccupancy.set(cKey, lesson.id);
      }
    }
  }

  return ids;
});

// Filtered Lessons by View Mode
const filteredLessons = $derived.by(() => {
  if (viewMode === "class") {
    if (!selectedClassId) return [];
    return scheduledLessons.filter(
      (l) => String(l.section_id) === String(selectedClassId),
    );
  }

  if (viewMode === "teacher") {
    if (!selectedTeacherId) return [];
    return scheduledLessons.filter(
      (l) => String(l.teacher_id) === String(selectedTeacherId),
    );
  }

  if (viewMode === "room") {
    if (!selectedRoomId) return [];
    return scheduledLessons.filter(
      (l) => String(l.room_id) === String(selectedRoomId),
    );
  }

  if (viewMode === "subject") {
    if (!selectedSubjectId) return [];
    return scheduledLessons.filter(
      (l) => String(l.subject_id) === String(selectedSubjectId),
    );
  }

  return scheduledLessons;
});

const searchedLessons = $derived.by(() => {
  const query = searchTerm.trim().toLocaleLowerCase();
  if (!query) return filteredLessons;

  return filteredLessons.filter((lesson) =>
    [
      getSubjectName(lesson.subject_id),
      getTeacherName(lesson.teacher_id),
      getClassName(lesson.section_id),
      getRoomName(lesson.room_id),
    ].some((value) => value.toLocaleLowerCase().includes(query)),
  );
});

// Draft Lessons for AI Scheduler Preview
const draftLessons = $derived.by(() => {
  if (!previewDraft || !activeSolveJob?.solution) return [];
  try {
    const sol =
      typeof activeSolveJob.solution === "string"
        ? JSON.parse(activeSolveJob.solution)
        : activeSolveJob.solution;
    const assignments = sol.assignments || [];
    return assignments.map((a: any) => ({
      ...a,
      isDraft: true,
    }));
  } catch {
    return [];
  }
});

// Name Resolution Helpers
function getSubjectName(subjectId?: string | null): string {
  if (!subjectId) return m.common_search_group_subjects();
  const sub = subjects.find((s) => String(s.id) === String(subjectId));
  return sub ? sub.name_ar || sub.name : m.common_search_group_subjects();
}

function getTeacherName(teacherId?: string | null): string {
  if (!teacherId) return "";
  const teach = teachers.find((t) => String(t.id) === String(teacherId));
  return teach ? teach.full_name_ar || teach.full_name : "";
}

function getClassName(clsId?: string | null): string {
  if (!clsId) return "";
  const cls = classes.find((c) => String(c.id) === String(clsId));
  return cls ? cls.name_ar || cls.name : "";
}

function getRoomName(roomId?: string | null): string {
  if (!roomId) return "";
  const rm = rooms.find((r) => String(r.id) === String(roomId));
  return rm ? rm.name_ar || rm.name : "";
}

function getSubjectColor(subjectId: string): string {
  const sub = subjects.find((s) => String(s.id) === String(subjectId));
  if (sub && "color" in sub && sub.color) return String(sub.color);
  return "var(--primary)";
}

// Workload summary for Teacher view
const teacherWorkload = $derived.by(() => {
  if (viewMode !== "teacher" || !selectedTeacherId) return null;
  const teacherLessons = scheduledLessons.filter(
    (l) => String(l.teacher_id) === selectedTeacherId,
  );
  const nonBreakSlotsCount = timeSlots.filter((s) => !s.is_break).length;
  const totalWeeklySlots = nonBreakSlotsCount * weekdays.length;
  const teachingCount = teacherLessons.length;
  const freeCount = Math.max(0, totalWeeklySlots - teachingCount);
  const assignedRooms = new Set(
    teacherLessons.map((l) => l.room_id).filter(Boolean),
  ).size;
  return { teachingCount, freeCount, assignedRooms, totalWeeklySlots };
});

// Occupancy summary for Room view
const roomOccupancy = $derived.by(() => {
  if (viewMode !== "room" || !selectedRoomId) return null;
  const roomLessons = scheduledLessons.filter(
    (l) => String(l.room_id) === selectedRoomId,
  );
  const nonBreakSlotsCount = timeSlots.filter((s) => !s.is_break).length;
  const totalWeeklySlots = nonBreakSlotsCount * weekdays.length;
  const occupancyRate =
    totalWeeklySlots > 0
      ? Math.round((roomLessons.length / totalWeeklySlots) * 100)
      : 0;
  const rm = rooms.find((r) => String(r.id) === selectedRoomId);
  return {
    occupiedCount: roomLessons.length,
    totalSlots: totalWeeklySlots,
    occupancyRate,
    capacity: rm?.capacity || null,
  };
});

// Action Handlers
function refetchSchedule() {
  queryClient.invalidateQueries({
    queryKey: ["period-schedule", orgId, effectiveActivePeriodId],
  });
  queryClient.invalidateQueries({
    queryKey: ["time-slots", orgId, effectiveActivePeriodId],
  });
}

function openNewLessonSheet(day?: number, slotId?: string) {
  targetLesson = null;
  const selectedSlot =
    timeSlots.find((slot) => String(slot.id) === String(slotId)) ??
    timeSlots.find(
      (slot) => !slot.is_break && Number(slot.day_of_week) === day,
    ) ??
    timeSlots.find((slot) => !slot.is_break);
  activeCellDay = Number(
    selectedSlot?.day_of_week ?? day ?? weekdays[0]?.value ?? 0,
  );
  activeCellSlotId = selectedSlot ? String(selectedSlot.id) : "";
  isLessonSheetOpen = true;
}

function openEditLessonSheet(lesson: any) {
  targetLesson = lesson;
  activeCellDay = Number(lesson?.day_of_week) || 0;
  activeCellSlotId = String(lesson?.time_slot_id || "");
  isLessonSheetOpen = true;
}

function openLessonAttendance(lesson: any) {
  if (!lesson.section_id || !lesson.id) return;

  const date = new Date();
  const dayOffset = (Number(lesson.day_of_week) - 1 - date.getDay() + 7) % 7;
  date.setDate(date.getDate() + dayOffset);
  const dateValue = [
    date.getFullYear(),
    String(date.getMonth() + 1).padStart(2, "0"),
    String(date.getDate()).padStart(2, "0"),
  ].join("-");
  const params = new URLSearchParams({
    classId: String(lesson.section_id),
    lessonId: String(lesson.id),
    date: dateValue,
  });

  goto(
    `${resolve("/(app)/[orgId]/academics/attendance", { orgId })}?${params.toString()}`,
  );
}

async function handleMoveLesson(
  lesson: any,
  _targetDay: number,
  targetSlotId: string,
) {
  try {
    const request: ScheduleLessonRequest = {
      section_subject_id: lesson.section_subject_id,
      time_slot_id: targetSlotId,
      room_id: lesson.room_id || undefined,
      teacher_id: lesson.teacher_id || undefined,
    };
    await api.updateScheduledLesson(lesson.id, request);

    toast.success(m.academics_schedule_lesson_moved());
    refetchSchedule();
  } catch (err) {
    toast.error(extractErrorMessage(err));
    refetchSchedule();
  }
}

async function handleDeleteLesson(lessonId: string) {
  try {
    await api.deleteScheduledLesson(lessonId);
    toast.success(m.academics_schedule_lesson_deleted());
    refetchSchedule();
  } catch (err) {
    toast.error(extractErrorMessage(err));
  }
}

const isLoading = $derived(
  periodsQuery.isLoading ||
    classesQuery.isLoading ||
    subjectsQuery.isLoading ||
    teachersQuery.isLoading ||
    roomsQuery.isLoading ||
    (Boolean(effectiveActivePeriodId) &&
      (timeSlotsQuery.isLoading ||
        periodScheduleQuery.isLoading ||
        sectionSubjectsQuery.isLoading)),
);

const error = $derived(
  periodsQuery.error ||
    classesQuery.error ||
    subjectsQuery.error ||
    teachersQuery.error ||
    roomsQuery.error ||
    timeSlotsQuery.error ||
    periodScheduleQuery.error ||
    sectionSubjectsQuery.error,
);

function handleDirectPrint() {
  const activePeriod = periods.find(
    (p) => String(p.id) === String(effectiveActivePeriodId),
  );
  const periodTitle = activePeriod
    ? l(activePeriod.name_ar ?? undefined, activePeriod.name) ||
      activePeriod.name
    : "";

  let targetId = "master";
  let targetTitle = m.academics_schedule_master_title();
  let relevantLessons = scheduledLessons;

  if (viewMode === "class" && selectedClassId) {
    const cls = classes.find((c) => String(c.id) === String(selectedClassId));
    targetId = String(selectedClassId);
    targetTitle = cls
      ? m.academics_schedule_print_class({
          name: l(cls.name_ar ?? undefined, cls.name) || cls.name,
        })
      : m.academics_schedule_print_class_fallback();
    relevantLessons = scheduledLessons.filter(
      (l) => String(l.section_id) === String(selectedClassId),
    );
  } else if (viewMode === "teacher" && selectedTeacherId) {
    const teach = teachers.find(
      (t) => String(t.id) === String(selectedTeacherId),
    );
    targetId = String(selectedTeacherId);
    targetTitle = teach
      ? m.academics_schedule_print_teacher({
          name:
            l(teach.full_name_ar ?? undefined, teach.full_name) ||
            teach.full_name,
        })
      : m.academics_schedule_print_teacher_fallback();
    relevantLessons = scheduledLessons.filter(
      (l) => String(l.teacher_id) === String(selectedTeacherId),
    );
  } else if (viewMode === "room" && selectedRoomId) {
    const rm = rooms.find((r) => String(r.id) === String(selectedRoomId));
    targetId = String(selectedRoomId);
    targetTitle = rm
      ? m.academics_schedule_print_room({
          name: l(rm.name_ar ?? undefined, rm.name) || rm.name,
        })
      : m.academics_schedule_print_room_fallback();
    relevantLessons = scheduledLessons.filter(
      (l) => String(l.room_id) === String(selectedRoomId),
    );
  }

  const scheduleRows = timeSlots.map((slot) => {
    const isBreak = Boolean(slot.is_break);
    const slotName =
      l(slot.name_ar ?? undefined, slot.name) || `P${slot.slot_index}`;
    const timeRange = `${slot.start_time?.slice(0, 5) || ""} - ${slot.end_time?.slice(0, 5) || ""}`;
    const rowObj: Record<string, string> = {
      slotName: `${slotName} (${timeRange})`,
      timeRange,
    };

    for (const day of weekdays) {
      const dayVal = String(day.value ?? day.day);
      if (isBreak) {
        rowObj[`day_${dayVal}`] =
          l(slot.name_ar ?? undefined, slot.name) ||
          m.common_onboarding_break_recess();
      } else {
        const lesson = relevantLessons.find(
          (l) =>
            Number(l.day_of_week) === Number(dayVal) &&
            String(l.time_slot_id) === String(slot.id),
        );
        if (lesson) {
          const sub = getSubjectName(lesson.subject_id);
          const teacher =
            viewMode !== "teacher" && lesson.teacher_id
              ? ` (${getTeacherName(lesson.teacher_id)})`
              : "";
          const cls =
            viewMode !== "class" && lesson.section_id
              ? ` - ${getClassName(lesson.section_id)}`
              : "";
          const rm =
            viewMode !== "room" && lesson.room_id
              ? ` [${getRoomName(lesson.room_id)}]`
              : "";
          rowObj[`day_${dayVal}`] = `${sub}${teacher}${cls}${rm}`;
        } else {
          rowObj[`day_${dayVal}`] = "—";
        }
      }
    }

    return rowObj;
  });

  const scheduleContext = {
    schedule: {
      title: targetTitle,
      periodName: periodTitle,
      exportType: viewMode,
      date: formatDate(new Date()),
      totalSlots: timeSlots.length,
      totalDays: weekdays.length,
    },
    weekdays: weekdays.map((d) => ({
      day: d.value ?? d.day,
      label: d.label,
    })),
    timeSlots: timeSlots.map((s) => ({
      id: s.id,
      name: l(s.name_ar ?? undefined, s.name) || `P${s.slot_index}`,
      timeRange: `${s.start_time?.slice(0, 5) || ""} - ${s.end_time?.slice(0, 5) || ""}`,
      isBreak: Boolean(s.is_break),
    })),
    matrix: scheduleRows,
    lessons: relevantLessons.map((l) => ({
      subject: getSubjectName(l.subject_id),
      teacher: getTeacherName(l.teacher_id),
      class: getClassName(l.section_id),
      room: getRoomName(l.room_id),
      day: l.day_of_week,
      slotId: l.time_slot_id,
    })),
  };

  openPrintDialog("schedule", [targetId], [targetTitle], {
    documentType: "SCHEDULE",
    contexts: [scheduleContext],
  });
}
</script>

<div class="h-full w-full flex flex-col items-stretch overflow-hidden font-sans">
  <!-- Top Horizontal Command & Toolbar -->
  <ScheduleToolbar
    activePeriodId={effectiveActivePeriodId}
    {periods}
    bind:viewMode
    bind:selectedClassId
    bind:selectedTeacherId
    bind:selectedRoomId
    bind:selectedSubjectId
    bind:searchTerm
    bind:compactView
    {classes}
    {teachers}
    {rooms}
    {orgUnits}
    {subjects}
    onPeriodChange={(val) => {
      activePeriodId = val;
      refetchSchedule();
    }}
    onAutoScheduleClick={() =>
      goto(
        `${resolve("/(app)/[orgId]/academics/schedule/generator", { orgId })}?periodId=${effectiveActivePeriodId}`,
      )}
    onTimeSlotsClick={() => (isTimeSlotsSheetOpen = true)}
    onExportClick={handleDirectPrint}
    onNewLessonClick={() => openNewLessonSheet()}
  />

  <!-- Main Body: Full-Width Workspace Canvas -->
  <div class="flex flex-1 min-h-0 min-w-0 w-full overflow-hidden">
    <!-- Central Schedule Workspace Canvas -->
    <main class="flex flex-1 min-h-0 min-w-0 flex-col p-3 gap-2 overflow-hidden">
      {#if isLoading}
        <div class="flex flex-1 flex-col overflow-hidden rounded-xl border border-border/60 bg-card">
          <div class="flex h-12 shrink-0 items-center gap-3 border-b border-border/60 px-4">
            <Skeleton class="h-7 w-32 rounded-md" />
            <Skeleton class="h-7 w-56 rounded-md" />
            <Skeleton class="ms-auto h-7 w-40 rounded-md" />
          </div>
          <div class="grid shrink-0 grid-cols-6 border-b border-border/60">
            {#each Array(6) as _, index (index)}
              <Skeleton class="m-2 h-7 rounded-md" />
            {/each}
          </div>
          <div class="flex-1 space-y-3 overflow-hidden p-4">
            {#each Array(5) as _, rowIndex (rowIndex)}
              <div class="grid grid-cols-6 gap-3">
                {#each Array(6) as _, columnIndex (columnIndex)}
                  <Skeleton class="h-20 rounded-lg" />
                {/each}
              </div>
            {/each}
          </div>
        </div>
      {:else if error}
        <ErrorState
          title={m.academics_schedule_load_error()}
          message={error instanceof Error ? error.message : m.shared_ui_error()}
          onRetry={refetchSchedule}
        />
      {:else if !effectiveActivePeriodId}
        <EmptyState
          icon={Calendar}
          title={m.academics_studio_no_periods()}
          description={m.academics_schedule_no_periods_desc()}
          actionLabel={m.academics_studio_manage_periods()}
          onaction={() =>
            goto(resolve("/(app)/[orgId]/settings/periods", { orgId }))}
          class="mx-auto my-auto w-full max-w-xl rounded-xl border border-border/60 bg-card shadow-xs"
        />
      {:else if timeSlots.length === 0}
        <EmptyState
          icon={ClockRotateRight}
          title={m.academics_schedule_no_slots()}
          description={m.academics_schedule_no_slots_desc()}
          actionLabel={m.academics_schedule_config_slots_btn()}
          onaction={() => (isTimeSlotsSheetOpen = true)}
          class="mx-auto my-auto w-full max-w-xl rounded-xl border border-border/60 bg-card shadow-xs"
        />
      {:else if classes.length === 0}
        <EmptyState
          icon={Users}
          title={m.academics_schedule_no_classes()}
          description={m.academics_schedule_no_classes_desc()}
          class="mx-auto my-auto w-full max-w-xl rounded-xl border border-border/60 bg-card shadow-xs"
        />
      {:else}
        <!-- Contextual Header Info Banner (For Teacher Workload / Room Occupancy) -->
        {#if teacherWorkload}
          <div class="flex items-center justify-between px-3 py-2 rounded-lg border border-blue-500/20 bg-blue-500/5 text-xs">
            <div class="flex items-center gap-2">
              <User size={14} class="text-blue-500" />
              <span class="font-bold text-foreground">{getTeacherName(selectedTeacherId)}</span>
            </div>
            <div class="flex items-center gap-4 text-[11px] text-muted-foreground">
              <span>{m.common_schedule_teaching_load()}: <strong class="text-foreground">{teacherWorkload.teachingCount}</strong></span>
              <span>{m.common_schedule_free_periods()}: <strong class="text-emerald-600 dark:text-emerald-400">{teacherWorkload.freeCount}</strong></span>
              <span>{m.common_search_group_rooms()}: <strong class="text-foreground">{teacherWorkload.assignedRooms}</strong></span>
            </div>
          </div>
        {:else if roomOccupancy}
          <div class="flex items-center justify-between px-3 py-2 rounded-lg border border-emerald-500/20 bg-emerald-500/5 text-xs">
            <div class="flex items-center gap-2">
              <Pin size={14} class="text-emerald-500" />
              <span class="font-bold text-foreground">{getRoomName(selectedRoomId)}</span>
              {#if roomOccupancy.capacity}
                <Badge variant="outline" class="h-4 text-[9px]">{m.academics_class_label_capacity()}: {roomOccupancy.capacity}</Badge>
              {/if}
            </div>
            <div class="flex items-center gap-4 text-[11px] text-muted-foreground">
              <span>{m.common_schedule_occupied_periods()}: <strong class="text-foreground">{roomOccupancy.occupiedCount}</strong></span>
              <span>{m.common_schedule_occupancy_rate()}: <strong class="text-primary">{roomOccupancy.occupancyRate}%</strong></span>
            </div>
          </div>
        {/if}

        <!-- Scheduling Grid View / Matrix View -->
        {#if viewMode === "matrix"}
          <ScheduleMatrix
            {timeSlots}
            {classes}
            selectedDay={activeMatrixDay}
            lessons={scheduledLessons}
            {getSubjectName}
            {getTeacherName}
            {getClassName}
            {getRoomName}
            {getSubjectColor}
            onDayChange={(day) => (selectedMatrixDay = day)}
            onLessonClick={openEditLessonSheet}
            onCellClick={(clsId, slotId, day) => {
              selectedClassId = clsId;
              openNewLessonSheet(day, slotId);
            }}
          />
        {:else}
          {@const hasSelection =
            (viewMode === "class" && !!selectedClassId) ||
            (viewMode === "teacher" && !!selectedTeacherId) ||
            (viewMode === "room" && !!selectedRoomId) ||
            (viewMode === "subject" && !!selectedSubjectId)}
          {#if !hasSelection}
            <EmptyState
              icon={viewMode === "class"
                ? Users
                : viewMode === "teacher"
                  ? User
                  : viewMode === "room"
                    ? Pin
                    : BookOpen}
              title={viewMode === "class"
                ? m.academics_studio_select_class()
                : viewMode === "teacher"
                  ? m.academics_studio_select_teacher()
                  : viewMode === "room"
                    ? m.academics_studio_select_room()
                    : m.academics_schedule_select_subject()}
              description={m.academics_schedule_choose_view_item()}
              class="mx-auto my-auto w-full max-w-xl rounded-xl border border-border/60 bg-card shadow-xs"
            />
          {:else if searchedLessons.length === 0}
            <EmptyState
              icon={searchTerm.trim() ? BookOpen : Calendar}
              title={searchTerm.trim()
                ? m.academics_schedule_no_search_results()
                : m.academics_schedule_no_lessons()}
              description={searchTerm.trim()
                ? m.academics_schedule_no_search_results_desc()
                : m.academics_schedule_no_lessons_desc()}
              actionLabel={searchTerm.trim() ? "" : m.academics_schedule_add_first_lesson()}
              onaction={searchTerm.trim() ? undefined : () => openNewLessonSheet()}
              class="mx-auto my-auto w-full max-w-xl rounded-xl border border-border/60 bg-card shadow-xs"
            />
          {:else}
            <TimetableGrid
              {timeSlots}
              {weekdays}
              lessons={searchedLessons}
              {conflictLessonIds}
              {draftLessons}
              {previewDraft}
              {viewMode}
              compact={compactView}
              locale={getLocale()}
              {getSubjectName}
              {getTeacherName}
              {getClassName}
              {getRoomName}
              {getSubjectColor}
              onCellClick={(day, slotId) => openNewLessonSheet(day, slotId)}
              onEditLesson={openEditLessonSheet}
              onLessonClick={openEditLessonSheet}
              onAttendanceClick={openLessonAttendance}
              onDeleteLesson={handleDeleteLesson}
              onMoveLesson={handleMoveLesson}
            />
          {/if}
        {/if}
      {/if}
    </main>
  </div>
</div>

<!-- Lesson Create / Edit Side Sheet -->
<LessonSheet
  bind:open={isLessonSheetOpen}
  {orgId}
  periodId={effectiveActivePeriodId}
  classId={selectedClassId}
  dayOfWeek={activeCellDay}
  timeSlotId={activeCellSlotId}
  lesson={targetLesson}
  {classes}
  {subjects}
  {teachers}
  {rooms}
  {timeSlots}
  onSuccess={refetchSchedule}
/>

<!-- Time Slots Management Sheet -->
<TimeSlotSheet
  bind:open={isTimeSlotsSheetOpen}
  periodId={effectiveActivePeriodId}
  {timeSlots}
  onSuccess={refetchSchedule}
/>

<!-- AI Scheduler Drawer -->
<SolverDrawer
  bind:open={isSolverDrawerOpen}
  periodId={effectiveActivePeriodId}
  bind:previewDraft
  bind:activeJob={activeSolveJob}
  onApply={refetchSchedule}
/>
