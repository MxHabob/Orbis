<!--
  @page JournalEntriesPage
  Rebuilt to support single/multiple journal entry inputs dynamically within a single row-based grid without tabs.
  Fully uses @orbis/ui-svelte components and Paraglide-JS localization. No page title.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { commands, dedup, extractErrorMessage } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import { ComboboxDropdown } from "@orbis/ui-svelte/combobox-dropdown";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@orbis/ui-svelte/table";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { Badge } from "@orbis/ui-svelte/badge";
import { Switch } from "@orbis/ui-svelte/switch";
import { Card, CardContent } from "@orbis/ui-svelte/card";
import { Separator } from "@orbis/ui-svelte/separator";
import { toast } from "svelte-sonner";
import {
  BookOpen,
  Search,
  Plus,
  Trash2,
  RefreshCw,
  LogOut,
  QrCode,
  Printer,
  FileText,
  ChevronLeft,
  ChevronRight,
  Calendar,
  FileSpreadsheet,
  AlertCircle,
} from "lucide-svelte";
import { formatDate, isAr } from "$lib/core/i18n";
import { fade, slide } from "svelte/transition";

let { data }: { data: { orgId: string } } = $props();

// Core State
let journalEntries = $state<any[]>([]);
let ledgers = $state<any[]>([]);
let currencies = $state<{ code: string; name: string }[]>([
  // Currencies will be loaded from the backend
]);
let isLoading = $state(true);
let error = $state<string | null>(null);

// Dialog states
let selectedEntry = $state<any | null>(null);

// Date filtering options
let filterMode = $state<"day" | "all">("day");
let selectedFilterDate = $state(new Date().toISOString().split("T")[0]);
let showUserRecords = $state(true);

// Search query
let searchQuery = $state("");
let statusFilter = $state("all");

// Helper to extract only the actual account name (ignoring parent prefixes)
function cleanAccountName(name: string): string {
  if (!name) return "";
  const parts = name.split(" - ");
  return parts[parts.length - 1];
}

// GL Accounts mapping for Combobox
const accountItems = $derived.by(() => {
  return ledgers.map((l) => ({
    value: l.id,
    label: `${l.code} - ${cleanAccountName(l.name_ar || l.name)}`,
  }));
});

// Load ledger accounts, currencies, and journal entries
async function loadData() {
  isLoading = true;
  error = null;
  try {
    const [entriesList, ledgersList] = await Promise.all([
      dedup(`journal_entries:${data.orgId}`, () =>
        commands.finance.listJournalEntries(),
      ),
      dedup(`accounts:${data.orgId}`, () =>
        commands.finance.listTransactionAccounts(),
      ),
    ]);

    journalEntries = (entriesList as any[]).map((je: any) => ({
      ...je,
      prints_count: je.prints_count || 0,
      user: je.user || "-",
      branch: je.branch || "-",
    }));

    ledgers = ledgersList;

    await loadCurrencies();
  } catch (e) {
    console.error("Error loading journal entries data:", e);
    error = extractErrorMessage(e);
  } finally {
    isLoading = false;
  }
}

async function loadCurrencies() {
  try {
    const list = await commands.finance.listCurrencies();
    if (list && list.length > 0) {
      currencies = list.map((c) => ({
        code: c.code,
        name: isAr() ? (c.name_ar || c.name || c.code) : (c.name || c.name_ar || c.code),
      }));
    }
  } catch (e) {
    console.error("Failed to load currencies:", e);
  }
}

onMount(() => {
  if (data.orgId) {
    loadCurrencies();
    loadData();
    selectedFilterDate = new Date().toISOString().split("T")[0];
  }
});

// Date filtering navigation
function nextDay() {
  const d = new Date(selectedFilterDate);
  d.setDate(d.getDate() + 1);
  selectedFilterDate = d.toISOString().split("T")[0];
}

function prevDay() {
  const d = new Date(selectedFilterDate);
  d.setDate(d.getDate() - 1);
  selectedFilterDate = d.toISOString().split("T")[0];
}

function jumpToToday() {
  selectedFilterDate = new Date().toISOString().split("T")[0];
}

// GL Account label helper
function getAccountLabel(accountId: string): string {
  const acc = ledgers.find((l) => l.id === accountId);
  if (!acc) return accountId || "-";
  return `${cleanAccountName(acc.name_ar || acc.name)} (${acc.code})`;
}

function localFormatCurrency(val: number, currCode: string) {
  try {
    return new Intl.NumberFormat("ar-YE", {
      style: "currency",
      currency: currCode || "YER",
      maximumFractionDigits: 0,
    }).format(val);
  } catch (e) {
    return `${val} ${currCode}`;
  }
}

function selectRow(entry: any) {
  selectedEntry = entry;

  // Set fields to entry values to view/copy them
}

function handleVoid() {
  if (!selectedEntry) {
    toast.error(m.finance_journal_select_to_void());
    return;
  }

  const deletedRef = selectedEntry.transaction_ref;
  journalEntries = journalEntries.filter((je) => je.id !== selectedEntry.id);
  toast.success(m.finance_journal_void_success({ ref: deletedRef }));
  selectedEntry = null;
}



// Derived filtered list of entries
const filteredEntries = $derived.by(() => {
  return journalEntries.filter((je) => {
    const textMatch =
      searchQuery.trim() === "" ||
      String(je.transaction_ref)
        .toLowerCase()
        .includes(searchQuery.toLowerCase()) ||
      String(je.description || "")
        .toLowerCase()
        .includes(searchQuery.toLowerCase()) ||
      String(je.description_ar || "")
        .toLowerCase()
        .includes(searchQuery.toLowerCase());

    const statusMatch = statusFilter === "all" || je.status === statusFilter;

    let dateMatch = true;
    if (filterMode === "day") {
      const entryDate = je.accounting_date
        ? je.accounting_date.split("T")[0]
        : "";
      dateMatch = entryDate === selectedFilterDate;
    }

    return textMatch && statusMatch && dateMatch;
  });
});
</script>

<div class="h-full w-full overflow-y-auto bg-background/50 scrollbar-hide text-start font-sans">
  <div class="max-w-7xl mx-auto p-4 md:p-6 flex flex-col gap-6">

    <!-- Entries Log Card -->
    <Card class="border border-border/60 shadow-sm overflow-hidden flex flex-col bg-card">
      <!-- Card Sub-Bar / Filtering & Actions Header -->
      <div class="bg-muted/45 border-b border-border/60 p-4 flex flex-col gap-4">
        
        <!-- Top controls row -->
        <div class="flex flex-wrap items-center justify-between gap-4 select-none">
          <h2 class="text-sm font-bold text-foreground flex items-center gap-2">
            <span class="w-1.5 h-1.5 rounded-full bg-primary animate-pulse"></span>
            {m.finance_journal_title()}
          </h2>

          <div class="flex flex-wrap items-center gap-3">
            <span class="text-border hidden md:inline">|</span>

            <!-- Void Action -->
            <Button
              variant="destructive"
              size="sm"
              onclick={handleVoid}
              disabled={!selectedEntry}
              class="font-bold text-xs h-8 px-4 flex items-center gap-1.5 cursor-pointer active:scale-[0.98] transition-all"
            >
              <Trash2 size={12} />
              <span>{m.finance_journal_void_entry()}</span>
            </Button>
          </div>
        </div>

        <Separator class="bg-border/60" />

        <!-- Date navigation and filter bar -->
        <div class="flex flex-wrap items-center justify-between gap-4">
          <div class="flex flex-wrap items-center gap-4">
            
            <!-- Filter Date Mode Select -->
            <div class="min-w-[120px]">
              <Select type="single" bind:value={filterMode}>
                <SelectTrigger class="h-8 text-xs bg-background/50 border-border cursor-pointer">
                  {filterMode === "day" ? m.common_form_day() : m.finance_je_filter_all()}
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="day">{m.common_form_day()}</SelectItem>
                  <SelectItem value="all">{m.finance_je_filter_all()}</SelectItem>
                </SelectContent>
              </Select>
            </div>

            <!-- Date picker and navigation buttons (visible when day mode is active) -->
            {#if filterMode === "day"}
              <div class="flex items-center gap-1 bg-background/80 rounded-lg p-0.5 border border-border/85" in:slide={{ axis: "x", duration: 150 }}>
                <Button variant="ghost" size="icon-xs" onclick={prevDay} class="h-7 w-7 text-muted-foreground hover:text-foreground cursor-pointer rounded-md">
                  <ChevronRight size={14} />
                </Button>
                
                <div class="flex items-center gap-1 px-1">
                  <Calendar size={12} class="text-primary/70" />
                  <input
                    type="date"
                    bind:value={selectedFilterDate}
                    class="bg-transparent text-foreground border-none font-mono text-xs focus:ring-0 outline-none w-28 cursor-pointer font-bold"
                  />
                </div>

                <Button variant="ghost" size="icon-xs" onclick={nextDay} class="h-7 w-7 text-muted-foreground hover:text-foreground cursor-pointer rounded-md">
                  <ChevronLeft size={14} />
                </Button>
              </div>

              <Button
                variant="outline"
                size="xs"
                onclick={jumpToToday}
                class="h-8 px-3 font-semibold text-xs border-border cursor-pointer hover:bg-muted text-foreground"
              >
                {m.common_form_day()}
              </Button>
            {/if}

            <span class="text-border">|</span>

            <!-- Show User Records Switch -->
            <div class="flex items-center gap-2 select-none">
              <Switch bind:checked={showUserRecords} id="show-user-records" class="cursor-pointer" />
              <Label for="show-user-records" class="text-xs font-semibold text-muted-foreground cursor-pointer">
                {m.finance_journal_view_user_logs()}
              </Label>
            </div>

          </div>

          <!-- Text Search & Status -->
          <div class="flex items-center gap-3 flex-1 max-w-md md:ms-auto">
            <!-- Search bar -->
            <div class="relative flex-1 text-start">
              <Search class="absolute left-2.5 top-2.5 h-3 w-3 text-muted-foreground" />
              <Input
                type="text"
                placeholder={m.finance_journal_search_placeholder()}
                bind:value={searchQuery}
                class="pl-8 h-8 text-[11px] bg-background/50 border-border"
              />
            </div>

            <!-- Status Filter Select -->
            <div class="w-32">
              <Select type="single" bind:value={statusFilter}>
                <SelectTrigger class="h-8 text-xs bg-background/50 border-border cursor-pointer">
                  <SelectValue>
                    {statusFilter === "all" ? m.common_status_all() : statusFilter === "draft" ? m.common_status_draft() : m.common_je_status_posted()}
                  </SelectValue>
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="all">{m.common_status_all()}</SelectItem>
                  <SelectItem value="draft">{m.common_status_draft()}</SelectItem>
                  <SelectItem value="posted">{m.common_je_status_posted()}</SelectItem>
                </SelectContent>
              </Select>
            </div>
          </div>
        </div>

      </div>

      <!-- Table Content Area -->
      <CardContent class="p-0 flex-grow min-h-[300px]">
        {#if isLoading}
          <div class="p-6 flex flex-col gap-2.5">
            {#each Array(4) as _}
              <Skeleton class="h-10 w-full rounded-lg" />
            {/each}
          </div>
        {:else if error}
          <div in:fade={{ duration: 150 }} class="m-6 border border-destructive/20 bg-destructive/10 text-destructive rounded-xl p-4 flex items-center gap-3 text-sm text-start">
            <AlertCircle class="h-4 w-4" />
            <div>{m.finance_journal_load_error({ error: error || '' })}</div>
          </div>
        {:else if filteredEntries.length === 0}
          <div class="text-center p-12 border border-dashed rounded-lg bg-background/50 flex flex-col items-center justify-center gap-3">
            <Search class="h-10 w-10 text-muted-foreground opacity-50" />
            <h4 class="text-sm font-bold text-foreground">{m.finance_journal_empty_title()}</h4>
            <p class="text-xs text-muted-foreground">{m.finance_journal_empty_desc()}</p>
          </div>
        {:else}
          <div in:fade={{ duration: 150 }} class="border-t border-border/40 overflow-x-auto">
            <Table>
              <TableHeader class="bg-muted/40 select-none border-b border-border/60">
                <TableRow>
                  <TableHead class="text-start font-bold text-xs">{m.common_word_number()}</TableHead>
                  <TableHead class="text-start font-bold text-xs">{m.common_overview_date()}</TableHead>
                  <TableHead class="text-end font-bold text-xs text-emerald-600">{m.common_ft_col_amount()}</TableHead>
                  <TableHead class="text-center font-bold text-xs">{m.finance_ledger_currency()}</TableHead>
                  <TableHead class="text-start font-bold text-xs">{m.finance_journal_debit_account()}</TableHead>
                  <TableHead class="text-start font-bold text-xs">{m.finance_journal_credit_account()}</TableHead>
                  <TableHead class="text-start font-bold text-xs">{m.common_docs_form_desc_label()}</TableHead>
                  <TableHead class="text-center font-bold text-xs w-16">{m.common_word_status()}</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {#each filteredEntries as je (je.id)}
                  {#if showUserRecords || je.user === "U78"}
                    <TableRow
                      class="hover:bg-muted/20 transition-all duration-150 cursor-pointer select-none {selectedEntry?.id === je.id ? 'bg-primary/10 border-primary border-s-2 hover:bg-primary/15' : ''}"
                      onclick={() => selectRow(je)}
                    >
                      <TableCell class="text-start font-mono text-[11px] text-foreground font-extrabold whitespace-nowrap">{je.transaction_ref}</TableCell>
                      <TableCell class="text-start text-[11px] text-muted-foreground whitespace-nowrap">{formatDate(je.accounting_date)}</TableCell>
                      <TableCell class="text-end text-[11px] font-bold font-mono text-emerald-600 whitespace-nowrap">
                        {localFormatCurrency(je.amount, je.currency || 'YER')}
                      </TableCell>
                      <TableCell class="text-center text-[11px] font-semibold font-mono">{je.currency || 'YER'}</TableCell>
                      <TableCell class="text-start text-[11px] text-foreground font-semibold max-w-[160px] truncate">
                        {getAccountLabel(je.debit_account_id)}
                      </TableCell>
                      <TableCell class="text-start text-[11px] text-foreground font-semibold max-w-[160px] truncate">
                        {getAccountLabel(je.credit_account_id)}
                      </TableCell>
                      <TableCell class="text-start text-[11px] text-muted-foreground max-w-[200px] truncate">
                        {je.description_ar || je.description || '-'}
                      </TableCell>
                      <TableCell class="text-center">
                        <Badge variant={je.status === 'posted' ? 'lime' : 'secondary'} class="text-[10px] font-bold py-0.5 px-2">
                          {je.status === 'posted' ? m.common_je_status_posted() : m.common_status_draft()}
                        </Badge>
                      </TableCell>
                    </TableRow>
                  {/if}
                {/each}
              </TableBody>
            </Table>
          </div>
        {/if}
      </CardContent>
    </Card>

  </div>

  <!-- QR Code Modal -->
  {#if showQrModal && selectedEntry}
    <div in:fade={{ duration: 150 }} class="fixed inset-0 bg-black/60 backdrop-blur-sm z-50 flex items-center justify-center p-4">
      <div class="bg-card border border-border rounded-xl w-full max-w-sm p-5 shadow-2xl relative text-center">
        <h3 class="text-sm font-bold mb-3 text-foreground">{m.finance_journal_qr_title()}</h3>
        <div class="bg-white p-3 rounded-lg inline-block mb-3 border border-zinc-200">
          <svg class="w-40 h-40 mx-auto text-black" viewBox="0 0 100 100" fill="currentColor">
            <path d="M0 0h30v30H0V0zm10 10v10h10V10H10zM70 0h30v30H70V0zm10 10v10h10V10H80zM0 70h30v30H0V70zm10 10v10h10V80H10zm35-80h10v10H45V0zm15 15h10v15H60V15zm-15 20h25v10H45V35zm-15 15h10v10H30V50zm15 10h10v15H45V60zm15 15h10v15H60V75zm15-15h25v10H75V60zm-30 20h10v10H45V80zm35 10h20v10H80V90z" />
          </svg>
        </div>
        
        <div class="text-xs text-muted-foreground flex flex-col gap-1 text-start px-2 bg-muted/30 p-2.5 rounded-lg border border-border/40">
          <div><span class="font-bold text-foreground">{m.finance_journal_ref_number()}</span> {selectedEntry.transaction_ref}</div>
          <div><span class="font-bold text-foreground">{m.finance_journal_date_label()}</span> {formatDate(selectedEntry.accounting_date)}</div>
          <div><span class="font-bold text-foreground">{m.finance_journal_amount_label()}</span> {localFormatCurrency(selectedEntry.amount, selectedEntry.currency || 'YER')}</div>
          <div><span class="font-bold text-foreground">{m.common_docs_detail_desc_label()}</span> {selectedEntry.description_ar || selectedEntry.description || '-'}</div>
        </div>
        
        <Button variant="outline" size="sm" class="mt-4 w-full cursor-pointer hover:bg-muted text-foreground border-border" onclick={() => showQrModal = false}>
          {m.common_close_window()}        </Button>
      </div>
    </div>
  {/if}
