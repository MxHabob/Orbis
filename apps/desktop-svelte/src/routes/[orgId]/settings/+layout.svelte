<!--
@component SettingsLayout
Wraps all Settings routes inside a GitButler-style resizable double-sidebar layout
using `<MainViewport>`. Renders vertical sidebar links.
-->
<script lang="ts">
import { page } from "$app/state";
import { onMount } from "svelte";
import { commands } from "$lib/services/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { deLocalizeUrl, localizeHref, getLocale } from "$lib/paraglide/runtime";
import {
  Settings,
  Palette,
  Bell,
  Lock,
  Users,
  Calendar,
  Network,
  History,
  FileDown,
  ChevronRight,
  Shield,
  Coins,
  ListOrdered,
} from "lucide-svelte";
import MainViewport from "$lib/components/layout/MainViewport.svelte";
import type { Snippet } from "svelte";
import * as m from "$lib/paraglide/messages";

let { data, children }: { data: { orgId: string }; children: Snippet } =
  $props();

// Active path helper
function isActive(pathSegment: string): boolean {
  const unlocalizedPathname = deLocalizeUrl(page.url).pathname;
  const baseUrl = `/${data.orgId}/settings`;
  if (pathSegment === "") {
    return unlocalizedPathname === baseUrl;
  }
  return (
    unlocalizedPathname === `${baseUrl}/${pathSegment}` ||
    unlocalizedPathname.startsWith(`${baseUrl}/${pathSegment}/`)
  );
}

const navItems = $derived([
  {
    path: "",
    label: m.settings_nav_general(),
    icon: Settings,
  },
  {
    path: "appearance",
    label: m.settings_nav_appearance(),
    icon: Palette,
  },
  {
    path: "notifications",
    label: m.settings_nav_notifications(),
    icon: Bell,
  },
  {
    path: "security",
    label: m.settings_nav_security(),
    icon: Lock,
  },
  {
    path: "users",
    label: m.settings_nav_users(),
    icon: Users,
  },
  {
    path: "permissions",
    label: m.settings_nav_permissions(),
    icon: Shield,
  },
  {
    path: "cycles",
    label: m.settings_nav_cycles(),
    icon: Calendar,
  },
  {
    path: "units",
    label: m.settings_nav_units(),
    icon: Network,
  },
  {
    path: "audit",
    label: m.settings_nav_audit(),
    icon: History,
  },
  {
    path: "import-export",
    label: m.settings_nav_import_export(),
    icon: FileDown,
  },
  {
    path: "currencies",
    label: m.settings_nav_currencies ? m.settings_nav_currencies() : "العملات وأسعار الصرف",
    icon: Coins,
  },
  {
    path: "ordering",
    label: m.settings_nav_ordering ? m.settings_nav_ordering() : "ترتيب الطلاب",
    icon: ListOrdered,
  },
]);
</script>

<div class="h-full w-full bg-background overflow-hidden" dir={getLocale() === "ar" ? "rtl" : "ltr"}>
  <MainViewport
    name="settings-workspace"
    leftWidth={{ default: 240, min: 200 }}
  >
    <!-- Left Resizable Panel: Vertical Secondary Sidebar -->
    {#snippet left()}
      <div class="h-full border-r border-border bg-card/20 p-4 flex flex-col gap-4 overflow-y-auto scrollbar-hide select-none">
        <!-- Settings Header Title -->
        <div class="flex flex-col gap-1 px-2">
          <h2 class="text-sm font-semibold tracking-tight">{m.settings_layout_title()}</h2>
          <p class="text-[10px] text-muted-foreground">{m.settings_layout_desc()}</p>
        </div>

        <!-- Vertical Links List -->
        <div class="flex flex-col gap-1 w-full text-xs">
          {#each navItems as item (item.path)}
            <Button
              href={localizeHref(`/${data.orgId}/settings/${item.path}`)}
              variant="ghost"
              class="w-full flex items-center justify-between px-3 py-2 rounded-lg text-start transition-all duration-150 cursor-pointer h-9
                {isActive(item.path) ? 'bg-primary/10 text-primary font-semibold hover:bg-primary/15' : 'text-muted-foreground hover:bg-accent hover:text-foreground'}"
            >
              <div class="flex items-center gap-2">
                <item.icon size={14} class={isActive(item.path) ? 'text-primary' : 'text-muted-foreground'} />
                <span>{item.label}</span>
              </div>

              {#if isActive(item.path)}
                <ChevronRight size={10} class="text-primary transition-transform duration-200 {getLocale() === 'ar' ? 'rotate-180' : ''}" />
              {/if}
            </Button>
          {/each}
        </div>
      </div>
    {/snippet}

    <!-- Middle Panel: Display Child Sub-routes -->
    {#snippet middle()}
      <div class="h-full w-full overflow-hidden">
        {@render children()}
      </div>
    {/snippet}
  </MainViewport>
</div>
