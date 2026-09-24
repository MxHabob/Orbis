<!--
  @component StudentGrades
  Student subject grades report card and GPA analytics.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { api } from "$lib/api";
import { Button } from "@orbis/ui/button";
import { BookOpen, Calendar, Refresh, AcademicCap } from "@orbis/icons";
import { onMount } from "svelte";
import { getLocale } from "$lib/paraglide/runtime";

import { formatDate } from "$lib/core/i18n";
import { OrbisGrid, type OrbisColumnDef } from "@orbis/svelte-grid";
import { academicContext } from "$lib/stores";
import { FilterTrigger, type UniversalFilter } from "@orbis/ui/filter-builder";
import { gradesFilterFields } from "$lib/core/filter-configs";

let { orgId, studentId }: { orgId: string; studentId: string } = $props();

const periodId = $derived(academicContext.effectivePeriodId || "");

let report = $state<any>(null);
let isLoading = $state(true);

onMount(() => loadGrades());

async function loadGrades() {
  isLoading = true;
  try {
    if (!orgId || !studentId || !periodId) return;
    const enrollments = await api.listEnrollments();
    const activeEnrollment = enrollments.find(
      (e: any) => e.student_id === studentId,
    );
    if (activeEnrollment?.id) {
      report = await api.calculateGpa({ student_id: activeEnrollment.id, period_id: periodId }).catch(() => null);
    }
  } catch (e) {
    console.error(e);
  } finally {
    isLoading = false;
  }
}

let activeFilter = $state<UniversalFilter>({
  logic: "AND",
  conditions: [],
  groups: [],
});

const gridSource = $derived(
  (report?.grades || []).map((g: any, i: number) => ({
    ...g,
    __idx: i + 1,
    subject_name: g.subject_name || g.subject_id || "—",
    exam_type: g.assessment_name || m.academics_grades_default_assessment(),
    score: Number(g.score || 0),
    max_score: 100,
    percentage: Number(g.score || 0),
    grade_letter: g.grade_letter || "—",
  })),
);

const columns = $derived.by<OrbisColumnDef[]>(() => [
  { id: "__idx", prop: "__idx", header: "#", name: "#", width: 48, size: 48, readonly: true, pin: "left", align: "center" },
  { id: "subject_name", prop: "subject_name", header: m.common_word_subject(), name: m.common_word_subject(), width: 200, size: 200, readonly: true, pin: "left", cellClass: "font-medium" },
  { id: "exam_type", prop: "exam_type", header: m.academics_grades_col_assessment(), name: m.academics_grades_col_assessment(), width: 140, size: 140, readonly: true },
  { id: "score", prop: "score", header: m.academics_grades_col_score(), name: m.academics_grades_col_score(), width: 90, size: 90, readonly: true, align: "end", type: "number" },
  { id: "max_score", prop: "max_score", header: "Max", name: "Max", width: 80, size: 80, readonly: true, align: "end", type: "number" },
  { id: "percentage", prop: "percentage", header: m.academics_grades_col_indicator(), name: m.academics_grades_col_indicator(), width: 100, size: 100, readonly: true, align: "end", type: "percentage" },
  { id: "grade_letter", prop: "grade_letter", header: "Grade", name: "Grade", width: 90, size: 90, readonly: true, align: "center", type: "badge" },
]);
</script>

<div class="flex flex-col gap-5 text-xs text-start">
  <div class="flex items-center justify-between">
    <div>
      <h2 class="text-sm font-bold">{m.academics_grades_title()}</h2>
      <p class="text-[10px] text-muted-foreground">{m.academics_grades_subtitle()}</p>
    </div>
    <div class="flex items-center gap-2">
      <FilterTrigger
        bind:filter={activeFilter}
        fields={gradesFilterFields}
      />
      <Button
        variant="outline"
        size="sm"
        class="h-7 w-7 p-0 text-muted-foreground hover:text-foreground"
        onclick={loadGrades}
        title={m.academics_grades_refresh()}
      >
        <Refresh size={12} class={isLoading ? 'animate-spin' : ''} />
      </Button>
    </div>
  </div>

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

  {#if report?.grades?.length}
    <div class="grid grid-cols-2 gap-3 mt-1">
      <div class="border border-border bg-card/30 p-4 rounded-xl flex items-center justify-between">
        <div class="flex flex-col">
          <span class="text-[10px] text-muted-foreground">{m.academics_grades_overall_grade()}</span>
          <span class="text-sm font-bold text-gradient-brand mt-0.5">{report?.cumulative_gpa != null ? Number(report.cumulative_gpa).toFixed(2) : "—"}</span>
        </div>
        <AcademicCap size={18} class="text-primary" />
      </div>
      <div class="border border-border bg-card/30 p-4 rounded-xl flex items-center justify-between">
        <div class="flex flex-col">
          <span class="text-[10px] text-muted-foreground">{m.academics_grades_last_update()}</span>
          <span class="text-xs font-semibold text-foreground mt-0.5">{report?.calculated_at ? formatDate(report.calculated_at) : "—"}</span>
        </div>
        <Calendar size={18} class="text-muted-foreground" />
      </div>
    </div>
  {/if}
</div>
