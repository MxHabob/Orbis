<!--
  @component DisbursementsManager
  Minimal, elegant, and high-performance Disbursement Vouchers (سندات الصرف) workspace.
  Adheres strictly to Orbis Clean Architecture and the FinanceOverview design system:
  - Clean card-based disbursement voucher entry
  - Direct selection of treasury (credit) and expense/beneficiary (debit) accounts
  - Sleek top toolbar with search, method filter, and export
  - High-performance OrbisGrid table
  - Real backend double-entry recording and printable vouchers
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api } from "$lib/api";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Card, CardHeader, CardTitle, CardContent } from "@orbis/ui/card";
import { toast } from "@orbis/ui";
import {
  Plus,
  Refresh,
  Search,
  Download,
  XMark,
  Printer,
} from "@orbis/icons";
import { OrbisGrid, type OrbisColumnDef } from "@orbis/svelte-grid";
import AccountSearchInput from "$lib/features/finance/components/account-search-input.svelte";
import SearchableSelect from "$lib/shared/components/searchable-select.svelte";
import PdfReportViewer from "$lib/shared/components/pdf-report-viewer.svelte";
import { formatCurrency, formatDate, isAr } from "$lib/core/i18n";
import { fade, slide } from "svelte/transition";

let { orgId }: { orgId: string } = $props();

// ── State ──────────────────────────────────────────────────────────────
let disbursements = $state<any[]>([]);
let accounts = $state<any[]>([]);
let treasuries = $state<any[]>([]);
let currencies = $state<{ code: string; name: string }[]>([
  { code: "YER", name: m.finance_ui_currency_yer() },
  { code: "USD", name: m.finance_ui_currency_usd() },
  { code: "SAR", name: m.finance_ui_currency_sar() },
]);
let isLoading = $state(true);
let isSubmitting = $state(false);
let showAddForm = $state(false);

// Form Fields
let referenceNumber = $state("");
let voucherDate = $state(new Date().toISOString().split("T")[0]);
let amount = $state<number | null>(null);
let selectedCurrency = $state("YER");
let treasuryAccountId = $state(""); // Credit: Cash/Bank
let beneficiaryAccountId = $state(""); // Debit: Expense/Supplier
let paymentMethod = $state<"Cash" | "Bank" | "Check">("Cash");
let checkOrTransferRef = $state("");
let notes = $state("");

// Filters
let searchQuery = $state("");
let methodFilter = $state("all");
let filterDate = $state("");

// Print
let printDialogOpen = $state(false);
let printParameters = $state<Record<string, any>>({});
let printTitle = $state(m.finance_ui_print_disbursement());

onMount(async () => {
  await loadData();
});

async function loadData() {
  isLoading = true;
  try {
    const [entriesList, accountsList] = await Promise.all([
      api.listJournalEntries().catch(() => []),
      api.listAccounts().catch(() => []),
    ]);

    accounts = accountsList || [];
    treasuries = (accountsList || []).filter(
      (a: any) =>
        a.type === "asset" ||
        a.account_type === "asset" ||
        a.sub_type === "cash" ||
        a.sub_type === "bank"
    );

    if (treasuries.length > 0 && !treasuryAccountId) {
      treasuryAccountId = treasuries[0].id;
    }

    // Identify disbursement vouchers (entries where credit is a treasury/cash account)
    const treasuryIds = new Set(treasuries.map((t) => t.id));
    disbursements = (entriesList || []).filter((e: any) => {
      const creditId = e.credit_account_id || e.lines?.[1]?.account_id;
      return creditId && treasuryIds.has(creditId);
    });
  } catch (e) {
    console.error("Failed to load disbursements data:", e);
    toast.error(m.finance_ui_disbursements_load_failed());
  } finally {
    isLoading = false;
  }
}

function clearForm() {
  referenceNumber = "";
  voucherDate = new Date().toISOString().split("T")[0];
  amount = null;
  selectedCurrency = "YER";
  beneficiaryAccountId = "";
  paymentMethod = "Cash";
  checkOrTransferRef = "";
  notes = "";
}

function getAccountName(id?: string | null): string {
  if (!id) return "—";
  const acc = accounts.find((a) => a.id === id);
  if (!acc) return id.slice(0, 8);
  return acc.name_ar || acc.name || acc.code || "—";
}

async function handleSaveDisbursement() {
  if (!amount || amount <= 0) {
    toast.error(m.finance_ui_disbursement_amount_invalid());
    return;
  }
  if (!treasuryAccountId) {
    toast.error(m.finance_ui_disbursement_treasury_required());
    return;
  }
  if (!beneficiaryAccountId) {
    toast.error(m.finance_ui_disbursement_beneficiary_required());
    return;
  }
  if (treasuryAccountId === beneficiaryAccountId) {
    toast.error(m.finance_ui_disbursement_accounts_must_differ());
    return;
  }

  isSubmitting = true;
  try {
    await api.processDisbursement({
      treasury_account_id: treasuryAccountId,
      beneficiary_account_id: beneficiaryAccountId,
      amount: Number(amount),
      currency_code: selectedCurrency,
      payment_method: paymentMethod.toLowerCase(),
      reference: checkOrTransferRef.trim() || referenceNumber.trim() || undefined,
      notes: notes.trim() || undefined,
      voucher_date: voucherDate,
    });
    toast.success(m.finance_ui_disbursement_posted_success());
    clearForm();
    showAddForm = false;
    await loadData();
  } catch (err: any) {
    console.error("Failed to record disbursement:", err);
    toast.error(err?.message || m.finance_ui_disbursement_save_failed());
  } finally {
    isSubmitting = false;
  }
}

function handlePrint(d: any) {
  printParameters = {
    voucher_id: d.id,
    reference_number: d.reference_number || d.id.slice(0, 8),
    org_id: orgId,
  };
  printTitle = m.finance_ui_disbursement_print_title({ number: d.reference_number || d.id.slice(0, 8) });
  printDialogOpen = true;
}

// ── Grid Data ─────────────────────────────────────────────────────────
const gridSource = $derived(
  disbursements.map((d, idx) => {
    const beneficiaryId = d.debit_account_id || d.lines?.[0]?.account_id;
    const treasuryId = d.credit_account_id || d.lines?.[1]?.account_id;
    const amt = Number(d.amount || d.lines?.[0]?.debit || 0);
    const curr = d.currency_code || d.currency || "YER";

    return {
      ...d,
      __idx: idx + 1,
      receipt_display: d.reference_number || d.id.slice(0, 8),
      date_display: d.accounting_date ? formatDate(d.accounting_date) : "—",
      beneficiary_display: getAccountName(beneficiaryId),
      treasury_display: getAccountName(treasuryId),
      amount_formatted: formatCurrency(amt, curr),
      currency_display: curr,
      notes_display: d.description_ar || d.description || "—",
      status_display: m.finance_ui_disbursed_status(),
    };
  })
);

const filteredSource = $derived(
  gridSource.filter((row) => {
    if (filterDate && row.accounting_date && !String(row.accounting_date).startsWith(filterDate)) {
      return false;
    }
    return true;
  })
);

const columns: OrbisColumnDef[] = [
  { id: "__idx", prop: "__idx", header: "#", name: "#", width: 48, size: 48, readonly: true, pin: "left", align: "center" },
  { id: "receipt_display", prop: "receipt_display", header: m.finance_ui_voucher_number(), name: m.finance_ui_voucher_number(), width: 110, size: 110, readonly: true, cellClass: "font-mono font-bold" },
  { id: "date_display", prop: "date_display", header: m.finance_ui_date(), name: m.finance_ui_date(), width: 110, size: 110, readonly: true, align: "center" },
  { id: "beneficiary_display", prop: "beneficiary_display", header: m.finance_ui_disbursement_beneficiary(), name: m.finance_ui_disbursement_beneficiary(), width: 220, size: 220, readonly: true, cellClass: "font-medium" },
  { id: "treasury_display", prop: "treasury_display", header: m.finance_ui_treasury_account(), name: m.finance_ui_treasury_account(), width: 180, size: 180, readonly: true },
  { id: "amount_formatted", prop: "amount_formatted", header: m.finance_ui_disbursed_amount(), name: m.finance_ui_disbursed_amount(), width: 140, size: 140, readonly: true, align: "end", cellClass: "font-mono font-bold text-rose-600 dark:text-rose-400" },
  { id: "currency_display", prop: "currency_display", header: m.finance_ui_currency(), name: m.finance_ui_currency(), width: 80, size: 80, readonly: true, align: "center" },
  { id: "notes_display", prop: "notes_display", header: m.finance_ui_description_notes(), name: m.finance_ui_description_notes(), width: 220, size: 220, readonly: true },
  { id: "status_display", prop: "status_display", header: m.finance_ui_status(), name: m.finance_ui_status(), width: 90, size: 90, type: "badge", align: "center", readonly: true },
];

function exportCsv() {
  const rows = filteredSource.map((d) => [
    d.receipt_display,
    d.date_display,
    d.beneficiary_display,
    d.treasury_display,
    d.amount_formatted,
    d.currency_display,
    d.notes_display,
    d.status_display,
  ]);
  const csv = [
    [m.finance_ui_voucher_number(), m.finance_ui_date(), m.finance_ui_beneficiary(), m.finance_ui_treasury(), m.finance_ui_amount(), m.finance_ui_currency(), m.finance_ui_notes(), m.finance_ui_status()],
    ...rows,
  ]
    .map((row) => row.map((v) => `"${String(v ?? "").replaceAll('"', '""')}"`).join(","))
    .join("\n");
  const url = URL.createObjectURL(new Blob(["\ufeff" + csv], { type: "text/csv;charset=utf-8" }));
  const link = document.createElement("a");
  link.href = url;
  link.download = `disbursements-${orgId}.csv`;
  link.click();
  URL.revokeObjectURL(url);
}
</script>

<div class="flex h-full min-h-0 w-full flex-col overflow-hidden p-6 font-sans bg-background/50">
  <div class="flex min-h-0 w-full flex-1 flex-col gap-4" in:fade={{ duration: 150 }}>
    
    <!-- Top Action Bar -->
    <header class="flex shrink-0 items-center justify-between gap-4">
      <div class="flex items-center gap-2">
        <Button
          variant={showAddForm ? "secondary" : "default"}
          size="sm"
          class="h-8 px-3 text-xs font-bold shrink-0 gap-1.5"
          onclick={() => { showAddForm = !showAddForm; }}
        >
          {#if showAddForm}
            <XMark size={14} />
            <span>{m.finance_ui_cancel()}</span>
          {:else}
            <Plus size={14} />
            <span>{m.finance_ui_new_disbursement()}</span>
          {/if}
        </Button>
      </div>

      <div class="flex items-center gap-2">
        <div class="relative w-48 sm:w-64">
          <Search size={14} class="absolute inset-s-2.5 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
          <Input
            type="text"
            placeholder={m.finance_ui_disbursement_search_placeholder()}
            bind:value={searchQuery}
            class="h-8 ps-8 text-xs bg-card border-border/60"
          />
        </div>

        <Input
          type="date"
          bind:value={filterDate}
          class="h-8 w-36 text-xs bg-card border-border/60 font-mono"
        />

        <Button variant="ghost" size="icon-sm" onclick={loadData} class="text-muted-foreground">
          <Refresh size={14} class={isLoading ? "animate-spin" : ""} />
        </Button>

        <Button variant="ghost" size="icon-sm" onclick={exportCsv} class="text-muted-foreground">
          <Download size={14} />
        </Button>
      </div>
    </header>

    <!-- In-Page Creation Card (Matching FinanceOverview Card Style) -->
    {#if showAddForm}
      <div transition:slide={{ duration: 200 }} class="shrink-0">
        <Card class="border border-primary/20 bg-card shadow-sm text-start">
          <CardHeader class="py-3 px-4 border-b border-border/40">
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-2">
                <span class="h-2 w-2 rounded-full bg-rose-500"></span>
                <CardTitle class="text-xs font-bold uppercase tracking-wider text-foreground">
                  {m.finance_ui_disbursement_create_heading()}
                </CardTitle>
              </div>
              <Button variant="ghost" size="icon-xs" onclick={() => showAddForm = false} class="text-muted-foreground">
                <XMark size={14} />
              </Button>
            </div>
          </CardHeader>

          <CardContent class="p-4 flex flex-col gap-4">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <Label class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_disbursement_from_account()}</Label>
                <SearchableSelect
                  value={treasuryAccountId}
                  placeholder={m.finance_ui_treasury_select_placeholder()}
                  items={treasuries}
                  getItemValue={(t) => t.id}
                  getItemLabel={(t) => `${t.name_ar || t.name} (${t.currency || "YER"})`}
                  onchange={(val) => { if (val) treasuryAccountId = val; }}
                  class="w-full h-8 text-xs bg-background border-border/60"
                />
              </div>

              <div>
                <Label class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_disbursement_to_account()}</Label>
                <AccountSearchInput
                  bind:value={beneficiaryAccountId}
                  {accounts}
                  placeholder={m.finance_ui_expense_account_select_placeholder()}
                />
              </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
              <div>
                <Label class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_voucher_number_optional()}</Label>
                <Input
                  type="text"
                  bind:value={referenceNumber}
                  placeholder={m.finance_ui_automatic()}
                  class="h-8 text-xs font-mono bg-background border-border/60"
                />
              </div>

              <div>
                <Label class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_date()}</Label>
                <Input
                  type="date"
                  bind:value={voucherDate}
                  class="h-8 text-xs font-mono bg-background border-border/60"
                />
              </div>

              <div>
                <Label class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_disbursed_amount()}</Label>
                <Input
                  type="number"
                  bind:value={amount}
                  placeholder="0.00"
                  class="h-8 text-xs font-mono font-bold text-rose-600 dark:text-rose-400 bg-background border-border/60"
                />
              </div>

              <div>
                <Label class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_currency()}</Label>
                <SearchableSelect
                  value={selectedCurrency}
                  placeholder={m.finance_ui_currency_placeholder()}
                  items={currencies}
                  getItemValue={(c) => c.code}
                  getItemLabel={(c) => c.name}
                  onchange={(val) => { if (val) selectedCurrency = val; }}
                  class="w-full h-8 text-xs bg-background border-border/60"
                />
              </div>
            </div>

            <div>
              <Label class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_disbursement_notes()}</Label>
              <Input
                type="text"
                bind:value={notes}
                placeholder={m.finance_ui_disbursement_notes_placeholder()}
                class="h-8 text-xs bg-background border-border/60"
              />
            </div>

            <div class="flex items-center justify-end gap-2 pt-2 border-t border-border/40">
              <Button variant="ghost" size="sm" class="h-8 text-xs" onclick={clearForm}>
                {m.finance_ui_clear_fields()}
              </Button>
              <Button
                variant="default"
                size="sm"
                class="h-8 text-xs font-bold px-4"
                disabled={isSubmitting}
                onclick={handleSaveDisbursement}
              >
                {isSubmitting ? m.finance_ui_saving() : m.finance_ui_disbursement_save()}
              </Button>
            </div>
          </CardContent>
        </Card>
      </div>
    {/if}

    <!-- Grid Container -->
    <div class="flex-1 min-h-0 overflow-hidden">
      <OrbisGrid
        columns={columns}
        source={filteredSource}
        loading={isLoading}
        clientQuery
        search={searchQuery}
        rtl={isAr()}
        readonly
        emptyTitle={m.finance_ui_no_disbursements()}
      />
    </div>
  </div>
</div>

<PdfReportViewer
  bind:open={printDialogOpen}
  reportTemplateId="disbursement-voucher"
  parameters={printParameters}
  title={printTitle}
/>
