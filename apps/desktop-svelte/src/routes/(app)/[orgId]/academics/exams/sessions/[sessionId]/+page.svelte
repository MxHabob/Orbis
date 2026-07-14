<!--
@component ExamSessionDetail
Session detail page with tabs: Subjects, Schedule, Seating, Halls, Proctors.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { onMount } from "svelte";
import { Button } from "@orbis/ui-svelte/button";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import {
  Table,
  TableHeader,
  TableRow,
  TableHead,
  TableBody,
  TableCell,
} from "@orbis/ui-svelte/table";
import { toast } from "svelte-sonner";
import { localizeHref, getLocale } from "$lib/paraglide/runtime";
import { fade } from "svelte/transition";
import {
  ArrowLeft,
  BookOpen,
  Calendar,
  Hash,
  Building2,
  UserCheck,
  Plus,
  Zap,
  Lock,
  Shuffle,
  Play,
  CheckCircle2,
  Pencil,
  Trash,
  XCircle,
  Printer,
} from "lucide-svelte";

import ExamStatusBadge from "$lib/features/exams/components/status-badge.svelte";
import SubjectAddSheet from "$lib/features/exams/components/subject-add-sheet.svelte";
import PrintDialog from "$lib/shared/components/print-dialog.svelte";
import { commands } from "$lib/core/ipc";
import { authStore, aiContext } from "$lib/stores";

let { data }: { data: { orgId: string } } = $props();

// Extract sessionId from route params
const sessionId = $derived(page.params.sessionId);

// State
let session = $state<any>(null);
let subjects = $state<any[]>([]);
let schedules = $state<any[]>([]);
let seatingNumbers = $state<any[]>([]);
let printDialogOpen = $state(false);
let selectedSeatIds = $state<Set<string>>(new Set());

function toggleSeatSelection(id: string) {
  if (selectedSeatIds.has(id)) {
    selectedSeatIds.delete(id);
    selectedSeatIds = new Set(selectedSeatIds);
  } else {
    selectedSeatIds.add(id);
    selectedSeatIds = new Set(selectedSeatIds);
  }
}

function toggleSelectAllSeats() {
  if (selectedSeatIds.size === seatingNumbers.length) {
    selectedSeatIds = new Set();
  } else {
    selectedSeatIds = new Set(seatingNumbers.map((sn) => String(sn.id)));
  }
}

let halls = $state<any[]>([]);
let proctors = $state<any[]>([]);
let rooms = $state<any[]>([]);
let isLoading = $state(true);
let activeTab = $state("subjects");
let isSubjectSheetOpen = $state(false);

let activeJob = $state<any>(null);
let solveJobs = $state<any[]>([]);
let solveError = $state<string | null>(null);
let isSolving = $state(false);
let timeElapsed = $state(0);
let pollingInterval = $state<any>(null);
let elapsedTimer = $state<any>(null);

let classes = $state<any[]>([]);
let schoolSubjects = $state<any[]>([]);
let previewDraft = $state(false);

let isConfigModalOpen = $state(false);
let activeConfigTab = $state("schedule");

let preferredPeriod = $state("both"); // "both" | "morning" | "afternoon"
let maxExamsPerDay = $state(2);
let defaultDurationMinutes = $state(90);
let proctorsPerHall = $state(2);
let preventOwnSubject = $state(true);
let maxSessionsPerTeacher = $state(5);
let separateDivisions = $state(false);
let separateGenders = $state(false);
let balanceHalls = $state(true);

const draftSchedulesInGrid = $derived.by(() => {
  if (!previewDraft || !activeJob || !activeJob.best_solution) return [];
  try {
    const solution = JSON.parse(activeJob.best_solution);
    const assignments = solution.assignments || [];
    return assignments.map((a: any) => {
      const slotParts = a.slot_id ? a.slot_id.split("_") : [];
      const exam_date = slotParts[0] || "";
      const period = slotParts[1] || "";
      return {
        id: a.id,
        exam_date,
        period,
        exam_subject_id: a.exam_subject_id,
        applicable_class_ids: a.class_id,
        start_time: period === "morning" ? "08:00" : "13:00",
        end_time: (() => {
          const exam_subject = subjects.find((s: any) => s.id === a.exam_subject_id);
          const duration = exam_subject ? exam_subject.duration_minutes : 90;
          const start_time = period === "morning" ? "08:00" : "13:00";
          return addMinutes(start_time, duration);
        })(),
        isDraft: true,
      };
    });
  } catch (e) {
    console.error("Error parsing draft solution for exams:", e);
    return [];
  }
});

const activeSchedules = $derived(
  previewDraft ? draftSchedulesInGrid : schedules,
);

const sessionDates = $derived(
  session ? generateDatesRange(session.start_date, session.end_date) : [],
);

function getSubjectName(subjectId: string) {
  const sub = schoolSubjects.find((s: any) => String(s.id) === String(subjectId));
  return sub ? sub.name_ar || sub.name : subjectId;
}

function getClassName(classId: string) {
  const cls = classes.find((c) => String(c.id) === String(classId));
  return cls ? cls.name_ar || cls.name : classId;
}

function getSubjectColor(subjectId: string) {
  const sub = schoolSubjects.find((s: any) => String(s.id) === String(subjectId));
  return sub ? sub.color : "#3b82f6";
}

function formatDateWithDay(dateStr: string): string {
  try {
    const date = new Date(dateStr);
    const options: Intl.DateTimeFormatOptions = {
      weekday: "long",
      day: "numeric",
      month: "short",
    };
    return date.toLocaleDateString(
      m.academics_exams_sessions_sessionId_en_us(),
      options,
    );
  } catch (e) {
    return dateStr;
  }
}

function getExamsInCell(date: string, period: string) {
  return activeSchedules.filter(
    (s: any) =>
      String(s.exam_date) === String(date) &&
      String(s.period) === String(period),
  );
}

function addMinutes(timeStr: string, minutes: number): string {
  const parts = timeStr.split(":");
  if (parts.length < 2) return timeStr;
  const h = parseInt(parts[0], 10) || 0;
  const m = parseInt(parts[1], 10) || 0;
  const total = h * 60 + m + minutes;
  const rh = Math.floor(total / 60) % 24;
  const rm = total % 60;
  return `${String(rh).padStart(2, "0")}:${String(rm).padStart(2, "0")}`;
}

const tabs = $derived([
  {
    id: "subjects",
    label: m.common_exams_detail_subjects(),
    icon: BookOpen,
    count: subjects.length,
  },
  {
    id: "schedule",
    label: m.common_exams_detail_schedule(),
    icon: Calendar,
    count: activeSchedules.length,
  },
  {
    id: "seating",
    label: m.common_exams_detail_seating(),
    icon: Hash,
    count: seatingNumbers.length,
  },
  {
    id: "halls",
    label: m.common_exams_stat_halls(),
    icon: Building2,
    count: halls.length,
  },
  {
    id: "proctors",
    label: m.common_exams_stat_proctors(),
    icon: UserCheck,
    count: proctors.length,
  },
]);

$effect(() => {
  aiContext.setContext({
    domain: "exam",
    pageName: "Exam Session Details",
    recordId: sessionId,
    metadata: {
      sessionName: session?.name || "",
      status: session?.status || "",
      subjectsCount: subjects.length,
      schedulesCount: activeSchedules.length
    }
  });
  return () => aiContext.clearContext();
});

async function loadSession() {
  if (!data.orgId || !sessionId) return;
  isLoading = true;
  try {
    session = await commands.exam.getSession(sessionId as string);

    // Load subjects
    try {
      subjects = await commands.exam.getSubjects(sessionId);
    } catch (err) {
      console.error("Failed to load subjects", err);
    }

    // Load schedules
    try {
      schedules = await commands.exam.getSchedules(sessionId);
    } catch (err) {
      console.error("Failed to load schedules", err);
    }

    // Load seating numbers
    try {
      seatingNumbers = await commands.exam.getSeatingNumbers(sessionId,
      );
    } catch (err) {
      console.error("Failed to load seating numbers", err);
    }

    // Load halls
    try {
      halls = await commands.exam.getHalls(sessionId);
    } catch (err) {
      console.error("Failed to load halls", err);
    }

    // Load proctors
    try {
      proctors = await commands.exam.getProctors(sessionId);
    } catch (err) {
      console.error("Failed to load proctors", err);
    }

    // Load school rooms catalog
    try {
      rooms = await commands.scheduling.listRooms();
    } catch (err) {
      console.error("Failed to load school rooms", err);
    }

    // Load school classes & subjects catalogs
    try {
      const [classList, subjectList] = await Promise.all([
        commands.academic.listClasses(),
        commands.academic.listSubjects(),
      ]);
      classes = classList;
      schoolSubjects = subjectList;
    } catch (err) {
      console.error("Failed to load classes/subjects catalogs", err);
    }

    // Load active solve jobs for exams
    await loadSolveJobs();
  } catch (e) {
    toast.error("Failed to load session details");
    console.error(e);
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (data.orgId && sessionId) {
    loadSession();
  }
});

// Quick actions
async function handleGenerateSeating() {
  const userId = authStore.userId || authStore.user?.id || "desktop_user";
  const promise = commands.exam.generateSeating(sessionId as string, userId);
  toast.promise(promise, {
    loading: "جاري توليد أرقام الجلوس...",
    success: "تم توليد أرقام الجلوس بنجاح!",
    error: (err) => (err as any)?.message || "فشل توليد أرقام الجلوس",
  });
  try {
    await promise;
    await loadSession();
  } catch (err) {
    console.error(err);
  }
}

async function handleGenerateSecrets() {
  const userId = authStore.userId || authStore.user?.id || "desktop_user";
  const promise = commands.exam.generateSecrets(sessionId as string, userId);
  toast.promise(promise, {
    loading: "جاري توليد الأرقام السرية...",
    success: "تم توليد الأرقام السرية بنجاح!",
    error: (err) => (err as any)?.message || "فشل توليد الأرقام السرية",
  });
  try {
    await promise;
    await loadSession();
  } catch (err) {
    console.error(err);
  }
}

// Generate an array of YYYY-MM-DD strings between two dates inclusive
function generateDatesRange(startStr: string, endStr: string): string[] {
  const dates: string[] = [];
  try {
    const start = new Date(startStr);
    const end = new Date(endStr);
    for (let d = new Date(start); d <= end; d.setDate(d.getDate() + 1)) {
      dates.push(d.toISOString().split("T")[0]);
    }
  } catch (e) {
    console.error(e);
  }
  return dates;
}

async function loadSolveJobs() {
  if (!sessionId) return;
  try {
    const list = await commands.exam.listSolveJobs(sessionId);
    solveJobs = list;

    // Auto-resume polling if any job is running in background
    const running = list.find(
      (j: any) => j.status === "pending" || j.status === "running",
    );
    if (running) {
      activeJob = running;
      isSolving = true;
      startPollingJob(String(running.id));
    }
  } catch (e) {
    console.error("Error loading solve jobs:", e);
  }
}

async function handleAutoSchedule() {
  if (!session) return;
  const available_dates = generateDatesRange(
    session.start_date,
    session.end_date,
  );
  if (available_dates.length === 0) {
    toast.error("تاريخ البداية أو النهاية غير صالح");
    return;
  }

  isSolving = true;
  solveError = null;
  activeJob = null;

  try {
    const userId = authStore.userId || authStore.user?.id || "desktop_user";
    const req = {
      exam_session_id: sessionId,
      available_dates,
      max_exams_per_day: maxExamsPerDay,
      preferred_period: preferredPeriod,
      default_duration_minutes: defaultDurationMinutes,
    };
    const job = await commands.exam.startSolve(userId, req);
    activeJob = job;
    toast.success(
      m.common_schedule_ai_optimization_started(),
    );
    startPollingJob(String(job.id));
  } catch (e: any) {
    isSolving = false;
    solveError = (e as any)?.message || String(e);
    toast.error(solveError || "Error");
  }
}

async function startPollingJob(jobId: string) {
  if (pollingInterval) clearInterval(pollingInterval);
  if (elapsedTimer) clearInterval(elapsedTimer);

  solveError = null;
  timeElapsed = 0;
  elapsedTimer = setInterval(() => {
    timeElapsed++;
  }, 1000);

  pollingInterval = setInterval(async () => {
    try {
      const job = await commands.exam.getSolveJob(jobId);
      activeJob = job;

      if (
        job.status === "completed" ||
        job.status === "failed" ||
        job.status === "cancelled"
      ) {
        stopPolling();
        isSolving = false;
        await loadSolveJobs();
        if (job.status === "completed") {
          toast.success(
            m.academics_exams_sessions_sessionId_exam_schedule_optimization_completed_successfully(),
          );
        } else if (job.status === "failed") {
          solveError =
            (job.error_message as string) ||
            (m.common_schedule_ai_optimization_failed());
          toast.error(solveError || "Error");
        }
      }
    } catch (e) {
      console.error("Error polling solve job status:", e);
    }
  }, 2000);
}

function stopPolling() {
  if (pollingInterval) {
    clearInterval(pollingInterval);
    pollingInterval = null;
  }
  if (elapsedTimer) {
    clearInterval(elapsedTimer);
    elapsedTimer = null;
  }
}

async function handleCancelSolve() {
  if (!activeJob) return;
  try {
    const job = await commands.exam.cancelSolve(activeJob.id);
    activeJob = job;
    stopPolling();
    isSolving = false;
    toast.success(
      m.academics_exams_sessions_sessionId_optimization_job_cancelled(),
    );
    await loadSolveJobs();
  } catch (err: any) {
    toast.error((err as any)?.message || "Failed to cancel");
  }
}

async function handleApplySolve() {
  if (!activeJob) return;
  const promise = commands.exam.applySolve(activeJob.id);
  toast.promise(promise, {
    loading:
      m.academics_exams_sessions_sessionId_applying_optimized_schedule(),
    success:
      m.academics_exams_sessions_sessionId_applied_schedule_successfully(),
    error: (err: any) => (err as any)?.message || "Failed to apply",
  });
  try {
    await promise;
    activeJob = null;
    await loadSolveJobs();
    await loadSession();
  } catch (err) {
    console.error(err);
  }
}

async function handleDeleteSchedule(scheduleId: string) {
  if (
    !confirm(
      m.academics_exams_sessions_sessionId_are_you_sure_you_want(),
    )
  )
    return;
  const promise = commands.exam.deleteSchedule(scheduleId);
  toast.promise(promise, {
    loading:
      m.academics_exams_sessions_sessionId_deleting_scheduled_exam(),
    success:
      m.academics_exams_sessions_sessionId_deleted_exam_successfully(),
    error: (err: any) => (err as any)?.message || "Failed to delete",
  });
  try {
    await promise;
    await loadSession();
  } catch (err) {
    console.error(err);
  }
}

async function handleToggleScheduleStatus(sched: any) {
  const newStatus = sched.status === "cancelled" ? "scheduled" : "cancelled";
  const label =
    newStatus === "cancelled"
      ? m.academics_exams_sessions_sessionId_deactivating_exam()
      : m.academics_exams_sessions_sessionId_activating_exam();
  const labelSuccess =
    newStatus === "cancelled"
      ? m.academics_exams_sessions_sessionId_deactivated_successfully()
      : m.academics_exams_sessions_sessionId_activated_successfully();

  const promise = commands.exam.updateScheduleStatus(sched.id,
    newStatus,
  );
  toast.promise(promise, {
    loading: label,
    success: labelSuccess,
    error: (err: any) => (err as any)?.message || "Failed to update status",
  });
  try {
    await promise;
    await loadSession();
  } catch (err) {
    console.error(err);
  }
}

async function handleAllocateHalls() {
  const promise = commands.exam.allocateHalls(sessionId as string, {
    separate_divisions: separateDivisions,
    separate_genders: separateGenders,
    balance_halls: balanceHalls,
  });

  toast.promise(promise, {
    loading: "جاري توزيع الطلاب على القاعات...",
    success: "تم توزيع الطلاب على القاعات بنجاح!",
    error: (err: any) => (err as any)?.message || "فشل توزيع الطلاب",
  });

  try {
    await promise;
    await loadSession();
  } catch (err) {
    console.error(err);
  }
}

async function handleActivate() {
  if (!session) return;
  const promise = commands.exam.updateSession(sessionId as string, {
    status: "active",
  });

  toast.promise(promise, {
    loading: "جاري تفعيل دورة الامتحانات...",
    success: "تم تفعيل دورة الامتحانات بنجاح!",
    error: (err) => (err as any)?.message || "فشل تفعيل دورة الامتحانات",
  });

  try {
    await promise;
    await loadSession();
  } catch (err) {
    console.error(err);
  }
}

async function handleToggleHall(room: any) {
  const promise = commands.exam.createHall({
    exam_session_id: sessionId,
    room_id: room.id,
    hall_number: room.room_number || room.name || "لجنة",
    capacity: room.capacity || 30,
  });

  toast.promise(promise, {
    loading: `جاري تفعيل القاعة ${room.room_number || room.name}...`,
    success: "تم تفعيل القاعة كـ لجنة اختبار بنجاح!",
    error: (err) => (err as any)?.message || "فشل تفعيل القاعة",
  });

  try {
    await promise;
    await loadSession();
  } catch (err) {
    console.error(err);
  }
}

async function handleAssignProctors() {
  if (schedules.length === 0) {
    toast.error("يرجى جدولة الامتحانات أولاً قبل تعيين المراقبين");
    return;
  }

  let successCount = 0;
  isLoading = true;
  try {
    for (const sched of schedules) {
      await commands.exam.autoAssignProctors(sessionId as string, sched.id, {
        proctors_per_hall: proctorsPerHall,
        prevent_own_subject: preventOwnSubject,
        max_sessions_per_teacher: maxSessionsPerTeacher,
      });
      successCount++;
    }
    toast.success(
      `تم تعيين المراقبين بنجاح لـ ${successCount} فترات امتحانية!`,
    );
    await loadSession();
  } catch (err: any) {
    toast.error((err as any)?.message || "فشل تعيين المراقبين");
  } finally {
    isLoading = false;
  }
}
</script>

<div class="h-full w-full overflow-y-auto scrollbar-hide p-6 flex flex-col gap-5 select-none">
	{#if isLoading}
		<Skeleton class="h-20 w-full rounded-xl" />
		<Skeleton class="h-10 w-full rounded-lg" />
		<Skeleton class="h-64 w-full rounded-xl" />
	{:else}
		<!-- Back + Header -->
		<div class="flex flex-col gap-3" in:fade={{ duration: 150 }}>
			<Button
				href={localizeHref(`/${data.orgId}/academics/exams/sessions`)}
				variant="ghost"
				class="w-fit h-7 px-2 text-[10px] text-muted-foreground flex items-center gap-1 cursor-pointer"
			>
				<ArrowLeft size={12} />
				<span>{m.common_action_back()}</span>
			</Button>

			<!-- Session Info Banner -->
			<div class="card-tactile bg-card p-5 flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3">
				<div class="flex flex-col gap-1.5">
					<div class="flex items-center gap-2">
						<h1 class="text-sm font-bold text-foreground">
							{session?.name_ar || session?.name || `Session ${sessionId}`}
						</h1>
						<ExamStatusBadge status={session?.status || "draft"} />
					</div>
					{#if session}
						<div class="flex items-center gap-3 text-[10px] text-muted-foreground">
							<span class="flex items-center gap-1">
								<Calendar size={10} />
								{session.start_date} → {session.end_date}
							</span>
							<span class="px-1.5 py-0.5 rounded bg-muted/60 border border-border/40 text-[9px]">
								{session.session_type}
							</span>
						</div>
					{/if}
				</div>
				<div class="flex items-center gap-2">
					<Button
						variant="outline"
						class="h-7 px-3 text-[10px] flex items-center gap-1 cursor-pointer"
						onclick={() => toast.info("Edit sheet coming soon")}
					>
						<Pencil size={10} />
						{m.common_action_edit()}
					</Button>
					<Button
						variant="default"
						class="h-7 px-3 text-[10px] flex items-center gap-1 cursor-pointer"
						onclick={handleActivate}
					>
						<Play size={10} />
						{m.common_exams_detail_activate()}
					</Button>
				</div>
			</div>
		</div>

		<!-- Tabs -->
		<div class="flex items-center gap-1 border-b border-border pb-0" in:fade={{ delay: 50, duration: 200 }}>
			{#each tabs as tab (tab.id)}
				<button
					class="px-3 py-2 text-[10px] font-medium transition-all duration-150 border-b-2 cursor-pointer flex items-center gap-1.5
						{activeTab === tab.id
							? 'border-primary text-primary font-semibold'
							: 'border-transparent text-muted-foreground hover:text-foreground hover:border-border'}"
					onclick={() => activeTab = tab.id}
				>
					<tab.icon size={12} />
					<span>{tab.label}</span>
					{#if tab.count > 0}
						<span class="text-[8px] px-1 py-0.5 rounded bg-muted/60 font-mono">{tab.count}</span>
					{/if}
				</button>
			{/each}
		</div>

		<!-- Tab Content -->
		<div class="flex-1" in:fade={{ delay: 100, duration: 200 }}>

			<!-- ═══ Subjects Tab ═══ -->
			{#if activeTab === "subjects"}
				<div class="flex flex-col gap-4">
					{#if subjects.length === 0}
						<div class="flex flex-col items-center justify-center py-12 gap-3 text-center">
							<BookOpen size={24} class="text-muted-foreground" />
							<p class="text-xs text-muted-foreground">{m.academics_exams_no_subjects()}</p>
							<Button variant="outline" class="h-7 px-3 text-[10px] cursor-pointer" onclick={() => isSubjectSheetOpen = true}>
								<Plus size={12} class="me-1" />
								{m.common_exams_subjects_add()}
							</Button>
						</div>
					{:else}
						<div class="card-tactile bg-card overflow-hidden">
							<Table>
								<TableHeader>
									<TableRow class="text-[10px]">
										<TableHead class="w-10">#</TableHead>
										<TableHead>{m.common_word_subject()}</TableHead>
										<TableHead class="w-20">{m.common_exams_subjects_score()}</TableHead>
										<TableHead class="w-24">{m.common_exams_subjects_duration()}</TableHead>
										<TableHead class="w-24">{m.common_bulk_col_exam_type()}</TableHead>
									</TableRow>
								</TableHeader>
								<TableBody>
									{#each subjects as subject, i (subject.id)}
										<TableRow class="text-xs">
											<TableCell class="font-mono text-muted-foreground">{i + 1}</TableCell>
											<TableCell class="font-medium">{subject.subject_id}</TableCell>
											<TableCell class="font-mono">{subject.max_score}</TableCell>
											<TableCell class="font-mono">{subject.duration_minutes}{m.academics_exams_duration_minutes_abbr()}</TableCell>
											<TableCell>
												<span class="px-1.5 py-0.5 rounded bg-muted/60 border border-border/40 text-[9px]">
													{subject.exam_type}
												</span>
											</TableCell>
										</TableRow>
									{/each}
								</TableBody>
							</Table>
						</div>
						<Button variant="outline" class="w-fit h-7 px-3 text-[10px] cursor-pointer" onclick={() => isSubjectSheetOpen = true}>
							<Plus size={12} class="me-1" />
							{m.common_exams_subjects_add()}
						</Button>
					{/if}
				</div>

			<!-- ═══ Schedule Tab ═══ -->
			{:else if activeTab === "schedule"}
				<div class="flex flex-col gap-4">
					{#if activeSchedules.length === 0}
						<div class="flex flex-col items-center justify-center py-12 gap-3 text-center">
							<Calendar size={24} class="text-muted-foreground" />
							<p class="text-xs text-muted-foreground">{m.academics_exams_no_exams_scheduled()}</p>
							<Button variant="outline" class="h-7 px-3 text-[10px] cursor-pointer" onclick={handleAutoSchedule}>
								<Zap size={12} class="me-1" />
								{m.common_exams_actions_auto_schedule()}
							</Button>
						</div>
					{:else}
						<!-- Period-Based Exam Timetable Component with Premium Aesthetics -->
						<div class="timetable-container border rounded-2xl flex flex-col overflow-hidden shadow-sm select-none">
							<div 
								class="grid gap-px text-xs w-full bg-border-line"
								style="grid-template-columns: 180px 1fr 1fr;"
							>
								<!-- Header Row -->
								<div class="timetable-header p-4 font-bold text-center flex items-center justify-center border-b uppercase text-[10px] tracking-wider">
									{m.academics_exams_sessions_sessionId_day_date()}
								</div>
								<div class="timetable-header p-4 font-bold text-center flex items-center justify-center border-b uppercase text-[10px] tracking-wider">
									{m.academics_exams_sessions_sessionId_morning_period_08_00()}
								</div>
								<div class="timetable-header p-4 font-bold text-center flex items-center justify-center border-b uppercase text-[10px] tracking-wider">
									{m.academics_exams_sessions_sessionId_afternoon_period_13_00()}
								</div>

								<!-- Rows (Dates) -->
								{#each sessionDates as date}
									<!-- Date Cell -->
									<div class="timetable-period-info p-3 font-semibold text-center flex flex-col justify-center items-center gap-1 min-h-[90px] border-b bg-muted/10 border-e">
										<span class="text-[11px] tracking-wide font-extrabold text-foreground text-center">
											{formatDateWithDay(date)}
										</span>
									</div>

									<!-- Morning Cell -->
									<div class="timetable-cell p-3 text-start h-full min-h-[90px] relative flex flex-col justify-start items-stretch gap-2 border-b bg-card border-e">
										{#each getExamsInCell(date, "morning") as sched (sched.id)}
											<div 
												style="--subject-color: {getSubjectColor(sched.exam_subject_id)};"
												class="relative overflow-hidden w-full p-2.5 flex flex-row items-stretch justify-between gap-2.5 select-none transition-all duration-150 rounded-lg border
													{sched.isDraft ? 'border-dashed border-primary/50 bg-primary/2' : 'border-border/60 hover:border-primary/20 bg-card hover:bg-muted/10'} 
													{sched.status === 'cancelled' ? 'opacity-40 bg-muted/20 border-dashed border-muted-foreground/30 line-through' : ''}"
											>
												<div class="flex-1 flex flex-col justify-center items-start text-start gap-1 min-w-0">
													<div class="flex items-center gap-1.5 w-full">
														<span class="w-1.5 h-1.5 rounded-full bg-(--subject-color) shrink-0"></span>
														<span class="font-bold text-[11px] leading-tight text-foreground line-clamp-1">
															{getSubjectName(sched.exam_subject_id)}
														</span>
														{#if sched.isDraft}
															<span class="text-[8px] px-1 py-0.5 rounded bg-primary/10 text-primary font-bold">{m.common_status_draft()}</span>
														{/if}
													</div>
													
													<div class="text-[9px] text-muted-foreground mt-0.5 flex flex-wrap items-center gap-x-2 gap-y-0.5">
														<span class="font-bold text-primary">{getClassName(sched.applicable_class_ids)}</span>
														<span class="opacity-60">•</span>
														<span class="font-mono">{sched.start_time} - {sched.end_time}</span>
													</div>
												</div>

												{#if !sched.isDraft}
													<div class="flex items-center gap-1 shrink-0 self-center">
														<button 
															type="button" 
															class="text-muted-foreground hover:text-foreground p-1 rounded transition-colors duration-150 cursor-pointer"
															onclick={() => handleToggleScheduleStatus(sched)}
															title={sched.status === "cancelled" ? "إعادة تفعيل الامتحان" : "إلغاء تفعيل الامتحان"}
														>
															{#if sched.status === "cancelled"}
																<CheckCircle2 size={12} class="text-emerald-500 hover:text-emerald-600" />
															{:else}
																<XCircle size={12} class="text-amber-500 hover:text-amber-600" />
															{/if}
														</button>
														<button 
															type="button" 
															class="text-muted-foreground hover:text-destructive p-1 rounded transition-colors duration-150 cursor-pointer"
															onclick={() => handleDeleteSchedule(sched.id)}
															title="حذف الامتحان نهائياً"
														>
															<Trash size={12} />
														</button>
													</div>
												{/if}
											</div>
										{/each}
									</div>

									<!-- Afternoon Cell -->
									<div class="timetable-cell p-3 text-start h-full min-h-[90px] relative flex flex-col justify-start items-stretch gap-2 border-b bg-card">
										{#each getExamsInCell(date, "afternoon") as sched (sched.id)}
											<div 
												style="--subject-color: {getSubjectColor(sched.exam_subject_id)};"
												class="relative overflow-hidden w-full p-2.5 flex flex-row items-stretch justify-between gap-2.5 select-none transition-all duration-150 rounded-lg border
													{sched.isDraft ? 'border-dashed border-primary/50 bg-primary/2' : 'border-border/60 hover:border-primary/20 bg-card hover:bg-muted/10'} 
													{sched.status === 'cancelled' ? 'opacity-40 bg-muted/20 border-dashed border-muted-foreground/30 line-through' : ''}"
											>
												<div class="flex-1 flex flex-col justify-center items-start text-start gap-1 min-w-0">
													<div class="flex items-center gap-1.5 w-full">
														<span class="w-1.5 h-1.5 rounded-full bg-(--subject-color) shrink-0"></span>
														<span class="font-bold text-[11px] leading-tight text-foreground line-clamp-1">
															{getSubjectName(sched.exam_subject_id)}
														</span>
														{#if sched.isDraft}
															<span class="text-[8px] px-1 py-0.5 rounded bg-primary/10 text-primary font-bold">{m.common_status_draft()}</span>
														{/if}
													</div>
													
													<div class="text-[9px] text-muted-foreground mt-0.5 flex flex-wrap items-center gap-x-2 gap-y-0.5">
														<span class="font-bold text-primary">{getClassName(sched.applicable_class_ids)}</span>
														<span class="opacity-60">•</span>
														<span class="font-mono">{sched.start_time} - {sched.end_time}</span>
													</div>
												</div>

												{#if !sched.isDraft}
													<div class="flex items-center gap-1 shrink-0 self-center">
														<button 
															type="button" 
															class="text-muted-foreground hover:text-foreground p-1 rounded transition-colors duration-150 cursor-pointer"
															onclick={() => handleToggleScheduleStatus(sched)}
															title={sched.status === "cancelled" ? "إعادة تفعيل الامتحان" : "إلغاء تفعيل الامتحان"}
														>
															{#if sched.status === "cancelled"}
																<CheckCircle2 size={12} class="text-emerald-500 hover:text-emerald-600" />
															{:else}
																<XCircle size={12} class="text-amber-500 hover:text-amber-600" />
															{/if}
														</button>
														<button 
															type="button" 
															class="text-muted-foreground hover:text-destructive p-1 rounded transition-colors duration-150 cursor-pointer"
															onclick={() => handleDeleteSchedule(sched.id)}
															title="حذف الامتحان نهائياً"
														>
															<Trash size={12} />
														</button>
													</div>
												{/if}
											</div>
										{/each}
									</div>
								{/each}
							</div>
						</div>
					{/if}
				</div>

			<!-- ═══ Seating Tab ═══ -->
			{:else if activeTab === "seating"}
				<div class="flex flex-col gap-4">
					{#if seatingNumbers.length === 0}
						<div class="flex flex-col items-center justify-center py-12 gap-3 text-center">
							<Hash size={24} class="text-muted-foreground" />
							<p class="text-xs text-muted-foreground">{m.academics_exams_no_seating_numbers()}</p>
							<Button variant="default" class="h-7 px-3 text-[10px] cursor-pointer" onclick={handleGenerateSeating}>
								<Hash size={12} class="me-1" />
								{m.common_exams_actions_generate_seating()}
							</Button>
						</div>
					{:else}
						<div class="flex items-center justify-between gap-3 px-1 select-none">
							<span class="text-xs text-muted-foreground">{m.academics_exams_issued()}{seatingNumbers.length}{m.academics_exams_seating_numbers_suffix()}</span>
							{#if selectedSeatIds.size > 0}
								<Button 
									variant="outline" 
									class="h-7 px-3 text-[10px] flex items-center gap-1.5 cursor-pointer border-primary/30 text-primary"
									onclick={() => printDialogOpen = true}
								>
									<Printer size={12} />
									<span>{m.common_print_templates()} ({selectedSeatIds.size})</span>
								</Button>
							{/if}
						</div>
						<div class="card-tactile bg-card overflow-hidden">
							<Table>
								<TableHeader>
									<TableRow class="text-[10px]">
										<TableHead class="w-10 text-center">
											<input 
												type="checkbox" 
												checked={selectedSeatIds.size === seatingNumbers.length && seatingNumbers.length > 0} 
												onchange={toggleSelectAllSeats}
												class="rounded border-border text-primary focus:ring-primary w-3.5 h-3.5 cursor-pointer"
											/>
										</TableHead>
										<TableHead class="w-10">#</TableHead>
										<TableHead>{m.academics_exams_seat_number()}</TableHead>
										<TableHead>{m.common_word_student()}</TableHead>
										<TableHead>{m.common_table_class()}</TableHead>
										<TableHead>{m.academics_exams_level()}</TableHead>
									</TableRow>
								</TableHeader>
								<TableBody>
									{#each seatingNumbers as sn, i (sn.id)}
										<TableRow class="text-xs">
											<TableCell class="w-10 text-center" onclick={(e) => e.stopPropagation()}>
												<input 
													type="checkbox" 
													checked={selectedSeatIds.has(String(sn.id))} 
													onchange={() => toggleSeatSelection(String(sn.id))}
													class="rounded border-border text-primary focus:ring-primary w-3.5 h-3.5 cursor-pointer"
												/>
											</TableCell>
											<TableCell class="font-mono text-muted-foreground">{i + 1}</TableCell>
											<TableCell class="font-mono font-bold text-primary">{sn.seating_number}</TableCell>
											<TableCell>{sn.student_name_ar || sn.enrollment_id}</TableCell>
											<TableCell>{sn.class_id}</TableCell>
											<TableCell>{sn.level}</TableCell>
										</TableRow>
									{/each}
								</TableBody>
							</Table>
						</div>
					{/if}
				</div>

			<!-- ═══ Halls Tab ═══ -->
			{:else if activeTab === "halls"}
				<div class="flex flex-col gap-4">
					<div class="flex items-center justify-between text-xs text-muted-foreground select-none px-1">
						<span>{m.academics_exams_activated()}{halls.length}{m.academics_exams_committees_out_of()}{rooms.length}{m.academics_exams_available_classrooms_suffix()}</span>
						<Button variant="outline" class="h-6 px-2 text-[9px] cursor-pointer" onclick={() => { activeConfigTab = "halls"; isConfigModalOpen = true; }}>
							<Shuffle size={10} class="me-1" />
							توزيع الطلاب تلقائياً
						</Button>
					</div>

					{#if rooms.length === 0}
						<div class="flex flex-col items-center justify-center py-12 gap-3 text-center">
							<Building2 size={24} class="text-muted-foreground" />
							<p class="text-xs text-muted-foreground">{m.academics_exams_no_classrooms()}</p>
						</div>
					{:else}
						<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
							{#each rooms as room (room.id)}
								{@const activeHall = halls.find((h) => h.room_id === room.id)}
								{@const isAct = !!activeHall}
								<div class="card-tactile bg-card p-4 flex flex-col gap-3 transition-all duration-200 {isAct ? 'border-primary/40 shadow-sm ring-1 ring-primary/10' : 'opacity-85 hover:opacity-100'}">
									<div class="flex items-center justify-between">
										<div class="flex flex-col">
											<span class="text-xs font-bold text-foreground">{room.room_number || room.name}</span>
											{#if room.building}
												<span class="text-[8px] text-muted-foreground mt-0.5">{room.building}{m.academics_exams_floor_connector()}{room.floor}</span>
											{/if}
										</div>
										<span class="text-[9px] font-mono px-1.5 py-0.5 rounded {isAct ? 'bg-emerald-500/10 text-emerald-600 border border-emerald-500/20' : 'bg-muted text-muted-foreground border border-border/40'}">
											{isAct ? "لجنة نشطة" : "غير مفعلة"}
										</span>
									</div>

									{#if isAct}
										{@const utilPct = activeHall.capacity > 0 ? Math.round((activeHall.allocated_count / activeHall.capacity) * 100) : 0}
										<div class="flex flex-col gap-1.5">
											<div class="flex justify-between text-[9px] text-muted-foreground font-mono">
												<span>{m.academics_exams_allocated_capacity()}{activeHall.capacity}</span>
												<span>{m.academics_exams_allocated()}{activeHall.allocated_count}{m.academics_exams_student_suffix()}</span>
											</div>
											<div class="h-1 w-full rounded-full bg-muted/40 overflow-hidden">
												<div
													class="h-full rounded-full transition-all duration-300 {utilPct > 90 ? 'bg-red-500' : utilPct > 70 ? 'bg-amber-500' : 'bg-emerald-500'}"
													style="width: {utilPct}%"
												></div>
											</div>
										</div>
									{:else}
										<div class="flex items-center justify-between mt-1">
											<span class="text-[9px] text-muted-foreground">{m.academics_exams_default_capacity()}{room.capacity || 30}</span>
											<Button
												variant="ghost"
												class="h-6 px-2 text-[9px] font-semibold text-primary hover:bg-primary/5 cursor-pointer"
												onclick={() => handleToggleHall(room)}
											>
												<Plus size={10} class="me-1" />
												تفعيل كلجنة
											</Button>
										</div>
									{/if}
								</div>
							{/each}
						</div>
					{/if}
				</div>

			<!-- ═══ Proctors Tab ═══ -->
			{:else if activeTab === "proctors"}
				<div class="flex flex-col gap-4">
					{#if proctors.length === 0}
						<div class="flex flex-col items-center justify-center py-12 gap-3 text-center">
							<UserCheck size={24} class="text-muted-foreground" />
							<p class="text-xs text-muted-foreground">{m.academics_exams_no_proctors_assigned()}</p>
							<Button variant="outline" class="h-7 px-3 text-[10px] cursor-pointer" onclick={() => { activeConfigTab = "proctors"; isConfigModalOpen = true; }}>
								<UserCheck size={12} class="me-1" />
								{m.common_exams_actions_assign_proctors()}
							</Button>
						</div>
					{:else}
						<div class="card-tactile bg-card overflow-hidden">
							<Table>
								<TableHeader>
									<TableRow class="text-[10px]">
										<TableHead>{m.academics_exams_proctor()}</TableHead>
										<TableHead>{m.academics_exams_committee()}</TableHead>
										<TableHead>{m.common_user_form_role()}</TableHead>
										<TableHead class="w-20">{m.common_word_status()}</TableHead>
									</TableRow>
								</TableHeader>
								<TableBody>
									{#each proctors as p (p.id)}
										<TableRow class="text-xs">
											<TableCell class="font-medium">{p.teacher_id}</TableCell>
											<TableCell>{p.exam_hall_id}</TableCell>
											<TableCell>{p.role}</TableCell>
											<TableCell>
												{#if p.is_confirmed === 1}
													<CheckCircle2 size={14} class="text-emerald-500" />
												{:else}
													<span class="text-[9px] text-amber-500">{m.academics_exams_pending_confirmation()}</span>
												{/if}
											</TableCell>
										</TableRow>
									{/each}
								</TableBody>
							</Table>
						</div>
					{/if}
				</div>
			{/if}
		</div>

		{#if activeJob}
			<div class="border border-border rounded-xl p-4 bg-muted/40 mb-4 flex flex-col sm:flex-row sm:items-center justify-between gap-4" transition:fade>
				<div class="flex items-center gap-3">
					<div class="relative flex h-3 w-3">
						{#if activeJob.status === 'running'}
							<span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-sky-400 opacity-75"></span>
							<span class="relative inline-flex rounded-full h-3 w-3 bg-sky-500"></span>
						{:else if activeJob.status === 'completed'}
							<span class="relative inline-flex rounded-full h-3 w-3 bg-emerald-500"></span>
						{:else if activeJob.status === 'failed'}
							<span class="relative inline-flex rounded-full h-3 w-3 bg-rose-500"></span>
						{:else}
							<span class="relative inline-flex rounded-full h-3 w-3 bg-amber-500"></span>
						{/if}
					</div>
					<div>
						<h4 class="text-xs font-bold flex items-center gap-1.5">
							{#if activeJob.status === 'running'}
								جاري تحسين وجدولة الامتحانات تلقائياً...
							{:else if activeJob.status === 'completed'}
								اكتمل التحسين والجدولة التلقائية بنجاح!
							{:else if activeJob.status === 'failed'}
								فشلت عملية التحسين والجدولة.
							{:else}
								تم إلغاء الجدولة التلقائية.
							{/if}
						</h4>
						<p class="text-[10px] text-muted-foreground mt-0.5 font-mono">
							{#if activeJob.status === 'running'}
								الوقت المنقضي: {timeElapsed} ثانية | النتيجة الحالية: {activeJob.score || '—'}
							{:else if activeJob.status === 'completed'}
								النتيجة النهائية: {activeJob.score || '—'}
							{/if}
						</p>
					</div>
				</div>
				<div class="flex items-center gap-2">
					{#if activeJob.status === 'running'}
						<Button size="sm" variant="destructive" class="h-8 text-[10px] px-3 font-bold cursor-pointer" onclick={handleCancelSolve}>
							إلغاء العملية
						</Button>
					{:else if activeJob.status === 'completed'}
						<Button size="sm" variant={previewDraft ? "secondary" : "outline"} class="h-8 text-[10px] px-3 font-bold cursor-pointer" onclick={() => previewDraft = !previewDraft}>
							{previewDraft ? "إلغاء المعاينة" : "معاينة الجدول المقترح"}
						</Button>
						<Button size="sm" variant="default" class="h-8 text-[10px] px-3 font-bold cursor-pointer" onclick={handleApplySolve}>
							تطبيق الجدول المقترح
						</Button>
					{/if}
				</div>
			</div>
		{/if}

		<!-- Quick Actions Bar -->
		<div class="border-t border-border pt-4 flex flex-wrap items-center gap-2" in:fade={{ delay: 150, duration: 200 }}>
			<span class="text-[9px] text-muted-foreground font-bold uppercase tracking-wider me-2">{m.academics_exams_quick_actions()}</span>
			<Button variant="outline" class="h-7 px-2.5 text-[9px] flex items-center gap-1 cursor-pointer" onclick={handleGenerateSeating}>
				<Hash size={10} /> أرقام الجلوس
			</Button>
			<Button variant="outline" class="h-7 px-2.5 text-[9px] flex items-center gap-1 cursor-pointer" onclick={handleGenerateSecrets}>
				<Lock size={10} /> الأرقام السرية
			</Button>
			<Button variant="outline" class="h-7 px-2.5 text-[9px] flex items-center gap-1 cursor-pointer" onclick={() => { activeConfigTab = "schedule"; isConfigModalOpen = true; }}>
				<Zap size={10} /> جدولة تلقائية
			</Button>
			<Button variant="outline" class="h-7 px-2.5 text-[9px] flex items-center gap-1 cursor-pointer" onclick={() => { activeConfigTab = "halls"; isConfigModalOpen = true; }}>
				<Shuffle size={10} /> توزيع اللجان
			</Button>
		</div>
	{/if}
</div>

{#if isConfigModalOpen}
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div 
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4" 
		transition:fade={{ duration: 150 }}
		onclick={(e) => { if (e.target === e.currentTarget) isConfigModalOpen = false; }}
	>
		<div class="bg-card border border-border shadow-xl rounded-2xl w-full max-w-lg overflow-hidden flex flex-col max-h-[90vh] text-start">
			<!-- Modal Header -->
			<div class="border-b border-border p-4 flex items-center justify-between bg-muted/20">
				<div>
					<h3 class="text-sm font-bold text-foreground">{m.academics_exams_ai_wizard()}</h3>
					<p class="text-[10px] text-muted-foreground mt-0.5">{m.academics_exams_ai_customize()}</p>
				</div>
				<button 
					type="button" 
					class="text-muted-foreground hover:text-foreground cursor-pointer text-xs p-1"
					onclick={() => isConfigModalOpen = false}
				>
					✕
				</button>
			</div>

			<!-- Dialog Navigation Tabs -->
			<div class="flex border-b border-border bg-muted/10 p-1 gap-1">
				<button 
					type="button"
					class="flex-1 text-[11px] font-bold py-2 text-center rounded-lg transition-all cursor-pointer 
						{activeConfigTab === 'schedule' ? 'bg-card text-primary shadow-sm border border-border/40' : 'text-muted-foreground hover:text-foreground'}"
					onclick={() => activeConfigTab = "schedule"}
				>
					🗓️ جدولة الفترات
				</button>
				<button 
					type="button"
					class="flex-1 text-[11px] font-bold py-2 text-center rounded-lg transition-all cursor-pointer 
						{activeConfigTab === 'halls' ? 'bg-card text-primary shadow-sm border border-border/40' : 'text-muted-foreground hover:text-foreground'}"
					onclick={() => activeConfigTab = "halls"}
				>
					🏫 توزيع اللجان والطلاب
				</button>
				<button 
					type="button"
					class="flex-1 text-[11px] font-bold py-2 text-center rounded-lg transition-all cursor-pointer 
						{activeConfigTab === 'proctors' ? 'bg-card text-primary shadow-sm border border-border/40' : 'text-muted-foreground hover:text-foreground'}"
					onclick={() => activeConfigTab = "proctors"}
				>
					👮 تعيين المراقبين
				</button>
			</div>

			<!-- Tab Contents -->
			<div class="p-5 flex-1 overflow-y-auto flex flex-col gap-4 text-xs">
				{#if activeConfigTab === "schedule"}
					<!-- Period Solver settings -->
					<div class="flex flex-col gap-3.5">
						<div class="flex flex-col gap-1.5">
							<span class="font-bold text-foreground">{m.academics_exams_scheduling_preference()}</span>
							<select 
								bind:value={preferredPeriod}
								class="bg-background text-foreground border border-border rounded-lg px-3 py-2 outline-none focus:ring-1 focus:ring-primary w-full"
							>
								<option value="both">{m.academics_exams_two_sessions()}</option>
								<option value="morning">{m.academics_exams_morning_session()}</option>
								<option value="afternoon">{m.academics_exams_afternoon_session()}</option>
							</select>
						</div>

						<div class="flex flex-col gap-1.5">
							<span class="font-bold text-foreground">{m.academics_exams_max_exams_per_day()}</span>
							<select 
								bind:value={maxExamsPerDay}
								class="bg-background text-foreground border border-border rounded-lg px-3 py-2 outline-none focus:ring-1 focus:ring-primary w-full"
							>
								<option value={1}>{m.academics_exams_one_exam_per_day()}</option>
								<option value={2}>{m.academics_exams_two_exams_per_day()}</option>
							</select>
						</div>

						<div class="flex flex-col gap-1.5">
							<span class="font-bold text-foreground">{m.academics_exams_default_duration()}</span>
							<input 
								type="number"
								bind:value={defaultDurationMinutes}
								min="30"
								max="240"
								class="bg-background text-foreground border border-border rounded-lg px-3 py-2 outline-none focus:ring-1 focus:ring-primary w-full font-mono"
							/>
						</div>
					</div>

					<div class="mt-4 pt-4 border-t border-border flex items-center justify-end gap-2">
						<Button variant="outline" size="sm" class="cursor-pointer" onclick={() => isConfigModalOpen = false}>{m.common_action_cancel()}</Button>
						<Button variant="default" size="sm" class="cursor-pointer font-bold" onclick={() => { isConfigModalOpen = false; handleAutoSchedule(); }}>
							<Zap size={12} class="me-1" /> بدء الجدولة الذكية
						</Button>
					</div>

				{:else if activeConfigTab === "halls"}
					<!-- Halls allocation settings -->
					<div class="flex flex-col gap-3.5">
						<div class="flex items-center justify-between p-3 rounded-lg border border-border/60 bg-muted/5">
							<div class="flex flex-col gap-0.5 max-w-[80%]">
								<span class="font-bold text-foreground">{m.academics_exams_separate_major()}</span>
								<p class="text-[10px] text-muted-foreground">{m.academics_exams_allocate_different_tracks()}</p>
							</div>
							<input type="checkbox" bind:checked={separateDivisions} class="w-4 h-4 rounded border-border text-primary focus:ring-primary cursor-pointer" />
						</div>

						<div class="flex items-center justify-between p-3 rounded-lg border border-border/60 bg-muted/5">
							<div class="flex flex-col gap-0.5 max-w-[80%]">
								<span class="font-bold text-foreground">{m.academics_exams_separate_gender()}</span>
								<p class="text-[10px] text-muted-foreground">{m.academics_exams_allocate_genders_separate()}</p>
							</div>
							<input type="checkbox" bind:checked={separateGenders} class="w-4 h-4 rounded border-border text-primary focus:ring-primary cursor-pointer" />
						</div>

						<div class="flex items-center justify-between p-3 rounded-lg border border-border/60 bg-muted/5">
							<div class="flex flex-col gap-0.5 max-w-[80%]">
								<span class="font-bold text-foreground">{m.academics_exams_balance_student_numbers()}</span>
								<p class="text-[10px] text-muted-foreground">{m.academics_exams_balance_allocate_evenly()}</p>
							</div>
							<input type="checkbox" bind:checked={balanceHalls} class="w-4 h-4 rounded border-border text-primary focus:ring-primary cursor-pointer" />
						</div>
					</div>

					<div class="mt-4 pt-4 border-t border-border flex items-center justify-end gap-2">
						<Button variant="outline" size="sm" class="cursor-pointer" onclick={() => isConfigModalOpen = false}>{m.common_action_cancel()}</Button>
						<Button variant="default" size="sm" class="cursor-pointer font-bold" onclick={() => { isConfigModalOpen = false; handleAllocateHalls(); }}>
							<Shuffle size={12} class="me-1" /> بدء التوزيع التلقائي
						</Button>
					</div>

				{:else if activeConfigTab === "proctors"}
					<!-- Proctors assignment settings -->
					<div class="flex flex-col gap-3.5">
						<div class="flex flex-col gap-1.5">
							<span class="font-bold text-foreground">{m.academics_exams_required_proctors()}</span>
							<select 
								bind:value={proctorsPerHall}
								class="bg-background text-foreground border border-border rounded-lg px-3 py-2 outline-none focus:ring-1 focus:ring-primary w-full font-mono"
							>
								<option value={1}>{m.academics_exams_one_proctor()}</option>
								<option value={2}>{m.academics_exams_two_proctors()}</option>
								<option value={3}>{m.academics_exams_three_proctors()}</option>
							</select>
						</div>

						<div class="flex items-center justify-between p-3 rounded-lg border border-border/60 bg-muted/5">
							<div class="flex flex-col gap-0.5 max-w-[80%]">
								<span class="font-bold text-foreground">{m.academics_exams_prevent_proctoring_own_subject()}</span>
								<p class="text-[10px] text-muted-foreground">{m.academics_exams_ensure_no_proctoring_own_subject()}</p>
							</div>
							<input type="checkbox" bind:checked={preventOwnSubject} class="w-4 h-4 rounded border-border text-primary focus:ring-primary cursor-pointer" />
						</div>

						<div class="flex flex-col gap-1.5">
							<span class="font-bold text-foreground">{m.academics_exams_max_proctoring_sessions()}</span>
							<input 
								type="number"
								bind:value={maxSessionsPerTeacher}
								min="1"
								max="20"
								class="bg-background text-foreground border border-border rounded-lg px-3 py-2 outline-none focus:ring-1 focus:ring-primary w-full font-mono"
							/>
						</div>
					</div>

					<div class="mt-4 pt-4 border-t border-border flex items-center justify-end gap-2">
						<Button variant="outline" size="sm" class="cursor-pointer" onclick={() => isConfigModalOpen = false}>{m.common_action_cancel()}</Button>
						<Button variant="default" size="sm" class="cursor-pointer font-bold" onclick={() => { isConfigModalOpen = false; handleAssignProctors(); }}>
							<UserCheck size={12} class="me-1" /> تعيين المراقبين تلقائياً
						</Button>
					</div>
				{/if}
			</div>
		</div>
	</div>
{/if}

<SubjectAddSheet
	bind:open={isSubjectSheetOpen}
	orgId={data.orgId}
	sessionId={sessionId as string}
	onSuccess={loadSession}
/>

<PrintDialog
	bind:open={printDialogOpen}
	entityType="exam"
	entityIds={Array.from(selectedSeatIds)}
/>

