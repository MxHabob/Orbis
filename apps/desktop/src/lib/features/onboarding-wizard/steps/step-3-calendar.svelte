<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import { onMount } from "svelte";
import * as m from "$lib/paraglide/messages";
import { toast } from "@orbis/ui";
import { api, type AcademicYearDto, type AcademicPeriodDto } from "$lib/api";
import { orgContext, systemStore } from "$lib/stores";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Badge } from "@orbis/ui/badge";
import OnboardingStepHeader from "../onboarding-step-header.svelte";
import OnboardingSection from "../onboarding-section.svelte";
import { getTasksForStep } from "../onboarding-steps";
import { refreshOnboardingState } from "../refresh-onboarding-state";
import { BilingualNameFields } from "@orbis/ui/bilingual-name-fields";
import { suggestLocalTranslation } from "$lib/shared/translation/local-engine";
import DeleteRecordButton from "../components/delete-record-button.svelte";

let { orgId }: { orgId: string } = $props();

let isLoading = $state(true);
let isSubmittingYear = $state(false);
let isSubmittingPeriod = $state(false);
let successMessage = $state<string | null>(null);

// Existing Data
let years = $state<AcademicYearDto[]>([]);
let periods = $state<AcademicPeriodDto[]>([]);

// Academic Year Form
let yearName = $state("");
let yearNameAr = $state("");
let yearStartDate = $state("");
let yearEndDate = $state("");

// Selected Year for Period Creation
let selectedYearId = $state<string>("");
let periodName = $state("");
let periodNameAr = $state("");
let periodStartDate = $state("");
let periodEndDate = $state("");
let periodType = $state("custom");
let periodKind = $state("teaching");
let periodParentId = $state("");

const stepTasks = $derived(getTasksForStep("calendar", systemStore.onboardingTasks));
const isTaskComplete = $derived(stepTasks.length > 0 && stepTasks.every((t) => t.status === "completed"));
const selectedYear = $derived(years.find((year) => String(year.id) === selectedYearId));

function getHijriCalendarDefaults(startDate: string) {
  const formatter = new Intl.DateTimeFormat("en-US-u-ca-islamic-umalqura", {
    year: "numeric",
    month: "numeric",
    day: "numeric",
    timeZone: "UTC",
  });
  if (formatter.resolvedOptions().calendar !== "islamic-umalqura") {
    throw new Error(m.onboarding_step3_default_periods_calendar_error());
  }

  let cursorTime = Date.parse(`${startDate.slice(0, 10)}T00:00:00.000Z`);
  if (Number.isNaN(cursorTime)) {
    throw new Error(m.onboarding_step3_default_periods_calendar_error());
  }

  const getParts = (date: Date) => {
    const values = Object.fromEntries(
      formatter.formatToParts(date).map(({ type, value }) => [type, value]),
    );
    return {
      year: Number(values.year),
      month: Number(values.month),
      day: Number(values.day),
    };
  };

  const start = getParts(new Date(cursorTime));
  if (start.month !== 1 || start.day !== 1) {
    throw new Error(m.onboarding_step3_default_periods_start_error());
  }

  const monthLengths: number[] = [];
  for (let index = 0; index < 8; index += 1) {
    const targetYear = start.year + Math.floor(index / 12);
    const targetMonth = (index % 12) + 1;
    let days = 0;
    while (true) {
      const current = getParts(new Date(cursorTime));
      if (current.year !== targetYear || current.month !== targetMonth) break;
      days += 1;
      if (days > 30) {
        throw new Error(m.onboarding_step3_default_periods_calendar_error());
      }
      cursorTime += 24 * 60 * 60 * 1000;
    }
    if (days < 29 || days > 30) {
      throw new Error(m.onboarding_step3_default_periods_calendar_error());
    }
    monthLengths.push(days);
  }

  return { hijriStartYear: start.year, monthLengths };
}

function validateDateRange(start: string, end: string, message: string) {
  if (!start || !end || end <= start) {
    toast.error(message);
    return false;
  }
  return true;
}

onMount(async () => {
  await loadData();
});

async function loadData() {
  try {
    isLoading = true;
    const [yearsRes, periodsRes] = await Promise.all([
      api.listAcademicYears(),
      api.listAcademicPeriods(),
    ]);
    years = yearsRes;
    periods = periodsRes;
    if (years.length > 0 && !selectedYearId) {
      selectedYearId = String(years[0].id);
    }
  } catch (err: any) {
    console.error("Failed to load calendar data:", err);
    toast.error(err?.message || m.common_action_failed());
  } finally {
    isLoading = false;
  }
}

async function handleCreateYear() {
  if (!yearName.trim() && !yearNameAr.trim()) {
    toast.error(m.onboarding_step3_year_err_name());
    return;
  }
  if (!validateDateRange(yearStartDate, yearEndDate, m.onboarding_step3_year_err_dates())) {
    return;
  }

  isSubmittingYear = true;
  successMessage = null;

  try {
    const newYear = await api.createAcademicYear({
      name: yearName.trim() || yearNameAr.trim(),
      name_ar: yearNameAr.trim() || undefined,
      start_date: new Date(`${yearStartDate}T00:00:00Z`).toISOString(),
      end_date: new Date(`${yearEndDate}T23:59:59Z`).toISOString(),
    });

    years = [...years, newYear];
    selectedYearId = String(newYear.id);
    yearName = "";
    yearNameAr = "";
    yearStartDate = "";
    yearEndDate = "";
    await refreshOnboardingState(orgId, m.common_action_failed());
    successMessage = m.onboarding_step3_year_created_success();
  } catch (err: any) {
    console.error("Failed to create academic year:", err);
    toast.error(err?.message || m.common_action_failed());
  } finally {
    isSubmittingYear = false;
  }
}

async function handleCreatePeriod() {
  if (!selectedYearId) {
    toast.error(m.onboarding_step3_period_err_year());
    return;
  }
  if (!periodName.trim() && !periodNameAr.trim()) {
    toast.error(m.onboarding_step3_year_err_name());
    return;
  }
  if (!validateDateRange(periodStartDate, periodEndDate, m.onboarding_step3_year_err_dates())) {
    return;
  }

  isSubmittingPeriod = true;
  successMessage = null;
  try {
    const created = await api.createAcademicPeriod({
      year_id: selectedYearId,
      parent_id: periodParentId || undefined,
      name: periodName.trim() || periodNameAr.trim(),
      name_ar: periodNameAr.trim() || undefined,
      period_type: periodType,
      period_kind: periodKind,
      start_date: new Date(`${periodStartDate}T00:00:00Z`).toISOString(),
      end_date: new Date(`${periodEndDate}T23:59:59Z`).toISOString(),
    });
    periods = [...periods, created];
    periodName = "";
    periodNameAr = "";
    periodStartDate = "";
    periodEndDate = "";
    periodParentId = "";
    await refreshOnboardingState(orgId, m.common_action_failed());
    successMessage = m.onboarding_step3_periods_generated_success();
  } catch (err: unknown) {
    console.error("Failed to create academic period:", err);
    toast.error(err instanceof Error ? err.message : m.common_action_failed());
  } finally {
    isSubmittingPeriod = false;
  }
}

async function handleGenerateDefaultPeriods() {
  if (!orgContext.isSchool) return;
  if (!selectedYear) {
    toast.error(m.onboarding_step3_period_err_year());
    return;
  }

  let calendarDefaults: ReturnType<typeof getHijriCalendarDefaults>;
  try {
    calendarDefaults = getHijriCalendarDefaults(selectedYear.start_date);
  } catch (err: unknown) {
    toast.error(err instanceof Error ? err.message : m.common_action_failed());
    return;
  }

  isSubmittingPeriod = true;
  successMessage = null;
  try {
    await api.onboardingProvisionDefaults(
      {
        academic_year_id: String(selectedYear.id),
        provision_hijri_school_calendar: true,
        hijri_start_year: calendarDefaults.hijriStartYear,
        hijri_month_lengths: calendarDefaults.monthLengths,
      },
      orgId,
    );
    periods = await api.listAcademicPeriods();
    await refreshOnboardingState(orgId, m.common_action_failed());
    successMessage = m.onboarding_step3_periods_generated_success();
  } catch (err: unknown) {
    console.error("Failed to generate default school periods:", err);
    toast.error(err instanceof Error ? err.message : m.common_action_failed());
  } finally {
    isSubmittingPeriod = false;
  }
}

async function deletePeriod(period: AcademicPeriodDto) {
  const deleted = await api.deleteAcademicPeriod(String(period.id));
  if (!deleted) throw new Error(m.common_action_failed());
  periods = periods.filter((item) => item.id !== period.id);
  await refreshOnboardingState(orgId, m.common_action_failed());
}

</script>

<div class="space-y-6">
  <OnboardingStepHeader
    stepNumber={3}
    title={m.onboarding_step3_title()}
    description={m.onboarding_step3_subtitle()}
    isRequired={true}
    status={isTaskComplete ? "completed" : years.length > 0 ? "in_progress" : "not_started"}
  />


  {#if successMessage}
    <div class="rounded-lg border border-emerald-500/30 bg-emerald-500/10 p-4 text-sm text-emerald-600 dark:text-emerald-400 font-medium flex items-center justify-between">
      <span>{successMessage}</span>
      <Button variant="ghost" size="sm" onclick={() => successMessage = null}>✕</Button>
    </div>
  {/if}

  {#if isLoading}
    <div class="flex items-center justify-center p-12 text-muted-foreground text-sm gap-2">
      <LoadingIndicator size={16} class="size-4 text-primary" />
      <span>{m.common_control_loading()}</span>
    </div>
  {:else}
    <!-- Section 1: Academic Year -->
    <OnboardingSection
      title={m.onboarding_step3_year_title()}
      description={m.onboarding_step3_year_desc()}
      badge={years.length > 0 ? `${years.length}` : undefined}
      isComplete={years.length > 0}
    >
      <div class="space-y-4">
        <!-- Year Form -->
        <div class="rounded-lg border border-border/40 bg-muted/20 p-4 space-y-4">
          <div class="grid gap-3 sm:grid-cols-2">
            <div class="sm:col-span-2">
              <BilingualNameFields
                arId="onboarding-year-name-ar"
                enId="onboarding-year-name-en"
                arLabel={m.onboarding_step3_year_name_ar()}
                enLabel={m.onboarding_step3_year_name_en()}
                bind:arValue={yearNameAr}
                bind:enValue={yearName}
                arPlaceholder={m.onboarding_step3_year_name_ar_placeholder()}
                enPlaceholder="2026/2027"
                kind="label"
                requestSuggestion={suggestLocalTranslation}
                replaceUneditedInitialValues
                disabled={isSubmittingYear}
                loadingMessage={m.bilingual_entry_loading()}
                failureMessage={m.bilingual_entry_failure()}
              />
            </div>
            <div class="space-y-1.5">
              <Label for="onboarding-year-start" class="text-xs font-medium">
                {m.onboarding_step3_start_date()}
              </Label>
              <Input
                id="onboarding-year-start"
                type="date"
                bind:value={yearStartDate}
              />
            </div>
            <div class="space-y-1.5">
              <Label for="onboarding-year-end" class="text-xs font-medium">
                {m.onboarding_step3_end_date()}
              </Label>
              <Input
                id="onboarding-year-end"
                type="date"
                bind:value={yearEndDate}
              />
            </div>
          </div>

          <div class="flex flex-wrap justify-end gap-2">
            <Button
              variant="default"
              size="sm"
              disabled={isSubmittingYear || (!yearName.trim() && !yearNameAr.trim())}
              onclick={handleCreateYear}
              class="text-xs font-semibold"
            >
              {isSubmittingYear ? m.onboarding_step3_creating_btn() : m.onboarding_step3_create_year_btn()}
            </Button>
          </div>
        </div>

        <!-- Configured Years List -->
        {#if years.length > 0}
          <div class="space-y-2">
            <span class="text-xs font-semibold text-muted-foreground uppercase">
              {m.onboarding_step3_configured_years()}
            </span>
            <div class="divide-y divide-border/40 rounded-lg border border-border/50 bg-card overflow-hidden">
              {#each years as yr (yr.id)}
                <div class="flex items-center justify-between p-3 text-xs">
                  <div class="flex items-center gap-3">
                    <span class="flex size-7 items-center justify-center rounded-md bg-primary/10 font-bold text-primary text-[11px]">
                      📅
                    </span>
                    <div>
                      <div class="font-medium text-foreground">
                        {yr.name_ar || yr.name}
                      </div>
                      <div class="text-[11px] font-mono text-muted-foreground">
                        {yr.start_date.split('T')[0]} → {yr.end_date.split('T')[0]}
                      </div>
                    </div>
                  </div>
                  <Badge variant="outline" class="border-emerald-500/30 text-emerald-600 dark:text-emerald-400 text-[10px]">
                    {m.onboarding_active_badge()}
                  </Badge>
                </div>
              {/each}
            </div>
          </div>
        {/if}
      </div>
    </OnboardingSection>

    <!-- Section 2: Academic Terms / Periods -->
    <OnboardingSection
      title={m.onboarding_step3_periods_title()}
      description={m.onboarding_step3_periods_desc()}
      badge={periods.length > 0 ? `${periods.length}` : undefined}
      isComplete={periods.length > 0}
    >
      <div class="space-y-4">
        {#if orgContext.isSchool}
          <div class="flex flex-wrap items-center justify-between gap-3 rounded-lg border border-primary/20 bg-primary/5 p-4">
            <p class="text-xs text-muted-foreground">
              {m.onboarding_step3_default_periods_desc()}
            </p>
            <Button
              type="button"
              variant="outline"
              size="sm"
              disabled={isSubmittingPeriod || !selectedYearId}
              onclick={handleGenerateDefaultPeriods}
              class="text-xs font-semibold"
            >
              {isSubmittingPeriod
                ? m.onboarding_step3_creating_btn()
                : m.onboarding_step3_default_periods_btn()}
            </Button>
          </div>
        {/if}

        <div class="rounded-lg border border-border/40 bg-muted/20 p-4 space-y-4">
          <div class="grid gap-3 sm:grid-cols-2">
            <div class="sm:col-span-2">
              <BilingualNameFields
                arId="onboarding-period-name-ar"
                enId="onboarding-period-name-en"
                arLabel={m.onboarding_step3_year_name_ar()}
                enLabel={m.onboarding_step3_year_name_en()}
                bind:arValue={periodNameAr}
                bind:enValue={periodName}
                arPlaceholder={m.onboarding_step3_year_name_ar_placeholder()}
                enPlaceholder="Period name"
                kind="label"
                requestSuggestion={suggestLocalTranslation}
                replaceUneditedInitialValues
                disabled={isSubmittingPeriod}
                loadingMessage={m.bilingual_entry_loading()}
                failureMessage={m.bilingual_entry_failure()}
              />
            </div>
            <div class="space-y-1.5">
              <Label for="onboarding-period-type" class="text-xs font-medium">
                Period type
              </Label>
              <select
                id="onboarding-period-type"
                class="h-10 w-full rounded-md border border-input bg-background px-3 text-sm"
                bind:value={periodType}
                disabled={isSubmittingPeriod}
              >
                <option value="custom">Custom</option>
                <option value="semester">Semester</option>
                <option value="trimester">Trimester</option>
                <option value="quarter">Quarter</option>
                <option value="month">Month</option>
                <option value="module">Module</option>
                <option value="block">Block</option>
              </select>
            </div>
            <div class="space-y-1.5">
              <Label for="onboarding-period-kind" class="text-xs font-medium">
                Period role
              </Label>
              <select
                id="onboarding-period-kind"
                class="h-10 w-full rounded-md border border-input bg-background px-3 text-sm"
                bind:value={periodKind}
                disabled={isSubmittingPeriod}
              >
                <option value="teaching">Teaching</option>
                <option value="examination">Examination</option>
                <option value="break">Break</option>
                <option value="registration">Registration</option>
                <option value="preparatory">Preparatory</option>
              </select>
            </div>
            <div class="space-y-1.5">
              <Label for="onboarding-period-start" class="text-xs font-medium">
                {m.onboarding_step3_start_date()}
              </Label>
              <Input id="onboarding-period-start" type="date" bind:value={periodStartDate} />
            </div>
            <div class="space-y-1.5">
              <Label for="onboarding-period-end" class="text-xs font-medium">
                {m.onboarding_step3_end_date()}
              </Label>
              <Input id="onboarding-period-end" type="date" bind:value={periodEndDate} />
            </div>
            {#if periods.some((period) => period.year_id === selectedYearId)}
              <div class="space-y-1.5 sm:col-span-2">
                <Label for="onboarding-period-parent" class="text-xs font-medium">
                  Parent period (optional)
                </Label>
                <select
                  id="onboarding-period-parent"
                  class="h-10 w-full rounded-md border border-input bg-background px-3 text-sm"
                  bind:value={periodParentId}
                  disabled={isSubmittingPeriod}
                >
                  <option value="">No parent</option>
                  {#each periods.filter((period) => period.year_id === selectedYearId) as period (period.id)}
                    <option value={String(period.id)}>{period.name_ar || period.name}</option>
                  {/each}
                </select>
              </div>
            {/if}
          </div>
          <div class="flex justify-end">
            <Button
              variant="default"
              size="sm"
              disabled={isSubmittingPeriod || !selectedYearId}
              onclick={handleCreatePeriod}
              class="text-xs font-semibold"
            >
              Add period
            </Button>
          </div>
        </div>

        <!-- Existing Periods List -->
        {#if periods.length > 0}
          <div class="space-y-2">
            <span class="text-xs font-semibold text-muted-foreground uppercase">
              {m.onboarding_step3_configured_periods()}
            </span>
            <div class="grid gap-2 sm:grid-cols-2">
              {#each periods as p (p.id)}
                <div class="flex items-center justify-between rounded-lg border border-border/50 bg-card p-3 text-xs">
                  <div class="flex items-center gap-2">
                    <span class="flex size-6 items-center justify-center rounded bg-primary/10 text-primary text-[10px] font-bold">
                      📑
                    </span>
                    <div>
                      <div class="font-medium text-foreground">
                        {p.name_ar || p.name}
                      </div>
                      <div class="text-[10px] text-muted-foreground">
                        {p.start_date.split('T')[0]} → {p.end_date.split('T')[0]}
                      </div>
                    </div>
                  </div>
                  <div class="flex items-center gap-2">
                    <Badge variant="secondary" class="text-[10px] uppercase">
                      {p.period_type}
                    </Badge>
                    <DeleteRecordButton label={p.name_ar || p.name} onDelete={() => deletePeriod(p)} />
                  </div>
                </div>
              {/each}
            </div>
          </div>
        {/if}
      </div>
    </OnboardingSection>

  {/if}
</div>
