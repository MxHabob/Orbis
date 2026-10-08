<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { effectiveContext, systemStore } from "$lib/stores";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import { Progress } from "@orbis/ui/progress";
import OnboardingStepHeader from "../onboarding-step-header.svelte";
import OnboardingSection from "../onboarding-section.svelte";
import {
  deriveSteps,
  findStepForTask,
  isTaskAccessible,
} from "../onboarding-steps";
import { navigateOnboardingRoute } from "../navigation";
import { refreshOnboardingState } from "../refresh-onboarding-state";
import { l } from "$lib/core/i18n";

let { orgId }: { orgId: string } = $props();

let isRefreshing = $state(false);
let isLaunching = $state(false);
let launchSuccess = $state(false);
let errorMessage = $state<string | null>(null);

async function refreshDAG() {
  isRefreshing = true;
  errorMessage = null;
  try {
    await refreshOnboardingState(orgId, m.onboarding_step8_err_readiness());
  } catch (err: any) {
    console.error("Failed to refresh onboarding DAG state:", err);
    errorMessage = err?.message || m.onboarding_step8_err_readiness();
  } finally {
    isRefreshing = false;
  }
}

const tasks = $derived(systemStore.onboardingTasks);
const steps = $derived(
  deriveSteps(tasks, (permission) => effectiveContext.hasPermission(permission)),
);
const overallProgress = $derived(systemStore.onboardingProgressPercentage);
const isReady = $derived(systemStore.isInstitutionReady);
const warnings = $derived(systemStore.onboardingState?.warnings ?? []);
const requiredTaskCount = $derived(systemStore.onboardingState?.required_tasks ?? 0);
const completedRequiredTaskCount = $derived(systemStore.onboardingState?.required_completed ?? 0);

async function handleCompleteAndLaunch() {
  if (isLaunching || isRefreshing) return;
  isLaunching = true;
  errorMessage = null;

  try {
    await refreshDAG();
    if (errorMessage) return;
    if (!systemStore.isInstitutionReady) {
      errorMessage = m.onboarding_step8_err_readiness();
      return;
    }

    launchSuccess = true;

    setTimeout(async () => {
      await navigateOnboardingRoute(`/${orgId}`);
    }, 900);
  } catch (err: any) {
    console.error("Failed to complete onboarding:", err);
    errorMessage = err?.message || m.onboarding_step8_err_launch();
  } finally {
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

  {#if isRefreshing && !systemStore.onboardingState}
    <div class="flex min-h-56 flex-col items-center justify-center gap-3 rounded-2xl border border-border/70 bg-card p-8 text-center">
      <LoadingIndicator size={24} class="size-6 text-primary" />
      <p class="text-sm text-muted-foreground" aria-live="polite">
        {m.common_control_loading()}
      </p>
    </div>
  {:else if !systemStore.onboardingState}
    <div class="flex min-h-56 flex-col items-center justify-center gap-4 rounded-2xl border border-border/70 bg-card p-8 text-center">
      <p class="max-w-md text-sm text-muted-foreground">
        {m.onboarding_step8_err_readiness()}
      </p>
      <Button variant="outline" size="sm" onclick={refreshDAG}>
        {m.onboarding_step8_recheck_btn()}
      </Button>
    </div>
  {:else if launchSuccess}
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
                {m.onboarding_step8_required_tasks_count({
                  completed: completedRequiredTaskCount,
                  total: requiredTaskCount,
                })}
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

    {#if warnings.length > 0}
      <div class="space-y-2 rounded-xl border border-amber-500/30 bg-amber-500/10 p-4 text-sm text-amber-900 dark:text-amber-100" role="status">
        <h2 class="font-semibold">{m.ui_onboarding_review_initialization_warnings()}</h2>
        <ul class="list-inside list-disc space-y-1">
          {#each warnings as warning (warning)}
            <li>{warning}</li>
          {/each}
        </ul>
      </div>
    {/if}

    <OnboardingSection
      title={m.onboarding_step8_checklist_title()}
      description={m.onboarding_step8_checklist_desc()}
    >
      <div class="divide-y divide-border/40 rounded-lg border border-border/50 bg-card overflow-hidden">
        {#each tasks as task (task.id)}
          {@const step = findStepForTask(steps, task.id)}
          {@const statusLabel = task.status === "completed"
            ? m.onboarding_step8_status_ready()
            : task.status === "in_progress"
              ? m.onboarding_task_status_in_progress()
              : task.status === "blocked"
                ? m.onboarding_task_status_blocked()
                : task.status === "optional"
                  ? m.onboarding_task_status_optional()
                  : task.status === "not_applicable"
                    ? m.onboarding_task_status_not_applicable()
                    : m.onboarding_task_status_not_started()}
          <div class="flex flex-wrap items-center justify-between gap-3 p-4 transition-colors hover:bg-muted/30">
            <div class="min-w-0 flex-1 space-y-1">
              <div class="flex flex-wrap items-center gap-2">
                <span class="text-sm font-medium text-foreground">
                  {l(task.title_ar, task.title)}
                </span>
                {#if task.is_required}
                  <Badge variant="secondary" class="text-[10px]">
                    {m.onboarding_task_required_label()}
                  </Badge>
                {/if}
                <Badge
                  variant={task.status === "blocked" ? "destructive" : "outline"}
                  class="text-[10px]"
                >
                  {statusLabel}
                </Badge>
                {#if task.status === "in_progress"}
                  <span class="text-xs tabular-nums text-muted-foreground">{task.progress}%</span>
                {/if}
              </div>
              {#if task.status === "blocked" && (task.blocking_reason_ar || task.blocking_reason)}
                <p class="text-xs leading-relaxed text-muted-foreground">
                  {l(task.blocking_reason_ar, task.blocking_reason)}
                </p>
              {/if}
            </div>
            {#if step && step.id !== "review" && task.status !== "completed" && task.status !== "not_applicable"}
              <Button
                variant="ghost"
                size="sm"
                disabled={!isTaskAccessible(steps, task.id)}
                onclick={() => navigateOnboardingRoute(`/${orgId}/onboarding/${step.route}`)}
                class="shrink-0 text-xs font-medium text-primary hover:bg-primary/10 hover:text-primary"
              >
                {m.onboarding_step8_configure_btn()}
              </Button>
            {/if}
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
