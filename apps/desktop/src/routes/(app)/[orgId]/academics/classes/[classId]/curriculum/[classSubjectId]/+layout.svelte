<script lang="ts">
  import { page } from "$app/state";
  import { createQuery } from "@tanstack/svelte-query";
  import { api, } from "$lib/api";
  import { Button } from "@orbis/ui/button";
  import SecondaryMenu from "$lib/shell/secondary-menu.svelte";
  import { getLocale } from "$lib/paraglide/runtime";
  import { academicContext } from "$lib/stores";
  import AssessmentSchemeSheet from "$lib/features/assessment/assessment-scheme-sheet.svelte";
  import ReportViewer from "$lib/shared/components/report-viewer.svelte";

  let { children } = $props();

  const orgId = $derived(String(page.params.orgId || ""));
  const classId = $derived(String(page.params.classId || ""));
  const classSubjectId = $derived(String(page.params.classSubjectId || ""));
  const isArabic = $derived(getLocale() === "ar");

  let showConfigSheet = $state(false);
  let showPrintDialog = $state(false);

  // Subject Offering Details
  const subjectQuery = createQuery(() => ({
    queryKey: ["subject-info-sub-layout", classId, classSubjectId],
    queryFn: async () => {
      const offerings = await api.listClassSubjects(classId, academicContext.effectivePeriodId || "");
      const offering = offerings.find((o: any) => String(o.id) === classSubjectId);
      if (!offering) throw new Error("Subject not found");
      const catalog = await api.listSubjects();
      const catSub = catalog.find((s: any) => String(s.id) === String(offering.subject_id));
      return { ...offering, catalog: catSub };
    },
    enabled: !!classId && !!classSubjectId,
  }));

  const usersQuery = createQuery(() => ({
    queryKey: ["all-users-subject-sub-layout"],
    queryFn: () => api.listUsers(null),
    staleTime: 10 * 60 * 1000,
  }));

  const subjectInfo = $derived(subjectQuery.data);
  const users = $derived((usersQuery.data || []) as any[]);

  const teacherDisplayName = $derived.by(() => {
    if (!subjectInfo) return "";
    if (subjectInfo.teacher_name) return subjectInfo.teacher_name;
    const tid = subjectInfo.primary_teacher_id;
    if (!tid) return isArabic ? "غير مسند لمعلم" : "Unassigned";
    const foundUser = users.find((u) => String(u.id) === String(tid));
    if (foundUser) return isArabic ? foundUser.name_ar || foundUser.full_name || foundUser.name : foundUser.full_name || foundUser.name || foundUser.name_ar;
    if (String(tid).startsWith("dev-")) return isArabic ? "معلم تجريبي" : "Demo Teacher";
    return isArabic ? "معلم المادة" : "Subject Teacher";
  });

  const basePath = $derived(`/${orgId}/academics/classes/${classId}/curriculum/${classSubjectId}`);

  const menuItems = $derived([
    {
      path: "",
      label: isArabic ? "دفتر الدرجات" : "Coursework Gradebook",
    },
    {
      path: "term-results",
      label: isArabic ? "نتائج الترم" : "Term Results",
    },
  ]);
</script>

<div class="flex h-full w-full flex-col font-sans text-start gap-4">
  
  <!-- Sub-Toolbar: SecondaryMenu Tabs & Subject Action Controls -->
  <div class="flex flex-wrap items-center justify-between gap-3 border-b border-border/40 pb-3 shrink-0">
    
    <!-- Secondary Menu Tab Switcher -->
    <SecondaryMenu items={menuItems} {basePath} />

    <!-- Meta & Actions on the Right -->
    <div class="flex items-center gap-2">
      {#if subjectInfo}
        <div class="hidden sm:flex items-center gap-2 px-2.5 py-1 rounded-md bg-muted/30 text-[11px] text-muted-foreground me-1">
          <span class="font-medium text-foreground">{teacherDisplayName}</span>
          {#if subjectInfo.weekly_hours}
            <span class="opacity-30">•</span>
               <span>{subjectInfo.weekly_hours} {isArabic ? "ساعات/أسبوع" : "hrs/wk"}</span>
          {/if}
        </div>
      {/if}

      <Button
        variant="outline"
        size="sm"
        class="h-8 gap-1.5 text-xs font-semibold"
        onclick={() => showConfigSheet = true}
      >
        <span>{isArabic ? "إعداد التقييمات" : "Assessments Scheme"}</span>
      </Button>

      <Button
        variant="outline"
        size="sm"
        class="h-8 gap-1.5 text-xs font-semibold"
        onclick={() => showPrintDialog = true}
      >
        <span class="hidden sm:inline">{isArabic ? "طباعة" : "Print"}</span>
      </Button>
    </div>

  </div>

  <!-- Main Content Space -->
  <div class="flex-1 w-full overflow-hidden flex flex-col">
    {@render children()}
  </div>

</div>

{#if showConfigSheet}
  <AssessmentSchemeSheet bind:open={showConfigSheet} {classSubjectId} periodId={academicContext.effectivePeriodId || ""} />
{/if}

<ReportViewer
  bind:open={showPrintDialog}
  templateId="33333333-3333-3333-3333-333333333333"
  parameters={{ class_subject_id: classSubjectId, period_id: academicContext.effectivePeriodId }}
  reportTitle={isArabic ? "طباعة سجل الدرجات" : "Print Gradebook"}
/>
