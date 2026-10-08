<script lang="ts">
  import { createMutation, useQueryClient } from "@tanstack/svelte-query";
  import { onMount } from "svelte";
  import * as m from "$lib/paraglide/messages";
  import { toast } from "@orbis/ui";
  import {
    api,
    queryKeys,
    type AcademicYearDto,
    type ProvisionDefaultsReqDto,
  } from "$lib/api";
  import { systemStore } from "$lib/stores";
  import { refreshOnboardingState } from "../refresh-onboarding-state";
  import { Button } from "@orbis/ui/button";
  import { Checkbox } from "@orbis/ui/checkbox";
  import { Label } from "@orbis/ui/label";
  import { Card, CardContent } from "@orbis/ui/card";
  import OnboardingStepHeader from "../onboarding-step-header.svelte";
  import OnboardingPresetBar from "../components/onboarding-preset-bar.svelte";
  import { getTasksForStep } from "../onboarding-steps";

  let { orgId }: { orgId: string } = $props();

  let years = $state<AcademicYearDto[]>([]);
  let selectedYearId = $state("");
  let enablePeriodAutopilot = $state(false);
  let isLoading = $state(true);
  const queryClient = useQueryClient();
  const provisionDefaultsMutation = createMutation(() => ({
    mutationFn: (request: ProvisionDefaultsReqDto) =>
      api.onboardingProvisionDefaults(request, orgId),
    onSuccess: async () => {
      await queryClient.invalidateQueries({
        queryKey: queryKeys.onboarding.state(orgId),
      });
    },
  }));
  const isSubmitting = $derived(provisionDefaultsMutation.isPending);
  let successMessage = $state<string | null>(null);

  const defaultTimeSlots = $derived([
    { name: m.onboarding_step_scheduling_lesson_1(), start: "07:30", end: "08:15" },
    { name: m.onboarding_step_scheduling_lesson_2(), start: "08:15", end: "09:00" },
    { name: m.onboarding_step_scheduling_morning_break(), start: "09:00", end: "09:15" },
    { name: m.onboarding_step_scheduling_lesson_3(), start: "09:15", end: "10:00" },
    { name: m.onboarding_step_scheduling_lesson_4(), start: "10:00", end: "10:45" },
    { name: m.onboarding_step_scheduling_lunch_break(), start: "10:45", end: "11:15" },
    { name: m.onboarding_step_scheduling_lesson_5(), start: "11:15", end: "12:00" },
    { name: m.onboarding_step_scheduling_lesson_6(), start: "12:00", end: "12:45" },
    { name: m.onboarding_step_scheduling_lesson_7(), start: "12:45", end: "13:30" },
  ]);
  const schoolDays = $derived([
    m.common_day_sunday(),
    m.common_day_monday(),
    m.common_day_tuesday(),
    m.common_day_wednesday(),
    m.common_day_thursday(),
  ]);

  const selectedYear = $derived(
    years.find((year) => String(year.id) === selectedYearId),
  );
  const stepTasks = $derived(
    getTasksForStep("scheduling", systemStore.onboardingTasks),
  );
  const isTaskComplete = $derived(
    stepTasks.length > 0 &&
      stepTasks.every((task) => task.status === "completed"),
  );

  onMount(loadYears);

  async function loadYears() {
    try {
      isLoading = true;
      years = await api.listAcademicYears();
      if (!selectedYearId && years.length > 0) {
        selectedYearId = String(years.find((year) => year.status === "active")?.id ?? years[0].id);
      }
    } catch (error) {
      console.error("Failed to load academic years for scheduling setup:", error);
      toast.error(error instanceof Error ? error.message : m.common_action_failed());
    } finally {
      isLoading = false;
    }
  }

  async function provisionSchedule() {
    if (!selectedYear) {
      toast.error(m.onboarding_step_scheduling_missing_calendar());
      return;
    }

    successMessage = null;
    try {
      await provisionDefaultsMutation.mutateAsync({
        academic_year_id: String(selectedYear.id),
        provision_hijri_school_calendar: false,
        provision_stages_and_grades: false,
        provision_subjects: false,
        provision_assessment_scheme: false,
        provision_tuition_fee_template: false,
        provision_time_slots: true,
        enable_period_autopilot: enablePeriodAutopilot,
      });
      await refreshOnboardingState(orgId, m.common_action_failed());
      successMessage = m.onboarding_step_scheduling_success();
    } catch (error) {
      console.error("Failed to provision the school timetable:", error);
      toast.error(error instanceof Error ? error.message : m.common_action_failed());
    }
  }
</script>

<div class="space-y-6">
  <OnboardingStepHeader
    stepNumber={5}
    title={m.onboarding_step_scheduling_title()}
    description={m.onboarding_step_scheduling_subtitle()}
    isRequired={true}
    status={isTaskComplete ? "completed" : years.length > 0 ? "in_progress" : "not_started"}
  />

  {#if successMessage}
    <div class="rounded-lg border border-emerald-500/30 bg-emerald-500/10 p-4 text-sm font-medium text-emerald-600 dark:text-emerald-400" role="status">
      {successMessage}
    </div>
  {/if}

  {#if isLoading}
    <div class="flex items-center justify-center p-12 text-sm text-muted-foreground">
      {m.common_control_loading()}
    </div>
  {:else}
    <Card>
      <CardContent class="space-y-4 p-5">
      <div class="space-y-1">
        <h2 class="text-base font-semibold">{m.onboarding_step_scheduling_year()}</h2>
        <p class="text-sm text-muted-foreground">{m.onboarding_step_scheduling_desc()}</p>
      </div>

      <select
        class="h-10 w-full rounded-md border border-input bg-background px-3 text-sm"
        bind:value={selectedYearId}
        disabled={years.length === 0 || isSubmitting}
        aria-label={m.onboarding_step_scheduling_year()}
      >
        {#each years as year (year.id)}
          <option value={String(year.id)}>{year.name_ar || year.name}</option>
        {/each}
      </select>

      {#if selectedYear}
        <OnboardingPresetBar
          title={m.ui_copy_recommended_4917dbf2()}
          description={`7 ${m.onboarding_step_scheduling_daily_lessons()} · 2 ${m.onboarding_step_scheduling_breaks()} · 5 ${m.onboarding_step_scheduling_days()}`}
        >
          {#each schoolDays as day (day)}
            <span class="rounded-full bg-primary/10 px-2.5 py-1 text-xs text-primary">{day}</span>
          {/each}
          {#each defaultTimeSlots as slot (slot.start)}
            <span class="rounded-full bg-muted/50 px-2.5 py-1 text-xs">
              {slot.name} <time class="font-mono text-muted-foreground">{slot.start}–{slot.end}</time>
            </span>
          {/each}
        </OnboardingPresetBar>

        <div class="flex items-start gap-3 rounded-md border border-border/50 p-3">
          <Checkbox id="onboarding-period-autopilot" bind:checked={enablePeriodAutopilot} />
          <div class="space-y-0.5">
            <Label for="onboarding-period-autopilot" class="text-sm font-medium">
              {m.onboarding_step_scheduling_autopilot()}
            </Label>
            <p class="text-xs text-muted-foreground">
              {m.onboarding_step_scheduling_autopilot_desc()}
            </p>
          </div>
        </div>
      {/if}

      {#if years.length === 0}
        <p class="text-sm text-muted-foreground">{m.onboarding_step_scheduling_missing_calendar()}</p>
      {/if}

      <div class="flex justify-end border-t border-border/50 pt-4">
        <Button
          disabled={!selectedYear || isSubmitting}
          onclick={provisionSchedule}
        >
          {isSubmitting
            ? m.onboarding_step_scheduling_provisioning()
            : m.onboarding_step_scheduling_provision_btn()}
        </Button>
      </div>
      </CardContent>
    </Card>
  {/if}
</div>
