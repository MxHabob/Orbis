<!--
@component ExamsLayout
Wraps all Exams routes with a resizable sidebar layout using MainViewport.
Uses the shared SidebarNav component.
-->
<script lang="ts">
import { page } from "$app/state";
import { ClipboardList, FileText, BarChart3 } from "lucide-svelte";
import MainViewport from "$lib/shell/main-viewport.svelte";
import SidebarNav from "$lib/shell/sidebar-nav.svelte";
import type { Snippet } from "svelte";
import * as m from "$lib/paraglide/messages";

let { data, children }: { data: { orgId: string }; children: Snippet } =
  $props();

let sessionsCount = $state(0);

$effect(() => {
  const _path = page.url.pathname;
  if (data.orgId) {
    // TODO: Wire to IPC commands when available
    sessionsCount = 0;
  }
});

const basePath = $derived(`/${data.orgId}/exams`);

const navItems = $derived([
  {
    path: "",
    label: m.exams_layout_nav_overview(),
    icon: BarChart3,
  },
  {
    path: "sessions",
    label: m.exams_layout_nav_sessions(),
    icon: ClipboardList,
    badge: sessionsCount > 0 ? sessionsCount : null,
  },
  {
    path: "reports",
    label: m.exams_layout_nav_reports(),
    icon: FileText,
  },
]);
</script>

<div class="h-full w-full bg-background overflow-hidden">
  <MainViewport name="exams-workspace" leftWidth={{ default: 220, min: 180 }}>
    {#snippet left()}
      <SidebarNav
        items={navItems}
        {basePath}
        title={m.exams_layout_panel_title()}
        description={m.exams_layout_panel_desc()}
      />
    {/snippet}

    {#snippet middle()}
      <div class="h-full w-full overflow-hidden">
        {@render children()}
      </div>
    {/snippet}
  </MainViewport>
</div>
