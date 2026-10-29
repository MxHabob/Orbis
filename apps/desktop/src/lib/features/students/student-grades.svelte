<script lang="ts">
import { api, extractErrorMessage, type GradeDto } from "$lib/api";
import * as m from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";
import PdfReportViewer from "$lib/shared/components/pdf-report-viewer.svelte";
import { academicContext } from "$lib/stores";
import { formatDate, formatDecimal, formatNumber, formatPercent } from "$lib/core/i18n";
import {
  AcademicCap,
  BookOpen,
  ChartBar,
  DocumentChartBar,
  InfoCircle,
  Printer,
  Refresh,
} from "@orbis/icons";
import { OrbisGrid, type OrbisColumnDef } from "@orbis/svelte-grid";
import { LoadingIndicator } from "@orbis/ui";
import { Button } from "@orbis/ui/button";
import {
  Card,
  CardAction,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@orbis/ui/card";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui/select";
import { createQuery } from "@tanstack/svelte-query";
import StudentMetricCard from "$lib/features/students/components/student-metric-card.svelte";
import StudentSectionState from "$lib/features/students/components/student-section-state.svelte";

let {
  orgId,
  studentId,
  classId,
}: {
  orgId: string;
  studentId: string;
  classId?: string;
} = $props();

const isRtl = $derived(getLocale() === "ar");

// View mode: "records" | "term" | "annual" | "certificate"
let activeTab = $state<"records" | "term" | "annual" | "certificate">(
  "records",
);

// Selection states
let selectedPeriodId = $state("all");
let selectedTermPeriodId = $state(
  academicContext.effectivePeriodId || academicContext.allPeriods[0]?.id || "",
);
let selectedCalendarId = $state(
  academicContext.effectiveYearId || academicContext.allYears[0]?.id || "",
);
let selectedCertType = $state<"secondary_thanawiya" | "basic_grade9">(
  "secondary_thanawiya",
);

// Dialog states for printing
let showPrintCertificateDialog = $state(false);
let printTemplateId = $state("academic.student-certificate-yemeni");
let printTitle = $state("");

// ── 1. Raw Assessment Records Query ─────────────────────────
const gradesQuery = createQuery(() => ({
  queryKey: ["assessment-grades", "student", orgId, studentId],
  queryFn: () => api.listGrades({ studentId }),
  enabled: !!orgId && !!studentId && activeTab === "records",
}));

// ── 2. Term Report Card Query ────────────────────────────────
const termReportQuery = createQuery(() => ({
  queryKey: [
    "assessment-term-results",
    studentId,
    selectedTermPeriodId,
    classId ?? "",
  ],
  queryFn: () =>
    api.getStudentTermResults(
      studentId,
      selectedTermPeriodId,
      classId || undefined,
    ),
  enabled:
    !!orgId && !!studentId && !!selectedTermPeriodId && activeTab === "term",
}));

// ── 3. Annual Report Query ───────────────────────────────────
const annualReportQuery = createQuery(() => ({
  queryKey: [
    "assessment-annual-results",
    studentId,
    selectedCalendarId,
    classId ?? "",
  ],
  queryFn: () =>
    api.getStudentAnnualResults(
      studentId,
      selectedCalendarId,
      classId || undefined,
    ),
  enabled:
    !!orgId && !!studentId && !!selectedCalendarId && activeTab === "annual",
}));

// ── 4. Certificate Results Query ─────────────────────────────
const certificateReportQuery = createQuery(() => ({
  queryKey: [
    "assessment-certificate-results",
    studentId,
    selectedCalendarId,
    selectedCertType,
    classId ?? "",
  ],
  queryFn: () =>
    api.getStudentCertificateResults(
      studentId,
      selectedCalendarId,
      selectedCertType,
      classId || undefined,
    ),
  enabled:
    !!orgId &&
    !!studentId &&
    !!selectedCalendarId &&
    activeTab === "certificate",
}));

const periodNames = $derived(
  new Map(
    academicContext.allPeriods.map((period) => [
      period.id,
      period.name_ar || period.name,
    ]),
  ),
);

const yearNames = $derived(
  new Map(
    academicContext.allYears.map((year) => [
      year.id,
      year.name_ar || year.name,
    ]),
  ),
);

// ── Helper: Format Status ────────────────────────────────────
function getStatusBadge(status?: string | null) {
  const s = String(status || "").toLowerCase();
  if (s.includes("honor") || s === "passedwithhonors") {
    return {
      label: m.student_grades_status_honors(),
      variant: "default" as const,
      color:
        "bg-emerald-600/15 text-emerald-700 dark:text-emerald-400 border-emerald-600/30",
    };
  }
  if (s.includes("pass") || s === "passed") {
    return {
      label: m.student_grades_status_passed(),
      variant: "default" as const,
      color:
        "bg-emerald-500/15 text-emerald-600 dark:text-emerald-400 border-emerald-500/30",
    };
  }
  if (
    s.includes("conditional") ||
    s.includes("makeup") ||
    s.includes("second")
  ) {
    return {
      label: m.student_grades_status_conditional_retake(),
      variant: "secondary" as const,
      color:
        "bg-amber-500/15 text-amber-700 dark:text-amber-400 border-amber-500/30",
    };
  }
  if (s.includes("fail") || s === "failed") {
    return {
      label: m.student_grades_status_failed(),
      variant: "destructive" as const,
      color:
        "bg-rose-500/15 text-rose-600 dark:text-rose-400 border-rose-500/30",
    };
  }
  return {
    label: m.student_grades_status_pending(),
    variant: "secondary" as const,
    color: "bg-muted text-muted-foreground border-border",
  };
}

function getAppreciation(percentage?: number | null) {
  if (percentage == null) return "—";
  if (percentage >= 90) return m.student_grades_appreciation_excellent();
  if (percentage >= 80) return m.student_grades_appreciation_very_good();
  if (percentage >= 65) return m.student_grades_appreciation_good();
  if (percentage >= 50) return m.student_grades_appreciation_pass();
  return m.student_grades_appreciation_fail();
}

// ── Print Triggers ───────────────────────────────────────────
function triggerPrintCertificate() {
  printTemplateId = "academic.student-certificate-yemeni";
  printTitle = m.student_grades_certificate_print_title();
  showPrintCertificateDialog = true;
}

function triggerPrintTermReport() {
  printTemplateId = "academic.student-term-report-card";
  printTitle = m.student_grades_term_report_print_title();
  showPrintCertificateDialog = true;
}

// ── Grid Configuration for Raw Records ──────────────────────
const filteredGrades = $derived(
  (gradesQuery.data ?? []).filter(
    (grade) =>
      selectedPeriodId === "all" || grade.period_id === selectedPeriodId,
  ),
);

const rows = $derived(
  filteredGrades.map((grade: GradeDto) => ({
    period: periodNames.get(grade.period_id) ?? grade.period_id,
    offering_id: grade.section_subject_id,
    component_id: grade.component_id ?? "—",
    score: Number(grade.score),
    maximum: Number(grade.max_score),
    weight: grade.weight == null ? "—" : Number(grade.weight),
    letter_grade: grade.letter_grade ?? "—",
    record_state: grade.is_exempt
      ? m.student_grades_state_exempt()
      : grade.is_excused
        ? m.student_grades_state_excused()
        : grade.is_final
          ? m.student_grades_state_final()
          : m.student_grades_state_recorded(),
    updated_at: formatDate(grade.updated_at, "datetime"),
  })),
);

const columns: OrbisColumnDef[] = [
  {
    id: "period",
    prop: "period",
    header: m.student_grades_period(),
    name: m.student_grades_period(),
    width: 170,
    size: 170,
  },
  {
    id: "offering_id",
    prop: "offering_id",
    header: m.student_grades_course_or_offering(),
    name: m.student_grades_course_or_offering(),
    width: 230,
    size: 230,
    pin: "left",
  },
  {
    id: "component_id",
    prop: "component_id",
    header: m.student_grades_assessment_component(),
    name: m.student_grades_assessment_component(),
    width: 190,
    size: 190,
  },
  {
    id: "score",
    prop: "score",
    header: m.student_grades_score(),
    name: m.student_grades_score(),
    width: 95,
    size: 95,
    align: "end",
    type: "number",
  },
  {
    id: "maximum",
    prop: "maximum",
    header: m.student_grades_maximum_score(),
    name: m.student_grades_maximum_score(),
    width: 105,
    size: 105,
    align: "end",
    type: "number",
  },
  {
    id: "weight",
    prop: "weight",
    header: m.student_grades_weight(),
    name: m.student_grades_weight(),
    width: 90,
    size: 90,
    align: "end",
    type: "number",
  },
  {
    id: "letter_grade",
    prop: "letter_grade",
    header: m.student_grades_grade(),
    name: m.student_grades_grade(),
    width: 90,
    size: 90,
  },
  {
    id: "record_state",
    prop: "record_state",
    header: m.student_grades_record_state(),
    name: m.student_grades_record_state(),
    width: 115,
    size: 115,
  },
  {
    id: "updated_at",
    prop: "updated_at",
    header: m.student_grades_last_updated(),
    name: m.student_grades_last_updated(),
    width: 180,
    size: 180,
  },
];
</script>

<div class="flex flex-col gap-4 text-xs text-start font-sans print:p-0">
  <!-- Mode Selection Tabs & Action Bar -->
  <div class="flex flex-wrap items-center justify-between gap-3 border-b border-border/80 pb-3">
    <div class="inline-flex rounded-lg border border-border bg-muted/30 p-1">
      <Button
        variant={activeTab === "records" ? "secondary" : "ghost"}
        size="sm"
        class="gap-1.5 text-xs font-semibold"
        onclick={() => (activeTab = "records")}
      >
        <BookOpen size={13} class={activeTab === "records" ? "text-primary" : ""} />
        <span>{m.student_grades_tab_assessment_records()}</span>
      </Button>

      <Button
        variant={activeTab === "term" ? "secondary" : "ghost"}
        size="sm"
        class="gap-1.5 text-xs font-semibold"
        onclick={() => (activeTab = "term")}
      >
        <DocumentChartBar size={13} class={activeTab === "term" ? "text-primary" : ""} />
        <span>{m.student_grades_tab_term_report()}</span>
      </Button>

      <Button
        variant={activeTab === "annual" ? "secondary" : "ghost"}
        size="sm"
        class="gap-1.5 text-xs font-semibold"
        onclick={() => (activeTab = "annual")}
      >
        <ChartBar size={13} class={activeTab === "annual" ? "text-primary" : ""} />
        <span>{m.student_grades_tab_annual_results()}</span>
      </Button>

      <Button
        variant={activeTab === "certificate" ? "secondary" : "ghost"}
        size="sm"
        class="gap-1.5 text-xs font-semibold"
        onclick={() => (activeTab = "certificate")}
      >
        <AcademicCap size={13} class={activeTab === "certificate" ? "text-primary" : ""} />
        <span>{m.student_grades_tab_certificate_results()}</span>
      </Button>
    </div>

    <!-- Contextual Actions -->
    <div class="flex items-center gap-2">
      {#if activeTab === "certificate"}
        <Button
          variant="outline"
          size="sm"
          class="h-8 gap-1.5 text-xs font-semibold text-primary border-primary/30 hover:bg-primary/5"
          onclick={triggerPrintCertificate}
        >
          <Printer size={13} />
          <span>{m.student_grades_print_official_certificate()}</span>
        </Button>
      {:else if activeTab === "term"}
        <Button
          variant="outline"
          size="sm"
          class="h-8 gap-1.5 text-xs font-semibold"
          onclick={triggerPrintTermReport}
        >
          <Printer size={13} />
          <span>{m.student_grades_print_term_report()}</span>
        </Button>
      {/if}
    </div>
  </div>

  <!-- ═══════════════════════════════════════════════════════════ -->
  <!-- TAB 1: RAW ASSESSMENT RECORDS                               -->
  <!-- ═══════════════════════════════════════════════════════════ -->
  {#if activeTab === "records"}
    <Card class="overflow-hidden">
      <CardHeader class="flex flex-row flex-wrap items-center justify-between gap-3 border-b border-border px-4 py-3">
        <div class="flex items-center gap-2">
          <BookOpen size={16} class="text-primary" />
          <div class="flex flex-col">
            <CardTitle class="text-xs font-bold uppercase tracking-wider">
              {m.student_grades_assessment_records_title()}
            </CardTitle>
            <CardDescription class="text-[10px]">
              {filteredGrades.length} {m.student_grades_records_label()}
            </CardDescription>
          </div>
        </div>

        <CardAction>
          <div class="flex items-center gap-2 print:hidden">
            <Select
              type="single"
              value={selectedPeriodId}
              onValueChange={(value) => {
                if (value) selectedPeriodId = value;
              }}
            >
              <SelectTrigger class="h-8 min-w-40 text-xs">
                {selectedPeriodId === "all"
                  ? m.student_grades_all_periods()
                  : periodNames.get(selectedPeriodId) ?? m.student_grades_select_period()}
              </SelectTrigger>
              <SelectContent class="text-xs">
                <SelectItem value="all">{m.student_grades_all_periods()}</SelectItem>
                {#each academicContext.allPeriods as period (period.id)}
                  <SelectItem value={period.id}>
                    {period.name_ar || period.name}
                  </SelectItem>
                {/each}
              </SelectContent>
            </Select>

            <Button
              variant="outline"
              size="sm"
              class="h-8 gap-1.5 text-xs"
              onclick={() => gradesQuery.refetch()}
              disabled={gradesQuery.isFetching}
            >
              {#if gradesQuery.isFetching}<LoadingIndicator size={13} />{:else}<Refresh size={13} />{/if}
              {m.common_retry()}
            </Button>
          </div>
        </CardAction>
      </CardHeader>

      <CardContent class="p-0">
        {#if gradesQuery.isLoading}
          <div class="flex min-h-64 flex-col items-center justify-center gap-2">
            <LoadingIndicator size={24} />
            <span class="text-xs text-muted-foreground">{m.common_loading_data()}</span>
          </div>
        {:else if gradesQuery.error}
          <div class="flex min-h-64 flex-col items-center justify-center p-6 text-center text-destructive">
            <InfoCircle class="mb-2 h-8 w-8 stroke-current opacity-80" />
            <p class="text-xs font-semibold">{m.common_load_error()}</p>
            <p class="mt-1 text-[10px] text-muted-foreground">
              {extractErrorMessage(gradesQuery.error)}
            </p>
            <Button
              variant="outline"
              size="sm"
              class="mt-4"
              onclick={() => gradesQuery.refetch()}
            >
              {m.common_retry()}
            </Button>
          </div>
        {:else if rows.length === 0}
          <StudentSectionState
            icon={BookOpen}
            title={m.academics_grades_empty_title()}
            description={m.academics_grades_empty_subtitle()}
          />
        {:else}
          <div class="h-115 w-full">
            <OrbisGrid
              {columns}
              source={rows}
              loading={gradesQuery.isFetching}
              clientQuery
              rtl={isRtl}
              readonly
            />
          </div>
        {/if}
      </CardContent>
    </Card>

  <!-- ═══════════════════════════════════════════════════════════ -->
  <!-- TAB 2: TERM REPORT CARD                                     -->
  <!-- ═══════════════════════════════════════════════════════════ -->
  {:else if activeTab === "term"}
    <div class="flex flex-col gap-4">
      <!-- Period Selector Header -->
      <Card>
        <CardContent class="flex flex-wrap items-center justify-between gap-3 p-3">
          <div class="flex items-center gap-3">
            <div class="h-9 w-9 rounded-lg bg-primary/10 flex items-center justify-center text-primary">
              <DocumentChartBar size={18} />
            </div>
            <div>
              <h2 class="text-xs font-bold text-foreground">
                {m.student_grades_term_report_heading()}
              </h2>
              <p class="text-[11px] text-muted-foreground">
                {m.student_grades_term_report_description()}
              </p>
            </div>
          </div>

          <div class="flex items-center gap-2">
            <Select
              type="single"
              value={selectedTermPeriodId}
              onValueChange={(val) => {
                if (val) selectedTermPeriodId = val;
              }}
            >
              <SelectTrigger class="h-8 min-w-45 text-xs">
                {periodNames.get(selectedTermPeriodId) ?? m.student_grades_select_term()}
              </SelectTrigger>
              <SelectContent class="text-xs">
                {#each academicContext.allPeriods as period (period.id)}
                  <SelectItem value={period.id}>
                    {period.name_ar || period.name}
                  </SelectItem>
                {/each}
              </SelectContent>
            </Select>

            <Button
              variant="outline"
              size="sm"
              class="h-8 gap-1.5 text-xs"
              onclick={() => termReportQuery.refetch()}
              disabled={termReportQuery.isFetching}
            >
              {#if termReportQuery.isFetching}<LoadingIndicator size={13} />{:else}<Refresh size={13} />{/if}
              {m.common_retry()}
            </Button>
          </div>
        </CardContent>
      </Card>

      {#if termReportQuery.isLoading}
        <div class="flex min-h-64 flex-col items-center justify-center gap-2 rounded-xl border border-border bg-card p-8">
          <LoadingIndicator size={24} />
          <span class="text-xs text-muted-foreground">{m.common_loading_data()}</span>
        </div>
      {:else if termReportQuery.error}
        <div class="flex min-h-64 flex-col items-center justify-center rounded-xl border border-destructive/20 bg-destructive/5 p-8 text-center text-destructive">
          <InfoCircle class="mb-2 h-8 w-8 stroke-current opacity-80" />
          <p class="text-xs font-semibold">{m.common_load_error()}</p>
          <p class="mt-1 text-[10px] text-muted-foreground">
            {extractErrorMessage(termReportQuery.error)}
          </p>
        </div>
      {:else if termReportQuery.data}
        {@const termReport = termReportQuery.data}
        {@const badge = getStatusBadge(termReport.status)}

        <!-- Performance Summary KPIs -->
        <div class="grid grid-cols-2 gap-3 sm:grid-cols-5">
          <StudentMetricCard
            title={m.student_grades_total_score()}
            value={`${formatDecimal(Number(termReport.total_score), 1)} / ${formatNumber(Number(termReport.max_possible_score))}`}
            icon={AcademicCap}
          />
          <StudentMetricCard
            title={m.student_grades_percentage()}
            value={formatPercent(Number(termReport.overall_percentage))}
            icon={ChartBar}
          />
          <StudentMetricCard
            title={m.student_grades_appreciation_label()}
            value={getAppreciation(Number(termReport.overall_percentage))}
            icon={InfoCircle}
          />
          <StudentMetricCard
            title={m.student_grades_academic_decision()}
            value={badge.label}
            icon={InfoCircle}
            tone={badge.variant === "destructive" ? "danger" : "success"}
          />
          <StudentMetricCard
            title={m.student_grades_rank_in_class()}
            value={termReport.rank_in_class ? `#${formatNumber(termReport.rank_in_class)}` : "—"}
            icon={BookOpen}
          >
            {#if termReport.total_students_in_class}
              {m.student_grades_of()} {formatNumber(termReport.total_students_in_class)}
            {/if}
          </StudentMetricCard>
        </div>

        <!-- Subject Results Table -->
        {#if termReport.subjects.length === 0}
          <StudentSectionState
            icon={BookOpen}
            title={m.academics_grades_empty_title()}
            description={m.academics_grades_empty_subtitle()}
          />
        {:else}
        <Card class="overflow-hidden">
          <div class="overflow-x-auto">
            <table class="w-full text-xs text-start">
              <thead class="bg-muted/40 border-b border-border text-[11px] font-semibold text-muted-foreground">
                <tr>
                  <th class="px-4 py-2.5 text-start">{m.student_grades_subject()}</th>
                  <th class="px-3 py-2.5 text-center">{m.student_grades_coursework_20()}</th>
                  <th class="px-3 py-2.5 text-center">{m.student_grades_term_exam_30()}</th>
                  <th class="px-3 py-2.5 text-center font-bold text-foreground">{m.student_grades_term_total_50()}</th>
                  <th class="px-3 py-2.5 text-center">{m.student_grades_percentage()}</th>
                  <th class="px-3 py-2.5 text-center">{m.student_grades_grade()}</th>
                  <th class="px-3 py-2.5 text-center">{m.student_grades_status()}</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-border/60">
                {#each termReport.subjects as subj (subj.subject_id)}
                  {@const subjPassed = String(subj.status).toLowerCase().includes("pass")}
                  <tr class="hover:bg-muted/20 transition-colors">
                    <td class="px-4 py-2.5 font-medium text-foreground">
                      {isRtl ? (subj.subject_name_ar || subj.subject_name) : subj.subject_name}
                    </td>
                    <td class="px-3 py-2.5 text-center text-muted-foreground">
                      {formatDecimal(Number(subj.monthly_contribution), 1)}
                    </td>
                    <td class="px-3 py-2.5 text-center text-muted-foreground">
                      {subj.term_final_exam != null ? formatDecimal(Number(subj.term_final_exam), 1) : "—"}
                    </td>
                    <td class="px-3 py-2.5 text-center font-bold text-foreground">
                      {formatDecimal(Number(subj.term_total), 1)}
                    </td>
                    <td class="px-3 py-2.5 text-center text-muted-foreground">
                      {formatPercent(Number(subj.percentage))}
                    </td>
                    <td class="px-3 py-2.5 text-center font-medium">
                      {subj.letter_grade ?? getAppreciation(Number(subj.percentage))}
                    </td>
                    <td class="px-3 py-2.5 text-center">
                      <span class={`inline-block px-2 py-0.5 rounded text-[10px] font-semibold ${
                        subjPassed
                          ? "bg-emerald-500/10 text-emerald-600 dark:text-emerald-400"
                          : "bg-rose-500/10 text-rose-600 dark:text-rose-400"
                      }`}>
                        {subjPassed ? m.student_grades_status_passed() : m.student_grades_status_failed()}
                      </span>
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        </Card>
        {/if}
      {/if}
    </div>

  <!-- ═══════════════════════════════════════════════════════════ -->
  <!-- TAB 3: ANNUAL REPORT CARD                                   -->
  <!-- ═══════════════════════════════════════════════════════════ -->
  {:else if activeTab === "annual"}
    <div class="flex flex-col gap-4">
      <!-- Calendar Selector Header -->
      <Card>
        <CardContent class="flex flex-wrap items-center justify-between gap-3 p-3">
          <div class="flex items-center gap-3">
            <div class="h-9 w-9 rounded-lg bg-primary/10 flex items-center justify-center text-primary">
              <ChartBar size={18} />
            </div>
            <div>
              <h2 class="text-xs font-bold text-foreground">
                {m.student_grades_annual_results_heading()}
              </h2>
              <p class="text-[11px] text-muted-foreground">
                {m.student_grades_annual_results_description()}
              </p>
            </div>
          </div>

          <div class="flex items-center gap-2">
            <Select
              type="single"
              value={selectedCalendarId}
              onValueChange={(val) => {
                if (val) selectedCalendarId = val;
              }}
            >
              <SelectTrigger class="h-8 min-w-45 text-xs">
                {yearNames.get(selectedCalendarId) ?? m.student_grades_select_academic_year()}
              </SelectTrigger>
              <SelectContent class="text-xs">
                {#each academicContext.allYears as year (year.id)}
                  <SelectItem value={year.id}>
                    {year.name_ar || year.name}
                  </SelectItem>
                {/each}
              </SelectContent>
            </Select>

            <Button
              variant="outline"
              size="sm"
              class="h-8 gap-1.5 text-xs"
              onclick={() => annualReportQuery.refetch()}
              disabled={annualReportQuery.isFetching}
            >
              {#if annualReportQuery.isFetching}<LoadingIndicator size={13} />{:else}<Refresh size={13} />{/if}
              {m.common_retry()}
            </Button>
          </div>
        </CardContent>
      </Card>

      {#if annualReportQuery.isLoading}
        <div class="flex min-h-64 flex-col items-center justify-center gap-2 rounded-xl border border-border bg-card p-8">
          <LoadingIndicator size={24} />
          <span class="text-xs text-muted-foreground">{m.common_loading_data()}</span>
        </div>
      {:else if annualReportQuery.error}
        <div class="flex min-h-64 flex-col items-center justify-center rounded-xl border border-destructive/20 bg-destructive/5 p-8 text-center text-destructive">
          <InfoCircle class="mb-2 h-8 w-8 stroke-current opacity-80" />
          <p class="text-xs font-semibold">{m.common_load_error()}</p>
          <p class="mt-1 text-[10px] text-muted-foreground">
            {extractErrorMessage(annualReportQuery.error)}
          </p>
        </div>
      {:else if annualReportQuery.data}
        {@const annualReport = annualReportQuery.data}
        {@const badge = getStatusBadge(annualReport.status)}

        <!-- Performance Summary KPIs -->
        <div class="grid grid-cols-2 gap-3 md:grid-cols-3 2xl:grid-cols-5">
          <StudentMetricCard
            title={m.student_grades_annual_total()}
            value={`${formatDecimal(Number(annualReport.total_score), 1)} / ${formatNumber(Number(annualReport.max_possible_score))}`}
            icon={AcademicCap}
          />
          <StudentMetricCard
            title={m.student_grades_annual_percentage()}
            value={formatPercent(Number(annualReport.overall_percentage))}
            icon={ChartBar}
          />
          <StudentMetricCard
            title={m.student_grades_appreciation_label()}
            value={getAppreciation(Number(annualReport.overall_percentage))}
            icon={InfoCircle}
          />
          <StudentMetricCard
            title={m.student_grades_promotion_status()}
            value={badge.label}
            icon={InfoCircle}
            tone={badge.variant === "destructive" ? "danger" : "success"}
          />
          <StudentMetricCard
            title={m.student_grades_rank()}
            value={annualReport.rank_in_class ? `#${formatNumber(annualReport.rank_in_class)}` : "—"}
            icon={BookOpen}
          >
            {#if annualReport.total_students_in_class}
              {m.student_grades_in_class()} {formatNumber(annualReport.total_students_in_class)}
            {/if}
          </StudentMetricCard>
        </div>

        <!-- Subject Results Table -->
        {#if annualReport.subjects.length === 0}
          <StudentSectionState
            icon={BookOpen}
            title={m.academics_grades_empty_title()}
            description={m.academics_grades_empty_subtitle()}
          />
        {:else}
        <Card class="overflow-hidden">
          <div class="overflow-x-auto">
            <table class="w-full text-xs text-start">
              <thead class="bg-muted/40 border-b border-border text-[11px] font-semibold text-muted-foreground">
                <tr>
                  <th class="px-4 py-2.5 text-start">{m.student_grades_subject()}</th>
                  <th class="px-3 py-2.5 text-center">{m.student_grades_term_1_50()}</th>
                  <th class="px-3 py-2.5 text-center">{m.student_grades_term_2_50()}</th>
                  <th class="px-3 py-2.5 text-center font-bold text-foreground">{m.student_grades_annual_total_100()}</th>
                  <th class="px-3 py-2.5 text-center">{m.student_grades_percentage()}</th>
                  <th class="px-3 py-2.5 text-center">{m.student_grades_grade()}</th>
                  <th class="px-3 py-2.5 text-center">{m.student_grades_status()}</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-border/60">
                {#each annualReport.subjects as subj (subj.subject_id)}
                  {@const subjPassed = String(subj.status).toLowerCase().includes("pass")}
                  <tr class="hover:bg-muted/20 transition-colors">
                    <td class="px-4 py-2.5 font-medium text-foreground">
                      {isRtl ? (subj.subject_name_ar || subj.subject_name) : subj.subject_name}
                    </td>
                    <td class="px-3 py-2.5 text-center text-muted-foreground">
                      {subj.term_1_result ? formatDecimal(Number(subj.term_1_result.term_total), 1) : "—"}
                    </td>
                    <td class="px-3 py-2.5 text-center text-muted-foreground">
                      {subj.term_2_result ? formatDecimal(Number(subj.term_2_result.term_total), 1) : "—"}
                    </td>
                    <td class="px-3 py-2.5 text-center font-bold text-foreground">
                      {formatDecimal(Number(subj.annual_total), 1)}
                    </td>
                    <td class="px-3 py-2.5 text-center text-muted-foreground">
                      {formatPercent(Number(subj.percentage))}
                    </td>
                    <td class="px-3 py-2.5 text-center font-medium">
                      {subj.letter_grade ?? getAppreciation(Number(subj.percentage))}
                    </td>
                    <td class="px-3 py-2.5 text-center">
                      <span class={`inline-block px-2 py-0.5 rounded text-[10px] font-semibold ${
                        subjPassed
                          ? "bg-emerald-500/10 text-emerald-600 dark:text-emerald-400"
                          : "bg-rose-500/10 text-rose-600 dark:text-rose-400"
                      }`}>
                        {subjPassed ? m.student_grades_status_passed() : m.student_grades_status_failed()}
                      </span>
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        </Card>
        {/if}
      {/if}
    </div>

  <!-- ═══════════════════════════════════════════════════════════ -->
  <!-- TAB 4: CERTIFICATE RESULTS (GRADE 9 & 12 PUBLIC EXAMS)      -->
  <!-- ═══════════════════════════════════════════════════════════ -->
  {:else if activeTab === "certificate"}
    <div class="flex flex-col gap-4">
      <!-- Certificate Configuration & Filters Header -->
      <Card>
        <CardContent class="flex flex-wrap items-center justify-between gap-3 p-3">
          <div class="flex items-center gap-3">
            <div class="h-9 w-9 rounded-lg bg-primary/10 flex items-center justify-center text-primary">
              <AcademicCap size={18} />
            </div>
            <div>
              <h2 class="text-xs font-bold text-foreground">
                {m.student_grades_public_certificate_results_heading()}
              </h2>
              <p class="text-[11px] text-muted-foreground">
                {isRtl
                  ? m.student_grades_public_certificate_results_description()
                  : m.student_grades_public_certificate_results_description()}
              </p>
            </div>
          </div>

          <div class="flex flex-wrap items-center gap-2">
            <!-- Certificate Type Selector -->
            <Select
              type="single"
              value={selectedCertType}
              onValueChange={(val) => {
                if (val === "secondary_thanawiya" || val === "basic_grade9") {
                  selectedCertType = val;
                }
              }}
            >
              <SelectTrigger class="h-8 min-w-[210px] text-xs">
                {selectedCertType === "secondary_thanawiya"
                  ? m.student_grades_secondary_certificate()
                  : m.student_grades_basic_certificate()}
              </SelectTrigger>
              <SelectContent class="text-xs">
                <SelectItem value="secondary_thanawiya">
                  {m.student_grades_secondary_certificate()}
                </SelectItem>
                <SelectItem value="basic_grade9">
                  {m.student_grades_basic_certificate()}
                </SelectItem>
              </SelectContent>
            </Select>

            <!-- Calendar Selector -->
            <Select
              type="single"
              value={selectedCalendarId}
              onValueChange={(val) => {
                if (val) selectedCalendarId = val;
              }}
            >
              <SelectTrigger class="h-8 min-w-[170px] text-xs">
                {yearNames.get(selectedCalendarId) ?? m.student_grades_select_academic_year()}
              </SelectTrigger>
              <SelectContent class="text-xs">
                {#each academicContext.allYears as year (year.id)}
                  <SelectItem value={year.id}>
                    {year.name_ar || year.name}
                  </SelectItem>
                {/each}
              </SelectContent>
            </Select>

            <Button
              variant="outline"
              size="sm"
              class="h-8 gap-1.5 text-xs"
              onclick={() => certificateReportQuery.refetch()}
              disabled={certificateReportQuery.isFetching}
            >
              {#if certificateReportQuery.isFetching}<LoadingIndicator size={13} />{:else}<Refresh size={13} />{/if}
              {m.common_retry()}
            </Button>
          </div>
        </CardContent>
      </Card>

      <!-- Yemen Sana'a MoE Policy Formula Explanation Banner -->
      {#if selectedCertType === "secondary_thanawiya"}
        <div class="flex items-center gap-2.5 rounded-lg border border-primary/25 bg-primary/5 px-3.5 py-2.5 text-xs text-primary">
          <InfoCircle size={16} class="shrink-0" />
          <div class="flex-1">
            <span class="font-bold">
              {m.student_grades_approved_formula_heading()}
            </span>
            <span class="ms-1 text-foreground">
              {m.student_grades_approved_formula_description()}
            </span>
          </div>
        </div>
      {/if}

      {#if certificateReportQuery.isLoading}
        <div class="flex min-h-64 flex-col items-center justify-center gap-2 rounded-xl border border-border bg-card p-8">
          <LoadingIndicator size={24} />
          <span class="text-xs text-muted-foreground">{m.common_loading_data()}</span>
        </div>
      {:else if certificateReportQuery.error}
        <div class="flex min-h-64 flex-col items-center justify-center rounded-xl border border-destructive/20 bg-destructive/5 p-8 text-center text-destructive">
          <InfoCircle class="mb-2 h-8 w-8 stroke-current opacity-80" />
          <p class="text-xs font-semibold">{m.common_load_error()}</p>
          <p class="mt-1 text-[10px] text-muted-foreground">
            {extractErrorMessage(certificateReportQuery.error)}
          </p>
        </div>
      {:else if certificateReportQuery.data}
        {@const certReport = certificateReportQuery.data}
        {@const badge = getStatusBadge(certReport.status)}

        <!-- Performance Summary KPIs -->
        <div class="grid grid-cols-2 gap-3 md:grid-cols-3 2xl:grid-cols-5">
          <StudentMetricCard
            title={m.student_grades_total_score()}
            value={`${formatDecimal(Number(certReport.total_score), 1)} / ${formatNumber(Number(certReport.max_possible_score))}`}
            icon={AcademicCap}
          />
          <StudentMetricCard
            title={m.student_grades_overall_percentage()}
            value={formatPercent(Number(certReport.overall_percentage))}
            icon={ChartBar}
          />
          <StudentMetricCard
            title={m.student_grades_appreciation_label()}
            value={getAppreciation(Number(certReport.overall_percentage))}
            icon={InfoCircle}
          />
          <StudentMetricCard
            title={m.student_grades_final_status()}
            value={badge.label}
            icon={InfoCircle}
            tone={badge.variant === "destructive" ? "danger" : "success"}
          />
          <StudentMetricCard
            title={m.student_grades_rank()}
            value={certReport.rank_in_class ? `#${formatNumber(certReport.rank_in_class)}` : "—"}
            icon={BookOpen}
          >
            {#if certReport.total_students_in_class}
              {m.student_grades_in_cohort()} {formatNumber(certReport.total_students_in_class)}
            {/if}
          </StudentMetricCard>
        </div>

        <!-- Certificate Subjects Table -->
        {#if certReport.subjects.length === 0}
          <StudentSectionState
            icon={BookOpen}
            title={m.academics_grades_empty_title()}
            description={m.academics_grades_empty_subtitle()}
          />
        {:else}
        <Card class="overflow-hidden">
          <div class="overflow-x-auto">
            <table class="w-full text-xs text-start">
              <thead class="bg-muted/40 border-b border-border text-[11px] font-semibold text-muted-foreground">
                <tr>
                  <th class="px-4 py-2.5 text-start">{m.student_grades_subject()}</th>
                  {#if selectedCertType === "secondary_thanawiya"}
                    <th class="px-3 py-2.5 text-center">{m.student_grades_ministerial_exam_80()}</th>
                    <th class="px-3 py-2.5 text-center">{m.student_grades_cumulative_average_20()}</th>
                  {/if}
                  <th class="px-3 py-2.5 text-center font-bold text-foreground">
                    {m.student_grades_final_score_100()}
                  </th>
                  <th class="px-3 py-2.5 text-center">{m.student_grades_minimum_pass_50()}</th>
                  <th class="px-3 py-2.5 text-center">{m.student_grades_percentage()}</th>
                  <th class="px-3 py-2.5 text-center">{m.student_grades_grade()}</th>
                  <th class="px-3 py-2.5 text-center">{m.student_grades_status()}</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-border/60">
                {#each certReport.subjects as subj (subj.subject_id)}
                  {@const subjPassed = String(subj.status).toLowerCase().includes("pass")}
                  <tr class="hover:bg-muted/20 transition-colors">
                    <td class="px-4 py-2.5 font-medium text-foreground">
                      {isRtl ? (subj.subject_name_ar || subj.subject_name) : subj.subject_name}
                    </td>

                    {#if selectedCertType === "secondary_thanawiya"}
                      <td class="px-3 py-2.5 text-center text-muted-foreground">
                        {subj.term_final_exam != null ? formatDecimal(Number(subj.term_final_exam), 1) : "—"}
                      </td>
                      <td class="px-3 py-2.5 text-center text-muted-foreground">
                        {formatDecimal(Number(subj.monthly_contribution), 1)}
                      </td>
                    {/if}

                    <td class="px-3 py-2.5 text-center font-bold text-foreground">
                      {formatDecimal(Number(subj.term_total), 1)}
                    </td>
                    <td class="px-3 py-2.5 text-center text-muted-foreground">
                      {formatNumber(50)}
                    </td>
                    <td class="px-3 py-2.5 text-center text-muted-foreground">
                      {formatPercent(Number(subj.percentage))}
                    </td>
                    <td class="px-3 py-2.5 text-center font-medium">
                      {subj.letter_grade ?? getAppreciation(Number(subj.percentage))}
                    </td>
                    <td class="px-3 py-2.5 text-center">
                      <span class={`inline-block px-2 py-0.5 rounded text-[10px] font-semibold ${
                        subjPassed
                          ? "bg-emerald-500/10 text-emerald-600 dark:text-emerald-400"
                          : "bg-rose-500/10 text-rose-600 dark:text-rose-400"
                      }`}>
                        {subjPassed ? m.student_grades_status_passed() : m.student_grades_status_failed()}
                      </span>
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        </Card>
        {/if}
      {/if}
    </div>
  {/if}
</div>

<!-- Print PDF Report Viewer Dialog -->
<PdfReportViewer
  bind:open={showPrintCertificateDialog}
  templateId={printTemplateId}
  parameters={{
    student_id: studentId,
    calendar_id: selectedCalendarId,
    period_id: selectedTermPeriodId,
    certificate_type: selectedCertType,
    class_id: classId,
  }}
  reportTitle={printTitle}
/>
