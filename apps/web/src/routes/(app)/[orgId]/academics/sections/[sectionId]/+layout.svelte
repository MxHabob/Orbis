<!--
  @component ClassLayout
  SvelteKit Layout for Section routes (/academics/sections/[sectionId]/*).
  Provides the cohesive header shell with breadcrumb navigation,
  class capacity badges, and sub-page context.
-->
<script lang="ts">
import { goto } from "$app/navigation";
import { page } from "$app/state";
import { useQueryClient } from "@tanstack/svelte-query";
import { toast } from "@orbis/ui";
import { Badge } from "@orbis/ui/badge";
import { Skeleton } from "@orbis/ui/skeleton";
import { Pencil, Share, Sparkles, Trash, Users } from "@orbis/icons";
import { localizeHref } from "$lib/paraglide/runtime";
import { l } from "$lib/core/i18n/locale";
import * as m from "$lib/paraglide/messages";
import { api, extractErrorMessage, queryKeys } from "$lib/api";
import { confirmStore } from "$lib/stores";
import DetailPageHeader from "$lib/shell/detail-page-header.svelte";
import SplitActionMenu from "$lib/shared/components/split-action-menu.svelte";
import type { SplitActionItem } from "$lib/shared/components/split-action-menu.svelte";
import ClassCreateSheet from "$lib/features/academics/components/class-create-sheet.svelte";
import CapacityIndicator from "$lib/shared/components/capacity-indicator.svelte";
import {
  createClassQuery,
  createClassStudentsQuery,
} from "$lib/features/academics";
import type { Snippet } from "svelte";
import type { LayoutData } from "./$types";

let { data, children }: { data: LayoutData; children: Snippet } = $props();
const queryClient = useQueryClient();

const sectionId = $derived(
  data.sectionId || String(page.params.sectionId || ""),
);
const orgId = $derived(data.orgId || String(page.params.orgId || ""));
const SectionSubjectId = $derived(
  page.params.classSubjectId ? String(page.params.classSubjectId) : null,
);

// Class Info Query using consolidated feature query
const classQuery = createClassQuery(() => sectionId);
const studentsQuery = createClassStudentsQuery(() => sectionId);
let isEditSheetOpen = $state(false);

const classData = $derived(classQuery.data);
const students = $derived(studentsQuery.data ?? []);
const isLoading = $derived(classQuery.isLoading);
const studentsCount = $derived(students.length);

const classDisplayName = $derived.by(() => {
  if (!classData) return "";
  return classData.name_ar || classData.name;
});

const currentPath = $derived(page.url.pathname);
const isStudentsPage = $derived(
  currentPath.endsWith("/students") || currentPath.endsWith("/students/"),
);
const isCurriculumListPage = $derived(
  (currentPath.endsWith("/curriculum") ||
    currentPath.endsWith("/curriculum/")) &&
    !SectionSubjectId,
);

// Dynamic Back navigation
const backHref = $derived.by(() => {
  if (SectionSubjectId) {
    return `/${orgId}/academics/sections/${sectionId}/curriculum`;
  }
  if (isStudentsPage || isCurriculumListPage) {
    return `/${orgId}/academics/sections/${sectionId}`;
  }
  return `/${orgId}/academics/sections`;
});

const backLabel = $derived.by(() => {
  if (SectionSubjectId) {
    return m.common_search_group_classes();
  }
  if (isStudentsPage || isCurriculumListPage) {
    return m.common_nav_dashboard();
  }
  return m.common_card_active_classes();
});

async function handleShareLink() {
  try {
    await navigator.clipboard.writeText(page.url.href);
    toast.success(m.common_copied_to_clipboard());
  } catch (error) {
    toast.error(extractErrorMessage(error));
  }
}

function handleAskOrbis() {
  const newUrl = new URL(page.url.href);
  newUrl.searchParams.set(
    "ask_orbis",
    m.academics_class_evaluation_ai_ask_orbis_query({ name: classDisplayName }),
  );
  goto(newUrl.pathname + newUrl.search, {
    replaceState: true,
    keepFocus: true,
    noScroll: true,
  });
}

async function handleDeleteSection() {
  if (!classData) return;
  const displayName =
    l(classData.name_ar, classData.name) || classData.code || sectionId;
  const confirmed = await confirmStore.prompt({
    title: m.common_action_delete(),
    description: m.academics_class_delete_confirm({ name: displayName }),
    confirmText: m.common_action_delete(),
    variant: "destructive",
  });
  if (!confirmed) return;

  try {
    await api.deleteSection(sectionId);
    toast.success(m.academics_class_delete_success());
    await queryClient.invalidateQueries({
      queryKey: queryKeys.academic.classes(orgId),
    });
    await goto(localizeHref(`/${orgId}/academics/sections`), {
      replaceState: true,
    });
  } catch (error) {
    toast.error(extractErrorMessage(error));
  }
}

const classHeaderActions = $derived<SplitActionItem[]>([
  {
    id: "edit-section",
    label: m.common_action_edit(),
    icon: Pencil,
    onSelect: () => (isEditSheetOpen = true),
  },
  {
    id: "ask-orbis",
    label: m.academics_student_evaluation_ask_orbis_btn_short(),
    icon: Sparkles,
    onSelect: handleAskOrbis,
  },
  {
    id: "delete-section",
    label: m.common_action_delete(),
    icon: Trash,
    separatorBefore: true,
    destructive: true,
    onSelect: handleDeleteSection,
  },
]);
</script>

<div class="h-full w-full flex flex-col overflow-hidden bg-background font-sans text-start">
  <!-- Unified Breadcrumb Header -->
  {#snippet headerActions()}
    {#if classData}
      <SplitActionMenu
        label={m.common_action_share()}
        menuLabel={m.operations_ui_actions()}
        icon={Share}
        actions={classHeaderActions}
        onprimary={handleShareLink}
      />
    {/if}
  {/snippet}
  <DetailPageHeader {backHref} {backLabel} actions={headerActions}>
    {#if isLoading}
      <Skeleton class="h-6 w-36" />
      <Skeleton class="h-4.5 w-16" />
    {:else if classData}
      <div class="flex items-center gap-2 min-w-0 flex-wrap">
        <!-- Root Class Name (Clickable link when in subpages) -->
        {#if SectionSubjectId || isStudentsPage || isCurriculumListPage}
          <a
            href={localizeHref(`/${orgId}/academics/sections/${sectionId}`)}
            class="text-base font-bold text-muted-foreground hover:text-primary transition-colors truncate shrink-0"
          >
            {classDisplayName || sectionId}
          </a>
          <span class="text-muted-foreground/60 text-sm font-semibold">/</span>
        {:else}
          <h1 class="text-lg font-bold tracking-tight text-foreground truncate shrink min-w-0">
            {classDisplayName || sectionId}
          </h1>
        {/if}

        <!-- Subpage Context Title -->
        {#if isStudentsPage}
          <h1 class="text-base font-bold text-foreground truncate">
            {m.academics_class_detail_tab_students()}
          </h1>
        {:else if isCurriculumListPage}
          <h1 class="text-base font-bold text-foreground truncate">
            {m.common_search_group_subjects()}
          </h1>
        {/if}

        <!-- Class Capacity & Badges (shown on main dashboard) -->
        {#if !SectionSubjectId}
          <Badge
            variant="secondary"
            class="text-[9px] py-0 px-1.5 h-4.5 rounded shrink-0 font-sans"
          >
            <Users size={10} class="me-1" />
            {studentsCount}
          </Badge>
          <div class="w-24 shrink-0 flex items-center">
            <CapacityIndicator
              current={studentsCount}
              max={classData.capacity ?? 30}
              size="sm"
            />
          </div>
          {#if classData.level}
            <Badge
              variant="outline"
              class="text-[9px] py-0 px-1.5 h-4.5 rounded shrink-0 font-sans"
            >
              {classData.level}
            </Badge>
          {/if}
        {/if}
      </div>
    {:else}
      <span class="text-sm text-muted-foreground">
        {m.error_not_found()}
      </span>
    {/if}
  </DetailPageHeader>

  <ClassCreateSheet
    bind:open={isEditSheetOpen}
    {orgId}
    section={classData}
    onSuccess={() => {
      void classQuery.refetch();
    }}
  />

  <!-- Main Viewport Content -->
  <div class="flex-1 w-full overflow-y-auto p-6 scrollbar-hide">
    <div class="w-full h-full max-w-7xl mx-auto">
      {@render children()}
    </div>
  </div>
</div>
