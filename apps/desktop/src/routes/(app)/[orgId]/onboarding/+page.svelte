<!--
  @component OnboardingHub
  Authoritative, minimalist DAG setup orchestrator for post-bootstrap institution configuration.
  Built with clean typography, UI design tokens, and uncluttered layout.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { goto } from "$app/navigation";
import { resolve } from "$app/paths";
import { localizeHref, getLocale } from "$lib/paraglide/runtime";
import { api } from "$lib/api";
import { fetchLatestOnboardingState } from "$lib/features/onboarding-wizard/onboarding-queries";
import { effectiveContext, systemStore } from "$lib/stores";
import type {
  OnboardingTaskCategory,
  OnboardingTaskStatus,
  ProvisionDefaultsReqDto,
  SetupTaskDto,
} from "$lib/api/types";
import {
  deriveSteps,
  findNextIncompleteStep,
  findStepForTask,
  isTaskAccessible,
} from "$lib/features/onboarding-wizard/onboarding-steps";

// @orbis/ui components
import { Button } from "@orbis/ui/button";
import { Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter } from "@orbis/ui/card";
import { Badge } from "@orbis/ui/badge";
import { Progress } from "@orbis/ui/progress";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Checkbox } from "@orbis/ui/checkbox";
import { BilingualNameFields } from "@orbis/ui/bilingual-name-fields";
import { suggestLocalTranslation } from "$lib/shared/translation/local-engine";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from "@orbis/ui/dialog";

const orgId = $derived(page.params.orgId);
const isArabic = $derived(getLocale().startsWith("ar"));

// Filter States
let searchQuery = $state("");
let selectedCategory = $state<string>("all");
let selectedStatus = $state<string>("all");
let isProvisionModalOpen = $state(false);
let isProvisioning = $state(false);
let provisioningError = $state<string | null>(null);
let isLoadingOnboarding = $state(false);
let onboardingLoadFailed = $state(false);

// Provision Form Defaults
let provYearName = $state("");
let provYearNameAr = $state("");
let provStagesAndGrades = $state(false);
let provSubjects = $state(false);
let provAssessmentScheme = $state(false);
let provTuitionFee = $state(false);

async function loadState() {
  isLoadingOnboarding = true;
  onboardingLoadFailed = false;
  try {
    systemStore.onboardingState = await fetchLatestOnboardingState(orgId);
  } catch (error) {
    console.error("Failed to load the onboarding overview state:", error);
    systemStore.onboardingState = null;
    onboardingLoadFailed = true;
  } finally {
    isLoadingOnboarding = false;
  }
}

const state = $derived(systemStore.onboardingState);
const tasks = $derived(systemStore.onboardingTasks);
const steps = $derived(
  deriveSteps(
    tasks,
    (permission) => effectiveContext.hasPermission(permission),
  ),
);
const categories = $derived(state?.categories ?? []);

// Category label translations
function getCategoryLabel(cat: string) {
  const c = categories.find((item) => item.category === cat);
  if (c) return isArabic ? c.label_ar : c.label;
  switch (cat) {
    case "organization": return m.ui_onboarding_progress_organization();
    case "calendar": return m.ui_onboarding_progress_calendar();
    case "academics": return m.ui_onboarding_progress_academics();
    case "assessment": return m.ui_onboarding_progress_assessment();
    case "finance": return m.ui_onboarding_progress_finance();
    case "students": return m.ui_onboarding_progress_students();
    case "templates": return m.ui_onboarding_progress_documents();
    case "integrations": return m.ui_onboarding_progress_integrations();
    default: return cat;
  }
}

const selectedCategoryLabel = $derived.by(() => {
  if (selectedCategory === "all") return m.ui_onboarding_progress_all_categories();
  const c = categories.find((item) => item.category === selectedCategory);
  if (c) return isArabic ? c.label_ar : c.label;
  return getCategoryLabel(selectedCategory);
});

// Status Display Helper
function getStatusInfo(status: OnboardingTaskStatus) {
  switch (status) {
    case "completed":
      return {
        label: m.common_exams_status_completed(),
        variant: "outline" as const,
        classes: "border-emerald-500/30 text-emerald-600 dark:text-emerald-400 bg-emerald-500/5",
      };
    case "in_progress":
      return {
        label: m.ui_onboarding_progress_in_progress(),
        variant: "secondary" as const,
        classes: "text-amber-600 dark:text-amber-400 bg-amber-500/10",
      };
    case "blocked":
      return {
        label: m.ui_onboarding_progress_blocked(),
        variant: "secondary" as const,
        classes: "text-muted-foreground bg-muted/60",
      };
    case "optional":
      return {
        label: m.ui_onboarding_progress_optional(),
        variant: "outline" as const,
        classes: "text-muted-foreground",
      };
    case "not_applicable":
      return {
        label: m.onboarding_task_status_not_applicable(),
        variant: "outline" as const,
        classes: "text-muted-foreground",
      };
    default:
      return {
        label: m.ui_onboarding_progress_not_started(),
        variant: "secondary" as const,
        classes: "text-muted-foreground",
      };
  }
}

// Filtered Tasks
const filteredTasks = $derived.by(() => {
  return tasks.filter((t) => {
    if (searchQuery.trim()) {
      const q = searchQuery.toLowerCase();
      const matchEn = t.title.toLowerCase().includes(q) || t.description.toLowerCase().includes(q);
      const matchAr = t.title_ar.includes(q) || t.description_ar.includes(q);
      if (!matchEn && !matchAr) return false;
    }

    if (selectedCategory !== "all" && t.category !== selectedCategory) {
      return false;
    }

    return true;
  });
});

async function handleNavigate(task: SetupTaskDto) {
  const step = findStepForTask(steps, task.id);
  if (!isTaskAccessible(steps, task.id)) return;
  await goto(resolve(localizeHref(`/${orgId}/onboarding/${step.route}`)));
}

async function startGuidedSetup() {
  const target = findNextIncompleteStep(steps) ??
    steps.find((step) => step.id === "review" && step.isAccessible);
  if (!target) return;
  await goto(resolve(localizeHref(`/${orgId}/onboarding/${target.route}`)));
}

async function handleProvisionDefaults() {
  isProvisioning = true;
  provisioningError = null;
  try {
    const request: ProvisionDefaultsReqDto = {
      academic_year_name: provYearName || undefined,
      academic_year_name_ar: provYearNameAr || undefined,
      provision_academic_year: Boolean(provYearName.trim() || provYearNameAr.trim()),
      provision_stages_and_grades: provStagesAndGrades,
      provision_subjects: provSubjects,
      provision_assessment_scheme: provAssessmentScheme,
      provision_tuition_fee_template: provTuitionFee,
    };
    await systemStore.provisionBlueprintDefaults(
      request,
      orgId,
    );
    isProvisionModalOpen = false;
  } catch (e: any) {
    provisioningError = e?.message || (m.ui_onboarding_progress_failed_to_provision_defaults());
  } finally {
    isProvisioning = false;
  }
}
</script>

<svelte:head>
  <title>{m.ui_onboarding_progress_institution_onboarding_readiness()} — Orbis</title>
</svelte:head>

{#if isLoadingOnboarding}
  <div class="flex min-h-64 items-center justify-center px-4 py-10">
    <p class="text-sm text-muted-foreground" role="status" aria-live="polite">
      {m.onboarding_state_loading()}
    </p>
  </div>
{:else if onboardingLoadFailed}
  <div class="mx-auto flex min-h-64 w-full max-w-xl flex-col items-center justify-center gap-4 px-4 py-10 text-center">
    <div class="w-full space-y-4 rounded-2xl border border-destructive/30 bg-card p-6 text-start">
      <p class="text-sm font-semibold text-destructive" role="alert">
        {m.onboarding_state_load_failed()}
      </p>
      <Button variant="outline" size="sm" onclick={loadState}>
        {m.onboarding_retry()}
      </Button>
    </div>
  </div>
{:else}
<div class="max-w-6xl mx-auto space-y-6 p-4 sm:p-6 pb-20">
  <!-- Minimal Page Header -->
  <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 border-b border-border/40 pb-5">
    <div>
      <h1 class="text-xl font-semibold tracking-tight text-foreground">
        {m.ui_onboarding_progress_institution_setup_readiness()}
      </h1>
      <p class="text-xs text-muted-foreground mt-1">
        {m.shared_ui_onboarding_intro()}
      </p>
    </div>

    <div class="flex items-center gap-2">
      <Button
        variant="default"
        size="sm"
        onclick={startGuidedSetup}
        class="h-9 text-xs font-semibold bg-primary text-primary-foreground shadow-xs"
      >
        <span>{m.onboarding_hub_launch_wizard()}</span>
      </Button>

      <Button
        variant="outline"
        size="sm"
        onclick={() => (isProvisionModalOpen = true)}
        class="h-9 text-xs"
      >
        <span>{m.ui_onboarding_progress_auto_scaffold_blueprint()}</span>
      </Button>

      <Button
        variant="outline"
        size="sm"
        onclick={loadState}
        disabled={isLoadingOnboarding || systemStore.isLoading}
        class="h-9 text-xs"
      >
        <span>{m.common_control_refresh()}</span>
      </Button>
    </div>
  </div>

  <!-- Overall Progress Card -->
  <Card class="border-border/60 shadow-xs">
    <CardContent class="p-5 space-y-4">
      <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3">
        <div class="space-y-1">
          <div class="flex items-center gap-2">
            <span class="text-sm font-semibold text-foreground">
              {m.ui_onboarding_progress_overall_readiness()}
            </span>
            <span class="text-sm font-mono font-medium text-primary">
              {state?.progress_percentage ?? 0}%
            </span>
          </div>
          <p class="text-xs text-muted-foreground">
            {m.shared_ui_onboarding_tasks_completed({
              completed: state?.completed_tasks ?? 0,
              total: state?.total_tasks ?? 0,
            })}
          </p>
        </div>

        <div class="flex items-center flex-wrap gap-2 text-xs">
          <Badge variant="outline" class="font-normal">
            {m.ui_onboarding_progress_required()}
            <span class="font-medium ms-1 font-mono">{state?.required_completed ?? 0}/{state?.required_tasks ?? 0}</span>
          </Badge>

          {#if (state?.blocked_tasks ?? 0) > 0}
            <Badge variant="secondary" class="font-normal text-muted-foreground">
              {m.ui_onboarding_progress_blocked_2()}
              <span class="font-medium ms-1 font-mono">{state?.blocked_tasks}</span>
            </Badge>
          {/if}

          {#if state?.is_institution_ready}
            <Badge variant="outline" class="border-emerald-500/30 text-emerald-600 dark:text-emerald-400 bg-emerald-500/5 font-normal">
              {m.ui_onboarding_progress_ready_for_operations()}
            </Badge>
          {:else}
            <Badge variant="secondary" class="text-amber-600 dark:text-amber-400 font-normal">
              {m.ui_onboarding_progress_in_preparation()}
            </Badge>
          {/if}
        </div>
      </div>

      <Progress value={state?.progress_percentage ?? 0} max={100} class="h-1.5" />
    </CardContent>
  </Card>

  <!-- Recommended Next Action (Subtle Notice) -->
  {#if state?.recommended_next_task && isTaskAccessible(steps, state.recommended_next_task.id)}
    {@const next = state.recommended_next_task}
    <div class="rounded-xl border border-border/80 bg-muted/30 p-4 flex flex-col sm:flex-row sm:items-center justify-between gap-3">
      <div class="space-y-0.5">
        <span class="text-[11px] font-medium text-muted-foreground">
          {m.ui_onboarding_progress_recommended_next_step()}
        </span>
        <h3 class="text-sm font-semibold text-foreground">
          {isArabic ? next.title_ar : next.title}
        </h3>
        <p class="text-xs text-muted-foreground">
          {isArabic ? next.description_ar : next.description}
        </p>
      </div>

      <Button
        variant="default"
        size="sm"
        onclick={() => handleNavigate(next)}
        class="h-8 text-xs shrink-0"
      >
        <span>{m.ui_onboarding_progress_configure()}</span>
      </Button>
    </div>
  {/if}

  <!-- Toolbar: Search & Status Filters -->
  <div class="flex flex-col sm:flex-row items-stretch sm:items-center justify-between gap-3 pt-1">
    <div class="relative flex-1 max-w-sm">
      <Input
        type="text"
        placeholder={m.ui_onboarding_progress_search_tasks()}
        bind:value={searchQuery}
        class="h-9 text-xs"
      />
    </div>

    <div class="flex items-center flex-wrap gap-1.5 text-xs">
      <!-- Status Buttons -->
      <div class="flex items-center gap-1 p-0.5 rounded-lg border border-border/60 bg-muted/30">
        <button
          type="button"
          onclick={() => (selectedStatus = "all")}
          class="px-2.5 py-1 rounded-md transition-colors {selectedStatus === 'all' ? 'bg-background text-foreground shadow-xs font-medium' : 'text-muted-foreground hover:text-foreground'}"
        >
          {m.common_filter_all()} ({tasks.length})
        </button>
        <button
          type="button"
          onclick={() => (selectedStatus = "required")}
          class="px-2.5 py-1 rounded-md transition-colors {selectedStatus === 'required' ? 'bg-background text-foreground shadow-xs font-medium' : 'text-muted-foreground hover:text-foreground'}"
        >
          {m.ui_onboarding_progress_required_2()} ({tasks.filter((t) => t.is_required).length})
        </button>
        <button
          type="button"
          onclick={() => (selectedStatus = "pending")}
          class="px-2.5 py-1 rounded-md transition-colors {selectedStatus === 'pending' ? 'bg-background text-foreground shadow-xs font-medium' : 'text-muted-foreground hover:text-foreground'}"
        >
          {m.ui_onboarding_progress_pending()} ({tasks.filter((t) => t.status !== 'completed').length})
        </button>
        <button
          type="button"
          onclick={() => (selectedStatus = "blocked")}
          class="px-2.5 py-1 rounded-md transition-colors {selectedStatus === 'blocked' ? 'bg-background text-foreground shadow-xs font-medium' : 'text-muted-foreground hover:text-foreground'}"
        >
          {m.ui_onboarding_progress_blocked_3()} ({tasks.filter((t) => t.status === 'blocked').length})
        </button>
        <button
          type="button"
          onclick={() => (selectedStatus = "completed")}
          class="px-2.5 py-1 rounded-md transition-colors {selectedStatus === 'completed' ? 'bg-background text-foreground shadow-xs font-medium' : 'text-muted-foreground hover:text-foreground'}"
        >
          {m.ui_onboarding_progress_done()} ({tasks.filter((t) => t.status === 'completed').length})
        </button>
      </div>

      <!-- Category Filter Dropdown -->
      {#if categories.length > 0}
        <Select
          type="single"
          value={selectedCategory}
          onValueChange={(v) => (selectedCategory = v ?? "all")}
        >
          <SelectTrigger class="h-9 text-xs">
            {selectedCategoryLabel}
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">{m.ui_onboarding_progress_all_categories()}</SelectItem>
            {#each categories as cat (cat.category)}
              <SelectItem value={cat.category}>
                {isArabic ? cat.label_ar : cat.label} ({cat.completed_tasks}/{cat.total_tasks})
              </SelectItem>
            {/each}
          </SelectContent>
        </Select>
      {/if}
    </div>
  </div>

  <!-- Tasks Grid -->
  <div class="grid grid-cols-1 md:grid-cols-2 gap-3.5">
    {#each filteredTasks as task (task.id)}
      {@const statusInfo = getStatusInfo(task.status)}
      {@const isBlocked = task.status === "blocked"}
      {@const isComplete = task.status === "completed"}
      {@const step = findStepForTask(steps, task.id)}
      {@const isAccessible = isTaskAccessible(steps, task.id)}

      <Card class="border-border/60 shadow-xs hover:border-border transition-colors flex flex-col justify-between {isBlocked ? 'opacity-70 bg-muted/10' : ''}">
        <CardContent class="p-4 space-y-3">
          <!-- Header: Title, Category & Status Badge -->
          <div class="flex items-start justify-between gap-3">
            <div class="space-y-0.5">
              <div class="flex items-center gap-1.5 flex-wrap">
                <h3 class="text-sm font-semibold text-foreground">
                  {isArabic ? task.title_ar : task.title}
                </h3>
                {#if task.is_required}
                  <Badge variant="outline" class="text-[10px] px-1.5 py-0 h-4 font-normal text-muted-foreground">
                    {m.ui_onboarding_progress_required_3()}
                  </Badge>
                {/if}
              </div>
              <span class="text-[11px] text-muted-foreground">
                {getCategoryLabel(task.category)}
              </span>
            </div>

            <Badge
              variant={statusInfo.variant}
              class="text-[11px] px-2 py-0.5 font-normal {statusInfo.classes}"
            >
              {statusInfo.label}
            </Badge>
          </div>

          <!-- Description -->
          <p class="text-xs text-muted-foreground leading-relaxed">
            {isArabic ? task.description_ar : task.description}
          </p>

          <!-- Prerequisite condition (if blocked or helpful) -->
          {#if isBlocked && task.blocking_reason}
            <div class="text-[11px] text-muted-foreground border-t border-border/40 pt-2">
              <span class="font-medium text-foreground">{m.ui_onboarding_progress_prerequisite()}</span>
              <span class="ms-1">{isArabic && task.blocking_reason_ar ? task.blocking_reason_ar : task.blocking_reason}</span>
            </div>
          {/if}
        </CardContent>

        <!-- Footer: Action Button -->
        <CardFooter class="px-4 py-3 border-t border-border/40 bg-muted/10 flex items-center justify-between gap-3">
          <span class="text-[11px] font-mono text-muted-foreground">
            {task.progress}%
          </span>

          <Button
            variant={isComplete ? "outline" : "default"}
            size="sm"
            disabled={!isAccessible}
            onclick={() => handleNavigate(task)}
            class="h-8 text-xs font-medium"
          >
            {#if isComplete}
              <span>{m.ui_onboarding_progress_review()}</span>
            {:else if isBlocked}
              <span>{m.academics_exams_center_locked()}</span>
            {:else}
              <span>{m.ui_setup_card_configure()}</span>
            {/if}
          </Button>
        </CardFooter>
      </Card>
    {:else}
      <div class="col-span-full py-16 text-center text-muted-foreground space-y-1">
        <p class="text-sm font-medium">{m.ui_onboarding_progress_no_tasks_match_your_filters()}</p>
        <p class="text-xs">{m.ui_onboarding_progress_try_adjusting_your_search_query_or_status_filter()}</p>
      </div>
    {/each}
  </div>
</div>
{/if}

<!-- Auto-Scaffolding Blueprint Dialog -->
<Dialog bind:open={isProvisionModalOpen}>
  <DialogContent class="sm:max-w-md">
    <DialogHeader>
      <DialogTitle class="text-base font-semibold">
        {m.ui_onboarding_progress_auto_scaffold_blueprint_defaults()}
      </DialogTitle>
      <DialogDescription class="text-xs">
        {m.shared_ui_provision_defaults_description()}
      </DialogDescription>
    </DialogHeader>

    {#if provisioningError}
      <div class="p-3 rounded-lg border border-destructive/30 bg-destructive/10 text-destructive text-xs">
        {provisioningError}
      </div>
    {/if}

    <div class="space-y-3.5 py-2 text-xs">
      <BilingualNameFields
        arId="prov-year-name-ar"
        enId="prov-year-name"
        arLabel={m.ui_onboarding_progress_year_name_ar()}
        enLabel={m.ui_onboarding_progress_year_name_en()}
        bind:arValue={provYearNameAr}
        bind:enValue={provYearName}
        arPlaceholder={m.shared_ui_example_academic_year()}
        enPlaceholder={m.shared_ui_example_academic_year()}
        kind="label"
        requestSuggestion={suggestLocalTranslation}
        replaceUneditedInitialValues
        disabled={isProvisioning}
        loadingMessage={m.bilingual_entry_loading()}
        failureMessage={m.bilingual_entry_failure()}
      />

      <div class="space-y-2 pt-2 border-t border-border/40">
        <span class="text-xs font-medium text-foreground block">
          {m.ui_onboarding_progress_components_to_generate()}
        </span>

        <div class="flex items-center gap-2.5 py-1">
          <Checkbox bind:checked={provStagesAndGrades} id="prov-stages" />
          <Label for="prov-stages" class="text-xs font-normal text-foreground cursor-pointer">
            {m.ui_onboarding_progress_classroom_sections_grades_1_3()}
          </Label>
        </div>

        <div class="flex items-center gap-2.5 py-1">
          <Checkbox bind:checked={provSubjects} id="prov-subjects" />
          <Label for="prov-subjects" class="text-xs font-normal text-foreground cursor-pointer">
            {m.ui_onboarding_progress_core_curriculum_subjects()}
          </Label>
        </div>

        <div class="flex items-center gap-2.5 py-1">
          <Checkbox bind:checked={provAssessmentScheme} id="prov-assessment" />
          <Label for="prov-assessment" class="text-xs font-normal text-foreground cursor-pointer">
            {m.ui_onboarding_progress_standard_100_point_assessment_scheme()}
          </Label>
        </div>

        <div class="flex items-center gap-2.5 py-1">
          <Checkbox bind:checked={provTuitionFee} id="prov-fee" />
          <Label for="prov-fee" class="text-xs font-normal text-foreground cursor-pointer">
            {m.ui_onboarding_progress_standard_annual_tuition_template()}
          </Label>
        </div>
      </div>
    </div>

    <DialogFooter class="gap-2 sm:gap-0">
      <Button
        variant="outline"
        size="sm"
        onclick={() => (isProvisionModalOpen = false)}
        disabled={isProvisioning}
        class="text-xs"
      >
        {m.common_action_cancel()}
      </Button>

      <Button
        variant="default"
        size="sm"
        onclick={handleProvisionDefaults}
        disabled={isProvisioning}
        class="text-xs"
      >
        {isProvisioning
          ? (m.ui_onboarding_progress_scaffolding())
          : (m.ui_onboarding_progress_scaffold_now())}
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
