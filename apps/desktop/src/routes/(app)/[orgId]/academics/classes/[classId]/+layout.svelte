<!--
  @component ClassLayout
  SvelteKit Layout for Class routes (/academics/classes/[classId]/*).
  Provides the cohesive header shell with breadcrumb navigation,
  class capacity badges, and sub-page context.
-->
<script lang="ts">
import { page } from "$app/state";
import { Badge } from "@orbis/ui/badge";
import { Skeleton } from "@orbis/ui/skeleton";
import { Users } from "@orbis/icons";
import { localizeHref } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";
import DetailPageHeader from "$lib/shell/detail-page-header.svelte";
import CapacityIndicator from "$lib/shared/components/capacity-indicator.svelte";
import {
  createClassQuery,
  createClassStudentsQuery,
} from "$lib/features/academics";
import type { Snippet } from "svelte";
import type { LayoutData } from "./$types";

let { data, children }: { data: LayoutData; children: Snippet } = $props();

const classId = $derived(data.classId || String(page.params.classId || ""));
const orgId = $derived(data.orgId || String(page.params.orgId || ""));
const classSubjectId = $derived(
  page.params.classSubjectId ? String(page.params.classSubjectId) : null,
);

// Class Info Query using consolidated feature query
const classQuery = createClassQuery(() => classId);
const studentsQuery = createClassStudentsQuery(() => classId);

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
    !classSubjectId,
);

// Dynamic Back navigation
const backHref = $derived.by(() => {
  if (classSubjectId) {
    return `/${orgId}/academics/classes/${classId}/curriculum`;
  }
  if (isStudentsPage || isCurriculumListPage) {
    return `/${orgId}/academics/classes/${classId}`;
  }
  return `/${orgId}/academics/classes`;
});

const backLabel = $derived.by(() => {
  if (classSubjectId) {
    return m.common_search_group_classes();
  }
  if (isStudentsPage || isCurriculumListPage) {
    return m.common_nav_dashboard();
  }
  return m.common_card_active_classes();
});
</script>

<div class="h-full w-full flex flex-col overflow-hidden bg-background font-sans text-start">
  <!-- Unified Breadcrumb Header -->
  <DetailPageHeader {backHref} {backLabel}>
    {#if isLoading}
      <Skeleton class="h-6 w-36" />
      <Skeleton class="h-4.5 w-16" />
    {:else if classData}
      <div class="flex items-center gap-2 min-w-0 flex-wrap">
        <!-- Root Class Name (Clickable link when in subpages) -->
        {#if classSubjectId || isStudentsPage || isCurriculumListPage}
          <a
            href={localizeHref(`/${orgId}/academics/classes/${classId}`)}
            class="text-base font-bold text-muted-foreground hover:text-primary transition-colors truncate shrink-0"
          >
            {classDisplayName || classId}
          </a>
          <span class="text-muted-foreground/60 text-sm font-semibold">/</span>
        {:else}
          <h1 class="text-lg font-bold tracking-tight text-foreground truncate shrink min-w-0">
            {classDisplayName || classId}
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
        {#if !classSubjectId}
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
          {#if classData.grade_level}
            <Badge
              variant="outline"
              class="text-[9px] py-0 px-1.5 h-4.5 rounded shrink-0 font-sans"
            >
              {classData.grade_level}
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

  <!-- Main Viewport Content -->
  <div class="flex-1 w-full overflow-y-auto p-6 scrollbar-hide">
    <div class="w-full h-full max-w-7xl mx-auto">
      {@render children()}
    </div>
  </div>
</div>
