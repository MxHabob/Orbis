<!--
@component AcademicsSchedulePage
Displays the weekly academic schedule grid (Sunday to Thursday),
allows selecting classes or teachers, scheduling lessons, and deleting scheduled lessons.
Triggers the LessonCreateSheet for scheduling lessons.
-->
<script lang="ts">
import { onMount } from "svelte";
import { commands } from "$lib/services/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { 
  Plus, Calendar, Clock, MapPin, User, BookOpen, 
  X, Trash2
} from "lucide-svelte";
import { toast } from "svelte-sonner";
import { getLocale } from "$lib/paraglide/runtime";
import { fade } from "svelte/transition";
import LessonCreateSheet from "$lib/components/academics/lesson-create-sheet.svelte";

let { data }: { data: { orgId: string } } = $props();
const isAr = $derived(getLocale() === "ar");

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

const weekdays = $derived([
  { value: 1, label: isAr ? "الأحد" : "Sunday" },
  { value: 2, label: isAr ? "الأثنين" : "Monday" },
  { value: 3, label: isAr ? "الثلاثاء" : "Tuesday" },
  { value: 4, label: isAr ? "الأربعاء" : "Wednesday" },
  { value: 5, label: isAr ? "الخميس" : "Thursday" },
]);

const defaultTimeSlots = [
  { name: "1st Period", name_ar: "الحصة الأولى", start_time: "08:00", end_time: "08:45" },
  { name: "2nd Period", name_ar: "الحصة الثانية", start_time: "08:45", end_time: "09:30" },
  { name: "3rd Period", name_ar: "الحصة الثالثة", start_time: "09:45", end_time: "10:30" },
  { name: "4th Period", name_ar: "الحصة الرابعة", start_time: "10:30", end_time: "11:15" },
  { name: "5th Period", name_ar: "الحصة الخامسة", start_time: "11:30", end_time: "12:15" },
];

// Load everything
async function initializeSchedule() {
  isLoading = true;
  error = null;
  try {
    // 1. Fetch cycles and details
    const cycleList = await commands.academics.getCycles(data.orgId);
    cycles = cycleList;

    // Take active cycle or fallback
    const active = cycleList.find((c) => c.is_current === 1 || c.is_current === true) || cycleList[0];
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
    teachers = userList.filter((u: any) => u.role === "teacher" || u.role === "admin" || u.role === "super_admin");

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
    for (const ts of defaultTimeSlots) {
      try {
        await commands.schedule.createTimeSlot(data.orgId, {
          name: ts.name,
          name_ar: ts.name_ar,
          start_time: ts.start_time,
          end_time: ts.end_time,
          cycle_id: activeCycleId,
        });
      } catch (err) {
        console.warn("Failed creating DB slot, fallback to mock usage", err);
      }
    }
    slots = await commands.schedule.getTimeSlots(data.orgId, activeCycleId);
  }
  timeSlots = slots.length > 0 ? slots : defaultTimeSlots.map((ts, i) => ({ ...ts, id: `mock_ts_${i}` }));
}

async function loadScheduleLessons() {
  if (!activeCycleId) return;
  lessons = [];
  try {
    if (viewMode === "class" && selectedClassId) {
      lessons = await commands.schedule.getClassSchedule(data.orgId, activeCycleId, selectedClassId);
    } else if (viewMode === "teacher" && selectedTeacherId) {
      lessons = await commands.schedule.getTeacherSchedule(data.orgId, activeCycleId, selectedTeacherId);
    }
  } catch (e) {
    console.error("Error loading schedule lessons:", e);
  }
}

onMount(() => {
  if (data.orgId) {
    initializeSchedule();
  }
});

// Reactively reload schedule lessons when filters change
$effect(() => {
  if (selectedClassId || selectedTeacherId || viewMode) {
    loadScheduleLessons();
  }
});

// Get lesson inside cell
function getLessonInCell(day: number, slotId: string) {
  return lessons.find((l) => Number(l.day_of_week) === day && String(l.time_slot_id) === String(slotId));
}

// Get labels
function getSubjectName(subjectId: string) {
  const sub = subjects.find((s) => String(s.id) === String(subjectId));
  return sub ? (sub.name_ar || sub.name) : (isAr ? "مادة غير معروفة" : "Unknown Subject");
}

function getTeacherName(teacherId: string) {
  const teach = teachers.find((t) => String(t.id) === String(teacherId));
  return teach ? teach.full_name : (isAr ? "معلم غير محدد" : "No Teacher");
}

function getClassName(classId: string) {
  const cls = classes.find((c) => String(c.id) === String(classId));
  return cls ? (cls.name_ar || cls.name) : (isAr ? "فصل غير معروف" : "Unknown Class");
}

// Delete Scheduled Lesson
async function handleDeleteLesson(lessonId: string) {
  if (!confirm(isAr ? "هل أنت متأكد من رغبتك في حذف هذه الحصة من الجدول؟" : "Are you sure you want to delete this scheduled lesson?")) return;
  try {
    await commands.schedule.deleteLesson(lessonId);
    toast.success(isAr ? "تم حذف الحصة بنجاح" : "Lesson deleted successfully");
    await loadScheduleLessons();
  } catch (e) {
    toast.error(isAr ? "فشل حذف الحصة" : "Failed to delete lesson");
  }
}

function openScheduler(day: number, slotId: string) {
  if (viewMode !== "class") {
    toast.error(isAr ? "جدولة الحصص متاحة فقط عند تصفية الجداول بالفصول" : "Scheduling is only available when viewing by Class");
    return;
  }
  activeDay = day;
  activeSlotId = slotId;
  isModalOpen = true;
}
</script>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6">
	<!-- Timetable Header Filter Controls -->
	<div class="flex flex-col md:flex-row md:items-center md:justify-between gap-4 bg-card/25 p-4 rounded-xl border border-border/80 shrink-0 select-none">
		<div class="flex items-center gap-3">
			<span class="text-xs font-bold text-muted-foreground uppercase tracking-wider">
				{isAr ? "عرض الجدول حسب:" : "View Timetable By:"}
			</span>
			<div class="flex p-0.5 rounded-lg bg-muted border border-border/60">
				<button 
					class="text-[10px] font-bold px-3 py-1.5 rounded-md transition-all duration-150 cursor-pointer {viewMode === 'class' ? 'bg-card text-foreground shadow-sm' : 'text-muted-foreground'}"
					onclick={() => viewMode = 'class'}
				>
					{isAr ? "الفصل الدراسي" : "Class"}
				</button>
				<button 
					class="text-[10px] font-bold px-3 py-1.5 rounded-md transition-all duration-150 cursor-pointer {viewMode === 'teacher' ? 'bg-card text-foreground shadow-sm' : 'text-muted-foreground'}"
					onclick={() => viewMode = 'teacher'}
				>
					{isAr ? "المعلم" : "Teacher"}
				</button>
			</div>
		</div>

		<!-- Selector Box -->
		<div class="flex items-center gap-3">
			{#if viewMode === 'class'}
				<span class="text-xs text-muted-foreground">{isAr ? "الفصل المختار:" : "Select Class:"}</span>
				<select 
					bind:value={selectedClassId} 
					class="bg-background text-foreground border border-border rounded-md px-3 py-1.5 text-xs focus:ring-1 focus:ring-ring outline-none"
				>
					{#each classes as cls (cls.id)}
						<option value={String(cls.id)}>{cls.name_ar || cls.name}</option>
					{/each}
				</select>
			{:else}
				<span class="text-xs text-muted-foreground">{isAr ? "المعلم المختار:" : "Select Teacher:"}</span>
				<select 
					bind:value={selectedTeacherId} 
					class="bg-background text-foreground border border-border rounded-md px-3 py-1.5 text-xs focus:ring-1 focus:ring-ring outline-none"
				>
					{#each teachers as teach (teach.id)}
						<option value={String(teach.id)}>{teach.full_name}</option>
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
			<p class="text-xs text-destructive font-semibold">{isAr ? "خطأ في تحميل الجدول" : "Error loading schedule"}</p>
			<p class="text-xs text-muted-foreground">{error}</p>
			<Button variant="outline" class="text-xs" onclick={initializeSchedule}>{isAr ? "إعادة المحاولة" : "Retry"}</Button>
		</div>
	{:else}
		<!-- Weekly Schedule Matrix -->
		<div class="flex-1 overflow-x-auto min-h-0 border border-border rounded-xl bg-card/10" in:fade={{ duration: 200 }}>
			<table class="w-full border-collapse text-xs select-none min-w-[700px] h-full table-fixed">
				<thead>
					<tr class="bg-card border-b border-border">
						<!-- Time Slots Column Header -->
						<th class="p-3 font-bold text-muted-foreground uppercase text-[10px] border-r border-border text-center w-36">
							{isAr ? "الحصة / الوقت" : "Period / Time"}
						</th>
						<!-- Weekdays Column Headers -->
						{#each weekdays as day (day.value)}
							<th class="p-3 font-bold text-muted-foreground uppercase text-[10px] text-center border-r border-border last:border-r-0">
								{day.label}
							</th>
						{/each}
					</tr>
				</thead>
				<tbody>
					{#each timeSlots as slot (slot.id)}
						<tr class="border-b border-border last:border-b-0 hover:bg-card/5">
							<!-- Period Info Cell -->
							<td class="p-3 border-r border-border font-medium text-center bg-card/20 flex flex-col justify-center items-center gap-1.5 h-24">
								<span class="font-bold text-foreground">{slot.name_ar || slot.name}</span>
								<span class="text-[9px] text-muted-foreground font-mono bg-background px-1.5 py-0.5 rounded border border-border/80 flex items-center gap-1">
									<Clock size={8} />
									<span>{slot.start_time} - {slot.end_time}</span>
								</span>
							</td>

							<!-- Days Cells -->
							{#each weekdays as day (day.value)}
								{@const lesson = getLessonInCell(day.value, slot.id)}
								<td class="p-2 border-r border-border last:border-r-0 text-center align-middle h-24 relative group">
									{#if lesson}
										<!-- Scheduled Lesson Card -->
										<div class="card-tactile bg-primary/5 border border-primary/20 p-2 rounded-lg flex flex-col justify-between items-center text-center h-full group/card relative">
											<!-- Close icon to delete -->
											<button 
												onclick={() => handleDeleteLesson(lesson.id)}
												class="absolute top-1 right-1 opacity-0 group-hover/card:opacity-100 h-4 w-4 text-destructive hover:bg-destructive/10 rounded flex items-center justify-center transition-opacity duration-150"
												title={isAr ? "حذف الحصة" : "Delete Lesson"}
											>
												<X size={10} />
											</button>
											<div class="flex flex-col items-center gap-1 mt-0.5">
												<span class="font-bold text-foreground text-[10px] leading-tight line-clamp-1">
													{getSubjectName(lesson.subject_id)}
												</span>
												<span class="text-[9px] text-muted-foreground leading-tight flex items-center gap-0.5">
													<User size={8} />
													<span class="line-clamp-1">{viewMode === 'class' ? getTeacherName(lesson.teacher_id) : getClassName(lesson.class_id)}</span>
												</span>
											</div>
											{#if lesson.room}
												<span class="text-[8px] font-mono text-muted-foreground bg-background px-1 py-0.2 rounded border border-border/60 flex items-center gap-0.5">
													<MapPin size={7} />
													<span>{lesson.room}</span>
												</span>
											{/if}
										</div>
									{:else}
										<!-- Empty Cell (Offer scheduling in Class mode) -->
										<button 
											onclick={() => openScheduler(day.value, slot.id)}
											class="w-full h-full border border-dashed border-transparent hover:border-primary/20 hover:bg-primary/5 rounded-lg flex items-center justify-center text-muted-foreground/0 hover:text-primary transition-all duration-150 cursor-pointer"
										>
											<Plus size={16} />
										</button>
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

