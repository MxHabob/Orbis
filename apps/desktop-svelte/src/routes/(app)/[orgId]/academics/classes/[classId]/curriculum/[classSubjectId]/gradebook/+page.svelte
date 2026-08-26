<script lang="ts">
  import { page } from "$app/state";
  import { commands } from "$lib/core/ipc";
  import { Badge } from "@orbis/ui-svelte/badge";
  import { Button } from "@orbis/ui-svelte/button";
  import { Calculator, RefreshCcw, Save } from "@lucide/svelte";
  import { toast } from "svelte-sonner";
  import { getLocale } from "$lib/paraglide/runtime";
  import { academicContext } from "$lib/stores";
  import { OrbisGrid } from "@orbis/svelte-grid";
  import type { CellEditEvent } from "@orbis/svelte-grid";
  import { buildGradeColumns, transformMatrixToSource, recalculateRowTotals } from "$lib/tables";

  const orgId = $derived(String(page.params.orgId || ""));
  const classId = $derived(String(page.params.classId || ""));
  const classSubjectId = $derived(String(page.params.classSubjectId || ""));
  const isArabic = $derived(getLocale() === "ar");

  let matrixData = $state<any>(null);
  let gridSource = $state<any[]>([]);
  let assessmentKeys = $state<string[]>([]);
  let isMatrixLoading = $state(true);
  let isSaving = $state(false);
  let isDirty = $state(false);

  const selectedPeriodId = $derived(academicContext.effectivePeriodId || "");
  const canEnterGrades = $derived(!academicContext.isReadOnly);

  const gridColumns = $derived.by(() => {
    if (!matrixData) return [];
    return buildGradeColumns(matrixData, {
      readonly: !canEnterGrades,
      locale: getLocale(),
    });
  });

  async function loadGradeMatrix() {
    isMatrixLoading = true;
    try {
      const result = await commands.assessment.matrix(
        selectedPeriodId,
        classSubjectId,
      );

      matrixData = result;

      if (result?.students?.length > 0) {
        const first = result.students[0];
        assessmentKeys = first ? first.grades.map((g: any) => g.assessment_id || g.exam_type) : [];
      }

      gridSource = transformMatrixToSource(result, getLocale(), academicContext.gradeScaleItems);
      isDirty = false;
    } catch (e: any) {
      toast.error(e?.message || "Error loading grade matrix");
    } finally {
      isMatrixLoading = false;
    }
  }

  $effect(() => {
    if (selectedPeriodId && classId && classSubjectId) {
      loadGradeMatrix();
    }
  });

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
          const score = row[`score_${key}`];
          if (score !== null && score !== undefined) {
            studentsToSave.push({
              enrollment_id: row.enrollment_id,
              score: Number(score),
              notes: null,
              exam_type: key,
            });
          }
        }
      }

      if (studentsToSave.length === 0) {
        toast.info(isArabic ? "لا توجد تغييرات للحفظ" : "No changes to save");
        return;
      }

      await commands.assessment.bulkSubjectEntry({
        period_id: selectedPeriodId,
        class_subject_id: classSubjectId,
        students: studentsToSave,
      });

      toast.success(isArabic ? "تم حفظ الدرجات" : "Grades saved successfully");
      isDirty = false;
    } catch (e: any) {
      toast.error(e?.message || "Failed to save grades");
    } finally {
      isSaving = false;
    }
  }
</script>

<div class="flex h-[calc(100vh-160px)] w-full flex-col overflow-hidden bg-background">
  <div class="flex shrink-0 items-center justify-between border-b border-border/40 bg-muted/10 p-3">
    <div class="flex items-center gap-2">
      <Calculator size={16} class="text-primary" />
      <h3 class="text-sm font-bold">{isArabic ? "دفتر الدرجات" : "Gradebook"}</h3>
      {#if isDirty}
        <Badge variant="outline" class="ml-2 bg-amber-500/10 text-[9px] text-amber-600">{isArabic ? "تغييرات غير محفوظة" : "Unsaved changes"}</Badge>
      {/if}
    </div>
    <div class="flex items-center gap-2">
      <Button
        variant="outline"
        size="sm"
        class="h-8 w-8 p-0 cursor-pointer"
        onclick={loadGradeMatrix}
        disabled={isMatrixLoading}
      >
        <RefreshCcw size={13} class={isMatrixLoading ? "animate-spin" : ""} />
      </Button>
      <Button
        variant="default"
        size="sm"
        class="h-8 cursor-pointer gap-1.5"
        disabled={!isDirty || isSaving || !canEnterGrades}
        onclick={handleSaveAll}
      >
        <Save size={14} />
        {isArabic ? "حفظ" : "Save"}
      </Button>
    </div>
  </div>

  <div class="flex-1 overflow-hidden p-3">
    {#if isMatrixLoading && !gridSource.length}
      <div class="flex h-full items-center justify-center text-xs text-muted-foreground">
        {isArabic ? "جاري التحميل..." : "Loading..."}
      </div>
    {:else if !gridColumns.length}
      <div class="flex h-full flex-col items-center justify-center text-center text-xs text-muted-foreground">
        {isArabic ? "لا توجد مكونات تقييم لهذه المادة" : "No assessment components configured for this subject"}
      </div>
    {:else}
      <div class="h-full overflow-hidden rounded-md border border-border/60">
        <OrbisGrid
          columns={gridColumns}
          source={gridSource}
          height="100%"
          rtl={isArabic}
          readonly={!canEnterGrades}
          onCellEdit={handleCellEdit}
        />
      </div>
    {/if}
  </div>
</div>
