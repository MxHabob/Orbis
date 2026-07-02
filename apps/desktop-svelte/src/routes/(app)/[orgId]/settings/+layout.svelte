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
} from "lucide-svelte";
import MainViewport from "$lib/components/layout/MainViewport.svelte";
import SidebarNav from "$lib/components/layout/SidebarNav.svelte";
import type { Snippet } from "svelte";
import * as m from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";

let { data, children }: { data: { orgId: string }; children: Snippet } =
  $props();

const basePath = $derived(`/${data.orgId}/settings`);

const navItems = $derived([
  {
    path: "",
    label: m.settings_nav_general(),
    icon: Settings,
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
    path: "rooms",
    label: m.settings_nav_rooms ? m.settings_nav_rooms() : "الغرف والقاعات",
    icon: MapPin,
  },
  {
    path: "subjects",
    label: m.settings_nav_subjects
      ? m.settings_nav_subjects()
      : "المواد الدراسية",
    icon: BookOpen,
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
    label: m.settings_nav_currencies
      ? m.settings_nav_currencies()
      : "العملات وأسعار الصرف",
    icon: Coins,
  },
  {
    path: "ordering",
    label: m.settings_nav_ordering ? m.settings_nav_ordering() : "ترتيب الطلاب",
    icon: ListOrdered,
  },

  {
    path: "fee-templates",
    label: m.settings_nav_fee_templates
      ? m.settings_nav_fee_templates()
      : "الرسوم",
    icon: Receipt,
  },
  {
    path: "discount-templates",
    label: m.settings_nav_discount_templates
      ? m.settings_nav_discount_templates()
      : "الخصومات",
    icon: Percent,
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
        title={m.settings_layout_title()}
      />
    {/snippet}

    {#snippet middle()}
      <div class="h-full w-full overflow-hidden">
        {@render children()}
      </div>
    {/snippet}
  </MainViewport>
</div>
