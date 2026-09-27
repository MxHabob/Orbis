<!--
  @component StudentGrades
  Student subject grades report card and GPA analytics with standardized Cards.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { api } from "$lib/api";
import { Button } from "@orbis/ui/button";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardAction,
  CardContent,
} from "@orbis/ui/card";
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
    report = await api.calculateGpa({ student_id: studentId, period_id: periodId }).catch(() => null);
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
  (report?.grades || []).map((g, i: number) => ({
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
  { id: "subject_name", prop: "subject_name", header: m.common_word_subject(), name: m.common_word_subject(), width: 200, size: 200, readonly: true, pin: "left", cellClass: "font-semibold text-foreground" },
  { id: "exam_type", prop: "exam_type", header: m.academics_grades_col_assessment(), name: m.academics_grades_col_assessment(), width: 140, size: 140, readonly: true },
  { id: "score", prop: "score", header: m.academics_grades_col_score(), name: m.academics_grades_col_score(), width: 90, size: 90, readonly: true, align: "end", type: "number", cellClass: "font-mono font-bold" },
  { id: "max_score", prop: "max_score", header: "Max", name: "Max", width: 80, size: 80, readonly: true, align: "end", type: "number", cellClass: "font-mono text-muted-foreground" },
  { id: "percentage", prop: "percentage", header: m.academics_grades_col_indicator(), name: m.academics_grades_col_indicator(), width: 100, size: 100, readonly: true, align: "end", type: "percentage", cellClass: "font-mono font-medium" },
  { id: "grade_letter", prop: "grade_letter", header: "Grade", name: "Grade", width: 90, size: 90, readonly: true, align: "center", type: "badge" },
]);
</script>

<div class="flex flex-col gap-6 text-xs text-start font-sans">
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
          <Button
            variant="ghost"
            size="icon-sm"
            class="text-muted-foreground hover:text-foreground"
            onclick={loadGrades}
            title={m.academics_grades_refresh()}
          >
            <Refresh size={13} class={isLoading ? 'animate-spin' : ''} />
          </Button>
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

  {#if report?.grades?.length}
    <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
      <Card class="p-4 flex items-center justify-between">
        <div class="flex flex-col">
          <span class="text-[10px] text-muted-foreground font-semibold">{m.academics_grades_overall_grade()}</span>
          <span class="text-base font-bold font-mono text-primary mt-0.5">{report?.cumulative_gpa != null ? Number(report.cumulative_gpa).toFixed(2) : "—"}</span>
        </div>
        <AcademicCap size={20} class="text-primary" />
      </Card>
      <Card class="p-4 flex items-center justify-between">
        <div class="flex flex-col">
          <span class="text-[10px] text-muted-foreground font-semibold">{m.academics_grades_last_update()}</span>
          <span class="text-xs font-semibold text-foreground mt-0.5">{report?.calculated_at ? formatDate(report.calculated_at) : "—"}</span>
        </div>
        <Calendar size={20} class="text-muted-foreground" />
      </Card>
    </div>
  {/if}
</div>
