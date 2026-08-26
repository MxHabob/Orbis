<script lang="ts">
  import { page } from "$app/state";
  import { BookOpen, ClipboardCheck, Calculator, FileCheck2, FileText } from "@lucide/svelte";
  import { getLocale } from "$lib/paraglide/runtime";

  const orgId = $derived(String(page.params.orgId || ""));
  const classId = $derived(String(page.params.classId || ""));
  const classSubjectId = $derived(String(page.params.classSubjectId || ""));
  const isArabic = $derived(getLocale() === "ar");
  const actions = $derived([
    { path: "assessment-scheme", label: isArabic ? "نظام التقييم" : "Assessment scheme", description: isArabic ? "حدد المكونات والأوزان والحدود" : "Define components, weights, and limits", icon: ClipboardCheck },
    { path: "gradebook", label: isArabic ? "دفتر الدرجات" : "Gradebook", description: isArabic ? "أدخل الدرجات الخام للطلاب" : "Enter raw student grades", icon: Calculator },
    { path: "term-results", label: isArabic ? "نتائج الفصل" : "Term results", description: isArabic ? "راجع النتائج المحسوبة واعتمدها" : "Review and approve calculated results", icon: FileCheck2 },
    { path: "reports", label: isArabic ? "التقارير" : "Reports", description: isArabic ? "أخرج تقارير المادة الرسمية" : "Create official subject reports", icon: FileText },
  ]);
</script>

<div class="flex flex-col gap-4">
  <div class="border border-border/70 bg-muted/10 p-4"><div class="flex items-center gap-2"><BookOpen size={16} class="text-primary" /><h3 class="text-xs font-bold">{isArabic ? "مسار عمل المادة" : "Subject workflow"}</h3></div><p class="mt-2 max-w-2xl text-[11px] leading-5 text-muted-foreground">{isArabic ? "ابدأ بإعداد نظام التقييم، ثم أدخل الدرجات، وبعدها احسب النتائج الرسمية وأخرج التقارير." : "Set up the assessment scheme, enter grades, calculate official results, and produce reports."}</p></div>
  <div class="grid gap-3 sm:grid-cols-2">
    {#each actions as action (action.path)}
      {@const Icon = action.icon}
      <a href={`/${orgId}/academics/classes/${classId}/subjects/${classSubjectId}/${action.path}`} class="group border border-border/70 p-4 transition-colors hover:border-primary/50 hover:bg-muted/20"><div class="flex items-start justify-between gap-3"><div class="flex h-8 w-8 items-center justify-center border border-border/70 text-primary"><Icon size={15} /></div><span class="text-[10px] text-muted-foreground group-hover:text-primary">{isArabic ? "فتح" : "Open"}</span></div><h4 class="mt-4 text-xs font-semibold">{action.label}</h4><p class="mt-1 text-[10px] leading-4 text-muted-foreground">{action.description}</p></a>
    {/each}
  </div>
</div>