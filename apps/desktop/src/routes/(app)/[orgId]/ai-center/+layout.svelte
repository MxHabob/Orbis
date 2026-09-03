<!--
@component AiCenterLayout
Full-width workspace layout for AI Center with a header sub-navigation bar using <SecondaryMenu>.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import {
  Compass,
  ScrollText,
  Settings,
  Shield,
  Video,
  Plug,
} from "@lucide/svelte";
import SecondaryMenu from "$lib/shell/secondary-menu.svelte";
import type { Snippet } from "svelte";
let { data, children }: { data: { orgId: string }; children: Snippet } =
  $props();

const basePath = $derived(`/${data.orgId}/ai-center`);

const navItems = $derived([
  {
    path: "",
    label: m.ai_center_nav_overview ? m.ai_center_nav_overview() : "النظرة العامة",
    icon: Compass,
  },
  {
    path: "live",
    label: m.ai_center_nav_live ? m.ai_center_nav_live() : "البث المباشر",
    icon: Video,
  },
  {
    path: "integrations",
    label: m.ai_center_nav_integrations ? m.ai_center_nav_integrations() : "التكاملات",
    icon: Plug,
  },
  {
    path: "permissions",
    label: m.ai_center_nav_permissions ? m.ai_center_nav_permissions() : "الصلاحيات",
    icon: Shield,
  },
  {
    path: "settings",
    label: m.ai_center_nav_settings ? m.ai_center_nav_settings() : "الإعدادات",
    icon: Settings,
  },
  {
    path: "ai-logs",
    label: m.ai_center_nav_logs ? m.ai_center_nav_logs() : "سجل الأنشطة",
    icon: ScrollText,
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
