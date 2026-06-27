<!--
@component ExamSessionDetail
Session detail page with tabs: Subjects, Schedule, Seating, Halls, Proctors.
-->
<script lang="ts">
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
} from "lucide-svelte";
import * as m from "$lib/paraglide/messages";
import ExamStatusBadge from "$lib/components/exams/status-badge.svelte";
import SubjectAddSheet from "$lib/components/exams/subject-add-sheet.svelte";
import { commands } from "$lib/services/ipc";
import { authStore } from "$lib/stores";

let { data }: { data: { orgId: string } } = $props();

// Extract sessionId from route params
const sessionId = $derived(page.params.sessionId);

// State
let session = $state<any>(null);
let subjects = $state<any[]>([]);
let schedules = $state<any[]>([]);
let seatingNumbers = $state<any[]>([]);
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

const tabs = $derived([
  { id: "subjects", label: m.exams_detail_subjects(), icon: BookOpen, count: subjects.length },
  { id: "schedule", label: m.exams_detail_schedule(), icon: Calendar, count: schedules.length },
  { id: "seating", label: m.exams_detail_seating(), icon: Hash, count: seatingNumbers.length },
  { id: "halls", label: m.exams_detail_halls(), icon: Building2, count: halls.length },
  { id: "proctors", label: m.exams_detail_proctors(), icon: UserCheck, count: proctors.length },
]);

async function loadSession() {
  if (!data.orgId || !sessionId) return;
  isLoading = true;
  try {
    session = await commands.exams.getSession(data.orgId, sessionId);
    
    // Load subjects
    try {
      subjects = await commands.exams.getSubjects(data.orgId, sessionId);
    } catch (err) {
      console.error("Failed to load subjects", err);
    }

    // Load schedules
    try {
      schedules = await commands.exams.getSchedules(data.orgId, sessionId);
    } catch (err) {
      console.error("Failed to load schedules", err);
    }

    // Load seating numbers
    try {
      seatingNumbers = await commands.exams.getSeatingNumbers(data.orgId, sessionId);
    } catch (err) {
      console.error("Failed to load seating numbers", err);
    }

    // Load halls
    try {
      halls = await commands.exams.getHalls(data.orgId, sessionId);
    } catch (err) {
      console.error("Failed to load halls", err);
    }

    // Load proctors
    try {
      proctors = await commands.exams.getProctors(data.orgId, sessionId);
    } catch (err) {
      console.error("Failed to load proctors", err);
    }

    // Load school rooms catalog
    try {
      rooms = await commands.schedule.getRooms(data.orgId);
    } catch (err) {
      console.error("Failed to load school rooms", err);
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
  const promise = commands.exams.generateSeating(data.orgId, sessionId, userId);
  toast.promise(promise, {
    loading: "جاري توليد أرقام الجلوس...",
    success: "تم توليد أرقام الجلوس بنجاح!",
    error: (err) => err?.message || "فشل توليد أرقام الجلوس",
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
  const promise = commands.exams.generateSecrets(data.orgId, sessionId, userId);
  toast.promise(promise, {
    loading: "جاري توليد الأرقام السرية...",
    success: "تم توليد الأرقام السرية بنجاح!",
    error: (err) => err?.message || "فشل توليد الأرقام السرية",
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
    const list = await commands.exams.listSolveJobs(data.orgId, sessionId);
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
  const available_dates = generateDatesRange(session.start_date, session.end_date);
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
      max_exams_per_day: 2,
      preferred_period: "both",
      default_duration_minutes: 90,
    };
    const job = await commands.exams.startSolve(data.orgId, userId, req);
    activeJob = job;
    toast.success(
      getLocale() === "ar"
        ? "تم بدء الجدولة التلقائية في الخلفية"
        : "Started auto-scheduling in the background",
    );
    startPollingJob(String(job.id));
  } catch (e: any) {
    isSolving = false;
    solveError = e?.message || String(e);
    toast.error(solveError);
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
      const job = await commands.exams.getSolveJob(data.orgId, jobId);
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
            getLocale() === "ar"
              ? "اكتملت عملية الجدولة التلقائية بنجاح!"
              : "Exam schedule optimization completed successfully!",
          );
        } else if (job.status === "failed") {
          solveError =
            (job.error_message as string) ||
            (getLocale() === "ar"
              ? "فشلت عملية الجدولة"
              : "Optimization job failed");
          toast.error(solveError);
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
    const job = await commands.exams.cancelSolve(data.orgId, activeJob.id);
    activeJob = job;
    stopPolling();
    isSolving = false;
    toast.success(
      getLocale() === "ar"
        ? "تم إلغاء عملية الجدولة"
        : "Optimization job cancelled",
    );
    await loadSolveJobs();
  } catch (err: any) {
    toast.error(err?.message || "Failed to cancel");
  }
}

async function handleApplySolve() {
  if (!activeJob) return;
  const promise = commands.exams.applySolve(data.orgId, activeJob.id);
  toast.promise(promise, {
    loading:
      getLocale() === "ar"
        ? "جاري تطبيق الجدول المحسّن..."
        : "Applying optimized schedule...",
    success:
      getLocale() === "ar"
        ? "تم تطبيق الجدول بنجاح!"
        : "Applied schedule successfully!",
    error: (err: any) => err?.message || "Failed to apply",
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

async function handleAllocateHalls() {
  const promise = commands.exams.allocateHalls(data.orgId, sessionId, {
    separate_divisions: false,
    separate_genders: false,
    balance_halls: true,
  });

  toast.promise(promise, {
    loading: "جاري توزيع الطلاب على القاعات...",
    success: "تم توزيع الطلاب على القاعات بنجاح!",
    error: (err) => err?.message || "فشل توزيع الطلاب",
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
  const promise = commands.exams.updateSession(data.orgId, sessionId, {
    status: "active"
  });

  toast.promise(promise, {
    loading: "جاري تفعيل دورة الامتحانات...",
    success: "تم تفعيل دورة الامتحانات بنجاح!",
    error: (err) => err?.message || "فشل تفعيل دورة الامتحانات",
  });

  try {
    await promise;
    await loadSession();
  } catch (err) {
    console.error(err);
  }
}

async function handleToggleHall(room: any) {
  const promise = commands.exams.createHall(data.orgId, {
    exam_session_id: sessionId,
    room_id: room.id,
    hall_number: room.room_number || room.name || "لجنة",
    capacity: room.capacity || 30,
  });

  toast.promise(promise, {
    loading: `جاري تفعيل القاعة ${room.room_number || room.name}...`,
    success: "تم تفعيل القاعة كـ لجنة اختبار بنجاح!",
    error: (err) => err?.message || "فشل تفعيل القاعة",
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
      await commands.exams.autoAssignProctors(data.orgId, sessionId, sched.id, {
        proctors_per_hall: 2,
        prevent_own_subject: true,
        max_sessions_per_teacher: 5,
      });
      successCount++;
    }
    toast.success(`تم تعيين المراقبين بنجاح لـ ${successCount} فترات امتحانية!`);
    await loadSession();
  } catch (err: any) {
    toast.error(err?.message || "فشل تعيين المراقبين");
  } finally {
    isLoading = false;
  }
}
</script>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-5 select-none">
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
				<span>{m.exams_detail_back()}</span>
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
						{m.exams_detail_edit()}
					</Button>
					<Button
						variant="default"
						class="h-7 px-3 text-[10px] flex items-center gap-1 cursor-pointer"
						onclick={handleActivate}
					>
						<Play size={10} />
						{m.exams_detail_activate()}
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
							<p class="text-xs text-muted-foreground">لم يتم إضافة مواد بعد</p>
							<Button variant="outline" class="h-7 px-3 text-[10px] cursor-pointer" onclick={() => isSubjectSheetOpen = true}>
								<Plus size={12} class="me-1" />
								{m.exams_subjects_add()}
							</Button>
						</div>
					{:else}
						<div class="card-tactile bg-card overflow-hidden">
							<Table>
								<TableHeader>
									<TableRow class="text-[10px]">
										<TableHead class="w-10">#</TableHead>
										<TableHead>{m.exams_subjects_name()}</TableHead>
										<TableHead class="w-20">{m.exams_subjects_score()}</TableHead>
										<TableHead class="w-24">{m.exams_subjects_duration()}</TableHead>
										<TableHead class="w-24">{m.exams_subjects_type()}</TableHead>
									</TableRow>
								</TableHeader>
								<TableBody>
									{#each subjects as subject, i (subject.id)}
										<TableRow class="text-xs">
											<TableCell class="font-mono text-muted-foreground">{i + 1}</TableCell>
											<TableCell class="font-medium">{subject.subject_id}</TableCell>
											<TableCell class="font-mono">{subject.max_score}</TableCell>
											<TableCell class="font-mono">{subject.duration_minutes}د</TableCell>
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
							{m.exams_subjects_add()}
						</Button>
					{/if}
				</div>

			<!-- ═══ Schedule Tab ═══ -->
			{:else if activeTab === "schedule"}
				<div class="flex flex-col gap-4">
					{#if schedules.length === 0}
						<div class="flex flex-col items-center justify-center py-12 gap-3 text-center">
							<Calendar size={24} class="text-muted-foreground" />
							<p class="text-xs text-muted-foreground">لم يتم جدولة امتحانات بعد</p>
							<Button variant="outline" class="h-7 px-3 text-[10px] cursor-pointer" onclick={handleAutoSchedule}>
								<Zap size={12} class="me-1" />
								{m.exams_actions_auto_schedule()}
							</Button>
						</div>
					{:else}
						<!-- Timeline Grid -->
						<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
							{#each schedules as sched (sched.id)}
								<div class="card-tactile bg-card p-4 flex flex-col gap-2">
									<div class="flex items-center justify-between">
										<span class="text-[10px] font-bold text-foreground">{sched.exam_date}</span>
										<span class="text-[9px] px-1.5 py-0.5 rounded {sched.period === 'morning' ? 'bg-sky-500/10 text-sky-600' : 'bg-violet-500/10 text-violet-600'}">
											{sched.period === 'morning' ? '☀️ صباحي' : '🌙 مسائي'}
										</span>
									</div>
									<div class="text-xs font-medium text-foreground">{sched.exam_subject_id}</div>
									<div class="text-[9px] text-muted-foreground font-mono">
										{sched.start_time} → {sched.end_time}
									</div>
								</div>
							{/each}
						</div>
					{/if}
				</div>

			<!-- ═══ Seating Tab ═══ -->
			{:else if activeTab === "seating"}
				<div class="flex flex-col gap-4">
					{#if seatingNumbers.length === 0}
						<div class="flex flex-col items-center justify-center py-12 gap-3 text-center">
							<Hash size={24} class="text-muted-foreground" />
							<p class="text-xs text-muted-foreground">لم يتم إصدار أرقام الجلوس بعد</p>
							<Button variant="default" class="h-7 px-3 text-[10px] cursor-pointer" onclick={handleGenerateSeating}>
								<Hash size={12} class="me-1" />
								{m.exams_actions_generate_seating()}
							</Button>
						</div>
					{:else}
						<div class="card-tactile bg-card overflow-hidden">
							<Table>
								<TableHeader>
									<TableRow class="text-[10px]">
										<TableHead class="w-10">#</TableHead>
										<TableHead>رقم الجلوس</TableHead>
										<TableHead>الطالب</TableHead>
										<TableHead>الفصل</TableHead>
										<TableHead>المستوى</TableHead>
									</TableRow>
								</TableHeader>
								<TableBody>
									{#each seatingNumbers as sn, i (sn.id)}
										<TableRow class="text-xs">
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
						<span>تم تفعيل {halls.length} لجنة من أصل {rooms.length} قاعة دراسية متاحة بالمدرسة</span>
						<Button variant="outline" class="h-6 px-2 text-[9px] cursor-pointer" onclick={handleAllocateHalls}>
							<Shuffle size={10} class="me-1" />
							توزيع الطلاب تلقائياً
						</Button>
					</div>

					{#if rooms.length === 0}
						<div class="flex flex-col items-center justify-center py-12 gap-3 text-center">
							<Building2 size={24} class="text-muted-foreground" />
							<p class="text-xs text-muted-foreground">لا توجد قاعات مسجلة في نظام المدرسة بعد</p>
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
												<span class="text-[8px] text-muted-foreground mt-0.5">{room.building} - الطابق {room.floor}</span>
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
												<span>السعة المحددة: {activeHall.capacity}</span>
												<span>الموزعين: {activeHall.allocated_count} طالب</span>
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
											<span class="text-[9px] text-muted-foreground">السعة الافتراضية: {room.capacity || 30}</span>
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
							<p class="text-xs text-muted-foreground">لم يتم تعيين مراقبين بعد</p>
							<Button variant="outline" class="h-7 px-3 text-[10px] cursor-pointer" onclick={handleAssignProctors}>
								<UserCheck size={12} class="me-1" />
								{m.exams_actions_assign_proctors()}
							</Button>
						</div>
					{:else}
						<div class="card-tactile bg-card overflow-hidden">
							<Table>
								<TableHeader>
									<TableRow class="text-[10px]">
										<TableHead>المراقب</TableHead>
										<TableHead>اللجنة</TableHead>
										<TableHead>الدور</TableHead>
										<TableHead class="w-20">الحالة</TableHead>
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
													<span class="text-[9px] text-amber-500">قيد التأكيد</span>
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
						<Button size="sm" variant="default" class="h-8 text-[10px] px-3 font-bold cursor-pointer" onclick={handleApplySolve}>
							تطبيق الجدول المقترح
						</Button>
					{/if}
				</div>
			</div>
		{/if}

		<!-- Quick Actions Bar -->
		<div class="border-t border-border pt-4 flex flex-wrap items-center gap-2" in:fade={{ delay: 150, duration: 200 }}>
			<span class="text-[9px] text-muted-foreground font-bold uppercase tracking-wider me-2">إجراءات سريعة</span>
			<Button variant="outline" class="h-7 px-2.5 text-[9px] flex items-center gap-1 cursor-pointer" onclick={handleGenerateSeating}>
				<Hash size={10} /> أرقام الجلوس
			</Button>
			<Button variant="outline" class="h-7 px-2.5 text-[9px] flex items-center gap-1 cursor-pointer" onclick={handleGenerateSecrets}>
				<Lock size={10} /> الأرقام السرية
			</Button>
			<Button variant="outline" class="h-7 px-2.5 text-[9px] flex items-center gap-1 cursor-pointer" onclick={handleAutoSchedule}>
				<Zap size={10} /> جدولة تلقائية
			</Button>
			<Button variant="outline" class="h-7 px-2.5 text-[9px] flex items-center gap-1 cursor-pointer" onclick={handleAllocateHalls}>
				<Shuffle size={10} /> توزيع اللجان
			</Button>
		</div>
	{/if}
</div>

<SubjectAddSheet
	bind:open={isSubjectSheetOpen}
	orgId={data.orgId}
	sessionId={sessionId}
	onSuccess={loadSession}
/>

