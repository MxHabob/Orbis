<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Badge } from "@orbis/ui-svelte/badge";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { AlertCircle } from "lucide-svelte";

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

const sortedTasks = $derived([...tasks].sort((a, b) => a.scheduled_at.localeCompare(b.scheduled_at)));
</script>

<div class="p-4 border-b border-border flex flex-col gap-3">
  <h4 class="text-xs font-bold text-foreground leading-normal text-start">
    {m.common_autopilot_popover_tasks_section()}
  </h4>

  {#if isLoading}
    <div class="space-y-2">
      <Skeleton class="h-6 w-full rounded-md" />
      <Skeleton class="h-6 w-full rounded-md" />
    </div>
  {:else}
    {#if sortedTasks.length === 0}
      <p class="text-[10px] text-muted-foreground text-start">
        {m.common_autopilot_popover_no_tasks()}
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
                  {m.common_calendar_status_done()}
                {:else if isFailed}
                  {m.common_calendar_status_failed()}
                {:else if isExecuting}
                  {m.common_autopilot_popover_task_status_executing()}
                {:else if isSuggested}
                  {m.common_calendar_field_mode_suggested()}
                {:else}
                  {m.common_calendar_status_pending()}
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
                  {m.common_autopilot_popover_attempts_count({ attempts: task.attempts, max: config?.max_auto_retries ?? 3 })}
                </span>
              {/if}
            </div>
          </div>
        {/each}
      </div>
    {/if}
  {/if}
</div>
