<script lang="ts">
  import { page } from "$app/state";
  import { onMount } from "svelte";
  import type { Snippet } from "svelte";
  import * as m from "$lib/paraglide/messages";
  import { getLocale } from "$lib/paraglide/runtime";
  import { systemStore } from "$lib/stores";
  import { Button } from "@orbis/ui/button";
  import { deriveSteps, findNextIncompleteStep } from "./onboarding-steps";
  import { navigateOnboardingRoute } from "./navigation";
  import {
    ChevronLeft,
    ChevronRight,
    XMark,
    Check,
    TaskList,
  } from "@orbis/icons";

  let { orgId, children }: { orgId: string; children: Snippet } = $props();

  const currentPath = $derived(page.url.pathname);
  const isRtl = $derived(getLocale().startsWith("ar"));

  onMount(async () => {
    await systemStore.loadOnboardingState(orgId);
  });

  const steps = $derived(deriveSteps(systemStore.onboardingTasks));
  const overallProgress = $derived(systemStore.onboardingProgressPercentage);

  const currentStepIndex = $derived(
    steps.findIndex((step) => currentPath.endsWith(`/onboarding/${step.route}`))
  );

  async function exitSetup() {
    await navigateOnboardingRoute(`/${orgId}`);
  }

  async function goToOverview() {
    await navigateOnboardingRoute(`/${orgId}/onboarding`);
  }

  async function goToStep(index: number) {
    if (index >= 0 && index < steps.length) {
      await navigateOnboardingRoute(`/${orgId}/onboarding/${steps[index].route}`);
    }
  }

  async function goToPrev() {
    if (currentStepIndex > 0) {
      await goToStep(currentStepIndex - 1);
    } else if (currentStepIndex === 0) {
      await goToOverview();
    }
  }

  async function goToNext() {
    if (currentStepIndex === -1) {
      const next = findNextIncompleteStep(steps);
      if (next) {
        await navigateOnboardingRoute(`/${orgId}/onboarding/${next.route}`);
      } else {
        await goToStep(0);
      }
    } else if (currentStepIndex < steps.length - 1) {
      await goToStep(currentStepIndex + 1);
    } else {
      await exitSetup();
    }
  }
</script>

<div class="relative flex h-full w-full min-h-0 flex-col overflow-hidden bg-background font-sans text-foreground selection:bg-primary/20">
  <!-- Ambient background glow for high-end feel -->
  <div class="pointer-events-none absolute inset-0 -z-10 overflow-hidden" aria-hidden="true">
    <div class="absolute -top-[20%] left-1/2 -translate-x-1/2 h-[450px] w-[750px] bg-primary/5 blur-3xl dark:bg-primary/10"></div>
  </div>

  <!-- Floating Header (Centered Top, Minimalist, No Titles or Text) -->
  <header
    class="absolute top-4 left-1/2 -translate-x-1/2 z-30 flex items-center gap-2 sm:gap-3 bg-accent/30  backdrop-blur-md px-3 py-1.5 max-w-[calc(100vw-2rem)]"
  >
    <!-- Step Navigation Indicator Dots / Pills -->
    <nav class="flex items-center gap-1.5 px-1 overflow-x-auto scrollbar-hide" aria-label={m.onboarding_stages_title()}>
      {#each steps as step, index (step.id)}
        {@const isActive = index === currentStepIndex}
        {@const isCompleted = step.status === "completed"}
        <button
          type="button"
          disabled={!step.isAccessible}
          onclick={() => goToStep(index)}
          class="group relative flex items-center justify-center p-1 rounded-full transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary disabled:opacity-40 disabled:cursor-not-allowed"
          title={step.label}
          aria-label={step.label}
          aria-current={isActive ? "step" : undefined}
        >
          {#if isActive}
            <span class="h-2 w-6 rounded-full bg-primary shadow-xs shadow-primary/40 transition-all duration-300"></span>
          {:else if isCompleted}
            <span class="size-2 rounded-full bg-primary/70 transition-all duration-200 group-hover:scale-125"></span>
          {:else}
            <span class="size-2 rounded-full bg-muted-foreground/30 transition-all duration-200 group-hover:bg-muted-foreground/60 group-hover:scale-125"></span>
          {/if}
        </button>
      {/each}
    </nav>

    <!-- Subtle Divider -->
    <div class="h-3.5 w-px bg-border/60 shrink-0" aria-hidden="true"></div>

    <!-- Circular Progress SVG Ring -->
    <div
      class="relative flex size-5 shrink-0 items-center justify-center"
      title={`${overallProgress}%`}
      aria-label={`${overallProgress}% completed`}
    >
      <svg class="size-5 -rotate-90 text-muted/30" viewBox="0 0 24 24">
        <circle cx="12" cy="12" r="9" stroke="currentColor" stroke-width="2.5" fill="none" />
        <circle
          cx="12"
          cy="12"
          r="9"
          stroke="currentColor"
          stroke-width="2.5"
          stroke-dasharray="56.5487"
          stroke-dashoffset={56.5487 - (56.5487 * overallProgress) / 100}
          stroke-linecap="round"
          fill="none"
          class="text-primary transition-all duration-500 ease-out"
        />
      </svg>
      {#if overallProgress === 100}
        <Check class="absolute size-2.5 text-primary" />
      {/if}
    </div>

    <!-- Subtle Divider -->
    <div class="h-3.5 w-px bg-border/60 shrink-0" aria-hidden="true"></div>

    <!-- Close / Exit Button -->
    <Button
      variant="ghost"
      size="icon"
      onclick={exitSetup}
      class="size-7 shrink-0 rounded-full text-muted-foreground hover:bg-muted hover:text-foreground transition-colors"
      title={m.onboarding_exit_dashboard()}
      aria-label={m.onboarding_exit_dashboard()}
    >
      <XMark class="size-3.5" />
    </Button>
  </header>

  <!-- Main Viewport -->
  <main
    id="onboarding-viewport"
    class="min-h-0 flex-1 overflow-y-auto overscroll-contain scrollbar-hide pt-20 pb-24 px-4 sm:px-6 md:px-8"
  >
    <div class="mx-auto w-full max-w-3xl py-2">
      {@render children()}
    </div>
  </main>

  <!-- Floating Footer (Centered Bottom, Minimalist, No Titles or Text) -->
  <footer
    class="absolute bottom-4 left-1/2 -translate-x-1/2 z-30 flex items-center gap-1.5 bg-accent/30  backdrop-blur-md px-2 py-1.5 max-w-[calc(100vw-2rem)]"
  >
    <!-- Previous Step Button -->
    <Button
      variant="outline"
      size="icon"
      disabled={currentStepIndex <= 0 && currentStepIndex !== -1}
      onclick={goToPrev}
      class="size-8"
      title="Previous"
      aria-label="Previous step"
    >
      {#if isRtl}
        <ChevronRight class="size-4" />
      {:else}
        <ChevronLeft class="size-4" />
      {/if}
    </Button>

    <!-- Overview / Hub Toggle Button -->
    <Button
      variant={currentStepIndex === -1 ? "secondary" : "ghost"}
      size="icon"
      onclick={goToOverview}
      class="size-8 "
      title="Overview"
      aria-label="Onboarding Overview"
    >
      <TaskList class="size-4" />
    </Button>

    <!-- Next Step / Finish Button -->
    <Button
      variant="default"
      size="icon"
      onclick={goToNext}
      class="size-8"
      title={currentStepIndex === steps.length - 1 ? "Finish" : "Next"}
      aria-label={currentStepIndex === steps.length - 1 ? "Finish onboarding" : "Next step"}
    >
      {#if currentStepIndex === steps.length - 1}
        <Check class="size-4" />
      {:else if isRtl}
        <ChevronLeft class="size-4" />
      {:else}
        <ChevronRight class="size-4" />
      {/if}
    </Button>
  </footer>
</div>
