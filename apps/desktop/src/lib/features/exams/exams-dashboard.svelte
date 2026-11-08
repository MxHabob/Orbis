<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Skeleton } from "@orbis/ui/skeleton";
import { Card } from "@orbis/ui/card";
import { ArrowRight, Calendar, ChartBar, DocumentCheck } from "@orbis/icons";
import { localizeHref } from "$lib/paraglide/runtime";
import { formatDate } from "$lib/core/i18n";
import { extractErrorMessage, type ExamSessionDto } from "$lib/api";
import { authStore, academicContext, uiStore } from "$lib/stores";
import { createPeriodsQuery } from "$lib/features/academics";
import { createExamSessionsQuery } from "$lib/features/exams/queries";
import PeriodSelect from "$lib/features/academics/components/period-select.svelte";
import ExamStatusBadge from "$lib/features/exams/components/status-badge.svelte";
import SessionCreateSheet from "$lib/features/exams/components/session-create-sheet.svelte";

let { orgId }: { orgId: string } = $props();

let selectedPeriodId = $state("");
let isCreateSheetOpen = $state(false);

const periodsQuery = createPeriodsQuery(() => orgId);
const periods = $derived(periodsQuery.data ?? []);
const activePeriod = $derived(
  periods.find(
    (period) =>
      period.status === "active" ||
      period.status === "active_teaching" ||
      period.status === "final_exams" ||
      period.status === "grade_submission",
  ) ?? periods[0],
);
const activePeriodId = $derived(
  selectedPeriodId || (activePeriod ? String(activePeriod.id) : ""),
);
const selectedPeriod = $derived(
  periods.find((period) => String(period.id) === activePeriodId),
);

const sessionsQuery = createExamSessionsQuery(() => activePeriodId);
const sessions = $derived<ExamSessionDto[]>(sessionsQuery.data ?? []);
const isLoading = $derived(periodsQuery.isLoading || sessionsQuery.isLoading);
const error = $derived(periodsQuery.error || sessionsQuery.error);
const preferenceScope = () => ({
  userId: authStore.userId,
  organizationId: orgId,
});

$effect(() => {
  if (periodsQuery.isLoading || periods.length === 0) return;

  const isAvailable = (periodId: string) =>
    periods.some((period) => String(period.id) === periodId);
  const rememberedPeriodId = uiStore.getPreference(
    preferenceScope(),
    "exams.selectedPeriodId",
    "",
    (value): value is string => typeof value === "string",
  );
  const nextPeriodId = isAvailable(selectedPeriodId)
    ? selectedPeriodId
    : isAvailable(rememberedPeriodId)
      ? rememberedPeriodId
      : isAvailable(academicContext.effectivePeriodId)
        ? academicContext.effectivePeriodId
        : activePeriodId;

  if (nextPeriodId !== selectedPeriodId) selectedPeriodId = nextPeriodId;
  if (nextPeriodId) {
    academicContext.selectedPeriodId = nextPeriodId;
    uiStore.setPreference(
      preferenceScope(),
      "exams.selectedPeriodId",
      nextPeriodId,
    );
  }
});
const isGradeEntryOpen = $derived(
  Boolean(selectedPeriod?.is_grade_entry_open && !selectedPeriod.is_locked),
);
const activeSessionsCount = $derived(
  sessions.filter(
    (session) => session.status === "active" || session.status === "running",
  ).length,
);
const visibleSessions = $derived(
  [...sessions]
    .sort((a, b) => a.start_date.localeCompare(b.start_date))
    .slice(0, 5),
);

const shortcuts = $derived([
  {
    href: `/${orgId}/academics/exams/sessions`,
    title: m.common_exams_sessions_title(),
    icon: Calendar,
  },
  {
    href: `/${orgId}/academics/exams/control`,
    title: m.academics_exams_center_control_title(),
    icon: ChartBar,
  },
  {
    href: `/${orgId}/academics/exams/reports`,
    title: m.academics_exams_center_reports_title(),
    icon: DocumentCheck,
  },
]);

function refetchAll() {
  periodsQuery.refetch();
  sessionsQuery.refetch();
}
</script>

<div class="flex h-full min-h-0 w-full flex-col gap-4 overflow-y-auto bg-background p-4 font-sans text-start select-none scrollbar-hide sm:p-6">
  <header class="flex shrink-0 flex-col gap-3 border-b border-border/40 pb-4 sm:flex-row sm:items-center sm:justify-between">
    <h1 class="text-base font-bold text-foreground">
      {m.academics_exams_center_title()}
    </h1>

    <div class="flex w-full flex-col gap-2 sm:w-auto sm:flex-row sm:items-center">
      <PeriodSelect
        value={activePeriodId || null}
        orgId={orgId}
        placeholder={m.common_metrics_period()}
        disabled={periodsQuery.isLoading}
        onchange={(periodId) => (selectedPeriodId = periodId || "")}
        class="w-full sm:min-w-52"
      />
      <Button
        variant="lime"
        class="h-9 shrink-0 px-4 text-xs font-semibold"
        onclick={() => (isCreateSheetOpen = true)}
      >
        {m.academics_exams_center_add_session()}
      </Button>
    </div>
  </header>

  {#if isLoading}
    <div class="grid shrink-0 grid-cols-2 divide-x divide-border/40 rounded-xl border border-border/50 bg-card sm:grid-cols-3">
      {#each Array(3) as _, index (index)}
        <div class="p-4">
          <Skeleton class="h-3 w-20" />
          <Skeleton class="mt-2 h-5 w-10" />
        </div>
      {/each}
    </div>
    <div class="grid min-h-80 flex-1 grid-cols-1 gap-4 lg:grid-cols-[minmax(0,1.7fr)_minmax(15rem,0.8fr)]">
      <Skeleton class="h-full min-h-64 w-full rounded-xl" />
      <Skeleton class="h-full min-h-64 w-full rounded-xl" />
    </div>
  {:else if error}
    <Card class="flex min-h-0 flex-1 flex-col items-center justify-center gap-3 p-8 text-center">
      <p class="text-sm font-semibold text-destructive">
        {m.academics_exams_center_error_title()}
      </p>
      <p class="max-w-xl text-xs text-muted-foreground">
        {extractErrorMessage(error)}
      </p>
      <Button variant="outline" class="h-9 px-4 text-xs" onclick={refetchAll}>
        {m.academics_exams_center_retry()}
      </Button>
    </Card>
  {:else}
    <dl class="grid shrink-0 grid-cols-2 divide-x divide-border/40 rounded-xl border border-border/50 bg-card sm:grid-cols-3">
      <div class="p-3 sm:px-5 sm:py-4">
        <dt class="text-[11px] text-muted-foreground">
          {m.academics_exams_center_kpi_sessions()}
        </dt>
        <dd class="mt-1 text-lg font-semibold leading-none text-foreground">
          {sessions.length}
        </dd>
      </div>
      <div class="border-t border-border/40 p-3 sm:border-t-0 sm:px-5 sm:py-4">
        <dt class="text-[11px] text-muted-foreground">
          {m.academics_exams_center_kpi_active()}
        </dt>
        <dd class="mt-1 text-lg font-semibold leading-none text-foreground">
          {activeSessionsCount}
        </dd>
      </div>
      <div class="col-span-2 border-t border-border/40 p-3 sm:col-span-1 sm:border-t-0 sm:px-5 sm:py-4">
        <dt class="text-[11px] text-muted-foreground">
          {m.academics_exams_center_kpi_grades()}
        </dt>
        <dd class="mt-1 flex items-center gap-2 text-sm font-medium leading-none text-foreground">
          <span class="h-2 w-2 rounded-full {isGradeEntryOpen ? 'bg-emerald-500' : 'bg-muted-foreground/50'}"></span>
          {selectedPeriod
            ? isGradeEntryOpen
              ? m.academics_exams_center_open()
              : m.academics_exams_center_locked()
            : m.academics_exams_center_no_active_year()}
        </dd>
      </div>
    </dl>

    <div class="grid min-h-0 flex-1 grid-cols-1 gap-4 lg:grid-cols-[minmax(0,1.7fr)_minmax(15rem,0.8fr)]">
      <Card class="flex min-h-64 min-w-0 flex-col overflow-hidden">
        <div class="flex shrink-0 items-center justify-between gap-3 border-b border-border/40 px-4 py-3">
          <h2 class="text-sm font-semibold text-foreground">
            {m.common_exams_sessions_title()}
          </h2>
          <Button
            href={localizeHref(`/${orgId}/academics/exams/sessions`)}
            variant="ghost"
            class="h-8 shrink-0 px-2 text-xs font-semibold text-primary"
          >
            {m.common_action_view_all()}
            <ArrowRight size={14} class="ms-1 rtl:rotate-180" />
          </Button>
        </div>

        {#if visibleSessions.length === 0}
          <div class="flex flex-1 flex-col items-center justify-center gap-3 p-6 text-center">
            <p class="text-xs text-muted-foreground">
              {m.academics_exams_center_sessions_empty_insight()}
            </p>
            <Button
              variant="outline"
              class="h-8 px-3 text-xs"
              onclick={() => (isCreateSheetOpen = true)}
            >
              {m.academics_exams_center_add_session()}
            </Button>
          </div>
        {:else}
          <div class="min-h-0 flex-1 divide-y divide-border/40 overflow-y-auto scrollbar-hide">
            {#each visibleSessions as session (session.id)}
              <a
                href={localizeHref(`/${orgId}/academics/exams/sessions/${session.id}`)}
                class="group flex min-w-0 items-center justify-between gap-3 px-4 py-3 transition-colors hover:bg-muted/30"
              >
                <span class="flex min-w-0 items-center gap-3">
                  <span class="flex h-8 w-8 shrink-0 items-center justify-center rounded-lg bg-muted/50 text-muted-foreground">
                    <Calendar size={15} />
                  </span>
                  <span class="min-w-0">
                    <span class="block truncate text-xs font-medium text-foreground group-hover:text-primary">
                      {session.name_ar || session.name}
                    </span>
                    <span class="mt-1 block truncate text-[10px] text-muted-foreground">
                      {formatDate(session.start_date, "date")}
                      {#if session.end_date !== session.start_date}
                        — {formatDate(session.end_date, "date")}
                      {/if}
                    </span>
                  </span>
                </span>
                <span class="flex shrink-0 items-center gap-2">
                  <ExamStatusBadge status={session.status} size="xs" />
                  <ArrowRight size={14} class="text-muted-foreground/50 rtl:rotate-180" />
                </span>
              </a>
            {/each}
          </div>
        {/if}
      </Card>

      <Card class="flex min-h-64 min-w-0 flex-col overflow-hidden">
        <h2 class="border-b border-border/40 px-4 py-3 text-sm font-semibold text-foreground">
          {m.academics_exams_center_directory_title()}
        </h2>
        <nav class="divide-y divide-border/40" aria-label={m.academics_exams_center_directory_title()}>
          {#each shortcuts as item (item.href)}
            <a
              href={localizeHref(item.href)}
              class="group flex items-center gap-3 px-4 py-3 transition-colors hover:bg-muted/30"
            >
              <span class="flex h-8 w-8 shrink-0 items-center justify-center rounded-lg bg-muted/50 text-muted-foreground group-hover:text-primary">
                <item.icon size={15} />
              </span>
              <span class="min-w-0 flex-1 truncate text-xs font-medium text-foreground group-hover:text-primary">
                {item.title}
              </span>
              <ArrowRight size={14} class="shrink-0 text-muted-foreground/50 rtl:rotate-180" />
            </a>
          {/each}
        </nav>
      </Card>
    </div>
  {/if}
</div>

<SessionCreateSheet
  bind:open={isCreateSheetOpen}
  {orgId}
  onSuccess={refetchAll}
/>
