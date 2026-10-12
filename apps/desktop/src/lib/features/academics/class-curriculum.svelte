<!--
  @component ClassCurriculum
  Feature component for Class Materials and Curriculum view.
  Uses @orbis/svelte-grid directly inside a standardized Card container.
-->
<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import { createQuery } from "@tanstack/svelte-query";
import { api, extractErrorMessage, type SectionSubjectDto, type SubjectDto } from "$lib/api";
import { Button } from "@orbis/ui/button";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardAction,
  CardContent,
} from "@orbis/ui/card";
import { localizeHref } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";
import { academicContext } from "$lib/stores";
import { OrbisGrid, type OrbisColumnDef, type GridActionEvent } from "@orbis/svelte-grid";
import { goto } from "$app/navigation";
import { BookOpen, Plus, Refresh } from "@orbis/icons";
import SectionSubjectSheet from "$lib/features/academics/components/class-subject-sheet.svelte";

interface EnrichedSectionSubject extends SectionSubjectDto {
  catalog?: SubjectDto;
  teacher_name?: string;
  is_active?: boolean;
}

let { orgId, classId }: { orgId: string; classId: string } = $props();

const periodId = $derived(academicContext.effectivePeriodId);
let showSubjectSheet = $state(false);

const subjectsQuery = createQuery(() => ({
  queryKey: ["class-subjects-natural", classId, periodId],
  queryFn: async (): Promise<EnrichedSectionSubject[]> => {
    const offerings = await api.listSectionSubjects(classId, periodId);
    const catalog = await api.listSubjects();
    const byId = new Map(catalog.map((s) => [String(s.id), s]));
    return offerings.map((o) => ({
      ...o,
      catalog: byId.get(String(o.subject_id)),
    }));
  },
  enabled: !!classId && !!periodId,
  staleTime: 5 * 60 * 1000,
}));

const subjects = $derived((subjectsQuery.data || []) as EnrichedSectionSubject[]);
const isLoading = $derived(subjectsQuery.isLoading);
const error = $derived(subjectsQuery.error);

function subjectName(subject: EnrichedSectionSubject): string {
  return subject.catalog?.name_ar || subject.catalog?.name || subject.subject_id;
}

const columns: OrbisColumnDef[] = [
  {
    id: "subject_name",
    prop: "subject_name",
    header: m.common_search_group_subjects(),
    name: m.common_search_group_subjects(),
    width: 240,
    size: 240,
    cellClass: "font-semibold text-foreground",
  },
  {
    id: "teacher_display",
    prop: "teacher_display",
    header: m.common_teacher(),
    name: m.common_teacher(),
    width: 220,
    size: 220,
    cellClass: "text-muted-foreground",
  },
  {
    id: "weekly_hours",
    prop: "weekly_hours",
    header: m.common_hours_per_week(),
    name: m.common_hours_per_week(),
    width: 130,
    size: 130,
    align: "center",
    cellClass: "font-mono font-medium",
  },
  {
    id: "status",
    prop: "status",
    header: m.common_status(),
    name: m.common_status(),
    width: 110,
    size: 110,
    type: "badge",
    align: "center",
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
  <Card class="flex-1 flex flex-col min-h-0 overflow-hidden">
    <CardHeader class="py-3 px-4 flex flex-row items-center justify-between">
      <div class="flex items-center gap-2.5">
        <div class="w-8 h-8 rounded-lg bg-primary/10 flex items-center justify-center text-primary shrink-0">
          <BookOpen class="w-4 h-4" />
        </div>
        <div class="flex flex-col text-start">
          <CardTitle class="text-xs font-bold uppercase tracking-wider text-foreground">
            {m.common_search_group_subjects()}
          </CardTitle>
          <CardDescription class="text-[10px]">
            {m.shared_ui_class_curriculum_description()}
          </CardDescription>
        </div>
      </div>

      <CardAction>
        <div class="flex items-center gap-2">
          <Button
            variant="ghost"
            size="icon-sm"
            class="text-muted-foreground"
            onclick={() => subjectsQuery.refetch()}
          >
            {#if isLoading}<LoadingIndicator  size={14} />{:else}<Refresh size={14}   />{/if}
          </Button>

          <Button
            variant="default"
            size="sm"
            class="h-8 gap-1.5 text-xs font-bold shadow-sm"
            onclick={() => (showSubjectSheet = true)}
          >
            <Plus size={14} />
            {m.common_search_add()}
          </Button>
        </div>
      </CardAction>
    </CardHeader>

    <CardContent class="p-0 flex-1 min-h-0 flex flex-col overflow-hidden">
      {#if error}
        <div class="flex flex-1 flex-col items-center justify-center py-16 text-destructive gap-2 font-sans p-6">
          <h3 class="font-semibold text-xs">{m.common_error_occurred()}</h3>
          <p class="text-[10px] text-muted-foreground">{extractErrorMessage(error)}</p>
          <Button variant="outline" size="sm" class="h-8 mt-2 text-xs" onclick={() => subjectsQuery.refetch()}>
            {m.common_retry()}
          </Button>
        </div>
      {:else}
        <div class="flex-1 min-h-0 w-full h-full">
          <OrbisGrid
            {columns}
            source={gridSource}
            loading={isLoading}
            height="100%"
            rowSize={44}
            readonly
            onaction={handleGridAction}
            emptyTitle={m.common_empty_classes_desc()}
          />
        </div>
      {/if}
    </CardContent>
  </Card>

  <SectionSubjectSheet
    bind:open={showSubjectSheet}
    {orgId}
    {classId}
    onSuccess={() => subjectsQuery.refetch()}
  />
</div>
