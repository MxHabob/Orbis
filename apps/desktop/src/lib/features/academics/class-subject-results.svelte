<!--
  @component ClassSubjectResults
  Feature component displaying term results and calculated grades for a class subject.
  Uses @orbis/svelte-grid / DataTable for high performance.
-->
<script lang="ts">
import { createQuery } from "@tanstack/svelte-query";
import { api, extractErrorMessage } from "$lib/api";
import type { GradeDto } from "$lib/api/types";
import { Button } from "@orbis/ui/button";
import { DocumentCheck, InfoCircle, Refresh } from "@orbis/icons";
import { academicContext } from "$lib/stores";
import * as m from "$lib/paraglide/messages";
import { DataTable, BadgeCell } from "$lib/tables";
import type { OrbisColumnDef } from "@orbis/svelte-grid";

export interface EnrichedGradeDto extends GradeDto {
  student_name?: string | null;
  student_name_ar?: string | null;
  student_number?: string | null;
  max_score?: number | null;
  obtained_score?: number | null;
  grade_value?: string | null;
  is_passed?: number | boolean | null;
}

let { orgId, classId, classSubjectId }: { orgId: string; classId: string; classSubjectId: string } = $props();

const selectedPeriodId = $derived(academicContext.effectivePeriodId || "");

const termResultsQuery = createQuery(() => ({
  queryKey: ["term-results-enriched", selectedPeriodId, classSubjectId],
  queryFn: async (): Promise<EnrichedGradeDto[]> => {
    const res = await api.listGrades({ classSubjectId });
    return (res || []) as EnrichedGradeDto[];
  },
  enabled: !!selectedPeriodId && !!classSubjectId,
}));

const termResults = $derived(termResultsQuery.data ?? []);
const isLoading = $derived(termResultsQuery.isLoading);
const error = $derived(termResultsQuery.error);

function getStudentName(r: EnrichedGradeDto): string {
  return r.student_name_ar || r.student_name || r.student_number || "-";
}

const columns: OrbisColumnDef[] = [
  {
    id: "student_name",
    prop: "student_name",
    header: m.common_student_name ? m.common_student_name() : "Student Name",
    name: m.common_student_name ? m.common_student_name() : "Student Name",
    width: 240,
    size: 240,
    cellClass: "font-semibold text-foreground",
  },
  {
    id: "max_score",
    prop: "max_score",
    header: m.common_assessment_max_score ? m.common_assessment_max_score() : "Max Score",
    name: m.common_assessment_max_score ? m.common_assessment_max_score() : "Max Score",
    width: 120,
    size: 120,
    align: "center",
    cellClass: "font-mono text-muted-foreground",
  },
  {
    id: "obtained_score",
    prop: "obtained_score",
    header: m.common_assessment_obtained_score ? m.common_assessment_obtained_score() : "Obtained Score",
    name: m.common_assessment_obtained_score ? m.common_assessment_obtained_score() : "Obtained Score",
    width: 130,
    size: 130,
    align: "center",
    cellClass: "font-bold font-mono text-foreground",
  },
  {
    id: "grade_value",
    prop: "grade_value",
    header: m.common_grade ? m.common_grade() : "Grade",
    name: m.common_grade ? m.common_grade() : "Grade",
    width: 100,
    size: 100,
    align: "center",
    cell: BadgeCell,
  },
  {
    id: "status",
    prop: "status",
    header: m.common_status ? m.common_status() : "Status",
    name: m.common_status ? m.common_status() : "Status",
    width: 120,
    size: 120,
    align: "center",
    cell: BadgeCell,
  },
];

const gridSource = $derived(
  termResults.map((r) => ({
    ...r,
    student_name: getStudentName(r),
    max_score: r.max_score ?? 100,
    obtained_score: r.obtained_score ?? 0,
    grade_value: r.grade_value || "-",
    status: r.is_passed === 1 || r.is_passed === true ? "passed" : "failed",
  })),
);
</script>

<div class="flex flex-col h-full w-full gap-2.5 font-sans">
  <!-- Action Toolbar -->
  <div class="flex items-center justify-between shrink-0 px-0.5">
    <div class="flex items-center gap-2">
      <div class="flex items-center gap-1.5 text-xs text-muted-foreground font-medium">
        <DocumentCheck size={14} class="text-primary opacity-80" />
        <span>
          {termResults.length} {m.common_term_results_recorded ? m.common_term_results_recorded() : "recorded term results"}
        </span>
      </div>
    </div>

    <div class="flex items-center gap-2">
      <Button 
        variant="outline" 
        size="sm" 
        class="h-8 px-2.5 gap-1.5 cursor-pointer text-xs font-semibold shadow-2xs" 
        onclick={() => termResultsQuery.refetch()} 
        disabled={isLoading}
      >
        <Refresh size={12} class={isLoading ? "animate-spin" : ""} />
        <span>{m.common_retry ? m.common_retry() : "Refresh"}</span>
      </Button>
    </div>
  </div>

  <!-- Grid Container -->
  <div class="flex-1 overflow-hidden border border-border/70 rounded-xl shadow-xs bg-card">
    {#if error}
      <div class="flex h-full flex-col items-center justify-center text-xs text-destructive gap-2 p-6">
        <InfoCircle size={24} />
        <span class="font-semibold">{extractErrorMessage(error)}</span>
        <Button variant="outline" size="sm" class="h-8 mt-2 text-xs" onclick={() => termResultsQuery.refetch()}>
          {m.common_retry ? m.common_retry() : "Retry"}
        </Button>
      </div>
    {:else}
      <DataTable
        {columns}
        source={gridSource}
        loading={isLoading}
        height="100%"
        rowSize={44}
        emptyTitle={m.common_no_term_results ? m.common_no_term_results() : "No term results calculated yet"}
      />
    {/if}
  </div>
</div>
