<script lang="ts">
import { page } from "$app/state";
import { Search } from "@orbis/icons";
import { Badge } from "@orbis/ui/badge";
import { Button } from "@orbis/ui/button";
import { Card, CardContent } from "@orbis/ui/card";
import { Input } from "@orbis/ui/input";
import { Progress } from "@orbis/ui/progress";
import { Select, SelectContent, SelectItem, SelectTrigger } from "@orbis/ui/select";
import type { OnboardingTaskStatus, SetupTaskDto } from "$lib/api/types";
import { effectiveContext, systemStore } from "$lib/stores";
import EmptyState from "$lib/shared/components/empty-state.svelte";
import { getLocale } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";
import { navigateOnboardingRoute } from "$lib/features/onboarding-wizard/navigation";
import { fetchLatestOnboardingState } from "$lib/features/onboarding-wizard/onboarding-queries";
import { filterSetupTasks, type SetupTaskFilter } from "$lib/features/onboarding-wizard/task-filters";
import {
  deriveSteps,
  findNextIncompleteStep,
  findStepForTask,
  isTaskAccessible,
} from "$lib/features/onboarding-wizard/onboarding-steps";

const orgId = $derived(page.params.orgId!);
const isArabic = $derived(getLocale() === "ar");
const onboardingState = $derived(systemStore.onboardingState);
const tasks = $derived(onboardingState?.tasks ?? []);
const categories = $derived(onboardingState?.categories ?? []);
const steps = $derived(deriveSteps(tasks, (permission) => effectiveContext.hasPermission(permission)));
const recommendedTask = $derived(onboardingState?.recommended_next_task);
const canOpenRecommended = $derived(isTaskAccessible(steps, recommendedTask?.id));
const guidedStep = $derived(
  findNextIncompleteStep(steps) ?? steps.find((step) => step.id === "review" && step.isAccessible),
);

let search = $state("");
let category = $state("all");
let filter = $state<SetupTaskFilter>("all");
let isRefreshing = $state(false);
let refreshFailed = $state(false);

const filters = [
  { value: "all", label: m.common_filter_all },
  { value: "required", label: m.ui_onboarding_progress_required_2 },
  { value: "pending", label: m.ui_onboarding_progress_pending },
  { value: "blocked", label: m.onboarding_task_status_blocked },
  { value: "completed", label: m.common_exams_status_completed },
] satisfies { value: SetupTaskFilter; label: () => string }[];
const statusLabels = {
  completed: m.common_exams_status_completed,
  in_progress: m.onboarding_task_status_in_progress,
  blocked: m.onboarding_task_status_blocked,
  not_started: m.onboarding_task_status_not_started,
  optional: m.onboarding_task_status_optional,
  not_applicable: m.onboarding_task_status_not_applicable,
} satisfies Record<OnboardingTaskStatus, () => string>;

const activeFilter = $derived(filters.find((option) => option.value === filter) ?? filters[0]);
const filteredTasks = $derived(filterSetupTasks(tasks, { category, status: filter, search }));

function localized(english: string, arabic?: string | null) {
  return (isArabic ? arabic : english) || english || arabic || "";
}

function categoryLabel(value: string) {
  const match = categories.find((item) => item.category === value);
  return match ? localized(match.label, match.label_ar) : "";
}

function resetFilters() {
  search = "";
  category = "all";
  filter = "all";
}

async function refresh() {
  if (isRefreshing) return;
  isRefreshing = true;
  refreshFailed = false;
  try {
    const requestedOrg = orgId;
    const result = await fetchLatestOnboardingState(requestedOrg);
    if (page.params.orgId !== requestedOrg) return;
    if (result.org_id !== requestedOrg) throw new Error(m.onboarding_state_load_failed());
    systemStore.onboardingState = result;
  } catch {
    refreshFailed = true;
  } finally {
    isRefreshing = false;
  }
}

function openTask(task: SetupTaskDto) {
  const step = findStepForTask(steps, task.id);
  if (step && isTaskAccessible(steps, task.id)) {
    return navigateOnboardingRoute(`/${orgId}/onboarding/${step.route}`);
  }
}

function openGuidedSetup() {
  if (guidedStep) return navigateOnboardingRoute(`/${orgId}/onboarding/${guidedStep.route}`);
}
</script>

<svelte:head>
  <title>{m.ui_onboarding_progress_institution_onboarding_readiness()} – Orbis</title>
</svelte:head>

{#if onboardingState}
  <div class="space-y-6">
    <header class="flex flex-wrap items-start justify-between gap-4 border-b border-border pb-5">
      <div class="space-y-2">
        <h1 class="text-xl font-semibold tracking-tight sm:text-2xl">{m.ui_onboarding_progress_institution_setup_readiness()}</h1>
        <p class="max-w-2xl text-sm text-muted-foreground">{m.shared_ui_onboarding_intro()}</p>
      </div>
      <Button variant="outline" size="sm" onclick={refresh} disabled={isRefreshing} aria-busy={isRefreshing}>
        {isRefreshing ? m.common_control_loading() : m.common_control_refresh()}
      </Button>
    </header>

    {#if refreshFailed}
      <p class="rounded-lg border border-destructive/25 bg-destructive/5 p-3 text-sm text-destructive" role="alert">
        {m.onboarding_state_load_failed()}
      </p>
    {/if}

    <Card>
      <CardContent class="space-y-4 p-4 sm:p-6">
        <div class="flex flex-wrap items-start justify-between gap-3">
          <div class="space-y-1">
            <h2 class="text-base font-semibold">{m.ui_onboarding_progress_overall_readiness()}</h2>
            <p class="text-sm text-muted-foreground">
              {m.shared_ui_onboarding_tasks_completed({ completed: onboardingState.completed_tasks, total: onboardingState.total_tasks })}
            </p>
          </div>
          <Badge variant={onboardingState.is_institution_ready ? "outline" : "secondary"}>
            {onboardingState.is_institution_ready ? m.ui_onboarding_progress_ready_for_operations() : m.ui_onboarding_progress_in_preparation()}
          </Badge>
        </div>
        <div class="flex items-center gap-3">
          <Progress value={onboardingState.progress_percentage} max={100} class="h-2" aria-label={m.ui_onboarding_progress_overall_readiness()} />
          <span class="shrink-0 text-sm font-medium tabular-nums" dir="ltr">{onboardingState.progress_percentage}%</span>
        </div>
        {#if (recommendedTask && canOpenRecommended) || guidedStep}
          <div class="flex flex-wrap items-center justify-between gap-3 border-t border-border pt-4">
            {#if recommendedTask && canOpenRecommended}
              <div class="space-y-1">
                <p class="text-xs text-muted-foreground">{m.ui_onboarding_progress_recommended_next_step()}</p>
                <h3 class="text-sm font-medium">{localized(recommendedTask.title, recommendedTask.title_ar)}</h3>
              </div>
              <Button size="sm" onclick={() => openTask(recommendedTask)}>
                {m.ui_onboarding_progress_configure()}
              </Button>
            {:else}
              <Button size="sm" onclick={openGuidedSetup}>{m.onboarding_hub_launch_wizard()}</Button>
            {/if}
          </div>
        {/if}
      </CardContent>
    </Card>

    <section aria-label={m.ui_onboarding_progress_search_tasks()} class="space-y-3">
      <div class="grid gap-2 sm:grid-cols-[minmax(0,1fr)_minmax(0,12rem)_minmax(0,12rem)]">
        <Input type="search" bind:value={search} placeholder={m.ui_onboarding_progress_search_tasks()} aria-label={m.ui_onboarding_progress_search_tasks()} />
        <Select type="single" value={category} onValueChange={(value) => (category = value ?? "all")}>
          <SelectTrigger class="w-full" aria-label={m.ui_onboarding_progress_all_categories()}>
            {categoryLabel(category) || m.ui_onboarding_progress_all_categories()}
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">{m.ui_onboarding_progress_all_categories()}</SelectItem>
            {#each categories as item (item.category)}
              <SelectItem value={item.category}>{localized(item.label, item.label_ar)}</SelectItem>
            {/each}
          </SelectContent>
        </Select>
        <Select type="single" value={filter} onValueChange={(value) => {
          filter = filters.find((option) => option.value === value)?.value ?? "all";
        }}>
          <SelectTrigger class="w-full" aria-label={m.common_status()}>{activeFilter.label()}</SelectTrigger>
          <SelectContent>
            {#each filters as option (option.value)}
              <SelectItem value={option.value}>{option.label()}</SelectItem>
            {/each}
          </SelectContent>
        </Select>
      </div>

      <Card>
        {#if filteredTasks.length}
          <ul class="divide-y divide-border" aria-live="polite">
            {#each filteredTasks as task (task.id)}
              {@const accessible = isTaskAccessible(steps, task.id)}
              {@const group = categoryLabel(task.category)}
              <li class="grid gap-3 px-4 py-4 sm:grid-cols-[minmax(0,1fr)_auto] sm:items-start sm:px-6">
                <div class="min-w-0 space-y-1.5">
                  {#if group}<p class="text-xs text-muted-foreground">{group}</p>{/if}
                  <h3 class="text-sm font-semibold">{localized(task.title, task.title_ar)}</h3>
                  <p class="text-sm leading-relaxed text-muted-foreground">{localized(task.description, task.description_ar)}</p>
                  {#if task.status === "blocked" && (task.blocking_reason || task.blocking_reason_ar)}
                    <p class="text-xs leading-relaxed text-muted-foreground">
                      <span class="font-medium">{m.ui_onboarding_progress_prerequisite()}</span>
                      {localized(task.blocking_reason ?? "", task.blocking_reason_ar)}
                    </p>
                  {/if}
                </div>
                <div class="flex flex-wrap items-center gap-2 sm:justify-end">
                  {#if task.is_required && task.status !== "completed"}
                    <Badge variant="outline">{m.ui_onboarding_progress_required_3()}</Badge>
                  {/if}
                  <Badge variant="secondary">{statusLabels[task.status]()}</Badge>
                  {#if accessible}
                    <Button variant="outline" size="sm" onclick={() => openTask(task)}>
                      {task.status === "completed" ? m.ui_onboarding_progress_review() : m.ui_onboarding_progress_configure()}
                    </Button>
                  {/if}
                </div>
              </li>
            {/each}
          </ul>
        {:else}
          <EmptyState icon={Search}
            title={m.ui_onboarding_progress_no_tasks_match_your_filters()}
            description={m.ui_onboarding_progress_try_adjusting_your_search_query_or_status_filter()}
            actionLabel={m.academics_reports_reset_filters()}
            onaction={resetFilters}
          />
        {/if}
      </Card>
    </section>
  </div>
{/if}
