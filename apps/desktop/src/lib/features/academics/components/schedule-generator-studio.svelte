<script lang="ts">
import { goto } from "$app/navigation";
import { resolve } from "$app/paths";
import { page } from "$app/state";
import { formatDate, getWeekdayOptions } from "$lib/core/i18n";
import {
  api,
  extractErrorMessage,
  isDesktopApp,
  type SectionSubjectDto,
  type TimetableProblem,
  type TimetableSolveStatusDto,
} from "$lib/api";
import * as m from "$lib/paraglide/messages";
import { createQuery, useQueryClient } from "@tanstack/svelte-query";
import { toast } from "@orbis/ui";
import { Badge } from "@orbis/ui/badge";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui/select";
import {
  AcademicCap,
  AdjustmentsHorizontal,
  BookOpen,
  Calendar,
  Check,
  ClockRotateRight,
  Eye,
  InfoCircle,
  Sparkles,
  User,
} from "@orbis/icons";
import TimeSlotSheet from "./time-slot-sheet.svelte";

interface Props {
  orgId: string;
}

type StudioTab = "overview" | "curriculum" | "schedule" | "generator";

let { orgId }: Props = $props();
const queryClient = useQueryClient();

let activeTab = $state<StudioTab>("overview");
let activeYearId = $state(page.url.searchParams.get("yearId") ?? "");
let activePeriodId = $state(page.url.searchParams.get("periodId") ?? "");
let selectedClassId = $state("all");
let selectedSubjectIds = $state<string[]>([]);
let showClassesLane = $state(true);
let showTeachersLane = $state(true);
let showCurriculumLane = $state(true);
let isTimeSlotSheetOpen = $state(false);
let isSolving = $state(false);
let isApplying = $state(false);
let activeJobId = $state("");
let activeJobApplied = $state(false);

const yearsQuery = createQuery(() => ({
  queryKey: ["academic-years", orgId],
  queryFn: () => api.listacademicCalendars(),
  enabled: !!orgId,
  staleTime: 5 * 60 * 1000,
}));

const periodsQuery = createQuery(() => ({
  queryKey: ["periods", orgId],
  queryFn: () => api.listAcademicPeriods(),
  enabled: !!orgId,
  staleTime: 5 * 60 * 1000,
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

const sidecarQuery = createQuery(() => ({
  queryKey: ["solver-sidecar-status", orgId],
  queryFn: () => api.getSolverSidecarStatus(),
  enabled: !!orgId && isDesktopApp,
  refetchInterval: 10_000,
}));

const years = $derived(yearsQuery.data ?? []);
const periods = $derived(periodsQuery.data ?? []);
const classes = $derived(classesQuery.data ?? []);
const subjects = $derived(subjectsQuery.data ?? []);
const teachers = $derived(teachersQuery.data?.items ?? []);
const rooms = $derived(roomsQuery.data ?? []);

const effectiveYearId = $derived(
  (years.some((year) => year.id === activeYearId) ? activeYearId : "") ||
    periods.find((period) => period.id === activePeriodId)?.year_id ||
    years.find((year) => year.is_active)?.id ||
    years[0]?.id ||
    "",
);
const periodsForYear = $derived(
  periods.filter((period) => period.year_id === effectiveYearId),
);
const effectiveActivePeriodId = $derived(
  periodsForYear.some((period) => period.id === activePeriodId)
    ? activePeriodId
    : (periodsForYear.find((period) => period.status === "active")?.id ??
        periodsForYear[0]?.id ??
        ""),
);
const activeYear = $derived(years.find((year) => year.id === effectiveYearId));
const activePeriod = $derived(
  periods.find((period) => period.id === effectiveActivePeriodId),
);

const timeSlotsQuery = createQuery(() => ({
  queryKey: ["time-slots", orgId, effectiveActivePeriodId],
  queryFn: () => api.listTimeSlots(effectiveActivePeriodId),
  enabled: !!orgId && !!effectiveActivePeriodId,
  staleTime: 60 * 1000,
}));

const lessonsQuery = createQuery(() => ({
  queryKey: ["period-schedule", orgId, effectiveActivePeriodId],
  queryFn: () =>
    api.listScheduledLessons({ periodId: effectiveActivePeriodId }),
  enabled: !!orgId && !!effectiveActivePeriodId,
  staleTime: 30 * 1000,
}));

const sectionSubjectsQuery = createQuery(() => ({
  queryKey: [
    "all-class-subjects-matrix",
    orgId,
    effectiveActivePeriodId,
    classes.map((section) => section.id).join(","),
  ],
  queryFn: async () => {
    if (!effectiveActivePeriodId || classes.length === 0) return [];
    const result = await Promise.all(
      classes.map((section) =>
        api.listSectionSubjects(section.id, effectiveActivePeriodId),
      ),
    );
    return result.flat();
  },
  enabled: !!orgId && !!effectiveActivePeriodId && classes.length > 0,
  staleTime: 30 * 1000,
}));

const timeSlots = $derived(timeSlotsQuery.data ?? []);
const regularSlots = $derived(timeSlots.filter((slot) => !slot.is_break));
const lessons = $derived(lessonsQuery.data ?? []);
const sectionSubjects = $derived(
  (sectionSubjectsQuery.data ?? []).filter((item) => item.is_active),
);
const classesWithCurriculum = $derived(
  classes.filter((section) =>
    sectionSubjects.some((item) => item.section_id === section.id),
  ),
);
const subjectById = $derived(
  new Map(subjects.map((subject) => [subject.id, subject])),
);
const sectionSubjectById = $derived(
  new Map(sectionSubjects.map((item) => [item.id, item])),
);
const roomIds = $derived(new Set(rooms.map((room) => room.id)));
const timeSlotById = $derived(
  new Map(timeSlots.map((slot) => [slot.id, slot])),
);
const totalWeeklyQuota = $derived(
  sectionSubjects.reduce(
    (total, item) => total + Math.max(0, Number(item.weekly_hours) || 0),
    0,
  ),
);
const pendingLessonCount = $derived(
  sectionSubjects.reduce((total, item) => {
    const scheduled = lessons.filter(
      (lesson) => lesson.section_subject_id === item.id,
    ).length;
    return total + Math.max(0, (Number(item.weekly_hours) || 0) - scheduled);
  }, 0),
);
const overAssignedSubjects = $derived(
  sectionSubjects.filter((item) => {
    const scheduled = lessons.filter(
      (lesson) => lesson.section_subject_id === item.id,
    ).length;
    return scheduled > (Number(item.weekly_hours) || 0);
  }),
);
const teachersById = $derived(
  new Map(teachers.map((teacher) => [teacher.id, teacher])),
);
const missingTeachers = $derived(
  sectionSubjects.filter((item) => {
    const pending =
      (Number(item.weekly_hours) || 0) -
      lessons.filter((lesson) => lesson.section_subject_id === item.id).length;
    return (
      pending > 0 &&
      (!item.instructor_id || !teachersById.has(item.instructor_id))
    );
  }),
);
const invalidPinnedLessons = $derived(
  lessons.filter((lesson) => {
    const item = sectionSubjectById.get(lesson.section_subject_id);
    const slot = timeSlotById.get(lesson.time_slot_id);
    return (
      !item ||
      !slot ||
      slot.is_break ||
      !(lesson.teacher_id || item.instructor_id) ||
      (lesson.room_id != null && !roomIds.has(lesson.room_id))
    );
  }),
);
const capacityIssues = $derived.by(() => {
  const quotaByClass = new Map<string, number>();
  for (const item of sectionSubjects) {
    quotaByClass.set(
      item.section_id,
      (quotaByClass.get(item.section_id) ?? 0) +
        Math.max(0, Number(item.weekly_hours) || 0),
    );
  }
  return [...quotaByClass.entries()]
    .filter(([, weeklyQuota]) => weeklyQuota > regularSlots.length)
    .map(([sectionId]) => sectionId);
});

const isPeriodReadOnly = $derived(
  !!activePeriod &&
    (activePeriod.is_locked ||
      activePeriod.status === "closed" ||
      activePeriod.status === "archived"),
);
const hasSetupData = $derived(
  yearsQuery.isSuccess &&
    periodsQuery.isSuccess &&
    classesQuery.isSuccess &&
    subjectsQuery.isSuccess &&
    teachersQuery.isSuccess &&
    roomsQuery.isSuccess,
);
const isLoading = $derived(
  yearsQuery.isLoading ||
    periodsQuery.isLoading ||
    classesQuery.isLoading ||
    subjectsQuery.isLoading ||
    teachersQuery.isLoading ||
    roomsQuery.isLoading ||
    (!!effectiveActivePeriodId &&
      (timeSlotsQuery.isLoading ||
        lessonsQuery.isLoading ||
        sectionSubjectsQuery.isLoading)),
);
const readinessIssues = $derived.by(() => {
  const issues: string[] = [];
  if (!activePeriod) return issues;
  if (regularSlots.length === 0) {
    issues.push(m.academics_studio_no_time_slots());
  }
  if (sectionSubjects.length === 0) {
    issues.push(m.academics_studio_no_curriculum());
  }
  if (missingTeachers.length > 0) {
    issues.push(
      m.academics_studio_missing_teachers({
        count: missingTeachers.length,
      }),
    );
  }
  if (overAssignedSubjects.length > 0) {
    issues.push(m.academics_studio_over_assigned());
  }
  if (invalidPinnedLessons.length > 0) {
    issues.push(
      m.academics_studio_invalid_pinned_lessons({
        count: invalidPinnedLessons.length,
      }),
    );
  }
  for (const sectionId of capacityIssues) {
    issues.push(
      m.academics_studio_capacity_exceeded({
        name: getClassName(sectionId),
      }),
    );
  }
  if (isPeriodReadOnly) {
    issues.push(m.academics_studio_locked_period());
  }
  return issues;
});
const solveStatusQuery = createQuery(() => ({
  queryKey: ["timetable-solve-status", orgId, activeJobId],
  queryFn: () => api.getTimetableSolveStatus(activeJobId),
  enabled: isDesktopApp && !!activeJobId,
  refetchInterval: (query) => {
    const status = query.state.data?.status;
    return status === "completed" ||
      status === "failed" ||
      status === "cancelled"
      ? false
      : 2000;
  },
}));
const solveStatus = $derived(
  (solveStatusQuery.data ?? null) as TimetableSolveStatusDto | null,
);
const hasFeasibleSolution = $derived(
  /^0hard(?:\/|$)/.test(solveStatus?.score ?? ""),
);
const solutionAssignments = $derived(
  solveStatus?.best_solution?.assignments ?? [],
);
const unplacedAssignmentCount = $derived(
  solutionAssignments.filter((assignment) => !assignment.time_slot_id).length,
);
const newSolutionAssignments = $derived(
  solutionAssignments.filter((assignment) => {
    if (!assignment.time_slot_id) return false;
    return !lessons.some(
      (lesson) =>
        lesson.section_subject_id === assignment.section_subject_id &&
        lesson.time_slot_id === assignment.time_slot_id,
    );
  }),
);
const isSolveRunning = $derived(
  solveStatus?.status === "pending" || solveStatus?.status === "running",
);
const canGenerate = $derived(
  isDesktopApp &&
    sidecarQuery.data?.is_healthy === true &&
    hasSetupData &&
    !isLoading &&
    !yearsQuery.error &&
    !periodsQuery.error &&
    !classesQuery.error &&
    !subjectsQuery.error &&
    !teachersQuery.error &&
    !roomsQuery.error &&
    !timeSlotsQuery.error &&
    !lessonsQuery.error &&
    !sectionSubjectsQuery.error &&
    !isSolveRunning &&
    !!activePeriod &&
    !isPeriodReadOnly &&
    readinessIssues.length === 0 &&
    pendingLessonCount > 0 &&
    !isSolving,
);
const canApplySolution = $derived(
  solveStatus?.status === "completed" &&
    hasFeasibleSolution &&
    unplacedAssignmentCount === 0 &&
    newSolutionAssignments.length > 0 &&
    !activeJobApplied &&
    !isPeriodReadOnly,
);
const slotIndexes = $derived(
  [...new Set(timeSlots.map((slot) => slot.slot_index))].sort(
    (left, right) => left - right,
  ),
);
const weekdays = $derived.by(() => {
  const order = new Map(
    getWeekdayOptions().map((weekday, index) => [weekday.value, index]),
  );
  return [...new Set(timeSlots.map((slot) => slot.day_of_week))]
    .filter((day) => Number.isInteger(day) && day >= 1 && day <= 7)
    .sort((left, right) => (order.get(left) ?? 0) - (order.get(right) ?? 0));
});
const teacherWorkloads = $derived.by(() => {
  const loads = new Map<string, number>();
  for (const item of sectionSubjects) {
    if (!item.instructor_id) continue;
    loads.set(
      item.instructor_id,
      (loads.get(item.instructor_id) ?? 0) +
        Math.max(0, Number(item.weekly_hours) || 0),
    );
  }
  return loads;
});
const selectedScheduleClassId = $derived(
  (selectedClassId !== "all" ? selectedClassId : "") ||
    classesWithCurriculum[0]?.id ||
    "",
);
const visibleDays = $derived(weekdays);
const visibleSubjects = $derived(
  subjects.filter(
    (subject) =>
      selectedSubjectIds.length === 0 ||
      selectedSubjectIds.includes(subject.id),
  ),
);
const visibleSectionSubjects = $derived(
  sectionSubjects.filter((item) =>
    visibleSubjects.some((subject) => subject.id === item.subject_id),
  ),
);
const visiblePlannerLessons = $derived(
  lessons.filter((lesson) => {
    const item = sectionSubjectById.get(lesson.section_subject_id);
    return (
      item &&
      (selectedClassId === "all" ||
        item.section_id === selectedScheduleClassId) &&
      visibleSubjects.some((subject) => subject.id === item.subject_id)
    );
  }),
);
const visiblePlannerTeachers = $derived(
  [
    ...new Set(
      visibleSectionSubjects
        .map((item) => item.instructor_id)
        .filter((id): id is string => !!id),
    ),
  ]
    .map((id) => teachersById.get(id))
    .filter((teacher) => !!teacher),
);

function getClassName(sectionId: string): string {
  const section = classes.find((item) => item.id === sectionId);
  return section?.name_ar || section?.name || section?.code || "";
}

function getSubjectName(subjectId: string): string {
  const subject = subjectById.get(subjectId);
  return (
    subject?.name_ar || subject?.name || m.academics_studio_unknown_subject()
  );
}

function getTeacherName(teacherId?: string | null): string {
  if (!teacherId) return m.academics_studio_unassigned();
  const teacher = teachersById.get(teacherId);
  return (
    teacher?.full_name_ar ||
    teacher?.full_name ||
    m.academics_studio_unassigned()
  );
}

function getRoomName(roomId?: string | null): string {
  if (!roomId) return "";
  const room = rooms.find((item) => item.id === roomId);
  return room?.name_ar || room?.name || "";
}

function getDayName(day: number): string {
  return (
    getWeekdayOptions().find((weekday) => weekday.value === day)?.label ??
    String(day)
  );
}

function lessonsForDay(day: number) {
  return visiblePlannerLessons
    .flatMap((lesson) => {
      const slot = timeSlotById.get(lesson.time_slot_id);
      const item = sectionSubjectById.get(lesson.section_subject_id);
      if (!slot || !item || slot.day_of_week !== day) return [];
      return [{ lesson, slot, item }];
    })
    .sort((left, right) => left.slot.slot_index - right.slot.slot_index);
}

function curriculumForDay(day: number) {
  const dayLessons = lessonsForDay(day);
  return visibleSectionSubjects.filter((item) =>
    dayLessons.some((entry) => entry.item.id === item.id),
  );
}

function teacherLessonsForDay(day: number, teacherId: string) {
  return lessonsForDay(day).filter(
    ({ lesson, item }) =>
      (lesson.teacher_id || item.instructor_id) === teacherId,
  );
}

function toggleSubjectFilter(subjectId: string) {
  const current =
    selectedSubjectIds.length === 0
      ? subjects.map((subject) => subject.id)
      : selectedSubjectIds;
  const next = current.includes(subjectId)
    ? current.filter((id) => id !== subjectId)
    : [...current, subjectId];
  selectedSubjectIds = next.length === subjects.length ? [] : next;
}

function dateRange(start: string, end: string): string {
  return `${formatDate(start, "date")} – ${formatDate(end, "date")}`;
}

function handleYearChange(value: string | undefined) {
  activeYearId = value ?? "";
  selectedClassId = "all";
  const yearPeriods = periods.filter(
    (period) => period.year_id === activeYearId,
  );
  activePeriodId =
    yearPeriods.find((period) => period.status === "active")?.id ??
    yearPeriods[0]?.id ??
    "";
  activeJobId = "";
  activeJobApplied = false;
}

function handlePeriodChange(value: string | undefined) {
  activePeriodId = value ?? "";
  selectedClassId = "all";
  activeJobId = "";
  activeJobApplied = false;
}

async function handleQuotaChange(item: SectionSubjectDto, value: string) {
  const weeklyHours = Number(value);
  if (!Number.isInteger(weeklyHours) || weeklyHours < 0 || weeklyHours > 60) {
    toast.error(m.academics_studio_invalid_weekly_hours());
    await queryClient.invalidateQueries({
      queryKey: ["all-class-subjects-matrix", orgId, effectiveActivePeriodId],
    });
    return;
  }

  try {
    await api.updateSectionSubject(item.id, { weekly_hours: weeklyHours });
    toast.success(m.academics_studio_weekly_hours());
    await queryClient.invalidateQueries({
      queryKey: ["all-class-subjects-matrix", orgId],
    });
  } catch (error) {
    toast.error(extractErrorMessage(error));
  }
}

async function handleTeacherChange(
  item: SectionSubjectDto,
  value: string | undefined,
) {
  try {
    await api.updateSectionSubject(item.id, {
      instructor_id: value === "unassigned" ? null : (value ?? null),
    });
    toast.success(m.academics_studio_assigned_teacher());
    await queryClient.invalidateQueries({
      queryKey: ["all-class-subjects-matrix", orgId],
    });
  } catch (error) {
    toast.error(extractErrorMessage(error));
  }
}

function buildTimetableProblem(): TimetableProblem {
  const jobId = crypto.randomUUID();
  const hourAssignments: TimetableProblem["assignments"] = [];
  for (const item of sectionSubjects) {
    const scheduled = lessons.filter(
      (lesson) => lesson.section_subject_id === item.id,
    ).length;
    const outstanding = Math.max(
      0,
      (Number(item.weekly_hours) || 0) - scheduled,
    );
    for (let hourIndex = 0; hourIndex < outstanding; hourIndex += 1) {
      hourAssignments.push({
        id: `${item.id}-${hourIndex}`,
        section_subject_id: item.id,
        section_id: item.section_id,
        subject_id: item.subject_id,
        teacher_id: item.instructor_id ?? "",
        hour_index: hourIndex,
      });
    }
  }

  const pinnedLessons = lessons.flatMap((lesson) => {
    const item = sectionSubjectById.get(lesson.section_subject_id);
    const slot = timeSlotById.get(lesson.time_slot_id);
    const teacherId = lesson.teacher_id || item?.instructor_id;
    if (!item || !slot || slot.is_break || !teacherId) return [];
    return [
      {
        id: lesson.id,
        section_id: item.section_id,
        teacher_id: teacherId,
        subject_id: item.subject_id,
        section_subject_id: item.id,
        time_slot_id: slot.id,
        day_of_week: slot.day_of_week,
        room_id: lesson.room_id ?? null,
      },
    ];
  });

  return {
    job_id: jobId,
    org_id: orgId,
    period_id: effectiveActivePeriodId,
    mode: pinnedLessons.length > 0 ? "incremental" : "full",
    time_slots: timeSlots.map((slot) => ({
      id: slot.id,
      slot_number: slot.slot_index,
      start_time: slot.start_time.slice(0, 5),
      end_time: slot.end_time.slice(0, 5),
      is_break: slot.is_break,
      applicable_days: [slot.day_of_week],
    })),
    rooms: rooms.map((room) => ({
      id: room.id,
      name: room.name_ar || room.name,
      capacity: room.capacity,
      room_type: room.room_type,
    })),
    assignments: hourAssignments,
    pinned_lessons: pinnedLessons,
    unavailable_slots: [],
  };
}

async function handleGenerate() {
  if (!canGenerate) return;
  isSolving = true;
  activeJobId = "";
  activeJobApplied = false;
  activeTab = "generator";

  try {
    const problem = buildTimetableProblem();
    await api.submitTimetable(problem);
    activeJobId = problem.job_id;
    toast.success(m.academics_schedule_solver_started());
    await queryClient.invalidateQueries({
      queryKey: ["solver-sidecar-status", orgId],
    });
  } catch (error) {
    toast.error(extractErrorMessage(error));
  } finally {
    isSolving = false;
  }
}

async function handleApplySolution() {
  if (!canApplySolution) return;
  isApplying = true;
  try {
    const requests = newSolutionAssignments.flatMap((assignment) => {
      if (!assignment.time_slot_id) return [];
      return [
        {
          section_subject_id: assignment.section_subject_id,
          time_slot_id: assignment.time_slot_id,
          teacher_id: assignment.teacher_id || undefined,
          room_id: assignment.room_id || undefined,
        },
      ];
    });
    await api.bulkScheduleLessons({ lessons: requests });
    activeJobApplied = true;
    await Promise.all([
      queryClient.invalidateQueries({
        queryKey: ["period-schedule", orgId, effectiveActivePeriodId],
      }),
      queryClient.invalidateQueries({
        queryKey: ["class-schedule"],
      }),
    ]);
    toast.success(m.academics_schedule_solver_applied());
    activeTab = "schedule";
  } catch (error) {
    toast.error(extractErrorMessage(error));
  } finally {
    isApplying = false;
  }
}

function openSchedule() {
  goto(resolve("/(app)/[orgId]/academics/schedule", { orgId }));
}

function openYearSettings() {
  goto(resolve("/(app)/[orgId]/settings/academic-calendars", { orgId }));
}

function openPeriodSettings() {
  goto(resolve("/(app)/[orgId]/settings/periods", { orgId }));
}

const pageError = $derived(
  yearsQuery.error ||
    periodsQuery.error ||
    classesQuery.error ||
    subjectsQuery.error ||
    teachersQuery.error ||
    roomsQuery.error ||
    timeSlotsQuery.error ||
    lessonsQuery.error ||
    sectionSubjectsQuery.error ||
    solveStatusQuery.error,
);
</script>

<div class="flex h-full w-full flex-col overflow-hidden bg-background text-start">
  <header class="flex shrink-0 flex-wrap items-center justify-between gap-3 border-b border-border/60 bg-card px-5 py-2.5">
    <div class="flex min-w-0 items-center gap-3">
      <button type="button" class="text-xs text-muted-foreground hover:text-foreground" onclick={openSchedule}>
        {m.academics_studio_back()}
      </button>
      <span class="text-muted-foreground/50">/</span>
      <h1 class="truncate text-base font-semibold text-foreground">{m.academics_planning_studio_title()}</h1>
    </div>
    <div class="flex flex-wrap items-center gap-2">
      <Badge variant="outline" class="max-w-52 truncate text-[10px]">
        {activePeriod?.name_ar || activePeriod?.name || m.academics_studio_select_period()}
      </Badge>
      {#if activeTab === "overview"}
        <Badge variant="secondary" class="text-[10px]">{m.academics_studio_weekly_template()}</Badge>
      {/if}
      <Button variant="outline" size="sm" class="h-8 gap-1.5 text-xs" onclick={openSchedule}>
        <Eye size={13} />
        {m.academics_studio_preview()}
      </Button>
      <Button
        size="sm"
        class="h-8 gap-1.5 text-xs"
        disabled={!canGenerate}
        onclick={handleGenerate}
      >
        <Sparkles size={13} />
        {m.academics_schedule_generate()}
      </Button>
    </div>
  </header>

  <div class="flex min-h-0 flex-1 overflow-hidden">
    <aside class="flex w-64 shrink-0 flex-col overflow-y-auto border-e border-border/60 bg-card">
      <details open class="border-b border-border/50">
        <summary class="cursor-pointer px-4 py-3 text-xs font-semibold text-foreground">
          {m.academics_studio_data_filters()}
        </summary>
        <div class="space-y-3 px-4 pb-4">
          <label class="block space-y-1.5">
            <span class="text-[11px] text-muted-foreground">{m.academics_studio_academic_year()}</span>
            <Select type="single" value={effectiveYearId} onValueChange={handleYearChange}>
              <SelectTrigger class="h-9 w-full bg-background text-xs">
                <span class="truncate">{activeYear?.name_ar || activeYear?.name || m.academics_studio_manage_years()}</span>
              </SelectTrigger>
              <SelectContent class="text-start">
                {#each years as year (year.id)}
                  <SelectItem value={year.id} class="text-xs">{year.name_ar || year.name}</SelectItem>
                {/each}
              </SelectContent>
            </Select>
          </label>
          <label class="block space-y-1.5">
            <span class="text-[11px] text-muted-foreground">{m.academics_studio_select_period()}</span>
            <Select type="single" value={effectiveActivePeriodId} onValueChange={handlePeriodChange}>
              <SelectTrigger class="h-9 w-full bg-background text-xs">
                <span class="truncate">{activePeriod?.name_ar || activePeriod?.name || m.academics_studio_select_period()}</span>
              </SelectTrigger>
              <SelectContent class="text-start">
                {#each periodsForYear as period (period.id)}
                  <SelectItem value={period.id} class="text-xs">{period.name_ar || period.name}</SelectItem>
                {/each}
              </SelectContent>
            </Select>
          </label>
          <label class="block space-y-1.5">
            <span class="text-[11px] text-muted-foreground">{m.academics_studio_classes()}</span>
            <Select
              type="single"
              value={selectedClassId}
              onValueChange={(value) => (selectedClassId = value ?? "all")}
            >
              <SelectTrigger class="h-9 w-full bg-background text-xs">
                <span class="truncate">
                  {selectedClassId !== "all" && selectedScheduleClassId
                    ? getClassName(selectedScheduleClassId)
                    : m.academics_studio_all_sections()}
                </span>
              </SelectTrigger>
              <SelectContent class="text-start">
                <SelectItem value="all" class="text-xs">{m.academics_studio_all_sections()}</SelectItem>
                {#each classesWithCurriculum as section (section.id)}
                  <SelectItem value={section.id} class="text-xs">{getClassName(section.id)}</SelectItem>
                {/each}
              </SelectContent>
            </Select>
          </label>
          <div class="flex gap-2">
            <Button variant="outline" size="sm" class="h-8 flex-1 gap-1 text-[10px]" onclick={openYearSettings}>
              <Calendar size={12} />
              {m.academics_studio_manage_years()}
            </Button>
            <Button variant="outline" size="sm" class="h-8 flex-1 gap-1 text-[10px]" onclick={openPeriodSettings}>
              <AdjustmentsHorizontal size={12} />
              {m.academics_studio_manage_periods()}
            </Button>
          </div>
        </div>
      </details>

      <details class="border-b border-border/50">
        <summary class="cursor-pointer px-4 py-3 text-xs font-semibold text-foreground">
          {m.academics_studio_annual_periods()}
        </summary>
        <div class="space-y-1 px-3 pb-3">
          {#each periodsForYear as period (period.id)}
            <button
              type="button"
              class="flex w-full items-start justify-between gap-2 rounded-md px-2 py-2 text-start hover:bg-muted/40 {period.id === effectiveActivePeriodId ? 'bg-primary/5' : ''}"
              onclick={() => handlePeriodChange(period.id)}
            >
              <span class="min-w-0">
                <span class="block truncate text-[11px] font-medium text-foreground">{period.name_ar || period.name}</span>
                <span class="mt-0.5 block text-[9px] text-muted-foreground">{dateRange(period.start_date, period.end_date)}</span>
              </span>
              {#if period.is_locked}
                <Badge variant="outline" class="shrink-0 px-1.5 py-0 text-[9px]">{m.academics_studio_locked_period()}</Badge>
              {/if}
            </button>
          {/each}
        </div>
      </details>

      <details open class="border-b border-border/50">
        <summary class="cursor-pointer px-4 py-3 text-xs font-semibold text-foreground">
          {m.academics_studio_subject_filter()}
        </summary>
        <div class="space-y-1 px-4 pb-3">
          {#each subjects as subject (subject.id)}
            <label class="flex cursor-pointer items-center gap-2 rounded-md px-1 py-1.5 text-xs text-foreground hover:bg-muted/40">
              <input
                type="checkbox"
                checked={selectedSubjectIds.length === 0 || selectedSubjectIds.includes(subject.id)}
                class="size-3.5 accent-primary"
                onchange={() => toggleSubjectFilter(subject.id)}
              />
              <span class="size-2 rounded-full" style={`background-color:${subject.color || 'var(--primary)'}`}></span>
              <span class="min-w-0 flex-1 truncate">{subject.name_ar || subject.name}</span>
            </label>
          {/each}
        </div>
      </details>

      <details open class="border-b border-border/50">
        <summary class="cursor-pointer px-4 py-3 text-xs font-semibold text-foreground">
          {m.academics_studio_content_types()}
        </summary>
        <div class="space-y-1 px-4 pb-3">
          <label class="flex cursor-pointer items-center gap-2 rounded-md px-1 py-1.5 text-xs text-foreground hover:bg-muted/40">
            <input type="checkbox" bind:checked={showClassesLane} class="size-3.5 accent-primary" />
            <AcademicCap size={13} class="text-primary" />
            <span>{m.academics_studio_classes()}</span>
          </label>
          <label class="flex cursor-pointer items-center gap-2 rounded-md px-1 py-1.5 text-xs text-foreground hover:bg-muted/40">
            <input type="checkbox" bind:checked={showTeachersLane} class="size-3.5 accent-primary" />
            <User size={13} class="text-sky-600" />
            <span>{m.academics_studio_teachers()}</span>
          </label>
          <label class="flex cursor-pointer items-center gap-2 rounded-md px-1 py-1.5 text-xs text-foreground hover:bg-muted/40">
            <input type="checkbox" bind:checked={showCurriculumLane} class="size-3.5 accent-primary" />
            <BookOpen size={13} class="text-emerald-600" />
            <span>{m.academics_studio_quotas()}</span>
          </label>
        </div>
      </details>

      <section class="border-b border-border/50 px-4 py-3">
        <h2 class="mb-2 text-xs font-semibold text-foreground">{m.academics_studio_week_summary()}</h2>
        <div class="space-y-2.5 text-[11px]">
          <div class="flex items-center justify-between gap-2">
            <span class="text-muted-foreground">{m.academics_studio_weekly_hours()}</span>
            <strong class="text-foreground">{totalWeeklyQuota}</strong>
          </div>
          <div class="flex items-center justify-between gap-2">
            <span class="text-muted-foreground">{m.academics_studio_current_schedule({ count: lessons.length })}</span>
            <strong class="text-foreground">{lessons.length}</strong>
          </div>
          <div class="flex items-center justify-between gap-2">
            <span class="text-muted-foreground">{m.academics_studio_pending_hours()}</span>
            <strong class="text-primary">{pendingLessonCount}</strong>
          </div>
          <div class="flex items-center justify-between gap-2">
            <span class="text-muted-foreground">{m.academics_studio_teachers()}</span>
            <strong class="text-foreground">{visiblePlannerTeachers.length}</strong>
          </div>
        </div>
      </section>

      <nav class="mt-auto space-y-1 p-3">
        <p class="px-2 pb-1 text-[10px] font-semibold text-muted-foreground">{m.academics_studio_workspaces()}</p>
        {#each [
          { id: "overview" as const, label: m.academics_studio_overview_tab(), icon: Calendar },
          { id: "curriculum" as const, label: m.academics_studio_quotas(), icon: BookOpen },
          { id: "schedule" as const, label: m.academics_studio_schedule_tab(), icon: Eye },
          { id: "generator" as const, label: m.academics_studio_generator_tab(), icon: Sparkles },
        ] as item (item.id)}
          <button
            type="button"
            class="flex w-full items-center gap-2 rounded-md px-2.5 py-2 text-start text-xs font-medium transition-colors {activeTab === item.id
              ? 'bg-primary/10 text-primary'
              : 'text-muted-foreground hover:bg-muted/50 hover:text-foreground'}"
            aria-current={activeTab === item.id ? "page" : undefined}
            onclick={() => (activeTab = item.id)}
          >
            <item.icon size={14} />
            {item.label}
          </button>
        {/each}
      </nav>
    </aside>

    <main class="min-h-0 min-w-0 flex-1 overflow-auto bg-background p-4">
      {#if pageError}
        <div class="mb-4 rounded-lg border border-destructive/30 bg-destructive/5 p-3 text-xs text-destructive" role="alert">
          {extractErrorMessage(pageError)}
        </div>
      {/if}

      {#if isLoading}
        <div class="grid gap-3 sm:grid-cols-2 xl:grid-cols-4" aria-busy="true">
          {#each [1, 2, 3, 4] as item (item)}
            <div class="h-24 animate-pulse rounded-xl border border-border/60 bg-muted/30"></div>
          {/each}
          <div class="col-span-full h-64 animate-pulse rounded-xl border border-border/60 bg-muted/30"></div>
        </div>
      {:else if activeTab === "overview"}
        <div class="flex min-h-full min-w-[940px] flex-col overflow-hidden rounded-lg border border-border/70 bg-card">
          <div class="flex flex-wrap items-center justify-between gap-3 border-b border-border/60 px-3 py-2">
            <div class="flex items-center gap-2">
              <Badge variant="secondary" class="text-[10px]">{m.academics_studio_schedule_tab()}</Badge>
              <span class="text-xs font-medium text-foreground">{activePeriod?.name_ar || activePeriod?.name || m.academics_studio_select_period()}</span>
              {#if activeYear}
                <span class="text-[10px] text-muted-foreground">· {dateRange(activeYear.start_date, activeYear.end_date)}</span>
              {/if}
            </div>
            <div class="flex items-center gap-2">
              <Button variant="outline" size="sm" class="h-7 gap-1 text-[10px]" disabled={!effectiveActivePeriodId} onclick={() => (isTimeSlotSheetOpen = true)}>
                <ClockRotateRight size={12} />
                {m.academics_studio_manage_slots()}
              </Button>
              <Button variant="outline" size="sm" class="h-7 gap-1 text-[10px]" onclick={() => (activeTab = "curriculum")}>
                <BookOpen size={12} />
                {m.academics_studio_quotas()}
              </Button>
              <Button size="sm" class="h-7 gap-1 text-[10px]" disabled={!canGenerate} onclick={handleGenerate}>
                <Sparkles size={12} />
                {m.academics_schedule_generate()}
              </Button>
            </div>
          </div>

          {#if !effectiveActivePeriodId}
            <div class="flex flex-1 flex-col items-center justify-center gap-3 p-10 text-center">
              <Calendar size={26} class="text-muted-foreground" />
              <p class="text-sm font-semibold text-foreground">{m.academics_studio_no_periods()}</p>
              <Button variant="outline" size="sm" onclick={openPeriodSettings}>{m.academics_studio_manage_periods()}</Button>
            </div>
          {:else if timeSlots.length === 0}
            <div class="flex flex-1 flex-col items-center justify-center gap-3 p-10 text-center">
              <ClockRotateRight size={26} class="text-muted-foreground" />
              <p class="max-w-md text-sm font-semibold text-foreground">{m.academics_studio_no_time_slots()}</p>
              <Button size="sm" onclick={() => (isTimeSlotSheetOpen = true)}>{m.academics_studio_manage_slots()}</Button>
            </div>
          {:else}
            <div class="flex-1 overflow-auto">
              <div class="min-w-[940px]">
                <div
                  class="sticky top-0 z-10 grid border-b border-border/60 bg-card"
                  style={`grid-template-columns: 48px repeat(${visibleDays.length}, minmax(130px, 1fr))`}
                >
                  <div class="border-e border-border/60 p-2"></div>
                  {#each visibleDays as day (day)}
                    {@const daySlotCount = timeSlots.filter((slot) => slot.day_of_week === day).length}
                    <div class="border-e border-border/60 p-2 text-center last:border-e-0">
                      <p class="text-[10px] text-muted-foreground">{daySlotCount} {m.academics_studio_slots_label()}</p>
                      <p class="mt-0.5 text-xs font-semibold text-foreground">{getDayName(day)}</p>
                    </div>
                  {/each}
                </div>

                {#if showClassesLane}
                  <section
                    class="grid border-b border-border/60"
                    style={`grid-template-columns: 48px repeat(${visibleDays.length}, minmax(130px, 1fr))`}
                  >
                    <div class="flex items-center justify-center border-e border-border/60 bg-violet-50/70 p-1 text-violet-700 dark:bg-violet-500/10 dark:text-violet-300">
                      <span class="text-[10px] font-semibold" style="writing-mode:vertical-rl">{m.academics_studio_classes()}</span>
                    </div>
                    {#each visibleDays as day (day)}
                      <div class="min-h-36 space-y-1.5 border-e border-border/50 p-1.5 last:border-e-0">
                        {#each lessonsForDay(day) as entry (entry.lesson.id)}
                          <article class="rounded-md border border-violet-200 bg-violet-50 p-2 text-violet-950 shadow-sm dark:border-violet-400/20 dark:bg-violet-500/10 dark:text-violet-100">
                            <div class="flex items-start justify-between gap-1">
                              <span class="min-w-0 truncate text-[10px] font-bold">{getClassName(entry.item.section_id)}</span>
                              <span class="shrink-0 font-mono text-[9px] opacity-70">{entry.slot.start_time.slice(0, 5)}</span>
                            </div>
                            <p class="mt-1 truncate text-[10px]">{getSubjectName(entry.item.subject_id)}</p>
                            <div class="mt-1 flex flex-wrap gap-x-2 text-[9px] opacity-75">
                              <span>{getTeacherName(entry.lesson.teacher_id || entry.item.instructor_id)}</span>
                              {#if entry.lesson.room_id}<span>{getRoomName(entry.lesson.room_id)}</span>{/if}
                            </div>
                          </article>
                        {:else}
                          <div class="flex min-h-32 items-center justify-center rounded-md border border-dashed border-border/40 text-[10px] text-muted-foreground/60">—</div>
                        {/each}
                      </div>
                    {/each}
                  </section>
                {/if}

                {#if showTeachersLane}
                  <section
                    class="grid border-b border-border/60"
                    style={`grid-template-columns: 48px repeat(${visibleDays.length}, minmax(130px, 1fr))`}
                  >
                    <div class="flex items-center justify-center border-e border-border/60 bg-sky-50/70 p-1 text-sky-700 dark:bg-sky-500/10 dark:text-sky-300">
                      <span class="text-[10px] font-semibold" style="writing-mode:vertical-rl">{m.academics_studio_teachers()}</span>
                    </div>
                    {#each visibleDays as day (day)}
                      <div class="min-h-32 space-y-1.5 border-e border-border/50 p-1.5 last:border-e-0">
                        {#each lessonsForDay(day) as entry (entry.lesson.id)}
                          <article class="rounded-md border border-sky-200 bg-sky-50 p-2 text-sky-950 shadow-sm dark:border-sky-400/20 dark:bg-sky-500/10 dark:text-sky-100">
                            <div class="flex items-start justify-between gap-1">
                              <span class="min-w-0 truncate text-[10px] font-semibold">{getTeacherName(entry.lesson.teacher_id || entry.item.instructor_id)}</span>
                              <span class="shrink-0 font-mono text-[9px] opacity-70">{entry.slot.start_time.slice(0, 5)}</span>
                            </div>
                            <p class="mt-1 truncate text-[9px] opacity-80">{getClassName(entry.item.section_id)} · {getSubjectName(entry.item.subject_id)}</p>
                          </article>
                        {:else}
                          <div class="flex min-h-28 items-center justify-center rounded-md border border-dashed border-border/40 text-[10px] text-muted-foreground/60">—</div>
                        {/each}
                      </div>
                    {/each}
                  </section>
                {/if}

                {#if showCurriculumLane}
                  <section
                    class="grid"
                    style={`grid-template-columns: 48px repeat(${visibleDays.length}, minmax(130px, 1fr))`}
                  >
                    <div class="flex items-center justify-center border-e border-border/60 bg-emerald-50/70 p-1 text-emerald-700 dark:bg-emerald-500/10 dark:text-emerald-300">
                      <span class="text-[10px] font-semibold" style="writing-mode:vertical-rl">{m.academics_studio_quotas()}</span>
                    </div>
                    {#each visibleDays as day (day)}
                      <div class="min-h-28 space-y-1.5 border-e border-border/50 p-1.5 last:border-e-0">
                        {#each curriculumForDay(day) as item (`${day}-${item.id}`)}
                          {@const scheduled = lessons.filter((lesson) => lesson.section_subject_id === item.id).length}
                          <article class="rounded-md border border-emerald-200 bg-emerald-50 p-2 text-emerald-950 shadow-sm dark:border-emerald-400/20 dark:bg-emerald-500/10 dark:text-emerald-100">
                            <p class="truncate text-[10px] font-semibold">{getSubjectName(item.subject_id)}</p>
                            <p class="mt-1 truncate text-[9px] opacity-80">{getClassName(item.section_id)}</p>
                            <p class="mt-1 text-[9px] opacity-70">{scheduled}/{Number(item.weekly_hours) || 0} {m.academics_studio_hours()}</p>
                          </article>
                        {:else}
                          <div class="flex min-h-24 items-center justify-center rounded-md border border-dashed border-border/40 text-[10px] text-muted-foreground/60">—</div>
                        {/each}
                      </div>
                    {/each}
                  </section>
                {/if}
              </div>
            </div>
          {/if}
        </div>
      {:else if activeTab === "curriculum"}
        <div class="space-y-4">
          <div class="flex flex-wrap items-center justify-between gap-3">
            <div>
              <h2 class="text-base font-bold text-foreground">{m.academics_studio_quotas()}</h2>
              <p class="mt-1 text-xs text-muted-foreground">
                {m.academics_studio_assigned_subjects({ count: sectionSubjects.length })}
              </p>
            </div>
            <Button variant="outline" size="sm" class="gap-2 text-xs" onclick={openPeriodSettings}>
              <Calendar size={14} />
              {m.academics_studio_manage_periods()}
            </Button>
          </div>

          {#if sectionSubjects.length === 0}
            <div class="rounded-xl border border-dashed border-border/70 p-10 text-center text-sm text-muted-foreground">
              {m.academics_studio_no_curriculum()}
            </div>
          {:else}
            <div class="overflow-x-auto rounded-xl border border-border/60 bg-card shadow-xs">
              <table class="w-full min-w-[700px] border-collapse text-xs">
                <thead class="bg-muted/30">
                  <tr class="border-b border-border/60">
                    <th class="p-3 text-start font-semibold text-foreground">{m.academics_studio_class()}</th>
                    <th class="p-3 text-start font-semibold text-foreground">{m.academics_studio_subject()}</th>
                    <th class="p-3 text-start font-semibold text-foreground">{m.academics_studio_assigned_teacher()}</th>
                    <th class="w-36 p-3 text-center font-semibold text-foreground">{m.academics_studio_weekly_hours()}</th>
                    <th class="w-32 p-3 text-center font-semibold text-foreground">{m.academics_studio_status()}</th>
                  </tr>
                </thead>
                <tbody>
                  {#each sectionSubjects as item (item.id)}
                    {@const scheduled = lessons.filter((lesson) => lesson.section_subject_id === item.id).length}
                    {@const target = Number(item.weekly_hours) || 0}
                    <tr class="border-b border-border/40 last:border-b-0 hover:bg-muted/10">
                      <td class="p-3 font-medium text-foreground">{getClassName(item.section_id)}</td>
                      <td class="p-3 text-foreground">{getSubjectName(item.subject_id)}</td>
                      <td class="p-3">
                        <Select
                          type="single"
                          value={item.instructor_id || "unassigned"}
                          onValueChange={(value) => handleTeacherChange(item, value)}
                        >
                          <SelectTrigger class="h-8 w-52 bg-background text-xs">
                            <span class="truncate">{getTeacherName(item.instructor_id)}</span>
                          </SelectTrigger>
                          <SelectContent class="max-h-60 text-start">
                            <SelectItem value="unassigned" class="text-xs">{m.academics_studio_unassigned()}</SelectItem>
                            {#each teachers as teacher (teacher.id)}
                              <SelectItem value={teacher.id} class="text-xs">
                                {teacher.full_name_ar || teacher.full_name}
                              </SelectItem>
                            {/each}
                          </SelectContent>
                        </Select>
                      </td>
                      <td class="p-3">
                        <Input
                          class="mx-auto h-8 w-20 text-center"
                          type="number"
                          min="0"
                          max="60"
                          step="1"
                          value={String(target)}
                          aria-label={`${m.academics_studio_weekly_hours()} — ${getClassName(item.section_id)} / ${getSubjectName(item.subject_id)}`}
                          onchange={(event) => handleQuotaChange(item, event.currentTarget.value)}
                        />
                      </td>
                      <td class="p-3 text-center">
                        <Badge
                          variant={scheduled >= target ? "default" : "outline"}
                          class={scheduled >= target ? "bg-emerald-600 text-white" : ""}
                        >
                          {scheduled}/{target}
                        </Badge>
                      </td>
                    </tr>
                  {/each}
                </tbody>
              </table>
            </div>
          {/if}

          <section class="rounded-xl border border-border/60 bg-card p-4 shadow-xs">
            <div class="mb-3 flex items-center gap-2">
              <User size={15} class="text-primary" />
              <h3 class="text-sm font-bold text-foreground">{m.academics_studio_teacher_loads()}</h3>
            </div>
            {#if teachers.length === 0}
              <p class="text-xs text-muted-foreground">{m.academics_studio_no_teacher()}</p>
            {:else}
              <div class="grid gap-2 sm:grid-cols-2 xl:grid-cols-3">
                {#each teachers as teacher (teacher.id)}
                  {@const load = teacherWorkloads.get(teacher.id) ?? 0}
                  <div class="flex items-center justify-between gap-3 rounded-lg border border-border/50 px-3 py-2">
                    <span class="truncate text-xs font-medium text-foreground">{teacher.full_name_ar || teacher.full_name}</span>
                    <Badge variant="secondary" class="shrink-0 font-mono text-[10px]">
                      {load} {m.academics_studio_hours()}
                    </Badge>
                  </div>
                {/each}
              </div>
            {/if}
          </section>
        </div>
      {:else if activeTab === "schedule"}
        <div class="flex h-full min-h-[420px] flex-col gap-4">
          <div class="flex flex-wrap items-center justify-between gap-3">
            <div>
              <h2 class="text-base font-bold text-foreground">{m.academics_studio_schedule_tab()}</h2>
              <p class="mt-1 text-xs text-muted-foreground">
                {m.academics_studio_current_schedule({ count: lessons.length })}
              </p>
            </div>
            <div class="flex flex-wrap gap-2">
              {#if classesWithCurriculum.length > 0}
                <Select
                  type="single"
                  value={selectedScheduleClassId}
                  onValueChange={(value) => (selectedClassId = value ?? "")}
                >
                  <SelectTrigger class="h-9 w-52 bg-card text-xs">
                    <span class="truncate">{getClassName(selectedScheduleClassId)}</span>
                  </SelectTrigger>
                  <SelectContent class="max-h-60 text-start">
                    {#each classesWithCurriculum as section (section.id)}
                      <SelectItem value={section.id} class="text-xs">{getClassName(section.id)}</SelectItem>
                    {/each}
                  </SelectContent>
                </Select>
              {/if}
              <Button variant="outline" size="sm" class="h-9 gap-2 text-xs" onclick={openSchedule}>
                <Eye size={14} />
                {m.academics_studio_preview()}
              </Button>
            </div>
          </div>

          {#if timeSlots.length === 0}
            <div class="flex flex-1 flex-col items-center justify-center gap-3 rounded-xl border border-dashed border-border/70 p-8 text-center">
              <ClockRotateRight size={24} class="text-muted-foreground" />
              <p class="max-w-md text-sm text-muted-foreground">{m.academics_studio_no_time_slots()}</p>
              <Button
                variant="outline"
                size="sm"
                disabled={!effectiveActivePeriodId}
                onclick={() => (isTimeSlotSheetOpen = true)}
              >
                {m.academics_studio_manage_slots()}
              </Button>
            </div>
          {:else}
            <div class="flex-1 overflow-auto rounded-xl border border-border/60 bg-card shadow-xs">
              <table class="w-full min-w-[760px] border-collapse text-xs">
                <thead class="sticky top-0 z-10 bg-muted/50">
                  <tr class="border-b border-border/60">
                    <th class="w-28 p-3 text-start font-semibold text-foreground">{m.academics_schedule_time_slots_heading()}</th>
                    {#each weekdays as day (day)}
                      <th class="p-3 text-center font-semibold text-foreground">{getDayName(day)}</th>
                    {/each}
                  </tr>
                </thead>
                <tbody>
                  {#each slotIndexes as slotIndex (slotIndex)}
                    {@const firstSlot = timeSlots.find((slot) => slot.slot_index === slotIndex)}
                    <tr class="border-b border-border/40 last:border-b-0">
                      <th class="border-e border-border/40 p-3 text-start font-medium text-foreground">
                        <span class="block">{firstSlot?.name_ar || firstSlot?.name || slotIndex}</span>
                        <span class="mt-1 block whitespace-nowrap font-mono text-[10px] text-muted-foreground">
                          {firstSlot?.start_time.slice(0, 5)} – {firstSlot?.end_time.slice(0, 5)}
                        </span>
                      </th>
                      {#each weekdays as day (day)}
                        {@const slot = timeSlots.find((item) => item.day_of_week === day && item.slot_index === slotIndex)}
                        {@const lesson = slot
                          ? lessons.find((item) => item.time_slot_id === slot.id && sectionSubjectById.get(item.section_subject_id)?.section_id === selectedScheduleClassId)
                          : undefined}
                        {@const link = lesson ? sectionSubjectById.get(lesson.section_subject_id) : undefined}
                        <td class="min-w-32 p-1.5 align-top">
                          {#if slot?.is_break}
                            <div class="rounded-lg border border-amber-500/20 bg-amber-500/5 px-2 py-2 text-center text-[10px] text-muted-foreground">
                              {slot.name_ar || slot.name || m.common_onboarding_break_recess()}
                            </div>
                          {:else if lesson && link}
                            <div class="min-h-12 rounded-lg border border-primary/20 bg-primary/5 p-2">
                              <p class="font-semibold text-foreground">{getSubjectName(link.subject_id)}</p>
                              <p class="mt-1 truncate text-[10px] text-muted-foreground">
                                {getTeacherName(lesson.teacher_id || link.instructor_id)}
                              </p>
                              {#if lesson.room_id}
                                <p class="mt-0.5 truncate text-[10px] text-muted-foreground">{getRoomName(lesson.room_id)}</p>
                              {/if}
                            </div>
                          {:else}
                            <div class="min-h-12 rounded-lg border border-dashed border-border/50 bg-muted/10"></div>
                          {/if}
                        </td>
                      {/each}
                    </tr>
                  {/each}
                </tbody>
              </table>
            </div>
          {/if}
        </div>
      {:else}
        <div class="mx-auto max-w-4xl space-y-4">
          <section class="rounded-xl border border-border/60 bg-card p-4 shadow-xs">
            <div class="flex flex-wrap items-start justify-between gap-4">
              <div>
                <div class="flex items-center gap-2">
                  <Sparkles size={17} class="text-primary" />
                  <h2 class="text-base font-bold text-foreground">{m.academics_schedule_solver_title()}</h2>
                </div>
                <p class="mt-2 max-w-2xl text-xs leading-relaxed text-muted-foreground">
                  {m.academics_schedule_generate_desc()}
                </p>
              </div>
              <Badge variant={sidecarQuery.data?.is_healthy ? "default" : "outline"}>
                {sidecarQuery.data?.is_healthy
                  ? m.academics_studio_solver_ready()
                  : !isDesktopApp
                    ? m.academics_studio_solver_unavailable()
                    : sidecarQuery.isPending
                      ? m.academics_studio_solver_checking()
                      : m.academics_studio_solver_not_ready()}
              </Badge>
            </div>

            <div class="mt-4 grid gap-3 sm:grid-cols-3">
              <div class="rounded-lg bg-muted/25 p-3">
                <p class="text-[11px] text-muted-foreground">{m.academics_studio_academic_year()}</p>
                <p class="mt-1 truncate text-sm font-semibold text-foreground">{activeYear?.name_ar || activeYear?.name || "—"}</p>
              </div>
              <div class="rounded-lg bg-muted/25 p-3">
                <p class="text-[11px] text-muted-foreground">{m.academics_studio_select_period()}</p>
                <p class="mt-1 truncate text-sm font-semibold text-foreground">{activePeriod?.name_ar || activePeriod?.name || "—"}</p>
              </div>
              <div class="rounded-lg bg-muted/25 p-3">
                <p class="text-[11px] text-muted-foreground">{m.academics_studio_generate_incremental()}</p>
                <p class="mt-1 text-sm font-semibold text-foreground">
                  {lessons.length > 0 ? m.academics_schedule_generate_incremental() : m.academics_studio_generate_full()}
                </p>
              </div>
            </div>

            <div class="mt-4 flex flex-wrap items-center justify-between gap-3 border-t border-border/50 pt-4">
              <div class="space-y-1 text-xs text-muted-foreground">
                <p>{m.academics_studio_assigned_subjects({ count: sectionSubjects.length })}</p>
                <p>{m.academics_schedule_items_created()}: {pendingLessonCount}</p>
              </div>
              <Button disabled={!canGenerate} onclick={handleGenerate} class="gap-2">
                <Sparkles size={14} />
                {isSolving ? m.academics_schedule_computing() : m.academics_schedule_generate()}
              </Button>
            </div>
          </section>

          <section class="rounded-xl border border-border/60 bg-card p-4 shadow-xs">
            <div class="mb-3 flex items-center justify-between gap-3">
              <h3 class="text-sm font-bold text-foreground">{m.academics_studio_readiness()}</h3>
              {#if activePeriod?.is_locked}
                <Badge variant="outline">{m.academics_studio_locked_period()}</Badge>
              {/if}
            </div>
            {#if readinessIssues.length > 0}
              <ul class="space-y-2">
                {#each readinessIssues as issue (issue)}
                  <li class="flex items-start gap-2 rounded-lg bg-amber-500/5 px-3 py-2 text-xs">
                    <InfoCircle size={14} class="mt-0.5 shrink-0 text-amber-600" />
                    <span>{issue}</span>
                  </li>
                {/each}
              </ul>
            {:else if pendingLessonCount === 0}
              <div class="flex items-center gap-2 text-xs text-emerald-700 dark:text-emerald-400">
                <Check size={14} />
                {m.academics_studio_no_pending()}
              </div>
            {:else}
              <p class="text-xs text-muted-foreground">{m.academics_studio_solution_ready()}</p>
            {/if}
          </section>

          {#if activeJobId}
            <section class="rounded-xl border border-border/60 bg-card p-4 shadow-xs">
              <div class="flex flex-wrap items-center justify-between gap-3">
                <div>
                  <h3 class="text-sm font-bold text-foreground">{m.academics_studio_generator_tab()}</h3>
                  <p class="mt-1 font-mono text-[10px] text-muted-foreground">{activeJobId}</p>
                </div>
                <Badge
                  variant={solveStatus?.status === "completed" ? "default" : solveStatus?.status === "failed" ? "destructive" : "secondary"}
                >
                  {solveStatus?.status === "completed"
                    ? m.academics_schedule_status_completed()
                    : solveStatus?.status === "failed"
                      ? m.academics_schedule_status_failed()
                      : isSolveRunning
                        ? m.academics_schedule_status_running()
                        : solveStatus?.status ?? m.academics_schedule_status_running()}
                </Badge>
              </div>

              {#if solveStatusQuery.isError}
                <p class="mt-3 rounded-lg bg-destructive/5 p-3 text-xs text-destructive" role="alert">
                  {extractErrorMessage(solveStatusQuery.error)}
                </p>
              {:else if solveStatus?.error}
                <p class="mt-3 rounded-lg bg-destructive/5 p-3 text-xs text-destructive" role="alert">
                  {solveStatus.error}
                </p>
              {:else if solveStatus?.status === "completed"}
                <div class="mt-3 flex flex-wrap items-center justify-between gap-3 rounded-lg border border-emerald-600/20 bg-emerald-600/5 p-3">
                  <div class="text-xs">
                    <p class="font-semibold text-foreground">{m.academics_studio_solution_ready()}</p>
                    <p class="mt-1 text-muted-foreground">
                      {m.academics_studio_draft({ count: solutionAssignments.length })}
                      {#if solveStatus.score}
                        · {m.academics_schedule_score()}: {solveStatus.score}
                      {/if}
                    </p>
                    {#if unplacedAssignmentCount > 0}
                      <p class="mt-1 text-destructive">
                        {m.academics_studio_unplaced_lessons({ count: unplacedAssignmentCount })}
                      </p>
                    {/if}
                    {#if !hasFeasibleSolution}
                      <p class="mt-1 text-destructive">
                        {m.academics_studio_infeasible_solution()}
                      </p>
                    {/if}
                  </div>
                  <div class="flex flex-wrap gap-2">
                    <Button variant="outline" size="sm" class="gap-1.5 text-xs" onclick={() => (activeTab = "schedule")}>
                      <Eye size={13} />
                      {m.academics_studio_preview()}
                    </Button>
                    <Button
                      size="sm"
                      class="gap-1.5 text-xs"
                      disabled={!canApplySolution || isApplying}
                      onclick={handleApplySolution}
                    >
                      <Check size={14} />
                      {isApplying ? m.academics_schedule_applying() : m.academics_schedule_apply_proposed()}
                    </Button>
                  </div>
                </div>
              {/if}

              {#if solutionAssignments.length > 0}
                <div class="mt-4 overflow-x-auto rounded-lg border border-border/50">
                  <table class="w-full min-w-[620px] border-collapse text-xs">
                    <thead class="bg-muted/30">
                      <tr class="border-b border-border/50">
                        <th class="p-2.5 text-start font-semibold">{m.academics_studio_class()}</th>
                        <th class="p-2.5 text-start font-semibold">{m.academics_studio_subject()}</th>
                        <th class="p-2.5 text-start font-semibold">{m.academics_studio_teacher()}</th>
                        <th class="p-2.5 text-start font-semibold">{m.academics_studio_schedule_tab()}</th>
                        <th class="p-2.5 text-start font-semibold">{m.academics_studio_rooms()}</th>
                      </tr>
                    </thead>
                    <tbody>
                      {#each solutionAssignments as assignment (assignment.id)}
                        {@const item = sectionSubjectById.get(assignment.section_subject_id)}
                        {@const slot = assignment.time_slot_id ? timeSlotById.get(assignment.time_slot_id) : undefined}
                        <tr class="border-b border-border/40 last:border-b-0">
                          <td class="p-2.5">{item ? getClassName(item.section_id) : "—"}</td>
                          <td class="p-2.5">{item ? getSubjectName(item.subject_id) : "—"}</td>
                          <td class="p-2.5">{getTeacherName(assignment.teacher_id)}</td>
                          <td class="p-2.5">
                            {#if slot}
                              {getDayName(slot.day_of_week)} · {slot.start_time.slice(0, 5)}–{slot.end_time.slice(0, 5)}
                            {:else}
                              <span class="text-destructive">—</span>
                            {/if}
                          </td>
                          <td class="p-2.5">{getRoomName(assignment.room_id) || "—"}</td>
                        </tr>
                      {/each}
                    </tbody>
                  </table>
                </div>
              {/if}
            </section>
          {/if}

          {#if !isDesktopApp}
            <p class="rounded-lg border border-amber-500/20 bg-amber-500/5 p-3 text-xs text-muted-foreground">
              {m.academics_studio_solver_unavailable()}
            </p>
          {/if}
        </div>
      {/if}
    </main>
  </div>

  <TimeSlotSheet
    bind:open={isTimeSlotSheetOpen}
    periodId={effectiveActivePeriodId}
    {timeSlots}
    onSuccess={() => {
      queryClient.invalidateQueries({
        queryKey: ["time-slots", orgId, effectiveActivePeriodId],
      });
      queryClient.invalidateQueries({
        queryKey: ["period-schedule", orgId, effectiveActivePeriodId],
      });
    }}
  />
</div>
