<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import { onMount } from "svelte";
import * as m from "$lib/paraglide/messages";
import { api, type AcademicYearDto, type AcademicPeriodDto } from "$lib/api";
import {
  getHijriSchoolMonthLengths,
  getNextHijriSchoolCalendar,
} from "$lib/core/i18n/hijri";
import { systemStore } from "$lib/stores";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Badge } from "@orbis/ui/badge";
import OnboardingStepHeader from "../onboarding-step-header.svelte";
import OnboardingSection from "../onboarding-section.svelte";
import { getTasksForStep } from "../onboarding-steps";
import { navigateOnboardingRoute } from "../navigation";

let { orgId }: { orgId: string } = $props();

let isLoading = $state(true);
let isSubmittingYear = $state(false);
let isSubmittingPeriod = $state(false);
let errorMessage = $state<string | null>(null);
let successMessage = $state<string | null>(null);

// Existing Data
let years = $state<AcademicYearDto[]>([]);
let periods = $state<AcademicPeriodDto[]>([]);

function dateInputValue(date: Date) {
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, "0");
  const day = String(date.getDate()).padStart(2, "0");
  return `${year}-${month}-${day}`;
}

const defaultSchoolCalendar = getNextHijriSchoolCalendar();
const defaultHijriYear =
  defaultSchoolCalendar?.hijriYear ?? new Date().getFullYear() - 579;
const defaultYearStart = defaultSchoolCalendar?.startDate ?? new Date();
const defaultYearEnd = defaultSchoolCalendar?.endDate ?? new Date();

// Academic Year Form
let yearName = $state(`${defaultHijriYear}/${defaultHijriYear + 1}`);
let yearNameAr = $state(`العام الدراسي ${defaultHijriYear}-${defaultHijriYear + 1}هـ`);
let yearStartDate = $state(dateInputValue(defaultYearStart));
let yearEndDate = $state(dateInputValue(defaultYearEnd));

// Selected Year for Period Creation
let selectedYearId = $state<string>("");

const stepTasks = $derived(getTasksForStep("calendar", systemStore.onboardingTasks));
const isTaskComplete = $derived(stepTasks.length > 0 && stepTasks.every((t) => t.status === "completed"));
const isSchool = $derived(systemStore.onboardingState?.institution_type === "school");

onMount(async () => {
  await loadData();
});

async function loadData() {
  try {
    isLoading = true;
    const [yearsRes, periodsRes] = await Promise.all([
      api.listAcademicYears().catch(() => []),
      api.listAcademicPeriods().catch(() => []),
    ]);
    years = yearsRes;
    periods = periodsRes;
    if (years.length > 0 && !selectedYearId) {
      selectedYearId = String(years[0].id);
    }
  } catch (err: any) {
    console.error("Failed to load calendar data:", err);
    errorMessage = err?.message || m.common_action_failed();
  } finally {
    isLoading = false;
  }
}

async function handleCreateYear() {
  if (!yearName.trim() && !yearNameAr.trim()) {
    errorMessage = m.onboarding_step3_year_err_name();
    return;
  }
  if (!yearStartDate || !yearEndDate) {
    errorMessage = m.onboarding_step3_year_err_dates();
    return;
  }

  isSubmittingYear = true;
  errorMessage = null;
  successMessage = null;

  try {
    const newYear = await api.createAcademicYear({
      name: yearName.trim() || yearNameAr.trim(),
      name_ar: yearNameAr.trim() || undefined,
      start_date: new Date(yearStartDate).toISOString(),
      end_date: new Date(yearEndDate).toISOString(),
    });

    years = [...years, newYear];
    selectedYearId = String(newYear.id);
    await systemStore.loadOnboardingState(orgId);
    successMessage = m.onboarding_step3_year_created_success();
  } catch (err: any) {
    console.error("Failed to create academic year:", err);
    errorMessage = err?.message || m.common_action_failed();
  } finally {
    isSubmittingYear = false;
  }
}

async function handleGenerateSemesters(scheme: "2_semesters" | "3_trimesters" | "4_quarters") {
  if (!selectedYearId) {
    errorMessage = m.onboarding_step3_period_err_year();
    return;
  }

  isSubmittingPeriod = true;
  errorMessage = null;
  successMessage = null;

  try {
    const currentYear = years.find((year) => String(year.id) === String(selectedYearId));
    if (scheme === "2_semesters") {
      const calendar = getHijriSchoolMonthLengths(
        currentYear?.start_date ?? `${yearStartDate}T00:00:00Z`,
      );
      if (!calendar) {
        errorMessage = m.onboarding_step_scheduling_invalid_hijri_start();
        return;
      }
      await api.onboardingProvisionDefaults(
        {
          academic_year_id: selectedYearId,
          academic_year_name: currentYear?.name ?? yearName,
          academic_year_name_ar: currentYear?.name_ar ?? yearNameAr,
          start_date: currentYear?.start_date ?? new Date(yearStartDate).toISOString(),
          end_date: currentYear?.end_date ?? new Date(yearEndDate).toISOString(),
          provision_hijri_school_calendar: true,
          hijri_start_year: calendar.hijriYear,
          hijri_month_lengths: calendar.monthLengths,
          enable_period_autopilot: true,
        },
        orgId,
      );
      periods = await api.listAcademicPeriods();
      await systemStore.loadOnboardingState(orgId);
      successMessage = m.onboarding_step3_periods_generated_success();
      return;
    }

    const startYear = currentYear ? new Date(currentYear.start_date).getFullYear() : 2026;
    const endYear = currentYear ? new Date(currentYear.end_date).getFullYear() : 2027;

    const templates = scheme === "2_semesters" ? [
      { name: "First Semester", nameAr: "الفصل الدراسي الأول", type: "semester", start: `${startYear}-09-01`, end: `${startYear + 1}-01-20` },
      { name: "Second Semester", nameAr: "الفصل الدراسي الثاني", type: "semester", start: `${startYear + 1}-02-01`, end: `${endYear}-06-25` },
    ] : scheme === "3_trimesters" ? [
      { name: "First Trimester", nameAr: "الفصل الدراسي الأول", type: "trimester", start: `${startYear}-09-01`, end: `${startYear}-11-30` },
      { name: "Second Trimester", nameAr: "الفصل الدراسي الثاني", type: "trimester", start: `${startYear}-12-05`, end: `${startYear + 1}-03-05` },
      { name: "Third Trimester", nameAr: "الفصل الدراسي الثالث", type: "trimester", start: `${startYear + 1}-03-15`, end: `${endYear}-06-25` },
    ] : [
      { name: "Quarter 1", nameAr: "الربع الأول", type: "quarter", start: `${startYear}-09-01`, end: `${startYear}-11-10` },
      { name: "Quarter 2", nameAr: "الربع الثاني", type: "quarter", start: `${startYear}-11-15`, end: `${startYear + 1}-01-20` },
      { name: "Quarter 3", nameAr: "الربع الثالث", type: "quarter", start: `${startYear + 1}-02-01`, end: `${startYear + 1}-04-10` },
      { name: "Quarter 4", nameAr: "الربع الرابع", type: "quarter", start: `${startYear + 1}-04-15`, end: `${endYear}-06-25` },
    ];

    for (const t of templates) {
      const created = await api.createAcademicPeriod({
        year_id: selectedYearId,
        name: t.name,
        name_ar: t.nameAr,
        period_type: t.type,
        start_date: new Date(t.start).toISOString(),
        end_date: new Date(t.end).toISOString(),
      });
      periods = [...periods, created];
    }

    await systemStore.loadOnboardingState(orgId);
    successMessage = m.onboarding_step3_periods_generated_success();
  } catch (err: any) {
    console.error("Failed to generate periods:", err);
    errorMessage = err?.message || m.common_action_failed();
  } finally {
    isSubmittingPeriod = false;
  }
}

async function handleContinue() {
  if (years.length === 0 && yearName.trim()) {
    await handleCreateYear();
  }
  await navigateOnboardingRoute(`/${orgId}/onboarding/academics`);
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

  {#if errorMessage}
    <div class="rounded-lg border border-destructive/30 bg-destructive/10 p-4 text-sm text-destructive font-medium flex items-center justify-between">
      <span>{errorMessage}</span>
      <Button variant="ghost" size="sm" onclick={() => errorMessage = null}>✕</Button>
    </div>
  {/if}

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
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {m.onboarding_step3_year_name_en()}
              </Label>
              <Input
                bind:value={yearName}
                placeholder="2026/2027"
              />
            </div>
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {m.onboarding_step3_year_name_ar()}
              </Label>
              <Input
                bind:value={yearNameAr}
                placeholder={m.onboarding_step3_year_name_ar_placeholder()}
              />
            </div>
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {m.onboarding_step3_start_date()}
              </Label>
              <Input
                type="date"
                bind:value={yearStartDate}
              />
            </div>
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {m.onboarding_step3_end_date()}
              </Label>
              <Input
                type="date"
                bind:value={yearEndDate}
              />
            </div>
          </div>

          <div class="flex justify-end">
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
        <!-- Quick Preset Generator Buttons -->
        <div class="rounded-lg border border-primary/20 bg-primary/5 p-4 space-y-3">
          <div class="text-xs font-semibold text-foreground">
            ⚡ {m.onboarding_step3_quick_schemes()}
          </div>
          <p class="text-xs text-muted-foreground">
            {m.onboarding_step3_quick_schemes_desc()}
          </p>
          <div class="flex flex-wrap gap-2">
            <Button
              variant="outline"
              size="sm"
              disabled={isSubmittingPeriod || years.length === 0}
              onclick={() => handleGenerateSemesters("2_semesters")}
              class="text-xs bg-background hover:bg-muted"
            >
              {isSchool
                ? m.onboarding_step3_hijri_school_calendar()
                : m.onboarding_step3_2_semesters()}
            </Button>
            {#if !isSchool}
              <Button
                variant="outline"
                size="sm"
                disabled={isSubmittingPeriod || years.length === 0}
                onclick={() => handleGenerateSemesters("3_trimesters")}
                class="text-xs bg-background hover:bg-muted"
              >
                {m.onboarding_step3_3_trimesters()}
              </Button>
              <Button
                variant="outline"
                size="sm"
                disabled={isSubmittingPeriod || years.length === 0}
                onclick={() => handleGenerateSemesters("4_quarters")}
                class="text-xs bg-background hover:bg-muted"
              >
                {m.onboarding_step3_4_quarters()}
              </Button>
            {/if}
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
                  <Badge variant="secondary" class="text-[10px] uppercase">
                    {p.period_type}
                  </Badge>
                </div>
              {/each}
            </div>
          </div>
        {/if}
      </div>
    </OnboardingSection>

    <!-- Inline Actions Bar -->
    <div class="mt-8 flex items-center justify-end gap-3 pb-8 pt-4 border-t border-border/40">
      <Button
        variant="default"
        disabled={years.length === 0 && !yearName.trim()}
        onclick={handleContinue}
        class="text-xs font-semibold shadow-xs"
      >
        {m.onboarding_step3_continue_btn()}
      </Button>
    </div>
  {/if}
</div>
