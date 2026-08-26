<script lang="ts">
  import { page } from "$app/state";
  import { onMount } from "svelte";
  import { commands } from "$lib/core/ipc";
  import { Badge } from "@orbis/ui-svelte/badge";
  import { BookOpen, ClipboardCheck, Calculator, FileCheck2, FileText } from "@lucide/svelte";
  import { getLocale } from "$lib/paraglide/runtime";
  import type { Snippet } from "svelte";

  let { children }: { children: Snippet } = $props();

  const orgId = $derived(String(page.params.orgId || ""));
  const classId = $derived(String(page.params.classId || ""));
  const classSubjectId = $derived(String(page.params.classSubjectId || ""));
  const isArabic = $derived(getLocale() === "ar");
  let subject = $state<any>(null);
  let isLoading = $state(true);

  async function loadSubject() {
    try {
      const classes = await commands.academic.listClasses();
      const classData = classes.find((item: any) => String(item.id) === classId);
      const offerings = await commands.academic.listClassSubjects(
        classId,
        String(classData?.period_id || ""),
      );
      subject = offerings.find((item: any) => String(item.id) === classSubjectId);
    } finally {
      isLoading = false;
    }
  }

  onMount(loadSubject);

  const basePath = $derived(`/${orgId}/academics/classes/${classId}/curriculum/${classSubjectId}`);
  const links = $derived([
    { path: "", label: isArabic ? "ملخص المادة" : "Subject overview", icon: BookOpen },
    { path: "assessment-scheme", label: isArabic ? "نظام التقييم" : "Assessment scheme", icon: ClipboardCheck },
    { path: "gradebook", label: isArabic ? "دفتر الدرجات" : "Gradebook", icon: Calculator },
    { path: "term-results", label: isArabic ? "نتائج الفصل" : "Term results", icon: FileCheck2 },
    { path: "reports", label: isArabic ? "التقارير" : "Reports", icon: FileText },
  ]);
</script>

<div class="flex flex-col gap-5 text-start">
  <div class="flex flex-wrap items-center justify-between gap-3 border-b border-border/60 pb-4">
    <div class="flex items-center gap-2"><BookOpen size={18} class="text-primary" /><div><h2 class="text-sm font-bold">{isLoading ? (isArabic ? "جاري تحميل المادة..." : "Loading subject...") : subject?.subject_name || subject?.name || subject?.subject_id || (isArabic ? "المادة" : "Subject")}</h2><p class="mt-1 text-[10px] text-muted-foreground">{isArabic ? "سياق المادة داخل هذا الفصل" : "Subject context within this class"}</p></div></div>
    <Badge variant={subject?.is_active === 0 ? "outline" : "secondary"} class="text-[9px]">{subject?.is_active === 0 ? (isArabic ? "غير نشطة" : "Inactive") : (isArabic ? "نشطة" : "Active")}</Badge>
  </div>
  <nav class="flex flex-wrap gap-1 border-b border-border/60 pb-2" aria-label={isArabic ? "تنقل المادة" : "Subject navigation"}>
    {#each links as link (link.path)}{@const Icon = link.icon}<a href={`${basePath}/${link.path}`} class="inline-flex items-center gap-1.5 px-3 py-2 text-[11px] font-medium text-muted-foreground hover:bg-muted/50 hover:text-foreground" class:font-semibold={page.url.pathname === `${basePath}/${link.path}` || (link.path === "" && page.url.pathname === basePath)}><Icon size={13} />{link.label}</a>{/each}
  </nav>
  <div>{@render children()}</div>
</div>