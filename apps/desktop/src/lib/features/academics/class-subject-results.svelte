<!--
  @component ClassSubjectResults
  Feature component displaying term results and calculated grades for a class subject.
  Aligns with the standard Yemeni model: 20 monthly work + 30 final exam = 50 term total.
-->
<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import { createQuery } from "@tanstack/svelte-query";
import { api, extractErrorMessage, type StudentTermReportCard, type GradeDto } from "$lib/api";
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
import { DocumentCheck, InfoCircle, Refresh } from "@orbis/icons";
import { academicContext } from "$lib/stores";
import * as m from "$lib/paraglide/messages";
import { OrbisGrid, type OrbisColumnDef } from "@orbis/svelte-grid";
import { getLocale } from "$lib/paraglide/runtime";

let { orgId, classId, classSubjectId }: { orgId: string; classId: string; classSubjectId: string } = $props();

const selectedPeriodId = $derived(academicContext.effectivePeriodId || "");

const resultsQuery = createQuery(() => ({
  queryKey: ["term-results-enriched", selectedPeriodId, classSubjectId, classId],
  queryFn: async () => {
    const [grades, classReports] = await Promise.all([
      api.listGrades({ classSubjectId }).catch(() => [] as GradeDto[]),
      selectedPeriodId && classId
        ? api.getClassTermResults(classId, selectedPeriodId).catch(() => [] as StudentTermReportCard[])
        : Promise.resolve([] as StudentTermReportCard[]),
    ]);
    return { grades, classReports };
  },
  enabled: !!selectedPeriodId && !!classSubjectId,
}));

const queryData = $derived(resultsQuery.data ?? { grades: [], classReports: [] });
const isLoading = $derived(resultsQuery.isLoading);
const error = $derived(resultsQuery.error);

interface EnrichedRow {
  student_id: string;
  student_name: string;
  student_number: string;
  monthly_score: number;
  monthly_max: number;
  exam_score: number | string;
  exam_max: number;
  term_total: number;
  term_max: number;
  percentage: number;
  letter_grade: string;
  status: string;
  rank_in_class: number | string;
}

const rows = $derived.by<EnrichedRow[]>(() => {
  const { grades, classReports } = queryData;

  // If we have calculated report cards for the class, map from report cards
  if (classReports && classReports.length > 0) {
    return classReports.map((report) => {
      // Find this subject in the student's report
      const subj = report.subjects.find((s) => s.subject_id === classSubjectId) || report.subjects[0];
      return {
        student_id: report.student_id,
        student_name: report.student_name,
        student_number: report.student_number || "—",
        monthly_score: subj?.monthly_contribution ?? 0,
        monthly_max: subj?.monthly_contribution_max ?? 20,
        exam_score: subj?.term_final_exam != null ? subj.term_final_exam : "—",
        exam_max: subj?.term_final_exam_max ?? 30,
        term_total: subj?.term_total ?? 0,
        term_max: subj?.term_max ?? 50,
        percentage: subj?.percentage ?? 0,
        letter_grade: subj?.letter_grade || "—",
        status: subj?.status === "passed" || subj?.status === "honors" ? "passed" : "failed",
        rank_in_class: report.rank_in_class ?? "—",
      };
    });
  }

  // Fallback to raw grades if calculation not run yet
  return (grades || []).map((g: any, i: number) => ({
    student_id: g.student_id || `student-${i}`,
    student_name: g.student_name_ar || g.student_name || g.student_number || m.academics_grade_sheet_student_fallback({ number: i + 1 }),
    student_number: g.student_number || "—",
    monthly_score: Number(g.score || 0),
    monthly_max: 20,
    exam_score: "—",
    exam_max: 30,
    term_total: Number(g.score || 0),
    term_max: 50,
    percentage: Number(g.percentage || 0),
    letter_grade: g.grade_letter || "—",
    status: g.is_passed ? "passed" : "failed",
    rank_in_class: "—",
  }));
});

const columns: OrbisColumnDef[] = [
  {
    id: "rank_in_class",
    prop: "rank_in_class",
    header: m.ui_rank_18c8bf(),
    name: m.ui_rank_18c8bf(),
    width: 65,
    size: 65,
    align: "center",
    pin: "left",
    cellClass: "font-mono font-bold text-primary",
  },
  {
    id: "student_name",
    prop: "student_name",
    header: m.common_student_name(),
    name: m.common_student_name(),
    width: 220,
    size: 220,
    pin: "left",
    cellClass: "font-semibold text-foreground",
  },
  {
    id: "monthly_score",
    prop: "monthly_score",
    header: m.ui_work_20_d82c4d(),
    name: m.ui_work_20_d82c4d(),
    width: 110,
    size: 110,
    align: "center",
    type: "number",
    cellClass: "font-mono font-medium",
  },
  {
    id: "exam_score",
    prop: "exam_score",
    header: m.ui_exam_30_af237e(),
    name: m.ui_exam_30_af237e(),
    width: 110,
    size: 110,
    align: "center",
    type: "number",
    cellClass: "font-mono font-medium",
  },
  {
    id: "term_total",
    prop: "term_total",
    header: m.ui_total_50_e12994(),
    name: m.ui_total_50_e12994(),
    width: 115,
    size: 115,
    align: "center",
    type: "number",
    cellClass: "font-mono font-bold text-primary",
  },
  {
    id: "percentage",
    prop: "percentage",
    header: m.ui_percentage_ae5fd2(),
    name: m.ui_percentage_ae5fd2(),
    width: 100,
    size: 100,
    align: "center",
    type: "percentage",
    cellClass: "font-mono",
  },
  {
    id: "letter_grade",
    prop: "letter_grade",
    header: m.common_grade(),
    name: m.common_grade(),
    width: 90,
    size: 90,
    align: "center",
    type: "badge",
  },
  {
    id: "status",
    prop: "status",
    header: m.common_status(),
    name: m.common_status(),
    width: 100,
    size: 100,
    align: "center",
    type: "badge",
  },
];

function handlePrint() {
  window.print();
}
</script>

<div class="flex flex-col h-full w-full gap-2.5 font-sans">
  <Card class="flex-1 flex flex-col min-h-0 overflow-hidden">
    <!-- Header -->
    <CardHeader class="py-3 px-4 flex flex-row items-center justify-between border-b border-border">
      <div class="flex items-center gap-2.5">
        <div class="w-8 h-8 rounded-lg bg-primary/10 flex items-center justify-center text-primary shrink-0">
          <DocumentCheck class="w-4 h-4" />
        </div>
        <div class="flex flex-col text-start">
          <CardTitle class="text-xs font-bold uppercase tracking-wider text-foreground">
            {m.ui_subject_term_grade_sheet_20_work_cdbe0f()}
          </CardTitle>
          <CardDescription class="text-[10px]">
            {rows.length} {m.common_term_results_recorded()}
          </CardDescription>
        </div>
      </div>

      <CardAction>
        <div class="flex items-center gap-2">
          <Button
            variant="outline"
            size="sm"
            class="h-8 px-2.5 gap-1.5 cursor-pointer text-xs font-semibold shadow-2xs"
            onclick={handlePrint}
          >
            <span>🖨️</span>
            <span>{m.ui_print_sheet_a421b0()}</span>
          </Button>

          <Button 
            variant="outline" 
            size="sm" 
            class="h-8 px-2.5 gap-1.5 cursor-pointer text-xs font-semibold shadow-2xs" 
            onclick={() => resultsQuery.refetch()} 
            disabled={isLoading}
          >
            {#if isLoading}<LoadingIndicator size={12} />{:else}<Refresh size={12} />{/if}
            <span>{m.common_retry()}</span>
          </Button>
        </div>
      </CardAction>
    </CardHeader>

    <CardContent class="flex-1 p-0 min-h-0 relative">
      {#if isLoading}
        <div class="absolute inset-0 flex flex-col items-center justify-center gap-2 bg-background/50 backdrop-blur-2xs z-10">
          <LoadingIndicator size={24} />
          <span class="text-xs text-muted-foreground">{m.common_loading_data()}</span>
        </div>
      {:else if error}
        <div class="absolute inset-0 flex flex-col items-center justify-center p-6 text-center text-destructive">
          <InfoCircle class="w-8 h-8 mb-2 stroke-current opacity-80" />
          <p class="text-xs font-semibold">{m.common_load_error()}</p>
          <p class="text-[10px] text-muted-foreground mt-1">{extractErrorMessage(error)}</p>
          <Button variant="outline" size="sm" class="mt-4" onclick={() => resultsQuery.refetch()}>
            {m.common_retry()}
          </Button>
        </div>
      {:else}
        <OrbisGrid
          {columns}
          source={rows}
          rtl={getLocale() === "ar"}
          emptyTitle={m.ui_no_grades_recorded_for_this_subj_c8326d()}
          emptyDescription={m.ui_record_monthly_work_or_final_exa_fb0db7()}
          readonly
        />
      {/if}
    </CardContent>
  </Card>
</div>
