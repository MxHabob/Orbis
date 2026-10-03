<!--
  @component ClassSubjectLayout
  SvelteKit Layout for specific Subject Offering under a Class.
  Provides tab navigation between Coursework Gradebook and Term Results,
  along with assessments scheme configuration and PDF print triggers.
-->
<script lang="ts">
import { page } from "$app/state";
import { createQuery } from "@tanstack/svelte-query";
import { api } from "$lib/api";
import type { ClassSubjectDto, SubjectDto, UserDto } from "$lib/api/types";
import { Button } from "@orbis/ui/button";
import SecondaryMenu from "$lib/shell/secondary-menu.svelte";
import * as m from "$lib/paraglide/messages";
import { academicContext } from "$lib/stores";
import AssessmentSchemeSheet from "$lib/features/assessment/assessment-scheme-sheet.svelte";
import PdfReportViewer from "$lib/shared/components/pdf-report-viewer.svelte";
import type { Snippet } from "svelte";
import type { LayoutData } from "./$types";

let { data, children }: { data: LayoutData; children: Snippet } = $props();

const orgId = $derived(data.orgId || String(page.params.orgId || ""));
const classId = $derived(data.classId || String(page.params.classId || ""));
const classSubjectId = $derived(
  data.classSubjectId || String(page.params.classSubjectId || ""),
);

let showConfigSheet = $state(false);
let showPrintDialog = $state(false);

export interface ClassSubjectWithCatalog extends ClassSubjectDto {
  catalog?: SubjectDto;
  teacher_name?: string | null;
}

// Subject Offering Details
const subjectQuery = createQuery(() => ({
  queryKey: ["subject-info-sub-layout", classId, classSubjectId],
  queryFn: async (): Promise<ClassSubjectWithCatalog> => {
    const offerings = await api.listClassSubjects(
      classId,
      academicContext.effectivePeriodId || "",
    );
    const offering = offerings.find(
      (o: ClassSubjectDto) => String(o.id) === classSubjectId,
    );
    if (!offering) throw new Error("Subject not found");
    const catalog = await api.listSubjects();
    const catSub = catalog.find(
      (s: SubjectDto) => String(s.id) === String(offering.subject_id),
    );
    return { ...offering, catalog: catSub };
  },
  enabled: !!classId && !!classSubjectId,
}));

const usersQuery = createQuery(() => ({
  queryKey: ["all-users-subject-sub-layout"],
  queryFn: (): Promise<UserDto[]> => api.listUsers(),
  staleTime: 10 * 60 * 1000,
}));

const subjectInfo = $derived(subjectQuery.data);
const users = $derived(usersQuery.data ?? []);

const teacherDisplayName = $derived.by(() => {
  if (!subjectInfo) return "";
  if (subjectInfo.teacher_name) return subjectInfo.teacher_name;
  const tid = subjectInfo.primary_teacher_id;
  if (!tid) return m.common_unassigned();
  const foundUser = users.find((u: UserDto) => String(u.id) === String(tid));
  if (foundUser) {
    return foundUser.full_name_ar || foundUser.full_name;
  }
  return m.common_teacher();
});

const basePath = $derived(
  `/${orgId}/academics/classes/${classId}/curriculum/${classSubjectId}`,
);

const menuItems = $derived([
  {
    path: "",
    label: m.common_coursework_gradebook(),
  },
  {
    path: "term-results",
    label: m.common_term_results(),
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
            <span>{subjectInfo.weekly_hours} {m.common_hours_per_week()}</span>
          {/if}
        </div>
      {/if}

      <Button
        variant="outline"
        size="sm"
        class="h-8 gap-1.5 text-xs font-semibold"
        onclick={() => showConfigSheet = true}
      >
        <span>{m.common_assessment_scheme()}</span>
      </Button>

      <Button
        variant="outline"
        size="sm"
        class="h-8 gap-1.5 text-xs font-semibold"
        onclick={() => showPrintDialog = true}
      >
        <span class="hidden sm:inline">{m.common_action_print()}</span>
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

<PdfReportViewer
  bind:open={showPrintDialog}
  templateId="33333333-3333-3333-3333-333333333333"
  parameters={{ class_subject_id: classSubjectId, period_id: academicContext.effectivePeriodId }}
  reportTitle={m.common_print_gradebook()}
/>
