<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import { onMount } from "svelte";
import * as m from "$lib/paraglide/messages";
import { toast } from "@orbis/ui";
import { api, type academicCalendarDto, type AcademicPeriodDto } from "$lib/api";
import { orgContext, systemStore } from "$lib/stores";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Badge } from "@orbis/ui/badge";
import { Textarea } from "@orbis/ui/textarea";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
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
let years = $state<academicCalendarDto[]>([]);
let periods = $state<AcademicPeriodDto[]>([]);

// Academic Year Form
let yearName = $state("");
let yearNameAr = $state("");
let yearStartDate = $state("");
let yearEndDate = $state("");
let editingYearId = $state<string | null>(null);
let editingYearName = $state("");
let editingYearNameAr = $state("");
let editingYearStartDate = $state("");
let editingYearEndDate = $state("");
let editingYearNotes = $state("");

// Selected Year for Period Creation
let selectedYearId = $state<string>("");
let periodName = $state("");
let periodNameAr = $state("");
let periodCode = $state("");
let periodStartDate = $state("");
let periodEndDate = $state("");
let periodType = $state("custom");
let periodKind = $state("teaching");
let periodParentId = $state("");
let editingPeriodId = $state<string | null>(null);
let editingPeriodName = $state("");
let editingPeriodNameAr = $state("");
let editingPeriodCode = $state("");
let editingPeriodStartDate = $state("");
let editingPeriodEndDate = $state("");
let editingPeriodKind = $state("teaching");
let isUpdatingRecord = $state(false);

const stepTasks = $derived(getTasksForStep("calendar", systemStore.onboardingTasks));
const isTaskComplete = $derived(stepTasks.length > 0 && stepTasks.every((t) => t.status === "completed"));
const selectedYear = $derived(years.find((year) => String(year.id) === selectedYearId));
const organizationToday = $derived(getOrganizationDate(new Date()));
const yearStartIsPast = $derived(!!yearStartDate && yearStartDate.slice(0, 10) < organizationToday);

function toDateTimeInput(value: string) {
  if (!value) return "";
  const date = new Date(value);
  if (Number.isNaN(date.getTime())) return "";
  const localDate = new Date(date.getTime() - date.getTimezoneOffset() * 60_000);
  return localDate.toISOString().slice(0, 16);
}

function toApiDateTime(value: string) {
  const date = new Date(value);
  if (Number.isNaN(date.getTime())) {
    throw new Error(m.onboarding_step3_year_err_dates());
  }
  return date.toISOString();
}

function getDateParts(date: Date, timeZone: string, calendar = "gregory") {
  const formatter = new Intl.DateTimeFormat(`en-US-u-ca-${calendar}`, {
    year: "numeric",
    month: "numeric",
    day: "numeric",
    timeZone,
  });
  const values = Object.fromEntries(
    formatter.formatToParts(date).map(({ type, value }) => [type, value]),
  );
  return {
    year: Number(values.year),
    month: Number(values.month),
    day: Number(values.day),
  };
}

function dateString({ year, month, day }: { year: number; month: number; day: number }) {
  return `${year}-${String(month).padStart(2, "0")}-${String(day).padStart(2, "0")}`;
}

function getOrganizationDate(date: Date) {
  return dateString(getDateParts(date, orgContext.timezone));
}

function shiftDate(date: string, days: number) {
  const timestamp = Date.parse(`${date}T12:00:00.000Z`) + days * 24 * 60 * 60 * 1000;
  return dateString(getDateParts(new Date(timestamp), "UTC"));
}

function findHijriDate(year: number, month: number, day: number, aroundDate: string) {
  const hijriFormatter = new Intl.DateTimeFormat("en-US-u-ca-islamic-umalqura", {
    year: "numeric",
    month: "numeric",
    day: "numeric",
    timeZone: "UTC",
  });
  if (hijriFormatter.resolvedOptions().calendar !== "islamic-umalqura") {
    throw new Error(m.onboarding_step3_default_periods_calendar_error());
  }

  const target = `${year}-${month}-${day}`;
  for (let offset = -400; offset <= 400; offset += 1) {
    const date = shiftDate(aroundDate, offset);
    const timestamp = Date.parse(`${date}T12:00:00.000Z`);
    const parts = Object.fromEntries(
      hijriFormatter.formatToParts(new Date(timestamp)).map(({ type, value }) => [type, value]),
    );
    if (`${Number(parts.year)}-${Number(parts.month)}-${Number(parts.day)}` === target) {
      return date;
    }
  }
  throw new Error(m.onboarding_step3_default_periods_calendar_error());
}

function formatGregorianYear(date: string) {
  return String(getDateParts(new Date(`${date}T12:00:00.000Z`), "UTC").year);
}

function suggestacademicCalendar() {
  const now = new Date();
  const today = getOrganizationDate(now);
  let startDate: string;
  let endDate: string;
  let name: string;
  let nameAr: string;

  if (orgContext.isSchool) {
    const hijriToday = getDateParts(
      new Date(`${today}T12:00:00.000Z`),
      "UTC",
      "islamic-umalqura",
    );
    const hijriYear = hijriToday.year + (hijriToday.month > 8 ? 1 : 0);
    startDate = findHijriDate(hijriYear, 1, 1, today);
    endDate = shiftDate(findHijriDate(hijriYear, 9, 1, today), -1);
    const startGregorianYear = formatGregorianYear(startDate);
    const endGregorianYear = formatGregorianYear(endDate);
    name = `${hijriYear}-${hijriYear + 1} AH / ${startGregorianYear}-${endGregorianYear}`;
    nameAr = `${hijriYear}-${hijriYear + 1} هـ / ${startGregorianYear}-${endGregorianYear} م`;
  } else if (orgContext.isUniversity) {
    const currentYear = getDateParts(now, orgContext.timezone).year;
    const currentMonth = getDateParts(now, orgContext.timezone).month;
    const startYear = currentMonth >= 9 ? currentYear : currentYear - 1;
    const endYear = startYear + 1;
    startDate = `${startYear}-09-01`;
    endDate = `${endYear}-08-31`;
    name = `${startYear}/${endYear}`;
    nameAr = `${startYear}/${endYear} م`;
  } else {
    const gregorianYear = getDateParts(now, orgContext.timezone).year;
    startDate = today;
    const [year, month, day] = today.split("-").map(Number);
    const nextYear = year + 1;
    const lastDayOfTargetMonth = new Date(Date.UTC(nextYear, month, 0)).getUTCDate();
    const sameDateNextYear = dateString({
      year: nextYear,
      month,
      day: Math.min(day, lastDayOfTargetMonth),
    });
    endDate = shiftDate(sameDateNextYear, -1);
    name = `Training Cycle ${gregorianYear}`;
    nameAr = `الدورة التدريبية ${gregorianYear}`;
  }

  yearName = name;
  yearNameAr = nameAr;
  yearStartDate = `${startDate}T00:00`;
  yearEndDate = `${endDate}T23:59`;
  successMessage = null;
}

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

  const monthLengths: number[] = [];
  for (let index = 0; index < 8; index += 1) {
    const currentMonth = getParts(new Date(cursorTime));
    const targetYear = currentMonth.year;
    const targetMonth = currentMonth.month;
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
    if (days < (index === 0 ? 1 : 29) || days > 30) {
      throw new Error(m.onboarding_step3_default_periods_calendar_error());
    }
    monthLengths.push(days);
  }

  return {
    hijriStartYear: start.year,
    hijriStartMonth: start.month,
    hijriStartDay: start.day,
    monthLengths,
  };
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
      api.listacademicCalendars(),
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
    const newYear = await api.createacademicCalendar({
      name: yearName.trim() || yearNameAr.trim(),
      name_ar: yearNameAr.trim() || undefined,
      start_date: toApiDateTime(yearStartDate),
      end_date: toApiDateTime(yearEndDate),
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
      code: periodCode.trim() || undefined,
      start_date: toApiDateTime(periodStartDate),
      end_date: toApiDateTime(periodEndDate),
    });
    periods = [...periods, created];
    periodName = "";
    periodNameAr = "";
    periodCode = "";
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
        calendar_id: String(selectedYear.id),
        provision_hijri_school_calendar: true,
        hijri_start_year: calendarDefaults.hijriStartYear,
        hijri_start_month: calendarDefaults.hijriStartMonth,
        hijri_start_day: calendarDefaults.hijriStartDay,
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

let isApplyingTemplate = $state(false);

async function handleApplyOfficialTemplate(templateCode: string) {
  if (!selectedYear) {
    toast.error(m.onboarding_step3_period_err_year());
    return;
  }

  isApplyingTemplate = true;
  successMessage = null;
  try {
    await api.applyCalendarTemplate(String(selectedYear.id), {
      template_code: templateCode,
    });
    periods = await api.listAcademicPeriods();
    await refreshOnboardingState(orgId, m.common_action_failed());
    successMessage = "Official calendar template applied successfully.";
  } catch (err: unknown) {
    console.error("Failed to apply official calendar template:", err);
    toast.error(err instanceof Error ? err.message : m.common_action_failed());
  } finally {
    isApplyingTemplate = false;
  }
}

function startYearEdit(year: academicCalendarDto) {
  editingYearId = String(year.id);
  editingYearName = year.name;
  editingYearNameAr = year.name_ar ?? "";
  editingYearStartDate = toDateTimeInput(year.start_date);
  editingYearEndDate = toDateTimeInput(year.end_date);
  editingYearNotes = year.notes ?? "";
}

function cancelYearEdit() {
  editingYearId = null;
}

async function saveYearEdit(year: academicCalendarDto) {
  if (!editingYearName.trim() && !editingYearNameAr.trim()) {
    toast.error(m.onboarding_step3_year_err_name());
    return;
  }
  if (!validateDateRange(editingYearStartDate, editingYearEndDate, m.onboarding_step3_year_err_dates())) {
    return;
  }

  isUpdatingRecord = true;
  try {
    const updated = await api.updateacademicCalendar(String(year.id), {
      name: editingYearName.trim() || editingYearNameAr.trim(),
      name_ar: editingYearNameAr.trim() || null,
      start_date: toApiDateTime(editingYearStartDate),
      end_date: toApiDateTime(editingYearEndDate),
      notes: editingYearNotes.trim() || null,
    });
    years = years.map((item) => item.id === updated.id ? updated : item);
    editingYearId = null;
    await refreshOnboardingState(orgId, m.common_action_failed());
  } catch (err: unknown) {
    console.error("Failed to update academic year:", err);
    toast.error(err instanceof Error ? err.message : m.common_action_failed());
  } finally {
    isUpdatingRecord = false;
  }
}

async function transitionYear(year: academicCalendarDto, action: "activate" | "close" | "archive") {
  isSubmittingYear = true;
  try {
    const updated = action === "activate"
      ? await api.activateacademicCalendar(String(year.id))
      : action === "close"
        ? await api.closeacademicCalendar(String(year.id))
        : await api.archiveacademicCalendar(String(year.id));
    years = years.map((item) => item.id === updated.id ? updated : item);
    await refreshOnboardingState(orgId, m.common_action_failed());
  } catch (err: unknown) {
    console.error(`Failed to ${action} academic year:`, err);
    toast.error(err instanceof Error ? err.message : m.common_action_failed());
  } finally {
    isSubmittingYear = false;
  }
}

function startPeriodEdit(period: AcademicPeriodDto) {
  editingPeriodId = String(period.id);
  editingPeriodName = period.name;
  editingPeriodNameAr = period.name_ar ?? "";
  editingPeriodCode = period.code ?? "";
  editingPeriodStartDate = toDateTimeInput(period.start_date);
  editingPeriodEndDate = toDateTimeInput(period.end_date);
  editingPeriodKind = period.period_kind;
}

function cancelPeriodEdit() {
  editingPeriodId = null;
}

async function savePeriodEdit(period: AcademicPeriodDto) {
  if (!editingPeriodName.trim() && !editingPeriodNameAr.trim()) {
    toast.error(m.onboarding_step3_year_err_name());
    return;
  }
  if (!validateDateRange(editingPeriodStartDate, editingPeriodEndDate, m.onboarding_step3_year_err_dates())) {
    return;
  }

  isUpdatingRecord = true;
  try {
    const updated = await api.updateAcademicPeriod(String(period.id), {
      name: editingPeriodName.trim() || editingPeriodNameAr.trim(),
      name_ar: editingPeriodNameAr.trim() || null,
      code: editingPeriodCode.trim() || null,
      start_date: toApiDateTime(editingPeriodStartDate),
      end_date: toApiDateTime(editingPeriodEndDate),
      period_kind: editingPeriodKind,
    });
    periods = periods.map((item) => item.id === updated.id ? updated : item);
    editingPeriodId = null;
    await refreshOnboardingState(orgId, m.common_action_failed());
  } catch (err: unknown) {
    console.error("Failed to update academic period:", err);
    toast.error(err instanceof Error ? err.message : m.common_action_failed());
  } finally {
    isUpdatingRecord = false;
  }
}

async function transitionPeriod(period: AcademicPeriodDto, action: "lock" | "unlock" | "close") {
  isSubmittingPeriod = true;
  try {
    const updated = action === "lock"
      ? await api.lockAcademicPeriod(String(period.id))
      : action === "unlock"
        ? await api.unlockAcademicPeriod(String(period.id))
        : await api.closeAcademicPeriod(String(period.id));
    periods = periods.map((item) => item.id === updated.id ? updated : item);
    await refreshOnboardingState(orgId, m.common_action_failed());
  } catch (err: unknown) {
    console.error(`Failed to ${action} academic period:`, err);
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

async function deleteYear(year: academicCalendarDto) {
  const deleted = await api.deleteacademicCalendar(String(year.id));
  if (!deleted) throw new Error(m.common_action_failed());
  years = years.filter((item) => item.id !== year.id);
  if (selectedYearId === String(year.id)) {
    selectedYearId = years[0] ? String(years[0].id) : "";
  }
  await refreshOnboardingState(orgId, m.common_action_failed());
  successMessage = m.onboarding_step3_year_deleted_success();
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
                type="datetime-local"
                bind:value={yearStartDate}
              />
            </div>
            <div class="space-y-1.5">
              <Label for="onboarding-year-end" class="text-xs font-medium">
                {m.onboarding_step3_end_date()}
              </Label>
              <Input
                id="onboarding-year-end"
                type="datetime-local"
                bind:value={yearEndDate}
              />
            </div>
          </div>

          <p class="text-xs text-muted-foreground">
            {m.onboarding_step3_suggest_year_desc()}
          </p>

          {#if yearStartIsPast}
            <div
              role="alert"
              class="rounded-md border border-amber-500/40 bg-amber-500/10 px-3 py-2 text-xs text-amber-900 dark:text-amber-200"
            >
              {m.onboarding_step3_year_start_past_notice({ startDate: yearStartDate })}
            </div>
          {/if}

          <div class="flex flex-wrap justify-between gap-2">
            <Button
              type="button"
              variant="outline"
              size="sm"
              disabled={isSubmittingYear}
              onclick={suggestacademicCalendar}
              class="text-xs"
            >
              {m.onboarding_step3_suggest_year_btn()}
            </Button>
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
                <div class="space-y-3 p-3 text-xs">
                  <div class="flex items-center justify-between gap-3">
                    <div class="flex items-center gap-3">
                      <span class="flex size-7 items-center justify-center rounded-md bg-primary/10 font-bold text-primary text-[11px]">
                        📅
                      </span>
                      <div>
                        <div class="font-medium text-foreground">
                          {yr.name_ar || yr.name}
                        </div>
                        <div class="text-[11px] font-mono text-muted-foreground">
                          {yr.start_date.replace("T", " ").slice(0, 16)} → {yr.end_date.replace("T", " ").slice(0, 16)}
                        </div>
                      </div>
                    </div>
                    <div class="flex flex-wrap items-center justify-end gap-2">
                      <Badge variant="outline" class={yr.is_active
                        ? "border-emerald-500/30 text-emerald-600 dark:text-emerald-400 text-[10px]"
                        : "text-[10px]"}>
                        {yr.status}
                      </Badge>
                      <Button variant="outline" size="sm" disabled={isUpdatingRecord || isSubmittingYear} onclick={() => startYearEdit(yr)}>
                        {m.common_action_edit()}
                      </Button>
                      {#if !yr.is_active && yr.status !== "archived"}
                        <Button variant="outline" size="sm" disabled={isSubmittingYear} onclick={() => transitionYear(yr, "activate")}>
                          {m.onboarding_step3_activate()}
                        </Button>
                      {/if}
                      {#if yr.status !== "closed" && yr.status !== "archived"}
                        <Button variant="outline" size="sm" disabled={isSubmittingYear} onclick={() => transitionYear(yr, "close")}>
                          {m.common_action_close()}
                        </Button>
                      {/if}
                      {#if yr.status !== "archived"}
                        <Button variant="outline" size="sm" disabled={isSubmittingYear} onclick={() => transitionYear(yr, "archive")}>
                          {m.onboarding_step3_archive()}
                        </Button>
                      {/if}
                      <DeleteRecordButton
                        label={yr.name_ar || yr.name}
                        onDelete={() => deleteYear(yr)}
                      />
                    </div>
                  </div>
                  {#if editingYearId === String(yr.id)}
                    <div class="grid gap-3 rounded-md border border-border/50 bg-muted/20 p-3 sm:grid-cols-2">
                      <div class="sm:col-span-2">
                        <BilingualNameFields
                          arId="edit-year-name-ar"
                          enId="edit-year-name-en"
                          arLabel={m.onboarding_step3_year_name_ar()}
                          enLabel={m.onboarding_step3_year_name_en()}
                          bind:arValue={editingYearNameAr}
                          bind:enValue={editingYearName}
                          kind="label"
                          requestSuggestion={suggestLocalTranslation}
                          disabled={isUpdatingRecord}
                          loadingMessage={m.bilingual_entry_loading()}
                          failureMessage={m.bilingual_entry_failure()}
                        />
                      </div>
                      <div class="space-y-1.5">
                        <Label for={`edit-year-start-${yr.id}`} class="text-xs">{m.onboarding_step3_start_date()}</Label>
                        <Input id={`edit-year-start-${yr.id}`} type="datetime-local" bind:value={editingYearStartDate} />
                      </div>
                      <div class="space-y-1.5">
                        <Label for={`edit-year-end-${yr.id}`} class="text-xs">{m.onboarding_step3_end_date()}</Label>
                        <Input id={`edit-year-end-${yr.id}`} type="datetime-local" bind:value={editingYearEndDate} />
                      </div>
                      <div class="space-y-1.5 sm:col-span-2">
                        <Label for={`edit-year-notes-${yr.id}`} class="text-xs">{m.onboarding_step3_notes()}</Label>
                        <Textarea id={`edit-year-notes-${yr.id}`} bind:value={editingYearNotes} rows={3} disabled={isUpdatingRecord} />
                      </div>
                      <div class="flex justify-end gap-2 sm:col-span-2">
                        <Button variant="outline" size="sm" disabled={isUpdatingRecord} onclick={cancelYearEdit}>{m.common_action_cancel()}</Button>
                        <Button size="sm" disabled={isUpdatingRecord} onclick={() => saveYearEdit(yr)}>{m.common_save_changes()}</Button>
                      </div>
                    </div>
                  {/if}
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

          <!-- Official Calendar Template -->
          {#if periods.length > 0}
            <div class="flex flex-wrap items-center justify-between gap-3 rounded-lg border border-accent/30 bg-accent/5 p-4">
              <div class="space-y-1">
                <p class="text-xs font-semibold text-foreground">
                  🇾🇪 Official Yemen School Calendar (1448 AH / 2026-2027)
                </p>
                <p class="text-xs text-muted-foreground">
                  Apply supplementary periods from Ministerial Decision No. 100:
                  admin opening, teacher prep, mid-year break, and public examination windows.
                </p>
              </div>
              <Button
                type="button"
                variant="outline"
                size="sm"
                disabled={isApplyingTemplate || !selectedYearId}
                onclick={() => handleApplyOfficialTemplate("YE-1447-1448")}
                class="text-xs font-semibold"
              >
                {isApplyingTemplate ? "Applying…" : "Apply Official Calendar"}
              </Button>
            </div>
          {/if}
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
              <Label for="onboarding-period-code" class="text-xs font-medium">
                {m.onboarding_step3_code()}
              </Label>
              <Input id="onboarding-period-code" bind:value={periodCode} disabled={isSubmittingPeriod} />
            </div>
            <div class="space-y-1.5">
              <Label for="onboarding-period-type" class="text-xs font-medium">
                {m.onboarding_academic_period_type()}
              </Label>
              <Select
                type="single"
                value={periodType}
                onValueChange={(value) => {
                  if (value) periodType = value;
                }}
                disabled={isSubmittingPeriod}
              >
                <SelectTrigger id="onboarding-period-type" class="h-10 w-full">
                  <span>{periodType}</span>
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="custom">Custom</SelectItem>
                  <SelectItem value="semester">Semester</SelectItem>
                  <SelectItem value="trimester">Trimester</SelectItem>
                  <SelectItem value="quarter">Quarter</SelectItem>
                  <SelectItem value="month">Month</SelectItem>
                  <SelectItem value="module">Module</SelectItem>
                  <SelectItem value="block">Block</SelectItem>
                </SelectContent>
              </Select>
            </div>
            <div class="space-y-1.5">
              <Label for="onboarding-period-kind" class="text-xs font-medium">
                {m.onboarding_academic_period_role()}
              </Label>
              <Select
                type="single"
                value={periodKind}
                onValueChange={(value) => {
                  if (value) periodKind = value;
                }}
                disabled={isSubmittingPeriod}
              >
                <SelectTrigger id="onboarding-period-kind" class="h-10 w-full">
                  <span>{periodKind}</span>
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="teaching">Teaching</SelectItem>
                  <SelectItem value="examination">Examination</SelectItem>
                  <SelectItem value="break">Break</SelectItem>
                  <SelectItem value="registration">Registration</SelectItem>
                  <SelectItem value="preparatory">Preparatory</SelectItem>
                </SelectContent>
              </Select>
            </div>
            <div class="space-y-1.5">
              <Label for="onboarding-period-start" class="text-xs font-medium">
                {m.onboarding_step3_start_date()}
              </Label>
              <Input id="onboarding-period-start" type="datetime-local" bind:value={periodStartDate} />
            </div>
            <div class="space-y-1.5">
              <Label for="onboarding-period-end" class="text-xs font-medium">
                {m.onboarding_step3_end_date()}
              </Label>
              <Input id="onboarding-period-end" type="datetime-local" bind:value={periodEndDate} />
            </div>
            {#if periods.some((period) => period.year_id === selectedYearId)}
              <div class="space-y-1.5 sm:col-span-2">
                <Label for="onboarding-period-parent" class="text-xs font-medium">
                  {m.onboarding_academic_period_parent_optional()}
                </Label>
                <Select
                  type="single"
                  value={periodParentId || "none"}
                  onValueChange={(value) => {
                    if (value) periodParentId = value === "none" ? "" : value;
                  }}
                  disabled={isSubmittingPeriod}
                >
                  <SelectTrigger id="onboarding-period-parent" class="h-10 w-full">
                    <span>
                      {periodParentId
                        ? periods.find((period) => String(period.id) === periodParentId)
                            ?.name_ar ||
                          periods.find((period) => String(period.id) === periodParentId)?.name
                        : "No parent"}
                    </span>
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="none">No parent</SelectItem>
                    {#each periods.filter((period) => period.year_id === selectedYearId) as period (period.id)}
                      <SelectItem value={String(period.id)}>
                        {period.name_ar || period.name}
                      </SelectItem>
                    {/each}
                  </SelectContent>
                </Select>
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
                <div class="space-y-3 rounded-lg border border-border/50 bg-card p-3 text-xs">
                  <div class="flex flex-wrap items-center justify-between gap-3">
                    <div class="flex items-center gap-2">
                      <span class="flex size-6 items-center justify-center rounded bg-primary/10 text-primary text-[10px] font-bold">
                        📑
                      </span>
                      <div>
                        <div class="font-medium text-foreground">
                          {p.name_ar || p.name}
                        </div>
                        <div class="text-[10px] text-muted-foreground">
                          {years.find((year) => String(year.id) === p.year_id)?.name_ar ||
                            years.find((year) => String(year.id) === p.year_id)?.name}
                          ·
                          {p.start_date.replace("T", " ").slice(0, 16)} → {p.end_date.replace("T", " ").slice(0, 16)}
                          {#if p.parent_id}
                            · {periods.find((parent) => String(parent.id) === p.parent_id)?.name_ar ||
                              periods.find((parent) => String(parent.id) === p.parent_id)?.name}
                          {/if}
                          {#if p.code} · {p.code}{/if}
                        </div>
                      </div>
                    </div>
                    <div class="flex flex-wrap items-center justify-end gap-2">
                      <Badge variant="secondary" class="text-[10px] uppercase">{p.period_type}</Badge>
                      <Badge variant="outline" class="text-[10px] uppercase">{p.period_kind}</Badge>
                      <Badge variant={p.is_locked ? "destructive" : "outline"} class="text-[10px]">
                        {p.is_locked ? m.onboarding_step3_locked() : p.status}
                      </Badge>
                      <Badge variant="outline" class="text-[10px]">
                        {p.is_enrollment_open ? m.onboarding_step3_enrollment_open() : m.onboarding_step3_enrollment_closed()}
                      </Badge>
                      <Badge variant="outline" class="text-[10px]">
                        {p.is_grade_entry_open ? m.onboarding_step3_grade_entry_open() : m.onboarding_step3_grade_entry_closed()}
                      </Badge>
                      <Button variant="outline" size="sm" disabled={isUpdatingRecord || p.is_locked} onclick={() => startPeriodEdit(p)}>
                        {m.common_action_edit()}
                      </Button>
                      {#if p.is_locked}
                        <Button variant="outline" size="sm" disabled={isSubmittingPeriod} onclick={() => transitionPeriod(p, "unlock")}>
                          {m.onboarding_step3_unlock()}
                        </Button>
                      {:else}
                        <Button variant="outline" size="sm" disabled={isSubmittingPeriod} onclick={() => transitionPeriod(p, "close")}>
                          {m.common_action_close()}
                        </Button>
                      {/if}
                      <DeleteRecordButton label={p.name_ar || p.name} onDelete={() => deletePeriod(p)} />
                    </div>
                  </div>
                  {#if editingPeriodId === String(p.id)}
                    <div class="grid gap-3 rounded-md border border-border/50 bg-muted/20 p-3 sm:grid-cols-2">
                      <div class="sm:col-span-2">
                        <BilingualNameFields
                          arId="edit-period-name-ar"
                          enId="edit-period-name-en"
                          arLabel={m.onboarding_step3_year_name_ar()}
                          enLabel={m.onboarding_step3_year_name_en()}
                          bind:arValue={editingPeriodNameAr}
                          bind:enValue={editingPeriodName}
                          kind="label"
                          requestSuggestion={suggestLocalTranslation}
                          disabled={isUpdatingRecord}
                          loadingMessage={m.bilingual_entry_loading()}
                          failureMessage={m.bilingual_entry_failure()}
                        />
                      </div>
                      <div class="space-y-1.5">
                        <Label for={`edit-period-code-${p.id}`} class="text-xs">{m.onboarding_step3_code()}</Label>
                        <Input id={`edit-period-code-${p.id}`} bind:value={editingPeriodCode} disabled={isUpdatingRecord} />
                      </div>
                      <div class="space-y-1.5">
                        <Label for={`edit-period-kind-${p.id}`} class="text-xs">{m.onboarding_step3_period_kind()}</Label>
                        <Select
                          type="single"
                          value={editingPeriodKind}
                          onValueChange={(value) => { if (value) editingPeriodKind = value; }}
                          disabled={isUpdatingRecord}
                        >
                          <SelectTrigger id={`edit-period-kind-${p.id}`} class="h-10 w-full"><span>{editingPeriodKind}</span></SelectTrigger>
                          <SelectContent>
                            <SelectItem value="teaching">Teaching</SelectItem>
                            <SelectItem value="examination">Examination</SelectItem>
                            <SelectItem value="break">Break</SelectItem>
                            <SelectItem value="registration">Registration</SelectItem>
                            <SelectItem value="preparatory">Preparatory</SelectItem>
                          </SelectContent>
                        </Select>
                      </div>
                      <div class="space-y-1.5">
                        <Label for={`edit-period-start-${p.id}`} class="text-xs">{m.onboarding_step3_start_date()}</Label>
                        <Input id={`edit-period-start-${p.id}`} type="datetime-local" bind:value={editingPeriodStartDate} disabled={isUpdatingRecord} />
                      </div>
                      <div class="space-y-1.5">
                        <Label for={`edit-period-end-${p.id}`} class="text-xs">{m.onboarding_step3_end_date()}</Label>
                        <Input id={`edit-period-end-${p.id}`} type="datetime-local" bind:value={editingPeriodEndDate} disabled={isUpdatingRecord} />
                      </div>
                      <div class="flex justify-end gap-2 sm:col-span-2">
                        <Button variant="outline" size="sm" disabled={isUpdatingRecord} onclick={cancelPeriodEdit}>{m.common_action_cancel()}</Button>
                        <Button size="sm" disabled={isUpdatingRecord} onclick={() => savePeriodEdit(p)}>{m.common_save_changes()}</Button>
                      </div>
                    </div>
                  {/if}
                </div>
              {/each}
            </div>
          </div>
        {/if}
      </div>
    </OnboardingSection>

  {/if}
</div>
