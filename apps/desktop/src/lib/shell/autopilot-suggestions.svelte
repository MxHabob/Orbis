<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Badge } from "@orbis/ui/badge";
import { Skeleton } from "@orbis/ui/skeleton";
import { ClockRotateRight, InfoCircle, Sparkles } from "@orbis/icons";

import { formatDate } from "$lib/core/i18n";

let {
  tasks,
  isLoading,
  config,
  getTaskTypeLabel,
}: {
  tasks: any[];
  isLoading: boolean;
  config: any;
  getTaskTypeLabel: (type: string) => string;
} = $props();

const sortedTasks = $derived(
  [...tasks].sort((a, b) =>
    (a.scheduled_at || "").localeCompare(b.scheduled_at || ""),
  ),
);
</script>

<div class="flex flex-col gap-3 p-4">
  {#if sortedTasks.length > 0 || isLoading}
    <div class="flex items-center justify-between">
      <h4 class="text-[11px] font-bold text-muted-foreground leading-normal text-start uppercase tracking-wider">
        {m.common_autopilot_popover_tasks_section()}
      </h4>
      {#if sortedTasks.length > 0}
        <span class="text-[10px] font-mono text-muted-foreground bg-muted/40 px-1.5 py-0.5 rounded">
          {sortedTasks.length}
        </span>
      {/if}
    </div>
  {/if}

  {#if isLoading}
    <div class="space-y-2">
      <Skeleton class="h-10 w-full rounded-lg" />
      <Skeleton class="h-10 w-full rounded-lg" />
    </div>
  {:else}
    {#if sortedTasks.length === 0}
      <div class="flex min-h-[140px] flex-col items-center justify-center gap-2 rounded-xl bg-muted/20 border border-border/40 px-4 py-6 text-center">
        <div class="size-10 rounded-xl bg-primary/10 text-primary flex items-center justify-center shadow-inner">
          <Sparkles size={20} />
        </div>
        <span class="flex items-center gap-1.5 text-xs font-semibold text-foreground">
          <span class="size-1.5 rounded-full bg-emerald-500" aria-hidden="true"></span>
          {m.common_phase_active()}
        </span>
        <p class="max-w-[240px] text-[11px] leading-relaxed text-muted-foreground">
          {m.common_autopilot_popover_no_tasks()}
        </p>
      </div>
    {:else}
      <div class="max-h-[220px] overflow-y-auto scrollbar-hide space-y-2 text-start">
        {#each sortedTasks as task}
          {@const isCompleted = task.status === "completed"}
          {@const isFailed = task.status === "failed"}
          {@const isExecuting = task.status === "executing"}
          {@const isSuggested = task.status === "suggested"}
          {@const errorMsg = task.error_log ?? task.last_error}
          
          <div class="p-2.5 rounded-lg border border-border/50 bg-card/60 hover:bg-card hover:border-border/80 transition-colors flex flex-col gap-1.5">
            <div class="flex items-center justify-between gap-1.5 text-xs">
              <div class="flex items-center gap-1.5 min-w-0">
                {#if isCompleted}
                  <span class="size-1.5 rounded-full bg-emerald-500 shrink-0"></span>
                {:else if isFailed}
                  <span class="size-1.5 rounded-full bg-destructive shrink-0"></span>
                {:else if isExecuting}
                  <span class="size-1.5 rounded-full bg-primary shrink-0 animate-pulse"></span>
                {:else if isSuggested}
                  <span class="size-1.5 rounded-full bg-amber-500 shrink-0"></span>
                {:else}
                  <span class="size-1.5 rounded-full bg-muted-foreground/60 shrink-0"></span>
                {/if}
                <span class="font-semibold text-foreground truncate">
                  {getTaskTypeLabel(task.task_type)}
                </span>
              </div>
              
              <Badge 
                variant={isCompleted ? 'lime' : isFailed ? 'destructive' : isExecuting ? 'default' : isSuggested ? 'warning' : 'secondary'} 
                class="text-[9px] h-4 px-1.5 shrink-0"
              >
                {#if isCompleted}
                  {m.common_schedule_ai_status_done()}
                {:else if isFailed}
                  {m.common_calendar_status_failed()}
                {:else if isExecuting}
                  {m.common_promo_job_status_executing()}
                {:else if isSuggested}
                  {m.common_calendar_field_mode_suggested()}
                {:else}
                  {m.common_calendar_status_pending()}
                {/if}
              </Badge>
            </div>
            
            <div class="text-[10px] flex items-center justify-between text-muted-foreground">
              <span class="flex items-center gap-1">
                <ClockRotateRight size={10} class="shrink-0" />
                {formatDate(task.scheduled_at)}
              </span>
              
              {#if task.attempts > 0}
                <span class="text-[9px] font-mono">
                  {m.common_autopilot_popover_attempts_count({ attempts: task.attempts, max: task.max_retries ?? config?.max_retries ?? 3 })}
                </span>
              {/if}
            </div>

            {#if isFailed && errorMsg}
              <div class="text-destructive bg-destructive/10 border border-destructive/20 rounded p-1.5 font-medium flex items-center gap-1.5 text-[9px]">
                <InfoCircle size={10} class="shrink-0" />
                <span class="truncate">{errorMsg}</span>
              </div>
            {/if}
          </div>
        {/each}
      </div>
    {/if}
  {/if}
</div>
