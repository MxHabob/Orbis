<!--
  @component OnboardingPopover
  Sidebar & Global Popover presenting the complete 8-stage Onboarding & Organization Setup matrix.
  Follows the same sleek sidebar button and popover pattern as GitIntegrationPopover & AutomationStatusPopover.
-->
<script lang="ts">
import { goto } from "$app/navigation";
import { localizeHref } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";
import { systemStore, authStore } from "$lib/stores";
import { Popover, PopoverTrigger, PopoverContent } from "@orbis/ui/popover";
import { Tooltip, TooltipContent, TooltipTrigger } from "@orbis/ui";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import { Progress } from "@orbis/ui/progress";
import { TaskList } from "@orbis/icons";
import { deriveSteps } from "./onboarding-steps";
import { type Snippet } from "svelte";

let {
  orgId = authStore.orgId ?? "",
  side = "right",
  align = "end",
  trigger = undefined,
}: {
  orgId?: string;
  side?: "top" | "bottom" | "left" | "right";
  align?: "start" | "center" | "end";
  trigger?: Snippet;
} = $props();

let isOpen = $state(false);

const tasks = $derived(systemStore.onboardingTasks);
const steps = $derived(deriveSteps(tasks));
const progress = $derived(systemStore.onboardingProgressPercentage);
const isReady = $derived(systemStore.isInstitutionReady);
const requiredTasks = $derived(tasks.filter((t) => t.is_required));
const completedRequiredTasks = $derived(requiredTasks.filter((t) => t.status === "completed"));

const nextStep = $derived(
  steps.find((s) => s.status !== "completed") ?? steps[0]
);

async function handleOpenChange(open: boolean) {
  isOpen = open;
  if (open && orgId) {
    await systemStore.loadOnboardingState(orgId);
  }
}

function handleNavigate(route: string) {
  isOpen = false;
  if (orgId) {
    goto(localizeHref(`/${orgId}/onboarding/${route}`));
  }
}

function handleOpenHub() {
  isOpen = false;
  if (orgId) {
    goto(localizeHref(`/${orgId}/onboarding`));
  }
}
</script>

<Popover bind:open={isOpen} onOpenChange={handleOpenChange}>
  <PopoverTrigger>
    {#if trigger}
      {@render trigger()}
    {:else}
      {#snippet child({ props }: { props: any })}
        <Tooltip>
          <TooltipTrigger>
            <Button
              {...props}
              variant="outline"
              size="icon-sm"
              class="relative cursor-pointer rounded-[10px]"
              aria-label={m.onboarding_popover_title()}
            >
              <TaskList size={16} strokeWidth={1.8} class="text-foreground" />
              {#if !isReady}
                <span class="absolute -top-0.5 -inset-e-0.5 w-1.5 h-1.5 rounded-full bg-primary animate-pulse"></span>
              {:else}
                <span class="absolute -top-0.5 -inset-e-0.5 w-1.5 h-1.5 rounded-full bg-emerald-500"></span>
              {/if}
            </Button>
          </TooltipTrigger>
          <TooltipContent side="right" sideOffset={12}>
            {m.onboarding_popover_title()} ({progress}%)
          </TooltipContent>
        </Tooltip>
      {/snippet}
    {/if}
  </PopoverTrigger>

  <PopoverContent
    {side}
    {align}
    sideOffset={12}
    class="w-84 sm:w-96 p-0 overflow-hidden z-50 rounded-2xl border border-border/80 bg-popover text-popover-foreground shadow-2xl backdrop-blur-xl animate-in fade-in zoom-in-95 duration-200 select-none"
  >
    <!-- Header -->
    <div class="p-4 border-b border-border/50 bg-muted/20 space-y-2.5">
      <div class="flex items-center justify-between gap-2">
        <div class="flex items-center gap-2 min-w-0">
          <div class="size-7 shrink-0 rounded-lg bg-primary/10 text-primary flex items-center justify-center font-bold text-xs">
            🎯
          </div>
          <div class="truncate">
            <h4 class="text-xs font-bold text-foreground truncate">
              {m.onboarding_popover_title()}
            </h4>
            <p class="text-[11px] text-muted-foreground truncate">
              {m.onboarding_popover_subtitle()}
            </p>
          </div>
        </div>

        {#if isReady}
          <Badge variant="outline" class="border-emerald-500/30 bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 text-[10px] shrink-0">
            {m.onboarding_step8_ready_badge()}
          </Badge>
        {:else}
          <Badge variant="outline" class="border-primary/30 bg-primary/10 text-primary font-mono text-[10px] shrink-0">
            {progress}%
          </Badge>
        {/if}
      </div>

      <!-- Linear Progress Bar -->
      <div class="space-y-1">
        <Progress value={progress} max={100} class="h-1.5 bg-muted" />
        <div class="flex items-center justify-between text-[10px] text-muted-foreground">
          <span>
            {m.onboarding_step8_required_tasks_count({
              completed: completedRequiredTasks.length,
              total: requiredTasks.length,
            })}
          </span>
          <span class="font-mono font-medium">{progress}%</span>
        </div>
      </div>
    </div>

    <!-- Steps List -->
    <div class="max-h-72 overflow-y-auto divide-y divide-border/30 p-1">
      {#each steps as step (step.id)}
        <button
          type="button"
          onclick={() => handleNavigate(step.route)}
          class="w-full flex items-center justify-between p-2.5 rounded-lg text-start transition-colors hover:bg-muted/40 cursor-pointer group"
        >
          <div class="flex items-center gap-2.5 min-w-0">
            <!-- Step Badge Number / Check -->
            <div
              class="size-6 shrink-0 rounded-full flex items-center justify-center text-[11px] font-bold transition-transform group-hover:scale-105 {step.status ===
              'completed'
                ? 'bg-emerald-500 text-white shadow-xs'
                : step.status === 'in_progress'
                ? 'bg-blue-500 text-white ring-2 ring-blue-500/20'
                : 'bg-muted text-muted-foreground border border-border'}"
            >
              {#if step.status === "completed"}
                ✓
              {:else}
                {step.order}
              {/if}
            </div>

            <div class="min-w-0">
              <div class="text-xs font-semibold text-foreground truncate flex items-center gap-1.5">
                <span class="truncate">{step.label}</span>
                {#if step.isRequired}
                  <span class="text-[10px] text-amber-500 font-bold">*</span>
                {/if}
              </div>
              <p class="text-[10px] text-muted-foreground truncate">
                {step.description}
              </p>
            </div>
          </div>

          <div class="shrink-0 flex items-center gap-1.5 ms-2">
            {#if step.status === "completed"}
              <span class="text-[10px] text-emerald-600 dark:text-emerald-400 font-medium">
                {m.onboarding_step8_status_ready()}
              </span>
            {:else if step.status === "in_progress"}
              <span class="text-[10px] text-blue-600 dark:text-blue-400 font-semibold">
                {step.progress}%
              </span>
            {:else}
              <span class="text-[10px] text-muted-foreground">
                {m.onboarding_step8_status_pending()}
              </span>
            {/if}
            <span class="text-muted-foreground group-hover:text-foreground text-xs transition-transform group-hover:translate-x-0.5">
              ›
            </span>
          </div>
        </button>
      {/each}
    </div>

    <!-- Footer Actions -->
    <div class="p-3 border-t border-border/50 bg-muted/20 flex items-center justify-between gap-2">
      <Button
        variant="ghost"
        size="sm"
        onclick={handleOpenHub}
        class="text-xs text-muted-foreground hover:text-foreground h-8 px-2.5"
      >
        <span>{m.onboarding_popover_view_hub()}</span>
      </Button>

      {#if nextStep && !isReady}
        <Button
          variant="default"
          size="sm"
          onclick={() => handleNavigate(nextStep.route)}
          class="text-xs font-semibold h-8 px-3 shadow-xs"
        >
          <span>{m.onboarding_popover_resume_wizard()} →</span>
        </Button>
      {/if}
    </div>
  </PopoverContent>
</Popover>
