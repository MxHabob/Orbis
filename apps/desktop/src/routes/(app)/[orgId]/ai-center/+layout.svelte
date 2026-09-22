<!--
@component AiCenterLayout
Full-width workspace layout for AI Center with a header sub-navigation bar using <SecondaryMenu>.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { AdjustmentsHorizontal, Camera, Cube, GlobeAlt, Settings, TaskList, UserBadgeCheck } from "@orbis/icons";
import SecondaryMenu from "$lib/shell/secondary-menu.svelte";
import type { Snippet } from "svelte";
let { data, children }: { data: { orgId: string }; children: Snippet } =
  $props();

const basePath = $derived(`/${data.orgId}/ai-center`);

const navItems = $derived([
  {
    path: "",
    label: m.ai_center_nav_overview
      ? m.ai_center_nav_overview()
      : "النظرة العامة",
    icon: GlobeAlt,
  },
  {
    path: "live",
    label: m.ai_center_nav_live ? m.ai_center_nav_live() : "البث المباشر",
    icon: Camera,
  },
  {
    path: "integrations",
    label: m.ai_center_nav_integrations
      ? m.ai_center_nav_integrations()
      : "التكاملات",
    icon: Cube,
  },
  {
    path: "permissions",
    label: m.ai_center_nav_permissions
      ? m.ai_center_nav_permissions()
      : "الصلاحيات",
    icon: UserBadgeCheck,
  },
  {
    path: "settings",
    label: m.ai_center_nav_settings ? m.ai_center_nav_settings() : "الإعدادات",
    icon: Settings,
  },
  {
    path: "ai-logs",
    label: m.ai_center_nav_logs ? m.ai_center_nav_logs() : "سجل الأنشطة",
    icon: TaskList,
  },
]);
</script>

<div
  class="h-full w-full bg-background flex flex-col overflow-hidden"
>
  <header class="border-b border-border/40 px-6 pt-2 pb-0 shrink-0">
    <SecondaryMenu items={navItems} {basePath} />
  </header>

  <!-- Route Content Viewport -->
  <main class="flex-1 overflow-hidden">
    {@render children()}
  </main>
</div>
