<!--
@component FinanceLayout
Wraps all Finance routes inside a resizable double-sidebar layout
using <MainViewport>. Uses the shared SidebarNav component.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { commands } from "$lib/core/ipc";
import { onMount } from "svelte";
import {
  Wallet,
  CreditCard,
  Landmark,
  BarChart3,
  Compass,
  BookOpen,
  ClipboardList,
} from "lucide-svelte";
import MainViewport from "$lib/shell/main-viewport.svelte";
import SidebarNav from "$lib/shell/sidebar-nav.svelte";
import ArchivedBanner from "$lib/features/academics/archived-banner.svelte";
import type { Snippet } from "svelte";

let { data, children }: { data: { orgId: string }; children: Snippet } =
  $props();

// Sidebar live statistics
let unpaidFeesCount = $state(0);
let treasuriesCount = $state(0);

async function loadSidebarStats() {
  try {
    const [feeList, treasuryList] = await Promise.all([
      commands.finance.listStudentFeesByEnrollment("").catch(() => []),
      commands.finance.listTreasuries().catch(() => []),
    ]);
    unpaidFeesCount = feeList.filter(
      (f) => f.status === "unpaid" || f.status === "partially_paid",
    ).length;
    treasuriesCount = treasuryList.length;
  } catch (e) {
    console.error("Error loading sidebar stats:", e);
  }
}

// Load once on mount — not on every sub-route change
onMount(() => {
  if (data.orgId) loadSidebarStats();
});

const basePath = $derived(`/${data.orgId}/finance`);

const navItems = $derived([
  {
    path: "",
    label: m.common_nav_overview(),
    icon: Compass,
  },
  {
    path: "fees",
    label: m.finance_layout_nav_fees(),
    icon: Wallet,
    badge: unpaidFeesCount > 0 ? unpaidFeesCount : null,
  },
  {
    path: "payments",
    label: m.finance_layout_nav_payments(),
    icon: CreditCard,
  },

  {
    path: "treasuries",
    label: m.finance_layout_nav_treasuries(),
    icon: Landmark,
    badge: treasuriesCount > 0 ? treasuriesCount : null,
  },
  {
    path: "journal-entries",
    label: m.common_je_title ? m.common_je_title() : "القيود اليومية",
    icon: BookOpen,
  },
  {
    path: "statement",
    label: m.finance_layout_nav_ledger
      ? m.finance_layout_nav_ledger()
      : "كشف الحساب",
    icon: ClipboardList,
  },
  {
    path: "reports",
    label: m.finance_layout_nav_reports(),
    icon: BarChart3,
  },
]);
</script>

<div class="h-full w-full bg-background overflow-hidden">
  <MainViewport name="finance-workspace" leftWidth={{ default: 240, min: 200 }}>
    {#snippet left()}
      <SidebarNav
        items={navItems}
        {basePath}
        title={m.finance_layout_panel_title()}
      />
    {/snippet}

    {#snippet middle()}
      <div class="h-full w-full flex flex-col overflow-hidden">
        <ArchivedBanner />
        <div class="flex-1 overflow-hidden">
          {@render children()}
        </div>
      </div>
    {/snippet}
  </MainViewport>
</div>

