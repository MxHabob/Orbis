<!--
@component ClassGradesPage
Unified Grades & Students page for a class.
Displays a table of enrolled students with inline grade entry fields.
Merges the concepts of "students list" and "grade entry" into one view.
-->
<script lang="ts">
import { page } from "$app/state";
import { onMount } from "svelte";
import { commands } from "$lib/services/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { Badge } from "@orbis/ui-svelte/badge";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@orbis/ui-svelte/table";
import {
  Users,
  ClipboardList,
  Save,
  Download,
  RefreshCcw,
} from "lucide-svelte";
import { toast } from "svelte-sonner";
import { fade } from "svelte/transition";
import * as m from "$lib/paraglide/messages";

const orgId = $derived(page.params.orgId);
const classId = $derived(page.params.classId);

// Data state
let students = $state<any[]>([]);
let assessments = $state<any[]>([]);
let subjects = $state<any[]>([]);
let cycles = $state<any[]>([]);
let isLoading = $state(true);
let isSaving = $state(false);

// Filters
let selectedSubjectId = $state<string>("");
let activeCycleId = $state<string>("");
let semesters = $state<any[]>([]);
let selectedSemesterId = $state<string>("");

// Grade entries: { [studentId_assessmentId]: score }
let gradeEntries = $state<Record<string, string>>({});
let hasUnsavedChanges = $state(false);

async function loadData() {
  isLoading = true;
  try {
    const [enrollments, subjectList, cycleList] = await Promise.all([
      commands.enrollments.getAll(orgId),
      commands.subjects.getAll(orgId),
      commands.academics.getCycles(orgId),
    ]);

    // Filter students for this class
    students = enrollments.filter((s) => String(s.class_id) === classId);
    subjects = subjectList;
    cycles = cycleList;

    const activeYear =
      cycleList.find(
        (c) =>
          c.type === "year" && (c.is_current === 1 || c.is_current === true),
      ) ||
      cycleList.find((c) => c.type === "year") ||
      cycleList[0];

    if (activeYear) {
      activeCycleId = String(activeYear.id);
      semesters = cycleList.filter(
        (c) => c.parent_id === activeYear.id && c.type === "term",
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
    toast.error("خطأ في تحميل البيانات");
  } finally {
    isLoading = false;
  }
}

// Load assessments when subject/semester changes
async function loadAssessments() {
  if (!classId || !selectedSubjectId) return;
  try {
    const list = await commands.assessments.getByClass({
      classId,
      subjectId: selectedSubjectId || undefined,
      termId: selectedSemesterId || activeCycleId || "",
    });
    assessments = list;

    // Initialize grade entries (try to load existing grades)
    gradeEntries = {};
    for (const student of students) {
      for (const assessment of list) {
        const key = `${student.id}_${assessment.id}`;
        gradeEntries[key] = "";
      }
    }

    // Load existing grades if API supports it
    try {
      const existingGrades = await commands.grades
        .getByClass(classId, {
          subjectId: selectedSubjectId,
          termId: selectedSemesterId || undefined,
        })
        .catch(() => []);

      for (const grade of existingGrades) {
        const key = `${grade.student_id}_${grade.assessment_id}`;
        if (key in gradeEntries) {
          gradeEntries[key] = String(grade.score ?? "");
        }
      }
    } catch {
      // Grades API may not exist yet
    }

    hasUnsavedChanges = false;
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

function updateGrade(studentId: string, assessmentId: string, value: string) {
  const key = `${studentId}_${assessmentId}`;
  gradeEntries[key] = value;
  hasUnsavedChanges = true;
}

async function saveAllGrades() {
  isSaving = true;
  try {
    const entries = Object.entries(gradeEntries)
      .filter(([_, value]) => value !== "")
      .map(([key, value]) => {
        const [studentId, assessmentId] = key.split("_");
        return {
          student_id: studentId,
          assessment_id: assessmentId,
          score: Number(value),
          class_id: classId,
          subject_id: selectedSubjectId,
        };
      });

    if (entries.length > 0) {
      await commands.grades.saveBulk(entries);
      toast.success(m.grades_saved ? m.grades_saved() : "تم حفظ الدرجات بنجاح");
      hasUnsavedChanges = false;
    }
  } catch (e: unknown) {
    toast.error((e as any)?.message || "فشل حفظ الدرجات");
  } finally {
    isSaving = false;
  }
}

// Calculate student total from entered grades
function getStudentTotal(studentId: string): number {
  let total = 0;
  for (const assessment of assessments) {
    const key = `${studentId}_${assessment.id}`;
    const score = Number(gradeEntries[key]);
    if (!isNaN(score)) total += score;
  }
  return total;
}

const maxTotal = $derived(
  assessments.reduce((sum, a) => sum + (Number(a.max_score) || 0), 0),
);
</script>

<div class="flex flex-col gap-5 text-start">
  <!-- Filters & Actions Bar -->
  <div class="flex items-center justify-between gap-4 flex-wrap">
    <div class="flex items-center gap-3 flex-wrap">
      <!-- Subject Filter -->
      <div class="flex items-center gap-2">
        <span class="text-xs text-muted-foreground font-medium">
          {m.assessments_subject_filter
            ? m.assessments_subject_filter()
            : "المادة"}:
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
              : "اختر مادة"}
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
            {m.assessments_semester_filter
              ? m.assessments_semester_filter()
              : "الفصل الدراسي"}:
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
                : "اختر فترة"}
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
        {m.classes_enrolled_students
          ? m.classes_enrolled_students()
          : "طالب"}
      </Badge>
    </div>

    <!-- Actions -->
    <div class="flex items-center gap-2 flex-nowrap shrink-0">
      <Button
        variant="outline"
        size="sm"
        class="h-8 text-xs gap-1.5"
        onclick={loadAssessments}
      >
        <RefreshCcw size={12} />
      </Button>
      <Button
        variant="lime"
        size="sm"
        class="h-8 text-xs gap-1.5 font-bold"
        disabled={!hasUnsavedChanges || isSaving}
        onclick={saveAllGrades}
      >
        <Save size={12} />
        {isSaving
          ? "..."
          : m.grades_save_all
            ? m.grades_save_all()
            : "حفظ الدرجات"}
      </Button>
    </div>
  </div>

  <!-- Grades Table -->
  {#if isLoading}
    <div class="flex flex-col gap-3">
      {#each Array(5) as _}
        <Skeleton class="h-12 w-full rounded-lg" />
      {/each}
    </div>
  {:else if students.length === 0}
    <div
      class="flex flex-col items-center justify-center py-12 text-muted-foreground border border-border border-dashed rounded-xl gap-3"
    >
      <Users size={40} class="text-muted-foreground/30" />
      <p class="text-xs font-semibold">لا يوجد طلاب مسجلون في هذا الفصل</p>
    </div>
  {:else}
    <!-- Info banner when no assessments exist -->
    {#if assessments.length === 0 && selectedSubjectId}
      <div
        class="flex items-center gap-3 p-3 rounded-lg border border-border bg-muted/30 text-xs text-muted-foreground"
      >
        <ClipboardList size={16} class="shrink-0" />
        <span>
          لا توجد تقييمات لهذه المادة بعد. أضف تقييمات من تبويب "التقييمات" لتتمكن من إدخال الدرجات.
        </span>
      </div>
    {/if}

    <div
      class="border border-border rounded-xl overflow-auto"
      in:fade={{ duration: 200 }}
    >
      <Table>
        <TableHeader>
          <TableRow class="bg-muted/30">
            <TableHead class="text-xs font-bold sticky start-0 bg-muted/30 z-10 min-w-[50px]">
              #
            </TableHead>
            <TableHead class="text-xs font-bold sticky start-[50px] bg-muted/30 z-10 min-w-[160px]">
              {m.grades_col_student ? m.grades_col_student() : "الطالب"}
            </TableHead>
            {#each assessments as assessment (assessment.id)}
              <TableHead class="text-xs font-bold text-center min-w-[100px]">
                <div class="flex flex-col gap-0.5">
                  <span>{assessment.name || assessment.type}</span>
                  <span class="text-[9px] text-muted-foreground font-normal">
                    /{assessment.max_score}
                  </span>
                </div>
              </TableHead>
            {/each}
            {#if assessments.length > 0}
              <TableHead class="text-xs font-bold text-center min-w-[80px]">
                {m.grades_col_total ? m.grades_col_total() : "المجموع"}
                <span class="text-[9px] text-muted-foreground font-normal block">
                  /{maxTotal}
                </span>
              </TableHead>
            {/if}
          </TableRow>
        </TableHeader>
        <TableBody>
          {#each students as student, idx (student.id || student.student_id)}
            {@const studentId = String(student.id || student.student_id)}
            {@const total = getStudentTotal(studentId)}
            {@const percentage = maxTotal > 0 ? Math.round((total / maxTotal) * 100) : 0}
            <TableRow class="hover:bg-accent/20 transition-colors">
              <TableCell class="text-xs text-muted-foreground font-mono sticky start-0 bg-background z-10">
                {idx + 1}
              </TableCell>
              <TableCell class="text-xs font-medium sticky start-[50px] bg-background z-10">
                {student.full_name || student.name_ar || student.name || studentId}
              </TableCell>
              {#each assessments as assessment (assessment.id)}
                {@const key = `${studentId}_${assessment.id}`}
                <TableCell class="text-center p-1">
                  <Input
                    type="number"
                    min="0"
                    max={String(assessment.max_score)}
                    value={gradeEntries[key] || ""}
                    oninput={(e) =>
                      updateGrade(
                        studentId,
                        String(assessment.id),
                        e.currentTarget.value,
                      )}
                    class="h-8 w-20 text-center text-xs mx-auto [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
                    placeholder="—"
                  />
                </TableCell>
              {/each}
              {#if assessments.length > 0}
                <TableCell class="text-center">
                  <div class="flex flex-col items-center gap-0.5">
                    <span
                      class="text-xs font-bold font-mono {percentage >= 90
                        ? 'text-emerald-500'
                        : percentage >= 75
                          ? 'text-primary'
                          : percentage >= 50
                            ? 'text-amber-500'
                            : percentage > 0
                              ? 'text-destructive'
                              : 'text-muted-foreground'}"
                    >
                      {total || "—"}
                    </span>
                    {#if total > 0}
                      <span class="text-[9px] text-muted-foreground"
                        >{percentage}%</span
                      >
                    {/if}
                  </div>
                </TableCell>
              {/if}
            </TableRow>
          {/each}
        </TableBody>
      </Table>
    </div>
  {/if}
</div>
