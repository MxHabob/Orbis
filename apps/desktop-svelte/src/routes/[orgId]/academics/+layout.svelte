<!--
@component AcademicsLayout
Wraps all Academics routes inside a resizable double-sidebar layout
using <MainViewport>. Uses the shared SidebarNav component.
-->
<script lang="ts">
  import { page } from "$app/state";
  import { commands } from "$lib/services/ipc";
  import {
    GraduationCap,
    Calendar,
    Compass,
    ClipboardList,
    ShieldCheck,
  } from "lucide-svelte";
  import MainViewport from "$lib/components/layout/MainViewport.svelte";
  import SidebarNav from "$lib/components/layout/SidebarNav.svelte";
  import type { Snippet } from "svelte";
  import * as m from "$lib/paraglide/messages";

  let { data, children }: { data: { orgId: string }; children: Snippet } =
    $props();

  // Sidebar live statistics
  let classesCount = $state(0);

  async function loadSidebarStats() {
    try {
      const classList = await commands.classes.getAll(data.orgId);
      classesCount = classList.length;
    } catch (e) {
      console.error("Error loading sidebar stats:", e);
    }
  }

  // Reactively reload stats on route pathname change
  $effect(() => {
    const path = page.url.pathname;
    if (data.orgId) {
      loadSidebarStats();
    }
  });

  const basePath = $derived(`/${data.orgId}/academics`);

  const navItems = $derived([
    {
      path: "",
      label: m.academics_layout_nav_overview(),
      icon: Compass,
    },
    {
      path: "classes",
      label: m.academics_layout_nav_classes(),
      icon: GraduationCap,
      badge: classesCount > 0 ? classesCount : null,
    },
    {
      path: "schedule",
      label: m.academics_layout_nav_schedule(),
      icon: Calendar,
    },
    {
      path: "exams",
      label: m.nav_exams(),
      icon: ClipboardList,
    },
    {
      path: "control",
      label: m.academics_control_label
        ? m.academics_control_label()
        : "الكنترول",
      icon: ShieldCheck,
    },
  ]);
</script>

<div class="h-full w-full bg-background overflow-hidden">
  <MainViewport name="academics-workspace" leftWidth={{ default: 240, min: 200 }}>
    {#snippet left()}
      <SidebarNav items={navItems} {basePath} />
    {/snippet}

    {#snippet middle()}
      <div class="h-full w-full overflow-hidden">
        {@render children()}
      </div>
    {/snippet}
  </MainViewport>
</div>
