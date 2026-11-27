<script lang="ts">
import {
  api,
  extractErrorMessage,
  queryKeys,
  type GradeDto,
  type ReportDefinitionDto,
  type SubjectAnnualResult,
  type SubjectTermResult,
} from "$lib/api";
import {
  formatDate,
  formatDecimal,
  formatNumber,
  formatPercent,
} from "$lib/core/i18n";
import * as m from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";
import PdfReportViewer from "$lib/shared/components/pdf-report-viewer.svelte";
import { academicContext, orgContext } from "$lib/stores";
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
import { Badge } from "@orbis/ui/badge";
import { Button } from "@orbis/ui/button";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui/select";
import { createQuery } from "@tanstack/svelte-query";

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
const instType = $derived(orgContext.institutionType);

type ViewTab = "records" | "term" | "annual" | "certificate";
let activeTab = $state<ViewTab>("records");

let selectedPeriodId = $state("all");
let selectedTermPeriodId = $state(
  academicContext.effectivePeriodId || academicContext.allPeriods[0]?.id || "",
);
let selectedCalendarId = $state(
  academicContext.currentYear?.id || academicContext.allYears[0]?.id || "",
);

// Print modal state
let showPrintDialog = $state(false);
let printReportCode = $state("rep.academic.student-term-report-card");
let printTitle = $state("");

// ── Dynamic Institution Labels ───────────────────────────────
const labels = $derived.by(() => {
  if (instType === "university") {
    return {
      course: m.student_grades_uni_course(),
      coursework: m.student_grades_uni_coursework(),
      finalExam: m.student_grades_uni_final_exam(),
      termTotal: m.student_grades_uni_total(),
      termTab: m.student_grades_uni_tab_semester(),
      annualTab: m.student_grades_uni_tab_cumulative(),
      certTab: m.student_grades_uni_tab_documents(),
      term1: m.student_grades_uni_semester_1(),
      term2: m.student_grades_uni_semester_2(),
      credits: m.student_grades_uni_credits(),
      gpa: m.student_grades_uni_term_gpa(),
      cumGpa: m.student_grades_uni_cgpa(),
    };
  }
  if (instType === "institute") {
    return {
      course: m.student_grades_inst_module(),
      coursework: m.student_grades_inst_theory(),
      finalExam: m.student_grades_inst_practical(),
      termTotal: m.student_grades_total(),
      termTab: m.student_grades_inst_tab_modules(),
      annualTab: m.student_grades_inst_tab_summary(),
      certTab: m.student_grades_certificates_tab(),
      term1: m.student_grades_inst_phase_1(),
      term2: m.student_grades_inst_phase_2(),
      credits: m.student_grades_inst_units(),
      gpa: m.student_grades_inst_average(),
      cumGpa: m.student_grades_inst_overall_avg(),
    };
  }
  return {
    course: m.student_grades_subject(),
    coursework: m.student_grades_school_coursework(),
    finalExam: m.student_grades_school_term_exam(),
    termTotal: m.student_grades_total(),
    termTab: m.student_grades_tab_term_report(),
    annualTab: m.student_grades_tab_annual_results(),
    certTab: m.student_grades_certificates_tab(),
    term1: m.student_grades_school_term_1(),
    term2: m.student_grades_school_term_2(),
    credits: m.student_grades_school_credits(),
    gpa: m.student_grades_school_gpa(),
    cumGpa: m.student_grades_school_cum_gpa(),
  };
});

// ── Reference Queries ────────────────────────────────────────
const settingsQuery = createQuery(() => ({
  queryKey: queryKeys.org.settings(orgId),
  queryFn: () => api.getSettings(),
  enabled: !!orgId,
  staleTime: 5 * 60 * 1000,
}));

const passingThreshold = $derived(
  Number(settingsQuery.data?.academic?.passing_threshold_percentage ?? 50),
);

const subjectsQuery = createQuery(() => ({
  queryKey: queryKeys.academic.subjects(orgId),
  queryFn: () => api.listSubjects(),
  enabled: !!orgId && activeTab === "records",
  staleTime: 5 * 60 * 1000,
}));

const sectionSubjectsQuery = createQuery(() => ({
  queryKey: queryKeys.academic.SectionSubjects(
    classId ?? "",
    academicContext.effectivePeriodId || "",
  ),
  queryFn: () =>
    api.listSectionSubjects(
      classId ?? "",
      academicContext.effectivePeriodId || "",
    ),
  enabled: !!orgId && !!classId && activeTab === "records",
  staleTime: 5 * 60 * 1000,
}));

const schemesQuery = createQuery(() => ({
  queryKey: ["assessment-schemes", orgId],
  queryFn: () => api.listAssessmentSchemes(),
  enabled: !!orgId && activeTab === "records",
  staleTime: 5 * 60 * 1000,
}));

const reportDefsQuery = createQuery(() => ({
  queryKey: ["report-definitions", "student-certificates", orgId],
  queryFn: () => api.listReportDefinitions(),
  enabled: !!orgId && activeTab === "certificate",
  staleTime: 5 * 60 * 1000,
}));

// ── Data Queries ─────────────────────────────────────────────
const gradesQuery = createQuery(() => ({
  queryKey: ["assessment-grades", "student", orgId, studentId],
  queryFn: () => api.listGrades({ studentId }),
  enabled: !!orgId && !!studentId && activeTab === "records",
}));

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

const certificateReportQuery = createQuery(() => ({
  queryKey: [
    "assessment-certificate-results",
    studentId,
    selectedCalendarId,
    "general",
    classId ?? "",
  ],
  queryFn: () =>
    api.getStudentCertificateResults(
      studentId,
      selectedCalendarId,
      "general",
      classId || undefined,
    ),
  enabled:
    !!orgId &&
    !!studentId &&
    !!selectedCalendarId &&
    activeTab === "certificate",
}));

// ── Lookup Maps ──────────────────────────────────────────────
const periodNames = $derived(
  new Map(
    academicContext.allPeriods.map((p) => [
      p.id,
      isRtl ? p.name_ar || p.name : p.name,
    ]),
  ),
);

const yearNames = $derived(
  new Map(
    academicContext.allYears.map((y) => [
      y.id,
      isRtl ? y.name_ar || y.name : y.name,
    ]),
  ),
);

const subjectById = $derived(
  new Map((subjectsQuery.data ?? []).map((s) => [s.id, s])),
);

const offeringNameById = $derived.by(() => {
  const map = new Map<string, string>();
  for (const ss of sectionSubjectsQuery.data ?? []) {
    const subj = subjectById.get(ss.subject_id);
    if (subj) {
      map.set(ss.id, isRtl ? subj.name_ar || subj.name : subj.name);
    }
  }
  return map;
});

const componentNameByCode = $derived.by(() => {
  const map = new Map<string, string>();
  for (const scheme of schemesQuery.data ?? []) {
    for (const comp of scheme.components ?? []) {
      if (comp.code && !map.has(comp.code)) {
        map.set(comp.code, isRtl ? comp.nameAr || comp.name : comp.name);
      }
    }
  }
  return map;
});

const studentDocuments = $derived.by((): ReportDefinitionDto[] => {
  const all = reportDefsQuery.data ?? [];
  return all.filter(
    (def) =>
      def.is_active &&
      (def.code.includes("certificate") ||
        def.code.includes("transcript") ||
        def.code.includes("term-report-card")),
  );
});

// ── Helpers ──────────────────────────────────────────────────
function getStatusInfo(status?: string | null) {
  const s = String(status || "").toLowerCase();
  if (s.includes("honor") || s === "passed_with_honors") {
    return {
      label: m.student_grades_status_honors(),
      badgeClass:
        "bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 border-emerald-500/20",
    };
  }
  if (s.includes("pass") || s === "passed") {
    return {
      label: m.student_grades_status_passed(),
      badgeClass:
        "bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 border-emerald-500/20",
    };
  }
  if (
    s.includes("conditional") ||
    s.includes("makeup") ||
    s.includes("second")
  ) {
    return {
      label: m.student_grades_status_conditional_retake(),
      badgeClass:
        "bg-amber-500/10 text-amber-600 dark:text-amber-400 border-amber-500/20",
    };
  }
  if (s.includes("fail") || s === "failed") {
    return {
      label: m.student_grades_status_failed(),
      badgeClass:
        "bg-destructive/10 text-destructive border-destructive/20",
    };
  }
  if (s.includes("incomplete")) {
    return {
      label: m.student_grades_status_incomplete(),
      badgeClass:
        "bg-amber-500/10 text-amber-600 dark:text-amber-400 border-amber-500/20",
    };
  }
  return {
    label: m.student_grades_status_pending(),
    badgeClass: "bg-muted text-muted-foreground border-border",
  };
}

function getLetterDisplay(report: object) {
  const r = report as {
    overall_letter_grade?: string | null;
    overall_letter_grade_ar?: string | null;
  };
  return isRtl
    ? r.overall_letter_grade_ar || r.overall_letter_grade || "—"
    : r.overall_letter_grade || r.overall_letter_grade_ar || "—";
}

function maxLabel(max: number | string | null | undefined) {
  const n = Number(max ?? 0);
  return n > 0 ? ` (${formatNumber(n)})` : "";
}

function openPrint(reportCode: string, title: string) {
  printReportCode = reportCode;
  printTitle = title;
  showPrintDialog = true;
}

// ── Raw Records Grid ─────────────────────────────────────────
const filteredGrades = $derived(
  (gradesQuery.data ?? []).filter(
    (g) => selectedPeriodId === "all" || g.period_id === selectedPeriodId,
  ),
);

const recordRows = $derived(
  filteredGrades.map((grade: GradeDto) => {
    const score = Number(grade.score);
    const max = Number(grade.max_score);
    const pct = max > 0 ? (score / max) * 100 : 0;
    const compLabel =
      (grade.component_code && componentNameByCode.get(grade.component_code)) ||
      grade.component_code ||
      (grade.is_final ? labels.finalExam : null) ||
      grade.notes ||
      "—";

    return {
      id: grade.id,
      subject:
        offeringNameById.get(grade.section_subject_id) ??
        grade.section_subject_id,
      period: periodNames.get(grade.period_id) ?? grade.period_id,
      component: compLabel,
      score,
      maximum: max,
      percentage: Number(pct.toFixed(1)),
      letter_grade: grade.letter_grade ?? "—",
      record_state: grade.is_absent
        ? m.student_grades_state_absent()
        : grade.is_exempt
          ? m.student_grades_state_exempt()
          : grade.is_excused
            ? m.student_grades_state_excused()
            : grade.is_final
              ? m.student_grades_state_final()
              : m.student_grades_state_recorded(),
      updated_at: formatDate(grade.updated_at, "datetime"),
    };
  }),
);

const recordColumns = $derived<OrbisColumnDef[]>([
  {
    id: "subject",
    prop: "subject",
    header: labels.course,
    name: labels.course,
    width: 210,
    size: 210,
    pin: "left",
  },
  {
    id: "period",
    prop: "period",
    header: m.student_grades_period(),
    name: m.student_grades_period(),
    width: 150,
    size: 150,
  },
  {
    id: "component",
    prop: "component",
    header: m.student_grades_assessment_component(),
    name: m.student_grades_assessment_component(),
    width: 170,
    size: 170,
  },
  {
    id: "score",
    prop: "score",
    header: m.student_grades_score(),
    name: m.student_grades_score(),
    width: 90,
    size: 90,
    align: "end",
    type: "number",
  },
  {
    id: "maximum",
    prop: "maximum",
    header: m.student_grades_maximum_score(),
    name: m.student_grades_maximum_score(),
    width: 95,
    size: 95,
    align: "end",
    type: "number",
  },
  {
    id: "percentage",
    prop: "percentage",
    header: m.student_grades_percentage(),
    name: m.student_grades_percentage(),
    width: 100,
    size: 100,
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
    width: 110,
    size: 110,
  },
  {
    id: "updated_at",
    prop: "updated_at",
    header: m.student_grades_last_updated(),
    name: m.student_grades_last_updated(),
    width: 165,
    size: 165,
  },
]);

const showCredits = $derived(
  orgContext.isUniversity ||
    (termReportQuery.data?.subjects ?? []).some(
      (s) => Number(s.credit_hours) !== 1,
    ) ||
    (annualReportQuery.data?.subjects ?? []).some(
      (s) => Number(s.credit_hours) !== 1,
    ),
);
</script>

<div class="flex flex-col gap-5 text-xs text-start font-sans print:p-0">
  <!-- ── Clean Header Navigation & Context Controls ─────────── -->
  <div class="flex flex-wrap items-center justify-between gap-3 border-b border-border pb-3">
    <nav class="flex items-center gap-1">
      {#each [
        { id: "records" as const, label: m.student_grades_tab_assessment_records(), icon: BookOpen },
        { id: "term" as const, label: labels.termTab, icon: DocumentChartBar },
        { id: "annual" as const, label: labels.annualTab, icon: ChartBar },
        { id: "certificate" as const, label: labels.certTab, icon: AcademicCap },
      ] as tab (tab.id)}
        {@const Icon = tab.icon}
        <button
          type="button"
          onclick={() => (activeTab = tab.id)}
          class={`inline-flex items-center gap-1.5 rounded-md px-3 py-1.5 text-xs font-medium transition-colors ${
            activeTab === tab.id
              ? "bg-secondary text-foreground font-semibold"
              : "text-muted-foreground hover:bg-muted/50 hover:text-foreground"
          }`}
        >
          <Icon size={13} class={activeTab === tab.id ? "text-primary" : ""} />
          <span>{tab.label}</span>
        </button>
      {/each}
    </nav>

    <!-- Right Contextual Filters & Actions -->
    <div class="flex flex-wrap items-center gap-2 print:hidden">
      {#if activeTab === "term"}
        <Select
          type="single"
          value={selectedTermPeriodId}
          onValueChange={(val) => {
            if (val) selectedTermPeriodId = val;
          }}
        >
          <SelectTrigger class="h-8 min-w-44 text-xs">
            {periodNames.get(selectedTermPeriodId) ??
              m.student_grades_select_term()}
          </SelectTrigger>
          <SelectContent class="text-xs">
            {#each academicContext.allPeriods as period (period.id)}
              <SelectItem value={period.id}>
                {isRtl ? period.name_ar || period.name : period.name}
              </SelectItem>
            {/each}
          </SelectContent>
        </Select>

        <Button
          variant="outline"
          size="sm"
          class="h-8 gap-1.5 text-xs"
          onclick={() =>
            openPrint(
              "rep.academic.student-term-report-card",
              m.student_grades_term_report_print_title(),
            )}
        >
          <Printer size={13} />
          <span>{m.student_grades_print_term_report()}</span>
        </Button>
      {:else if activeTab === "annual" || activeTab === "certificate"}
        <Select
          type="single"
          value={selectedCalendarId}
          onValueChange={(val) => {
            if (val) selectedCalendarId = val;
          }}
        >
          <SelectTrigger class="h-8 min-w-40 text-xs">
            {yearNames.get(selectedCalendarId) ??
              m.student_grades_select_academic_year()}
          </SelectTrigger>
          <SelectContent class="text-xs">
            {#each academicContext.allYears as year (year.id)}
              <SelectItem value={year.id}>
                {isRtl ? year.name_ar || year.name : year.name}
              </SelectItem>
            {/each}
          </SelectContent>
        </Select>
      {/if}
    </div>
  </div>

  <!-- ═══════════════════════════════════════════════════════════ -->
  <!-- TAB 1: DETAILED ASSESSMENT RECORDS                          -->
  <!-- ═══════════════════════════════════════════════════════════ -->
  {#if activeTab === "records"}
    {#if gradesQuery.error}
      <div class="flex min-h-56 flex-col items-center justify-center gap-2 rounded-md border border-destructive/20 bg-destructive/5 p-6 text-center text-destructive">
        <InfoCircle class="h-6 w-6 opacity-80" />
        <p class="text-xs font-semibold">{m.common_load_error()}</p>
        <p class="text-[11px] text-muted-foreground">
          {extractErrorMessage(gradesQuery.error)}
        </p>
        <Button
          variant="outline"
          size="sm"
          class="mt-1 h-8 gap-1.5 text-xs"
          onclick={() => gradesQuery.refetch()}
        >
          <Refresh size={13} />
          {m.common_retry()}
        </Button>
      </div>
    {:else}
      <div class={recordRows.length === 0 ? "h-64 w-full" : "h-115 w-full"}>
        <OrbisGrid
          columns={recordColumns}
          source={recordRows}
          loading={gradesQuery.isLoading}
          clientQuery
          rtl={isRtl}
          emptyTitle={m.academics_grades_empty_title()}
          emptyDescription={m.academics_grades_empty_subtitle()}
          readonly
        >
          {#snippet toolbarActions()}
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
                    : periodNames.get(selectedPeriodId) ??
                      m.student_grades_select_period()}
                </SelectTrigger>
                <SelectContent class="text-xs">
                  <SelectItem value="all">
                    {m.student_grades_all_periods()}
                  </SelectItem>
                  {#each academicContext.allPeriods as period (period.id)}
                    <SelectItem value={period.id}>
                      {isRtl ? period.name_ar || period.name : period.name}
                    </SelectItem>
                  {/each}
                </SelectContent>
              </Select>

              <Button
                variant="ghost"
                size="icon"
                class="h-8 w-8"
                onclick={() => gradesQuery.refetch()}
                disabled={gradesQuery.isFetching}
                title={m.common_retry()}
              >
                {#if gradesQuery.isFetching}
                  <LoadingIndicator size={13} />
                {:else}
                  <Refresh size={13} />
                {/if}
              </Button>
            </div>
          {/snippet}
        </OrbisGrid>
      </div>
    {/if}

  <!-- ═══════════════════════════════════════════════════════════ -->
  <!-- TAB 2: TERM / SEMESTER / MODULE REPORT                      -->
  <!-- ═══════════════════════════════════════════════════════════ -->
  {:else if activeTab === "term"}
    {#if termReportQuery.isLoading}
      <div class="flex min-h-56 flex-col items-center justify-center gap-2 rounded-md border border-border bg-card p-8">
        <LoadingIndicator size={22} />
        <span class="text-xs text-muted-foreground">{m.common_loading_data()}</span>
      </div>
    {:else if termReportQuery.error}
      <div class="flex min-h-56 flex-col items-center justify-center gap-2 rounded-md border border-destructive/20 bg-destructive/5 p-8 text-center text-destructive">
        <InfoCircle class="h-6 w-6 opacity-80" />
        <p class="text-xs font-semibold">{m.common_load_error()}</p>
        <p class="text-[11px] text-muted-foreground">
          {extractErrorMessage(termReportQuery.error)}
        </p>
      </div>
    {:else if !termReportQuery.data || termReportQuery.data.subjects.length === 0}
      <div class="flex min-h-56 flex-col items-center justify-center gap-1 rounded-md border border-border bg-card p-8 text-center">
        <p class="text-xs font-semibold text-foreground">{m.academics_grades_empty_title()}</p>
        <p class="text-[11px] text-muted-foreground">{m.academics_grades_empty_subtitle()}</p>
      </div>
    {:else}
      {@const report = termReportQuery.data}
      {@const statusInfo = getStatusInfo(report.status)}
      {@const sample = report.subjects[0]}

      <!-- Unified Summary Strip -->
      <div class="grid grid-cols-2 divide-y divide-border rounded-md border border-border bg-card sm:grid-cols-4 sm:divide-x sm:divide-y-0 rtl:sm:divide-x-reverse">
        <div class="flex flex-col gap-1 px-4 py-3">
          <span class="text-[11px] text-muted-foreground">{m.student_grades_total_score()}</span>
          <span class="text-sm font-semibold tabular-nums text-foreground">
            {formatDecimal(Number(report.total_score), 1)}
            <span class="text-xs font-normal text-muted-foreground">/ {formatNumber(Number(report.max_possible_score))}</span>
          </span>
        </div>

        <div class="flex flex-col gap-1 px-4 py-3">
          <span class="text-[11px] text-muted-foreground">{m.student_grades_percentage()}</span>
          <span class="text-sm font-semibold tabular-nums text-foreground">
            {formatPercent(Number(report.overall_percentage))}
          </span>
        </div>

        <div class="flex flex-col gap-1 px-4 py-3">
          <span class="text-[11px] text-muted-foreground">
            {report.gpa != null ? labels.gpa : m.student_grades_appreciation_label()}
          </span>
          <span class="text-sm font-semibold tabular-nums text-foreground">
            {report.gpa != null ? formatDecimal(Number(report.gpa), 2) : getLetterDisplay(report)}
          </span>
        </div>

        <div class="flex flex-col gap-1 px-4 py-3">
          <span class="text-[11px] text-muted-foreground">{m.student_grades_academic_decision()}</span>
          <div class="flex items-center gap-2">
            <Badge variant="outline" class={`text-[10px] font-semibold ${statusInfo.badgeClass}`}>
              {statusInfo.label}
            </Badge>
            {#if report.rank_in_class}
              <span class="text-[11px] tabular-nums text-muted-foreground">
                #{formatNumber(report.rank_in_class)}
              </span>
            {/if}
          </div>
        </div>
      </div>

      <!-- Clean Subject Results Table -->
      <div class="overflow-x-auto rounded-md border border-border bg-card">
        <table class="w-full text-xs text-start">
          <thead class="border-b border-border bg-muted/30 text-[11px] font-medium text-muted-foreground">
            <tr>
              <th class="px-4 py-2.5 text-start">{labels.course}</th>
              {#if showCredits}
                <th class="px-3 py-2.5 text-center">{labels.credits}</th>
              {/if}
              <th class="px-3 py-2.5 text-center">
                {labels.coursework}{maxLabel(sample?.monthly_contribution_max)}
              </th>
              <th class="px-3 py-2.5 text-center">
                {labels.finalExam}{maxLabel(sample?.term_final_exam_max)}
              </th>
              <th class="px-3 py-2.5 text-center font-semibold text-foreground">
                {labels.termTotal}{maxLabel(sample?.term_max)}
              </th>
              <th class="px-3 py-2.5 text-center">{m.student_grades_percentage()}</th>
              <th class="px-3 py-2.5 text-center">{m.student_grades_grade()}</th>
              <th class="px-3 py-2.5 text-center">{m.student_grades_status()}</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-border/60">
            {#each report.subjects as subj (subj.subject_id)}
              {@const subjStatus = getStatusInfo(subj.status)}
              <tr class="transition-colors hover:bg-muted/20">
                <td class="px-4 py-2.5 font-medium text-foreground">
                  {isRtl ? subj.subject_name_ar || subj.subject_name : subj.subject_name}
                </td>
                {#if showCredits}
                  <td class="px-3 py-2.5 text-center tabular-nums text-muted-foreground">
                    {formatNumber(Number(subj.credit_hours))}
                  </td>
                {/if}
                <td class="px-3 py-2.5 text-center tabular-nums text-muted-foreground">
                  {formatDecimal(Number(subj.monthly_contribution), 1)}
                </td>
                <td class="px-3 py-2.5 text-center tabular-nums text-muted-foreground">
                  {subj.term_final_exam != null ? formatDecimal(Number(subj.term_final_exam), 1) : "—"}
                </td>
                <td class="px-3 py-2.5 text-center font-semibold tabular-nums text-foreground">
                  {formatDecimal(Number(subj.term_total), 1)}
                </td>
                <td class="px-3 py-2.5 text-center tabular-nums text-muted-foreground">
                  {formatPercent(Number(subj.percentage))}
                </td>
                <td class="px-3 py-2.5 text-center font-medium text-foreground">
                  {subj.letter_grade ?? "—"}
                </td>
                <td class="px-3 py-2.5 text-center">
                  <Badge variant="outline" class={`text-[10px] font-medium ${subjStatus.badgeClass}`}>
                    {subjStatus.label}
                  </Badge>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {/if}

  <!-- ═══════════════════════════════════════════════════════════ -->
  <!-- TAB 3: ANNUAL / CUMULATIVE RESULTS                          -->
  <!-- ═══════════════════════════════════════════════════════════ -->
  {:else if activeTab === "annual"}
    {#if annualReportQuery.isLoading}
      <div class="flex min-h-56 flex-col items-center justify-center gap-2 rounded-md border border-border bg-card p-8">
        <LoadingIndicator size={22} />
        <span class="text-xs text-muted-foreground">{m.common_loading_data()}</span>
      </div>
    {:else if annualReportQuery.error}
      <div class="flex min-h-56 flex-col items-center justify-center gap-2 rounded-md border border-destructive/20 bg-destructive/5 p-8 text-center text-destructive">
        <InfoCircle class="h-6 w-6 opacity-80" />
        <p class="text-xs font-semibold">{m.common_load_error()}</p>
        <p class="text-[11px] text-muted-foreground">
          {extractErrorMessage(annualReportQuery.error)}
        </p>
      </div>
    {:else if !annualReportQuery.data || annualReportQuery.data.subjects.length === 0}
      <div class="flex min-h-56 flex-col items-center justify-center gap-1 rounded-md border border-border bg-card p-8 text-center">
        <p class="text-xs font-semibold text-foreground">{m.academics_grades_empty_title()}</p>
        <p class="text-[11px] text-muted-foreground">{m.academics_grades_empty_subtitle()}</p>
      </div>
    {:else}
      {@const report = annualReportQuery.data}
      {@const statusInfo = getStatusInfo(report.status)}
      {@const sample = report.subjects[0]}

      <!-- Unified Summary Strip -->
      <div class="grid grid-cols-2 divide-y divide-border rounded-md border border-border bg-card sm:grid-cols-4 sm:divide-x sm:divide-y-0 rtl:sm:divide-x-reverse">
        <div class="flex flex-col gap-1 px-4 py-3">
          <span class="text-[11px] text-muted-foreground">{m.student_grades_annual_total()}</span>
          <span class="text-sm font-semibold tabular-nums text-foreground">
            {formatDecimal(Number(report.total_score), 1)}
            <span class="text-xs font-normal text-muted-foreground">/ {formatNumber(Number(report.max_possible_score))}</span>
          </span>
        </div>

        <div class="flex flex-col gap-1 px-4 py-3">
          <span class="text-[11px] text-muted-foreground">{m.student_grades_annual_percentage()}</span>
          <span class="text-sm font-semibold tabular-nums text-foreground">
            {formatPercent(Number(report.overall_percentage))}
          </span>
        </div>

        <div class="flex flex-col gap-1 px-4 py-3">
          <span class="text-[11px] text-muted-foreground">
            {report.cumulative_gpa != null ? labels.cumGpa : m.student_grades_appreciation_label()}
          </span>
          <span class="text-sm font-semibold tabular-nums text-foreground">
            {report.cumulative_gpa != null ? formatDecimal(Number(report.cumulative_gpa), 2) : getLetterDisplay(report)}
          </span>
        </div>

        <div class="flex flex-col gap-1 px-4 py-3">
          <span class="text-[11px] text-muted-foreground">{m.student_grades_promotion_status()}</span>
          <div class="flex items-center gap-2">
            <Badge variant="outline" class={`text-[10px] font-semibold ${statusInfo.badgeClass}`}>
              {statusInfo.label}
            </Badge>
            {#if report.rank_in_class}
              <span class="text-[11px] tabular-nums text-muted-foreground">
                #{formatNumber(report.rank_in_class)}
              </span>
            {/if}
          </div>
        </div>
      </div>

      <!-- Clean Annual Table -->
      <div class="overflow-x-auto rounded-md border border-border bg-card">
        <table class="w-full text-xs text-start">
          <thead class="border-b border-border bg-muted/30 text-[11px] font-medium text-muted-foreground">
            <tr>
              <th class="px-4 py-2.5 text-start">{labels.course}</th>
              {#if showCredits}
                <th class="px-3 py-2.5 text-center">{labels.credits}</th>
              {/if}
              <th class="px-3 py-2.5 text-center">
                {labels.term1}{maxLabel(sample?.term_1_result?.term_max)}
              </th>
              <th class="px-3 py-2.5 text-center">
                {labels.term2}{maxLabel(sample?.term_2_result?.term_max)}
              </th>
              <th class="px-3 py-2.5 text-center font-semibold text-foreground">
                {m.student_grades_annual_total()}{maxLabel(sample?.annual_max)}
              </th>
              <th class="px-3 py-2.5 text-center">{m.student_grades_percentage()}</th>
              <th class="px-3 py-2.5 text-center">{m.student_grades_grade()}</th>
              <th class="px-3 py-2.5 text-center">{m.student_grades_status()}</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-border/60">
            {#each report.subjects as subj (subj.subject_id)}
              {@const subjStatus = getStatusInfo(subj.status)}
              <tr class="transition-colors hover:bg-muted/20">
                <td class="px-4 py-2.5 font-medium text-foreground">
                  {isRtl ? subj.subject_name_ar || subj.subject_name : subj.subject_name}
                </td>
                {#if showCredits}
                  <td class="px-3 py-2.5 text-center tabular-nums text-muted-foreground">
                    {formatNumber(Number(subj.credit_hours))}
                  </td>
                {/if}
                <td class="px-3 py-2.5 text-center tabular-nums text-muted-foreground">
                  {subj.term_1_result ? formatDecimal(Number(subj.term_1_result.term_total), 1) : "—"}
                </td>
                <td class="px-3 py-2.5 text-center tabular-nums text-muted-foreground">
                  {subj.term_2_result ? formatDecimal(Number(subj.term_2_result.term_total), 1) : "—"}
                </td>
                <td class="px-3 py-2.5 text-center font-semibold tabular-nums text-foreground">
                  {formatDecimal(Number(subj.annual_total), 1)}
                </td>
                <td class="px-3 py-2.5 text-center tabular-nums text-muted-foreground">
                  {formatPercent(Number(subj.percentage))}
                </td>
                <td class="px-3 py-2.5 text-center font-medium text-foreground">
                  {subj.letter_grade ?? "—"}
                </td>
                <td class="px-3 py-2.5 text-center">
                  <Badge variant="outline" class={`text-[10px] font-medium ${subjStatus.badgeClass}`}>
                    {subjStatus.label}
                  </Badge>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {/if}

  <!-- ═══════════════════════════════════════════════════════════ -->
  <!-- TAB 4: CERTIFICATES & OFFICIAL DOCUMENTS                    -->
  <!-- ═══════════════════════════════════════════════════════════ -->
  {:else if activeTab === "certificate"}
    <div class="flex flex-col gap-5">
      <!-- Available Official Documents List -->
      <div class="overflow-hidden rounded-md border border-border bg-card">
        <div class="border-b border-border bg-muted/30 px-4 py-2.5">
          <h3 class="text-xs font-semibold text-foreground">
            {m.student_grades_official_documents_heading()}
          </h3>
        </div>

        {#if reportDefsQuery.isLoading}
          <div class="flex items-center justify-center gap-2 p-6">
            <LoadingIndicator size={16} />
            <span class="text-xs text-muted-foreground">{m.common_loading_data()}</span>
          </div>
        {:else if studentDocuments.length === 0}
          <div class="p-6 text-center text-xs text-muted-foreground">
            {m.student_grades_no_certificate_templates()}
          </div>
        {:else}
          <div class="divide-y divide-border/60">
            {#each studentDocuments as doc (doc.code)}
              {@const docTitle = isRtl ? doc.name_ar || doc.name : doc.name}
              {@const docDesc = isRtl ? doc.description_ar || doc.description : doc.description}
              <div class="flex flex-wrap items-center justify-between gap-3 px-4 py-3 transition-colors hover:bg-muted/20">
                <div class="min-w-0 flex-1">
                  <p class="text-xs font-medium text-foreground">{docTitle}</p>
                  {#if docDesc}
                    <p class="mt-0.5 truncate text-[11px] text-muted-foreground">{docDesc}</p>
                  {/if}
                </div>
                <Button
                  variant="outline"
                  size="sm"
                  class="h-8 gap-1.5 text-xs"
                  onclick={() => openPrint(doc.code, docTitle)}
                >
                  <Printer size={13} />
                  <span>{m.student_grades_preview_and_print()}</span>
                </Button>
              </div>
            {/each}
          </div>
        {/if}
      </div>

      <!-- Certificate Results Summary (only shown when subjects exist) -->
      {#if certificateReportQuery.data && certificateReportQuery.data.subjects.length > 0}
        {@const certReport = certificateReportQuery.data}
        {@const sample = certReport.subjects[0]}
        <div class="overflow-x-auto rounded-md border border-border bg-card">
          <table class="w-full text-xs text-start">
            <thead class="border-b border-border bg-muted/30 text-[11px] font-medium text-muted-foreground">
              <tr>
                <th class="px-4 py-2.5 text-start">{labels.course}</th>
                <th class="px-3 py-2.5 text-center">
                  {labels.finalExam}{maxLabel(sample?.term_final_exam_max)}
                </th>
                <th class="px-3 py-2.5 text-center">
                  {m.student_grades_cumulative()}{maxLabel(sample?.monthly_contribution_max)}
                </th>
                <th class="px-3 py-2.5 text-center font-semibold text-foreground">
                  {labels.termTotal}{maxLabel(sample?.term_max)}
                </th>
                <th class="px-3 py-2.5 text-center">
                  {m.student_grades_min_pass()}
                </th>
                <th class="px-3 py-2.5 text-center">{m.student_grades_percentage()}</th>
                <th class="px-3 py-2.5 text-center">{m.student_grades_status()}</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-border/60">
              {#each certReport.subjects as subj (subj.subject_id)}
                {@const subjStatus = getStatusInfo(subj.status)}
                <tr class="transition-colors hover:bg-muted/20">
                  <td class="px-4 py-2.5 font-medium text-foreground">
                    {isRtl ? subj.subject_name_ar || subj.subject_name : subj.subject_name}
                  </td>
                  <td class="px-3 py-2.5 text-center tabular-nums text-muted-foreground">
                    {subj.term_final_exam != null ? formatDecimal(Number(subj.term_final_exam), 1) : "—"}
                  </td>
                  <td class="px-3 py-2.5 text-center tabular-nums text-muted-foreground">
                    {formatDecimal(Number(subj.monthly_contribution), 1)}
                  </td>
                  <td class="px-3 py-2.5 text-center font-semibold tabular-nums text-foreground">
                    {formatDecimal(Number(subj.term_total), 1)}
                  </td>
                  <td class="px-3 py-2.5 text-center tabular-nums text-muted-foreground">
                    {formatDecimal((Number(subj.term_max) * passingThreshold) / 100, 1)}
                  </td>
                  <td class="px-3 py-2.5 text-center tabular-nums text-muted-foreground">
                    {formatPercent(Number(subj.percentage))}
                  </td>
                  <td class="px-3 py-2.5 text-center">
                    <Badge variant="outline" class={`text-[10px] font-medium ${subjStatus.badgeClass}`}>
                      {subjStatus.label}
                    </Badge>
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      {/if}
    </div>
  {/if}
</div>

<PdfReportViewer
  bind:open={showPrintDialog}
  templateId={printReportCode}
  parameters={{
    student_id: studentId,
    calendar_id: selectedCalendarId,
    year_id: selectedCalendarId,
    period_id: selectedTermPeriodId,
    class_id: classId,
  }}
  reportTitle={printTitle}
/>
