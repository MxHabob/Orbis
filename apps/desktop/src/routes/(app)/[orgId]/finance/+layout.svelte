<!--
@component FinanceLayout
Wraps all Finance routes inside a resizable double-sidebar layout
using <MainViewport>. Uses the shared SidebarNav component.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import {
  createCashAccountsQuery,
  createStudentFeesQuery,
} from "$lib/features/finance";
import { Banknotes, BookOpen, BuildingLibrary, ChartBar, CreditCard, DocumentChartBar } from "@orbis/icons";
import MainViewport from "$lib/shell/main-viewport.svelte";
import SidebarNav from "$lib/shell/sidebar-nav.svelte";
import ArchivedBanner from "$lib/features/academics/archived-banner.svelte";
import type { Snippet } from "svelte";

let { data, children }: { data: { orgId: string }; children: Snippet } =
  $props();

let sidebarStatsReady = $state(false);
const feesQuery = createStudentFeesQuery(
  () => data.orgId,
  undefined,
  () => sidebarStatsReady,
);
const treasuriesQuery = createCashAccountsQuery(
  () => data.orgId,
  () => sidebarStatsReady,
);
const unpaidFeesCount = $derived(
  (feesQuery.data ?? []).filter(
    (fee: any) => fee.status === "unpaid" || fee.status === "partially_paid",
  ).length,
);
const treasuriesCount = $derived(treasuriesQuery.data?.length ?? 0);

onMount(() => {
  const timer = setTimeout(() => {
    sidebarStatsReady = true;
  }, 150);

  return () => clearTimeout(timer);
});

const basePath = $derived(`/${data.orgId}/finance`);

const navItems = $derived([
  {
    path: "",
    label: m.common_nav_overview(),
    icon: Banknotes,
  },
  {
    path: "fees",
    label: m.finance_layout_nav_fees(),
    icon: DocumentChartBar,
    badge: unpaidFeesCount > 0 ? unpaidFeesCount : null,
  },
  {
    path: "payments",
    label: "سندات القبض",
    icon: CreditCard,
  },
  {
    path: "disbursements",
    label: "سندات الصرف",
    icon: Banknotes,
  },

  {
    path: "accounts",
    label: m.finance_layout_nav_cash_accounts(),
    icon: BuildingLibrary,
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
    icon: DocumentChartBar,
  },
  {
    path: "reports",
    label: m.finance_layout_nav_reports(),
    icon: ChartBar,
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

