<!--
  @component StudentGrades
  Comprehensive student subject grades, term report card (50 pts: 20 work + 30 exam),
  annual certificate (100 pts), and rank in class/grade analytics.
-->
<script lang="ts">
  import { Skeleton } from "@orbis/ui/skeleton";
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import {
  api,
  type GradeRecordDto,
  type CumulativeGpaRecordDto,
  type StudentTermReportCard,
  type StudentAnnualReportCard,
} from "$lib/api";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardAction,
  CardContent,
} from "@orbis/ui/card";
import {
  AcademicCap,
  Calendar,
  ChartBar,
  Refresh,
  BookOpen,
} from "@orbis/icons";
import { onMount } from "svelte";
import { getLocale } from "$lib/paraglide/runtime";
import { formatDate } from "$lib/core/i18n";
import { OrbisGrid, type OrbisColumnDef } from "@orbis/svelte-grid";
import { academicContext } from "$lib/stores";
import { FilterTrigger, type UniversalFilter } from "@orbis/ui/filter-builder";
import { gradesFilterFields } from "$lib/core/filter-configs";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";

let { orgId, studentId, classId }: { orgId: string; studentId: string; classId?: string } = $props();

type ViewMode = "term" | "annual" | "raw";
let viewMode = $state<ViewMode>("term");

// Data states
let termReport = $state<StudentTermReportCard | null>(null);
let annualReport = $state<StudentAnnualReportCard | null>(null);
let gpaReport = $state<CumulativeGpaRecordDto | null>(null);
let rawGrades = $state<GradeRecordDto[]>([]);
let isLoading = $state(true);

// Period & Year Selection
let selectedPeriodId = $state<string>("");
let selectedYearId = $state<string>("");

$effect(() => {
  if (!selectedPeriodId && academicContext.effectivePeriodId) {
    selectedPeriodId = academicContext.effectivePeriodId;
  }
  if (!selectedYearId && academicContext.currentYear?.id) {
    selectedYearId = academicContext.currentYear.id;
  }
});

onMount(() => {
  loadData();
});

async function loadData() {
  if (!orgId || !studentId) return;
  isLoading = true;
  try {
    const pId = selectedPeriodId || academicContext.effectivePeriodId;
    const yId = selectedYearId || academicContext.currentYear?.id;

    const [termRes, annualRes, gpaRes, rawRes] = await Promise.all([
      pId
        ? api.getStudentTermResults(studentId, pId, classId).catch(() => null)
        : Promise.resolve(null),
      yId
        ? api.getStudentAnnualResults(studentId, yId, classId).catch(() => null)
        : Promise.resolve(null),
      pId
        ? api.calculateGpa({ student_id: studentId, period_id: pId }).catch(() => null)
        : Promise.resolve(null),
      api.listStudentGrades(studentId).catch(() => []),
    ]);

    termReport = termRes;
    annualReport = annualRes;
    gpaReport = gpaRes;
    rawGrades = rawRes || [];
  } catch (err) {
    console.error("Failed to load student assessment data:", err);
  } finally {
    isLoading = false;
  }
}

function handlePrint() {
  window.print();
}

function getStatusBadgeVariant(status?: string): "default" | "secondary" | "destructive" | "outline" {
  switch (status) {
    case "honors":
    case "passed":
      return "default";
    case "conditional_pass":
      return "secondary";
    case "failed":
    case "dismissed":
      return "destructive";
    default:
      return "outline";
  }
}

function getStatusLabel(status?: string): string {
  switch (status) {
    case "honors":
      return m.ui_passed_with_honors_1ba26b();
    case "passed":
      return m.ui_passed_93d5f7();
    case "conditional_pass":
      return m.ui_conditional_pass_b47e36();
    case "failed":
      return m.ui_failed_re_exam_e4d5f6();
    case "withdrawn":
      return m.ui_withdrawn_ef2e04();
    default:
      return status || "—";
  }
}

// Raw grades grid
let activeFilter = $state<UniversalFilter>({
  logic: "AND",
  conditions: [],
  groups: [],
});

const gridSource = $derived(
  rawGrades.map((g, i: number) => ({
    ...g,
    __idx: i + 1,
    subject_name: g.class_subject_id || "—",
    exam_type: g.assessment_id || m.academics_grades_default_assessment(),
    score: Number(g.score || 0),
    max_score: Number(g.max_score || 100),
    percentage: Number(g.percentage || 0),
    grade_letter: g.grade_letter || "—",
  })),
);

const columns = $derived.by<OrbisColumnDef[]>(() => [
  { id: "__idx", prop: "__idx", header: "#", name: "#", width: 48, size: 48, readonly: true, pin: "left", align: "center" },
  { id: "subject_name", prop: "subject_name", header: m.common_word_subject(), name: m.common_word_subject(), width: 200, size: 200, readonly: true, pin: "left", cellClass: "font-semibold text-foreground" },
  { id: "exam_type", prop: "exam_type", header: m.academics_grades_col_assessment(), name: m.academics_grades_col_assessment(), width: 140, size: 140, readonly: true },
  { id: "score", prop: "score", header: m.academics_grades_col_score(), name: m.academics_grades_col_score(), width: 90, size: 90, readonly: true, align: "end", type: "number", cellClass: "font-mono font-bold" },
  { id: "max_score", prop: "max_score", header: "Max", name: "Max", width: 80, size: 80, readonly: true, align: "end", type: "number", cellClass: "font-mono text-muted-foreground" },
  { id: "percentage", prop: "percentage", header: m.academics_grades_col_indicator(), name: m.academics_grades_col_indicator(), width: 100, size: 100, readonly: true, align: "end", type: "percentage", cellClass: "font-mono font-medium" },
  { id: "grade_letter", prop: "grade_letter", header: "Grade", name: "Grade", width: 90, size: 90, readonly: true, align: "center", type: "badge" },
]);
</script>

<div class="flex flex-col gap-6 text-xs text-start font-sans print:p-0">
  <!-- View Mode Switcher & Filter Header (Hidden in Print) -->
  <div class="flex flex-wrap items-center justify-between gap-3 pb-2 border-b border-border select-none print:hidden">
    <div class="flex items-center gap-1.5 p-1 bg-muted/60 rounded-lg">
      <Button
        variant={viewMode === "term" ? "default" : "ghost"}
        size="sm"
        class="h-7 text-xs px-3"
        onclick={() => { viewMode = "term"; loadData(); }}
      >
        {m.ui_term_report_card_50_pts_b8158a()}
      </Button>
      <Button
        variant={viewMode === "annual" ? "default" : "ghost"}
        size="sm"
        class="h-7 text-xs px-3"
        onclick={() => { viewMode = "annual"; loadData(); }}
      >
        {m.ui_annual_certificate_100_pts_0f5015()}
      </Button>
      <Button
        variant={viewMode === "raw" ? "default" : "ghost"}
        size="sm"
        class="h-7 text-xs px-3"
        onclick={() => { viewMode = "raw"; loadData(); }}
      >
        {m.ui_detailed_entries_1c131e()}
      </Button>
    </div>

    <div class="flex items-center gap-2">
      {#if viewMode === "term"}
        <Select
          type="single"
          value={selectedPeriodId}
          onValueChange={(val) => { if (val) { selectedPeriodId = val; loadData(); } }}
        >
          <SelectTrigger class="h-8 text-xs min-w-[160px]">
            {academicContext.allPeriods.find((p) => p.id === selectedPeriodId)?.name || m.ui_select_term_e8d1a7()}
          </SelectTrigger>
          <SelectContent class="text-xs">
            {#each academicContext.allPeriods as period}
              <SelectItem value={period.id}>{period.name_ar || period.name}</SelectItem>
            {/each}
          </SelectContent>
        </Select>
      {:else if viewMode === "annual"}
        <Select
          type="single"
          value={selectedYearId}
          onValueChange={(val) => { if (val) { selectedYearId = val; loadData(); } }}
        >
          <SelectTrigger class="h-8 text-xs min-w-[160px]">
            {academicContext.allYears.find((y) => y.id === selectedYearId)?.name || m.ui_select_year_c61be2()}
          </SelectTrigger>
          <SelectContent class="text-xs">
            {#each academicContext.allYears as yr}
              <SelectItem value={yr.id}>{yr.name_ar || yr.name}</SelectItem>
            {/each}
          </SelectContent>
        </Select>
      {/if}

      <Button
        variant="outline"
        size="sm"
        class="h-8 text-xs px-3 gap-1.5"
        onclick={handlePrint}
      >
        <span>🖨️</span>
        {m.ui_print_card_e8f9cc()}
      </Button>

      <Button
        variant="ghost"
        size="icon-sm"
        class="h-8 w-8 text-muted-foreground hover:text-foreground"
        onclick={loadData}
        title={m.academics_grades_refresh()}
      >
        {#if isLoading}<LoadingIndicator size={13} />{:else}<Refresh size={13} />{/if}
      </Button>
    </div>
  </div>

  {#if isLoading}
    <div class="space-y-4" aria-busy="true">
      <span class="sr-only">{m.ui_loading_assessment_data_6e49c7()}</span>
      <div class="grid grid-cols-2 gap-3 md:grid-cols-4 print:grid-cols-4">
        {#each Array.from({ length: 4 }) as _, index (index)}
          <div class="flex flex-col gap-3 rounded-lg border border-border/80 bg-card p-4">
            <Skeleton class="h-3 w-24" />
            <Skeleton class="h-7 w-20" />
            <Skeleton class="h-3 w-32 max-w-full" />
          </div>
        {/each}
      </div>
      <div class="overflow-hidden rounded-lg border border-border/80 bg-card shadow-xs">
        <div class="flex items-center justify-between border-b border-border/60 p-4">
          <div class="flex flex-col gap-2">
            <Skeleton class="h-5 w-40" />
            <Skeleton class="h-3 w-56 max-w-full" />
          </div>
          <Skeleton class="h-8 w-24" />
        </div>
        <div class="space-y-3 p-4">
          {#each Array.from({ length: 8 }) as _, rowIndex (rowIndex)}
            <div class="flex items-center gap-4">
              {#each Array.from({ length: 5 }) as _, columnIndex (columnIndex)}
                <Skeleton class="h-4 flex-1" />
              {/each}
            </div>
          {/each}
        </div>
      </div>
    </div>
  {:else if viewMode === "term"}
    <!-- TERM REPORT CARD VIEW -->
    {#if termReport}
      <!-- High-Level KPI Summary -->
      <div class="grid grid-cols-2 md:grid-cols-4 gap-3 print:grid-cols-4">
        <Card class="p-4 flex flex-col gap-1 border-border/80">
          <span class="text-[10px] text-muted-foreground font-semibold uppercase">{m.ui_total_score_502e35()}</span>
          <div class="flex items-baseline gap-1">
            <span class="text-xl font-bold font-mono text-primary">{termReport.total_score}</span>
            <span class="text-xs text-muted-foreground font-mono">/ {termReport.max_possible_score}</span>
          </div>
        </Card>

        <Card class="p-4 flex flex-col gap-1 border-border/80">
          <span class="text-[10px] text-muted-foreground font-semibold uppercase">{m.ui_percentage_0c2a56()}</span>
          <span class="text-xl font-bold font-mono text-foreground">{termReport.overall_percentage.toFixed(1)}%</span>
        </Card>

        <Card class="p-4 flex flex-col gap-1 border-border/80">
          <span class="text-[10px] text-muted-foreground font-semibold uppercase">{m.ui_rank_in_class_d68b49()}</span>
          <div class="flex items-baseline gap-1">
            <span class="text-xl font-bold font-mono text-primary">
              {termReport.rank_in_class ? `#${termReport.rank_in_class}` : "—"}
            </span>
            <span class="text-xs text-muted-foreground font-mono">/ {termReport.total_students_in_class}</span>
          </div>
        </Card>

        <Card class="p-4 flex flex-col gap-1 border-border/80">
          <span class="text-[10px] text-muted-foreground font-semibold uppercase">{m.ui_status_89fb2f()}</span>
          <div class="mt-1">
            <Badge variant={getStatusBadgeVariant(termReport.status)}>
              {getStatusLabel(termReport.status)}
            </Badge>
          </div>
        </Card>
      </div>

      <!-- Formal Term Breakdown Card -->
      <Card class="border-border/80 overflow-hidden shadow-xs">
        <CardHeader class="py-3 px-4 border-b border-border bg-muted/20">
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-2">
              <AcademicCap size={16} class="text-primary" />
              <CardTitle class="text-xs font-bold text-foreground">
                {m.ui_term_grade_sheet_standard_50_poi_7b9407()}
              </CardTitle>
            </div>
            <span class="text-[11px] font-mono text-muted-foreground">
              {termReport.period_name} | {termReport.class_name}
            </span>
          </div>
        </CardHeader>

        <CardContent class="p-0 overflow-x-auto">
          <table class="w-full text-xs text-start border-collapse">
            <thead>
              <tr class="bg-muted/40 text-muted-foreground border-b border-border text-[11px] font-semibold">
                <th class="py-2.5 px-3 text-start w-10">#</th>
                <th class="py-2.5 px-3 text-start min-w-[180px]">{m.ui_subject_0949f5()}</th>
                <th class="py-2.5 px-3 text-center">{m.ui_monthly_work_20_e3e4cb()}</th>
                <th class="py-2.5 px-3 text-center">{m.ui_term_exam_30_27e258()}</th>
                <th class="py-2.5 px-3 text-center">{m.ui_term_total_50_5a76ed()}</th>
                <th class="py-2.5 px-3 text-center">{m.ui_percentage_ae5fd2()}</th>
                <th class="py-2.5 px-3 text-center">{m.ui_grade_709612()}</th>
                <th class="py-2.5 px-3 text-center">{m.ui_status_9492be()}</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-border/60">
              {#each termReport.subjects as subj, idx}
                <tr class="hover:bg-muted/20 transition-colors">
                  <td class="py-2.5 px-3 font-mono text-muted-foreground">{idx + 1}</td>
                  <td class="py-2.5 px-3 font-semibold text-foreground">
                    {subj.subject_name_ar || subj.subject_name}
                  </td>
                  <td class="py-2.5 px-3 text-center font-mono font-medium">
                    {subj.monthly_contribution} <span class="text-[10px] text-muted-foreground">/ {subj.monthly_contribution_max}</span>
                  </td>
                  <td class="py-2.5 px-3 text-center font-mono font-medium">
                    {subj.term_final_exam != null ? subj.term_final_exam : "—"} <span class="text-[10px] text-muted-foreground">/ {subj.term_final_exam_max}</span>
                  </td>
                  <td class="py-2.5 px-3 text-center font-mono font-bold text-primary">
                    {subj.term_total} <span class="text-[10px] text-muted-foreground">/ {subj.term_max}</span>
                  </td>
                  <td class="py-2.5 px-3 text-center font-mono">
                    {subj.percentage.toFixed(1)}%
                  </td>
                  <td class="py-2.5 px-3 text-center">
                    <span class="inline-block px-2 py-0.5 rounded text-[10px] font-bold bg-muted font-mono">
                      {subj.letter_grade || "—"}
                    </span>
                  </td>
                  <td class="py-2.5 px-3 text-center">
                    <Badge variant={getStatusBadgeVariant(subj.status)}>
                      {getStatusLabel(subj.status)}
                    </Badge>
                  </td>
                </tr>
              {/each}
            </tbody>
            <tfoot>
              <tr class="bg-muted/30 font-bold border-t-2 border-border">
                <td colspan="2" class="py-3 px-3 text-start">
                  {m.ui_overall_grand_total_average_a199a7()}
                </td>
                <td colspan="2" class="py-3 px-3 text-center text-muted-foreground font-mono">
                  {termReport.total_score} / {termReport.max_possible_score}
                </td>
                <td class="py-3 px-3 text-center font-mono text-primary text-sm">
                  {termReport.total_score}
                </td>
                <td class="py-3 px-3 text-center font-mono text-sm">
                  {termReport.overall_percentage.toFixed(1)}%
                </td>
                <td colspan="2" class="py-3 px-3 text-center">
                  <Badge variant={getStatusBadgeVariant(termReport.status)}>
                    {getStatusLabel(termReport.status)}
                  </Badge>
                </td>
              </tr>
            </tfoot>
          </table>
        </CardContent>
      </Card>
    {:else}
      <div class="py-12 text-center text-muted-foreground border border-dashed border-border rounded-xl">
        <p>{m.ui_no_term_results_recorded_for_thi_c18ffe()}</p>
      </div>
    {/if}

  {:else if viewMode === "annual"}
    <!-- ANNUAL REPORT CARD VIEW -->
    {#if annualReport}
      <!-- High-Level KPI Summary -->
      <div class="grid grid-cols-2 md:grid-cols-4 gap-3 print:grid-cols-4">
        <Card class="p-4 flex flex-col gap-1 border-border/80">
          <span class="text-[10px] text-muted-foreground font-semibold uppercase">{m.ui_annual_total_c48419()}</span>
          <div class="flex items-baseline gap-1">
            <span class="text-xl font-bold font-mono text-primary">{annualReport.total_score}</span>
            <span class="text-xs text-muted-foreground font-mono">/ {annualReport.max_possible_score}</span>
          </div>
        </Card>

        <Card class="p-4 flex flex-col gap-1 border-border/80">
          <span class="text-[10px] text-muted-foreground font-semibold uppercase">{m.ui_overall_percentage_d8caae()}</span>
          <span class="text-xl font-bold font-mono text-foreground">{annualReport.overall_percentage.toFixed(1)}%</span>
        </Card>

        <Card class="p-4 flex flex-col gap-1 border-border/80">
          <span class="text-[10px] text-muted-foreground font-semibold uppercase">{m.ui_annual_rank_in_class_e940d7()}</span>
          <div class="flex items-baseline gap-1">
            <span class="text-xl font-bold font-mono text-primary">
              {annualReport.rank_in_class ? `#${annualReport.rank_in_class}` : "—"}
            </span>
            <span class="text-xs text-muted-foreground font-mono">/ {annualReport.total_students_in_class}</span>
          </div>
        </Card>

        <Card class="p-4 flex flex-col gap-1 border-border/80">
          <span class="text-[10px] text-muted-foreground font-semibold uppercase">{m.ui_final_decision_4f00d8()}</span>
          <div class="mt-1">
            <Badge variant={getStatusBadgeVariant(annualReport.status)}>
              {getStatusLabel(annualReport.status)}
            </Badge>
          </div>
        </Card>
      </div>

      <!-- Formal Annual Breakdown Card -->
      <Card class="border-border/80 overflow-hidden shadow-xs">
        <CardHeader class="py-3 px-4 border-b border-border bg-muted/20">
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-2">
              <AcademicCap size={16} class="text-primary" />
              <CardTitle class="text-xs font-bold text-foreground">
                {m.ui_annual_final_certificate_100_poi_9badb5()}
              </CardTitle>
            </div>
            <span class="text-[11px] font-mono text-muted-foreground">
              {annualReport.academic_year_name} | {annualReport.class_name}
            </span>
          </div>
        </CardHeader>

        <CardContent class="p-0 overflow-x-auto">
          <table class="w-full text-xs text-start border-collapse">
            <thead>
              <tr class="bg-muted/40 text-muted-foreground border-b border-border text-[11px] font-semibold">
                <th class="py-2.5 px-3 text-start w-10">#</th>
                <th class="py-2.5 px-3 text-start min-w-[180px]">{m.ui_subject_0949f5()}</th>
                <th class="py-2.5 px-3 text-center">{m.ui_term_1_50_adf4b5()}</th>
                <th class="py-2.5 px-3 text-center">{m.ui_term_2_50_43e373()}</th>
                <th class="py-2.5 px-3 text-center">{m.ui_annual_total_100_620c37()}</th>
                <th class="py-2.5 px-3 text-center">{m.ui_percentage_ae5fd2()}</th>
                <th class="py-2.5 px-3 text-center">{m.ui_grade_709612()}</th>
                <th class="py-2.5 px-3 text-center">{m.ui_status_9492be()}</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-border/60">
              {#each annualReport.subjects as subj, idx}
                <tr class="hover:bg-muted/20 transition-colors">
                  <td class="py-2.5 px-3 font-mono text-muted-foreground">{idx + 1}</td>
                  <td class="py-2.5 px-3 font-semibold text-foreground">
                    {subj.subject_name_ar || subj.subject_name}
                  </td>
                  <td class="py-2.5 px-3 text-center font-mono font-medium">
                    {subj.term_1_result ? subj.term_1_result.term_total : "—"}
                  </td>
                  <td class="py-2.5 px-3 text-center font-mono font-medium">
                    {subj.term_2_result ? subj.term_2_result.term_total : "—"}
                  </td>
                  <td class="py-2.5 px-3 text-center font-mono font-bold text-primary">
                    {subj.annual_total} <span class="text-[10px] text-muted-foreground">/ {subj.annual_max}</span>
                  </td>
                  <td class="py-2.5 px-3 text-center font-mono">
                    {subj.percentage.toFixed(1)}%
                  </td>
                  <td class="py-2.5 px-3 text-center">
                    <span class="inline-block px-2 py-0.5 rounded text-[10px] font-bold bg-muted font-mono">
                      {subj.letter_grade || "—"}
                    </span>
                  </td>
                  <td class="py-2.5 px-3 text-center">
                    <Badge variant={getStatusBadgeVariant(subj.status)}>
                      {getStatusLabel(subj.status)}
                    </Badge>
                  </td>
                </tr>
              {/each}
            </tbody>
            <tfoot>
              <tr class="bg-muted/30 font-bold border-t-2 border-border">
                <td colspan="2" class="py-3 px-3 text-start">
                  {m.ui_annual_grand_total_1be9f0()}
                </td>
                <td colspan="2" class="py-3 px-3 text-center text-muted-foreground font-mono">
                  {annualReport.total_score} / {annualReport.max_possible_score}
                </td>
                <td class="py-3 px-3 text-center font-mono text-primary text-sm">
                  {annualReport.total_score}
                </td>
                <td class="py-3 px-3 text-center font-mono text-sm">
                  {annualReport.overall_percentage.toFixed(1)}%
                </td>
                <td colspan="2" class="py-3 px-3 text-center">
                  <Badge variant={getStatusBadgeVariant(annualReport.status)}>
                    {getStatusLabel(annualReport.status)}
                  </Badge>
                </td>
              </tr>
            </tfoot>
          </table>
        </CardContent>
      </Card>
    {:else}
      <div class="py-12 text-center text-muted-foreground border border-dashed border-border rounded-xl">
        <p>{m.ui_no_completed_annual_results_for__bd2e81()}</p>
      </div>
    {/if}

  {:else}
    <!-- RAW DETAILED GRADES VIEW -->
    <Card class="overflow-hidden">
      <CardHeader class="py-3 px-4 flex flex-row items-center justify-between">
        <div class="flex flex-col text-start">
          <CardTitle class="text-xs font-bold uppercase tracking-wider text-foreground">
            {m.academics_grades_title()}
          </CardTitle>
          <CardDescription class="text-[10px]">
            {m.academics_grades_subtitle()}
          </CardDescription>
        </div>

        <CardAction>
          <div class="flex items-center gap-2">
            <FilterTrigger
              bind:filter={activeFilter}
              fields={gradesFilterFields}
            />
          </div>
        </CardAction>
      </CardHeader>

      <CardContent class="p-0">
        <div class="h-[350px] w-full">
          <OrbisGrid
            {columns}
            source={gridSource}
            loading={isLoading}
            clientQuery
            filter={activeFilter}
            emptyTitle={m.academics_grades_empty_title()}
            emptyDescription={m.academics_grades_empty_subtitle()}
            rtl={getLocale() === "ar"}
            readonly
          />
        </div>
      </CardContent>
    </Card>

    {#if gpaReport}
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
        <Card class="p-4 flex items-center justify-between">
          <div class="flex flex-col">
            <span class="text-[10px] text-muted-foreground font-semibold">{m.academics_grades_overall_grade()}</span>
            <span class="text-base font-bold font-mono text-primary mt-0.5">{gpaReport.gpa != null ? Number(gpaReport.gpa).toFixed(2) : "—"}</span>
          </div>
          <AcademicCap size={20} class="text-primary" />
        </Card>
        <Card class="p-4 flex items-center justify-between">
          <div class="flex flex-col">
            <span class="text-[10px] text-muted-foreground font-semibold">{m.academics_grades_last_update()}</span>
            <span class="text-xs font-semibold text-foreground mt-0.5">{gpaReport.calculated_at ? formatDate(gpaReport.calculated_at) : "—"}</span>
          </div>
          <Calendar size={20} class="text-muted-foreground" />
        </Card>
      </div>
    {/if}
  {/if}
</div>
