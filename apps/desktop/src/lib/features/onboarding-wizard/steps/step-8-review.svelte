<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import { onMount } from "svelte";
import * as m from "$lib/paraglide/messages";
import { systemStore } from "$lib/stores";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import { Progress } from "@orbis/ui/progress";
import OnboardingStepHeader from "../onboarding-step-header.svelte";
import OnboardingSection from "../onboarding-section.svelte";
import { deriveSteps } from "../onboarding-steps";
import { navigateOnboardingRoute } from "../navigation";

let { orgId }: { orgId: string } = $props();

let isRefreshing = $state(false);
let isLaunching = $state(false);
let launchSuccess = $state(false);
let errorMessage = $state<string | null>(null);

onMount(async () => {
  await refreshDAG();
});

async function refreshDAG() {
  isRefreshing = true;
  try {
    await systemStore.loadOnboardingState(orgId);
    await systemStore.loadReadiness();
  } catch (err: any) {
    console.error("Failed to refresh onboarding DAG state:", err);
    errorMessage = err?.message || m.onboarding_step8_err_readiness();
  } finally {
    isRefreshing = false;
  }
}

const tasks = $derived(systemStore.onboardingTasks);
const steps = $derived(deriveSteps(tasks));
const overallProgress = $derived(systemStore.onboardingProgressPercentage);
const isReady = $derived(systemStore.isInstitutionReady);
const requiredTasks = $derived(tasks.filter((t) => t.is_required));
const completedRequiredTasks = $derived(requiredTasks.filter((t) => t.status === "completed"));

async function handleCompleteAndLaunch() {
  isLaunching = true;
  errorMessage = null;

  try {
    await refreshDAG();
    launchSuccess = true;

    // Wait a brief moment to show celebration state, then navigate to dashboard
    setTimeout(async () => {
      await navigateOnboardingRoute(`/${orgId}`);
    }, 1200);
  } catch (err: any) {
    console.error("Failed to complete onboarding:", err);
    errorMessage = err?.message || m.onboarding_step8_err_launch();
    isLaunching = false;
  }
}
</script>

<div class="space-y-6">
  <OnboardingStepHeader
    stepNumber={10}
    totalSteps={10}
    title={m.onboarding_step8_title()}
    description={m.onboarding_step8_subtitle()}
    isRequired={true}
    status={isReady ? "completed" : "in_progress"}
  />

  {#if errorMessage}
    <div class="rounded-lg border border-destructive/30 bg-destructive/10 p-4 text-sm text-destructive font-medium flex items-center justify-between">
      <span>{errorMessage}</span>
      <Button variant="ghost" size="sm" onclick={() => (errorMessage = null)}>✕</Button>
    </div>
  {/if}

  {#if launchSuccess}
    <div class="rounded-xl border border-emerald-500/40 bg-emerald-500/10 p-8 text-center space-y-4 animate-in fade-in zoom-in duration-300">
      <div class="mx-auto flex size-16 items-center justify-center rounded-full bg-emerald-500 text-white text-3xl font-bold shadow-lg shadow-emerald-500/30">
        ✓
      </div>
      <h2 class="text-2xl font-bold text-foreground">
        {m.onboarding_step8_congrats_title()}
      </h2>
      <p class="text-sm text-muted-foreground max-w-md mx-auto">
        {m.onboarding_step8_congrats_desc()}
      </p>
      <div class="flex justify-center pt-2">
        <LoadingIndicator size={24} class="size-6 text-primary" />
      </div>
    </div>
  {:else}
    <!-- Readiness Scorecard -->
    <div class="rounded-xl border border-border/60 bg-card p-6 shadow-xs space-y-4">
      <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
        <div>
          <span class="text-xs font-semibold uppercase tracking-wider text-muted-foreground">
            {m.onboarding_step8_score_title()}
          </span>
          <div class="text-3xl font-extrabold text-foreground mt-1 flex items-center gap-3">
            <span>{overallProgress}%</span>
            {#if isReady}
              <Badge variant="outline" class="border-emerald-500/30 bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 text-xs py-1 px-2.5">
                {m.onboarding_step8_ready_badge()}
              </Badge>
            {:else}
              <Badge variant="outline" class="border-amber-500/30 bg-amber-500/10 text-amber-600 dark:text-amber-400 text-xs py-1 px-2.5">
                {m.onboarding_step8_required_tasks_count({ completed: completedRequiredTasks.length, total: requiredTasks.length })}
              </Badge>
            {/if}
          </div>
        </div>

        <Button
          variant="outline"
          size="sm"
          disabled={isRefreshing}
          onclick={refreshDAG}
          class="text-xs self-start sm:self-auto"
        >
          {isRefreshing ? m.onboarding_step8_rechecking_btn() : m.onboarding_step8_recheck_btn()}
        </Button>
      </div>

      <Progress value={overallProgress} class="h-2.5 bg-muted" />
    </div>

    <!-- Stage-by-Stage Audit Matrix -->
    <OnboardingSection
      title={m.onboarding_step8_checklist_title()}
      description={m.onboarding_step8_checklist_desc()}
    >
      <div class="divide-y divide-border/40 rounded-lg border border-border/50 bg-card overflow-hidden">
        {#each steps.filter((s) => s.id !== "review") as step (step.id)}
          <div class="flex items-center justify-between p-4 text-xs transition-colors hover:bg-muted/30">
            <div class="flex items-center gap-3">
              <!-- Step Status Badge -->
              <div class="flex size-7 shrink-0 items-center justify-center rounded-full text-xs font-semibold {step.status === 'completed'
                ? 'bg-emerald-500 text-white'
                : step.status === 'in_progress'
                ? 'bg-blue-500 text-white'
                : 'bg-muted text-muted-foreground'}">
                {#if step.status === "completed"}
                  ✓
                {:else}
                  {step.order}
                {/if}
              </div>

              <div>
                <div class="font-semibold text-foreground flex items-center gap-2">
                  <span>{step.label}</span>
                  {#if step.isRequired}
                    <span class="text-[10px] text-amber-500 font-medium">*</span>
                  {/if}
                </div>
                <div class="text-[11px] text-muted-foreground">
                  {step.description}
                </div>
              </div>
            </div>

            <div class="flex items-center gap-3">
              {#if step.status === "completed"}
                <Badge variant="outline" class="border-emerald-500/30 bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 text-[10px]">
                  {m.onboarding_step8_status_ready()}
                </Badge>
              {:else if step.status === "in_progress"}
                <Badge variant="outline" class="border-blue-500/30 bg-blue-500/10 text-blue-600 dark:text-blue-400 text-[10px]">
                  {step.progress}%
                </Badge>
              {:else}
                <Badge variant="secondary" class="text-[10px]">
                  {m.onboarding_step8_status_pending()}
                </Badge>
              {/if}

              <Button
                variant="ghost"
                size="sm"
                onclick={() => navigateOnboardingRoute(`/${orgId}/onboarding/${step.route}`)}
                class="text-xs font-medium text-primary hover:text-primary hover:bg-primary/10"
              >
                {m.onboarding_step8_configure_btn()}
              </Button>
            </div>
          </div>
        {/each}
      </div>
    </OnboardingSection>

    <!-- Inline Action & Final Launch Bar -->
    <div class="mt-8 flex items-center justify-end gap-3 pb-8 pt-4 border-t border-border/40">
      <Button
        variant="default"
        size="lg"
        disabled={isLaunching}
        onclick={handleCompleteAndLaunch}
        class="text-sm font-bold shadow-md bg-emerald-600 hover:bg-emerald-700 text-white px-6"
      >
        {isLaunching ? m.onboarding_step8_launching_btn() : m.onboarding_step8_launch_btn()}
      </Button>
    </div>
  {/if}
</div>
