<!--
  @component ClassSubjectResults
  Feature component displaying term results and calculated grades for a class subject.
  Uses @orbis/svelte-grid inside a standardized Card container.
-->
<script lang="ts">
import { createQuery } from "@tanstack/svelte-query";
import { api, extractErrorMessage } from "$lib/api";
import type { GradeDto } from "$lib/api/types";
import { Button } from "@orbis/ui/button";
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

export interface EnrichedGradeDto extends GradeDto {
  student_name?: string | null;
  student_name_ar?: string | null;
  student_number?: string | null;
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
    header: m.common_student_name(),
    name: m.common_student_name(),
    width: 240,
    size: 240,
    type: "avatar",
    cellClass: "font-semibold text-foreground",
  },
  {
    id: "max_score",
    prop: "max_score",
    header: m.common_assessment_max_score(),
    name: m.common_assessment_max_score(),
    width: 120,
    size: 120,
    align: "center",
    type: "number",
    cellClass: "font-mono text-muted-foreground",
  },
  {
    id: "obtained_score",
    prop: "obtained_score",
    header: m.common_assessment_obtained_score(),
    name: m.common_assessment_obtained_score(),
    width: 130,
    size: 130,
    align: "center",
    type: "number",
    cellClass: "font-bold font-mono text-foreground",
  },
  {
    id: "grade_value",
    prop: "grade_value",
    header: m.common_grade(),
    name: m.common_grade(),
    width: 100,
    size: 100,
    align: "center",
    type: "badge",
  },
  {
    id: "status",
    prop: "status",
    header: m.common_status(),
    name: m.common_status(),
    width: 120,
    size: 120,
    align: "center",
    type: "badge",
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
  <Card class="flex-1 flex flex-col min-h-0 overflow-hidden">
    <!-- Action Toolbar inside CardHeader -->
    <CardHeader class="py-3 px-4 flex flex-row items-center justify-between">
      <div class="flex items-center gap-2.5">
        <div class="w-8 h-8 rounded-lg bg-primary/10 flex items-center justify-center text-primary shrink-0">
          <DocumentCheck class="w-4 h-4" />
        </div>
        <div class="flex flex-col text-start">
          <CardTitle class="text-xs font-bold uppercase tracking-wider text-foreground">
            {m.shared_ui_subject_final_results()}
          </CardTitle>
          <CardDescription class="text-[10px]">
            {termResults.length} {m.common_term_results_recorded()}
          </CardDescription>
        </div>
      </div>

      <CardAction>
        <div class="flex items-center gap-2">
          <Button 
            variant="outline" 
            size="sm" 
            class="h-8 px-2.5 gap-1.5 cursor-pointer text-xs font-semibold shadow-2xs" 
            onclick={() => termResultsQuery.refetch()} 
            disabled={isLoading}
          >
            <Refresh size={12} class={isLoading ? "animate-spin" : ""} />
            <span>{m.common_retry()}</span>
          </Button>
        </div>
      </CardAction>
    </CardHeader>

    <!-- Grid Container inside CardContent -->
    <CardContent class="p-0 flex-1 min-h-0 flex flex-col overflow-hidden">
      {#if error}
        <div class="flex flex-1 flex-col items-center justify-center text-xs text-destructive gap-2 p-6">
          <InfoCircle size={24} />
          <span class="font-semibold">{extractErrorMessage(error)}</span>
          <Button variant="outline" size="sm" class="h-8 mt-2 text-xs" onclick={() => termResultsQuery.refetch()}>
            {m.common_retry()}
          </Button>
        </div>
      {:else}
        <div class="flex-1 min-h-0 w-full h-full">
          <OrbisGrid
            {columns}
            source={gridSource}
            loading={isLoading}
            height="100%"
            rowSize={44}
            readonly
            emptyTitle={m.common_no_term_results()}
          />
        </div>
      {/if}
    </CardContent>
  </Card>
</div>
