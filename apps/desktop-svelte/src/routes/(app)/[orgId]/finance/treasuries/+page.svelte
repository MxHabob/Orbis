<!--
@component TreasuriesManagementPage
Split administration page: Treasuries (left) + Ledger Accounts (right).
Both use OrbisGrid via DataTable.
-->
<script lang="ts">
import { onMount } from "svelte";
import { commands } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { Plus, Landmark, Layers } from "lucide-svelte";
import { getLocale } from "$lib/paraglide/runtime";
import { formatCurrency } from "$lib/core/i18n";
import { fade } from "svelte/transition";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
} from "@orbis/ui-svelte/dialog";
import * as finance from "$lib/paraglide/messages/finance";
import * as common from "$lib/paraglide/messages/common";


import { DataTable } from "$lib/tables";
import {
  treasuries as tDef,
  ledger as lDef,
  withIndex,
} from "$lib/tables/definitions";

let { data }: { data: { orgId: string } } = $props();

// ── State ──────────────────────────────────────────────────────────────
let treasuries = $state<any[]>([]);
let ledgers = $state<any[]>([]);
let isLoading = $state(true);
let activeTabType = $state<"gl" | "treasury" | "entity">("gl");
let isAddDialogOpen = $state(false);

function handleSuccess() {
  isAddDialogOpen = false;
  loadData();
}

// ── Data Loading ────────────────────────────────────────────────────────

async function loadData() {
  isLoading = true;
  try {
    const [tList, lList] = await Promise.all([
      commands.finance.listTreasuries(data.orgId),
      commands.finance.listTransactionAccounts(data.orgId),
    ]);
    treasuries = tList;
    ledgers = lList;
  } catch (e) {
    console.error("Error:", e);
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (data.orgId) loadData();
});

// ── Helpers ─────────────────────────────────────────────────────────────

const totalBalance = $derived(
  treasuries.reduce((sum, t) => sum + Number(t.balance || 0), 0),
);

function getTreasuryTypeLabel(t: string) {
  const low = String(t).toLowerCase();
  if (low === "bank") return common.treasury_type_bank();
  return common.treasury_type_cash();
}

function getLedgerTypeLabel(t: string) {
  const low = String(t).toLowerCase();
  if (low === "revenue") return finance.type_revenue();
  if (low === "expense") return finance.type_expense();
  if (low === "asset") return finance.type_asset();
  if (low === "liability") return finance.type_liability();
  return t;
}

// ── Grid Sources ────────────────────────────────────────────────────────

const treasurySource = $derived(
  withIndex(
    treasuries.map((t) => ({
      ...t,
      name: t.name_ar || t.name || "—",
      treasury_type: getTreasuryTypeLabel(t.treasury_type),
      balance: formatCurrency(Number(t.balance || 0), t.currency),
      currency_code: t.currency || "YER",
      status: "active",
    })),
  ),
);

const ledgerSource = $derived(
  withIndex(
    ledgers.map((l) => ({
      ...l,
      name: l.name_ar || l.name || "—",
      account_type: getLedgerTypeLabel(l.account_type),
    })),
  ),
);

const treasuryCols = $derived(
  tDef.columns({
    idx: "#",
    name: finance.col_name(),
    type: finance.col_category(),
    balance: finance.col_balance(),
    currency: "Currency",
  }),
);

const ledgerCols = $derived(
  lDef.columns({
    idx: "#",
    code: finance.col_code(),
    name: finance.col_name(),
    type: finance.col_type(),
  }),
);

const rtl = $derived(getLocale() === "ar");
</script>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 font-sans">
  <!-- Header -->
  <div class="text-start select-none">
    <h2 class="text-sm font-bold text-foreground uppercase tracking-wider">
      {finance.page_title()}
    </h2>
    <p class="text-[10px] text-muted-foreground mt-0.5">
      {finance.page_subtitle()}
    </p>
  </div>

  {#if isLoading}
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
      <Skeleton class="h-80 w-full rounded-xl" />
      <Skeleton class="h-80 w-full rounded-xl" />
    </div>
  {:else}
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <!-- Treasuries Section -->
      <div class="bg-card/45 border border-border rounded-xl p-5 flex flex-col gap-4 text-start">
        <div class="flex items-center justify-between border-b border-border/60 pb-3 select-none">
          <div class="flex flex-col">
            <h3 class="text-xs font-bold text-foreground uppercase tracking-wider flex items-center gap-1.5">
              <Landmark size={13} class="text-primary" />
              <span>{finance.vaults_title()}</span>
            </h3>
            <span class="text-[8px] text-muted-foreground mt-0.5">
              {finance.total_cash_assets({ balance: formatCurrency(totalBalance, "YER") })}
            </span>
          </div>
          <Button variant="outline" class="text-[10px] h-7 px-2" onclick={() => { activeTabType = "treasury"; isAddDialogOpen = true; }}>
            <Plus size={12} class="me-1" />
            {finance.new_treasury_button()}
          </Button>
        </div>
        <div class="flex-1 min-h-0" style="height: 300px">
          <DataTable
            columns={treasuryCols}
            source={treasurySource}
            emptyIcon={Landmark}
            emptyTitle={finance.no_treasuries()}
            {rtl}
          />
        </div>
      </div>

      <!-- Ledger Accounts Section -->
      <div class="bg-card/45 border border-border rounded-xl p-5 flex flex-col gap-4 text-start">
        <div class="flex items-center justify-between border-b border-border/60 pb-3 select-none">
          <div class="flex flex-col">
            <h3 class="text-xs font-bold text-foreground uppercase tracking-wider flex items-center gap-1.5">
              <Layers size={13} class="text-primary" />
              <span>{finance.ledger_chart_title()}</span>
            </h3>
            <span class="text-[8px] text-muted-foreground mt-0.5">
              {finance.ledger_accounts_count({ count: ledgers.length })}
            </span>
          </div>
          <Button variant="outline" class="text-[10px] h-7 px-2" onclick={() => { activeTabType = "gl"; isAddDialogOpen = true; }}>
            <Plus size={12} class="me-1" />
            {finance.new_ledger_button()}
          </Button>
        </div>
        <div class="flex-1 min-h-0" style="height: 300px">
          <DataTable
            columns={ledgerCols}
            source={ledgerSource}
            emptyIcon={Layers}
            emptyTitle={finance.no_ledgers()}
            {rtl}
          />
        </div>
      </div>
    </div>
  {/if}
</div>

<Dialog bind:open={isAddDialogOpen}>
  <DialogContent class="max-w-xl text-start select-none h-[80vh] flex flex-col p-6 bg-card border-border/80">
    <DialogHeader class="shrink-0 mb-4 text-start">
      <DialogTitle class="text-base font-extrabold text-foreground">
        {common.ledger_dialog_add_title()}
      </DialogTitle>
      <DialogDescription class="text-[11px] text-muted-foreground leading-relaxed">
        {common.ledger_dialog_add_desc()}
      </DialogDescription>
    </DialogHeader>
    {#if isAddDialogOpen}
      {#await import("$lib/features/finance/components/ledger-form.svelte") then { default: LedgerForm }}
        <LedgerForm
          orgId={data.orgId}
          onSuccess={handleSuccess}
          onCancel={() => isAddDialogOpen = false}
          defaultTab={activeTabType}
        />
      {/await}
    {/if}
  </DialogContent>
</Dialog>
