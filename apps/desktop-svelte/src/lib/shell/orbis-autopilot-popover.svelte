<script lang="ts">
import { Button } from "@orbis/ui-svelte/button";
import { Badge } from "@orbis/ui-svelte/badge";
import { Popover, PopoverTrigger, PopoverContent } from "@orbis/ui-svelte/popover";
import { Tooltip, TooltipContent, TooltipTrigger, TooltipProvider } from "@orbis/ui-svelte/tooltip";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { 
  Activity, 
  RefreshCw, 
  AlertTriangle, 
  AlertCircle,
  Settings
} from "lucide-svelte";
import { academicContext } from "$lib/stores";
import * as m from "$lib/paraglide/messages";
import { formatDate } from "$lib/core/i18n";
import {
  createAutopilotConfigQuery,
  createAutopilotTasksQuery,
  createAutopilotDiagnosticsQuery,
  useRunAutopilotTick,
} from "$lib/features/ai/queries";
import { toast } from "svelte-sonner";
import { extractErrorMessage } from "$lib/core/ipc/error";

let { orgId = "" }: { orgId?: string } = $props();

// Autopilot Queries
const configQuery = createAutopilotConfigQuery(orgId);
const config = $derived($configQuery.data);

const tasksQuery = createAutopilotTasksQuery(orgId);
const tasks = $derived($tasksQuery.data ?? []);

const diagnosticsQuery = createAutopilotDiagnosticsQuery(orgId);
const diagnostics = $derived($diagnosticsQuery.data);

// Mutations
const runTickMutation = useRunAutopilotTick();

// Friendly task name mapper
function getTaskTypeLabel(type: string): string {
  switch (type) {
    case "enrollment_open":
      return m.autopilot_task_enrollment_open();
    case "enrollment_close":
      return m.autopilot_task_enrollment_close();
    case "grade_entry_open":
      return m.autopilot_task_grade_entry_open();
    case "grade_entry_close":
      return m.autopilot_task_grade_entry_close();
    case "cycle_lock":
      return m.autopilot_task_cycle_lock();
    case "report_generation":
      return m.autopilot_task_report_generation();
    case "backup":
      return m.autopilot_task_backup();
    case "promotion_dry_run":
      return m.autopilot_task_promotion_dry_run();
    default:
      return type;
  }
}

// Friendly phase label mapper
function getPhaseLabel(phase: string) {
  switch (phase) {
    case "enrollment":
      return m.phase_enrollment();
    case "active":
      return m.phase_active();
    case "exam":
      return m.phase_exams();
    case "grading":
      return m.phase_grade_entry();
    case "locked":
      return m.phase_locked();
    case "archived":
      return m.phase_closed();
    default:
      return phase;
  }
}

async function handleRunTick() {
  if (!orgId) return;
  try {
    toast.promise($runTickMutation.mutateAsync({ orgId }), {
      loading: m.autopilot_popover_tick_running(),
      success: () => m.autopilot_popover_run_tick(),
      error: (e) => extractErrorMessage(e) || m.autopilot_popover_run_tick(),
    });
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  }
}

const phaseLabel = $derived(getPhaseLabel(academicContext.cyclePhase));
</script>

<Popover>
  <PopoverTrigger>
    {#snippet child({ props }: { props: any })}
      <TooltipProvider>
        <Tooltip>
          <TooltipTrigger>
            <Button 
              {...props}
              variant="outline" 
              size="icon-sm" 
              class="relative cursor-pointer rounded-[10px]"
              aria-label={m.autopilot_popover_title()}
            >
              <Activity size={16} strokeWidth={1.8} class="text-foreground" />
              {#if diagnostics && (diagnostics.stuck_tasks.length > 0 || diagnostics.overdue_tasks.length > 0)}
                <span class="absolute -top-0.5 -inset-e-0.5 w-1.5 h-1.5 rounded-full bg-destructive border border-background animate-pulse"></span>
              {:else if config?.is_enabled === 1 && !config?.paused_at}
                <span class="absolute -top-0.5 -inset-e-0.5 w-1.5 h-1.5 rounded-full bg-brand-lime border border-background"></span>
              {:else}
                <span class="absolute -top-0.5 -inset-e-0.5 w-1.5 h-1.5 rounded-full bg-muted-foreground/60 border border-background"></span>
              {/if}
            </Button>
          </TooltipTrigger>
          <TooltipContent side="right" sideOffset={12}>
            {m.autopilot_popover_title()}
          </TooltipContent>
        </Tooltip>
      </TooltipProvider>
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
        <span class="text-xs font-semibold text-foreground">{m.autopilot_popover_title()}</span>
        
        {#if !config}
          <Skeleton class="h-3.5 w-10 rounded" />
        {:else if config.is_enabled === 0}
          <Badge variant="outline" class="text-[8px] h-3.5 px-1 font-bold uppercase text-muted-foreground">
            {m.autopilot_popover_status_disabled()}
          </Badge>
        {:else if config.paused_at}
          <Badge variant="secondary" class="text-[8px] h-3.5 px-1 font-bold uppercase bg-amber-500/10 text-amber-600 dark:text-amber-400 border-none">
            {m.autopilot_popover_status_paused()}
          </Badge>
        {:else}
          <Badge variant="lime" class="text-[8px] h-3.5 px-1 font-bold uppercase tracking-wider">
            {m.autopilot_popover_status_active()}
          </Badge>
        {/if}
      </div>
      
      <!-- Settings Redirect Icon Link -->
      <a 
        href="/{orgId}/settings/academic-policy" 
        class="text-muted-foreground hover:text-foreground transition-colors shrink-0"
        title={m.autopilot_popover_config_title()}
      >
        <Settings size={14} />
      </a>
    </div>

    <!-- Diagnostics Panel (Stuck / Overdue Alerts) -->
    {#if diagnostics && (diagnostics.stuck_tasks.length > 0 || diagnostics.overdue_tasks.length > 0)}
      <div class="px-4 py-2.5 border-b border-border bg-destructive/5 text-[10px] text-destructive flex flex-col gap-1">
        <div class="flex items-center gap-1.5">
          <AlertTriangle size={12} class="shrink-0" />
          <span class="font-bold">{m.autopilot_popover_diagnostics_warning()}</span>
        </div>
        {#if diagnostics.stuck_tasks.length > 0}
          <div class="ps-4 text-[9px] text-destructive/80">• {m.autopilot_popover_stuck_tasks()}: {diagnostics.stuck_tasks.length}</div>
        {/if}
        {#if diagnostics.overdue_tasks.length > 0}
          <div class="ps-4 text-[9px] text-destructive/80">• {m.autopilot_popover_overdue_tasks()}: {diagnostics.overdue_tasks.length}</div>
        {/if}
      </div>
    {/if}

    <!-- Planned Tasks List -->
    <div class="p-4 border-b border-border flex flex-col gap-3">
      <h4 class="text-xs font-bold text-foreground leading-normal text-start">
        {m.autopilot_popover_tasks_section()}
      </h4>

      {#if $tasksQuery.isLoading}
        <div class="space-y-2">
          <Skeleton class="h-6 w-full rounded-md" />
          <Skeleton class="h-6 w-full rounded-md" />
        </div>
      {:else}
        {@const sortedTasks = [...tasks].sort((a, b) => a.scheduled_at.localeCompare(b.scheduled_at))}
        {#if sortedTasks.length === 0}
          <p class="text-[10px] text-muted-foreground text-start">
            {m.autopilot_popover_no_tasks()}
          </p>
        {:else}
          <div class="max-h-[220px] overflow-y-auto scrollbar-hide mt-1 relative ps-4 border-s border-border space-y-4 text-start">
            {#each sortedTasks as task}
              {@const isCompleted = task.status === "completed"}
              {@const isFailed = task.status === "failed"}
              {@const isExecuting = task.status === "executing"}
              {@const isSuggested = task.status === "suggested"}
              
              <div class="relative">
                <!-- Timeline indicator dot center-aligned with the border-s line -->
                {#if isCompleted}
                  <span class="absolute inset-s-[-21px] top-1.5 size-2 rounded-full bg-primary border border-background"></span>
                {:else if isFailed}
                  <span class="absolute inset-s-[-21px] top-1.5 size-2 rounded-full bg-destructive border border-background"></span>
                {:else if isExecuting}
                  <span class="absolute inset-s-[-21px] top-1.5 size-2 rounded-full bg-brand-lime border border-background animate-pulse"></span>
                {:else if isSuggested}
                  <span class="absolute inset-s-[-21px] top-1.5 size-2 rounded-full bg-amber-500 border border-background animate-bounce"></span>
                {:else}
                  <span class="absolute inset-s-[-21px] top-1.5 size-2 rounded-full bg-muted-foreground/60 border border-background"></span>
                {/if}

                <div class="flex items-center justify-between gap-1.5 text-[10px]">
                  <span class="font-bold leading-tight text-foreground">
                    {getTaskTypeLabel(task.task_type)}
                  </span>
                  
                  <Badge 
                    variant={isCompleted ? 'lime' : isFailed ? 'destructive' : 'secondary'} 
                    class="text-[8px] h-3 px-1 shrink-0"
                  >
                    {#if isCompleted}
                      {m.autopilot_popover_task_status_completed()}
                    {:else if isFailed}
                      {m.autopilot_popover_task_status_failed()}
                    {:else if isExecuting}
                      {m.autopilot_popover_task_status_executing()}
                    {:else if isSuggested}
                      {m.autopilot_popover_task_status_suggested()}
                    {:else}
                      {m.autopilot_popover_task_status_pending()}
                    {/if}
                  </Badge>
                </div>
                
                <div class="text-[9px] mt-0.5 flex flex-col gap-0.5">
                  <span class="text-muted-foreground">
                    {formatDate(task.scheduled_at)}
                  </span>
                  
                  <!-- If failed, show error message -->
                  {#if isFailed && task.last_error}
                    <span class="text-destructive font-medium flex items-center gap-1">
                      <AlertCircle size={8} /> {task.last_error}
                    </span>
                  {/if}

                  <!-- Show attempt count if > 0 -->
                  {#if task.attempts > 0}
                    <span class="text-muted-foreground/80">
                      {m.autopilot_popover_attempts_count({ attempts: task.attempts, max: config?.max_auto_retries ?? 3 })}
                    </span>
                  {/if}
                </div>
              </div>
            {/each}
          </div>
        {/if}
      {/if}
    </div>

    <!-- Manual Execution Actions -->
    <div class="px-4 py-3 border-b border-border bg-muted/10">
      <Button
        variant="outline"
        size="sm"
        class="w-full text-[10px] h-8 font-semibold cursor-pointer flex items-center justify-center gap-1.5"
        onclick={handleRunTick}
        disabled={$runTickMutation.isPending}
      >
        <RefreshCw size={12} class={$runTickMutation.isPending ? "animate-spin" : ""} />
        {#if $runTickMutation.isPending}
          {m.autopilot_popover_tick_running()}
        {:else}
          {m.autopilot_popover_run_tick()}
        {/if}
      </Button>
    </div>

    <!-- Metadata Details -->
    <div class="p-4 flex flex-col gap-2.5 bg-muted/10 text-xs">
      <div class="flex items-center justify-between">
        <span class="text-muted-foreground">{m.autopilot_popover_health_status()}</span>
        <Badge 
          variant={diagnostics?.status === 'critical' ? 'destructive' : diagnostics?.status === 'warning' ? 'secondary' : 'lime'}
          class="text-[9px] h-4 px-1.5"
        >
          {diagnostics?.status === 'critical' ? 'Critical' : diagnostics?.status === 'warning' ? 'Warning' : 'Healthy'}
        </Badge>
      </div>

      <div class="flex items-center justify-between">
        <span class="text-muted-foreground">{m.autopilot_popover_timezone()}</span>
        <span class="font-medium text-foreground">{config?.timezone || "Asia/Riyadh"}</span>
      </div>

      <div class="flex items-center justify-between">
        <span class="text-muted-foreground">{m.autopilot_popover_check_interval()}</span>
        <span class="font-medium text-foreground">
          {m.autopilot_popover_minutes({ minutes: config?.check_interval_minutes ?? 60 })}
        </span>
      </div>

      {#if config?.last_tick_at}
        <div class="flex items-center justify-between">
          <span class="text-muted-foreground">{m.autopilot_popover_last_run()}</span>
          <span class="font-medium text-foreground">{formatDate(config.last_tick_at)}</span>
        </div>
      {/if}
    </div>
  </PopoverContent>
</Popover>
