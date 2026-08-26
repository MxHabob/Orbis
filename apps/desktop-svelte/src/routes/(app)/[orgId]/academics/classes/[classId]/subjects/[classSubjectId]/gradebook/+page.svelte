<script lang="ts">
  import { page } from "$app/state";
  import { onMount } from "svelte";
  import { commands } from "$lib/core/ipc";
  import { Badge } from "@orbis/ui-svelte/badge";
  import { Button } from "@orbis/ui-svelte/button";
  import { Calculator, RefreshCcw, Save } from "@lucide/svelte";
  import { toast } from "svelte-sonner";
  import { getLocale } from "$lib/paraglide/runtime";

  const orgId = $derived(String(page.params.orgId || ""));
  const classId = $derived(String(page.params.classId || ""));
  const classSubjectId = $derived(String(page.params.classSubjectId || ""));
  const isArabic = $derived(getLocale() === "ar");
  let students = $state<any[]>([]);
  let components = $state<any[]>([]);
  let selectedPeriodId = $state("");
  let periodName = $state("");
  let entries = $state<Record<string, string>>({});
  let isLoading = $state(true);
  let isSaving = $state(false);
  let error = $state("");

  function enrollmentId(student: any): string {
    return String(student.id || student.enrollment_id || "");
  }

  function studentName(student: any): string {
    return student.full_name || student.name_ar || student.name || student.student_id || "-";
  }

  async function loadGradebook() {
    isLoading = true;
    error = "";
    try {
      const [enrollments, periods, classes] = await Promise.all([
        commands.academic.listEnrollments(),
        commands.calendar.listAcademicPeriods(),
        commands.academic.listClasses(),
      ]);
      const classData = classes.find((item: any) => String(item.id) === classId);
      const offerings = await commands.academic.listClassSubjects(
        classId,
        String(classData?.period_id || ""),
      );
      students = enrollments.filter((item: any) => String(item.class_id) === classId);
      const offering = offerings.find((item: any) => String(item.id) === classSubjectId);
      const yearId = offering?.period_id || periods.find((item: any) => item.period_type === "year" && item.status === "active")?.id;
      const term = periods.find((item: any) => String(item.parent_id) === String(yearId) && (item.period_type === "term" || item.period_type === "semester"));
      selectedPeriodId = String(term?.id || yearId || "");
      periodName = isArabic ? term?.name_ar || term?.name || "" : term?.name || term?.name_ar || "";
      components = selectedPeriodId ? await commands.assessment.listComponentsForClassSubject(classSubjectId, selectedPeriodId) : [];
      const grades = selectedPeriodId ? await commands.assessment.listGradesByPeriodAndClassSubject(selectedPeriodId, classSubjectId).catch(() => []) : [];
      const nextEntries: Record<string, string> = {};
      for (const student of students) for (const component of components) nextEntries[`${enrollmentId(student)}_${component.id}`] = "";
      for (const grade of grades) {
        const key = `${grade.enrollment_id}_${grade.component_id}`;
        if (key in nextEntries) nextEntries[key] = String(grade.score ?? "");
      }
      entries = nextEntries;
    } catch (cause) {
      error = cause instanceof Error ? cause.message : String(cause);
    } finally {
      isLoading = false;
    }
  }

  async function saveScore(student: any, component: any) {
    const key = `${enrollmentId(student)}_${component.id}`;
    const value = entries[key] ?? "";
    const score = value === "" ? 0 : Number(value);
    if (Number.isNaN(score) || score < 0 || score > Number(component.max_score)) return;
    isSaving = true;
    try {
      await commands.assessment.submit({
        period_id: selectedPeriodId,
        enrollment_id: enrollmentId(student),
        subject_name: component.name || "Subject",
        subject_name_ar: null,
        score,
        max_score: Number(component.max_score),
        weight: Number(component.weight ?? 1),
        exam_type: component.component_type || "coursework",
        semester: periodName || "Term",
        notes: null,
        is_final: 0,
        assessment_id: null,
        class_subject_id: classSubjectId,
        component_id: component.id,
      });
      toast.success(isArabic ? "تم حفظ الدرجة" : "Grade saved");
    } catch (cause) {
      toast.error(cause instanceof Error ? cause.message : String(cause));
    } finally {
      isSaving = false;
    }
  }

  onMount(loadGradebook);
</script>

<div class="flex flex-col gap-4 text-start">
  <div class="flex flex-wrap items-start justify-between gap-3"><div><div class="flex items-center gap-2"><Calculator size={17} class="text-primary" /><h3 class="text-sm font-bold">{isArabic ? "دفتر درجات المادة" : "Subject gradebook"}</h3></div><p class="mt-1 text-[10px] text-muted-foreground">{periodName || (isArabic ? "الفصل الحالي" : "Current term")}</p></div><Button variant="outline" size="icon-sm" class="h-8 w-8" onclick={loadGradebook} title={isArabic ? "تحديث" : "Refresh"}><RefreshCcw size={13} class={isLoading ? "animate-spin" : ""} /></Button></div>
  {#if error}<div class="border border-destructive/30 bg-destructive/5 p-3 text-xs text-destructive">{error}</div>
  {:else if isLoading}<div class="border border-border/60 py-16 text-center text-xs text-muted-foreground">{isArabic ? "جاري تحميل دفتر الدرجات..." : "Loading gradebook..."}</div>
  {:else if components.length === 0}<div class="border border-dashed border-border py-16 text-center text-xs text-muted-foreground">{isArabic ? "لا توجد مكونات تقييم لهذه المادة" : "No assessment components configured for this subject"}</div>
  {:else}<div class="overflow-x-auto border border-border/70"><table class="w-full min-w-190 text-xs"><thead class="border-b border-border/70 bg-muted/30 text-muted-foreground"><tr><th class="px-3 py-2 text-start">{isArabic ? "الطالب" : "Student"}</th>{#each components as component (component.id)}<th class="px-3 py-2 text-center">{component.name || component.component_type}<span class="block text-[9px] font-normal">/{component.max_score} · {component.weight}%</span></th>{/each}</tr></thead><tbody>{#each students as student (student.id)}<tr class="border-b border-border/50 last:border-0 hover:bg-muted/20"><td class="px-3 py-2 font-medium">{studentName(student)}</td>{#each components as component (component.id)}{@const key = `${enrollmentId(student)}_${component.id}`}<td class="px-2 py-1.5"><div class="flex items-center gap-1"><input class="h-7 w-20 border border-border bg-background px-2 text-center text-xs outline-none focus:border-primary" type="number" min="0" max={component.max_score} step="0.5" bind:value={entries[key]} onkeydown={(event) => event.key === "Enter" && saveScore(student, component)} /><button class="inline-flex h-7 w-7 items-center justify-center text-muted-foreground hover:bg-muted hover:text-primary" onclick={() => saveScore(student, component)} disabled={isSaving} title={isArabic ? "حفظ" : "Save"} aria-label={isArabic ? "حفظ" : "Save"}><Save size={12} /></button></div></td>{/each}</tr>{/each}</tbody></table>{#if students.length === 0}<div class="py-12 text-center text-xs text-muted-foreground">{isArabic ? "لا يوجد طلاب مسجلون" : "No enrolled students"}</div>{/if}</div><div class="flex items-center justify-between text-[10px] text-muted-foreground"><span>{students.length} {isArabic ? "طالب" : "students"}</span><Badge variant="outline" class="text-[9px]">{components.length} {isArabic ? "مكون" : "components"}</Badge></div>{/if}
</div>
