<!--
@component StudentGradesPage
Renders subject grades report card using OrbisGrid + summary analytics.
-->
<script lang="ts">
import { page } from "$app/state";
import { commands } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { GraduationCap, Award, Calendar, RefreshCcw } from "lucide-svelte";
import { onMount } from "svelte";
import { getLocale } from "$lib/paraglide/runtime";
import * as academics from "$lib/paraglide/messages/academics";
import { formatDate, formatPercent } from "$lib/core/i18n";

import { DataTable } from "$lib/tables";
import { studentGrades as sgDef, withIndex } from "$lib/tables/definitions";

import { academicContext } from "$lib/stores";

const studentId = $derived(page.params.studentId || "");
const orgId = $derived(page.params.orgId || "");
const periodId = $derived(academicContext.effectivePeriodId || "");

let report = $state<any>(null);
let isLoading = $state(true);

onMount(() => loadGrades());

async function loadGrades() {
  isLoading = true;
  try {
    if (!orgId || !studentId || !periodId) return;
    const enrollments = await commands.academic.listEnrollments(
      orgId,
      undefined,
      periodId,
    );
    const activeEnrollment = enrollments.find(
      (e) => e.student_id === studentId,
    );
    if (activeEnrollment) {
      report = await commands.assessment.getReportCard(
        activeEnrollment.id,
        periodId,
      );
    }
  } catch (e) {
    console.error(e);
  } finally {
    isLoading = false;
  }
}

// ── Grid Source ─────────────────────────────────────────────────────────

const gridSource = $derived(
  withIndex(
    (report?.grades || []).map((g: any) => ({
      ...g,
      subject_name: g.subject_name || g.subject_id || "—",
      exam_type: g.assessment_name || academics.grades_default_assessment(),
      score: Number(g.score || 0),
      max_score: Number(g.max_score || 100),
      percentage: Number(g.score || 0),
      grade_letter: g.grade_letter || "—",
    })),
  ),
);

const columns = $derived(
  sgDef.columns({
    idx: "#",
    subject: academics.grades_col_subject(),
    examType: academics.grades_col_assessment(),
    score: academics.grades_col_score(),
    maxScore: "Max",
    pct: academics.grades_col_indicator(),
    grade: "Grade",
  }),
);
</script>

<div class="flex flex-col gap-5 text-xs text-start">
  <div class="flex items-center justify-between">
    <div>
      <h2 class="text-sm font-bold">{academics.grades_title()}</h2>
      <p class="text-[10px] text-muted-foreground">{academics.grades_subtitle()}</p>
    </div>
    <Button
      variant="outline"
      size="icon-sm"
      class="h-7 w-7 text-muted-foreground hover:text-foreground"
      onclick={loadGrades}
      title={academics.grades_refresh()}
    >
      <RefreshCcw size={12} class={isLoading ? 'animate-spin' : ''} />
    </Button>
  </div>

  <DataTable
    {columns}
    source={gridSource}
    loading={isLoading}
    emptyIcon={GraduationCap}
    emptyTitle={academics.grades_empty_title()}
    emptyDescription={academics.grades_empty_subtitle()}
    rtl={getLocale() === "ar"}
    height="350px"
  />

  <!-- Summary Analytics -->
  {#if report?.grades?.length}
    <div class="grid grid-cols-2 gap-3 mt-1">
      <div class="border border-border bg-card/30 p-4 rounded-xl flex items-center justify-between">
        <div class="flex flex-col">
          <span class="text-[10px] text-muted-foreground">{academics.grades_overall_grade()}</span>
          <span class="text-sm font-bold text-gradient-brand mt-0.5">{academics.grades_overall_grade_value()}</span>
        </div>
        <Award size={18} class="text-primary" />
      </div>
      <div class="border border-border bg-card/30 p-4 rounded-xl flex items-center justify-between">
        <div class="flex flex-col">
          <span class="text-[10px] text-muted-foreground">{academics.grades_last_update()}</span>
          <span class="text-xs font-semibold text-foreground mt-0.5">{academics.grades_last_update_value()}</span>
        </div>
        <Calendar size={18} class="text-muted-foreground" />
      </div>
    </div>
  {/if}
</div>
