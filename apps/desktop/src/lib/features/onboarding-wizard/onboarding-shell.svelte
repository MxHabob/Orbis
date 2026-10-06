<script lang="ts">
import { page } from "$app/state";
import { goto } from "$app/navigation";
import { onMount } from "svelte";
import type { Snippet } from "svelte";
import * as m from "$lib/paraglide/messages";
import { authStore, systemStore } from "$lib/stores";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import { Progress } from "@orbis/ui/progress";
import {
  deriveSteps,
  findNextIncompleteStep,
  type OnboardingStep,
} from "./onboarding-steps";

let { orgId, children }: { orgId: string; children: Snippet } = $props();

const currentPath = $derived(page.url.pathname);

// Load onboarding DAG state from backend on mount
onMount(async () => {
  await systemStore.loadOnboardingState(orgId);
});

const onboardingState = $derived(systemStore.onboardingState);
const tasks = $derived(systemStore.onboardingTasks);
const steps = $derived(deriveSteps(tasks));
const overallProgress = $derived(systemStore.onboardingProgressPercentage);
const isReady = $derived(systemStore.isInstitutionReady);

// Current step index and object
const currentStepIndex = $derived.by(() => {
  const matchingIndex = steps.findIndex((s) =>
    currentPath.endsWith(`/onboarding/${s.route}`) ||
    (s.route === "review" && currentPath.endsWith("/onboarding/review"))
  );
  if (matchingIndex >= 0) return matchingIndex;
  // If at root /onboarding, we treat as overview (or first step)
  return -1;
});

const currentStep = $derived(
  currentStepIndex >= 0 ? steps[currentStepIndex] : null
);

async function navigateToStep(step: OnboardingStep) {
  if (!step.isAccessible) return;
  await goto(`/${orgId}/onboarding/${step.route}`);
}

async function exitSetup() {
  await goto(`/${orgId}`);
}
</script>

<div class="flex min-h-screen flex-col bg-background text-foreground font-sans selection:bg-primary/20">
  <!-- Top Navigation Bar -->
  <header class="sticky top-0 z-40 flex h-14 w-full items-center justify-between border-b border-border/60 bg-background/90 px-4 backdrop-blur-md sm:px-6">
    <!-- Left: Brand & Institution Info -->
    <div class="flex items-center gap-3">
      <div class="flex size-8 items-center justify-center rounded-lg bg-primary text-primary-foreground font-bold shadow-xs">
        O
      </div>
      <div class="flex flex-col">
        <div class="flex items-center gap-2">
          <span class="text-sm font-semibold tracking-tight text-foreground">
            {onboardingState?.organization?.name || authStore.currentOrg?.name || "Orbis"}
          </span>
          <span class="rounded bg-primary/10 px-1.5 py-0.5 text-[10px] font-semibold text-primary uppercase">
            {m.onboarding_wizard_title()}
          </span>
        </div>
        {#if onboardingState?.institution_type}
          <span class="text-[11px] text-muted-foreground capitalize">
            {onboardingState.institution_type} — {m.onboarding_workspace_badge()}
          </span>
        {/if}
      </div>
    </div>

    <!-- Center: Global Progress Widget -->
    <div class="hidden items-center gap-3 md:flex">
      <div class="w-36">
        <Progress value={overallProgress} class="h-2 bg-muted" />
      </div>
      <span class="text-xs font-medium text-muted-foreground">
        {m.onboarding_complete_percent({ percent: overallProgress })}
      </span>
      {#if isReady}
        <Badge variant="outline" class="border-emerald-500/30 bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 text-xs">
          {m.onboarding_ready_badge()}
        </Badge>
      {/if}
    </div>

    <!-- Right: Exit / Return to Dashboard -->
    <div class="flex items-center gap-2">
      <Button
        variant="ghost"
        size="sm"
        onclick={exitSetup}
        class="text-xs text-muted-foreground hover:text-foreground"
      >
        {m.onboarding_exit_dashboard()}
      </Button>
    </div>
  </header>

  <!-- Main Grid Layout: Sidebar + Step Content -->
  <div class="mx-auto flex w-full max-w-7xl flex-1 flex-col md:flex-row">
    <!-- Sidebar / Step Rail -->
    <aside class="w-full border-b border-border/40 bg-muted/20 p-4 md:w-72 md:border-b-0 md:border-e md:p-6 lg:w-80">
      <div class="sticky top-20 space-y-6">
        <div>
          <h2 class="text-xs font-semibold tracking-wider text-muted-foreground uppercase">
            {m.onboarding_stages_title()}
          </h2>
          <p class="mt-1 text-xs text-muted-foreground">
            {m.onboarding_stages_desc()}
          </p>
        </div>

        <!-- Step List -->
        <nav class="space-y-1.5" aria-label="Onboarding Steps">
          {#each steps as step, index (step.id)}
            {@const isActive = currentStepIndex === index}
            {@const isClickable = step.isAccessible}
            <button
              type="button"
              onclick={() => navigateToStep(step)}
              disabled={!isClickable}
              class="group flex w-full items-center gap-3 rounded-lg px-3 py-2.5 text-start transition-all {isActive
                ? 'bg-primary/10 text-primary font-medium shadow-2xs'
                : isClickable
                ? 'text-muted-foreground hover:bg-muted/60 hover:text-foreground'
                : 'cursor-not-allowed text-muted-foreground/40'}"
            >
              <!-- Status Icon Badge -->
              <div class="flex size-7 shrink-0 items-center justify-center rounded-full text-xs font-semibold transition-colors {isActive
                ? 'bg-primary text-primary-foreground shadow-xs'
                : step.status === 'completed'
                ? 'bg-emerald-500 text-white'
                : step.status === 'blocked'
                ? 'bg-muted text-muted-foreground/50'
                : 'bg-muted text-muted-foreground group-hover:bg-muted/80'}">
                {#if step.status === "completed"}
                  <span class="text-xs font-bold">✓</span>
                {:else if step.status === "blocked"}
                  <span class="text-[10px]">🔒</span>
                {:else}
                  <span>{step.order}</span>
                {/if}
              </div>

              <!-- Step Labels -->
              <div class="min-w-0 flex-1">
                <div class="flex items-center justify-between gap-1">
                  <span class="truncate text-xs font-medium sm:text-sm">
                    {step.getLabel()}
                  </span>
                  {#if step.isRequired && step.status !== 'completed'}
                    <span class="text-[10px] text-amber-500 font-medium">*</span>
                  {/if}
                </div>
                <p class="truncate text-[11px] text-muted-foreground/70 {isActive ? 'text-primary/70' : ''}">
                  {step.getDescription()}
                </p>
              </div>

              <!-- Progress / Status Indicator -->
              {#if step.status === "completed"}
                <span class="size-1.5 rounded-full bg-emerald-500"></span>
              {:else if step.progress > 0}
                <span class="text-[10px] font-mono text-muted-foreground">{step.progress}%</span>
              {/if}
            </button>
          {/each}
        </nav>

        <!-- Sidebar Summary Box -->
        <div class="rounded-lg border border-border/50 bg-card p-3 shadow-2xs">
          <div class="flex items-center justify-between text-xs text-muted-foreground">
            <span>{m.onboarding_tasks_completed_count()}</span>
            <span class="font-mono font-medium text-foreground">
              {systemStore.completedOnboardingTasks.length} / {tasks.length}
            </span>
          </div>
          <div class="mt-2">
            <Progress value={overallProgress} class="h-1.5 bg-muted" />
          </div>
        </div>
      </div>
    </aside>

    <!-- Main Content Workspace -->
    <main class="flex-1 overflow-y-auto px-4 py-8 sm:px-8 lg:px-12">
      <div class="mx-auto max-w-3xl">
        {@render children()}
      </div>
    </main>
  </div>
</div>
