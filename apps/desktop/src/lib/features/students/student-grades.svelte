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
import { DataTable } from "$lib/tables";
import { studentGrades as sgDef, withIndex } from "$lib/tables/definitions";
import { academicContext } from "$lib/stores";
import { applyFilter } from "@orbis/ui/filter-builder";
import type { UniversalFilter } from "@orbis/ui/filter-builder";
import { gradesFilterFields } from "$lib/tables/definitions/filter-configs";

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

const filteredGrades = $derived(
  applyFilter(report?.grades || [], activeFilter),
);

const gridSource = $derived(
  withIndex(
    filteredGrades.map((g: any) => ({
      ...g,
      subject_name: g.subject_name || g.subject_id || "—",
      exam_type: g.assessment_name || m.academics_grades_default_assessment(),
      score: Number(g.score || 0),
      max_score: "100",
      percentage: Number(g.score || 0),
      grade_letter: g.grade_letter || "—",
    })),
  ),
);

const columns = $derived(
  sgDef.columns({
    idx: "#",
    subject: m.common_word_subject(),
    examType: m.academics_grades_col_assessment(),
    score: m.academics_grades_col_score(),
    maxScore: "Max",
    pct: m.academics_grades_col_indicator(),
    grade: "Grade",
  }),
);
</script>

<div class="flex flex-col gap-5 text-xs text-start">
  <div class="flex items-center justify-between">
    <div>
      <h2 class="text-sm font-bold">{m.academics_grades_title()}</h2>
      <p class="text-[10px] text-muted-foreground">{m.academics_grades_subtitle()}</p>
    </div>
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

  <DataTable
    {columns}
    source={gridSource}
    loading={isLoading}
    filterable={true}
    filterFields={gradesFilterFields}
    bind:filterValue={activeFilter}
    emptyIcon={BookOpen}
    emptyTitle={m.academics_grades_empty_title()}
    emptyDescription={m.academics_grades_empty_subtitle()}
    rtl={getLocale() === "ar"}
    height="350px"
  />

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
