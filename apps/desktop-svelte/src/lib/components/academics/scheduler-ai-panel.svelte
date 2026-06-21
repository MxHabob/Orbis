<!--
@component SchedulerAiPanel
Highly-customizable, resizable sidebar panel for managing AI Timetable optimization (Timefold solver).
Includes constraints tuning, direct weekly subjects hours management, polling, and draft preview mode.
-->
<script lang="ts">
import { commands } from "$lib/services/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { toast } from "svelte-sonner";
import { getLocale } from "$lib/paraglide/runtime";
import { fade } from "svelte/transition";
import {
  Sparkles,
  Cpu,
  Clock,
  Square,
  CheckCircle2,
  AlertCircle,
  History,
  BookOpen,
} from "lucide-svelte";
import { authStore } from "$lib/stores/auth.svelte";
import Resizer from "$lib/components/layout/Resizer.svelte";

let {
  orgId,
  cycleId,
  selectedClassId,
  selectedTeacherId,
  viewMode,
  previewDraft = $bindable(false),
  activeJob = $bindable(null),
  subjects = [],
  onApply,
}: {
  orgId: string;
  cycleId: string;
  selectedClassId: string;
  selectedTeacherId: string;
  viewMode: "class" | "teacher";
  previewDraft: boolean;
  activeJob: any;
  subjects: any[];
  onApply: () => void;
} = $props();

// Resizer State
let preferredWidth = $state(20);
let panelEl = $state<HTMLElement | undefined>(undefined);
const minWidth = 18;
const maxWidth = 35;
const defaultValue = 20;

// School Scheduling Customizations State
let classSubjects = $state<any[]>([]);
let isUpdatingSubjectHours = $state(false);
let minimizeStudentGaps = $state(true);
let minimizeTeacherGaps = $state(true);
let strictRoomCapacity = $state(true);

// Solver State
let isSolving = $state(false);
let solverMode = $state<"full" | "incremental">("incremental");
let solveJobs = $state<any[]>([]);
let solveError = $state<string | null>(null);
let pollingInterval = $state<any>(null);
let timeElapsed = $state(0);
let elapsedTimer = $state<any>(null);

// Reactively reload class subjects when cycle or class changes
$effect(() => {
  if (selectedClassId && cycleId) {
    loadClassSubjects();
    loadSolveJobs();
  }
});

// Clean up polling on destroy
$effect(() => {
  return () => {
    stopPolling();
  };
});

async function loadClassSubjects() {
  if (!selectedClassId || !cycleId) return;
  try {
    const list = await commands.schedule.getClassSubjects(
      selectedClassId,
      cycleId,
    );
    classSubjects = list;
  } catch (e) {
    console.error("Error loading class subjects in panel:", e);
  }
}

function getSubjectName(subjectId: string) {
  const sub = subjects.find((s) => String(s.id) === String(subjectId));
  return sub
    ? sub.name_ar || sub.name
    : getLocale() === "ar"
      ? "مادة غير معروفة"
      : "Unknown Subject";
}

async function adjustSubjectHours(csId: string, delta: number) {
  const cs = classSubjects.find((x) => x.id === csId);
  if (!cs) return;
  const currentHours = cs.weekly_hours || 0;
  const newHours = Math.max(0, currentHours + delta);

  try {
    isUpdatingSubjectHours = true;
    await commands.schedule.updateClassSubject(csId, {
      weekly_hours: newHours,
    });
    await loadClassSubjects();
    onApply(); // Notify parent to refresh schedule grid
    toast.success(
      getLocale() === "ar"
        ? "تم تحديث الساعات بنجاح"
        : "Weekly hours updated successfully",
    );
  } catch (e) {
    toast.error(e instanceof Error ? e.message : String(e));
  } finally {
    isUpdatingSubjectHours = false;
  }
}

async function loadSolveJobs() {
  if (!cycleId) return;
  try {
    const list = await commands.schedule.listSolveJobs(orgId, cycleId);
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

async function triggerSolving() {
  if (!cycleId) return;
  isSolving = true;
  solveError = null;
  activeJob = null;
  previewDraft = false;

  try {
    const userId = authStore.userId || "desktop_user";
    const req = {
      cycle_id: cycleId,
      mode: solverMode,
    };
    const job = await commands.schedule.startSolve(orgId, userId, req);
    activeJob = job;
    toast.success(
      getLocale() === "ar"
        ? "تم بدء الجدولة التلقائية في الخلفية"
        : "Started auto-scheduling in the background",
    );
    startPollingJob(String(job.id));
  } catch (e) {
    isSolving = false;
    solveError = e instanceof Error ? e.message : String(e);
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
      const job = await commands.schedule.getSolveJob(orgId, jobId);
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
              : "Timetable optimization completed successfully!",
          );
          previewDraft = true;
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

async function cancelSolving() {
  if (!activeJob) return;
  try {
    const job = await commands.schedule.cancelSolve(orgId, activeJob.id);
    activeJob = job;
    stopPolling();
    isSolving = false;
    toast.info(
      getLocale() === "ar"
        ? "تم إلغاء الجدولة التلقائية"
        : "Optimization job cancelled",
    );
    await loadSolveJobs();
  } catch (e) {
    toast.error(e instanceof Error ? e.message : String(e));
  }
}

async function applySolving() {
  if (!activeJob) return;
  try {
    toast.loading(
      getLocale() === "ar" ? "جاري تطبيق الجدول..." : "Applying timetable...",
    );
    await commands.schedule.applySolve(orgId, activeJob.id);
    activeJob = null;
    previewDraft = false;
    toast.dismiss();
    toast.success(
      getLocale() === "ar"
        ? "تم تطبيق الجدول وحفظه بنجاح!"
        : "Timetable applied and saved successfully!",
    );
    onApply();
    await loadSolveJobs();
  } catch (e) {
    toast.dismiss();
    toast.error(e instanceof Error ? e.message : String(e));
  }
}

function discardDraft() {
  activeJob = null;
  previewDraft = false;
  toast.info(
    getLocale() === "ar"
      ? "تم تجاهل مسودة الجدول المقترح"
      : "Draft timetable discarded",
  );
}

function formatTime(seconds: number): string {
  const mins = Math.floor(seconds / 60);
  const secs = seconds % 60;
  return `${mins.toString().padStart(2, "0")}:${secs.toString().padStart(2, "0")}`;
}
</script>

<div 
	bind:this={panelEl}
	style="width: {preferredWidth}rem; min-width: {preferredWidth > 3 ? minWidth : 3}rem; max-width: {maxWidth}rem;"
	class="relative shrink-0 bg-card/65 backdrop-blur-md select-none transition-all duration-200 border-s border-border/80 flex flex-col"
	class:p-4={preferredWidth > 3}
	class:gap-4={preferredWidth > 3}
	in:fade={{ duration: 250 }}
>
	<!-- Resizer Component inside the sidebar (always rendered so we can drag it open) -->
	<Resizer
		viewport={panelEl}
		direction="left"
		minWidth={minWidth}
		maxWidth={maxWidth}
		defaultValue={defaultValue}
		collapsedWidth={3}
		persistId="scheduler-sidebar-width"
		onWidth={(w) => preferredWidth = w}
	/>

	{#if preferredWidth <= 3}
		<!-- Collapsed vertical strip -->
		<div class="h-full w-full flex flex-col items-center py-4 bg-card/30 select-none">
			<button
				type="button"
				class="h-8 w-8 flex items-center justify-center rounded-lg hover:bg-accent text-muted-foreground hover:text-foreground cursor-pointer transition-colors duration-150 mb-4"
				onclick={() => preferredWidth = defaultValue}
				title={getLocale() === "ar" ? "توسيع اللوحة" : "Expand Panel"}
			>
				<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
					<rect width="18" height="18" x="3" y="3" rx="2" />
					<path d="M15 3v18" />
				</svg>
			</button>

			<span class="vertical-text text-[10px] font-bold text-muted-foreground tracking-wider uppercase whitespace-nowrap">
				{getLocale() === "ar" ? "الجدولة الذكية (AI)" : "AI Auto-Scheduler"}
			</span>
		</div>
	{:else}
		<!-- Panel Header -->
		<div class="flex items-center justify-between pb-3 border-b border-border/80">
			<div class="flex items-center gap-2">
				<Sparkles size={16} class="text-purple-500 animate-pulse" />
				<span class="text-xs font-bold text-foreground tracking-wide">
					{getLocale() === "ar" ? "الجدولة الذكية (AI)" : "AI Auto-Scheduler"}
				</span>
			</div>
			<span class="text-[9px] font-semibold bg-purple-500/10 text-purple-600 dark:text-purple-400 px-1.5 py-0.5 rounded-full uppercase tracking-wider">
				Orbis AI v1.0
			</span>
		</div>

		<!-- Direct Weekly Load Editor section -->
		{#if classSubjects.length > 0}
			<div class="flex flex-col gap-2.5 pb-2.5 border-b border-border/60" in:fade={{ duration: 150 }}>
				<span class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider flex items-center gap-1.5">
					<BookOpen size={12} class="text-purple-500" />
					<span>{getLocale() === "ar" ? "نصاب الساعات الأسبوعية" : "Weekly Subject Hours"}</span>
				</span>
				<div class="flex flex-col gap-1.5 max-h-[160px] overflow-y-auto pr-1 scrollbar-hide">
					{#each classSubjects as cs (cs.id)}
						<div class="flex items-center justify-between p-1.5 bg-background border border-border/50 rounded-lg text-[9.5px]">
							<span class="font-bold text-foreground truncate max-w-[130px]" title={getSubjectName(cs.subject_id)}>
								{getSubjectName(cs.subject_id)}
							</span>
							<div class="flex items-center gap-1.5">
								<button 
									type="button"
									class="h-5 w-5 rounded bg-muted hover:bg-muted-foreground/10 text-foreground flex items-center justify-center font-bold cursor-pointer disabled:opacity-50 border border-border/60"
									onclick={() => adjustSubjectHours(cs.id, -1)}
									disabled={isUpdatingSubjectHours}
								>
									-
								</button>
								<span class="font-mono font-bold w-4 text-center text-foreground">
									{cs.weekly_hours || 0}
								</span>
								<button 
									type="button"
									class="h-5 w-5 rounded bg-muted hover:bg-muted-foreground/10 text-foreground flex items-center justify-center font-bold cursor-pointer disabled:opacity-50 border border-border/60"
									onclick={() => adjustSubjectHours(cs.id, 1)}
									disabled={isUpdatingSubjectHours}
								>
									+
								</button>
							</div>
						</div>
					{/each}
				</div>
			</div>
		{/if}

		<!-- Settings / Controls -->
		{#if !activeJob || activeJob.status === 'completed' || activeJob.status === 'failed' || activeJob.status === 'cancelled'}
			<div class="flex flex-col gap-3.5" in:fade={{ duration: 150 }}>
				<!-- Solver Mode -->
				<div class="flex flex-col gap-1.5">
					<span class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
						{getLocale() === "ar" ? "وضع الجدولة التلقائية" : "Scheduling Optimization Mode"}
					</span>
					<div class="grid grid-cols-2 p-0.5 rounded-lg bg-muted border border-border/60">
						<button 
							type="button"
							class="text-[10px] font-bold py-1.5 rounded-md transition-all duration-150 cursor-pointer {solverMode === 'incremental' ? 'bg-card text-foreground shadow-sm' : 'text-muted-foreground'}"
							onclick={() => solverMode = 'incremental'}
						>
							{getLocale() === "ar" ? "تدريجي (مسودة)" : "Incremental"}
						</button>
						<button 
							type="button"
							class="text-[10px] font-bold py-1.5 rounded-md transition-all duration-150 cursor-pointer {solverMode === 'full' ? 'bg-card text-foreground shadow-sm' : 'text-muted-foreground'}"
							onclick={() => solverMode = 'full'}
						>
							{getLocale() === "ar" ? "إعادة بالكامل" : "Full Reset"}
						</button>
					</div>
					<p class="text-[9px] text-muted-foreground leading-normal mt-0.5">
						{solverMode === 'incremental' 
							? (getLocale() === "ar" ? "يحافظ على الحصص الحالية المجدولة يدوياً، ويملأ الساعات الشاغرة فقط." : "Preserves current manual schedule, only schedules unassigned weekly hours.") 
							: (getLocale() === "ar" ? "يمسح الحصص الحالية في هذه الحلقة، ويعيد الجدولة بالكامل من الصفر." : "Clears all lessons in this cycle and generates a completely fresh schedule.")
						}
					</p>
				</div>

				<!-- AI Constraints Config -->
				<div class="flex flex-col gap-2 bg-muted/40 p-2.5 rounded-lg border border-border/60">
					<span class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
						{getLocale() === "ar" ? "محددات قيود الجدولة (AI)" : "AI Constraints Config"}
					</span>
					<label class="flex items-center justify-between cursor-pointer py-0.5">
						<span class="text-[9px] font-semibold text-foreground">
							{getLocale() === "ar" ? "تجنب فراغات الطلاب" : "Minimize student gaps"}
						</span>
						<input type="checkbox" bind:checked={minimizeStudentGaps} class="rounded text-purple-600 focus:ring-purple-500 accent-purple-600 cursor-pointer h-3.5 w-3.5" />
					</label>
					<label class="flex items-center justify-between cursor-pointer py-0.5">
						<span class="text-[9px] font-semibold text-foreground">
							{getLocale() === "ar" ? "تجنب فراغات المدرسين" : "Minimize teacher gaps"}
						</span>
						<input type="checkbox" bind:checked={minimizeTeacherGaps} class="rounded text-purple-600 focus:ring-purple-500 accent-purple-600 cursor-pointer h-3.5 w-3.5" />
					</label>
					<label class="flex items-center justify-between cursor-pointer py-0.5">
						<span class="text-[9px] font-semibold text-foreground">
							{getLocale() === "ar" ? "حظر مشاركة القاعات بالكامل" : "Strict Room Capacity"}
						</span>
						<input type="checkbox" bind:checked={strictRoomCapacity} class="rounded text-purple-600 focus:ring-purple-500 accent-purple-600 cursor-pointer h-3.5 w-3.5" />
					</label>
				</div>

				<Button 
					variant="lime" 
					class="w-full text-xs font-semibold flex items-center justify-center gap-1.5 py-1.5 h-9" 
					onclick={triggerSolving}
					disabled={isSolving}
				>
					<Cpu size={13} />
					<span>{getLocale() === "ar" ? "تشغيل المحسّن التلقائي" : "Run AI Optimizer"}</span>
				</Button>
			</div>
		{/if}

		<!-- Active Solver State -->
		{#if activeJob && (activeJob.status === 'pending' || activeJob.status === 'running')}
			<div class="bg-muted/40 border border-border/80 rounded-lg p-3.5 flex flex-col gap-3" in:fade={{ duration: 150 }}>
				<div class="flex items-center justify-between">
					<span class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
						{getLocale() === "ar" ? "الحالة الجارية" : "Optimizer Status"}
					</span>
					<span class="text-[9px] font-mono px-2 py-0.5 rounded bg-amber-500/10 text-amber-600 dark:text-amber-400 font-bold uppercase tracking-wider animate-pulse">
						{getLocale() === "ar" ? "جاري البحث..." : "Solving..."}
					</span>
				</div>

				<!-- Time counter -->
				<div class="flex items-center gap-2 text-xs font-semibold font-mono bg-background px-3 py-1.5 rounded border border-border/60 justify-center">
					<Clock size={12} class="text-amber-500 animate-spin" style="animation-duration: 3s" />
					<span>{formatTime(timeElapsed)}</span>
				</div>

				<!-- Score progression -->
				<div class="flex flex-col gap-1">
					<span class="text-[9px] text-muted-foreground">{getLocale() === "ar" ? "مؤشر القيود المؤقت:" : "Current Constraint Score:"}</span>
					<div class="bg-background border border-border/60 rounded p-2 font-mono text-[10px] flex justify-between">
						<span class="text-destructive font-semibold">
							{activeJob.score ? activeJob.score.split('hard/')[0] : '0'} Hard
						</span>
						<span class="text-primary font-semibold">
							{activeJob.score ? activeJob.score.split('hard/')[1] || '0' : '0'} Soft
						</span>
					</div>
				</div>

				<Button 
					variant="outline" 
					class="w-full text-xs font-semibold text-destructive hover:bg-destructive/10 border-destructive/20 flex items-center justify-center gap-1.5 py-1.5 h-8 mt-1" 
					onclick={cancelSolving}
				>
					<Square size={10} />
					<span>{getLocale() === "ar" ? "إلغاء التشغيل" : "Cancel Solving"}</span>
				</Button>
			</div>
		{/if}

		<!-- Draft Preview Mode Controls -->
		{#if activeJob && activeJob.status === 'completed'}
			<div class="bg-purple-500/5 border border-purple-500/20 rounded-lg p-3.5 flex flex-col gap-3" in:fade={{ duration: 150 }}>
				<div class="flex items-center justify-between">
					<span class="text-[10px] font-bold text-purple-600 dark:text-purple-400 uppercase tracking-wider">
						{getLocale() === "ar" ? "المسودة المقترحة" : "Proposed Solution"}
					</span>
					<span class="text-[9px] font-mono px-2 py-0.5 rounded bg-purple-500/10 text-purple-600 dark:text-purple-400 font-bold uppercase tracking-wider">
						{getLocale() === "ar" ? "جاهزة" : "Ready"}
					</span>
				</div>

				<!-- Final Score -->
				<div class="flex flex-col gap-1.5">
					<span class="text-[9px] text-muted-foreground">{getLocale() === "ar" ? "النقاط المحققة:" : "Optimized Score Summary:"}</span>
					<div class="bg-background border border-border/60 rounded p-2.5 font-mono text-[10px] flex justify-between">
						<span class="text-destructive font-bold">
							{activeJob.score ? activeJob.score.split('hard/')[0] : '0'} Hard
						</span>
						<span class="text-primary font-bold">
							{activeJob.score ? activeJob.score.split('hard/')[1] || '0' : '0'} Soft
						</span>
					</div>
				</div>

				<!-- Toggle preview -->
				<label class="flex items-center justify-between p-2 rounded bg-background border border-border/60 cursor-pointer select-none">
					<span class="text-[10px] font-bold text-foreground">
						{getLocale() === "ar" ? "معاينة الحصص المقترحة" : "Preview draft on grid"}
					</span>
					<input 
						type="checkbox" 
						bind:checked={previewDraft} 
						class="rounded text-purple-600 focus:ring-purple-500 accent-purple-600 cursor-pointer h-3.5 w-3.5"
					/>
				</label>

				<!-- Apply / Discard Actions -->
				<div class="flex gap-2 mt-1">
					<Button 
						variant="lime" 
						class="flex-1 text-xs font-semibold py-1.5 h-8 flex items-center justify-center gap-1"
						onclick={applySolving}
					>
						<CheckCircle2 size={11} />
						<span>{getLocale() === "ar" ? "اعتماد الجدول" : "Apply Table"}</span>
					</Button>
					<Button 
						variant="outline" 
						class="flex-1 text-xs font-semibold py-1.5 h-8 text-muted-foreground hover:bg-muted"
						onclick={discardDraft}
					>
						<span>{getLocale() === "ar" ? "تجاهل" : "Discard"}</span>
					</Button>
				</div>
			</div>
		{/if}

		<!-- Error Panel -->
		{#if solveError}
			<div class="bg-destructive/5 border border-destructive/20 rounded-lg p-3 flex items-start gap-2 text-destructive text-[10px]" in:fade={{ duration: 150 }}>
				<AlertCircle size={14} class="shrink-0 mt-0.5" />
				<div class="flex flex-col gap-1">
					<span class="font-bold">{getLocale() === "ar" ? "خطأ في عملية الجدولة:" : "Optimizer Error:"}</span>
					<span class="leading-normal">{solveError}</span>
				</div>
			</div>
		{/if}

		<!-- History List -->
		{#if solveJobs.length > 0}
			<div class="flex flex-col gap-2 mt-2" in:fade={{ duration: 150 }}>
				<div class="flex items-center gap-1.5 pb-1 border-b border-border/60">
					<History size={12} class="text-muted-foreground" />
					<span class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
						{getLocale() === "ar" ? "سجل الجدولة والتحسين" : "Optimization History"}
					</span>
				</div>
				<div class="flex flex-col gap-1.5 max-h-[140px] overflow-y-auto pr-1 scrollbar-hide">
					{#each solveJobs.slice(0, 5) as job (job.id)}
						<div class="flex flex-col p-2 bg-muted/30 border border-border/50 rounded-lg text-[9px] leading-tight select-text">
							<div class="flex items-center justify-between font-bold text-foreground">
								<span class="capitalize">{job.mode === 'full' ? (getLocale() === "ar" ? "كامل" : "Full") : (getLocale() === "ar" ? "تدريجي" : "Incremental")}</span>
								<span class="text-[8px] font-mono opacity-85">{new Date(job.created_at).toLocaleString(getLocale() === "ar" ? 'ar-EG' : 'en-US', { hour: '2-digit', minute: '2-digit', second: '2-digit' })}</span>
							</div>
							<div class="flex items-center justify-between text-muted-foreground mt-1">
								<span>{getLocale() === "ar" ? "النتيجة:" : "Score:"} <strong class="font-mono text-foreground">{job.score || 'N/A'}</strong></span>
								<span class="font-semibold uppercase tracking-wider text-[8px] px-1 rounded {job.status === 'completed' ? 'bg-primary/10 text-primary' : job.status === 'failed' ? 'bg-destructive/10 text-destructive' : 'bg-muted text-muted-foreground'}">
									{job.status === 'completed' ? (getLocale() === "ar" ? "ناجح" : "Done") : job.status === 'failed' ? (getLocale() === "ar" ? "فشل" : "Fail") : job.status}
								</span>
							</div>
						</div>
					{/each}
				</div>
			</div>
		{/if}
	{/if}
</div>

<style>
	.vertical-text {
		writing-mode: vertical-rl;
		text-orientation: mixed;
		transform: rotate(180deg);
	}

	:global(.resizer-dragging) {
		transition: none !important;
	}
</style>
