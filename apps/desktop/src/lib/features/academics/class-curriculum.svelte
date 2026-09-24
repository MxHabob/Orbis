<!--
  @component ClassCurriculum
  Feature component for Class Materials and Curriculum view.
  Uses @orbis/svelte-grid / DataTable to display subjects assigned to a class.
-->
<script lang="ts">
import { createQuery } from "@tanstack/svelte-query";
import { api, extractErrorMessage } from "$lib/api";
import { Button } from "@orbis/ui/button";
import { localizeHref } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";
import { academicContext } from "$lib/stores";
import { DataTable, BadgeCell } from "$lib/tables";
import type { OrbisColumnDef, GridActionEvent } from "@orbis/svelte-grid";
import { goto } from "$app/navigation";
import ClassSubjectSheet from "$lib/features/academics/components/class-subject-sheet.svelte";

let { orgId, classId }: { orgId: string; classId: string } = $props();

const periodId = $derived(academicContext.effectivePeriodId);
let showSubjectSheet = $state(false);

const subjectsQuery = createQuery(() => ({
  queryKey: ["class-subjects-natural", classId, periodId],
  queryFn: async () => {
    const offerings = await api.listClassSubjects(classId, periodId);
    const catalog = await api.listSubjects();
    const byId = new Map(catalog.map((s: any) => [String(s.id), s]));
    return offerings.map((o: any) => ({
      ...o,
      catalog: byId.get(String(o.subject_id)),
    }));
  },
  enabled: !!classId && !!periodId,
  staleTime: 5 * 60 * 1000,
}));

const subjects = $derived((subjectsQuery.data || []) as any[]);
const isLoading = $derived(subjectsQuery.isLoading);
const error = $derived(subjectsQuery.error);

function subjectName(subject: any): string {
  return subject.catalog?.name_ar || subject.catalog?.name || subject.subject_id;
}

const columns: OrbisColumnDef[] = [
  {
    id: "subject_name",
    prop: "subject_name",
    header: m.common_search_group_subjects ? m.common_search_group_subjects() : "Subject",
    name: m.common_search_group_subjects ? m.common_search_group_subjects() : "Subject",
    width: 240,
    size: 240,
    cellClass: "font-medium",
  },
  {
    id: "teacher_display",
    prop: "teacher_display",
    header: m.common_teacher ? m.common_teacher() : "Teacher",
    name: m.common_teacher ? m.common_teacher() : "Teacher",
    width: 220,
    size: 220,
  },
  {
    id: "weekly_hours",
    prop: "weekly_hours",
    header: m.common_hours_per_week ? m.common_hours_per_week() : "Hours / Week",
    name: m.common_hours_per_week ? m.common_hours_per_week() : "Hours / Week",
    width: 130,
    size: 130,
    align: "center",
  },
  {
    id: "status",
    prop: "status",
    header: m.common_status ? m.common_status() : "Status",
    name: m.common_status ? m.common_status() : "Status",
    width: 110,
    size: 110,
    cell: BadgeCell,
  },
];

const gridSource = $derived(
  subjects.map((subject) => ({
    ...subject,
    subject_name: subjectName(subject),
    teacher_display: subject.teacher_name || subject.primary_teacher_id || "-",
    weekly_hours: subject.weekly_hours || "-",
    status: subject.is_active === 0 ? "inactive" : "active",
  })),
);

function handleGridAction(event: GridActionEvent) {
  if (event.actionId === "dblclick" && event.row.id) {
    goto(
      localizeHref(
        `/${orgId}/academics/classes/${classId}/curriculum/${event.row.id}`,
      ),
    );
  }
}
</script>

<div class="flex flex-col gap-4 text-start h-full font-sans">
  <div class="flex items-center justify-between pb-3">
    <div>
      <h1 class="text-lg font-bold tracking-tight text-foreground">
        {m.academics_overview_card_classes_desc ? m.academics_overview_card_classes_desc() : "Materials & Curriculum"}
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
    <div class="flex flex-col items-center justify-center py-16 text-destructive border border-destructive/20 bg-destructive/5 rounded-xl gap-2 font-sans">
      <h3 class="font-semibold text-xs">{m.common_error_occurred ? m.common_error_occurred() : "Failed to load subjects"}</h3>
      <p class="text-[10px] text-muted-foreground">{extractErrorMessage(error)}</p>
      <Button variant="outline" size="sm" class="h-8 mt-2 text-xs" onclick={() => subjectsQuery.refetch()}>
        {m.common_retry ? m.common_retry() : "Retry"}
      </Button>
    </div>
  {:else}
    <DataTable
      {columns}
      source={gridSource}
      loading={isLoading}
      height="100%"
      rowSize={48}
      onaction={handleGridAction}
      emptyTitle={m.common_empty_classes_desc ? m.common_empty_classes_desc() : "No subjects assigned"}
    />
  {/if}

  <ClassSubjectSheet
    bind:open={showSubjectSheet}
    {orgId}
    {classId}
    onSuccess={() => subjectsQuery.refetch()}
  />
</div>
