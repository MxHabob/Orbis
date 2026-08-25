<script lang="ts">
import * as m from "$lib/paraglide/messages";
import SecondaryMenu from "$lib/shell/secondary-menu.svelte";
import { BarChart3, Scale, FileSpreadsheet, Landmark } from "@lucide/svelte";
import type { Snippet } from "svelte";

let { data, children }: { data: { orgId: string }; children: Snippet } = $props();
const basePath = $derived(`/${data.orgId}/finance/reports`);
const items = $derived([
  { path: "", label: "النظرة العامة", icon: BarChart3 },
  { path: "trial-balance", label: m.finance_reports_trial_balance(), icon: Scale },
  { path: "income-statement", label: m.finance_reports_income_statement_tab(), icon: FileSpreadsheet },
  { path: "balance-sheet", label: m.finance_reports_balance_sheet_tab(), icon: Landmark },
]);
</script>

<div class="flex h-full w-full flex-col overflow-hidden bg-background" dir="rtl">
  <div class="shrink-0 border-b border-border/40 bg-background px-2 pt-1">
    <SecondaryMenu items={items} {basePath} />
  </div>
  <main class="min-h-0 flex-1 overflow-hidden">
    {@render children()}
  </main>
</div>
