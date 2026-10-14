<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { formatDate } from "$lib/core/i18n";
import { Button } from "@orbis/ui/button";
import { Skeleton } from "@orbis/ui/skeleton";
import {
  Grid,
  TableHeader,
  TableRow,
  TableHead,
  TableBody,
  TableCell,
} from "@orbis/ui/table";
import { toast } from "@orbis/ui";
import { localizeHref } from "$lib/paraglide/runtime";
import { fade } from "svelte/transition";
import { ArrowLeft, ArrowsRightLeft, Bolt, BookOpen, Calendar, CheckCircle, Hashtag, Home, Lock, Pencil, Play, Plus, Printer, Trash, UserBadgeCheck, XMark } from "@orbis/icons";

import ExamStatusBadge from "$lib/features/exams/components/status-badge.svelte";
import SubjectAddSheet from "$lib/features/exams/components/subject-add-sheet.svelte";
import SessionCreateSheet from "$lib/features/exams/components/session-create-sheet.svelte";
import PrintDialog from "$lib/shared/components/print-dialog.svelte";
import { api } from "$lib/api";
import { aiContext, confirmStore } from "$lib/stores";

let { orgId, sessionId }: { orgId: string; sessionId: string } = $props();

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

const selectedSeatingContexts = $derived.by(() => {
  return seatingNumbers
    .filter((sn) => selectedSeatIds.has(String(sn.id)))
    .map((sn) => ({
      exam: {
        seatingNumber: sn.seating_number,
        hallName: sn.room_name ?? (sn as any).hall_name ?? "",
        deskNumber: (sn as any).desk_number ?? "",
        sessionName: session?.name ?? "",
      },
      student: {
        name: sn.student_name_ar || (sn as any).student_name || sn.enrollment_id || "",
        studentNumber: sn.seating_number,
        className: (sn as any).class_name ?? sn.class_id ?? "",
        level: sn.level ?? "",
      },
    }));
});

const selectedSeatingLabels = $derived.by(() => {
  return seatingNumbers
    .filter((sn) => selectedSeatIds.has(String(sn.id)))
    .map((sn) => `${sn.student_name_ar || sn.enrollment_id || ""} (${sn.seating_number})`);
});

let halls = $state<any[]>([]);
let proctors = $state<any[]>([]);
let rooms = $state<any[]>([]);
let isLoading = $state(true);
let activeTab = $state("subjects");
let isSubjectSheetOpen = $state(false);
let isEditSheetOpen = $state(false);

let activeJob = $state<any>(null);
let solveJobs = $state<any[]>([]);
let isSolving = $state(false);
let timeElapsed = $state(0);
let pollingInterval = $state<any>(null);
let elapsedTimer = $state<any>(null);

let classes = $state<any[]>([]);
let schoolSubjects = $state<any[]>([]);
let previewDraft = $state(false);

let isConfigModalOpen = $state(false);
let activeConfigTab = $state("schedule");

let preferredPeriod = $state("both");
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
          const exam_subject = subjects.find(
            (s: any) => s.id === a.exam_subject_id,
          );
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
  const sub = schoolSubjects.find(
    (s: any) => String(s.id) === String(subjectId),
  );
  return sub ? sub.name_ar || sub.name : subjectId;
}

function getClassName(classId: string) {
  const cls = classes.find((c) => String(c.id) === String(classId));
  return cls ? cls.name_ar || cls.name : classId;
}

function getSubjectColor(subjectId: string) {
  const sub = schoolSubjects.find(
    (s: any) => String(s.id) === String(subjectId),
  );
  return sub ? sub.color : "#3b82f6";
}

function formatDateWithDay(dateStr: string): string {
  return formatDate(dateStr, "date", {
    weekday: "long",
    day: "numeric",
    month: "short",
  }) || dateStr;
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
    icon: Hashtag,
    count: seatingNumbers.length,
  },
  {
    id: "halls",
    label: m.common_exams_stat_halls(),
    icon: Home,
    count: halls.length,
  },
  {
    id: "proctors",
    label: m.common_exams_stat_proctors(),
    icon: UserBadgeCheck,
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
      schedulesCount: activeSchedules.length,
    },
  });
  return () => aiContext.clearContext();
});

async function loadSession() {
  if (!orgId || !sessionId) return;
  isLoading = true;
  try {
    session = await api.getExamSession(sessionId);

    const [
      loadedSubjects,
      loadedSchedules,
      loadedSeatingNumbers,
      loadedHalls,
      loadedRooms,
      [classList, subjectList],
    ] = await Promise.all([
      Promise.resolve([]),
      api.listExamSchedules(sessionId).catch((err: unknown) => {
        console.error("Failed to load schedules", err);
        return [];
      }),
      api.listSeating(sessionId).catch((err: unknown) => {
        console.error("Failed to load seating numbers", err);
        return [];
      }),
      Promise.resolve([]),
      api.listRooms().catch((err: unknown) => {
        console.error("Failed to load school rooms", err);
        return [];
      }),
      Promise.all([api.listSections(), api.listSubjects()]).catch(
        (err: unknown) => {
          console.error("Failed to load classes/subjects catalogs", err);
          return [[], []];
        },
      ),
    ]);

    let loadedProctors: any[] = [];
    if (loadedSchedules.length > 0) {
      try {
        const proctorArrays = await Promise.all(
          loadedSchedules.map((s: any) =>
            api.listProctors(s.id).catch(() => []),
          ),
        );
        loadedProctors = proctorArrays.flat();
      } catch (e) {
        console.error("Failed to load proctors", e);
      }
    }

    subjects = loadedSubjects;
    schedules = loadedSchedules;
    seatingNumbers = loadedSeatingNumbers;
    halls = loadedHalls;
    proctors = loadedProctors;
    rooms = loadedRooms;
    classes = classList;
    schoolSubjects = subjectList;

    await loadSolveJobs();
  } catch (e) {
    toast.error("Failed to load session details");
    console.error(e);
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (orgId && sessionId) {
    loadSession();
  }
});

async function handleGenerateSeating() {
  try {
    const studentsRes = await api.listStudents({ pageSize: 500 });
    const studentIds = studentsRes.items.map((s: any) => s.student?.id || s.id);
    if (!studentIds.length) {
      toast.error(m.operations_ui_seating_no_students());
      return;
    }
    const roomIds = rooms.map((r: any) => r.id);

    const promise = api.generateSeating({
      exam_session_id: sessionId,
      student_ids: studentIds,
      room_ids: roomIds,
      prefix: "SEAT-",
      start_number: 1001,
    });

    toast.promise(promise, {
      loading: m.operations_ui_seating_generating(),
      success: m.operations_ui_seating_generate_success(),
      error: (err) => (err as any)?.message || m.operations_ui_seating_generate_error(),
    });

    await promise;
    await loadSession();
  } catch (err) {
    console.error("Failed to generate seating", err);
  }
}

async function handleGenerateSecrets() {
  try {
    if (!schedules.length) {
      toast.error(m.operations_ui_secrets_no_schedules());
      return;
    }
    const studentsRes = await api.listStudents({ pageSize: 500 });
    const studentIds = studentsRes.items.map((s: any) => s.student?.id || s.id);
    if (!studentIds.length) {
      toast.error(m.operations_ui_secrets_no_students());
      return;
    }

    const promises = schedules.map((schedule: any) =>
      api.generateSecrets({
        exam_schedule_id: schedule.id,
        student_ids: studentIds,
      }),
    );

    const promise = Promise.all(promises);
    toast.promise(promise, {
      loading: m.operations_ui_secrets_generating(),
      success: m.operations_ui_secrets_generate_success(),
      error: (err) => (err as any)?.message || m.operations_ui_secrets_generate_error(),
    });

    await promise;
    await loadSession();
  } catch (err) {
    console.error("Failed to generate secrets", err);
  }
}

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
  solveJobs = [];
  activeJob = null;
  isSolving = false;
}

async function handleAutoSchedule() {
  toast.error(m.operations_ui_auto_scheduling_desktop_unavailable());
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
  toast.error(m.operations_ui_cancel_scheduling_desktop_unavailable());
}

async function handleApplySolve() {
  toast.error(m.operations_ui_apply_scheduling_desktop_unavailable());
}

async function handleDeleteSchedule(scheduleId: string) {
  const confirmed = await confirmStore.prompt({
    title: m.operations_ui_delete_question_title(),
    description: m.academics_exams_sessions_sessionId_are_you_sure_you_want(),
    confirmText: m.common_action_delete(),
    variant: "destructive",
  });
  if (!confirmed) return;
  const promise = api.deleteExamSchedule(scheduleId);
  toast.promise(promise, {
    loading: m.academics_exams_sessions_sessionId_deleting_scheduled_exam(),
    success: m.academics_exams_sessions_sessionId_deleted_exam_successfully(),
    error: (err: any) => (err as any)?.message || m.operations_ui_exam_delete_error(),
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

  const promise = Promise.resolve();
  toast.promise(promise, {
    loading: label,
    success: labelSuccess,
    error: (err: any) => (err as any)?.message || m.operations_ui_exam_status_update_error(),
  });
  try {
    await promise;
    await loadSession();
  } catch (err) {
    console.error(err);
  }
}

async function handleAllocateHalls() {
  toast.error(m.operations_ui_hall_assignment_desktop_unavailable());
}

async function handleActivate() {
  if (!session) return;
  const targetStatus = session.status === "active" ? "completed" : "active";
  const promise = api.updateExamSession(sessionId, { status: targetStatus });

  toast.promise(promise, {
    loading: m.operations_ui_exam_session_status_updating(),
    success: m.operations_ui_exam_session_status_updated(),
    error: (err) => (err as any)?.message || m.operations_ui_exam_session_status_update_error(),
  });

  try {
    await promise;
    await loadSession();
  } catch (err) {
    console.error(err);
  }
}

async function handleDeleteSeating(seatingId: string) {
  const confirmed = await confirmStore.prompt({
    title: m.operations_ui_seating_delete_title(),
    description: m.operations_ui_seating_delete_confirm(),
    confirmText: m.common_action_delete(),
    variant: "destructive",
  });
  if (!confirmed) return;
  const promise = api.deleteSeating(seatingId);
  toast.promise(promise, {
    loading: m.operations_ui_seating_deleting(),
    success: m.operations_ui_seating_delete_success(),
    error: (err: any) => (err as any)?.message || m.operations_ui_seating_delete_error(),
  });
  try {
    await promise;
    await loadSession();
  } catch (err) {
    console.error(err);
  }
}

async function handleDeleteProctor(proctorId: string) {
  const confirmed = await confirmStore.prompt({
    title: m.operations_ui_proctor_assignment_cancel_title(),
    description: m.operations_ui_proctor_assignment_cancel_confirm(),
    confirmText: m.operations_ui_proctor_assignment_cancel(),
    variant: "destructive",
  });
  if (!confirmed) return;
  const promise = api.deleteProctor(proctorId);
  toast.promise(promise, {
    loading: m.operations_ui_proctor_assignment_cancelling(),
    success: m.operations_ui_proctor_assignment_cancel_success(),
    error: (err: any) => (err as any)?.message || m.operations_ui_proctor_assignment_cancel_error(),
  });
  try {
    await promise;
    await loadSession();
  } catch (err) {
    console.error(err);
  }
}

async function handleToggleHall(room: any) {
  const promise = Promise.resolve();

  toast.promise(promise, {
    loading: m.operations_ui_exam_room_activating({ room: room.room_number || room.name }),
    success: m.operations_ui_exam_room_activate_success(),
    error: (err) => (err as any)?.message || m.operations_ui_exam_room_activate_error(),
  });

  try {
    await promise;
    await loadSession();
  } catch (err) {
    console.error(err);
  }
}

async function handleAssignProctors() {
  toast.error(m.operations_ui_proctor_assignment_desktop_unavailable());
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
				href={localizeHref(`/${orgId}/academics/exams/sessions`)}
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
						onclick={() => isEditSheetOpen = true}
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
							<Grid>
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
							</Grid>
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
								<Bolt size={12} class="me-1" />
								{m.common_exams_actions_auto_schedule()}
							</Button>
						</div>
					{:else}
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
									<div class="timetable-period-info p-3 font-semibold text-center flex flex-col justify-center items-center gap-1 min-h-22.5 border-b bg-muted/10 border-e">
										<span class="text-[11px] tracking-wide font-extrabold text-foreground text-center">
											{formatDateWithDay(date)}
										</span>
									</div>

									<!-- Morning Cell -->
									<div class="timetable-cell p-3 text-start h-full min-h-22.5 relative flex flex-col justify-start items-stretch gap-2 border-b bg-card border-e">
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
															title={sched.status === "cancelled" ? m.operations_ui_reactivate_exam() : m.operations_ui_deactivate_exam()}
														>
															{#if sched.status === "cancelled"}
																<CheckCircle size={12} class="text-emerald-500 hover:text-emerald-600" />
															{:else}
																<XMark size={12} class="text-amber-500 hover:text-amber-600" />
															{/if}
														</button>
														<button 
															type="button" 
															class="text-muted-foreground hover:text-destructive p-1 rounded transition-colors duration-150 cursor-pointer"
															onclick={() => handleDeleteSchedule(sched.id)}
															title={m.operations_ui_delete_exam_permanently()}
														>
															<Trash size={12} />
														</button>
													</div>
												{/if}
											</div>
										{/each}
									</div>

									<!-- Afternoon Cell -->
									<div class="timetable-cell p-3 text-start h-full min-h-22.5 relative flex flex-col justify-start items-stretch gap-2 border-b bg-card">
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
															title={sched.status === "cancelled" ? m.operations_ui_reactivate_exam() : m.operations_ui_deactivate_exam()}
														>
															{#if sched.status === "cancelled"}
																<CheckCircle size={12} class="text-emerald-500 hover:text-emerald-600" />
															{:else}
																<XMark size={12} class="text-amber-500 hover:text-amber-600" />
															{/if}
														</button>
														<button 
															type="button" 
															class="text-muted-foreground hover:text-destructive p-1 rounded transition-colors duration-150 cursor-pointer"
															onclick={() => handleDeleteSchedule(sched.id)}
															title={m.operations_ui_delete_exam_permanently()}
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
							<Hashtag size={24} class="text-muted-foreground" />
							<p class="text-xs text-muted-foreground">{m.academics_exams_no_seating_numbers()}</p>
							<Button variant="default" class="h-7 px-3 text-[10px] cursor-pointer" onclick={handleGenerateSeating}>
								<Hashtag size={12} class="me-1" />
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
							<Grid>
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
										<TableHead class="w-12 text-center">{m.common_action_delete()}</TableHead>
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
											<TableCell class="text-center">
												<button
													type="button"
													class="text-muted-foreground hover:text-destructive p-1 rounded transition-colors cursor-pointer"
													onclick={() => handleDeleteSeating(sn.id)}
													title={m.operations_ui_delete_seating_number()}
												>
													<Trash size={12} />
												</button>
											</TableCell>
										</TableRow>
									{/each}
								</TableBody>
							</Grid>
						</div>
					{/if}
				</div>

			<!-- ═══ Halls Tab ═══ -->
			{:else if activeTab === "halls"}
				<div class="flex flex-col gap-4">
					<div class="flex items-center justify-between text-xs text-muted-foreground select-none px-1">
						<span>{m.academics_exams_activated()}{halls.length}{m.academics_exams_committees_out_of()}{rooms.length}{m.academics_exams_available_classrooms_suffix()}</span>
						<Button variant="outline" class="h-6 px-2 text-[9px] cursor-pointer" onclick={() => { activeConfigTab = "halls"; isConfigModalOpen = true; }}>
							<ArrowsRightLeft size={10} class="me-1" />
							{m.operations_ui_distribute_students_automatically()}
						</Button>
					</div>

					{#if rooms.length === 0}
						<div class="flex flex-col items-center justify-center py-12 gap-3 text-center">
							<Home size={24} class="text-muted-foreground" />
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
											{isAct ? m.operations_ui_active_committee() : m.operations_ui_inactive()}
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
												{m.operations_ui_activate_as_committee()}
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
							<UserBadgeCheck size={24} class="text-muted-foreground" />
							<p class="text-xs text-muted-foreground">{m.academics_exams_no_proctors_assigned()}</p>
							<Button variant="outline" class="h-7 px-3 text-[10px] cursor-pointer" onclick={() => { activeConfigTab = "proctors"; isConfigModalOpen = true; }}>
								<UserBadgeCheck size={12} class="me-1" />
								{m.common_exams_actions_assign_proctors()}
							</Button>
						</div>
					{:else}
						<div class="card-tactile bg-card overflow-hidden">
							<Grid>
								<TableHeader>
									<TableRow class="text-[10px]">
										<TableHead>{m.academics_exams_proctor()}</TableHead>
										<TableHead>{m.academics_exams_committee()}</TableHead>
										<TableHead>{m.common_user_form_role()}</TableHead>
										<TableHead class="w-20">{m.common_word_status()}</TableHead>
										<TableHead class="w-12 text-center">{m.common_action_delete()}</TableHead>
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
													<CheckCircle size={14} class="text-emerald-500" />
												{:else}
													<span class="text-[9px] text-amber-500">{m.academics_exams_pending_confirmation()}</span>
												{/if}
											</TableCell>
											<TableCell class="text-center">
												<button
													type="button"
													class="text-muted-foreground hover:text-destructive p-1 rounded transition-colors cursor-pointer"
													onclick={() => handleDeleteProctor(p.id)}
													title={m.operations_ui_cancel_proctor_assignment()}
												>
													<Trash size={12} />
												</button>
											</TableCell>
										</TableRow>
									{/each}
								</TableBody>
							</Grid>
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
								{m.operations_ui_auto_schedule_optimizing()}
							{:else if activeJob.status === 'completed'}
								{m.operations_ui_auto_schedule_success()}
							{:else if activeJob.status === 'failed'}
								{m.operations_ui_auto_schedule_failure()}
							{:else}
								{m.operations_ui_auto_schedule_cancelled()}
							{/if}
						</h4>
						<p class="text-[10px] text-muted-foreground mt-0.5 font-mono">
							{#if activeJob.status === 'running'}
								{m.operations_ui_scheduling_elapsed_score({ elapsed: timeElapsed, score: activeJob.score || "—" })}
							{:else if activeJob.status === 'completed'}
								{m.operations_ui_scheduling_final_score({ score: activeJob.score || "—" })}
							{/if}
						</p>
					</div>
				</div>
				<div class="flex items-center gap-2">
					{#if activeJob.status === 'running'}
						<Button size="sm" variant="destructive" class="h-8 text-[10px] px-3 font-bold cursor-pointer" onclick={handleCancelSolve}>
							{m.operations_ui_cancel_operation()}
						</Button>
					{:else if activeJob.status === 'completed'}
						<Button size="sm" variant={previewDraft ? "secondary" : "outline"} class="h-8 text-[10px] px-3 font-bold cursor-pointer" onclick={() => previewDraft = !previewDraft}>
							{previewDraft ? m.operations_ui_cancel_preview() : m.operations_ui_preview_proposed_schedule()}
						</Button>
						<Button size="sm" variant="default" class="h-8 text-[10px] px-3 font-bold cursor-pointer" onclick={handleApplySolve}>
							{m.operations_ui_apply_proposed_schedule()}
						</Button>
					{/if}
				</div>
			</div>
		{/if}

		<!-- Quick Actions Bar -->
		<div class="border-t border-border pt-4 flex flex-wrap items-center gap-2" in:fade={{ delay: 150, duration: 200 }}>
			<span class="text-[9px] text-muted-foreground font-bold uppercase tracking-wider me-2">{m.academics_exams_quick_actions()}</span>
			<Button variant="outline" class="h-7 px-2.5 text-[9px] flex items-center gap-1 cursor-pointer" onclick={handleGenerateSeating}>
				<Hashtag size={10} /> {m.operations_ui_seating_numbers()}
			</Button>
			<Button variant="outline" class="h-7 px-2.5 text-[9px] flex items-center gap-1 cursor-pointer" onclick={handleGenerateSecrets}>
				<Lock size={10} /> {m.operations_ui_secret_numbers()}
			</Button>
			<Button variant="outline" class="h-7 px-2.5 text-[9px] flex items-center gap-1 cursor-pointer" onclick={() => { activeConfigTab = "schedule"; isConfigModalOpen = true; }}>
				<Bolt size={10} /> {m.operations_ui_automatic_scheduling()}
			</Button>
			<Button variant="outline" class="h-7 px-2.5 text-[9px] flex items-center gap-1 cursor-pointer" onclick={() => { activeConfigTab = "halls"; isConfigModalOpen = true; }}>
				<ArrowsRightLeft size={10} /> {m.operations_ui_committee_distribution()}
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
					🗓️ {m.operations_ui_schedule_periods()}
				</button>
				<button 
					type="button" 
					class="flex-1 text-[11px] font-bold py-2 text-center rounded-lg transition-all cursor-pointer 
						{activeConfigTab === 'halls' ? 'bg-card text-primary shadow-sm border border-border/40' : 'text-muted-foreground hover:text-foreground'}"
					onclick={() => activeConfigTab = "halls"}
				>
					🏫 {m.operations_ui_distribute_committees_students()}
				</button>
				<button 
					type="button" 
					class="flex-1 text-[11px] font-bold py-2 text-center rounded-lg transition-all cursor-pointer 
						{activeConfigTab === 'proctors' ? 'bg-card text-primary shadow-sm border border-border/40' : 'text-muted-foreground hover:text-foreground'}"
					onclick={() => activeConfigTab = "proctors"}
				>
					👮 {m.operations_ui_assign_proctors()}
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
							<Bolt size={12} class="me-1" /> {m.operations_ui_start_smart_scheduling()}
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
							<ArrowsRightLeft size={12} class="me-1" /> {m.operations_ui_start_automatic_distribution()}
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
							<UserBadgeCheck size={12} class="me-1" /> {m.operations_ui_assign_proctors_automatically()}
						</Button>
					</div>
				{/if}
			</div>
		</div>
	</div>
{/if}

<SubjectAddSheet
	bind:open={isSubjectSheetOpen}
	{orgId}
	{sessionId}
	onSuccess={loadSession}
/>

<SessionCreateSheet
	bind:open={isEditSheetOpen}
	{orgId}
	{session}
	onSuccess={loadSession}
/>

<PrintDialog
	bind:open={printDialogOpen}
	entityType="exam"
	documentType="EXAM_CARD"
	entityIds={Array.from(selectedSeatIds)}
	entityLabels={selectedSeatingLabels}
	contexts={selectedSeatingContexts}
/>
