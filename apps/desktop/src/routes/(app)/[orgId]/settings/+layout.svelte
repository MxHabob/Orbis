<!--
@component SettingsLayout
Wraps all Settings routes inside a resizable double-sidebar layout
using <MainViewport>. Uses the shared SidebarNav component.
Now includes rooms and subjects (moved from academics).
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import {
  AppNotification,
  ArchiveBox,
  ArrowUpTray,
  BookOpen,
  BuildingOffice,
  Calendar,
  CheckCircle,
  ClockRotateRight,
  Database,
  DocumentPage,
  GlobeAlt,
  Keyframes,
  Pin,
  ReceiptPercent,
  Scale,
  Settings,
  ShieldCheck,
  Trash,
  UserGroup,
  Users,
} from "@orbis/icons";
import MainViewport from "$lib/shell/main-viewport.svelte";
import SidebarNav from "$lib/shell/sidebar-nav.svelte";
import type { Snippet } from "svelte";

import { getLocale } from "$lib/paraglide/runtime";

let { data, children }: { data: { orgId: string }; children: Snippet } =
  $props();

const basePath = $derived(`/${data.orgId}/settings`);

const navGroups = $derived([
  {
    title: getLocale() === "ar" ? "عام" : "General",
    items: [
      {
        path: "",
        label: m.settings_nav_general(),
        icon: Settings,
      },
      {
        path: "notifications",
        label: m.common_nav_notifications(),
        icon: AppNotification,
      },
    ],
  },
  {
    title: getLocale() === "ar" ? "المؤسسة" : "Organization",
    items: [
      {
        path: "users",
        label: m.settings_nav_users(),
        icon: Users,
      },
      {
        path: "groups",
        label: m.groups_nav_title ? m.groups_nav_title() : "User Groups",
        icon: UserGroup,
      },
      {
        path: "permissions",
        label: m.settings_nav_permissions(),
        icon: ShieldCheck,
      },
      {
        path: "units",
        label: m.settings_nav_units(),
        icon: BuildingOffice,
      },
    ],
  },
  {
    title: getLocale() === "ar" ? "الشؤون الأكاديمية" : "Academic Rules",
    items: [
      {
        path: "periods",
        label: m.settings_nav_cycles
          ? m.settings_nav_cycles()
          : "Academic Periods",
        icon: Calendar,
      },
      {
        path: "subjects",
        label: m.common_exams_detail_subjects(),
        icon: BookOpen,
      },
      {
        path: "rooms",
        label: m.common_nav_rooms(),
        icon: Pin,
      },
      {
        path: "academic-policy",
        label: m.common_nav_academic_policy(),
        icon: Scale,
      },
      {
        path: "assessment-schemes",
        label: getLocale() === "ar" ? "هياكل التقييم" : "Assessment Schemes",
        icon: Keyframes,
      },
      {
        path: "registration",
        label: m.settings_nav_registration(),
        icon: DocumentPage,
      },
      {
        path: "approvals",
        label: m.settings_nav_approvals(),
        icon: CheckCircle,
      },
    ],
  },
  {
    title: getLocale() === "ar" ? "المالية" : "Finance",
    items: [
      {
        path: "currencies",
        label: m.settings_nav_currencies(),
        icon: GlobeAlt,
      },
      {
        path: "fee-templates",
        label: m.common_breadcrumb_fees(),
        icon: DocumentPage,
      },
      {
        path: "discount-templates",
        label: m.common_dt_title(),
        icon: ReceiptPercent,
      },
    ],
  },
  {
    title: getLocale() === "ar" ? "العمليات" : "Operations",
    items: [
      {
        path: "branches",
        label: getLocale() === "ar" ? "الفروع" : "Branches",
        icon: BuildingOffice,
      },
      {
        path: "bulk-input",
        label: m.settings_nav_import_export(),
        icon: ArrowUpTray,
      },
      {
        path: "archive",
        label: m.settings_nav_archive(),
        icon: ArchiveBox,
      },
      {
        path: "archive/trash",
        label: getLocale() === "ar" ? "سلة المحذوفات" : "Trash",
        icon: Trash,
      },
      {
        path: "database",
        label: m.settings_nav_database(),
        icon: Database,
      },
      {
        path: "audit",
        label: m.common_nav_audit(),
        icon: ClockRotateRight,
      },
    ],
  },
]);
</script>

<div
  class="h-full w-full bg-background overflow-hidden"
>
  <MainViewport name="settings-workspace" leftWidth={{ default: 240, min: 200 }}>
    {#snippet left()}
      <SidebarNav
        groups={navGroups}
        {basePath}
        title={m.common_word_settings()}
      />
    {/snippet}

    {#snippet middle()}
      <div class="h-full w-full overflow-hidden">
        {@render children()}
      </div>
    {/snippet}
  </MainViewport>
</div>
