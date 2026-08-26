<script lang="ts">
  import { page } from "$app/state";
  import { FileText, ArrowRight } from "@lucide/svelte";
  import { getLocale } from "$lib/paraglide/runtime";

  const orgId = $derived(String(page.params.orgId || ""));
  const classId = $derived(String(page.params.classId || ""));
  const classSubjectId = $derived(String(page.params.classSubjectId || ""));
  const isArabic = $derived(getLocale() === "ar");
</script>

<div class="flex flex-col gap-4">
  <div class="flex items-center gap-2"><FileText size={17} class="text-primary" /><div><h3 class="text-sm font-bold">{isArabic ? "تقارير المادة" : "Subject reports"}</h3><p class="mt-1 text-[10px] text-muted-foreground">{isArabic ? "التقارير الرسمية يجب أن تعتمد على نتائج الفصل المعتمدة" : "Official reports are based on approved term results"}</p></div></div>
  <div class="border border-border/70 bg-muted/10 p-4 text-xs"><p class="text-muted-foreground">{isArabic ? "راجع نتائج الفصل أولًا، ثم استخدم أدوات التقارير بعد اعتمادها. هذا يمنع إخراج تقرير مبني على درجات الإدخال الخام." : "Review term results first, then use reporting tools after approval. This prevents reports from using raw grade entries."}</p><a class="mt-4 inline-flex items-center gap-2 text-primary hover:underline" href={`/${orgId}/academics/classes/${classId}/subjects/${classSubjectId}/term-results`}>{isArabic ? "فتح نتائج الفصل" : "Open term results"}<ArrowRight size={13} /></a></div>
</div>