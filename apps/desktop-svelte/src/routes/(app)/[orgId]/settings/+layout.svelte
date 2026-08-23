<!--
@component SettingsLayout
Wraps all Settings routes inside a resizable double-sidebar layout
using <MainViewport>. Uses the shared SidebarNav component.
Now includes rooms and subjects (moved from academics).
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
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
  Scale,
  Calculator,
  Layers,
} from "@lucide/svelte";
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
        icon: Bell,
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
        path: "permissions",
        label: m.settings_nav_permissions(),
        icon: Shield,
      },
      {
        path: "units",
        label: m.settings_nav_units(),
        icon: Network,
      },
    ],
  },
  {
    title: getLocale() === "ar" ? "الشؤون الأكاديمية" : "Academic Rules",
    items: [
      {
        path: "periods",
        label: m.settings_nav_cycles ? m.settings_nav_cycles() : "Academic Periods",
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
        icon: MapPin,
      },
      {
        path: "academic-policy",
        label: m.common_nav_academic_policy(),
        icon: Sliders,
      },
      {
        path: "assessment-schemes",
        label: getLocale() === "ar" ? "هياكل التقييم" : "Assessment Schemes",
        icon: Layers,
      },
      {
        path: "grade-scales",
        label: getLocale() === "ar" ? "مقاييس الدرجات" : "Grade Scales",
        icon: Scale,
      },
      {
        path: "calc-policies",
        label: getLocale() === "ar" ? "سياسات الحساب" : "Calculation Policies",
        icon: Calculator,
      },
      {
        path: "ranking-policies",
        label: getLocale() === "ar" ? "سياسات الترتيب" : "Ranking Policies",
        icon: Sliders,
      },
      {
        path: "registration",
        label: m.settings_nav_registration(),
        icon: FileText,
      },
      {
        path: "ordering",
        label: m.common_nav_ordering(),
        icon: ListOrdered,
      },
      {
        path: "approvals",
        label: m.settings_nav_approvals(),
        icon: CheckSquare,
      },
    ],
  },
  {
    title: getLocale() === "ar" ? "المالية" : "Finance",
    items: [
      {
        path: "currencies",
        label: m.settings_nav_currencies(),
        icon: Coins,
      },
      {
        path: "fee-templates",
        label: m.common_breadcrumb_fees(),
        icon: Receipt,
      },
      {
        path: "discount-templates",
        label: m.common_dt_title(),
        icon: Percent,
      },
    ],
  },
  {
    title: getLocale() === "ar" ? "العمليات" : "Operations",
    items: [
      {
        path: "branches",
        label: getLocale() === "ar" ? "الفروع" : "Branches",
        icon: Network,
      },
      {
        path: "bulk-input",
        label: m.settings_nav_import_export(),
        icon: FileDown,
      },
      {
        path: "archive",
        label: m.settings_nav_archive(),
        icon: FileText,
      },
      {
        path: "audit",
        label: m.common_nav_audit(),
        icon: History,
      }
    ],
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
