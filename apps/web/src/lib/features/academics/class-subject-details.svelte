<!--
  @component SectionSubjectDetails
  Feature component for the Coursework Gradebook spreadsheet.
  Provides interactive cell editing, total recalculation, and batch grade saving inside a standardized Card container.
  Supports numeric scores and status tokens (absent/excused/exempt).
-->
<script lang="ts">
import { periodOverrides } from "$lib/shared/utils/period-override.svelte";
import {
  gradePeriodOverride,
  isGradePeriodOverrideable,
} from "$lib/api/period-override";

import { LoadingIndicator } from "@orbis/ui";
import { api, extractErrorMessage } from "$lib/api";
import { clearDraft, loadDraft, saveDraft } from "$lib/api/draft-store";
import { Button } from "@orbis/ui/button";
import { toast } from "@orbis/ui";
import SettingsBottomBar from "$lib/shared/components/settings-bottom-bar.svelte";
import { getLocale } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";
import { academicContext, effectiveContext } from "$lib/stores";
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

type GradeState = "present" | "absent" | "excused" | "exempt";
type GradeCellState = { score: number | null; state: GradeState };

const locale = $derived(getLocale());
const selectedPeriodId = $derived(academicContext.effectivePeriodId || "");
const periods = $derived(academicContext.allPeriods);
const selectedPeriod = $derived(
  periods.find((p) => String(p.id) === selectedPeriodId),
);
const isPeriodLocked = $derived(Boolean(selectedPeriod?.is_locked));
const isGradeEntryOpen = $derived(Boolean(selectedPeriod?.can_enter_grades));
const canEnterGrades = $derived(
  !isPeriodLocked &&
    isGradeEntryOpen &&
    !academicContext.isReadOnly &&
    effectiveContext.hasPermission("grade:write"),
);
const canRequestOverride = $derived(
  !academicContext.isReadOnly &&
    effectiveContext.hasPermission("grade:write") &&
    effectiveContext.hasPermission("workflow:write") &&
    isGradePeriodOverrideable(selectedPeriod),
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
const error = $derived(
  studentsQuery.error || gradesQuery.error || schemesQuery.error,
);

let isSaving = $state(false);
let isDirty = $state(false);

function gradeStateOf(g: any): GradeState {
  if (g?.is_exempt) return "exempt";
  if (g?.is_excused) return "excused";
  if (g?.is_absent) return "absent";
  return "present";
}
function stateLabel(state: GradeState): string {
  if (state === "absent") return m.academics_grade_state_absent();
  if (state === "excused") return m.academics_grade_state_excused();
  if (state === "exempt") return m.academics_grade_state_exempt();
  return "";
}
function canonicalCell(
  cell: GradeCellState | null | undefined,
): GradeCellState | null {
  if (!cell) return null;
  if (
    cell.state === "present" &&
    (cell.score === null || cell.score === undefined)
  )
    return null;
  return {
    score: cell.state === "present" ? cell.score : (cell.score ?? 0),
    state: cell.state,
  };
}
function sameCell(a: GradeCellState | null, b: GradeCellState | null): boolean {
  const x = canonicalCell(a),
    y = canonicalCell(b);
  if (!x && !y) return true;
  if (!x || !y) return false;
  return x.state === y.state && Number(x.score ?? 0) === Number(y.score ?? 0);
}
function displayForCell(cell: GradeCellState | null): string | number | null {
  const c = canonicalCell(cell);
  if (!c) return null;
  return c.state === "present" ? (c.score ?? null) : stateLabel(c.state);
}
function parseCellInput(raw: unknown, maxScore: number): GradeCellState | null {
  if (raw === null || raw === undefined) return null;
  if (typeof raw === "number") {
    if (!Number.isFinite(raw)) return null;
    return { score: Math.max(0, Math.min(maxScore, raw)), state: "present" };
  }
  const text = String(raw).trim();
  if (text === "") return null;
  const norm = text.toLowerCase();
  if (
    [
      "\u063a",
      "\u063a\u0627\u0626\u0628",
      "a",
      "abs",
      "absent",
      m.academics_grade_state_absent().toLowerCase(),
    ].includes(norm)
  )
    return { score: 0, state: "absent" };
  if (
    [
      "\u0645",
      "\u0645\u0639\u0630\u0648\u0631",
      "\u0639\u0630\u0631",
      "e",
      "exc",
      "excused",
      m.academics_grade_state_excused().toLowerCase(),
    ].includes(norm)
  )
    return { score: 0, state: "excused" };
  if (
    [
      "\u0645\u0639",
      "\u0645\u0639\u0641\u0649",
      "\u0645\u0639\u0627\u0641",
      "x",
      "ex",
      "exempt",
      m.academics_grade_state_exempt().toLowerCase(),
    ].includes(norm)
  )
    return { score: 0, state: "exempt" };
  const digits = text
    .replace(/[\u0660-\u0669]/g, (d) =>
      String(
        "\u0660\u0661\u0662\u0663\u0664\u0665\u0666\u0667\u0668\u0669".indexOf(
          d,
        ),
      ),
    )
    .replace(/\u066b/g, ".")
    .replace(/,/g, ".");
  const n = Number(digits);
  if (!Number.isFinite(n)) return null;
  return { score: Math.max(0, Math.min(maxScore, n)), state: "present" };
}
function findOriginalGrade(studentId: string, key: string) {
  const grades = (gradesQuery.data ?? []) as any[];
  const hasScheme = schemeComponents.length > 0;
  const studentGrades = grades.filter(
    (grade) =>
      String(grade.student_id) === String(studentId) &&
      String(grade.period_id) === String(selectedPeriodId),
  );
  if (hasScheme) {
    return (
      studentGrades.find(
        (item) =>
          String(item.component_code ?? item.component_id ?? "") === key,
      ) ??
      (["final", "written_test", "term_exam"].includes(key)
        ? studentGrades.find((item) => Boolean(item.is_final))
        : null) ??
      null
    );
  }
  return (
    studentGrades.find(
      (item) => Boolean(item.is_final) === (key === "final"),
    ) ?? null
  );
}

const matrixData = $derived.by((): GradeMatrixResponse => {
  const roster = studentsQuery.data ?? [];
  const grades = gradesQuery.data ?? [];
  const hasScheme = schemeComponents.length > 0;

  const assessmentConfigs = hasScheme
    ? schemeComponents.map((comp) => ({
        code: comp.code,
        name: comp.name,
        name_ar: comp.name_ar,
        max: comp.max_score,
        sort_order: comp.sort_order,
      }))
    : [
        {
          code: "monthly",
          name: "Monthly",
          name_ar: "العملية",
          max: 20,
          sort_order: 1,
        },
        {
          code: "final",
          name: "Final",
          name_ar: "الاختبار",
          max: 30,
          sort_order: 2,
        },
      ];

  const students: GradeMatrixStudent[] = (roster as any[]).map(
    (student: any) => {
      const studentId = student.student?.id ?? student.id;
      const studentGrades = (grades as any[]).filter(
        (grade) =>
          String(grade.student_id) === String(studentId) &&
          String(grade.period_id) === String(selectedPeriodId),
      );
      const cells: GradeCell[] = assessmentConfigs.map(({ code, max }) => {
        const grade = hasScheme
          ? (studentGrades.find(
              (item) =>
                String(item.component_code ?? item.component_id ?? "") === code,
            ) ??
            (["final", "written_test", "term_exam"].includes(code)
              ? studentGrades.find((item) => Boolean(item.is_final))
              : null))
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
          component_code: grade?.component_code ?? null,
        };
      });
      // Totals reflect numeric scores in cells as stored (display calc); state-aware adjustments handled in UI layer.
      const total = cells.reduce((sum, cell) => sum + (cell.score || 0), 0);
      const maxTotal = cells.reduce((sum, cell) => sum + cell.max_score, 0);
      const percentage =
        maxTotal > 0 ? Math.round((total / maxTotal) * 100) : 0;

      return {
        enrollment_id:
          student.enrollment?.id ??
          studentGrades.find((grade) => grade.enrollment_id)?.enrollment_id ??
          "",
        student_id: String(studentId),
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
    },
  );

  return {
    period_id: selectedPeriodId,
    section_subject_id: SectionSubjectId,
    students,
    assessment_types: assessmentConfigs.map(({ code }) => code),
    components: hasScheme ? schemeComponents : undefined,
  };
});

let gridSource = $state<Record<string, unknown>[]>([]);
let baselineSource = $state<Record<string, unknown>[]>([]);
let lastMatrixKey = $state("");

function decorateRows(rows: Record<string, unknown>[]) {
  return rows.map((row) => {
    const r = { ...row } as Record<string, unknown>;
    for (const key of assessmentKeys) {
      const g = findOriginalGrade(String(r.student_id), key) as any;
      const cell: GradeCellState | null = g
        ? {
            score: Number.isFinite(Number(g.score)) ? Number(g.score) : null,
            state: gradeStateOf(g),
          }
        : null;
      r[`__cell_exam_${key}`] = cell;
      r[`exam_${key}`] = displayForCell(cell);
      // Keep meta consistent with original grade for save resolution.
      const meta = (r[`__meta_exam_${key}`] as any) || {};
      r[`__meta_exam_${key}`] = {
        ...meta,
        component_code: g?.component_code ?? meta.component_code ?? key,
        component_id: g?.component_id ?? meta.component_id ?? null,
        max_score: meta.max_score ?? g?.max_score ?? 100,
      };
    }
    recalcRowLocal(r);
    return r;
  });
}

const assessmentKeys = $derived(matrixData.assessment_types);

function gradeDraftKey(): string {
  return `grades:${orgId}:${SectionSubjectId}:${selectedPeriodId}`;
}

type GradeDraftMap = Record<string, Record<string, GradeCellState | null>>;

function persistGradeDraft() {
  const draft: GradeDraftMap = {};
  for (let i = 0; i < gridSource.length; i++) {
    const row = gridSource[i] as any;
    const base = (baselineSource[i] || {}) as any;
    const studentId = String(row.student_id ?? "");
    if (!studentId) continue;
    for (const key of assessmentKeys) {
      const current = canonicalCell(
        row[`__cell_exam_${key}`] as GradeCellState | null,
      );
      const original = canonicalCell(
        base[`__cell_exam_${key}`] as GradeCellState | null,
      );
      if (!sameCell(current, original)) {
        draft[studentId] ??= {};
        draft[studentId][key] = current;
      }
    }
  }
  if (Object.keys(draft).length > 0) {
    saveDraft(gradeDraftKey(), draft);
  } else {
    clearDraft(gradeDraftKey());
  }
}

$effect(() => {
  if (isMatrixLoading) return;
  const key = `${selectedPeriodId}|${SectionSubjectId}|${matrixData.students.length}|${assessmentKeys.join(",")}`;
  if (key !== lastMatrixKey && !isDirty) {
    const base = transformMatrixToSource(
      matrixData,
      getLocale(),
      academicContext.gradeScaleItems,
    );
    const decorated = decorateRows(base);
    baselineSource = decorated.map((r) => structuredClone(r));

    const draft = loadDraft<GradeDraftMap>(gradeDraftKey());
    let restored = false;
    const working = decorated.map((r) => {
      const row = structuredClone(r) as Record<string, unknown>;
      const studentDraft = draft?.[String(row.student_id ?? "")];
      if (studentDraft) {
        for (const [compKey, cellState] of Object.entries(studentDraft)) {
          row[`__cell_exam_${compKey}`] = cellState;
          row[`exam_${compKey}`] = displayForCell(cellState);
          restored = true;
        }
        recalcRowLocal(row);
      }
      return row;
    });
    gridSource = working;
    lastMatrixKey = key;
    if (restored) {
      isDirty = true;
    }
  }
});

function recalcRowLocal(row: Record<string, unknown>) {
  // Recalculate totals from cell states (present numeric, absent=0; excused/exempt excluded from totals/max).
  let total = 0;
  let maxTotal = 0;
  for (const key of assessmentKeys) {
    const meta = row[`__meta_exam_${key}`] as
      | { max_score?: number }
      | undefined;
    const max = Number(meta?.max_score) || 100;
    const cell = row[`__cell_exam_${key}`] as GradeCellState | null;
    if (cell?.state === "excused" || cell?.state === "exempt") continue;
    maxTotal += max;
    if (
      cell &&
      cell.state === "present" &&
      cell.score !== null &&
      cell.score !== undefined
    ) {
      const n = Number(cell.score);
      if (Number.isFinite(n)) total += n;
    }
  }
  const percentage = maxTotal > 0 ? Math.round((total / maxTotal) * 100) : 0;
  row.total = total;
  row.max_total = maxTotal;
  row.percentage = percentage;
  row.grade_letter = scoreToGradeLetter(
    percentage,
    academicContext.gradeScaleItems,
    row.grade_letter as string,
  );
}

const gridColumns = $derived.by(() => {
  const cols = buildGradeColumns(matrixData, {
    readonly: !canEnterGrades,
    locale: getLocale(),
  });
  // Make assessment cells text-editable to support status tokens (غ/م/مع, A/E/X).
  return cols.map((c) => {
    const id = String((c as any).id || (c as any).prop || "");
    if (id.startsWith("exam_")) {
      return {
        ...c,
        type: "text",
        align: "center",
        editor: !canEnterGrades ? undefined : "text",
      } as any;
    }
    return c;
  });
});

function handleRefresh() {
  clearDraft(gradeDraftKey());
  studentsQuery.refetch();
  gradesQuery.refetch();
  schemesQuery.refetch();
  isDirty = false;
}

function handleCellEdit(event: CellEditEvent) {
  if (!canEnterGrades) return;
  const { rowIndex, columnId, newValue } = event;
  const field = columnId;
  if (rowIndex < 0 || rowIndex >= gridSource.length) return;
  if (!String(field).startsWith("exam_")) return;

  const key = String(field).replace(/^exam_/, "");
  const meta = (gridSource[rowIndex] as any)?.[`__meta_exam_${key}`] as
    | { max_score?: number }
    | undefined;
  const max = Number(meta?.max_score) || 100;

  const updatedSource = [...gridSource];
  const row = { ...updatedSource[rowIndex]! } as Record<string, unknown>;
  const parsed = parseCellInput(newValue, max);
  row[`__cell_exam_${key}`] = parsed;
  row[field] = displayForCell(parsed);
  recalcRowLocal(row);

  updatedSource[rowIndex] = row;
  gridSource = updatedSource;
  isDirty = true;
  persistGradeDraft();
}

async function handleSaveAll() {
  if (!matrixData || !canEnterGrades) return;
  isSaving = true;

  try {
    const items: Array<
      Parameters<typeof api.batchRecordGrades>[0]["items"][number]
    > = [];
    let clearedCount = 0;

    for (let i = 0; i < gridSource.length; i++) {
      const row = gridSource[i] as any;
      const base = (baselineSource[i] || {}) as any;
      for (const key of assessmentKeys) {
        const current = canonicalCell(
          row[`__cell_exam_${key}`] as GradeCellState | null,
        );
        const original = canonicalCell(
          base[`__cell_exam_${key}`] as GradeCellState | null,
        );
        if (sameCell(current, original)) continue;
        if (!current) {
          if (original) clearedCount++;
          continue; // delete/clear not supported by batch upsert
        }

        const meta = row[`__meta_exam_${key}`] as
          | {
              max_score?: number;
              component_id?: string | null;
              component_code?: string | null;
            }
          | undefined;
        const compConfig = schemeComponents.find((c) => c.code === key);
        const maxScore = Number(
          compConfig?.max_score ?? meta?.max_score ?? 100,
        );
        const componentCode = meta?.component_code ?? compConfig?.code ?? key;
        const scoreNum = current.score ?? 0;

        items.push({
          student_id: String(row.student_id),
          period_id: selectedPeriodId,
          section_subject_id: SectionSubjectId,
          enrollment_id: row.enrollment_id
            ? String(row.enrollment_id)
            : undefined,
          score: String(scoreNum),
          max_score: String(maxScore),
          notes: null,
          component_code: componentCode,
          is_absent: current.state === "absent",
          is_excused: current.state === "excused",
          is_exempt: current.state === "exempt",
        });
      }
    }

    if (items.length === 0) {
      if (clearedCount > 0) {
        toast.info(m.common_grades_clear_saved_not_supported());
      } else {
        toast.info(m.common_no_changes_to_save());
        isDirty = false;
      }
      return;
    }

    await api.batchRecordGrades({ items });
    if (clearedCount > 0) {
      toast.success(m.common_grades_save_partial_cleared_ignored());
    } else {
      toast.success(m.common_grades_save_success());
    }
    clearDraft(gradeDraftKey());
    isDirty = false;
    gradesQuery.refetch();
  } catch (e: any) {
    if (!periodOverrides.handleError(orgId, e))
      toast.error(extractErrorMessage(e) || m.common_grades_save_error());
  } finally {
    isSaving = false;
  }
}
</script>

<div class="h-full min-h-0 w-full font-sans flex flex-col">
  {#if selectedPeriod && !isGradeEntryOpen}
    <div class="mx-3 mt-2 flex shrink-0 flex-wrap items-center justify-between gap-2 rounded-lg border bg-muted p-3 text-sm">
      <p>{isPeriodLocked ? m.common_control_period_locked_title() : m.common_control_entry_closed_title()}</p>
      {#if canRequestOverride}
        <Button variant="outline" size="sm" onclick={() => periodOverrides.open(orgId, [gradePeriodOverride(selectedPeriodId, SectionSubjectId)])}>
          {m.shared_ui_request_override()}
        </Button>
      {/if}
      <p class="text-[10px] text-muted-foreground">{m.common_control_entry_closed_desc()}</p>
    </div>
  {/if}

  {#if error}
    <div class="flex h-full flex-col items-center justify-center gap-2 p-6 text-center text-destructive">
      <p class="text-xs font-semibold">{m.common_load_error()}</p>
      <p class="text-[10px] text-muted-foreground">{extractErrorMessage(error)}</p>
      <Button variant="outline" size="sm" class="mt-2 h-8 text-xs" onclick={handleRefresh}>
        {m.common_retry()}
      </Button>
    </div>
  {:else if !gridColumns.length && !isMatrixLoading}
    <div class="flex h-full flex-col items-center justify-center gap-2 p-8 text-center">
      <p class="text-xs font-semibold text-foreground">{m.common_assessment_no_scheme()}</p>
      <p class="max-w-sm text-[11px] text-muted-foreground">{m.common_assessment_configure_hint()}</p>
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
