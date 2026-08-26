<script lang="ts">
  import { page } from "$app/state";
  import { onMount } from "svelte";
  import { commands } from "$lib/core/ipc";
  import { Button } from "@orbis/ui-svelte/button";
  import { Badge } from "@orbis/ui-svelte/badge";
  import { Select, SelectContent, SelectItem, SelectTrigger } from "@orbis/ui-svelte/select";
  import { FileCheck2, RefreshCcw, Calculator } from "@lucide/svelte";
  import { toast } from "svelte-sonner";
  import { getLocale } from "$lib/paraglide/runtime";

  const orgId = $derived(String(page.params.orgId || ""));
  const classId = $derived(String(page.params.classId || ""));
  const classSubjectId = $derived(String(page.params.classSubjectId || ""));
  const isArabic = $derived(getLocale() === "ar");
  let students = $state<any[]>([]);
  let results = $state<any[]>([]);
  let terms = $state<any[]>([]);
  let selectedTermId = $state("");
  let isLoading = $state(true);
  let isCalculating = $state(false);
  let error = $state("");

  const studentByEnrollment = $derived(new Map(students.map((student) => [String(student.id), student])));
  const passedCount = $derived(results.filter((result) => result.is_passing).length);

  function label(period: any): string {
    return isArabic ? period?.name_ar || period?.name || period?.id : period?.name || period?.name_ar || period?.id;
  }

  function studentName(result: any): string {
    const student = studentByEnrollment.get(String(result.enrollment_id));
    return student?.full_name || student?.name_ar || student?.name || result.student_id || "-";
  }

  async function loadResults() {
    results = selectedTermId
      ? await commands.assessment.listTermResults(selectedTermId, classSubjectId || null)
      : [];
  }

  async function loadData() {
    isLoading = true;
    error = "";
    try {
      const [periodList, enrollmentList, classList] = await Promise.all([
        commands.calendar.listAcademicPeriods(),
        commands.academic.listEnrollments(),
        commands.academic.listClasses(),
      ]);
      const classData = classList.find((item: any) => String(item.id) === classId);
      students = enrollmentList.filter((item: any) => String(item.class_id) === classId);
      const yearId = classData?.period_id || periodList.find((item: any) => item.period_type === "year" && item.status === "active")?.id;
      terms = periodList.filter((item: any) => String(item.parent_id) === String(yearId) && (item.period_type === "term" || item.period_type === "semester"));
      selectedTermId = terms.some((term) => String(term.id) === selectedTermId) ? selectedTermId : String(terms[0]?.id || yearId || "");
      await loadResults();
    } catch (cause) {
      error = cause instanceof Error ? cause.message : String(cause);
    } finally {
      isLoading = false;
    }
  }

  async function calculateResults() {
    if (!selectedTermId) return;
    isCalculating = true;
    try {
      const gradingIds = await commands.assessment.gradingChildIds(orgId, selectedTermId);
      const examId = await commands.assessment.examChildId(orgId, selectedTermId);
      await commands.assessment.calculateTermResultsBatch(selectedTermId, classId, gradingIds, examId);
      await loadResults();
      toast.success(isArabic ? "تم حساب نتائج المادة" : "Subject results calculated");
    } catch (cause) {
      toast.error(cause instanceof Error ? cause.message : String(cause));
    } finally {
      isCalculating = false;
    }
  }

  onMount(loadData);
</script>

<div class="flex flex-col gap-4 text-start">
  <div class="flex flex-wrap items-start justify-between gap-3"><div><div class="flex items-center gap-2"><FileCheck2 size={17} class="text-primary" /><h3 class="text-sm font-bold">{isArabic ? "نتائج مادة الفصل" : "Subject term results"}</h3></div><p class="mt-1 text-[10px] text-muted-foreground">{isArabic ? "النتائج الرسمية المحسوبة للمادة المحددة" : "Official calculated results for this subject"}</p></div><div class="flex items-center gap-2"><Button variant="outline" size="icon-sm" class="h-8 w-8" onclick={loadData} title={isArabic ? "تحديث" : "Refresh"}><RefreshCcw size={13} /></Button><Button variant="lime" size="sm" class="h-8 gap-1.5 text-xs" onclick={calculateResults} disabled={isCalculating || !selectedTermId}><Calculator size={13} />{isArabic ? "حساب النتائج" : "Calculate results"}</Button></div></div>
  <div class="flex flex-wrap items-center gap-3 border-y border-border/60 py-3"><Select type="single" value={selectedTermId} onValueChange={(value: string | undefined) => { selectedTermId = value || ""; loadResults(); }}><SelectTrigger class="h-8 w-48 text-xs">{label(terms.find((term) => String(term.id) === selectedTermId)) || (isArabic ? "اختر الفصل" : "Select term")}</SelectTrigger><SelectContent>{#each terms as term (term.id)}<SelectItem value={String(term.id)}>{label(term)}</SelectItem>{/each}</SelectContent></Select><Badge variant="secondary" class="text-[10px]">{results.length} {isArabic ? "نتيجة" : "results"}</Badge><Badge variant="outline" class="text-[10px]">{passedCount} {isArabic ? "ناجح" : "passed"}</Badge></div>
  {#if error}<div class="border border-destructive/30 bg-destructive/5 p-3 text-xs text-destructive">{error}</div>{:else if isLoading}<div class="border border-border/60 py-16 text-center text-xs text-muted-foreground">{isArabic ? "جاري تحميل النتائج..." : "Loading results..."}</div>{:else if results.length === 0}<div class="flex items-center justify-center border border-dashed border-border py-16 text-xs text-muted-foreground">{isArabic ? "لم يتم حساب نتائج هذه المادة بعد" : "No results calculated for this subject yet"}</div>{:else}<div class="overflow-x-auto border border-border/70"><table class="w-full min-w-190 text-xs"><thead class="border-b border-border/70 bg-muted/30 text-muted-foreground"><tr><th class="px-3 py-2 text-start">{isArabic ? "الطالب" : "Student"}</th><th class="px-3 py-2 text-end">{isArabic ? "أعمال الفصل" : "Coursework"}</th><th class="px-3 py-2 text-end">{isArabic ? "الاختبار" : "Exam"}</th><th class="px-3 py-2 text-end">{isArabic ? "المجموع" : "Total"}</th><th class="px-3 py-2 text-end">%</th><th class="px-3 py-2 text-center">{isArabic ? "التقدير" : "Grade"}</th><th class="px-3 py-2 text-center">{isArabic ? "الحالة" : "Status"}</th></tr></thead><tbody>{#each results as result (result.id)}<tr class="border-b border-border/50 last:border-0 hover:bg-muted/20"><td class="px-3 py-2 font-medium">{studentName(result)}</td><td class="px-3 py-2 text-end">{result.coursework_score ?? 0} / {result.coursework_max ?? 0}</td><td class="px-3 py-2 text-end">{result.exam_score ?? 0} / {result.exam_max ?? 0}</td><td class="px-3 py-2 text-end font-semibold">{result.total_score ?? 0} / {result.total_max ?? 0}</td><td class="px-3 py-2 text-end">{Number(result.percentage ?? 0).toFixed(1)}%</td><td class="px-3 py-2 text-center font-semibold">{result.grade_label || "-"}</td><td class="px-3 py-2 text-center"><Badge variant={result.is_passing ? "secondary" : "destructive"} class="text-[9px]">{result.is_passing ? (isArabic ? "ناجح" : "Passed") : (isArabic ? "راسب" : "Failed")}</Badge></td></tr>{/each}</tbody></table></div>{/if}
</div>