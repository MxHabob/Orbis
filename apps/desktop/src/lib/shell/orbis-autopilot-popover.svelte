<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import { Popover, PopoverTrigger, PopoverContent } from "@orbis/ui/popover";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
  TooltipProvider,
} from "@orbis/ui/tooltip";
import { Skeleton } from "@orbis/ui/skeleton";
import { Activity, RefreshCw, AlertTriangle, Settings } from "@lucide/svelte";
import { academicContext } from "$lib/stores";
import { Avatar } from "@orbis/avatar-engine";

import { formatDate } from "$lib/core/i18n";
import {
  createAutopilotConfigQuery,
  createAutopilotTasksQuery,
  createAutopilotDiagnosticsQuery,
  useRunAutopilotTick,
} from "$lib/features/ai/queries";
import { toast } from "@orbis/ui";
import { extractErrorMessage } from "$lib/api";
import AutopilotSuggestions from "./autopilot-suggestions.svelte";

let { orgId = "" }: { orgId?: string } = $props();

// Autopilot Queries
const configQuery = createAutopilotConfigQuery(() => orgId);
const config = $derived(configQuery.data);

const tasksQuery = createAutopilotTasksQuery(() => orgId);
const tasks = $derived(tasksQuery.data ?? []);

const diagnosticsQuery = createAutopilotDiagnosticsQuery(() => orgId);
const diagnostics = $derived(diagnosticsQuery.data);

// Mutations
const runTickMutation = useRunAutopilotTick();

// Friendly task name mapper
function getTaskTypeLabel(type: string): string {
  switch (type) {
    case "enrollment_open":
      return m.common_autopilot_task_enrollment_open();
    case "enrollment_close":
      return m.common_autopilot_task_enrollment_close();
    case "grade_entry_open":
      return m.common_autopilot_task_grade_entry_open();
    case "grade_entry_close":
      return m.common_autopilot_task_grade_entry_close();
    case "period_lock":
      return m.autopilot_task_period_lock();
    case "report_generation":
      return m.common_autopilot_task_report_generation();
    case "backup":
      return m.common_action_back();
    case "promotion_dry_run":
      return m.common_autopilot_task_promotion_dry_run();
    default:
      return type;
  }
}

// Friendly phase label mapper
function getPhaseLabel(phase: string) {
  switch (phase) {
    case "enrollment":
      return m.common_bulk_card_students_tag_enrollment();
    case "active":
      return m.common_phase_active();
    case "exam":
      return m.common_phase_exams();
    case "grading":
      return m.common_phase_grade_entry();
    case "locked":
      return m.common_phase_locked();
    case "archived":
      return m.common_phase_closed();
    default:
      return phase;
  }
}

async function handleRunTick() {
  if (!orgId) return;
  try {
    await toast.promise(runTickMutation.mutateAsync({ orgId }), {
      loading: m.common_autopilot_popover_tick_running(),
      success: () => m.common_autopilot_popover_run_tick(),
      error: (e) =>
        extractErrorMessage(e) || m.common_autopilot_popover_run_tick(),
    });
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  }
}
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
              aria-label={m.common_autopilot_popover_title()}
            >
              <Activity size={16} strokeWidth={1.8} class="text-foreground" />
              {#if diagnostics && (((diagnostics.stuck_tasks?.length ?? 0) > 0) || ((diagnostics.overdue_tasks?.length ?? 0) > 0))}
                <span class="absolute -top-0.5 -inset-e-0.5 w-1.5 h-1.5 rounded-full bg-destructive border border-background animate-pulse"></span>
              {:else if config?.autopilot_enabled === 1}
                <span class="absolute -top-0.5 -inset-e-0.5 w-1.5 h-1.5 rounded-full bg-brand-lime border border-background"></span>
              {:else}
                <span class="absolute -top-0.5 -inset-e-0.5 w-1.5 h-1.5 rounded-full bg-muted-foreground/60 border border-background"></span>
              {/if}
            </Button>
          </TooltipTrigger>
          <TooltipContent side="right" sideOffset={12}>
            {m.common_autopilot_popover_title()}
          </TooltipContent>
        </Tooltip>
      </TooltipProvider>
    {/snippet}
  </PopoverTrigger>
  
  <PopoverContent
    side="right"
    align="end"
    sideOffset={12}
    class="w-85 rounded-2xl p-0 overflow-hidden z-50 flex flex-col font-sans text-start border border-border/70 bg-popover text-popover-foreground shadow-xl animate-in fade-in slide-in-from-left-2 duration-200"
  >
    <!-- Header -->
    <div class="flex items-center justify-between gap-3 border-b border-border/70 bg-card px-4 py-3">
      <div class="flex min-w-0 items-center gap-1.5">
        <Avatar hat="pilot" width="30px" height="30px" class="shrink-0" />
        <span class="truncate text-xs font-semibold text-foreground">{m.common_autopilot_popover_title()}</span>
        
        {#if !config}
          <Skeleton class="h-3.5 w-10 rounded" />
        {:else if config.autopilot_enabled === 0}
          <Badge variant="outline" class="text-[8px] h-3.5 px-1 font-bold uppercase text-muted-foreground">
            {m.common_status_disabled()}
          </Badge>
        {:else}
          <Badge variant="lime" class="text-[8px] h-3.5 px-1 font-bold uppercase tracking-wider">
            {m.common_phase_active()}
          </Badge>
        {/if}
      </div>
      
      <!-- Settings Redirect Icon Link -->
      <a 
        href="/{orgId}/settings/academic-policy" 
        class="flex size-7 shrink-0 items-center justify-center text-muted-foreground transition-colors hover:text-foreground focus:outline-none focus-visible:ring-2 focus-visible:ring-primary/40"
        title={m.common_autopilot_popover_config_title()}
      >
        <Settings size={14} />
      </a>
    </div>

    <!-- Diagnostics Panel (Stuck / Overdue Alerts) -->
    {#if diagnostics && (((diagnostics.stuck_tasks?.length ?? 0) > 0) || ((diagnostics.overdue_tasks?.length ?? 0) > 0))}
      <div class="px-4 py-2 border-b border-destructive/15 bg-destructive/5 text-[10px] text-destructive flex items-center gap-1.5">
          <AlertTriangle size={12} class="shrink-0" />
          <span class="font-bold">{m.common_autopilot_popover_diagnostics_warning()}</span>
          <span class="ms-auto text-[9px] text-destructive/80">{(diagnostics.stuck_tasks?.length ?? 0) + (diagnostics.overdue_tasks?.length ?? 0)}</span>
      </div>
    {/if}

    <!-- Planned Tasks List -->
    <AutopilotSuggestions 
      {tasks} 
      isLoading={tasksQuery.isLoading} 
      {config} 
      {getTaskTypeLabel} 
    />

    <!-- Manual Execution Actions -->
    <div class="border-t border-border/50 bg-card px-4 py-3">
      <Button
        class="h-9 w-full cursor-pointer rounded-lg text-[10px] font-semibold shadow-sm flex items-center justify-center gap-1.5"
        onclick={handleRunTick}
        disabled={runTickMutation.isPending}
      >
        <RefreshCw size={12} class={runTickMutation.isPending ? "animate-spin" : ""} />
        {#if runTickMutation.isPending}
          {m.common_autopilot_popover_tick_running()}
        {:else}
          {m.common_autopilot_popover_run_tick()}
        {/if}
      </Button>
    </div>
  </PopoverContent>
</Popover>
