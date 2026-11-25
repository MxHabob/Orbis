<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api, extractErrorMessage } from "$lib/api";
import { Skeleton } from "@orbis/ui/skeleton";
import { toast } from "@orbis/ui";
import { slide } from "svelte/transition";
import { academicContext, authStore, uiStore } from "$lib/stores";
import { getLocale } from "$lib/paraglide/runtime";

import {
  buildGradeColumns,
  transformMatrixToSource,
  recalculateRowTotals,
  scoreToGradeLetter,
  extractSchemeComponents,
  findMonthlyScheme,
  type AssessmentColumnConfig,
  type GradeMatrixResponse,
  type GradeMatrixStudent,
  type GradeCell,
} from "./grade-helpers";
import type { CellEditEvent } from "@orbis/svelte-grid";
import * as XLSX from "xlsx";
import { ControlGrid } from "../academics";

let { orgId }: { orgId: string } = $props();

let periods = $derived(academicContext.allPeriods);
let subjects = $state<any[]>([]);
let selectedPeriodId = $state<string>("");
let selectedClassId = $state<string>("");
let selectedSubjectId = $state<string>("");
let classSectionSubjects = $state<any[]>([]);
let classSubjectsLoadKey = $state<string>("");
let isLoadingClassSubjects = $state(false);
let isLoading = $state(true);
let isMatrixLoading = $state(false);
let isSaving = $state(false);
let isDirty = $state(false);
let isExporting = $state(false);
let isImporting = $state(false);
let importFileInput: HTMLInputElement | undefined = $state();
let matrixData = $state<GradeMatrixResponse | null>(null);
let gridSource = $state<Record<string, unknown>[]>([]);
let assessmentKeys = $state<string[]>([]);
let schemeComponents = $state<AssessmentColumnConfig[]>([]);
let previousSelectedClassId: string | null = null;
const preferenceScope = () => ({
  userId: authStore.userId,
  organizationId: orgId,
});

const classSubjectOptions = $derived.by(() => {
  if (!selectedClassId || !selectedPeriodId) return [] as any[];
  const byId = new Map(subjects.map((s: any) => [String(s.id), s]));
  const seen = new Set<string>();
  const options: any[] = [];
  for (const offering of classSectionSubjects) {
    const subjectId = String((offering as any)?.subject_id ?? "");
    if (!subjectId || seen.has(subjectId)) continue;
    seen.add(subjectId);
    const subject = byId.get(subjectId);
    if (subject) options.push(subject);
  }
  return options;
});

$effect(() => {
  const periodId = selectedPeriodId;
  const classId = selectedClassId;
  const key = periodId && classId ? `${periodId}::${classId}` : "";

  if (!key) {
    classSectionSubjects = [];
    classSubjectsLoadKey = "";
    isLoadingClassSubjects = false;
    return;
  }

  let cancelled = false;
  isLoadingClassSubjects = true;
  classSubjectsLoadKey = "";
  classSectionSubjects = [];

  void (async () => {
    try {
      const offerings = await api.listSectionSubjects(classId, periodId);
      if (!cancelled) {
        classSectionSubjects = offerings ?? [];
        classSubjectsLoadKey = key;
      }
    } catch {
      if (!cancelled) {
        classSectionSubjects = [];
        classSubjectsLoadKey = key;
      }
    } finally {
      if (!cancelled) isLoadingClassSubjects = false;
    }
  })();

  return () => {
    cancelled = true;
  };
});

$effect(() => {
  if (!selectedClassId || !selectedPeriodId) {
    if (selectedSubjectId) {
      selectedSubjectId = "";
      academicContext.selectedSubjectId = "";
    }
    if (!isMatrixLoading) {
      matrixData = null;
      gridSource = [];
    }
    return;
  }

  const key = `${selectedPeriodId}::${selectedClassId}`;
  if (isLoadingClassSubjects || classSubjectsLoadKey !== key) return;

  const options = classSubjectOptions;
  if (options.length === 0) {
    if (selectedSubjectId) {
      selectedSubjectId = "";
      academicContext.selectedSubjectId = "";
    }
    if (!isMatrixLoading) {
      matrixData = null;
      gridSource = [];
    }
    return;
  }

  const hasValidSelection = options.some(
    (subject: any) => String(subject.id) === String(selectedSubjectId),
  );
  if (!hasValidSelection) {
    selectedSubjectId = String(options[0].id);
    academicContext.selectedSubjectId = selectedSubjectId;
  }
});

onMount(() => {
  selectedPeriodId =
    uiStore.getPreference(
      preferenceScope(),
      "exams.selectedPeriodId",
      "",
      (value): value is string => typeof value === "string",
    ) || academicContext.effectivePeriodId;
  if (academicContext.selectedClassId)
    selectedClassId = academicContext.selectedClassId;
  if (academicContext.selectedSubjectId)
    selectedSubjectId = academicContext.selectedSubjectId;
  previousSelectedClassId = selectedClassId;
});

$effect(() => {
  if (!periods.length) return;

  if (selectedPeriodId && !selectedPeriod) {
    selectedPeriodId =
      academicContext.activePeriod?.id ?? String(periods[0]?.id ?? "");
    return;
  }

  if (selectedPeriodId) {
    academicContext.selectedPeriodId = selectedPeriodId;
    uiStore.setPreference(
      preferenceScope(),
      "exams.selectedPeriodId",
      selectedPeriodId,
    );
  }
});
$effect(() => {
  if (selectedClassId) academicContext.selectedClassId = selectedClassId;
});
$effect(() => {
  if (selectedSubjectId) academicContext.selectedSubjectId = selectedSubjectId;
});

let selectedPeriod = $derived(
  periods.find((c) => String(c.id) === selectedPeriodId),
);
let isPeriodLocked = $derived(Boolean(selectedPeriod?.is_locked));
let isGradeEntryOpen = $derived(Boolean(selectedPeriod?.is_grade_entry_open));
let canEnterGrades = $derived(
  !isPeriodLocked && isGradeEntryOpen && !academicContext.isReadOnly,
);

let gridColumns = $derived.by(() => {
  if (!matrixData) return [];
  return buildGradeColumns(matrixData, {
    readonly: !canEnterGrades,
    locale: getLocale(),
    labels: {
      rowIndex: m.common_control_col_index(),
      studentName: m.common_word_student(),
      total: m.common_grades_col_total(),
      percentage: m.common_control_col_percentage(),
      gradeLetter: m.common_control_col_grade(),
    },
  });
});

async function loadInitialData() {
  isLoading = true;
  try {
    const [subjectList, schemes] = await Promise.all([
      api.listSubjects(),
      api.listAssessmentSchemes(),
    ]);
    subjects = subjectList;

    // Resolve the monthly-period scheme and extract its components
    const monthlyScheme = findMonthlyScheme(
      schemes as Record<string, unknown>[],
    );
    if (monthlyScheme) {
      schemeComponents = extractSchemeComponents(monthlyScheme);
    }

    // If no period selected yet, use context's effective period
    if (!selectedPeriodId && academicContext.effectivePeriodId) {
      selectedPeriodId = academicContext.effectivePeriodId;
    }
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  } finally {
    isLoading = false;
  }
}

// Reset subject and stale matrix when class changes
$effect(() => {
  const currentClassId = selectedClassId;
  if (
    previousSelectedClassId !== null &&
    currentClassId !== previousSelectedClassId
  ) {
    selectedSubjectId = "";
    academicContext.selectedSubjectId = "";
    matrixData = null;
    gridSource = [];
  }
  previousSelectedClassId = currentClassId;
});

// â”€â”€ Grade Matrix â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
async function loadGradeMatrix() {
  if (!selectedPeriodId || !selectedClassId || !selectedSubjectId) return;
  isMatrixLoading = true;
  matrixData = null;
  gridSource = [];
  isDirty = false;
  try {
    const [studentItems, sectionSubjects] = await Promise.all([
      (async () => {
        const items: any[] = [];
        let page = 1;
        let totalPages = 1;
        do {
          const result = await api.listStudents({
            page,
            page_size: 100,
            class_id: selectedClassId,
            period_id: selectedPeriodId,
          });
          items.push(...(result.items || []));
          totalPages = result.total_pages || 1;
          page += 1;
        } while (page <= totalPages);
        return items;
      })(),
      api.listSectionSubjects(selectedClassId, selectedPeriodId),
    ]);
    const sectionSubject = sectionSubjects.find(
      (offering) => String(offering.subject_id) === String(selectedSubjectId),
    );
    const gradeItems = sectionSubject
      ? await api.listGrades({ SectionSubjectId: sectionSubject.id })
      : [];

    // â”€â”€ Build assessment columns from scheme components â”€â”€â”€â”€â”€â”€â”€â”€
    const hasSchemeComponents = schemeComponents.length > 0;
    const assessmentConfigs = hasSchemeComponents
      ? schemeComponents.map((comp) => ({
          code: comp.code,
          name: comp.name,
          name_ar: comp.name_ar,
          max: comp.max_score,
          sort_order: comp.sort_order,
        }))
      : [
          // Legacy fallback when no scheme is configured
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
    const assessmentTypes = assessmentConfigs.map((c) => c.code);

    const students: GradeMatrixStudent[] = studentItems.map((st) => {
      const studentId = st.student.id;
      const matchingGrades = gradeItems.filter(
        (grade) =>
          String(grade.student_id) === String(studentId) &&
          String(grade.period_id) === String(selectedPeriodId),
      );
      const cells: GradeCell[] = assessmentConfigs.map(({ code, max }) => {
        // Match grade by component_id (component code) or fall back to is_final for legacy data
        const grade = hasSchemeComponents
          ? (matchingGrades.find(
              (item) =>
                String(item.component_code ?? item.component_id ?? "") === code,
            ) ??
            // Last-resort legacy match for old 2-column data
            (code === "final" || code === "written_test" || code === "term_exam"
              ? matchingGrades.find((item) => Boolean(item.is_final))
              : null))
          : matchingGrades.find(
              (item) => Boolean(item.is_final) === (code === "final"),
            );
        const gradeMax = Number(grade?.max_score);
        return {
          grade_id: grade?.id ?? null,
          exam_type: code,
          score: grade ? Number(grade.score) : null,
          max_score: Number.isFinite(gradeMax) && gradeMax > 0 ? gradeMax : max,
          component_id: grade?.component_id ?? null,
          component_code: grade?.component_code ?? null,
        };
      });
      const total = cells.reduce((acc, cell) => acc + (cell.score || 0), 0);
      const maxTotal = cells.reduce((acc, cell) => acc + cell.max_score, 0);
      const percentage =
        maxTotal > 0 ? Math.round((total / maxTotal) * 100) : 0;
      return {
        enrollment_id:
          st.enrollment?.id ??
          matchingGrades.find((grade) => grade.enrollment_id)?.enrollment_id ??
          studentId,
        student_id: studentId,
        student_name:
          st.person?.full_name ||
          st.person?.display_name ||
          st.student?.student_number ||
          "",
        student_name_ar:
          st.person?.full_name_ar ||
          st.person?.full_name ||
          st.person?.display_name ||
          st.student?.student_number ||
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

    const result: GradeMatrixResponse = {
      period_id: selectedPeriodId,
      section_subject_id: sectionSubject?.id ?? "",
      students,
      assessment_types: assessmentTypes,
      components: hasSchemeComponents ? schemeComponents : undefined,
    };

    matrixData = result;
    assessmentKeys = assessmentTypes;

    gridSource = transformMatrixToSource(
      result,
      getLocale(),
      academicContext.gradeScaleItems,
    );
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e) || "Error loading grade matrix");
  } finally {
    isMatrixLoading = false;
  }
}

$effect(() => {
  if (selectedPeriodId && selectedClassId && selectedSubjectId) {
    loadGradeMatrix();
  }
});

// â”€â”€ Cell Pencil Handler â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
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

// â”€â”€ Save All Grades â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
async function handleSaveAll() {
  if (!matrixData || !canEnterGrades) return;
  isSaving = true;

  try {
    const items: any[] = [];
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
          items.push({
            student_id: row.student_id,
            period_id: selectedPeriodId,
            section_subject_id: matrixData.section_subject_id,
            score: String(Number(score)),
            max_score: maxScore,
            enrollment_id: row.enrollment_id || undefined,
            component_code: meta?.component_code ?? key,
            notes: null,
          });
        }
      }
    }

    if (items.length === 0) {
      toast.warning(m.common_action_save());
      isSaving = false;
      return;
    }

    await api.batchRecordGrades({ items });
    toast.success(
      m.common_control_save_success({ total: String(items.length) }),
    );

    isDirty = false;
    await loadGradeMatrix();
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e) || "Error saving grades");
  } finally {
    isSaving = false;
  }
}

let isCalculatingRankings = $state(false);

async function handleCalculateRankings() {
  if (!selectedPeriodId || !matrixData?.students.length) return;
  isCalculatingRankings = true;
  try {
    await api.calculateRankings({
      items: matrixData.students.map(({ student_id, total }) => ({
        id: student_id,
        score: String(total),
      })),
    });
    toast.success(m.exams_control_rankings_calculated());
    await loadGradeMatrix();
  } catch (err: unknown) {
    toast.error(extractErrorMessage(err));
  } finally {
    isCalculatingRankings = false;
  }
}

onMount(() => {
  if (orgId) loadInitialData();
});

function triggerImportMatrix() {
  if (!canEnterGrades || !matrixData) return;
  if (importFileInput) {
    importFileInput.value = "";
    importFileInput.click();
  }
}

async function saveWorkbookFile(workbook: XLSX.WorkBook, filename: string) {
  try {
    const { save } = await import("@tauri-apps/plugin-dialog");
    const { writeFile } = await import("@tauri-apps/plugin-fs");
    const filePath = await save({
      defaultPath: filename,
      filters: [{ name: "Excel", extensions: ["xlsx"] }],
    });
    if (!filePath) return false;
    const buffer = XLSX.write(workbook, { bookType: "xlsx", type: "array" });
    await writeFile(filePath, new Uint8Array(buffer));
    return true;
  } catch {
    XLSX.writeFile(workbook, filename);
    return true;
  }
}

async function handleExportMatrix() {
  if (!matrixData || gridSource.length === 0) return;
  isExporting = true;
  try {
    const locale = getLocale();
    const componentKeys =
      assessmentKeys.length > 0
        ? assessmentKeys
        : matrixData.assessment_types || [];
    const rows = gridSource.map((row) => {
      const out: Record<string, unknown> = {
        student_id: row.student_id ?? "",
        enrollment_id: row.enrollment_id ?? "",
        student_name:
          (locale === "ar" ? row.student_name_ar : row.student_name) ||
          row.student_name ||
          "",
      };
      for (const key of componentKeys) {
        out[key] = row[`exam_${key}`] ?? "";
      }
      out.total = row.total ?? "";
      out.percentage = row.percentage ?? "";
      out.grade_letter = row.grade_letter ?? "";
      return out;
    });

    const headers = [
      "student_id",
      "enrollment_id",
      "student_name",
      ...componentKeys,
      "total",
      "percentage",
      "grade_letter",
    ];
    const sheet = XLSX.utils.json_to_sheet(rows, { header: headers });
    const workbook = XLSX.utils.book_new();
    XLSX.utils.book_append_sheet(workbook, sheet, "Grades");

    const metaRows = [
      { key: "period_id", value: selectedPeriodId || "" },
      { key: "section_subject_id", value: matrixData.section_subject_id || "" },
      { key: "class_id", value: selectedClassId || "" },
      { key: "subject_id", value: selectedSubjectId || "" },
      ...componentKeys.map((code) => {
        const comp = schemeComponents.find((c) => c.code === code);
        return {
          key: `component:${code}`,
          value: comp
            ? `${comp.name}${comp.name_ar ? ` / ${comp.name_ar}` : ""} (max ${comp.max_score})`
            : code,
        };
      }),
    ];
    XLSX.utils.book_append_sheet(
      workbook,
      XLSX.utils.json_to_sheet(metaRows),
      "Context",
    );

    const safeName = `grades-${(matrixData.section_subject_id || "matrix").slice(0, 8)}.xlsx`;
    await saveWorkbookFile(workbook, safeName);
    toast.success(m.common_action_export());
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e) || "Export failed");
  } finally {
    isExporting = false;
  }
}

async function handleImportMatrix(event: Event) {
  if (!matrixData || !canEnterGrades) return;
  const input = event.currentTarget as HTMLInputElement;
  const file = input.files?.[0];
  if (!file) return;

  isImporting = true;
  try {
    const buffer = await file.arrayBuffer();
    const workbook = XLSX.read(buffer, { type: "array" });
    const sheetName = workbook.SheetNames.includes("Grades")
      ? "Grades"
      : workbook.SheetNames[0];
    if (!sheetName) throw new Error("No sheet found");
    const sheet = workbook.Sheets[sheetName];
    const rows = XLSX.utils.sheet_to_json<Record<string, unknown>>(sheet, {
      defval: "",
    });
    if (rows.length === 0) {
      toast.warning(m.common_control_no_data());
      return;
    }

    const componentKeys =
      assessmentKeys.length > 0
        ? assessmentKeys
        : matrixData.assessment_types || [];
    const componentSet = new Set(componentKeys);
    const byStudentId = new Map<string, Record<string, unknown>>();
    for (const row of gridSource) {
      const id = String(row.student_id ?? "");
      if (id) byStudentId.set(id, row);
    }

    let matchedRows = 0;
    let updatedCells = 0;
    const updatedSource = gridSource.map((r) => ({ ...r }));

    for (const imported of rows) {
      const studentId = String(
        imported.student_id ??
          imported.studentId ??
          imported.StudentId ??
          imported["\u0645\u0639\u0631\u0641 \u0627\u0644\u0637\u0627\u0644\u0628"] ??
          "",
      ).trim();
      if (!studentId) continue;
      const rowIndex = updatedSource.findIndex(
        (r) => String(r.student_id ?? "") === studentId,
      );
      if (rowIndex === -1) continue;
      matchedRows++;
      const row = { ...updatedSource[rowIndex] };

      for (const key of componentKeys) {
        const raw =
          imported[key] ??
          imported[`exam_${key}`] ??
          imported[`comp:${key}`] ??
          imported[key.toUpperCase()] ??
          imported[key.toLowerCase()];
        if (raw === undefined || raw === "") continue;
        const num = Number(raw);
        if (Number.isNaN(num)) continue;
        const compConfig = schemeComponents.find((c) => c.code === key);
        const meta = row[`__meta_exam_${key}`] as
          | { max_score?: number }
          | undefined;
        const maxScore = compConfig?.max_score ?? meta?.max_score ?? 100;
        const clamped = Math.max(0, Math.min(maxScore, num));
        if (row[`exam_${key}`] !== clamped) {
          row[`exam_${key}`] = clamped;
          updatedCells++;
        }
      }

      recalculateRowTotals(row, componentKeys, academicContext.gradeScaleItems);
      updatedSource[rowIndex] = row;
    }

    if (matchedRows === 0) {
      toast.warning(m.common_control_no_data());
      return;
    }

    gridSource = updatedSource;
    isDirty = updatedCells > 0 || isDirty;
    toast.success(
      m.common_control_save_success({ total: String(updatedCells) }) +
        ` (${matchedRows} ${m.academics_students_tab_title()})`,
    );
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e) || "Import failed");
  } finally {
    isImporting = false;
    if (input) input.value = "";
  }
}
</script>

<div class="flex h-full min-h-0 w-full flex-col gap-3 overflow-hidden p-3 text-start select-none sm:gap-4 sm:p-4">
  {#if isLoading}
    <div class="flex min-h-0 flex-1 flex-col">
      <Skeleton class="h-full min-h-64 w-full rounded-xl" />
    </div>
  {:else}
    <!-- Warnings -->
    {#if selectedPeriod && isPeriodLocked}
      <div
        class="shrink-0 rounded-xl border border-destructive/20 bg-card px-4 py-2.5"
        in:slide={{ duration: 200 }}
      >
        <div class="flex flex-wrap items-center justify-between gap-x-4 gap-y-1 text-start">
          <p class="text-xs font-medium text-destructive">{m.common_control_period_locked_title()}</p>
          <p class="text-xs text-muted-foreground">{m.common_control_period_locked_desc()}</p>
        </div>
      </div>
    {:else if selectedPeriod && !isGradeEntryOpen}
      <div
        class="shrink-0 rounded-xl border border-amber-500/20 bg-card px-4 py-2.5"
        in:slide={{ duration: 200 }}
      >
        <div class="flex flex-wrap items-center justify-between gap-x-4 gap-y-1 text-start">
          <p class="text-xs font-medium text-amber-600">{m.common_control_entry_closed_title()}</p>
          <p class="text-xs text-muted-foreground">{m.common_control_entry_closed_desc()}</p>
        </div>
      </div>
    {/if}

    <ControlGrid
      bind:selectedClassId
      bind:selectedSubjectId
      {matrixData}
      {gridSource}
      {gridColumns}
      {isMatrixLoading}
      {isSaving}
      {isDirty}
      {canEnterGrades}
      {selectedPeriodId}
      subjectItems={selectedClassId && selectedPeriodId ? classSubjectOptions : undefined}
      onRefresh={loadGradeMatrix}
      onSaveAll={handleSaveAll}
      onCellEdit={handleCellEdit}
      onCalculateRankings={handleCalculateRankings}
      isCalculating={isCalculatingRankings}
      onExportTemplate={handleExportMatrix}
      onImportTemplate={triggerImportMatrix}
      {isExporting}
      {isImporting}
    />
    <input
      bind:this={importFileInput}
      type="file"
      accept=".xlsx,.xls"
      class="hidden"
      onchange={handleImportMatrix}
    />
  {/if}
</div>

