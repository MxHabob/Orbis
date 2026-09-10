<script lang="ts">
import { page } from "$app/state";
import { createQuery } from "@tanstack/svelte-query";
import { api } from "$lib/api";
import { Badge } from "@orbis/ui/badge";
import { Skeleton } from "@orbis/ui/skeleton";
import { Users, BookOpen, GraduationCap, ChevronRight } from "@lucide/svelte";
import { getLocale, localizeHref } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";
import { academicContext } from "$lib/stores";
import DetailPageHeader from "$lib/shell/detail-page-header.svelte";
import CapacityIndicator from "$lib/shared/components/capacity-indicator.svelte";

let { children } = $props();

const classId = $derived(String(page.params.classId || ""));
const classSubjectId = $derived(
  page.params.classSubjectId ? String(page.params.classSubjectId) : null,
);
const orgId = $derived(String(page.params.orgId || ""));
const periodId = $derived(academicContext.effectivePeriodId);
const isArabic = $derived(getLocale() === "ar");

// Class Info Query
const classQuery = createQuery(() => ({
  queryKey: ["class-detail-master-layout", classId, periodId],
  queryFn: () => api.getClass(classId),
  enabled: !!classId,
}));

// Students Count Query
const studentsQuery = createQuery(() => ({
  queryKey: ["class-students-count-master-layout", classId, periodId],
  queryFn: () => api.academic.listEnrichedStudents(classId),
  enabled: !!classId,
}));

// Subject Info Query (only when classSubjectId is present)
const subjectQuery = createQuery(() => ({
  queryKey: ["class-subject-title-layout", classId, classSubjectId, periodId],
  queryFn: async () => {
    if (!classSubjectId) return null;
    const offerings = await api.listClassSubjects(classId, periodId || "");
    const offering = offerings.find(
      (o: any) => String(o.id) === String(classSubjectId),
    );
    if (!offering) return null;
    const catalog = await api.listSubjects();
    const catSub = catalog.find(
      (s: any) => String(s.id) === String(offering.subject_id),
    );
    return { ...offering, catalog: catSub };
  },
  enabled: !!classId && !!classSubjectId,
}));

const classData = $derived(classQuery.data as any);
const subjectData = $derived(subjectQuery.data as any);
const isLoading = $derived(
  classQuery.isLoading || (!!classSubjectId && subjectQuery.isLoading),
);
const studentsCount = $derived(((studentsQuery.data || []) as any[]).length);

const classDisplayName = $derived.by(() => {
  if (!classData) return "";
  return isArabic
    ? classData.name_ar || classData.name
    : classData.name || classData.name_ar;
});

const subjectDisplayName = $derived.by(() => {
  if (!subjectData) return "";
  return isArabic
    ? subjectData.catalog?.name_ar ||
        subjectData.catalog?.name ||
        subjectData.subject_id
    : subjectData.catalog?.name ||
        subjectData.catalog?.name_ar ||
        subjectData.subject_id;
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
    return isArabic ? "مواد الفصل" : "Class Subjects";
  }
  if (isStudentsPage || isCurriculumListPage) {
    return isArabic ? "لوحة تحكم الفصل" : "Class Dashboard";
  }
  return isArabic ? "قائمة الفصول" : "Classes List";
});
</script>

<div
  class="h-full w-full flex flex-col overflow-hidden bg-background font-sans text-start"
>
  <!-- Unified Breadcrumb Header: Class Name / Subject Name -->
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
            title={isArabic
              ? "الانتقال إلى لوحة تحكم الفصل"
              : "Go to Class Dashboard"}
          >
            {classDisplayName || classId}
          </a>
          <span class="text-muted-foreground/60 text-sm font-semibold">/</span>
        {:else}
          <h1
            class="text-lg font-bold tracking-tight text-foreground truncate shrink min-w-0"
          >
            {classDisplayName || classId}
          </h1>
        {/if}

        <!-- Subpage Context: Specific Subject Name or Section -->
        {#if classSubjectId && subjectData}
          <div class="flex items-center gap-2 min-w-0">
            <h1 class="text-base font-bold text-foreground truncate">
              {subjectDisplayName}
            </h1>
            <Badge
              variant={subjectData.is_active === 0 ? "outline" : "secondary"}
              class="text-[9px] py-0 px-1.5 h-4.5 rounded font-sans uppercase shrink-0"
            >
              {subjectData.is_active === 0
                ? isArabic
                  ? "غير نشط"
                  : "Inactive"
                : isArabic
                  ? "نشط"
                  : "Active"}
            </Badge>
          </div>
        {:else if isStudentsPage}
          <h1 class="text-base font-bold text-foreground truncate">
            {isArabic ? "الطلاب" : "Students"}
          </h1>
        {:else if isCurriculumListPage}
          <h1 class="text-base font-bold text-foreground truncate">
            {isArabic ? "المواد والمناهج" : "Curriculum & Subjects"}
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
      <span class="text-sm text-muted-foreground"
        >{m.error_not_found ? m.error_not_found() : "Not found"}</span
      >
    {/if}
  </DetailPageHeader>

  <!-- Main Viewport Content -->
  <div class="flex-1 w-full overflow-y-auto p-6 scrollbar-hide">
    <div class="w-full h-full max-w-7xl mx-auto">
      {@render children()}
    </div>
  </div>
</div>
