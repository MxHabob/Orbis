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
import { Activity, RefreshCw, AlertTriangle, Settings, Brain, ArrowRight, Sparkles, CheckCircle2, ChevronRight } from "@lucide/svelte";
import { systemStore } from "$lib/stores";
import { Avatar } from "@orbis/avatar-engine";
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
  (((diagnostics.stuck_tasks?.length ?? 0) > 0) || ((diagnostics.overdue_tasks?.length ?? 0) > 0))
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
      return m.common_autopilot_task_period_lock?.() || m.autopilot_task_period_lock?.() || (isArabic ? "قفل الفترة الأكاديمية" : "Lock Academic Period");
    case "report_generation":
      return m.common_autopilot_task_report_generation();
    case "backup":
      return m.common_autopilot_task_backup?.() || (isArabic ? "نسخ احتياطي للنظام" : "System Backup");
    case "promotion_dry_run":
      return m.common_autopilot_task_promotion_dry_run();
    case "promotion_execute":
      return m.common_autopilot_task_promotion_execute?.() || (isArabic ? "تنفيذ ترحيل الطلاب" : "Execute Student Promotion");
    case "attendance_warning":
      return m.common_autopilot_task_attendance_warning?.() || (isArabic ? "إنذارات الغياب" : "Attendance Warning");
    case "fee_due_notice":
      return m.common_autopilot_task_fee_due_notice?.() || (isArabic ? "إشعارات استحقاق الرسوم" : "Fee Due Notice");
    case "period_transition":
      return m.common_autopilot_task_period_transition?.() || (isArabic ? "الانتقال للفترة التالية" : "Period Transition");
    case "sync_calendar":
      return m.common_autopilot_task_sync_calendar?.() || (isArabic ? "مزامنة التقويم" : "Sync Calendar");
    case "stale_task_cleanup":
      return m.common_autopilot_task_stale_task_cleanup?.() || (isArabic ? "تنظيف المهام المنتهية" : "Cleanup Stale Tasks");
    case "health_check":
      return m.common_autopilot_task_health_check?.() || (isArabic ? "فحص صحة النظام" : "Health Check");
    default:
      return type;
  }
}

// Mode badge label
function getModeLabel(mode?: string): string {
  switch (mode) {
    case "automatic":
      return m.common_autopilot_popover_mode_automatic?.() || (isArabic ? "تلقائي" : "Auto");
    case "suggested":
      return m.common_autopilot_popover_mode_suggested?.() || (isArabic ? "مقترحات" : "Suggested");
    case "manual":
      return m.common_autopilot_popover_mode_manual?.() || (isArabic ? "يدوي" : "Manual");
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
  goto(localizeHref("/onboarding"));
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
              class="relative cursor-pointer rounded-[10px]"
              aria-label={m.common_autopilot_popover_title()}
            >
              <Activity size={16} strokeWidth={1.8} class="text-foreground" />
              
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
    class="w-88 rounded-2xl p-0 overflow-hidden z-50 flex flex-col font-sans text-start border border-border/70 bg-popover text-popover-foreground shadow-xl animate-in fade-in slide-in-from-left-2 duration-200"
  >
    <!-- Header -->
    <div class="flex items-center justify-between gap-3 border-b border-border/70 bg-card px-4 py-3">
      <div class="flex min-w-0 items-center gap-1.5">
        <Avatar hat="pilot" width="30px" height="30px" class="shrink-0" />
        <span class="truncate text-xs font-semibold text-foreground">{m.common_autopilot_popover_title()}</span>
        
        {#if !isOperational}
          <Badge variant="outline" class="text-[8px] h-3.5 px-1 font-bold uppercase border-amber-500/40 text-amber-600 dark:text-amber-400 bg-amber-500/10">
            {isArabic ? "بانتظار التهيئة" : "Setup Needed"}
          </Badge>
        {:else if !config}
          <Skeleton class="h-3.5 w-10 rounded" />
        {:else if !config.enabled}
          <Badge variant="outline" class="text-[8px] h-3.5 px-1 font-bold uppercase text-muted-foreground">
            {m.common_status_disabled()}
          </Badge>
        {:else}
          <Badge variant="lime" class="text-[8px] h-3.5 px-1 font-bold uppercase tracking-wider">
            {getModeLabel(config.mode)}
          </Badge>
        {/if}
      </div>
      
      <!-- Quick Header Action Links -->
      <div class="flex items-center gap-1 shrink-0">
        <a 
          href="/{orgId}/ai-center" 
          class="flex size-7 items-center justify-center rounded-md text-muted-foreground transition-colors hover:text-primary hover:bg-muted/50 focus:outline-none focus-visible:ring-2 focus-visible:ring-primary/40"
          title={m.common_autopilot_popover_ai_center?.() || "AI Center"}
        >
          <Brain size={14} />
        </a>

        <a 
          href="/{orgId}/settings/academic-policy" 
          class="flex size-7 items-center justify-center rounded-md text-muted-foreground transition-colors hover:text-foreground hover:bg-muted/50 focus:outline-none focus-visible:ring-2 focus-visible:ring-primary/40"
          title={m.common_autopilot_popover_config_title()}
        >
          <Settings size={14} />
        </a>
      </div>
    </div>

    <!-- Onboarding Incomplete Notice -->
    {#if !isOperational}
      <div class="p-3.5 border-b border-amber-500/20 bg-amber-500/5 flex flex-col gap-2 text-start">
        <div class="flex items-start gap-2">
          <Sparkles size={14} class="text-amber-500 shrink-0 mt-0.5" />
          <div class="flex flex-col gap-0.5">
            <span class="text-[11px] font-bold text-amber-700 dark:text-amber-400">
              {m.common_autopilot_popover_onboarding_title?.() || (isArabic ? "التهيئة الأولية غير مكتملة" : "Setup Incomplete")}
            </span>
            <p class="text-[9.5px] leading-relaxed text-muted-foreground">
              {m.common_autopilot_popover_onboarding_desc?.() || (isArabic ? "يتطلب الطيار الآلي تهيئة العام الدراسي والفصول والمواد قبل بدء تشغيل دورات الأتمتة التلقائية." : "AutoPilot requires initial setup (academic year, classes, and subjects) before background automation can run.")}
            </p>
          </div>
        </div>

        {#if pendingSetupItems.length > 0}
          <div class="flex flex-wrap gap-1.5 mt-1">
            {#each pendingSetupItems as item}
              <span class="inline-flex items-center gap-1 text-[9px] px-2 py-0.5 rounded-md bg-background/80 border border-border text-foreground font-medium">
                <span class="size-1 rounded-full bg-amber-500"></span>
                {isArabic && item.label_ar ? item.label_ar : item.label}
              </span>
            {/each}
          </div>
        {/if}

        <Button
          size="sm"
          variant="outline"
          onclick={handleGoToOnboarding}
          class="mt-1.5 h-7 w-full text-[10px] font-bold border-amber-500/30 text-amber-700 dark:text-amber-300 hover:bg-amber-500/10 flex items-center justify-center gap-1.5"
        >
          <span>{m.common_autopilot_popover_onboarding_btn?.() || (isArabic ? "إكمال خطوات التهيئة" : "Complete Setup")}</span>
          <ChevronRight size={11} class="rtl:rotate-180" />
        </Button>
      </div>
    {/if}

    <!-- Diagnostics Panel (Stuck / Overdue Alerts) -->
    {#if hasDiagnosticsWarning && diagnostics}
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
        disabled={runTickMutation.isPending || !isOperational}
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
