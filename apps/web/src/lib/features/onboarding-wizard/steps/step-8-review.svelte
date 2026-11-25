<script lang="ts">
import { LoadingIndicator, toast } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { effectiveContext, systemStore } from "$lib/stores";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import { Progress } from "@orbis/ui/progress";
import { Card, CardContent } from "@orbis/ui/card";
import OnboardingSection from "../onboarding-section.svelte";
import {
  deriveSteps,
  findStepForTask,
  isTaskAccessible,
} from "../onboarding-steps";
import { navigateOnboardingRoute } from "../navigation";
import { refreshOnboardingState } from "../refresh-onboarding-state";
import { l } from "$lib/core/i18n";
import { extractErrorMessage, type SetupTaskDto } from "$lib/api";

let { orgId }: { orgId: string } = $props();

let isRefreshing = $state(false);
let isLaunching = $state(false);
async function refreshDAG(): Promise<boolean> {
  isRefreshing = true;
  try {
    await refreshOnboardingState(orgId, m.onboarding_step8_err_readiness());
    return true;
  } catch (error) {
    toast.error(extractErrorMessage(error));
    return false;
  } finally {
    isRefreshing = false;
  }
}

const tasks = $derived(systemStore.onboardingTasks);
const steps = $derived(
  deriveSteps(tasks, (permission) =>
    effectiveContext.hasPermission(permission),
  ),
);
const overallProgress = $derived(systemStore.onboardingProgressPercentage);
const isReady = $derived(systemStore.isInstitutionReady);
const warnings = $derived(systemStore.onboardingState?.warnings ?? []);
const requiredTaskCount = $derived(
  systemStore.onboardingState?.required_tasks ?? 0,
);
const completedRequiredTaskCount = $derived(
  systemStore.onboardingState?.required_completed ?? 0,
);

function getBlockingReason(task: SetupTaskDto): string {
  let reason = l(task.blocking_reason_ar, task.blocking_reason) ?? "";
  for (const dependencyId of task.dependencies) {
    const dependency = tasks.find((item) => item.id === dependencyId);
    if (dependency && reason.includes(dependencyId)) {
      reason = reason.replaceAll(
        dependencyId,
        l(dependency.title_ar, dependency.title),
      );
    }
  }
  return reason;
}

async function handleCompleteAndLaunch() {
  if (isLaunching || isRefreshing) return;
  isLaunching = true;

  try {
    if (!(await refreshDAG())) return;
    if (!systemStore.isInstitutionReady) {
      toast.error(m.onboarding_step8_launch_prerequisite());
      return;
    }

    await navigateOnboardingRoute(`/${orgId}`);
  } catch (error) {
    toast.error(extractErrorMessage(error));
  } finally {
    isLaunching = false;
  }
}
</script>

<div class="space-y-6">
  {#if isRefreshing && !systemStore.onboardingState}
    <Card>
      <CardContent class="min-h-56 items-center justify-center gap-3 p-8 text-center">
      <LoadingIndicator size={24} class="size-6 text-primary" />
      <p class="text-sm text-muted-foreground" aria-live="polite">
        {m.common_control_loading()}
      </p>
      </CardContent>
    </Card>
  {:else if !systemStore.onboardingState}
    <Card>
      <CardContent class="min-h-56 items-center justify-center gap-4 p-8 text-center">
      <p class="max-w-md text-sm text-muted-foreground">
        {m.onboarding_step8_err_readiness()}
      </p>
      <Button variant="outline" size="sm" onclick={refreshDAG}>
        {m.onboarding_step8_recheck_btn()}
      </Button>
      </CardContent>
    </Card>
  {:else}
    <!-- Readiness Scorecard -->
    <Card>
      <CardContent class="space-y-4 p-6">
        <div class="flex flex-col justify-between gap-4 sm:flex-row sm:items-center">
          <div>
            <span class="text-xs font-semibold uppercase tracking-wider text-muted-foreground">
              {m.onboarding_step8_score_title()}
            </span>
            <div class="mt-1 flex items-center gap-3 text-3xl font-extrabold text-foreground">
              <span>{overallProgress}%</span>
              {#if isReady}
                <Badge variant="outline" class="border-emerald-500/30 bg-emerald-500/10 text-xs py-1 px-2.5 text-emerald-600 dark:text-emerald-400">
                  {m.onboarding_step8_ready_badge()}
                </Badge>
              {:else}
                <Badge variant="outline" class="border-amber-500/30 bg-amber-500/10 text-xs py-1 px-2.5 text-amber-600 dark:text-amber-400">
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
            class="self-start text-xs sm:self-auto"
          >
            {isRefreshing ? m.onboarding_step8_rechecking_btn() : m.onboarding_step8_recheck_btn()}
          </Button>
        </div>

        <Progress value={overallProgress} class="h-2.5 bg-muted" />
      </CardContent>
    </Card>

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
      <ul class="divide-y divide-border">
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
          <li class="flex flex-wrap items-center justify-between gap-3 py-3">
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
                  variant="outline"
                  class={task.status === "blocked"
                    ? "border-amber-500/30 bg-amber-500/10 text-[10px] text-amber-700 dark:text-amber-300"
                    : "text-[10px]"}
                >
                  {statusLabel}
                </Badge>
                {#if task.status === "in_progress"}
                  <span class="text-xs tabular-nums text-muted-foreground">{task.progress}%</span>
                {/if}
              </div>
              {#if task.status === "blocked" && (task.blocking_reason_ar || task.blocking_reason)}
                <p class="text-xs leading-relaxed text-muted-foreground">
                  {getBlockingReason(task)}
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
          </li>
        {/each}
      </ul>
    </OnboardingSection>

    <div class="flex flex-wrap items-center justify-between gap-3 border-t border-border pt-4">
      {#if !isReady}
        <p class="text-sm text-muted-foreground">{m.onboarding_step8_launch_prerequisite()}</p>
      {/if}
      <Button
        variant="default"
        size="lg"
        disabled={!isReady || isLaunching || isRefreshing}
        onclick={handleCompleteAndLaunch}
        class="ms-auto"
      >
        {isLaunching ? m.onboarding_step8_launching_btn() : m.onboarding_step8_launch_btn()}
      </Button>
    </div>
  {/if}
</div>
