<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
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
import {
  ChartBar,
  ChevronRight,
  InfoCircle,
  Refresh,
  Settings,
  Sparkles,
} from "@orbis/icons";
import { systemStore } from "$lib/stores";
import { goto } from "$app/navigation";
import { localizeHref, getLocale } from "$lib/paraglide/runtime";

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
import { onMount } from "svelte";

let { orgId = "" }: { orgId?: string } = $props();

const isArabic = $derived(getLocale().startsWith("ar"));

// Autopilot Queries
const configQuery = createAutopilotConfigQuery(() => orgId);
const config = $derived(configQuery.data);

const tasksQuery = createAutopilotTasksQuery(() => orgId);
const tasks = $derived(tasksQuery.data ?? []);

const diagnosticsQuery = createAutopilotDiagnosticsQuery(() => orgId);
const diagnostics = $derived(diagnosticsQuery.data);

// Mutations
const runTickMutation = useRunAutopilotTick();

// Onboarding & Readiness state
const isOperational = $derived(systemStore.isOperational);
const pendingSetupItems = $derived(systemStore.pendingSetupItems);
const hasDiagnosticsWarning = $derived(
  diagnostics &&
    ((diagnostics.stuck_tasks?.length ?? 0) > 0 ||
      (diagnostics.overdue_tasks?.length ?? 0) > 0),
);

onMount(() => {
  if (!systemStore.orgReadiness) {
    systemStore.loadReadiness();
  }
});

function handleOpenChange(open: boolean) {
  if (open) {
    systemStore.loadReadiness();
    configQuery.refetch?.();
    tasksQuery.refetch?.();
    diagnosticsQuery.refetch?.();
  }
}

// Friendly task name mapper covering all domain task types
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
      return (
        m.common_autopilot_task_period_lock?.() ||
        m.autopilot_task_period_lock?.() ||
        m.common_autopilot_task_period_lock()
      );
    case "report_generation":
      return m.common_autopilot_task_report_generation();
    case "backup":
      return (
        m.common_autopilot_task_backup?.() || m.ui_autopilot_system_backup()
      );
    case "promotion_dry_run":
      return m.common_autopilot_task_promotion_dry_run();
    case "promotion_execute":
      return (
        m.common_autopilot_task_promotion_execute?.() ||
        m.ui_autopilot_execute_student_promotion()
      );
    case "attendance_warning":
      return (
        m.common_autopilot_task_attendance_warning?.() ||
        m.ui_autopilot_attendance_warning()
      );
    case "fee_due_notice":
      return (
        m.common_autopilot_task_fee_due_notice?.() ||
        m.ui_autopilot_fee_due_notice()
      );
    case "period_transition":
      return (
        m.common_autopilot_task_period_transition?.() ||
        m.ui_autopilot_period_transition()
      );
    case "sync_calendar":
      return (
        m.common_autopilot_task_sync_calendar?.() ||
        m.ui_autopilot_sync_calendar()
      );
    case "stale_task_cleanup":
      return (
        m.common_autopilot_task_stale_task_cleanup?.() ||
        m.ui_autopilot_cleanup_stale_tasks()
      );
    case "health_check":
      return (
        m.common_autopilot_task_health_check?.() ||
        m.ui_autopilot_health_check()
      );
    default:
      return type;
  }
}

// Mode badge label
function getModeLabel(mode?: string): string {
  switch (mode) {
    case "automatic":
      return (
        m.common_autopilot_popover_mode_automatic?.() || m.ui_autopilot_auto()
      );
    case "suggested":
      return (
        m.common_autopilot_popover_mode_suggested?.() ||
        m.ui_autopilot_suggested()
      );
    case "manual":
      return (
        m.common_autopilot_popover_mode_manual?.() || m.common_dt_type_manual()
      );
    default:
      return m.common_phase_active();
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

function handleGoToOnboarding() {
  goto(localizeHref(orgId ? `/${orgId}/onboarding` : "/onboarding"));
}
</script>

<Popover onOpenChange={handleOpenChange}>
  <PopoverTrigger>
    {#snippet child({ props }: { props: any })}
      <TooltipProvider>
        <Tooltip>
          <TooltipTrigger>
            <Button 
              {...props}
              variant="outline" 
              size="icon-sm" 
              class="relative cursor-pointer"
              aria-label={m.common_autopilot_popover_title()}
            >
              <ChartBar size={16} strokeWidth={1.8} class="text-foreground" />
              
              {#if hasDiagnosticsWarning}
                <span class="absolute -top-0.5 -inset-e-0.5 size-1.5 rounded-full bg-destructive border border-background animate-pulse"></span>
              {:else if !isOperational}
                <span class="absolute -top-0.5 -inset-e-0.5 size-1.5 rounded-full bg-amber-500 border border-background"></span>
              {:else if config?.enabled}
                <span class="absolute -top-0.5 -inset-e-0.5 size-1.5 rounded-full bg-emerald-500 border border-background"></span>
              {:else}
                <span class="absolute -top-0.5 -inset-e-0.5 size-1.5 rounded-full bg-muted-foreground/60 border border-background"></span>
              {/if}
            </Button>
          </TooltipTrigger>
          <TooltipContent side="inline-end" sideOffset={12}>
            {m.common_autopilot_popover_title()}
          </TooltipContent>
        </Tooltip>
      </TooltipProvider>
    {/snippet}
  </PopoverTrigger>
  
  <PopoverContent
    side="inline-end"
    align="end"
    sideOffset={12}
    class="w-88 p-0 overflow-hidden z-50 flex flex-col font-sans text-start backdrop-blur-md animate-in fade-in zoom-in-95 duration-200 select-none"
  >
    <!-- Header -->
    <div class="flex items-center justify-between gap-3 border-b border-border/60 bg-muted/20 px-4 py-3">
      <div class="flex min-w-0 items-center gap-2">
        <div class="size-7 shrink-0 rounded-lg bg-primary/10 text-primary flex items-center justify-center font-bold text-xs">
          <Sparkles size={16} />
        </div>
        <span class="truncate text-xs font-bold text-foreground">{m.common_autopilot_popover_title()}</span>
        
        {#if !isOperational}
          <Badge variant="outline" class="text-[9px] h-4 px-1.5 font-bold uppercase border-amber-500/40 text-amber-600 dark:text-amber-400 bg-amber-500/10 shrink-0">
            {m.ui_autopilot_setup_needed()}
          </Badge>
        {:else if !config}
          <Skeleton class="h-4 w-12 rounded" />
        {:else if !config.enabled}
          <Badge variant="outline" class="text-[9px] h-4 px-1.5 font-bold uppercase text-muted-foreground shrink-0">
            {m.common_status_disabled()}
          </Badge>
        {:else}
          <Badge variant="lime" class="text-[9px] h-4 px-1.5 font-bold uppercase tracking-wider shrink-0">
            {getModeLabel(config.mode)}
          </Badge>
        {/if}
      </div>
      
      <!-- Quick Header Action Links -->
      <div class="flex items-center gap-1 shrink-0">
        <a 
          href="/{orgId}/ai-center" 
          class="flex size-7 items-center justify-center rounded-md text-muted-foreground transition-colors hover:text-primary hover:bg-muted/50 focus:outline-none focus-visible:ring-2 focus-visible:ring-primary/40"
          title={m.common_autopilot_popover_ai_center()}
        >
          <Sparkles size={14} />
        </a>

        <a 
          href="/{orgId}/ai-center/settings" 
          class="flex size-7 items-center justify-center rounded-md text-muted-foreground transition-colors hover:text-foreground hover:bg-muted/50 focus:outline-none focus-visible:ring-2 focus-visible:ring-primary/40"
          title={m.common_autopilot_popover_config_title()}
        >
          <Settings size={14} />
        </a>
      </div>
    </div>

    <!-- Onboarding Incomplete Notice -->
    {#if !isOperational}
      <div class="p-3.5 border-b border-amber-500/20 bg-amber-500/5 dark:bg-amber-500/10 flex flex-col gap-2.5 text-start">
        <div class="flex items-start gap-2.5">
          <div class="size-6 shrink-0 rounded-md bg-amber-500/15 text-amber-600 dark:text-amber-400 flex items-center justify-center mt-0.5">
            <InfoCircle size={14} />
          </div>
          <div class="flex flex-col gap-0.5 min-w-0">
            <span class="text-xs font-bold text-amber-700 dark:text-amber-400">
              {m.common_autopilot_popover_onboarding_title()}
            </span>
            <p class="text-[11px] leading-relaxed text-muted-foreground">
              {m.common_autopilot_popover_onboarding_desc()}
            </p>
          </div>
        </div>

        {#if pendingSetupItems.length > 0}
          <div class="flex flex-wrap gap-1.5 mt-0.5">
            {#each pendingSetupItems as item}
              <span class="inline-flex items-center gap-1.5 text-[10px] px-2 py-0.5 rounded-md bg-background/80 border border-border/80 text-foreground font-medium shadow-2xs">
                <span class="size-1.5 rounded-full bg-amber-500"></span>
                {isArabic && item.label_ar ? item.label_ar : item.label}
              </span>
            {/each}
          </div>
        {/if}

        <Button
          size="sm"
          variant="outline"
          onclick={handleGoToOnboarding}
          class="mt-1 h-7.5 w-full text-[11px] font-semibold border-amber-500/30 text-amber-700 dark:text-amber-300 hover:bg-amber-500/10 flex items-center justify-center gap-1.5 shadow-xs"
        >
          <span>{m.common_autopilot_popover_onboarding_btn()}</span>
          <ChevronRight size={13} class="rtl:rotate-180" />
        </Button>
      </div>
    {/if}

    <!-- Diagnostics Panel (Stuck / Overdue Alerts) -->
    {#if hasDiagnosticsWarning && diagnostics}
      <div class="px-4 py-2 border-b border-destructive/15 bg-destructive/5 text-[10px] text-destructive flex items-center gap-1.5">
        <InfoCircle size={12} class="shrink-0" />
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
    <div class="border-t border-border/50 bg-muted/20 px-4 py-3">
      <Button
        class="h-8.5 w-full cursor-pointer rounded-lg text-xs font-semibold shadow-xs flex items-center justify-center gap-1.5"
        onclick={handleRunTick}
        disabled={runTickMutation.isPending || !isOperational}
      >
        {#if runTickMutation.isPending}<LoadingIndicator  size={13} />{:else}<Refresh size={13}   />{/if}
        {#if runTickMutation.isPending}
          {m.common_autopilot_popover_tick_running()}
        {:else}
          {m.common_autopilot_popover_run_tick()}
        {/if}
      </Button>
    </div>
  </PopoverContent>
</Popover>
