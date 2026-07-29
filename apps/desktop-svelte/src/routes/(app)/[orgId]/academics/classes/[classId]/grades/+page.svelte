<!--
@component ClassGradesPage
Unified Grades & Students page for a class.
Displays a table of enrolled students with inline grade entry fields.
Merges the concepts of "students list" and "grade entry" into one view.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { onMount } from "svelte";
import { commands } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Badge } from "@orbis/ui-svelte/badge";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import { Users, ClipboardList, Download, RefreshCcw } from "lucide-svelte";
import { toast } from "svelte-sonner";

import { getLocale } from "$lib/paraglide/runtime";
import { DataTable, AvatarCell } from "$lib/tables";
import type { OrbisColumnDef } from "@orbis/svelte-grid";
import OverrideRequestDialog from "$lib/shared/components/override-request-dialog.svelte";
import { usePeriodGate } from "$lib/shared/utils/use-period-gate";

const orgId = $derived(page.params.orgId);
const classId = $derived(page.params.classId);

// Data state
let students = $state<any[]>([]);
let assessments = $state<any[]>([]);
let subjects = $state<any[]>([]);
let periods = $state<any[]>([]);
let isLoading = $state(true);

// Filters
let selectedSubjectId = $state<string>("");
let activePeriodId = $state<string>("");
let semesters = $state<any[]>([]);
let selectedSemesterId = $state<string>("");

const gate = usePeriodGate(
  "grade_entry",
  () => selectedSemesterId || activePeriodId || "",
);
let showOverrideDialog = $state(false);

// Grade entries: { [studentId_assessmentId]: score }
let gradeEntries = $state<Record<string, string>>({});

async function loadData() {
  isLoading = true;
  try {
    const [enrollments, subjectList, periodList] = await Promise.all([
      commands.academic.listEnrollments(),
      commands.academic.listSubjects(),
      commands.calendar.listAcademicPeriods(),
    ]);

    // Filter students for this class
    students = enrollments.filter((s) => String(s.class_id) === classId);
    subjects = subjectList;
    periods = periodList;

    const activeYear =
      periodList.find(
        (c) =>
          c.period_type === "year" &&
          (c.status === "active" || c.status === "active"),
      ) ||
      periodList.find((c) => c.period_type === "year") ||
      periodList[0];

    if (activeYear) {
      activePeriodId = String(activeYear.id);
      semesters = periodList.filter(
        (c) => c.parent_id === activeYear.id && c.period_type === "term",
      );
      if (semesters.length > 0) {
        selectedSemesterId = String(semesters[0].id);
      }
    }

    if (subjectList.length > 0) {
      selectedSubjectId = String(subjectList[0].id);
    }
  } catch (e) {
    console.error("Error loading grades data:", e);
    toast.error(m.academics_grades_error_load());
  } finally {
    isLoading = false;
  }
}

// Load assessments when subject/semester changes
async function loadAssessments() {
  if (!classId || !selectedSubjectId) return;
  try {
    const list = await commands.assessment.listAssessments(
      selectedSemesterId || activePeriodId || "",
      selectedSubjectId || undefined,
    );
    assessments = list;

    // Initialize grade entries (try to load existing grades)
    const initialEntries: Record<string, string> = {};
    for (const student of students) {
      for (const assessment of list) {
        const key = `${student.id}_${assessment.id}`;
        initialEntries[key] = "";
      }
    }

    // Load existing grades if API supports it
    try {
      const period = selectedSemesterId || activePeriodId || "";
      const existingGrades = await commands.assessment
        .listGrades(period)
        .catch(() => []);

      for (const grade of existingGrades) {
        const key = `${grade.enrollment_id}_${grade.assessment_id}`;
        if (key in initialEntries) {
          initialEntries[key] = String(grade.score ?? "");
        }
      }
    } catch {
      // Grades API may not exist yet
    }

    gradeEntries = initialEntries;
  } catch (e) {
    console.error("Error loading assessments:", e);
    assessments = [];
  }
}

$effect(() => {
  if (classId && selectedSubjectId) {
    loadAssessments();
  }
});

onMount(loadData);

const maxTotal = $derived(
  assessments.reduce((sum, a) => sum + (Number(a.max_score) || 0), 0),
);

const rtl = $derived(getLocale() === "ar");

const gridColumns = $derived.by(() => {
  const cols: OrbisColumnDef[] = [
    {
      id: "__rowIndex",
      prop: "__rowIndex",
      header: "#",
      name: "#",
      width: 50,
      size: 50,
      pin: "colPinStart",
      readonly: true,
      sort: false,
      align: "center",
      cellClass: "orbis-cell-index",
    },
    {
      id: "student_name",
      prop: "student_name",
      header: m.common_word_student(),
      name: m.common_word_student(),
      width: 180,
      size: 180,

      pin: "colPinStart",
      readonly: true,
      sort: true,
      cell: AvatarCell,
    },
  ];

  for (const assessment of assessments) {
    cols.push({
      prop: `score_${assessment.id}`,
      name: `${assessment.title_ar || assessment.title || assessment.item_type || "التقييم"} (/${assessment.max_score})`,
      size: 110,

      readonly: !gate.isAllowed,
      sortable: false,
      align: "center",

      editor: "number",
      cellProperties: () => ({ class: "cell-number-input" }),
      editorConfig: {
        min: 0,
        max: Number(assessment.max_score),
        step: 0.5,
      },
    });
  }

  if (assessments.length > 0) {
    cols.push({
      prop: "__total",
      name: `${m.common_grades_col_total()} (/${maxTotal})`,
      size: 100,
      pin: "colPinEnd",
      readonly: true,
      sortable: true,
      align: "center",
      cellProperties: (row: any) => {
        const total = Number(row.__total);
        if (isNaN(total) || maxTotal <= 0)
          return { class: "orbis-cell-number text-muted-foreground font-bold" };
        const percentage = Math.round((total / maxTotal) * 100);
        let colorClass = "text-muted-foreground";
        if (percentage >= 90) colorClass = "text-emerald-500 font-bold";
        else if (percentage >= 75) colorClass = "text-primary font-bold";
        else if (percentage >= 50) colorClass = "text-amber-500 font-bold";
        else if (percentage > 0) colorClass = "text-destructive font-bold";
        return { class: `orbis-cell-number ${colorClass}` };
      },
    });
  }

  return cols;
});

const gridSource = $derived.by(() => {
  return students.map((student, idx) => {
    const studentId = String(student.id || student.student_id);
    const row: Record<string, any> = {
      __rowIndex: idx + 1,
      student_id: studentId,
      student_name:
        student.full_name || student.name_ar || student.name || studentId,
      avatar: student.avatar || null,
    };

    let total = 0;
    for (const assessment of assessments) {
      const key = `${studentId}_${assessment.id}`;
      const val = gradeEntries[key];
      const score = val !== "" && val != null ? Number(val) : null;
      row[`score_${assessment.id}`] = score;
      if (score !== null && !isNaN(score)) {
        total += score;
      }
    }
    row.__total = total;
    return row;
  });
});

async function handleCellEdit(event: any) {
  const { rowIndex, prop, newValue } = event;
  if (rowIndex < 0 || rowIndex >= students.length) return;

  const student = students[rowIndex];
  const studentId = String(student.id || student.student_id);

  if (prop.startsWith("score_")) {
    const assessmentId = prop.replace("score_", "");
    const assessment = assessments.find((a) => String(a.id) === assessmentId);
    if (!assessment) return;

    let val = newValue;
    if (val !== null && val !== undefined && val !== "") {
      const numVal = Number(val);
      if (isNaN(numVal) || numVal < 0) {
        val = "0";
      } else if (numVal > Number(assessment.max_score)) {
        val = String(assessment.max_score);
      } else {
        val = String(numVal);
      }
    } else {
      val = "";
    }

    const key = `${studentId}_${assessmentId}`;
    const oldVal = gradeEntries[key];
    if (oldVal === val) return; // No change

    gradeEntries[key] = val;

    // Real-time autosave
    try {
      const selectedSubject = subjects.find(
        (s) => String(s.id) === selectedSubjectId,
      );
      const semesterObj = semesters.find(
        (s) => String(s.id) === selectedSemesterId,
      );

      const request = {
        enrollment_id: student.id,
        subject_name: selectedSubject?.name || "Subject",
        score: val === "" ? 0 : Number(val),
        max_score: Number(assessment.max_score),
        exam_type: assessment.item_type || assessment.type || "exam",
        semester: semesterObj?.name || "Semester",
      };

      await commands.assessment.submitGrade(
        selectedSemesterId || activePeriodId || "",
        request,
      );
      toast.success(m.academics_grades_saved());
    } catch (e: unknown) {
      // Revert in-memory value if backend save fails
      gradeEntries[key] = oldVal;
      toast.error((e as any)?.message || m.academics_grades_error_save());
    }
  }
}
</script>

<div class="flex flex-col gap-5 text-start h-full">
  <!-- Filters & Actions Bar -->
  <div class="flex items-center justify-between gap-4 flex-wrap">
    <div class="flex items-center gap-3 flex-wrap">
      <!-- Subject Filter -->
      <div class="flex items-center gap-2">
        <span class="text-xs text-muted-foreground font-medium">
          {m.common_word_subject()}:
        </span>
        <Select
          type="single"
          value={selectedSubjectId}
          onValueChange={(v) => (selectedSubjectId = v ?? "")}
        >
          <SelectTrigger class="h-7 text-xs w-40 border-border/50">
            {selectedSubjectId
              ? String(
                  subjects.find((s) => String(s.id) === selectedSubjectId)
                    ?.name_ar ??
                    subjects.find((s) => String(s.id) === selectedSubjectId)
                      ?.name ??
                    selectedSubjectId,
                )
              : m.academics_grades_select_subject()}
          </SelectTrigger>
          <SelectContent>
            {#each subjects as subj (subj.id)}
              <SelectItem value={String(subj.id)}>
                {subj.name_ar || subj.name || subj.id}
              </SelectItem>
            {/each}
          </SelectContent>
        </Select>
      </div>

      <!-- Semester Filter -->
      {#if semesters.length > 0}
        <div class="flex items-center gap-2">
          <span class="text-xs text-muted-foreground font-medium">
            {m.common_word_semester()}:
          </span>
          <Select
            type="single"
            value={selectedSemesterId}
            onValueChange={(v) => (selectedSemesterId = v ?? "")}
          >
            <SelectTrigger class="h-7 text-xs w-40 border-border/50">
              {selectedSemesterId
                ? String(
                    semesters.find((s) => String(s.id) === selectedSemesterId)
                      ?.name_ar ??
                      semesters.find(
                        (s) => String(s.id) === selectedSemesterId,
                      )?.name ??
                      selectedSemesterId,
                  )
                : m.academics_grades_select_semester()}
            </SelectTrigger>
            <SelectContent>
              {#each semesters as sem (sem.id)}
                <SelectItem value={String(sem.id)}>
                  {sem.name_ar || sem.name || sem.id}
                </SelectItem>
              {/each}
            </SelectContent>
          </Select>
        </div>
      {/if}

      <!-- Student Count Badge -->
      <Badge variant="secondary" class="text-[10px] h-6 shrink-0">
        <Users size={10} class="me-1" />
        {students.length}
        {m.common_word_student_single()}
      </Badge>
    </div>

    <!-- Actions -->
    <div class="flex items-center gap-2 flex-nowrap shrink-0">
      {#if gate.requiresOverride}
        <Button
          variant="secondary"
          size="sm"
          class="h-8 text-xs gap-1.5"
          onclick={() => { showOverrideDialog = true; }}
        >
          🔑 طلب تجاوز
        </Button>
      {/if}
      <Button
        variant="outline"
        size="sm"
        class="h-8 text-xs gap-1.5"
        onclick={loadAssessments}
      >
        <RefreshCcw size={12} />
      </Button>
    </div>
  </div>

  <!-- Info banner when no assessments exist -->
  {#if assessments.length === 0 && selectedSubjectId && !isLoading}
    <div
      class="flex items-center gap-3 p-3 rounded-lg border border-border bg-muted/30 text-xs text-muted-foreground shrink-0"
    >
      <ClipboardList size={16} class="shrink-0" />
      <span>
        {m.academics_grades_no_assessments_banner()}
      </span>
    </div>
  {/if}

  <!-- Grades Table -->
  <div class="flex-1 min-h-0">
    <DataTable
      columns={gridColumns}
      source={gridSource}
      loading={isLoading}
      emptyIcon={Users}
      emptyTitle={m.academics_grades_no_students()}
      {rtl}
      height="100%"
      readonly={false}
      theme="compact"
      range={true}
      clipboard={true}
      rowSize={42}
      oncelledit={handleCellEdit}
    />
  </div>

  <OverrideRequestDialog
    bind:open={showOverrideDialog}
    orgId={orgId}
    overrideType="grade_modification"
    entityType="period"
    entityId={selectedSemesterId || activePeriodId || ""}
    riskLevel="high"
    onSuccess={() => { toast.success('تم تقديم طلب التجاوز لإدخال الدرجات بنجاح'); gate.refetch(); }}
  />
</div>
