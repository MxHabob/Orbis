<script lang="ts">
  import { createMutation, useQueryClient } from "@tanstack/svelte-query";
  import { onMount } from "svelte";
  import * as m from "$lib/paraglide/messages";
  import {
    api,
    queryKeys,
    type AcademicYearDto,
    type ProvisionDefaultsReqDto,
  } from "$lib/api";
  import { getHijriSchoolMonthLengths } from "$lib/core/i18n/hijri";
  import { systemStore } from "$lib/stores";
  import { refreshOnboardingState } from "../refresh-onboarding-state";
  import { Button } from "@orbis/ui/button";
  import OnboardingStepHeader from "../onboarding-step-header.svelte";
  import { getTasksForStep } from "../onboarding-steps";

  let { orgId }: { orgId: string } = $props();

  let years = $state<AcademicYearDto[]>([]);
  let selectedYearId = $state("");
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
  let errorMessage = $state<string | null>(null);
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
      errorMessage =
        error instanceof Error ? error.message : m.common_action_failed();
    } finally {
      isLoading = false;
    }
  }

  async function provisionSchedule() {
    if (!selectedYear) {
      errorMessage = m.onboarding_step_scheduling_missing_calendar();
      return;
    }

    const hijriCalendar = getHijriSchoolMonthLengths(selectedYear.start_date);
    if (!hijriCalendar) {
      errorMessage = m.onboarding_step_scheduling_invalid_hijri_start();
      return;
    }

    errorMessage = null;
    successMessage = null;
    try {
      await provisionDefaultsMutation.mutateAsync({
          academic_year_id: String(selectedYear.id),
          academic_year_name: selectedYear.name,
          academic_year_name_ar: selectedYear.name_ar,
          start_date: selectedYear.start_date,
          end_date: selectedYear.end_date,
          provision_hijri_school_calendar: true,
          hijri_start_year: hijriCalendar.hijriYear,
          hijri_month_lengths: hijriCalendar.monthLengths,
          provision_stages_and_grades: true,
          provision_subjects: true,
          provision_assessment_scheme: true,
          provision_tuition_fee_template: true,
          provision_time_slots: true,
          enable_period_autopilot: true,
        });
      await refreshOnboardingState(orgId, m.common_action_failed());
      successMessage = m.onboarding_step_scheduling_success();
    } catch (error) {
      console.error("Failed to provision the school timetable:", error);
      errorMessage =
        error instanceof Error ? error.message : m.common_action_failed();
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

  {#if errorMessage}
    <div class="rounded-lg border border-destructive/30 bg-destructive/10 p-4 text-sm font-medium text-destructive" role="alert">
      {errorMessage}
    </div>
  {/if}

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
    <section class="space-y-4 rounded-xl border border-border/50 bg-card p-5">
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
        <div class="grid gap-3 sm:grid-cols-2">
          <div class="rounded-lg border border-border/50 bg-muted/20 p-4">
            <div class="text-sm font-semibold">{m.onboarding_step_scheduling_hijri_summary()}</div>
            <p class="mt-1 text-xs text-muted-foreground">8 {m.onboarding_step_scheduling_months()}</p>
          </div>
          <div class="rounded-lg border border-border/50 bg-muted/20 p-4">
            <div class="text-sm font-semibold">{m.onboarding_step_scheduling_lessons()}</div>
            <p class="mt-1 text-xs text-muted-foreground">
              7 {m.onboarding_step_scheduling_daily_lessons()} · 2 {m.onboarding_step_scheduling_breaks()} · 5 {m.onboarding_step_scheduling_days()}
            </p>
          </div>
          <div class="rounded-lg border border-border/50 bg-muted/20 p-4">
            <div class="text-sm font-semibold">{m.onboarding_step_scheduling_exams()}</div>
            <p class="mt-1 text-xs text-muted-foreground">{m.onboarding_step_scheduling_exams_desc()}</p>
          </div>
          <div class="rounded-lg border border-border/50 bg-muted/20 p-4">
            <div class="text-sm font-semibold">{m.onboarding_step_scheduling_autopilot()}</div>
            <p class="mt-1 text-xs text-muted-foreground">{m.onboarding_step_scheduling_autopilot_desc()}</p>
          </div>
        </div>

        <div class="space-y-2 rounded-lg border border-border/50 p-4">
          <h3 class="text-sm font-semibold">{m.onboarding_step_scheduling_lessons()}</h3>
          <div class="flex flex-wrap gap-1.5" aria-label={m.onboarding_step_scheduling_days()}>
            {#each schoolDays as day (day)}
              <span class="rounded-full bg-primary/10 px-2.5 py-1 text-xs text-primary">{day}</span>
            {/each}
          </div>
          <div class="grid gap-1 sm:grid-cols-2">
            {#each defaultTimeSlots as slot (slot.start)}
              <div class="flex items-center justify-between gap-3 rounded-md bg-muted/30 px-3 py-2 text-xs">
                <span>{slot.name}</span>
                <time class="font-mono text-muted-foreground">{slot.start}–{slot.end}</time>
              </div>
            {/each}
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
    </section>
  {/if}
</div>
