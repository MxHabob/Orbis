<script lang="ts">
  import { page } from "$app/state";
  import { commands } from "$lib/core/ipc";
  import { Badge } from "@orbis/ui/badge";
  import { Button } from "@orbis/ui/button";
  import { toast } from "@orbis/ui";
  import { getLocale } from "$lib/paraglide/runtime";
  import * as m from "$lib/paraglide/messages";
  import { academicContext } from "$lib/stores";
  import type { CellEditEvent } from "@orbis/svelte-grid";
  import { DataTable, buildGradeColumns, transformMatrixToSource, recalculateRowTotals } from "$lib/tables";

  const classSubjectId = $derived(String(page.params.classSubjectId || ""));
  const isArabic = $derived(getLocale() === "ar");

  // Matrix Data (Coursework)
  const selectedPeriodId = $derived(academicContext.effectivePeriodId || "");
  const canEnterGrades = $derived(!academicContext.isReadOnly);

  let matrixData = $state<any>(null);
  let gridSource = $state<any[]>([]);
  let assessmentKeys = $state<string[]>([]);
  let isMatrixLoading = $state(false);
  let isSaving = $state(false);
  let isDirty = $state(false);

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
      const result = await commands.assessment.matrix(selectedPeriodId, classSubjectId);
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
    if (selectedPeriodId && classSubjectId) {
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
        toast.info(isArabic ? "لا توجد درجات جديدة للحفظ" : "No changes to save");
        return;
      }

      await commands.assessment.bulkSubjectEntry({
        period_id: selectedPeriodId,
        class_subject_id: classSubjectId,
        students: studentsToSave,
      });

      toast.success(isArabic ? "تم حفظ جميع الدرجات بنجاح" : "Grades saved successfully");
      isDirty = false;
    } catch (e: any) {
      toast.error(e?.message || "Failed to save grades");
    } finally {
      isSaving = false;
    }
  }
</script>

<div class="flex flex-col h-full w-full gap-2.5">
  
  <!-- Sleek Top Action Toolbar (No duplicate titles) -->
  <div class="flex items-center justify-between shrink-0 px-0.5">
    
    <div class="flex items-center gap-2">
      <div class="flex items-center gap-1.5 text-xs text-muted-foreground font-medium">
        <span>
          {isArabic 
            ? `${gridSource.length} طالب في السجل` 
            : `${gridSource.length} students enrolled`}
        </span>
      </div>

      {#if isDirty}
        <Badge variant="outline" class="bg-amber-500/10 text-[10px] text-amber-600 border-amber-500/30 font-bold animate-pulse">
          {isArabic ? "تغييرات غير محفوظة" : "Unsaved changes"}
        </Badge>
      {:else if gridSource.length > 0}
        <Badge variant="outline" class="bg-emerald-500/10 text-[10px] text-emerald-600 border-emerald-500/20 font-medium">
          {isArabic ? "البيانات محفوظة" : "All changes saved"}
        </Badge>
      {/if}
    </div>

    <!-- Actions -->
    <div class="flex items-center gap-2">
      <Button 
        variant="outline" 
        size="sm" 
        class="h-8 px-2.5 text-xs font-semibold gap-1.5 cursor-pointer shadow-2xs" 
        onclick={loadGradeMatrix} 
        disabled={isMatrixLoading}
        title={isArabic ? "إعادة تحميل الدرجات" : "Reload grades"}
      >
        <span>{isArabic ? "تحديث" : "Refresh"}</span>
      </Button>

      <Button 
        variant={isDirty ? "default" : "secondary"} 
        size="sm" 
        class="h-8 px-3.5 gap-1.5 text-xs font-bold cursor-pointer shadow-2xs transition-all" 
        disabled={!isDirty || isSaving || !canEnterGrades} 
        onclick={handleSaveAll}
      >
        <span>{isArabic ? "حفظ التغييرات" : "Save Changes"}</span>
      </Button>
    </div>

  </div>
  
  <!-- Main Grid Canvas -->
  <div class="flex-1 w-full overflow-hidden border border-border/70 rounded-xl shadow-xs bg-card">
    {#if !gridColumns.length && !isMatrixLoading}
      <div class="flex flex-col h-full items-center justify-center p-8 text-center bg-muted/5 gap-2">
        <p class="text-xs font-bold text-foreground">
          {isArabic ? "لم يتم تكوين خطة تقييم لهذه الفترة" : "No assessment scheme configured for this period"}
        </p>
        <p class="text-[11px] text-muted-foreground max-w-sm">
          {isArabic ? "يرجى الضغط على زر 'إعداد التقييمات' بالأعلى لتحديد عناصر الاختبارات والواجبات وأوزانها." : "Click 'Assessments Scheme' above to configure components and weights."}
        </p>
      </div>
    {:else}
      <DataTable
        columns={gridColumns}
        source={gridSource}
        loading={isMatrixLoading}
        height="100%"
        rtl={isArabic}
        readonly={!canEnterGrades}
        oncelledit={handleCellEdit}
        emptyTitle={isArabic ? "لا يوجد طلاب في السجل" : "No students in the register"}
      />
    {/if}
  </div>

</div>