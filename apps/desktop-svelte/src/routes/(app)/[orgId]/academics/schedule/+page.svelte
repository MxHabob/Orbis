<!--
@component AcademicsSchedulePage
Displays the weekly academic schedule grid,
allows selecting classes or teachers, scheduling lessons, and deleting scheduled lessons.
Triggers the LessonCreateSheet for scheduling lessons.
-->
<script lang="ts">
import { onMount } from "svelte";
import { commands } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { Plus, Clock, MapPin, User, X } from "lucide-svelte";
import { toast } from "svelte-sonner";
import { getLocale } from "$lib/paraglide/runtime";
import { fade } from "svelte/transition";
import LessonCreateSheet from "$lib/features/academics/components/lesson-create-sheet.svelte";
import SchedulerAiPanel from "$lib/features/academics/components/scheduler-ai-panel.svelte";
import { Timetable } from "@orbis/ui-svelte/timetable";
import * as m from "$lib/paraglide/messages";

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

    if (classes.length > 0) {
      selectedClassId = String(classes[0].id);
    }
    if (teachers.length > 0) {
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

    if (conflicts && conflicts.length > 0) {
      const first = conflicts[0] as any;
      const type = String(first.conflict_type || "");
      let arMsg = `لا يمكن نقل الحصة لوجود تضارب: `;
      if (type === "teacher") arMsg += "المعلم مشغول في هذا الوقت.";
      else if (type === "class") arMsg += "الصف لديه حصة أخرى في هذا الوقت.";
      else if (type === "room") arMsg += "القاعة/الغرفة مشغولة في هذا الوقت.";
      else arMsg += first.message || "";

      const enMsg = `Cannot move lesson: ${first.message || "conflict detected."}`;
      toast.error(getLocale() === "ar" ? arMsg : enMsg, { duration: 5000 });
      return;
    }

    toast.loading(
      getLocale() === "ar" ? "جاري نقل الحصة..." : "Moving lesson...",
    );
    await commands.schedule.updateLesson(data.orgId, lesson.id, {
      teacher_id: lesson.teacher_id,
      time_slot_id: targetSlotId,
      day_of_week: targetDay,
      room_id: lesson.room_id || undefined,
    });

    toast.dismiss();
    toast.success(
      getLocale() === "ar"
        ? "تم نقل الحصة بنجاح!"
        : "Lesson moved successfully!",
    );
    await loadScheduleLessons();
  } catch (err: any) {
    toast.dismiss();
    toast.error(err instanceof Error ? err.message : String(err));
  }
}

onMount(() => {
  if (data.orgId) {
    initializeSchedule();
  }
});

// Reactively reload schedule lessons when filters change
$effect(() => {
  if (selectedClassId || selectedTeacherId || viewMode || activeCycleId) {
    loadScheduleLessons();
  }
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
			<div class="flex flex-col md:flex-row md:items-center md:justify-between gap-4 bg-card/25 p-4 rounded-xl border border-border/80 shrink-0 select-none">
				<div class="flex items-center gap-3">
					<span class="text-xs font-bold text-muted-foreground uppercase tracking-wider">
						{m.schedule_page_view_by()}
					</span>
					<div class="flex p-0.5 rounded-lg bg-muted border border-border/60">
						<button 
							class="text-[10px] font-bold px-3 py-1.5 rounded-md transition-all duration-150 cursor-pointer {viewMode === 'class' ? 'bg-card text-foreground shadow-sm' : 'text-muted-foreground'}"
							onclick={() => viewMode = 'class'}
						>
							{m.schedule_page_class()}
						</button>
						<button 
							class="text-[10px] font-bold px-3 py-1.5 rounded-md transition-all duration-150 cursor-pointer {viewMode === 'teacher' ? 'bg-card text-foreground shadow-sm' : 'text-muted-foreground'}"
							onclick={() => viewMode = 'teacher'}
						>
							{m.schedule_page_teacher()}
						</button>
					</div>
				</div>

				<!-- Selector Box -->
				<div class="flex items-center gap-3">
					{#if viewMode === 'class'}
						<span class="text-xs text-muted-foreground">{m.schedule_page_sel_class()}</span>
						<select 
							bind:value={selectedClassId} 
							class="bg-background text-foreground border border-border rounded-md px-3 py-1.5 text-xs focus:ring-1 focus:ring-ring outline-none"
						>
							{#each classes as cls (cls.id)}
								<option value={String(cls.id)}>{cls.name_ar || cls.name}</option>
							{/each}
						</select>
					{:else}
						<span class="text-xs text-muted-foreground">{m.schedule_page_sel_teacher()}</span>
						<select 
							bind:value={selectedTeacherId} 
							class="bg-background text-foreground border border-border rounded-md px-3 py-1.5 text-xs focus:ring-1 focus:ring-ring outline-none"
						>
							{#each teachers as teach (teach.id)}
								<option value={String(teach.id)}>{getLocale() === "ar" ? (teach.name_ar || teach.full_name) : teach.full_name}</option>
							{/each}
						</select>
					{/if}
				</div>
			</div>

			<!-- Timetable Grid Content -->
			{#if isLoading}
				<div class="flex-1 w-full p-4 flex flex-col gap-3 bg-card border border-border rounded-lg">
					<Skeleton class="h-10 w-full" />
					<Skeleton class="h-20 w-full" />
					<Skeleton class="h-20 w-full" />
				</div>
			{:else if error}
				<div class="flex flex-col items-center justify-center p-8 border border-destructive/20 bg-destructive/5 rounded-xl text-center gap-3">
					<p class="text-xs text-destructive font-semibold">{m.schedule_page_error()}</p>
					<p class="text-xs text-muted-foreground">{error}</p>
					<Button variant="outline" class="text-xs" onclick={initializeSchedule}>{m.error_retry()}</Button>
				</div>
			{:else}
				<Timetable
					{timeSlots}
					{weekdays}
					{lessons}
					draftLessons={draftLessonsInGrid}
					{previewDraft}
					{viewMode}
					locale={getLocale()}
					{getSubjectName}
					{getTeacherName}
					{getClassName}
					{getSubjectColor}
					onCellClick={openScheduler}
					onDeleteLesson={handleDeleteLesson}
					onMoveLesson={handleMoveLesson}
				/>
			{/if}
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
