<!--
  @component SectionSubjectDetails
  Feature component for the Coursework Gradebook spreadsheet.
  Provides interactive cell editing, total recalculation, and batch grade saving inside a standardized Card container.
-->
<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import { api, extractErrorMessage } from "$lib/api";
import { Button } from "@orbis/ui/button";
import { toast } from "@orbis/ui";
import SettingsBottomBar from "$lib/shared/components/settings-bottom-bar.svelte";
import { getLocale } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";
import { academicContext } from "$lib/stores";
import { OrbisGrid, type CellEditEvent } from "@orbis/svelte-grid";
import { Refresh } from "@orbis/icons";
import {
  buildGradeColumns,
  recalculateRowTotals,
  scoreToGradeLetter,
  transformMatrixToSource,
  extractSchemeComponents,
  findMonthlyScheme,
  type AssessmentColumnConfig,
  type GradeCell,
  type GradeMatrixResponse,
  type GradeMatrixStudent,
} from "$lib/features/exams/grade-helpers";

import {
  createClassStudentsQuery,
  createGradesQuery,
} from "$lib/features/academics/queries";
import { createAssessmentSchemesQuery } from "$lib/features/academics/queries-assessment";

let {
  orgId,
  classId,
  SectionSubjectId,
}: { orgId: string; classId: string; SectionSubjectId: string } = $props();

// Matrix Data (Coursework)
const selectedPeriodId = $derived(academicContext.effectivePeriodId || "");
const periods = $derived(academicContext.allPeriods);
const selectedPeriod = $derived(
  periods.find((p) => String(p.id) === selectedPeriodId),
);
const isPeriodLocked = $derived(Boolean(selectedPeriod?.is_locked));
const isGradeEntryOpen = $derived(Boolean(selectedPeriod?.is_grade_entry_open));
const canEnterGrades = $derived(
  !isPeriodLocked && isGradeEntryOpen && !academicContext.isReadOnly,
);

const studentsQuery = createClassStudentsQuery(() => classId);
const gradesQuery = createGradesQuery(() => SectionSubjectId);
const schemesQuery = createAssessmentSchemesQuery(() => orgId);

const schemeComponents = $derived.by((): AssessmentColumnConfig[] => {
  const schemes = schemesQuery.data;
  if (!schemes || !Array.isArray(schemes) || schemes.length === 0) return [];
  const monthlyScheme = findMonthlyScheme(schemes as Record<string, unknown>[]);
  if (!monthlyScheme) return [];
  return extractSchemeComponents(monthlyScheme);
});

const isMatrixLoading = $derived(
  studentsQuery.isLoading || gradesQuery.isLoading || schemesQuery.isLoading,
);
const error = $derived(studentsQuery.error || gradesQuery.error || schemesQuery.error);

let isSaving = $state(false);
let isDirty = $state(false);

const matrixData = $derived.by((): GradeMatrixResponse => {
  const roster = studentsQuery.data ?? [];
  const grades = gradesQuery.data ?? [];
  const hasScheme = schemeComponents.length > 0;

  // Build assessment configs dynamically from scheme, with legacy fallback
  const assessmentConfigs = hasScheme
    ? schemeComponents.map((comp) => ({
        code: comp.code,
        name: comp.name,
        name_ar: comp.name_ar,
        max: comp.max_score,
        sort_order: comp.sort_order,
      }))
    : [
        { code: "monthly", name: "Monthly", name_ar: "العملية", max: 20, sort_order: 1 },
        { code: "final", name: "Final", name_ar: "الاختبار", max: 30, sort_order: 2 },
      ];

  const students: GradeMatrixStudent[] = roster.map((student: any) => {
    const studentId = student.student?.id ?? student.id;
    const studentGrades = grades.filter(
      (grade) =>
        String(grade.student_id) === String(studentId) &&
        String(grade.period_id) === String(selectedPeriodId),
    );
    const cells: GradeCell[] = assessmentConfigs.map(({ code, max }) => {
      // Match by component_id when using scheme, legacy is_final fallback otherwise
      const grade = hasScheme
        ? studentGrades.find(
            (item) =>
              String(item.component_id ?? "") === code ||
              (item.component_id && String(item.component_id).includes(code)),
          ) ??
          // Legacy fallback for old 2-column data
          (code === "final" || code === "written_test" || code === "term_exam"
            ? studentGrades.find((item) => Boolean(item.is_final))
            : null)
        : studentGrades.find(
            (item) => Boolean(item.is_final) === (code === "final"),
          );
      const gradeMax = Number(grade?.max_score);
      return {
        grade_id: grade?.id ?? null,
        assessment_id: grade?.component_id ?? null,
        exam_type: code,
        score: grade ? Number(grade.score) : null,
        max_score: Number.isFinite(gradeMax) && gradeMax > 0 ? gradeMax : max,
        component_id: grade?.component_id ?? null,
      };
    });
    const total = cells.reduce((sum, cell) => sum + (cell.score || 0), 0);
    const maxTotal = cells.reduce((sum, cell) => sum + cell.max_score, 0);
    const percentage = maxTotal > 0 ? Math.round((total / maxTotal) * 100) : 0;

    return {
      enrollment_id:
        student.enrollment?.id ??
        studentGrades.find((grade) => grade.enrollment_id)?.enrollment_id ??
        "",
      student_id: studentId,
      student_name:
        student.person?.full_name ||
        student.person?.display_name ||
        student.student?.student_number ||
        "",
      student_name_ar:
        student.person?.full_name_ar ||
        student.person?.full_name ||
        student.person?.display_name ||
        student.student?.student_number ||
        null,
      grades: cells,
      total,
      max_total: maxTotal,
      percentage,
      grade_letter: scoreToGradeLetter(
        percentage,
        academicContext.gradeScaleItems,
      ),
    };
  });

  return {
    period_id: selectedPeriodId,
    section_subject_id: SectionSubjectId,
    students,
    assessment_types: assessmentConfigs.map(({ code }) => code),
    components: hasScheme ? schemeComponents : undefined,
  };
});

let gridSource = $derived.by(() =>
  transformMatrixToSource(
    matrixData,
    getLocale(),
    academicContext.gradeScaleItems,
  ),
);
const assessmentKeys = $derived(matrixData.assessment_types);

const gridColumns = $derived.by(() => {
  return buildGradeColumns(matrixData, {
    readonly: !canEnterGrades,
    locale: getLocale(),
  });
});

function handleRefresh() {
  studentsQuery.refetch();
  gradesQuery.refetch();
  isDirty = false;
}

function handleCellEdit(event: CellEditEvent) {
  const { rowIndex, columnId, newValue } = event;
  const field = columnId;
  if (rowIndex < 0 || rowIndex >= gridSource.length) return;

  const updatedSource = [...gridSource];
  const row = { ...updatedSource[rowIndex]! };
  row[field] = newValue;
  recalculateRowTotals(row, assessmentKeys, academicContext.gradeScaleItems);

  updatedSource[rowIndex] = row;
  gridSource = updatedSource;
  isDirty = true;
}

async function handleSaveAll() {
  if (!matrixData || !canEnterGrades) return;
  isSaving = true;
  try {
    const studentsToSave: any[] = [];
    for (const row of gridSource) {
      for (const key of assessmentKeys) {
        const score = row[`exam_${key}`];
        if (score !== null && score !== undefined && score !== "") {
          const meta = row[`__meta_exam_${key}`] as
            | { max_score: number; component_id?: string | null }
            | undefined;
          // Read max_score from the scheme component, fall back to meta or legacy defaults
          const compConfig = schemeComponents.find((c) => c.code === key);
          const maxScore = compConfig?.max_score ?? meta?.max_score ?? 100;
          studentsToSave.push({
            student_id: row.student_id,
            enrollment_id: row.enrollment_id || undefined,
            score: Number(score),
            max_score: maxScore,
            notes: null,
            exam_type: key,
            component_id: meta?.component_id ?? key,
          });
        }
      }
    }

    if (studentsToSave.length === 0) {
      toast.info(m.common_no_changes_to_save());
      return;
    }

    for (const student of studentsToSave) {
      await api.recordGrade({
        period_id: selectedPeriodId,
        student_id: String(student.student_id),
        enrollment_id: student.enrollment_id || undefined,
        section_subject_id: SectionSubjectId,
        score: String(student.score),
        max_score: String(student.max_score),
        notes: student.notes ?? undefined,
        component_id: student.component_id ?? undefined,
      });
    }

    toast.success(m.common_grades_save_success());
    isDirty = false;
    // Refetch grades to reflect saved data
    gradesQuery.refetch();
  } catch (e: any) {
    toast.error(e?.message || m.common_grades_save_error());
  } finally {
    isSaving = false;
  }
}
</script>

<div class="h-full min-h-0 w-full font-sans flex flex-col">
  {#if selectedPeriod && isPeriodLocked}
    <div class="shrink-0 mx-3 mt-2 rounded-lg border border-destructive/20 bg-destructive/5 px-3 py-2">
      <p class="text-[11px] font-medium text-destructive">{m.common_control_period_locked_title()}</p>
      <p class="text-[10px] text-muted-foreground">{m.common_control_period_locked_desc()}</p>
    </div>
  {:else if selectedPeriod && !isGradeEntryOpen}
    <div class="shrink-0 mx-3 mt-2 rounded-lg border border-amber-500/20 bg-amber-500/5 px-3 py-2">
      <p class="text-[11px] font-medium text-amber-600">{m.common_control_entry_closed_title()}</p>
      <p class="text-[10px] text-muted-foreground">{m.common_control_entry_closed_desc()}</p>
    </div>
  {/if}

  {#if error}
    <div class="flex h-full flex-col items-center justify-center gap-2 p-6 text-center text-destructive">
      <p class="text-xs font-semibold">{m.common_load_error()}</p>
      <p class="text-[10px] text-muted-foreground">
        {extractErrorMessage(error)}
      </p>
      <Button variant="outline" size="sm" class="mt-2 h-8 text-xs" onclick={handleRefresh}>
        {m.common_retry()}
      </Button>
    </div>
  {:else if !gridColumns.length && !isMatrixLoading}
    <div class="flex h-full flex-col items-center justify-center gap-2 p-8 text-center">
      <p class="text-xs font-semibold text-foreground">
        {m.common_assessment_no_scheme()}
      </p>
      <p class="max-w-sm text-[11px] text-muted-foreground">
        {m.common_assessment_configure_hint()}
      </p>
    </div>
  {:else}
    <OrbisGrid
      columns={gridColumns}
      bind:source={gridSource}
      loading={isMatrixLoading}
      height="100%"
      rtl={getLocale() === "ar"}
      showAddRow={false}
      readonly={!canEnterGrades}
      oncelledit={handleCellEdit}
      emptyTitle={m.common_students_empty_title()}
    >
      {#snippet toolbarActions()}
        <Button
          variant="outline"
          size="sm"
          class="h-8 gap-1.5 px-2.5 text-xs"
          onclick={handleRefresh}
          disabled={isMatrixLoading}
          title={m.common_retry()}
        >
          {#if isMatrixLoading}<LoadingIndicator size={12} />{:else}<Refresh size={12} />{/if}
          <span>{m.common_retry()}</span>
        </Button>
      {/snippet}
    </OrbisGrid>
  {/if}
</div>

<SettingsBottomBar
  visible={canEnterGrades && isDirty}
  isDirty={isDirty}
  isSaving={isSaving}
  showReset={false}
  message={m.common_unsaved_changes()}
  saveLabel={m.common_save_changes()}
  savingLabel={m.common_state_saving()}
  onsave={handleSaveAll}
/>
