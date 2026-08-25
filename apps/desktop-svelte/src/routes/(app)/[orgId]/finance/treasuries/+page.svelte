<!-- Cash and bank accounts management page. -->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { commands } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { Plus, Landmark, RefreshCw, WalletCards } from "@lucide/svelte";
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

import { DataTable } from "$lib/tables";
import {
  cashAccounts as tDef,
  withIndex,
} from "$lib/tables/definitions";

let { data }: { data: { orgId: string } } = $props();

// ── State ──────────────────────────────────────────────────────────────
let cashAccounts = $state<any[]>([]);
let baseCurrency = $state("YER");
let isLoading = $state(true);
let isAddDialogOpen = $state(false);

function handleSuccess() {
  isAddDialogOpen = false;
  loadData();
}

// ── Data Loading ────────────────────────────────────────────────────────

async function loadData() {
  isLoading = true;
  try {
    const [accounts, currencies] = await Promise.all([
      commands.finance.listAccounts().then((accounts) => accounts.filter((account: any) => ["cash", "bank"].includes(account.sub_type))),
      commands.finance.listCurrencies(),
    ]);
    cashAccounts = accounts;
    const base = currencies.find((c: any) => c.is_base === 1);
    if (base) baseCurrency = base.code;
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
  cashAccounts.reduce((sum, account) => sum + Number(account.balances?.[baseCurrency] || 0), 0),
);

const activeAccountCount = $derived(cashAccounts.length);

function getTreasuryTypeLabel(t: string) {
  const low = String(t).toLowerCase();
  if (low === "bank") return m.common_cash_account_type_bank();
  return m.common_cash_account_type_cash();
}

// ── Grid Sources ────────────────────────────────────────────────────────

const treasurySource = $derived(
  withIndex(
    cashAccounts.map((account) => ({
      ...account,
      name: account.name_ar || account.name || "—",
      treasury_type: getTreasuryTypeLabel(account.sub_type),
      balance: formatCurrency(
        Number(account.balances?.[account.currency] || 0),
        account.currency || baseCurrency,
      ),
      currency_code: account.currency || baseCurrency,
      status: "active",
    })),
  ),
);

const treasuryCols = $derived(
  tDef.columns({
    idx: "#",
    name: m.common_col_name(),
    type: m.common_word_type(),
    balance: m.finance_col_balance(),
    currency: "Currency",
  }),
);

const rtl = $derived(getLocale() === "ar");
</script>

<div class="h-full w-full overflow-y-auto bg-background p-5 md:p-8 font-sans" dir={rtl ? "rtl" : "ltr"}>
  <div class="max-w-6xl mx-auto flex flex-col gap-7" in:fade={{ duration: 180 }}>
    <header class="flex justify-end">
      <Button variant="outline" class="h-9 px-3 text-xs font-bold shrink-0" onclick={() => { isAddDialogOpen = true; }}>
        <Plus size={14} class="me-1.5" />
        {m.finance_new_cash_account_button()}
      </Button>
    </header>

    <section class="grid grid-cols-1 sm:grid-cols-3 gap-3" aria-label={m.finance_cash_accounts_title()}>
      <div class="border border-border bg-card p-4 rounded-lg flex items-center gap-3">
        <div class="h-9 w-9 rounded-md bg-primary/10 text-primary flex items-center justify-center"><Landmark size={17} /></div>
        <div><p class="text-[10px] text-muted-foreground">{m.finance_cash_accounts_title()}</p><p class="text-xl font-black text-foreground">{activeAccountCount}</p></div>
      </div>
      <div class="border border-border bg-card p-4 rounded-lg flex items-center gap-3">
        <div class="h-9 w-9 rounded-md bg-emerald-500/10 text-emerald-500 flex items-center justify-center"><WalletCards size={17} /></div>
        <div><p class="text-[10px] text-muted-foreground">{m.finance_total_cash_assets({ balance: "" }).replace(/: $/, "")}</p><p class="text-base font-black text-foreground font-mono">{formatCurrency(totalBalance, baseCurrency)}</p></div>
      </div>
      <div class="border border-border bg-card p-4 rounded-lg flex items-center gap-3">
        <div class="h-9 w-9 rounded-md bg-sky-500/10 text-sky-500 flex items-center justify-center"><RefreshCw size={17} /></div>
        <div><p class="text-[10px] text-muted-foreground">{m.finance_col_balance()}</p><p class="text-xs font-bold text-foreground">{baseCurrency}</p></div>
      </div>
    </section>

    {#if isLoading}
      <div class="border border-border bg-card rounded-lg p-5"><Skeleton class="h-72 w-full rounded-md" /></div>
    {:else}
      <section class="border border-border bg-card rounded-lg overflow-hidden">
        <div class="px-5 py-4 border-b border-border flex items-center justify-between gap-4">
          <div class="text-start">
            <h2 class="text-sm font-bold text-foreground">{m.finance_cash_accounts_title()}</h2>
            <p class="text-[10px] text-muted-foreground mt-1">{m.finance_page_subtitle()}</p>
          </div>
          <Button variant="ghost" size="icon" class="h-8 w-8" title={m.common_control_refresh()} onclick={loadData}>
            <RefreshCw size={14} />
          </Button>
        </div>
        <div class="min-h-82.5">
          <DataTable columns={treasuryCols} source={treasurySource} emptyIcon={Landmark} emptyTitle={m.finance_no_cash_accounts()} {rtl} refreshable onRefresh={loadData} />
        </div>
      </section>
    {/if}
  </div>
</div>

<Dialog bind:open={isAddDialogOpen}>
  <DialogContent class="max-w-xl text-start select-none h-[80vh] flex flex-col p-6 bg-card border-border/80">
    <DialogHeader class="shrink-0 mb-4 text-start">
      <DialogTitle class="text-base font-extrabold text-foreground">{m.common_ledger_dialog_add_title()}</DialogTitle>
      <DialogDescription class="text-[11px] text-muted-foreground leading-relaxed">{m.common_ledger_dialog_add_desc()}</DialogDescription>
    </DialogHeader>
    {#if isAddDialogOpen}
      {#await import("$lib/features/finance/components/ledger-form.svelte") then { default: LedgerForm }}
        <LedgerForm orgId={data.orgId} onSuccess={handleSuccess} onCancel={() => isAddDialogOpen = false} defaultTab="cash_account" />
      {/await}
    {/if}
  </DialogContent>
</Dialog>
