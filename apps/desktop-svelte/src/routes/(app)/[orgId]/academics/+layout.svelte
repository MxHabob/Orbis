<!--
@component AcademicsLayout
Wraps all Academics routes inside a resizable double-sidebar layout
using <MainViewport>. Uses the shared SidebarNav component.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { commands } from "$lib/core/ipc";
import { onMount } from "svelte";
import {
  GraduationCap,
  Calendar,
  Compass,
  ClipboardList,
  ShieldCheck,
  CalendarDays,
  CalendarClock,
  TrendingUp,
} from "@lucide/svelte";
import MainViewport from "$lib/shell/main-viewport.svelte";
import SidebarNav from "$lib/shell/sidebar-nav.svelte";
import ArchivedBanner from "$lib/features/academics/archived-banner.svelte";
import type { Snippet } from "svelte";

let { data, children }: { data: { orgId: string }; children: Snippet } =
  $props();

// Sidebar live statistics
let classesCount = $state(0);

async function loadSidebarStats() {
  try {
    const classList = await commands.academic.listClasses();
    classesCount = classList.length;
  } catch (e) {
    console.error("Error loading sidebar stats:", e);
  }
}

// Load once on mount — not on every sub-route change
onMount(() => {
  if (data.orgId) loadSidebarStats();
});

const basePath = $derived(`/${data.orgId}/academics`);

const navItems = $derived([
  {
    path: "",
    label: m.common_nav_overview(),
    icon: Compass,
  },
  {
    path: "classes",
    label: m.common_search_group_classes(),
    icon: GraduationCap,
    badge: classesCount > 0 ? classesCount : null,
  },
  {
    path: "schedule",
    label: m.common_exams_detail_schedule(),
    icon: Calendar,
  },
  {
    path: "exams",
    label: m.common_nav_exams ? m.common_nav_exams() : "الامتحانات",
    icon: ClipboardList,
  },
  {
    path: "promotion",
    label: m.common_promo_title(),
    icon: TrendingUp,
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
