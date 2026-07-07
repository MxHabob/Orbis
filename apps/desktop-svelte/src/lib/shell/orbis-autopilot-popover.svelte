<script lang="ts">
import { Button } from "@orbis/ui-svelte/button";
import { Badge } from "@orbis/ui-svelte/badge";
import {
  Popover,
  PopoverTrigger,
  PopoverContent,
} from "@orbis/ui-svelte/popover";
import { Tooltip, TooltipContent, TooltipTrigger } from "@orbis/ui-svelte";
import { Activity, Settings, RefreshCw, AlertTriangle } from "lucide-svelte";
import { academicContext, workflowInbox } from "$lib/stores";
import * as m from "$lib/paraglide/messages";
import { l, formatDate } from "$lib/core/i18n";
import { createCalendarEventsQuery } from "$lib/features/academics";
import {
  createAutopilotConfigQuery,
  createAutopilotTasksQuery,
  createAutopilotDiagnosticsQuery,
  useConfigureAutopilot,
  useRunAutopilotTick,
} from "$lib/features/ai/queries";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { localizeHref } from "$lib/paraglide/runtime";
import type { CalendarEventType, CyclePhase } from "@orbis/desktop-client/types";
import { toast } from "svelte-sonner";
import { extractErrorMessage } from "$lib/core/ipc/error";

let { orgId = "" }: { orgId?: string } = $props();
const cycleId = $derived(academicContext.effectiveCycleId);

function isEventInPhase(eventType: CalendarEventType, phase: CyclePhase): boolean {
  switch (phase) {
    case "pre_start":
      return [
        "cycle_start",
        "enrollment_open",
        "enrollment_close",
        "add_drop_open",
        "add_drop_close",
      ].includes(eventType);
    case "active":
      return [
        "classes_start",
        "classes_end",
        "holiday",
        "break",
        "teacher_training",
      ].includes(eventType);
    case "exams":
      return ["exam_prep_start", "exam_start", "exam_end"].includes(eventType);
    case "grade_entry":
      return ["grade_entry_open", "grade_entry_close", "results_announcement"].includes(eventType);
    case "closed":
      return [
        "promotion_window_open",
        "promotion_window_close",
        "cycle_end",
      ].includes(eventType);
    default:
      return false;
  }
}

// Queries
const eventsQuery = createCalendarEventsQuery(orgId, cycleId);
const events = $derived($eventsQuery.data ?? []);

const configQuery = createAutopilotConfigQuery(orgId);
const config = $derived($configQuery.data);

const tasksQuery = createAutopilotTasksQuery(orgId);
const autopilotTasks = $derived($tasksQuery.data ?? []);

const diagnosticsQuery = createAutopilotDiagnosticsQuery(orgId);
const diagnostics = $derived($diagnosticsQuery.data);

// Mutations
const runTickMutation = useRunAutopilotTick();
const configureMutation = useConfigureAutopilot();

function getEventTaskStatus(eventId: string) {
  const task = autopilotTasks.find(t => t.calendar_event_id === eventId);
  return task?.status;
}

function getPhaseLabel(phase: string) {
  switch (phase) {
    case "pre_start":
      return m.phase_pre_start ? m.phase_pre_start() : "Pre-Start";
    case "active":
      return m.phase_active ? m.phase_active() : "Active";
    case "exams":
      return m.phase_exams ? m.phase_exams() : "Exams";
    case "grade_entry":
      return m.phase_grade_entry ? m.phase_grade_entry() : "Grade Entry";
    case "closed":
      return m.phase_closed ? m.phase_closed() : "Closed";
    default:
      return "";
  }
}

async function handleRunTick() {
  if (!orgId) return;
  try {
    toast.promise($runTickMutation.mutateAsync({ orgId }), {
      loading: m.calendar_processing() || "جاري تشغيل فحص الأتمتة الفوري...",
      success: () => "تم تشغيل الفحص وتحديث حالة الأتمتة بنجاح.",
      error: (e) => extractErrorMessage(e) || "فشل تشغيل فحص الأتمتة الفوري.",
    });
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  }
}

async function togglePause() {
  if (!orgId || !config) return;
  const isEnabled = config.is_enabled === 1;
  const isCurrentlyPaused = config.paused_at != null;
  const nextPaused = !isCurrentlyPaused;
  
  try {
    await $configureMutation.mutateAsync({
      orgId,
      isEnabled,
      paused: nextPaused,
      mode: config.mode,
    });
    toast.success(
      nextPaused
        ? "تم إيقاف الأتمتة مؤقتاً بنجاح."
        : "تم استئناف تشغيل الأتمتة بنجاح."
    );
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  }
}

const phaseLabel = $derived(getPhaseLabel(academicContext.cyclePhase));
</script>

<Popover>
	<PopoverTrigger>
		{#snippet child({ props }: { props: any })}
			<Tooltip>
				<TooltipTrigger>
					<Button 
						{...props}
						variant="outline" 
						size="icon-sm" 
						class="relative cursor-pointer rounded-[10px]"
						aria-label={m.sidebar_orbis_autopilot ? m.sidebar_orbis_autopilot() : "Orbis AutoPilot"}
					>
						<Activity size={16} strokeWidth={1.8} class="text-foreground" />
						{#if workflowInbox.hasActions || (diagnostics && (diagnostics.stuck_tasks.length > 0 || diagnostics.overdue_tasks.length > 0))}
							<span class="absolute -top-0.5 -inset-e-0.5 w-1.5 h-1.5 rounded-full bg-destructive border border-background animate-pulse"></span>
						{:else if config?.is_enabled === 1 && !config?.paused_at}
							<span class="absolute -top-0.5 -inset-e-0.5 w-1.5 h-1.5 rounded-full bg-brand-lime border border-background"></span>
						{:else}
							<span class="absolute -top-0.5 -inset-e-0.5 w-1.5 h-1.5 rounded-full bg-muted-foreground/60 border border-background"></span>
						{/if}
					</Button>
				</TooltipTrigger>
				<TooltipContent side="right" sideOffset={12}>
					{m.sidebar_orbis_autopilot ? m.sidebar_orbis_autopilot() : "Orbis AutoPilot"}
				</TooltipContent>
			</Tooltip>
		{/snippet}
	</PopoverTrigger>
	<PopoverContent
		side="right"
		align="end"
		sideOffset={12}
		class="w-[360px] rounded-xl p-0 overflow-hidden z-50 flex flex-col font-sans text-start border border-border/60 bg-popover text-popover-foreground shadow-lg animate-in fade-in slide-in-from-left-2 duration-200"
	>
		<!-- Header -->
		<div class="px-4 py-3 border-b border-border bg-muted/30 flex items-center justify-between">
			<div class="flex items-center gap-1.5">
				<Activity size={16} class="text-primary animate-pulse" />
				<span class="text-xs font-semibold text-foreground">{m.sidebar_orbis_autopilot() || "Orbis AutoPilot"}</span>
				
				{#if !config}
					<Skeleton class="h-3.5 w-10 rounded" />
				{:else if config.is_enabled === 0}
					<Badge variant="outline" class="text-[8px] h-3.5 px-1 font-bold uppercase text-muted-foreground">
						{m.sidebar_orbis_autopilot_status_disabled ? m.sidebar_orbis_autopilot_status_disabled() : "Disabled"}
					</Badge>
				{:else if config.paused_at}
					<Badge variant="secondary" class="text-[8px] h-3.5 px-1 font-bold uppercase bg-amber-500/10 text-amber-600 dark:text-amber-400 border-none">
						{m.sidebar_orbis_autopilot_status_paused ? m.sidebar_orbis_autopilot_status_paused() : "Paused"}
					</Badge>
				{:else}
					<Badge variant="lime" class="text-[8px] h-3.5 px-1 font-bold uppercase tracking-wider">
						{m.sidebar_orbis_autopilot_status_active ? m.sidebar_orbis_autopilot_status_active() : "Active"}
					</Badge>
				{/if}

				{#if config}
					<Badge variant="outline" class="text-[8px] h-3.5 px-1 font-bold uppercase text-muted-foreground">
						{config.mode === 'automatic' 
							? (m.calendar_field_mode_automatic ? m.calendar_field_mode_automatic() : "Auto") 
							: config.mode === 'suggested'
								? (m.calendar_field_mode_suggested ? m.calendar_field_mode_suggested() : "Suggested")
								: (m.calendar_field_mode_manual ? m.calendar_field_mode_manual() : "Manual")}
					</Badge>
				{/if}
			</div>
			<!-- Settings Link -->
			<Button 
				href={localizeHref(`/${orgId}/settings/academic-policy`)} 
				variant="ghost" 
				size="icon-xs" 
				class="rounded-[8px] h-7 w-7 text-muted-foreground hover:text-foreground cursor-pointer flex items-center justify-center"
				aria-label="Autopilot Settings"
			>
				<Settings size={14} />
			</Button>
		</div>

		<!-- Diagnostics Panel (Optional Alert) -->
		{#if diagnostics && (diagnostics.stuck_tasks.length > 0 || diagnostics.overdue_tasks.length > 0)}
			<div class="px-4 py-2 border-b border-border bg-destructive/5 text-[10px] text-destructive flex items-center gap-1.5">
				<AlertTriangle size={12} class="shrink-0" />
				<span class="font-medium">
					{diagnostics.stuck_tasks.length + diagnostics.overdue_tasks.length} {m.sidebar_no_actions ? m.sidebar_no_actions() : "stuck/overdue automation tasks detected."}
				</span>
			</div>
		{/if}

		<!-- Unified Timeline Section -->
		<div class="p-4 border-b border-border flex flex-col gap-3">
			<h4 class="text-xs font-bold text-foreground leading-normal text-start">
				{m.sidebar_orbis_autopilot_automation_title() || "System Automation Tasks"}
			</h4>

			{#if $eventsQuery.isLoading}
				<div class="space-y-2">
					<Skeleton class="h-6 w-full rounded-md" />
					<Skeleton class="h-6 w-full rounded-md" />
				</div>
			{:else}
				{@const sortedEvents = [...events].sort((a, b) => a.start_date.localeCompare(b.start_date))}
				{#if sortedEvents.length === 0}
					<p class="text-[10px] text-muted-foreground text-start">
						{m.sidebar_orbis_autopilot_no_automation() || "No automated tasks configured for this cycle."}
					</p>
				{:else}
					<div class="max-h-[220px] overflow-y-auto scrollbar-hide mt-1 relative ps-4 border-s border-border space-y-4 text-start">
						{#each sortedEvents as event}
							{@const taskStatus = event.id ? getEventTaskStatus(event.id) : null}
							{@const isExecuted = event.executed_at != null || taskStatus === "completed"}
							{@const isActivePhase = isEventInPhase(event.event_type, academicContext.cyclePhase) || taskStatus === "executing"}
							<div class="relative">
								<!-- Timeline indicator dot center-aligned with the border-s line -->
								{#if taskStatus === "completed" || isExecuted}
									<span class="absolute -start-[21px] top-1.5 size-2 rounded-full bg-primary border border-background"></span>
								{:else if taskStatus === "failed"}
									<span class="absolute -start-[21px] top-1.5 size-2 rounded-full bg-destructive border border-background"></span>
								{:else if taskStatus === "executing" || isActivePhase}
									<span class="absolute -start-[21px] top-1.5 size-2 rounded-full bg-brand-lime border border-background animate-pulse"></span>
								{:else if taskStatus === "suggested"}
									<span class="absolute -start-[21px] top-1.5 size-2 rounded-full bg-accent border border-background"></span>
								{:else if event.execution_mode === "automatic"}
									<span class="absolute -start-[21px] top-1.5 size-2 rounded-full bg-primary/80 border border-background"></span>
								{:else if event.execution_mode === "suggested"}
									<span class="absolute -start-[21px] top-1.5 size-2 rounded-full bg-accent/80 border border-background"></span>
								{:else}
									<span class="absolute -start-[21px] top-1.5 size-2 rounded-full bg-muted-foreground/50 border border-background"></span>
								{/if}

								<div class="flex items-center justify-between gap-1.5 text-[10px]">
									<span class="font-bold leading-tight {isActivePhase ? 'text-primary' : 'text-foreground'}">
										{l(event.title_ar, event.title)}
									</span>
									<Badge 
										variant={event.execution_mode === 'automatic' ? 'lime' : event.execution_mode === 'suggested' ? 'secondary' : 'outline'} 
										class="text-[8px] h-3 px-1 shrink-0"
									>
										{event.execution_mode === 'automatic' 
											? (m.calendar_field_mode_automatic ? m.calendar_field_mode_automatic() : "Auto") 
											: event.execution_mode === 'suggested'
												? (m.calendar_field_mode_suggested ? m.calendar_field_mode_suggested() : "Suggested")
												: (m.calendar_field_mode_manual ? m.calendar_field_mode_manual() : "Manual")}
									</Badge>
								</div>
								
								<p class="text-[9px] text-muted-foreground leading-normal mt-0.5">
									{#if taskStatus === "completed" || isExecuted}
										<span class="text-primary font-medium">✓ {m.calendar_status_done ? m.calendar_status_done() : "Done"}</span>
										{#if event.executed_at}
											<span class="text-muted-foreground"> ({formatDate(event.executed_at)})</span>
										{/if}
									{:else if taskStatus === "failed"}
										<span class="text-destructive font-medium">✗ Failed</span>
									{:else if taskStatus === "executing"}
										<span class="text-brand-lime font-medium animate-pulse">Running...</span>
									{:else if taskStatus === "suggested"}
										<span class="text-accent-foreground font-medium">Requires Approval</span>
									{:else}
										<span>{m.sidebar_orbis_autopilot_scheduled_for ? m.sidebar_orbis_autopilot_scheduled_for({ date: formatDate(event.start_date) }) : `Scheduled for ${formatDate(event.start_date)}`}</span>
									{/if}
								</p>
							</div>
						{/each}
					</div>
				{/if}
			{/if}
		</div>

		<!-- Control Actions -->
		<div class="px-4 py-3 border-b border-border bg-muted/10 flex items-center justify-between gap-3">
			<Button
				variant="outline"
				size="sm"
				class="w-full text-[10px] h-8 font-semibold cursor-pointer flex items-center justify-center gap-1.5"
				onclick={handleRunTick}
				disabled={$runTickMutation.isPending}
			>
				<RefreshCw size={12} class={$runTickMutation.isPending ? "animate-spin" : ""} />
				{#if $runTickMutation.isPending}
					{m.calendar_processing() || "Executing..."}
				{:else}
					{m.sidebar_orbis_autopilot_run_now() || "Run AutoPilot Tick"}
				{/if}
			</Button>

			<Button
				variant="ghost"
				size="sm"
				class="text-[10px] h-8 text-muted-foreground hover:text-foreground cursor-pointer shrink-0"
				onclick={togglePause}
				disabled={$configureMutation.isPending || !config}
			>
				{#if config?.paused_at}
					▶ {m.sidebar_orbis_autopilot_resume() || "Resume"}
				{:else}
					⏸ {m.sidebar_orbis_autopilot_pause() || "Pause"}
				{/if}
			</Button>
		</div>

		<!-- School Metadata / Status -->
		<div class="p-4 flex flex-col gap-2.5 bg-muted/10">
			<!-- Academic Phase -->
			<div class="flex items-center justify-between text-xs">
				<span class="text-muted-foreground">{m.sidebar_orbis_autopilot_phase ? m.sidebar_orbis_autopilot_phase() : "Phase"}</span>
				{#if phaseLabel}
					<Badge variant="secondary" class="text-[10px] h-4 px-1.5">{phaseLabel}</Badge>
				{:else}
					<span class="text-muted-foreground">—</span>
				{/if}
			</div>
			<!-- Year Status -->
			<div class="flex items-center justify-between text-xs">
				<span class="text-muted-foreground">{m.sidebar_orbis_autopilot_year ? m.sidebar_orbis_autopilot_year() : "Year"}</span>
				<span class="font-medium text-foreground">{academicContext.yearLabel || "—"}</span>
			</div>
			<!-- Pending Approvals -->
			<div class="flex items-center justify-between text-xs">
				<span class="text-muted-foreground">{m.approvals_tab_pending ? m.approvals_tab_pending() : "Pending"}</span>
				{#if workflowInbox.hasActions}
					<Badge variant="destructive" class="text-[10px] h-4 px-1.5">{workflowInbox.totalCount}</Badge>
				{:else}
					<span class="text-primary text-[10px] font-medium">✓</span>
				{/if}
			</div>
			<!-- Read-Only Mode Banner -->
			{#if academicContext.isReadOnly}
				<div class="flex items-center gap-1.5 text-[10px] text-muted-foreground bg-muted/50 rounded px-2 py-1 mt-1 justify-center">
					<span>🔒</span>
					<span>{m.year_archived_banner ? m.year_archived_banner() : "Read-only mode"}</span>
				</div>
			{/if}
		</div>
	</PopoverContent>
</Popover>
