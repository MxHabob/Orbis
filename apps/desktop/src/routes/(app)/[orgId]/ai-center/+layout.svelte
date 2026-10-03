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
    label: m.ai_center_nav_overview(),
    icon: GlobeAlt,
  },
  {
    path: "live",
    label: m.ai_center_nav_live(),
    icon: Camera,
  },
  {
    path: "integrations",
    label: m.ai_center_nav_integrations(),
    icon: Cube,
  },
  {
    path: "permissions",
    label: m.ai_center_nav_permissions(),
    icon: UserBadgeCheck,
  },
  {
    path: "settings",
    label: m.ai_center_nav_settings(),
    icon: Settings,
  },
  {
    path: "ai-logs",
    label: m.ai_center_nav_logs(),
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
