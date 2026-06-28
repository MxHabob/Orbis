<!--
  @page JournalEntriesPage
  Lists all double-entry journal logs in the system with search/filtering.
-->
<script lang="ts">
import { onMount } from "svelte";
import { commands } from "$lib/services/ipc";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@orbis/ui-svelte";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import {
  BookOpen,
  Search,
  Calendar,
  Layers,
  ArrowUpDown,
  Filter,
} from "lucide-svelte";
import { getLocale } from "$lib/paraglide/runtime";
import { formatCurrency, formatDate } from "$lib/i18n-utils";
import { fade } from "svelte/transition";
import * as m from "$lib/paraglide/messages";

let { data }: { data: { orgId: string } } = $props();

// State
let journalEntries = $state<any[]>([]);
let ledgers = $state<any[]>([]);
let isLoading = $state(true);
let error = $state<string | null>(null);

// Search & Filtering
let searchQuery = $state("");
let statusFilter = $state("all");

async function loadData() {
  isLoading = true;
  error = null;
  try {
    const [entriesList, ledgersList] = await Promise.all([
      commands.finance.getJournalEntries(data.orgId),
      commands.finance.getLedgerAccounts(data.orgId),
    ]);
    journalEntries = entriesList;
    ledgers = ledgersList;
  } catch (e) {
    console.error("Error loading journal entries:", e);
    error = e instanceof Error ? e.message : String(e);
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (data.orgId) {
    loadData();
  }
});

// Map account ID to name & code
function getAccountLabel(accountId: string): string {
  const acc = ledgers.find((l) => l.id === accountId);
  if (!acc) return accountId;
  return `${acc.name_ar || acc.name} (${acc.code})`;
}

// Filtered entries
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

    return textMatch && statusMatch;
  });
});
</script>

<div class="h-full flex flex-col overflow-hidden bg-background p-6">
  <!-- Top header section -->
  <div class="flex items-center justify-between pb-6 select-none border-b border-border">
    <div class="flex flex-col gap-1 text-start">
      <h1 class="text-xl font-bold tracking-tight text-foreground flex items-center gap-2">
        <BookOpen class="text-primary" size={20} />
        <span>{m.je_title ? m.je_title() : "القيود اليومية"}</span>
      </h1>
      <p class="text-xs text-muted-foreground">
        {m.je_desc ? m.je_desc() : "عرض وإدارة قيود اليومية المحاسبية المزدوجة ومطابقة الحسابات الدفترية"}
      </p>
    </div>
  </div>

  <!-- Filter / Search bar -->
  <div class="flex items-center justify-between gap-4 py-4 select-none">
    <!-- Search -->
    <div class="relative flex-1 max-w-sm">
      <Search class="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground" />
      <Input
        type="text"
        placeholder={m.search_placeholder ? m.search_placeholder() : "البحث بالرقم المرجعي أو البيان..."}
        bind:value={searchQuery}
        class="pl-9 h-9 text-xs"
      />
    </div>

    <!-- Filters dropdown/select -->
    <div class="flex items-center gap-2">
      <select
        bind:value={statusFilter}
        class="bg-background text-foreground border border-border rounded-md px-3 py-1.5 text-xs focus:ring-1 focus:ring-ring outline-none h-9 cursor-pointer"
      >
        <option value="all">{m.attendance_status_unknown() === "غير محدد" ? "كل الحالات" : "All Statuses"}</option>
        <option value="draft">{m.je_status_draft ? m.je_status_draft() : "مسودة"}</option>
        <option value="posted">{m.je_status_posted ? m.je_status_posted() : "مرحل"}</option>
      </select>

      <Button variant="outline" size="sm" onclick={loadData} class="h-9 text-xs flex items-center gap-1 cursor-pointer">
        <Layers size={13} />
        <span>{m.attendance_status_unknown() === "غير محدد" ? "تحديث" : "Refresh"}</span>
      </Button>
    </div>
  </div>

  <!-- Content panel -->
  <div class="flex-1 overflow-y-auto py-2">
    {#if isLoading}
      <div class="flex flex-col gap-3">
        {#each Array(5) as _}
          <Skeleton class="h-14 w-full rounded-lg" />
        {/each}
      </div>
    {:else if error}
      <div
        in:fade={{ duration: 150 }}
        class="border border-destructive/20 bg-destructive/10 text-destructive rounded-xl p-4 flex items-center gap-3 text-sm text-start"
      >
        <ShieldAlert size={18} />
        <div>
          <span class="font-bold">Error:</span> {error}
        </div>
      </div>
    {:else if filteredEntries.length === 0}
      <div
        in:fade={{ duration: 150 }}
        class="border border-dashed border-border/80 rounded-2xl p-12 flex flex-col items-center justify-center text-center gap-3 select-none"
      >
        <div class="w-12 h-12 rounded-full bg-primary/10 flex items-center justify-center text-primary mb-2">
          <BookOpen size={24} />
        </div>
        <h3 class="text-sm font-bold text-foreground">
          {m.je_empty ? m.je_empty() : "لا توجد قيود يومية"}
        </h3>
        <p class="text-xs text-muted-foreground max-w-sm">
          {m.je_empty_desc ? m.je_empty_desc() : "لم يتم تسجيل أي عمليات مالية أو قيود يومية بعد"}
        </p>
      </div>
    {:else}
      <div in:fade={{ duration: 150 }} class="border border-border/60 rounded-xl overflow-hidden bg-card/30">
        <Table>
          <TableHeader class="bg-muted/40 select-none">
            <TableRow>
              <TableHead class="text-start font-bold text-xs">{m.je_col_ref ? m.je_col_ref() : "الرقم المرجعي"}</TableHead>
              <TableHead class="text-start font-bold text-xs">{m.je_col_date ? m.je_col_date() : "تاريخ القيد"}</TableHead>
              <TableHead class="text-start font-bold text-xs">{m.je_col_debit_acc ? m.je_col_debit_acc() : "الحساب المدين"}</TableHead>
              <TableHead class="text-start font-bold text-xs">{m.je_col_credit_acc ? m.je_col_credit_acc() : "الحساب الدائن"}</TableHead>
              <TableHead class="text-start font-bold text-xs">{m.je_col_amount ? m.je_col_amount() : "المبلغ"}</TableHead>
              <TableHead class="text-start font-bold text-xs">{m.je_col_status ? m.je_col_status() : "الحالة"}</TableHead>
              <TableHead class="text-start font-bold text-xs">{m.je_col_desc ? m.je_col_desc() : "البيان"}</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {#each filteredEntries as je (je.id)}
              <TableRow class="hover:bg-muted/20 transition-all duration-150">
                <TableCell class="text-start font-mono text-xs text-foreground font-bold">{je.transaction_ref}</TableCell>
                <TableCell class="text-start text-xs text-muted-foreground whitespace-nowrap">{formatDate(je.accounting_date)}</TableCell>
                <TableCell class="text-start text-xs text-emerald-500 font-medium">
                  {getAccountLabel(je.debit_account_id)}
                </TableCell>
                <TableCell class="text-start text-xs text-rose-500 font-medium">
                  {getAccountLabel(je.credit_account_id)}
                </TableCell>
                <TableCell class="text-start text-xs font-bold text-foreground">
                  {formatCurrency(je.amount, je.currency || 'YER')}
                </TableCell>
                <TableCell class="text-start text-xs">
                  {#if String(je.status).toLowerCase() === "posted"}
                    <span class="px-2 py-0.5 rounded-full text-[10px] font-semibold bg-emerald-500/10 text-emerald-500 border border-emerald-500/20">
                      {m.je_status_posted ? m.je_status_posted() : "مرحل"}
                    </span>
                  {:else}
                    <span class="px-2 py-0.5 rounded-full text-[10px] font-semibold bg-amber-500/10 text-amber-500 border border-amber-500/20">
                      {m.je_status_draft ? m.je_status_draft() : "مسودة"}
                    </span>
                  {/if}
                </TableCell>
                <TableCell class="text-start text-xs text-muted-foreground line-clamp-1 max-w-[200px]">
                  {je.description_ar || je.description || '-'}
                </TableCell>
              </TableRow>
            {/each}
          </TableBody>
        </Table>
      </div>
    {/if}
  </div>
</div>
