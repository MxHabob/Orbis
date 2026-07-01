<!--
@component ClassDetailLayout
Master-Detail layout for a single class.
Provides a DetailPageHeader (back to classes list) and SecondaryMenu tabs
(Overview | Assessments | Teachers | Grades & Students).
Uses Pattern A: MainViewport with class info sidebar.
-->
<script lang="ts">
  import { page } from "$app/state";
  import { onMount } from "svelte";
  import { commands } from "$lib/services/ipc";
  import { Badge } from "@orbis/ui-svelte/badge";
  import { Skeleton } from "@orbis/ui-svelte/skeleton";
  import DetailPageHeader from "$lib/components/layout/DetailPageHeader.svelte";
  import SecondaryMenu from "$lib/components/layout/SecondaryMenu.svelte";
  import {
    GraduationCap,
    Users,
    Award,
    BookUser,
    ClipboardList,
  } from "lucide-svelte";
  import type { Snippet } from "svelte";
  import * as m from "$lib/paraglide/messages";
  import { getLocale } from "$lib/paraglide/runtime";

  let { children }: { children: Snippet } = $props();

  const orgId = $derived(page.params.orgId);
  const classId = $derived(page.params.classId);

  let classData = $state<any>(null);
  let studentsCount = $state(0);
  let isLoading = $state(true);

  async function loadClassData() {
    if (!orgId || !classId) return;
    isLoading = true;
    try {
      const [classes, enrollments] = await Promise.all([
        commands.classes.getAll(orgId),
        commands.enrollments.getAll(orgId),
      ]);
      classData = classes.find((c) => String(c.id) === classId);
      studentsCount = enrollments.filter(
        (s) => String(s.class_id) === classId,
      ).length;
    } catch (e) {
      console.error("Error loading class data:", e);
    } finally {
      isLoading = false;
    }
  }

  onMount(loadClassData);

  const basePath = $derived(`/${orgId}/academics/classes/${classId}`);

  const menuItems = $derived([
    {
      path: "",
      label: m.academics_class_detail_tab_overview
        ? m.academics_class_detail_tab_overview()
        : "نظرة عامة",
      icon: GraduationCap,
    },
    {
      path: "assessments",
      label: m.academics_class_detail_tab_assessments
        ? m.academics_class_detail_tab_assessments()
        : "التقييمات",
      icon: Award,
    },
    {
      path: "teachers",
      label: m.academics_class_detail_tab_teachers
        ? m.academics_class_detail_tab_teachers()
        : "المعلمون",
      icon: BookUser,
    },
    {
      path: "grades",
      label: m.academics_class_detail_tab_grades
        ? m.academics_class_detail_tab_grades()
        : "الدرجات والطلاب",
      icon: ClipboardList,
    },
  ]);
</script>

<div
  class="h-full w-full flex flex-col overflow-hidden bg-background"
  dir={getLocale() === "ar" ? "rtl" : "ltr"}
>
  <!-- Detail Page Header -->
  <DetailPageHeader
    backHref={`/${orgId}/academics/classes`}
    backLabel={m.academics_layout_nav_classes
      ? m.academics_layout_nav_classes()
      : "الفصول"}
  >
    {#if isLoading}
      <Skeleton class="h-6 w-32" />
      <Skeleton class="h-4.5 w-16" />
    {:else if classData}
      <h1
        class="text-lg font-bold tracking-tight text-foreground truncate shrink min-w-0"
      >
        {classData.name_ar || classData.name || classId}
      </h1>
      <Badge
        variant="secondary"
        class="text-[9px] py-0 px-1.5 h-4.5 rounded shrink-0"
      >
        <Users size={10} class="me-1" />
        {studentsCount}
      </Badge>
      {#if classData.grade_level}
        <Badge
          variant="outline"
          class="text-[9px] py-0 px-1.5 h-4.5 rounded shrink-0"
        >
          {classData.grade_level}
        </Badge>
      {/if}
    {:else}
      <span class="text-sm text-muted-foreground"
        >{m.error_not_found ? m.error_not_found() : "غير موجود"}</span
      >
    {/if}
  </DetailPageHeader>

  <!-- Content Area -->
  <div class="flex-1 w-full overflow-y-auto p-6 scrollbar-hide text-start">
    <div class="flex flex-col gap-6 w-full h-full">
      <!-- Secondary Menu Tabs -->
      <SecondaryMenu items={menuItems} {basePath} />

      <!-- Children Sub-Route Viewport -->
      <div class="w-full flex-1">
        {@render children()}
      </div>
    </div>
  </div>
</div>
