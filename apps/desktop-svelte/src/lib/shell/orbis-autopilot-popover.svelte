import * as m from "$lib/paraglide/messages";
<script lang="ts">
import { Button } from "@orbis/ui-svelte/button";
import { Badge } from "@orbis/ui-svelte/badge";
import {
  Popover,
  PopoverTrigger,
  PopoverContent,
} from "@orbis/ui-svelte/popover";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
  TooltipProvider,
} from "@orbis/ui-svelte/tooltip";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import {
  Activity,
  RefreshCw,
  AlertTriangle,
  AlertCircle,
  Settings,
} from "lucide-svelte";
import { academicContext } from "$lib/stores";
import * as common from "$lib/paraglide/messages/common";
import { formatDate } from "$lib/core/i18n";
import {
  createAutopilotConfigQuery,
  createAutopilotTasksQuery,
  createAutopilotDiagnosticsQuery,
  useRunAutopilotTick,
} from "$lib/features/ai/queries";
import { toast } from "svelte-sonner";
import { extractErrorMessage } from "$lib/core/ipc";
import AutopilotSuggestions from "./autopilot-suggestions.svelte";

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
      return common.autopilot_task_enrollment_open();
    case "enrollment_close":
      return common.autopilot_task_enrollment_close();
    case "grade_entry_open":
      return common.autopilot_task_grade_entry_open();
    case "grade_entry_close":
      return common.autopilot_task_grade_entry_close();
    case "period_lock":
      return m.autopilot_task_period_lock();
    case "report_generation":
      return common.autopilot_task_report_generation();
    case "backup":
      return common.action_back();
    case "promotion_dry_run":
      return common.autopilot_task_promotion_dry_run();
    default:
      return type;
  }
}

// Friendly phase label mapper
function getPhaseLabel(phase: string) {
  switch (phase) {
    case "enrollment":
      return common.phase_enrollment();
    case "active":
      return common.phase_active();
    case "exam":
      return common.phase_exams();
    case "grading":
      return common.phase_grade_entry();
    case "locked":
      return common.phase_locked();
    case "archived":
      return common.phase_closed();
    default:
      return phase;
  }
}

async function handleRunTick() {
  if (!orgId) return;
  try {
    toast.promise($runTickMutation.mutateAsync({ orgId }), {
      loading: common.autopilot_popover_tick_running(),
      success: () => common.autopilot_popover_run_tick(),
      error: (e) => extractErrorMessage(e) || common.autopilot_popover_run_tick(),
    });
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  }
}

const phaseLabel = $derived(getPhaseLabel(academicContext.periodPhase));
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
              aria-label={common.autopilot_popover_title()}
            >
              <Activity size={16} strokeWidth={1.8} class="text-foreground" />
              {#if diagnostics && (diagnostics.stuck_tasks.length > 0 || diagnostics.overdue_tasks.length > 0)}
                <span class="absolute -top-0.5 -inset-e-0.5 w-1.5 h-1.5 rounded-full bg-destructive border border-background animate-pulse"></span>
              {:else if config?.autopilot_enabled === 1}
                <span class="absolute -top-0.5 -inset-e-0.5 w-1.5 h-1.5 rounded-full bg-brand-lime border border-background"></span>
              {:else}
                <span class="absolute -top-0.5 -inset-e-0.5 w-1.5 h-1.5 rounded-full bg-muted-foreground/60 border border-background"></span>
              {/if}
            </Button>
          </TooltipTrigger>
          <TooltipContent side="right" sideOffset={12}>
            {common.autopilot_popover_title()}
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
        <span class="text-xs font-semibold text-foreground">{common.autopilot_popover_title()}</span>
        
        {#if !config}
          <Skeleton class="h-3.5 w-10 rounded" />
        {:else if config.autopilot_enabled === 0}
          <Badge variant="outline" class="text-[8px] h-3.5 px-1 font-bold uppercase text-muted-foreground">
            {common.autopilot_popover_status_disabled()}
          </Badge>
        {:else}
          <Badge variant="lime" class="text-[8px] h-3.5 px-1 font-bold uppercase tracking-wider">
            {common.autopilot_popover_status_active()}
          </Badge>
        {/if}
      </div>
      
      <!-- Settings Redirect Icon Link -->
      <a 
        href="/{orgId}/settings/academic-policy" 
        class="text-muted-foreground hover:text-foreground transition-colors shrink-0"
        title={common.autopilot_popover_config_title()}
      >
        <Settings size={14} />
      </a>
    </div>

    <!-- Status Details Panel -->
    {#if config && config.autopilot_enabled !== 0}
      <div class="px-4 py-2 border-b border-border bg-background flex flex-col gap-1.5 text-[10px]">
        <div class="flex items-center justify-between">
          <span class="text-muted-foreground">الوضع:</span>
          <span class="font-medium text-foreground">{config.autopilot_mode === "strict" ? "صارم" : config.autopilot_mode === "relaxed" ? "مرن" : "عادي"}</span>
        </div>
        {#if diagnostics?.last_run_at}
          <div class="flex items-center justify-between">
            <span class="text-muted-foreground">آخر فحص:</span>
            <span class="font-medium text-foreground" dir="ltr">{formatDate(diagnostics.last_run_at, "datetime")}</span>
          </div>
        {/if}
        {#if diagnostics?.next_run_at}
          <div class="flex items-center justify-between">
            <span class="text-muted-foreground">الفحص القادم:</span>
            <span class="font-medium text-foreground" dir="ltr">{formatDate(diagnostics.next_run_at, "datetime")}</span>
          </div>
        {/if}
      </div>
    {/if}

    <!-- Diagnostics Panel (Stuck / Overdue Alerts) -->
    {#if diagnostics && (diagnostics.stuck_tasks.length > 0 || diagnostics.overdue_tasks.length > 0)}
      <div class="px-4 py-2.5 border-b border-border bg-destructive/5 text-[10px] text-destructive flex flex-col gap-1">
        <div class="flex items-center gap-1.5">
          <AlertTriangle size={12} class="shrink-0" />
          <span class="font-bold">{common.autopilot_popover_diagnostics_warning()}</span>
        </div>
        {#if diagnostics.stuck_tasks.length > 0}
          <div class="ps-4 text-[9px] text-destructive/80">• {common.autopilot_popover_stuck_tasks()}: {diagnostics.stuck_tasks.length}</div>
        {/if}
        {#if diagnostics.overdue_tasks.length > 0}
          <div class="ps-4 text-[9px] text-destructive/80">• {common.autopilot_popover_overdue_tasks()}: {diagnostics.overdue_tasks.length}</div>
        {/if}
      </div>
    {/if}

    <!-- Planned Tasks List -->
    <AutopilotSuggestions 
      {tasks} 
      isLoading={$tasksQuery.isLoading} 
      {config} 
      {getTaskTypeLabel} 
    />

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
          {common.autopilot_popover_tick_running()}
        {:else}
          {common.autopilot_popover_run_tick()}
        {/if}
      </Button>
    </div>

    <!-- Metadata Details -->
    <div class="p-4 flex flex-col gap-2.5 bg-muted/10 text-xs">
      <div class="flex items-center justify-between">
        <span class="text-muted-foreground">{common.autopilot_popover_health_status()}</span>
        <Badge 
          variant={diagnostics?.status === 'critical' ? 'destructive' : diagnostics?.status === 'warning' ? 'secondary' : 'lime'}
          class="text-[9px] h-4 px-1.5"
        >
          {diagnostics?.status === 'critical' ? 'Critical' : diagnostics?.status === 'warning' ? 'Warning' : 'Healthy'}
        </Badge>
      </div>
    </div>
  </PopoverContent>
</Popover>
