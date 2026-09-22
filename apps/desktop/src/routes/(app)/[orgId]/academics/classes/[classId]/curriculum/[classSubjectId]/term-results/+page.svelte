<script lang="ts">
import { page } from "$app/state";
import { createQuery } from "@tanstack/svelte-query";
import { api, extractErrorMessage } from "$lib/api";
import { Badge } from "@orbis/ui/badge";
import { Button } from "@orbis/ui/button";
import { AdjustmentsHorizontal, CheckCircle, Grid, InfoCircle, Refresh, XMark } from "@orbis/icons";
import { academicContext } from "$lib/stores";
import { getLocale } from "$lib/paraglide/runtime";

const classSubjectId = $derived(String(page.params.classSubjectId || ""));
const selectedPeriodId = $derived(academicContext.effectivePeriodId || "");
const isArabic = $derived(getLocale() === "ar");

const termResultsQuery = createQuery(() => ({
  queryKey: ["term-results-enriched", selectedPeriodId, classSubjectId],
  queryFn: () => api.listGrades({ classSubjectId }).catch(() => []),
  enabled: !!selectedPeriodId && !!classSubjectId,
}));

const termResults = $derived((termResultsQuery.data || []) as any[]);

function getStudentName(enrollmentId: string): string {
  const s = termResults.find((r: any) => r.enrollment_id === enrollmentId);
  if (!s) return "-";
  return isArabic
    ? s.student_name_ar || s.student_name || s.student_number
    : s.student_name || s.student_name_ar || s.student_number;
}
</script>

<div class="flex flex-col h-full w-full gap-2.5 font-sans">
  
  <!-- Action Toolbar (No duplicate titles) -->
  <div class="flex items-center justify-between shrink-0 px-0.5">
    <div class="flex items-center gap-2">
      <div class="flex items-center gap-1.5 text-xs text-muted-foreground font-medium">
        <FileCheck2 size={14} class="text-primary opacity-80" />
        <span>
          {isArabic 
            ? `${termResults.length} نتيجة مسجلة لهذا الترم` 
            : `${termResults.length} calculated student results`}
        </span>
      </div>
    </div>

    <div class="flex items-center gap-2">
      <Button 
        variant="outline" 
        size="sm" 
        class="h-8 px-2.5 gap-1.5 cursor-pointer text-xs font-semibold shadow-2xs" 
        onclick={() => termResultsQuery.refetch()} 
        disabled={termResultsQuery.isLoading}
      >
        <Refresh size={12} class={termResultsQuery.isLoading ? "animate-spin" : ""} />
        <span>{isArabic ? "تحديث" : "Refresh"}</span>
      </Button>
    </div>
  </div>

  <!-- Grid Container -->
  <div class="flex-1 overflow-auto border border-border/70 rounded-xl shadow-xs bg-card">
    {#if termResultsQuery.isLoading}
      <div class="flex h-full items-center justify-center text-xs text-muted-foreground animate-pulse">
        {isArabic ? "جاري تحميل نتائج الترم..." : "Loading term results..."}
      </div>
    {:else if termResultsQuery.error}
      <div class="flex h-full flex-col items-center justify-center text-xs text-destructive gap-2 p-6">
        <InfoCircle size={24} />
        <span class="font-semibold">{extractErrorMessage(termResultsQuery.error)}</span>
      </div>
    {:else if termResults.length === 0}
      <div class="flex flex-col h-full items-center justify-center p-8 text-center bg-muted/5 gap-2">
        <FileCheck2 size={28} class="text-muted-foreground opacity-40 mb-1" />
        <p class="text-xs font-bold text-foreground">
          {isArabic ? "لم يتم احتساب نتائج الترم بعد" : "No term results calculated yet"}
        </p>
        <p class="text-[11px] text-muted-foreground max-w-sm">
          {isArabic ? "يتم احتساب نتائج الترم تلقائياً عند اعتماد درجات التقييمات الشهرية." : "Term results are computed once coursework grades are finalized."}
        </p>
      </div>
    {:else}
      <table class="w-full min-w-160 text-xs border-collapse">
        <thead class="border-b border-border/70 bg-muted/40 text-muted-foreground sticky top-0 text-[11px] font-bold">
          <tr>
            <th class="px-4 py-3 text-start">{isArabic ? "اسم الطالب" : "Student Name"}</th>
            <th class="px-4 py-3 text-start">{isArabic ? "الدرجة العظمى" : "Max Score"}</th>
            <th class="px-4 py-3 text-start">{isArabic ? "الدرجة المستحقة" : "Obtained Score"}</th>
            <th class="px-4 py-3 text-start">{isArabic ? "التقدير" : "Grade"}</th>
            <th class="px-4 py-3 text-center">{isArabic ? "الحالة" : "Status"}</th>
          </tr>
        </thead>
        <tbody>
          {#each termResults as r (r.id)}
            <tr class="border-b border-border/40 hover:bg-muted/20 transition-colors">
              <td class="px-4 py-3 font-bold text-foreground">{getStudentName(r.enrollment_id)}</td>
              <td class="px-4 py-3 text-muted-foreground font-mono">{r.max_score}</td>
              <td class="px-4 py-3 font-bold text-foreground font-mono">{r.obtained_score}</td>
              <td class="px-4 py-3">
                <Badge variant="outline" class="text-[10px] font-bold px-2 py-0.5">
                  {r.grade_value || "-"}
                </Badge>
              </td>
              <td class="px-4 py-3 text-center">
                {#if r.is_passed === 1}
                  <Badge variant="secondary" class="bg-emerald-500/10 text-emerald-600 border-emerald-500/20 text-[10px] font-bold">
                    <CheckCircle size={11} class="me-1 inline" />
                    {isArabic ? "ناجح" : "Passed"}
                  </Badge>
                {:else}
                  <Badge variant="destructive" class="text-[10px] font-bold">
                    <XMark size={11} class="me-1 inline" />
                    {isArabic ? "راسب" : "Failed"}
                  </Badge>
                {/if}
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    {/if}
  </div>

</div>