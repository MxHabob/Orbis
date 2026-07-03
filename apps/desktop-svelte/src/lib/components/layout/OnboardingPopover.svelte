<script lang="ts">
import {
  Popover,
  PopoverTrigger,
  PopoverContent,
  Button,
  Tooltip,
  TooltipTrigger,
  TooltipContent,
} from "@orbis/ui-svelte";
import { Check, ChevronRight } from "lucide-svelte";
import { onboardingState } from "$lib/components/onboarding/onboarding-state.svelte";
import { authStore } from "$lib/stores";
import { goto } from "$app/navigation";
import { localizeHref, getLocale } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";

function handleChecklistItemClick(stepIndex: number) {
  onboardingState.goToStep(stepIndex + 1);
  goto(localizeHref("/onboarding"));
}

// Onboarding tasks variables
const totalTasks = 8;
const completedTasks = $derived(
  authStore.isSetupComplete ? 8 : onboardingState.tasksCompletedCount,
);
const remainingTasks = $derived(totalTasks - completedTasks);
const progressPercent = $derived((completedTasks / totalTasks) * 100);

const activeIndex = $derived(
  authStore.isSetupComplete
    ? -1
    : onboardingState.completedSteps.findIndex((step) => !step),
);

const tasksList = $derived([
  {
    label: m.onboarding_popover_task_institution_type_label(),
    desc: m.onboarding_popover_task_institution_type_desc(),
  },
  {
    label: m.onboarding_popover_task_calendar_label({
      calendar: onboardingState.calendarLabel || m.onboarding_review_calendar(),
    }),
    desc: m.onboarding_popover_task_calendar_desc(),
  },
  {
    label: m.onboarding_popover_task_classroom_label({
      classroom:
        onboardingState.classroomLabel || m.onboarding_term_classroom(),
    }),
    desc: m.onboarding_popover_task_classroom_desc(),
  },
  {
    label: m.onboarding_popover_task_stage_label({
      stage: onboardingState.stageLabel || m.onboarding_term_stage(),
    }),
    desc: m.onboarding_popover_task_stage_desc({
      division: onboardingState.divisionLabel || m.onboarding_term_division(),
    }),
  },
  {
    label: m.onboarding_popover_task_schedule_label(),
    desc: m.onboarding_popover_task_schedule_desc(),
  },
  {
    label: m.onboarding_popover_task_subject_label({
      subject: onboardingState.subjectLabel || m.onboarding_term_subject(),
    }),
    desc: m.onboarding_popover_task_subject_desc(),
  },
  {
    label: m.onboarding_popover_task_fees_label(),
    desc: m.onboarding_popover_task_fees_desc(),
  },
  {
    label: m.onboarding_popover_task_review_label(),
    desc: m.onboarding_popover_task_review_desc(),
  },
]);
</script>

<Popover>
  <PopoverTrigger>
    {#snippet child({ props: popoverProps })}
      <Tooltip>
        <TooltipTrigger>
          {#snippet child({ props: tooltipProps })}
            <Button
              {...popoverProps}
              {...tooltipProps}
              variant="outline"
              size="icon-sm"
              class="relative cursor-pointer rounded-[10px] onboarding-popover-trigger"
            >
              <div class="relative size-5 flex items-center justify-center">
                <!-- Progress Circular Ring SVG -->
                <svg class="absolute inset-0 size-5 select-none" viewBox="0 0 24 24" width="20" height="20">
                  <!-- Background track -->
                  <circle
                    cx="12"
                    cy="12"
                    r="9"
                    class="stroke-muted/40 fill-none"
                    stroke-width="2.5"
                  />
                  <!-- Progress highlight rotated around center -->
                  {#if progressPercent > 0}
                    <circle
                      cx="12"
                      cy="12"
                      r="9"
                      class="stroke-primary fill-none transition-all duration-300 origin-center -rotate-90"
                      stroke-width="2.5"
                      stroke-dasharray="56.5"
                      stroke-dashoffset={56.5 - (56.5 * progressPercent) / 100}
                      stroke-linecap="round"
                    />
                  {/if}
                </svg>
                
                <!-- Inner status indicator -->
                {#if remainingTasks === 0}
                  <Check class="size-3 text-primary stroke-[3px]" />
                {:else}
                  <span class="text-[8.5px] font-extrabold text-foreground leading-none">
                    {remainingTasks}
                  </span>
                  <!-- Notification dot badge shifted outside the ring -->
                  <span class="absolute -top-0.5 -right-0.5 w-1.5 h-1.5 rounded-full bg-primary border border-background"></span>
                {/if}
              </div>
            </Button>
          {/snippet}
        </TooltipTrigger>
        <TooltipContent side="right" sideOffset={12}>
          {m.onboarding_progress_label() || 'Getting Started'}
        </TooltipContent>
      </Tooltip>
    {/snippet}
  </PopoverTrigger>
  
  <PopoverContent
    side="right"
    align="end"
    sideOffset={12}
    class="w-[360px] bg-card border border-border shadow-2xl rounded-xl p-0 overflow-hidden z-50 flex flex-col font-sans text-start onboarding-popover-panel relative text-card-foreground"
  >
    <!-- Pointer Bezel/Tail pointing to the sidebar trigger (positions on left in LTR, right in RTL) -->
    <div class="absolute bottom-[20px] start-[-6px] w-3 h-3 bg-card border-l border-b border-border rotate-45 z-20 rtl:start-auto rtl:end-[-6px] rtl:border-l-0 rtl:border-b-0 rtl:border-r rtl:border-t"></div>

    <!-- Header Section -->
    <div class="flex items-center gap-3.5 p-4 border-b border-border bg-card z-10 select-none">
      <!-- Outer Ring loader SVG -->
      <svg class="w-8 h-8 transform -rotate-90 text-primary shrink-0" viewBox="0 0 24 24">
        <circle cx="12" cy="12" r="9" class="stroke-muted fill-none" stroke-width="2.5" />
        <circle cx="12" cy="12" r="9" class="stroke-primary fill-none" stroke-width="2.5" stroke-dasharray="56.5" stroke-dashoffset={56.5 - (56.5 * progressPercent) / 100} stroke-linecap="round" />
      </svg>
      <div class="flex flex-col">
        <h4 class="font-bold text-sm text-foreground leading-tight">
          {m.onboarding_progress_label() || 'Getting Started'}
        </h4>
        <p class="text-[11px] text-muted-foreground font-semibold mt-0.5 font-sans">
          {m.onboarding_popover_tasks_completed({ completed: completedTasks, total: totalTasks })}
        </p>
      </div>
    </div>

    <!-- Scrollable Checklist Area -->
    <div class="onboarding-list flex-1 overflow-y-auto max-h-[340px] p-2 flex flex-col gap-0.5 bg-card z-10">
      {#each tasksList as task, idx}
        {@const isCompleted = authStore.isSetupComplete ? true : onboardingState.completedSteps[idx]}
        {@const isActive = !isCompleted && idx === activeIndex}
        
        <button
          onclick={() => {
            if (!authStore.isSetupComplete) {
              handleChecklistItemClick(idx);
            }
          }}
          disabled={authStore.isSetupComplete}
          class="flex items-start justify-between p-3 rounded-lg hover:bg-muted text-start w-full group transition-all duration-150 cursor-pointer text-foreground bg-transparent border-0 font-sans disabled:opacity-100 disabled:hover:bg-transparent"
        >
          <div class="flex items-start gap-3 flex-1 min-w-0">
            {#if isCompleted}
              <div class="mt-0.5 shrink-0 text-muted-foreground">
                <Check class="w-3.5 h-3.5 stroke-[3px]" />
              </div>
            {:else}
              <!-- Spacer to align texts of non-completed items -->
              <div class="w-3.5 h-3.5 shrink-0 mt-0.5"></div>
            {/if}
            
            <div class="flex flex-col gap-1 flex-1 min-w-0">
              <span class="text-[11.5px] leading-tight font-sans {isCompleted ? 'text-muted-foreground line-through decoration-muted-foreground/45 font-normal' : 'text-foreground font-bold'}">
                {task.label}
              </span>
              <span class="text-[10.5px] leading-relaxed font-sans font-normal {isCompleted ? 'text-muted-foreground/60' : isActive ? 'text-foreground/80' : 'text-muted-foreground/75'}">
                {task.desc}
              </span>
            </div>
          </div>
          
          {#if isActive}
            <div class="mt-1 shrink-0 text-muted-foreground ml-2 rtl:rotate-180">
              <ChevronRight size={12} strokeWidth={2.5} />
            </div>
          {/if}
        </button>
      {/each}
    </div>
  </PopoverContent>
</Popover>

<style>
  :global(.onboarding-list) {
    scrollbar-width: thin;
    scrollbar-color: var(--border) transparent;
  }
  :global(.onboarding-list)::-webkit-scrollbar {
    width: 3px !important;
    display: block !important;
  }
  :global(.onboarding-list)::-webkit-scrollbar-track {
    background: transparent !important;
  }
  :global(.onboarding-list)::-webkit-scrollbar-thumb {
    background-color: var(--border) !important;
    border-radius: 9999px !important;
  }
  :global(.onboarding-list)::-webkit-scrollbar-thumb:hover {
    background-color: rgba(var(--foreground), 0.25) !important;
  }
</style>
