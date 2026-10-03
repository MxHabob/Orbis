<!--
@component AcademicsLayout
Wraps all Academics routes inside a resizable double-sidebar layout
using <MainViewport>. Uses the shared SidebarNav component.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { createClassesQuery } from "$lib/features/academics";
import { AcademicCap, AdjustmentsHorizontal, BookOpen, Calendar, ChartBar, DocumentCheck, TaskList, UserBadgeCheck } from "@orbis/icons";
import MainViewport from "$lib/shell/main-viewport.svelte";
import SidebarNav from "$lib/shell/sidebar-nav.svelte";
import ArchivedBanner from "$lib/features/academics/archived-banner.svelte";
import type { Snippet } from "svelte";
import type { LayoutData } from "./$types";

let { data, children }: { data: LayoutData; children: Snippet } =
  $props();

const classesQuery = createClassesQuery(() => data.orgId);
const classesCount = $derived(classesQuery.data?.length ?? 0);

const basePath = $derived(`/${data.orgId}/academics`);

const navItems = $derived([
  {
    path: "",
    label: m.common_nav_overview(),
    icon: AcademicCap,
  },
  {
    path: "classes",
    label: m.common_search_group_classes(),
    icon: BookOpen,
    badge: classesCount > 0 ? classesCount : null,
  },
  {
    path: "schedule",
    label: m.common_exams_detail_schedule(),
    icon: Calendar,
  },
  {
    path: "exams",
    label: m.common_nav_exams(),
    icon: DocumentCheck,
  },
  {
    path: "promotion",
    label: m.common_promo_title(),
    icon: ChartBar,
  },
  {
    path: "reports",
    label: m.common_nav_reports(),
    icon: ChartBar,
  },
]);
</script>

<div class="h-full w-full bg-background overflow-hidden">
  <MainViewport name="academics-workspace" leftWidth={{ default: 240, min: 200 }}>
    {#snippet left()}
      <div class="flex flex-col h-full">
        <div class="flex-1 overflow-y-auto">
          <SidebarNav
            items={navItems}
            {basePath}
            title={m.academics_layout_panel_title()}
          />
        </div>
      </div>
    {/snippet}

    {#snippet middle()}
      <div class="h-full w-full flex flex-col overflow-hidden">
        <ArchivedBanner />
        <div class="flex-1 overflow-hidden">
          {@render children()}
        </div>
      </div>
    {/snippet}
  </MainViewport>
</div>
