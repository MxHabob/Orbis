<!--
  @component ScheduleWorkspace
  Feature component for Timetable and Schedule workspace.
  Supports Class, Teacher, Room, Subject, and Matrix perspectives.
-->
<script lang="ts">
import { goto } from "$app/navigation";
import {
  api,
  extractErrorMessage,
  queryKeys,
  type ScheduleLessonRequest,
  type OrgUnitDto,
} from "$lib/api";
import { getLocale, localizeHref } from "$lib/paraglide/runtime";
import { toast } from "@orbis/ui";
import { createQuery, useQueryClient } from "@tanstack/svelte-query";
import { TimetableGrid } from "@orbis/ui";
import { Button } from "@orbis/ui/button";
import { Skeleton } from "@orbis/ui/skeleton";
import { Badge } from "@orbis/ui/badge";
import { ClockRotateRight, Pin, User } from "@orbis/icons";
import * as m from "$lib/paraglide/messages";

import {
  ScheduleToolbar,
  ScheduleMatrix,
  LessonSheet,
  TimeSlotSheet,
  SolverDrawer,
} from "$lib/features/academics";
import ErrorState from "$lib/shared/components/error-state.svelte";
import { l, formatDate } from "$lib/core/i18n";
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
let selectedMatrixDay = $state(7);

// Dialog & Sheet States
let isLessonSheetOpen = $state(false);
let isTimeSlotsSheetOpen = $state(false);
let isSolverDrawerOpen = $state(false);

// Active Lesson for editing / creating
let targetLesson = $state<any | null>(null);
let activeCellDay = $state(7);
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

const timeSlots = $derived(timeSlotsQuery.data ?? []);
const allLessons = $derived(periodScheduleQuery.data ?? []);

// Weekdays definition (Saturday - Wednesday / Thursday for regional setup)
const weekdays = $derived([
  { value: 7, day: 7, label: m.common_day_saturday() },
  { value: 1, day: 1, label: m.common_day_sunday() },
  { value: 2, day: 2, label: m.common_day_monday() },
  { value: 3, day: 3, label: m.common_day_tuesday() },
  { value: 4, day: 4, label: m.common_day_wednesday() },
  { value: 5, day: 5, label: m.common_day_thursday() },
]);

// Conflict Detection
const conflictLessonIds = $derived.by(() => {
  const ids = new Set<string>();
  const teacherOccupancy = new Map<string, string>();
  const roomOccupancy = new Map<string, string>();
  const classOccupancy = new Map<string, string>();

  for (const lesson of allLessons) {
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
    return allLessons.filter(
      (l) => String(l.section_id) === String(selectedClassId),
    );
  }

  if (viewMode === "teacher") {
    if (!selectedTeacherId) return [];
    return allLessons.filter(
      (l) => String(l.teacher_id) === String(selectedTeacherId),
    );
  }

  if (viewMode === "room") {
    if (!selectedRoomId) return [];
    return allLessons.filter(
      (l) => String(l.room_id) === String(selectedRoomId),
    );
  }

  if (viewMode === "subject") {
    if (!selectedSubjectId) return [];
    return allLessons.filter(
      (l) => String(l.subject_id) === String(selectedSubjectId),
    );
  }

  return allLessons;
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
function getSubjectName(subjectId: string): string {
  const sub = subjects.find((s) => String(s.id) === String(subjectId));
  return sub ? sub.name_ar || sub.name : m.common_search_group_subjects();
}

function getTeacherName(teacherId: string): string {
  const teach = teachers.find((t) => String(t.id) === String(teacherId));
  return teach ? teach.name_ar || teach.full_name : "";
}

function getClassName(clsId: string): string {
  const cls = classes.find((c) => String(c.id) === String(clsId));
  return cls ? cls.name_ar || cls.name : "";
}

function getRoomName(roomId?: string): string {
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
  const teacherLessons = allLessons.filter(
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
  const roomLessons = allLessons.filter(
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
  activeCellDay = day || 1;
  activeCellSlotId =
    slotId || (timeSlots[0]?.id ? String(timeSlots[0].id) : "");
  isLessonSheetOpen = true;
}

function openEditLessonSheet(lesson: any) {
  targetLesson = lesson;
  activeCellDay = Number(lesson?.day_of_week) || 1;
  activeCellSlotId = String(lesson?.time_slot_id || "");
  isLessonSheetOpen = true;
}

async function handleMoveLesson(
  lesson: any,
  _targetDay: number,
  targetSlotId: string,
) {
  try {
    if (lesson.id) {
      await api.deleteScheduledLesson(lesson.id);
    }
    const request: ScheduleLessonRequest = {
      section_subject_id: lesson.section_subject_id,
      time_slot_id: targetSlotId,
      room_id: lesson.room_id || undefined,
      teacher_id: lesson.teacher_id || undefined,
    };
    await api.scheduleLesson(request);

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
    timeSlotsQuery.isLoading ||
    periodScheduleQuery.isLoading,
);

const error = $derived(
  periodsQuery.error ||
    classesQuery.error ||
    timeSlotsQuery.error ||
    periodScheduleQuery.error,
);

function handleDirectPrint() {
  const activePeriod = periods.find(
    (p) => String(p.id) === String(activePeriodId),
  );
  const periodTitle = activePeriod
    ? l(activePeriod.name_ar, activePeriod.name) || activePeriod.name
    : "";

  let targetId = "master";
  let targetTitle = m.academics_schedule_master_title();
  let relevantLessons = allLessons;

  if (viewMode === "class" && selectedClassId) {
    const cls = classes.find((c) => String(c.id) === String(selectedClassId));
    targetId = String(selectedClassId);
    targetTitle = cls
      ? m.academics_schedule_print_class({ name: l(cls.name_ar, cls.name) })
      : m.academics_schedule_print_class_fallback();
    relevantLessons = allLessons.filter(
      (l) => String(l.section_id) === String(selectedClassId),
    );
  } else if (viewMode === "teacher" && selectedTeacherId) {
    const teach = teachers.find(
      (t) => String(t.id) === String(selectedTeacherId),
    );
    targetId = String(selectedTeacherId);
    targetTitle = teach
      ? m.academics_schedule_print_teacher({
          name: l(teach.full_name_ar, teach.full_name),
        })
      : m.academics_schedule_print_teacher_fallback();
    relevantLessons = allLessons.filter(
      (l) => String(l.teacher_id) === String(selectedTeacherId),
    );
  } else if (viewMode === "room" && selectedRoomId) {
    const rm = rooms.find((r) => String(r.id) === String(selectedRoomId));
    targetId = String(selectedRoomId);
    targetTitle = rm
      ? m.academics_schedule_print_room({ name: l(rm.name_ar, rm.name) })
      : m.academics_schedule_print_room_fallback();
    relevantLessons = allLessons.filter(
      (l) => String(l.room_id) === String(selectedRoomId),
    );
  }

  const scheduleRows = timeSlots.map((slot) => {
    const isBreak = Boolean(slot.is_break);
    const slotName = l(slot.name_ar, slot.name) || `P${slot.slot_number}`;
    const timeRange = `${slot.start_time?.slice(0, 5) || ""} - ${slot.end_time?.slice(0, 5) || ""}`;
    const rowObj: Record<string, string> = {
      slotName: `${slotName} (${timeRange})`,
      timeRange,
    };

    for (const day of weekdays) {
      const dayVal = String(day.value ?? day.day);
      if (isBreak) {
        rowObj[`day_${dayVal}`] =
          l(slot.name_ar, slot.name) || m.common_onboarding_break_recess();
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
      name: l(s.name_ar, s.name) || `P${s.slot_number}`,
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
    onAutoScheduleClick={() => goto(localizeHref(`/${orgId}/academics/schedule/generator?periodId=${effectiveActivePeriodId}`))}
    onTimeSlotsClick={() => (isTimeSlotsSheetOpen = true)}
    onExportClick={handleDirectPrint}
    onNewLessonClick={() => openNewLessonSheet()}
  />

  <!-- Main Body: Full-Width Workspace Canvas -->
  <div class="flex flex-1 min-h-0 min-w-0 w-full overflow-hidden">
    <!-- Central Schedule Workspace Canvas -->
    <main class="flex flex-1 min-h-0 min-w-0 flex-col p-3 gap-2 overflow-hidden">
      {#if isLoading}
        <div class="flex-1 w-full flex flex-col bg-card border border-border/60 rounded-xl overflow-hidden p-4 gap-3">
          <Skeleton class="h-10 w-full" />
          <Skeleton class="flex-1 w-full" />
        </div>
      {:else if error}
        <ErrorState
          title={m.academics_schedule_load_error()}
          message={error instanceof Error ? error.message : m.shared_ui_error()}
          onRetry={refetchSchedule}
        />
      {:else if timeSlots.length === 0}
        <div class="flex-1 flex flex-col items-center justify-center p-12 border border-dashed border-border/60 bg-card rounded-2xl text-center gap-4 max-w-md mx-auto my-auto">
          <div class="p-3.5 bg-primary/10 rounded-full text-primary">
            <ClockRotateRight size={28} />
          </div>
          <div class="flex flex-col gap-1">
            <h3 class="font-bold text-sm text-foreground">
              {m.academics_schedule_no_slots()}
            </h3>
            <p class="text-xs text-muted-foreground leading-relaxed">
              {m.academics_schedule_no_slots_desc()}
            </p>
          </div>
          <Button size="sm" onclick={() => (isTimeSlotsSheetOpen = true)} class="gap-1.5 text-xs font-semibold">
            <ClockRotateRight size={13} />
            <span>{m.academics_schedule_config_slots_btn()}</span>
          </Button>
        </div>
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
            selectedDay={selectedMatrixDay}
            lessons={allLessons}
            {getSubjectName}
            {getTeacherName}
            {getClassName}
            {getRoomName}
            {getSubjectColor}
            onLessonClick={openEditLessonSheet}
            onCellClick={(clsId, slotId, day) => {
              selectedClassId = clsId;
              openNewLessonSheet(day, slotId);
            }}
          />
        {:else}
          <TimetableGrid
            {timeSlots}
            {weekdays}
            lessons={filteredLessons}
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
            onDeleteLesson={handleDeleteLesson}
            onMoveLesson={handleMoveLesson}
          />
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
