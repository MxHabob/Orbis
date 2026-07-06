<script lang="ts">
import { onMount } from "svelte";
import { page } from "$app/state";
import { commands } from "$lib/core/ipc";
import { toast } from "svelte-sonner";
import { getLocale } from "$lib/paraglide/runtime";
import LessonCreateSheet from "$lib/features/academics/components/lesson-create-sheet.svelte";
import SchedulerAiPanel from "$lib/features/academics/components/scheduler-ai-panel.svelte";
import * as m from "$lib/paraglide/messages";

// Component imports
import {
  ScheduleFilterControls,
  ScheduleTimetableWrapper,
} from "$lib/features/academics";

let { data }: { data: { orgId: string } } = $props();

// State
let classes = $state<any[]>([]);
let subjects = $state<any[]>([]);
let teachers = $state<any[]>([]);
let timeSlots = $state<any[]>([]);
let lessons = $state<any[]>([]);
let cycles = $state<any[]>([]);
let activeCycleId = $state<string>("");

let selectedClassId = $state<string>("");
let selectedTeacherId = $state<string>("");
let viewMode = $state<"class" | "teacher">("class");

let isLoading = $state(true);
let error = $state<string | null>(null);

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

const defaultTimeSlots = [
  {
    name: "1st Period",
    name_ar: "الحصة الأولى",
    start_time: "08:00",
    end_time: "08:45",
  },
  {
    name: "2nd Period",
    name_ar: "الحصة الثانية",
    start_time: "08:45",
    end_time: "09:30",
  },
  {
    name: "3rd Period",
    name_ar: "الحصة الثالثة",
    start_time: "09:45",
    end_time: "10:30",
  },
  {
    name: "4th Period",
    name_ar: "الحصة الرابعة",
    start_time: "10:30",
    end_time: "11:15",
  },
  {
    name: "5th Period",
    name_ar: "الحصة الخامسة",
    start_time: "11:30",
    end_time: "12:15",
  },
];

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

// Load everything
async function initializeSchedule() {
  isLoading = true;
  error = null;
  try {
    // 1. Fetch cycles and details
    const cycleList = await commands.academics.getCycles(data.orgId);
    cycles = cycleList;

    // Take active cycle or fallback
    const active =
      cycleList.find((c) => c.is_current === 1 || c.is_current === true) ||
      cycleList[0];
    if (active) {
      activeCycleId = String(active.id);
    } else {
      activeCycleId = "default_cycle";
    }

    // 2. Fetch classes, subjects, users
    const [classList, subjectList, userList] = await Promise.all([
      commands.classes.getAll(data.orgId),
      commands.subjects.getAll(data.orgId),
      commands.users.getUsers(data.orgId),
    ]);
    classes = classList;
    subjects = subjectList;
    teachers = userList.filter(
      (u: any) =>
        u.primary_role === "teacher" ||
        u.primary_role === "admin" ||
        u.primary_role === "super_admin",
    );

    const paramClassId = page.url.searchParams.get("classId") || page.url.searchParams.get("class");
    if (paramClassId && classList.some((c: any) => String(c.id) === paramClassId)) {
      selectedClassId = paramClassId;
      viewMode = "class";
    } else if (classes.length > 0) {
      selectedClassId = String(classes[0].id);
    }

    const paramTeacherId = page.url.searchParams.get("teacherId") || page.url.searchParams.get("teacher");
    if (paramTeacherId && userList.some((u: any) => String(u.id) === paramTeacherId)) {
      selectedTeacherId = paramTeacherId;
      viewMode = "teacher";
    } else if (teachers.length > 0) {
      selectedTeacherId = String(teachers[0].id);
    }

    // 3. Fetch/Setup time slots
    await setupTimeSlots();

    // 4. Load schedule lessons
    await loadScheduleLessons();
  } catch (e) {
    error = e instanceof Error ? e.message : String(e);
  } finally {
    isLoading = false;
  }
}

async function setupTimeSlots() {
  if (!activeCycleId) return;
  let slots = await commands.schedule.getTimeSlots(data.orgId, activeCycleId);

  if (slots.length === 0) {
    // Create standard time slots so database is initialized
    for (let i = 0; i < defaultTimeSlots.length; i++) {
      const ts = defaultTimeSlots[i];
      const [sh, sm] = ts.start_time.split(":").map(Number);
      const [eh, em] = ts.end_time.split(":").map(Number);
      const durationMinutes = eh * 60 + em - (sh * 60 + sm);
      try {
        await commands.schedule.createTimeSlot(data.orgId, {
          name: ts.name,
          name_ar: ts.name_ar,
          start_time: ts.start_time,
          end_time: ts.end_time,
          cycle_id: activeCycleId,
          slot_number: i + 1,
          duration_minutes: durationMinutes,
          is_break: 0,
        });
      } catch (err) {
        console.warn("Failed creating DB slot, fallback to mock usage", err);
      }
    }
    slots = await commands.schedule.getTimeSlots(data.orgId, activeCycleId);
  }
  timeSlots =
    slots.length > 0
      ? slots
      : defaultTimeSlots.map((ts, i) => ({ ...ts, id: `mock_ts_${i}` }));
}

async function loadScheduleLessons() {
  if (!activeCycleId) return;
  lessons = [];
  try {
    if (viewMode === "class" && selectedClassId) {
      lessons = await commands.schedule.getClassSchedule(
        data.orgId,
        activeCycleId,
        selectedClassId,
      );
    } else if (viewMode === "teacher" && selectedTeacherId) {
      lessons = await commands.schedule.getTeacherSchedule(
        data.orgId,
        activeCycleId,
        selectedTeacherId,
      );
    }
  } catch (e) {
    console.error("Error loading schedule lessons:", e);
  }
}

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
    const conflicts = await commands.schedule.checkConflicts({
      orgId: data.orgId,
      cycleId: activeCycleId,
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

    await commands.schedule.updateLessonTime(lesson.id, targetDay, targetSlotId);
    toast.success(
      getLocale() === "ar"
        ? "تم نقل الحصة بنجاح"
        : "Lesson moved successfully",
    );
    await loadScheduleLessons();
  } catch (e: any) {
    toast.dismiss();
    toast.error(e?.message || "Failed to move lesson");
  }
}

// React to class selection changes
$effect(() => {
  if (selectedClassId && viewMode === "class") {
    loadScheduleLessons();
  }
});

// React to teacher selection changes
$effect(() => {
  if (selectedTeacherId && viewMode === "teacher") {
    loadScheduleLessons();
  }
});

// React to viewMode toggle
$effect(() => {
  if (viewMode) {
    loadScheduleLessons();
  }
});

onMount(() => {
  initializeSchedule();
});

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

async function handleDeleteLesson(lessonId: string) {
  if (!confirm(m.schedule_page_confirm_delete())) return;
  try {
    await commands.schedule.deleteLesson(lessonId);
    toast.success(m.schedule_page_success_delete());
    await loadScheduleLessons();
  } catch (e) {
    toast.error(m.schedule_page_error_delete());
  }
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
      onRetry={initializeSchedule}
    />
  </div>

  <!-- AI Auto-Scheduler Sidebar Panel (Fixed Side) -->
  {#if !isLoading && !error}
    <SchedulerAiPanel
      orgId={data.orgId}
      cycleId={activeCycleId}
      selectedClassId={selectedClassId}
      selectedTeacherId={selectedTeacherId}
      viewMode={viewMode}
      subjects={subjects}
      bind:previewDraft={previewDraft}
      bind:activeJob={activeJob}
      onApply={loadScheduleLessons}
    />
  {/if}
</div>

<!-- Lesson Scheduling Side Sheet -->
<LessonCreateSheet
  bind:open={isModalOpen}
  orgId={data.orgId}
  cycleId={activeCycleId}
  classId={selectedClassId}
  dayOfWeek={activeDay}
  timeSlotId={activeSlotId}
  subjects={subjects}
  teachers={teachers}
  onSuccess={loadScheduleLessons}
/>
