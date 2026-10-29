<!--
  @component OnboardingPopover
  Sleek, minimalist Getting Started popover matching the Linear/Sentry design pattern.
  Features a circular progress ring, clean typography, strikethrough completed tasks, and active task indicators.
-->
<script lang="ts">
import { goto } from "$app/navigation";
import { resolve } from "$app/paths";
import { localizeHref } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";
import { effectiveContext, systemStore, authStore } from "$lib/stores";
import { Popover, PopoverTrigger, PopoverContent } from "@orbis/ui/popover";
import { Tooltip, TooltipContent, TooltipTrigger } from "@orbis/ui";
import { Button } from "@orbis/ui/button";
import { TaskList, Check, ChevronRight } from "@orbis/icons";
import {
  deriveSteps,
  findNextIncompleteStep,
  type OnboardingStep,
} from "./onboarding-steps";
import { type Snippet } from "svelte";
import { createOnboardingStateQuery } from "./onboarding-queries";

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
let isLoadingOnboarding = $state(false);
let onboardingLoadError = $state(false);
const onboardingStateQuery = createOnboardingStateQuery(() => orgId, false);

const tasks = $derived(systemStore.onboardingTasks);
const steps = $derived(
  deriveSteps(tasks, (permission) =>
    effectiveContext.hasPermission(permission),
  ),
);
const progress = $derived(systemStore.onboardingProgressPercentage);
const isReady = $derived(systemStore.isInstitutionReady);
const completedStepsCount = $derived(
  steps.filter((s) => s.status === "completed").length,
);

const nextStep = $derived(findNextIncompleteStep(steps));

async function handleOpenChange(open: boolean) {
  isOpen = open;
  if (open && orgId) {
    isLoadingOnboarding = true;
    onboardingLoadError = false;
    systemStore.onboardingState = null;
    try {
      const result = await onboardingStateQuery.refetch();
      if (result.isError || !result.data) {
        systemStore.onboardingState = null;
        onboardingLoadError = true;
      } else {
        systemStore.onboardingState = result.data;
      }
    } catch (error) {
      console.error(
        "Failed to load onboarding state for the progress menu:",
        error,
      );
      systemStore.onboardingState = null;
      onboardingLoadError = true;
    } finally {
      isLoadingOnboarding = false;
    }
  }
}

function handleNavigate(step: OnboardingStep) {
  if (!step.isAccessible) return;
  isOpen = false;
  if (orgId) {
    goto(resolve(localizeHref(`/${orgId}/onboarding/${step.route}`)));
  }
}

function handleOpenHub() {
  isOpen = false;
  if (orgId) {
    goto(resolve(localizeHref(`/${orgId}/onboarding`)));
  }
}
</script>

<Popover bind:open={isOpen} onOpenChange={handleOpenChange}>
  <PopoverTrigger>
    {#snippet child({ props }: { props: any })}
      {#if trigger}
        {@render trigger()}
      {:else}
        <Tooltip>
          <TooltipTrigger>
            <Button
              {...props}
              variant="outline"
              size="icon-sm"
              class="relative cursor-pointer"
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
          <TooltipContent side="inline-end" sideOffset={12}>
            {m.onboarding_popover_title()} ({progress}%)
          </TooltipContent>
        </Tooltip>
      {/if}
    {/snippet}
  </PopoverTrigger>

  <PopoverContent
    {side}
    {align}
    sideOffset={12}
    class="w-72 sm:w-[320px] p-5 overflow-hidden z-50 backdrop-blur-md animate-in fade-in zoom-in-95 duration-200 select-none flex flex-col gap-4"
  >
    {#if isLoadingOnboarding}
      <p class="text-sm text-muted-foreground" aria-live="polite">
        {m.onboarding_state_loading()}
      </p>
    {:else if onboardingLoadError}
      <p class="text-sm text-destructive" role="alert">
        {m.onboarding_state_load_failed()}
      </p>
    {/if}
    <!-- Header with Circular Progress Ring -->
    <div class="flex items-start gap-3.5 pb-3.5 border-b border-border/40">
      <!-- Circular Progress Ring SVG -->
      <div class="relative size-8 shrink-0 flex items-center justify-center mt-0.5">
        <svg class="size-8 -rotate-90 transform" viewBox="0 0 28 28">
          <!-- Track Circle -->
          <circle
            cx="14"
            cy="14"
            r="11"
            fill="transparent"
            stroke="currentColor"
            stroke-width="2.5"
            class="text-muted/40"
          />
          <!-- Progress Arc Circle -->
          <circle
            cx="14"
            cy="14"
            r="11"
            fill="transparent"
            stroke="currentColor"
            stroke-width="2.5"
            stroke-dasharray="69.115"
            stroke-dashoffset={69.115 - (69.115 * Math.min(Math.max(progress, 0), 100)) / 100}
            stroke-linecap="round"
            class="text-primary transition-all duration-500 ease-out"
          />
        </svg>
        {#if isReady}
          <Check class="absolute size-3.5 text-emerald-500" strokeWidth={2.5} />
        {/if}
      </div>

      <!-- Header Title & Subtitle -->
      <div class="flex flex-col gap-0.5 min-w-0">
        <h4 class="text-sm font-bold text-foreground truncate">
          {m.onboarding_popover_title()}
        </h4>
        <p class="text-xs font-medium text-primary truncate">
          {m.common_onboarding_popover_tasks_completed({
            completed: completedStepsCount,
            total: steps.length,
          })}
        </p>
      </div>
    </div>

    <!-- Steps List -->
    <div class="max-h-80 overflow-y-auto scrollbar-hide space-y-1 -mx-2 px-2">
      {#each steps as step (step.id)}
        {@const isCompleted = step.status === "completed"}
        {@const isActive = step.id === nextStep?.id && !isReady}

        <Button
          type="button"
          variant="ghost"
          size="sm"
          disabled={!step.isAccessible}
          onclick={() => handleNavigate(step)}
          class="group w-full flex items-start gap-3 p-2.5 rounded-xl text-start transition-colors hover:bg-muted/40 enabled:cursor-pointer disabled:cursor-not-allowed disabled:opacity-50"
        >
          <!-- Status Icon / Check Indicator -->
          <div class="size-5 shrink-0 flex items-center justify-center mt-0.5">
            {#if isCompleted}
              <Check size={15} strokeWidth={2.2} class="text-muted-foreground/60 group-hover:text-primary transition-colors" />
            {:else if isActive}
              <span class="size-2 rounded-full bg-primary ring-4 ring-primary/20 animate-pulse"></span>
            {:else}
              <span class="size-2 rounded-full bg-muted-foreground/30"></span>
            {/if}
          </div>

          <!-- Step Info -->
          <div class="flex-1 min-w-0">
            <div class="flex items-center justify-between gap-2">
              <span
                class="text-xs truncate {isCompleted
                  ? 'line-through text-muted-foreground/60 font-medium'
                  : isActive
                  ? 'text-foreground font-bold'
                  : 'text-muted-foreground/90 font-semibold'}"
              >
                {step.label}
              </span>

              {#if isActive}
                <ChevronRight
                  size={14}
                  class="text-muted-foreground group-hover:text-foreground shrink-0 transition-transform group-hover:translate-x-0.5 rtl:rotate-180 rtl:group-hover:-translate-x-0.5"
                />
              {/if}
            </div>

            {#if step.description}
              <p
                class="text-[11px] leading-relaxed mt-0.5 {isCompleted
                  ? 'text-muted-foreground/40'
                  : isActive
                  ? 'text-muted-foreground'
                  : 'text-muted-foreground/60'}"
              >
                {step.description}
              </p>
            {/if}
          </div>
        </Button>
      {/each}
    </div>

    <!-- Footer Actions -->
    <div class="pt-3 border-t border-border/40 flex items-center justify-between gap-2">
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
          onclick={() => handleNavigate(nextStep)}
          class="text-xs font-semibold h-8 px-3 shadow-xs"
        >
          <span>{m.onboarding_popover_resume_wizard()} →</span>
        </Button>
      {/if}
    </div>
  </PopoverContent>
</Popover>
