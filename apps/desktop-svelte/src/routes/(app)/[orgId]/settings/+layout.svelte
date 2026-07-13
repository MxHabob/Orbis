<!--
@component SettingsLayout
Wraps all Settings routes inside a resizable double-sidebar layout
using <MainViewport>. Uses the shared SidebarNav component.
Now includes rooms and subjects (moved from academics).
-->
<script lang="ts">
import {
  Settings,
  Bell,
  Lock,
  Users,
  Calendar,
  Network,
  History,
  FileDown,
  Shield,
  Coins,
  ListOrdered,
  FileText,
  MapPin,
  BookOpen,
  Receipt,
  Percent,
  CheckSquare,
  Sliders,
} from "lucide-svelte";
import MainViewport from "$lib/shell/main-viewport.svelte";
import SidebarNav from "$lib/shell/sidebar-nav.svelte";
import type { Snippet } from "svelte";
import * as settings from "$lib/paraglide/messages/settings";
import { getLocale } from "$lib/paraglide/runtime";

let { data, children }: { data: { orgId: string }; children: Snippet } =
  $props();

const basePath = $derived(`/${data.orgId}/settings`);

const navItems = $derived([
  {
    path: "",
    label: settings.nav_general(),
    icon: Settings,
  },

  {
    path: "notifications",
    label: settings.nav_notifications(),
    icon: Bell,
  },
  {
    path: "users",
    label: settings.nav_users(),
    icon: Users,
  },
  {
    path: "permissions",
    label: settings.nav_permissions(),
    icon: Shield,
  },
  {
    path: "periods",
    label: settings.nav_periods
      ? settings.nav_periods()
      : "Academic Periods",
    icon: Calendar,
  },
  {
    path: "units",
    label: settings.nav_units(),
    icon: Network,
  },
  {
    path: "rooms",
    label: settings.nav_rooms(),
    icon: MapPin,
  },
  {
    path: "subjects",
    label: settings.nav_subjects(),
    icon: BookOpen,
  },
  {
    path: "audit",
    label: settings.nav_audit(),
    icon: History,
  },
  {
    path: "bulk-input",
    label: settings.nav_import_export(),
    icon: FileDown,
  },
  {
    path: "currencies",
    label: settings.nav_currencies(),
    icon: Coins,
  },
  {
    path: "ordering",
    label: settings.nav_ordering(),
    icon: ListOrdered,
  },
  {
    path: "registration",
    label: settings.nav_registration(),
    icon: FileText,
  },

  {
    path: "fee-templates",
    label: settings.nav_fee_templates(),
    icon: Receipt,
  },
  {
    path: "discount-templates",
    label: settings.nav_discount_templates(),
    icon: Percent,
  },
  {
    path: "academic-policy",
    label: settings.nav_academic_policy(),
    icon: Sliders,
  },
  {
    path: "approvals",
    label: settings.nav_approvals(),
    icon: CheckSquare,
  },
]);
</script>

<div
  class="h-full w-full bg-background overflow-hidden"
  dir={getLocale() === "ar" ? "rtl" : "ltr"}
>
  <MainViewport name="settings-workspace" leftWidth={{ default: 240, min: 200 }}>
    {#snippet left()}
      <SidebarNav
        items={navItems}
        {basePath}
        title={settings.layout_title()}
      />
    {/snippet}

    {#snippet middle()}
      <div class="h-full w-full overflow-hidden">
        {@render children()}
      </div>
    {/snippet}
  </MainViewport>
</div>
