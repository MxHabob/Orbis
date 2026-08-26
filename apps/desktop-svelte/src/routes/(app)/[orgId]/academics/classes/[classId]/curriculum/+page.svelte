<script lang="ts">
  import { page } from "$app/state";
  import { createQuery } from "@tanstack/svelte-query";
  import { commands, extractErrorMessage } from "$lib/core/ipc";
  import { Badge } from "@orbis/ui-svelte/badge";
  import { Button } from "@orbis/ui-svelte/button";
  import { localizeHref, getLocale } from "$lib/paraglide/runtime";
  import * as m from "$lib/paraglide/messages";
  import { academicContext } from "$lib/stores";
  import { DataTable, BadgeCell } from "$lib/tables";
  import type { OrbisColumnDef, GridActionEvent } from "@orbis/svelte-grid";
  import { goto } from "$app/navigation";
  import ClassSubjectSheet from "$lib/features/academics/components/class-subject-sheet.svelte";

  const classId = $derived(String(page.params.classId || ""));
  const orgId = $derived(String(page.params.orgId || ""));
  const periodId = $derived(academicContext.effectivePeriodId);
  const isArabic = $derived(getLocale() === "ar");
  let showSubjectSheet = $state(false);

  const subjectsQuery = createQuery(() => ({
    queryKey: ["class-subjects-natural", classId, periodId],
    queryFn: async () => {
      const offerings = await commands.academic.listClassSubjects(classId, periodId);
      const catalog = await commands.academic.listSubjects();
      const byId = new Map(catalog.map((s: any) => [String(s.id), s]));
      return offerings.map((o: any) => ({ ...o, catalog: byId.get(String(o.subject_id)) }));
    },
    enabled: !!classId && !!periodId,
    staleTime: 5 * 60 * 1000,
  }));

  const subjects = $derived((subjectsQuery.data || []) as any[]);
  const isLoading = $derived(subjectsQuery.isLoading);
  const error = $derived(subjectsQuery.error);

  function subjectName(subject: any): string {
    return isArabic 
      ? subject.catalog?.name_ar || subject.catalog?.name || subject.subject_id 
      : subject.catalog?.name || subject.catalog?.name_ar || subject.subject_id;
  }

  const columns = $derived.by<OrbisColumnDef[]>(() => [
    {
      id: "subject_name",
      prop: "subject_name",
      header: isArabic ? "المادة" : "Subject",
      name: isArabic ? "المادة" : "Subject",
      width: 240,
      size: 240,
      cellClass: "font-medium",
    },
    {
      id: "teacher_display",
      prop: "teacher_display",
      header: isArabic ? "المعلم" : "Teacher",
      name: isArabic ? "المعلم" : "Teacher",
      width: 220,
      size: 220,
    },
    {
      id: "weekly_hours",
      prop: "weekly_hours",
      header: isArabic ? "الساعات / أسبوع" : "Hours / Week",
      name: isArabic ? "الساعات / أسبوع" : "Hours / Week",
      width: 130,
      size: 130,
      align: "center",
    },
    {
      id: "status",
      prop: "status",
      header: isArabic ? "الحالة" : "Status",
      name: isArabic ? "الحالة" : "Status",
      width: 110,
      size: 110,
      cell: BadgeCell,
    },
  ]);

  const gridSource = $derived(
    subjects.map((subject) => ({
      ...subject,
      subject_name: subjectName(subject),
      teacher_display: subject.teacher_name || subject.primary_teacher_id || "-",
      weekly_hours: subject.weekly_hours || "-",
      status: subject.is_active === 0
        ? (isArabic ? "غير نشط" : "Inactive")
        : (isArabic ? "نشط" : "Active"),
    })),
  );

  function handleGridAction(event: GridActionEvent) {
    if (event.actionId === "dblclick" && event.row.id) {
      goto(localizeHref(`/${orgId}/academics/classes/${classId}/curriculum/${event.row.id}`));
    }
  }
</script>

<div class="flex flex-col gap-4 text-start h-full font-sans">
  <div class="flex items-center justify-between pb-3">
    <div>
      <h1 class="text-lg font-bold tracking-tight text-foreground">
        {isArabic ? "المواد والمناهج" : "Materials & Curriculum"}
      </h1>
    </div>
    <Button
      variant="default"
      size="sm"
      class="h-8 gap-1.5 text-xs font-semibold"
      onclick={() => (showSubjectSheet = true)}
    >
      {m.common_search_add ? m.common_search_add() : "Assign Subject"}
    </Button>
  </div>

  {#if error}
    <div class="flex flex-col items-center justify-center py-16 text-destructiverounded-lg gap-2 font-sans">
      <h3 class="font-semibold text-xs">Failed to load subjects</h3>
      <p class="text-[10px] text-muted-foreground">{extractErrorMessage(error)}</p>
    </div>
  {:else if isLoading}
    <DataTable
      {columns}
      source={gridSource}
      loading={true}
      height="100%"
      rtl={isArabic}
    />
  {:else}
    <DataTable
      {columns}
      source={gridSource}
      height="100%"
      rowSize={48}
      rtl={isArabic}
      onaction={handleGridAction}
      emptyTitle={isArabic ? "لا توجد مواد مرتبطة" : "No subjects assigned"}
    />
  {/if}

  <ClassSubjectSheet
    bind:open={showSubjectSheet}
    {orgId}
    {classId}
    onSuccess={() => subjectsQuery.refetch()}
  />
</div>
