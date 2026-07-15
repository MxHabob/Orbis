<!--
@component AiCenterLayout
Wraps all AI Center routes inside a resizable sidebar layout using <MainViewport>.
Uses the shared SidebarNav component — same pattern as SettingsLayout.
-->
<script lang="ts">
import {
  LayoutDashboard,
  ScrollText,
  Settings,
  Shield,
  Video,
  Plug,
} from "lucide-svelte";
import MainViewport from "$lib/shell/main-viewport.svelte";
import SidebarNav from "$lib/shell/sidebar-nav.svelte";
import type { Snippet } from "svelte";
import { getLocale } from "$lib/paraglide/runtime";

let { data, children }: { data: { orgId: string }; children: Snippet } =
  $props();

const basePath = $derived(`/${data.orgId}/ai-center`);

const navItems = $derived([
  {
    path: "",
    label: "Overview",
    icon: LayoutDashboard,
  },
  {
    path: "logs",
    label: "Logs",
    icon: ScrollText,
  },
  {
    path: "settings",
    label: "Settings",
    icon: Settings,
  },
  {
    path: "permissions",
    label: "Permissions",
    icon: Shield,
  },
  {
    path: "live",
    label: "Live",
    icon: Video,
  },
  {
    path: "integrations",
    label: "Integrations",
    icon: Plug,
  },
]);
</script>

<div
  class="h-full w-full bg-background overflow-hidden"
  dir={getLocale() === "ar" ? "rtl" : "ltr"}
>
  <MainViewport name="ai-center-workspace" leftWidth={{ default: 220, min: 180 }}>
    {#snippet left()}
      <SidebarNav
        items={navItems}
        {basePath}
        title="AI Center"
      />
    {/snippet}

    {#snippet middle()}
      <div class="h-full w-full overflow-hidden">
        {@render children()}
      </div>
    {/snippet}
  </MainViewport>
</div>
