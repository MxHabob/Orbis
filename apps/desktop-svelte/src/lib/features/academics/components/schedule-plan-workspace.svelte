<!--
@component SchedulePlanWorkspace
Responsive workspace for preparing and reviewing an institution's timetable plan.
Includes constraints tuning, direct weekly subjects hours management, polling, and draft preview mode.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { commands, extractErrorMessage } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Checkbox } from "@orbis/ui-svelte/checkbox";
import { Label } from "@orbis/ui-svelte/label";
import { Badge } from "@orbis/ui-svelte/badge";
import { toast } from "svelte-sonner";
import { getLocale } from "$lib/paraglide/runtime";
import { fade } from "svelte/transition";
import {
  Square,
  Trash2,
	Sparkles,
} from "@lucide/svelte";
import Resizer from "$lib/shell/resizer.svelte";

let {
  periodId,
  selectedClassId,
  previewDraft = $bindable(false),
  activeJob = $bindable(null),
  subjects = [],
	standalone = false,
	embedded = false,
  onApply,
}: {
  periodId: string;
  selectedClassId: string;
  previewDraft: boolean;
  activeJob: any;
  subjects: any[];
	standalone?: boolean;
	embedded?: boolean;
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
// Solver State
let isSolving = $state(false);
let solveJobs = $state<any[]>([]);
let solveError = $state<string | null>(null);
let pollingInterval = $state<any>(null);
let timeElapsed = $state(0);
let elapsedTimer = $state<any>(null);

// Reactively reload class subjects when period or class changes
$effect(() => {
  if (selectedClassId && periodId) {
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
  if (!selectedClassId || !periodId) return;
  try {
    const list = await commands.academic.listClassSubjects(
      selectedClassId,
      periodId,
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
    : m.common_schedule_page_unknown_subject();
}

async function adjustSubjectHours(csId: string, delta: number) {
  const cs = classSubjects.find((x) => x.id === csId);
  if (!cs) return;
  const currentHours = cs.weekly_hours || 0;
  const newHours = Math.max(0, currentHours + delta);

  try {
    isUpdatingSubjectHours = true;
    await commands.academic.updateClassSubject(csId, {
      weekly_hours: newHours,
    });
    await loadClassSubjects();
    onApply(); // Notify parent to refresh schedule grid
    toast.success(m.common_schedule_ai_weekly_hours_updated());
  } catch (e) {
    toast.error(extractErrorMessage(e));
  } finally {
    isUpdatingSubjectHours = false;
  }
}

async function loadSolveJobs() {
  if (!periodId) return;
  try {
    const fn = (commands.scheduling as any).listJobs || (commands.scheduling as any).listSolverJobs;
    if (!fn) return;
    const list = await fn(periodId);
    solveJobs = Array.isArray(list) ? list : [];

    // Auto-resume polling if any job is running in background
    const running = solveJobs.find(
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
  if (!periodId) return;
  isSolving = true;
  solveError = null;
  activeJob = null;
  previewDraft = false;

  try {
    const fn = (commands.scheduling as any).start || (commands.scheduling as any).startSolver;
    if (!fn) throw new Error("Solver start command not found");
    const job = await fn(periodId);
    activeJob = job;
    toast.success(m.common_schedule_ai_optimization_started());
    if (job?.id) startPollingJob(String(job.id));
  } catch (e) {
    isSolving = false;
    solveError = extractErrorMessage(e);
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
      const fn = (commands.scheduling as any).getJob || (commands.scheduling as any).getSolverJob;
      if (!fn) return;
      const job = await fn(jobId);
      activeJob = job;

      if (
        job &&
        (job.status === "completed" ||
          job.status === "failed" ||
          job.status === "cancelled")
      ) {
        stopPolling();
        isSolving = false;
        await loadSolveJobs();
        if (job.status === "completed") {
          toast.success(m.common_schedule_ai_optimization_completed());
          previewDraft = true;
        } else if (job.status === "failed") {
          solveError =
            (job.error_message as string) ||
            m.common_schedule_ai_optimization_failed();
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
    const fn = (commands.scheduling as any).cancel || (commands.scheduling as any).cancelSolver;
    if (!fn) return;
    const job = await fn(activeJob.id);
    activeJob = job;
    stopPolling();
    isSolving = false;
    toast.info(m.common_action_cancel());
    await loadSolveJobs();
  } catch (e) {
    toast.error(extractErrorMessage(e));
  }
}

async function applySolving() {
  if (!activeJob) return;
  try {
    toast.loading(m.common_schedule_ai_applying_timetable());
    const fn = (commands.scheduling as any).apply || (commands.scheduling as any).applySolver;
    if (!fn) throw new Error("Apply solver command not found");
    await fn(activeJob.id);
    activeJob = null;
    previewDraft = false;
    toast.dismiss();
    toast.success(m.common_schedule_ai_timetable_applied());
    onApply();
    await loadSolveJobs();
  } catch (e) {
    toast.dismiss();
    toast.error(extractErrorMessage(e));
  }
}

function discardDraft() {
  activeJob = null;
  previewDraft = false;
  toast.info(m.common_schedule_ai_draft_discarded());
}

function formatTime(seconds: number): string {
  const mins = Math.floor(seconds / 60);
  const secs = seconds % 60;
  return `${mins.toString().padStart(2, "0")}:${secs.toString().padStart(2, "0")}`;
}

async function deleteSolverJob(jobId: string) {
  try {
    await commands.scheduling.deleteJob(jobId);
    toast.success(m.common_schedule_ai_record_deleted());

    // Clear state if deleting currently active job
    if (activeJob && String(activeJob.id) === String(jobId)) {
      activeJob = null;
      isSolving = false;
      previewDraft = false;
      stopPolling();
    }

    await loadSolveJobs();
  } catch (e) {
    toast.error(extractErrorMessage(e));
  }
}

</script>

<div
	bind:this={panelEl}
	style={standalone ? undefined : `width: ${preferredWidth}rem; min-width: ${preferredWidth > 3 ? minWidth : 3}rem; max-width: ${maxWidth}rem;`}
	class="relative select-none transition-all duration-200 flex flex-col {embedded ? 'w-full min-h-0' : standalone ? 'w-full h-full min-h-0 overflow-y-auto rounded-xl border border-border/80 bg-card/65 p-4 sm:p-6 lg:p-8 backdrop-blur-md' : 'shrink-0 border-s border-border/80 bg-card/65 backdrop-blur-md'}"
	class:p-4={!standalone && preferredWidth > 3}
	class:gap-4={!standalone && preferredWidth > 3}
	in:fade={{ duration: 250 }}
>
	{#if !standalone}
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
	{/if}

	{#if !standalone && preferredWidth <= 3}

		<!-- Collapsed vertical strip -->
		<div class="h-full w-full flex flex-col items-center py-4 bg-card/30 select-none">
			<Button
				variant="ghost"
				size="icon"
				class="mb-4 text-muted-foreground hover:text-foreground"
				onclick={() => preferredWidth = defaultValue}
				title={m.common_schedule_ai_expand_panel()}
			>
				<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
					<rect width="18" height="18" x="3" y="3" rx="2" />
					<path d="M15 3v18" />
				</svg>
			</Button>

			<span class="vertical-text text-[10px] font-bold text-muted-foreground tracking-wider uppercase whitespace-nowrap">
				{m.common_schedule_ai_auto_scheduler()}
			</span>
		</div>
	{:else}
		<!-- Panel Header -->
		<div class="flex items-center justify-between pb-3 border-b border-border/80">
			<div class="flex items-center gap-2">
				<span class="text-xs font-bold text-foreground tracking-wide">
					{m.common_schedule_ai_auto_scheduler()}
				</span>
			</div>
		</div>

		<!-- Weekly teaching load editor -->
		{#if classSubjects.length > 0}
			<div class="flex flex-col gap-2.5 pb-2.5" in:fade={{ duration: 150 }}>
				<div class="flex flex-col gap-1.5 max-h-40 overflow-y-auto pr-1 scrollbar-hide">
					{#each classSubjects as cs (cs.id)}
						<div class="flex items-center justify-between p-1.5 text-[9.5px]">
							<span class="font-bold text-foreground truncate max-w-32.5" title={getSubjectName(cs.subject_id)}>
								{getSubjectName(cs.subject_id)}
							</span>
							<div class="flex items-center gap-1.5">
								<Button 
									variant="secondary"
									size="icon-xs"
									class="cursor-pointer text-[10px] font-bold"
									onclick={() => adjustSubjectHours(cs.id, -1)}
									disabled={isUpdatingSubjectHours}
								>
									-
								</Button>
								<span class="font-mono font-bold w-4 text-center text-foreground">
									{cs.weekly_hours || 0}
								</span>
								<Button 
									variant="secondary"
									size="icon-xs"
									class="cursor-pointer text-[10px] font-bold"
									onclick={() => adjustSubjectHours(cs.id, 1)}
									disabled={isUpdatingSubjectHours}
								>
									+
								</Button>
							</div>
						</div>
					{/each}
				</div>
			</div>
		{/if}

		<!-- Settings / Controls -->
		{#if !activeJob || activeJob.status === 'completed' || activeJob.status === 'failed' || activeJob.status === 'cancelled'}
			<div class="flex flex-col gap-3.5" in:fade={{ duration: 150 }}>
				<Button 
					class="w-full text-xs font-semibold flex items-center justify-center gap-1.5 py-1.5 h-9" 
					onclick={triggerSolving}
					disabled={isSolving}
				>
					<Sparkles size={14} />
					<span>{m.common_schedule_ai_run_optimizer()}</span>
				</Button>
			</div>
		{/if}

		<!-- Active Solver State -->
		{#if activeJob && (activeJob.status === 'pending' || activeJob.status === 'running')}
			<div class="bg-muted/40 border border-border/80 rounded-lg p-3.5 flex flex-col gap-3" in:fade={{ duration: 150 }}>
				<div class="flex items-center justify-between">
					<span class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
						{m.common_schedule_ai_optimizer_status()}
					</span>
					<Badge variant="secondary" class="text-[9px] font-mono font-bold uppercase tracking-wider animate-pulse">
						{m.common_schedule_ai_solving()}
					</Badge>
				</div>

				<!-- Time counter -->
				<div class="flex items-center gap-2 text-xs font-semibold font-mono bg-background px-3 py-1.5 rounded border border-border/60 justify-center">
					<span>{formatTime(timeElapsed)}</span>
				</div>

				<!-- Score progression -->
				<div class="flex flex-col gap-1">
					<span class="text-[9px] text-muted-foreground">{m.common_schedule_ai_current_score()}</span>
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
					<span>{m.common_action_cancel()}</span>
				</Button>
			</div>
		{/if}

		<!-- Draft Preview Mode Controls -->
		{#if activeJob && activeJob.status === 'completed'}
			<div class="bg-primary/5 border border-primary/20 rounded-lg p-3.5 flex flex-col gap-3" in:fade={{ duration: 150 }}>
				<div class="flex items-center justify-between">
					<span class="text-[10px] font-bold text-primary uppercase tracking-wider">
						{m.common_schedule_ai_proposed_solution()}
					</span>
					<Badge variant="brand" class="text-[9px] font-mono font-bold uppercase tracking-wider">
						{m.common_schedule_ai_ready()}
					</Badge>
				</div>

				<!-- Final Score -->
				<div class="flex flex-col gap-1.5">
					<span class="text-[9px] text-muted-foreground">{m.common_schedule_ai_score_summary()}</span>
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
				<div class="flex items-center justify-between p-2 rounded bg-background border border-border/60">
					<Label class="text-[10px] font-bold text-foreground cursor-pointer" for="preview-draft">
						{m.common_schedule_ai_preview_draft()}
					</Label>
					<Checkbox id="preview-draft" checked={previewDraft} onCheckedChange={(v: boolean) => previewDraft = v} />
				</div>

				<!-- Apply / Discard Actions -->
				<div class="flex gap-2 mt-1">
					<Button 
						variant="lime" 
						class="flex-1 text-xs font-semibold py-1.5 h-8 flex items-center justify-center gap-1"
						onclick={applySolving}
					>
						<span>{m.common_schedule_ai_apply_table()}</span>
					</Button>
					<Button 
						variant="outline" 
						class="flex-1 text-xs font-semibold py-1.5 h-8 text-muted-foreground hover:bg-muted"
						onclick={discardDraft}
					>
						<span>{m.common_schedule_ai_discard()}</span>
					</Button>
				</div>
			</div>
		{/if}

		<!-- Error Panel -->
		{#if solveError}
			<div class="bg-destructive/5 border border-destructive/20 rounded-lg p-3 flex items-start gap-2 text-destructive text-[10px]" in:fade={{ duration: 150 }}>
				<div class="flex flex-col gap-1">
					<span class="font-bold">{m.common_schedule_ai_optimizer_error()}</span>
					<span class="leading-normal">{solveError}</span>
				</div>
			</div>
		{/if}

		<!-- History List -->
		{#if solveJobs.length > 0}
			<div class="flex flex-col gap-2 mt-2" in:fade={{ duration: 150 }}>
				<div class="flex items-center gap-1.5 pb-1 border-b border-border/60">
					<span class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
						{m.common_schedule_ai_optimization_history()}
					</span>
				</div>
				<div class="flex flex-col gap-1.5 max-h-35 overflow-y-auto pr-1 scrollbar-hide">
					{#each solveJobs.slice(0, 5) as job (job.id)}
						<div class="flex flex-col p-2 bg-muted/30 border border-border/50 rounded-lg text-[9px] leading-tight select-text">
							<div class="flex items-center justify-between font-bold text-foreground">
								<span class="capitalize">{job.mode === 'full' ? m.common_schedule_ai_mode_full() : m.common_schedule_ai_mode_incremental()}</span>
								<div class="flex items-center gap-1.5 select-none font-normal">
									<span class="text-[8px] font-mono opacity-85">{new Date(job.created_at).toLocaleTimeString(getLocale(), { hour: '2-digit', minute: '2-digit', second: '2-digit' })}</span>
									<Button 
										variant="ghost" 
										size="icon-xs"
										class="h-4 w-4 text-muted-foreground/60 hover:text-destructive p-0 cursor-pointer flex items-center justify-center rounded-sm"
										onclick={() => deleteSolverJob(job.id)}
										title={m.common_action_delete()}
									>
										<Trash2 size={10} />
									</Button>
								</div>
							</div>
							<div class="flex items-center justify-between text-muted-foreground mt-1">
								<span>{m.common_schedule_ai_score()} <strong class="font-mono text-foreground">{job.score || 'N/A'}</strong></span>
								<Badge 
									variant={job.status === 'completed' ? 'lime' : job.status === 'failed' ? 'destructive' : 'secondary'} 
									class="uppercase tracking-wider text-[8px] px-1 py-0.5"
								>
									{job.status === 'completed' ? m.common_schedule_ai_status_done() : job.status === 'failed' ? m.common_calendar_status_failed() : job.status}
								</Badge>
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
