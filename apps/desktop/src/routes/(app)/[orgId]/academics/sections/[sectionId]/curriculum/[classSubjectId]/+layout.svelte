<!--
  @component SectionSubjectLayout
  SvelteKit Layout for specific Subject Offering under a Class.
  Provides tab navigation between the coursework gradebook and assessment records,
  along with assessment scheme configuration.
-->
<script lang="ts">
import { page } from "$app/state";
import { createQuery } from "@tanstack/svelte-query";
import { api } from "$lib/api";
import type { SectionSubjectDto, SubjectDto, UserDto } from "$lib/api/types";
import { Button } from "@orbis/ui/button";
import SecondaryMenu from "$lib/shell/secondary-menu.svelte";
import * as m from "$lib/paraglide/messages";
import { academicContext } from "$lib/stores";
import AssessmentSchemeSheet from "$lib/features/assessment/assessment-scheme-sheet.svelte";
import type { Snippet } from "svelte";
import type { LayoutData } from "./$types";

let { data, children }: { data: LayoutData; children: Snippet } = $props();

const orgId = $derived(data.orgId || String(page.params.orgId || ""));
const sectionId = $derived(data.sectionId || String(page.params.sectionId || ""));
const SectionSubjectId = $derived(
  data.SectionSubjectId || String(page.params.SectionSubjectId || ""),
);

let showConfigSheet = $state(false);

export interface SectionSubjectWithCatalog extends SectionSubjectDto {
  catalog?: SubjectDto;
  teacher_name?: string | null;
}

// Subject Offering Details
const subjectQuery = createQuery(() => ({
  queryKey: ["subject-info-sub-layout", sectionId, SectionSubjectId],
  queryFn: async (): Promise<SectionSubjectWithCatalog> => {
    const offerings = await api.listSectionSubjects(
      sectionId,
      academicContext.effectivePeriodId || "",
    );
    const offering = offerings.find(
      (o: SectionSubjectDto) => String(o.id) === SectionSubjectId,
    );
    if (!offering) throw new Error("Subject not found");
    const catalog = await api.listSubjects();
    const catSub = catalog.find(
      (s: SubjectDto) => String(s.id) === String(offering.subject_id),
    );
    return { ...offering, catalog: catSub };
  },
  enabled: !!sectionId && !!SectionSubjectId,
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
  `/${orgId}/academics/sections/${sectionId}/curriculum/${SectionSubjectId}`,
);

const menuItems = $derived([
  {
    path: "",
    label: m.common_coursework_gradebook(),
  },
  {
    path: "assessments",
    label: "Assessment records",
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

    </div>
  </div>

  <!-- Main Content Space -->
  <div class="flex-1 w-full overflow-hidden flex flex-col">
    {@render children()}
  </div>
</div>

{#if showConfigSheet}
  <AssessmentSchemeSheet bind:open={showConfigSheet} {SectionSubjectId} periodId={academicContext.effectivePeriodId || ""} />
{/if}
