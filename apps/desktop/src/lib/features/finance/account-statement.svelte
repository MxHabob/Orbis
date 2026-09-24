<!-- Account Statement workspace component. -->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { goto } from "$app/navigation";
import { api } from "$lib/api";
import { getLocale } from "$lib/paraglide/runtime";
import { formatCurrency } from "$lib/core/i18n";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { ComboboxDropdown } from "@orbis/ui/combobox-dropdown";
import { Skeleton } from "@orbis/ui/skeleton";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardAction,
  CardContent,
  CardFooter,
} from "@orbis/ui/card";
import { OrbisGrid, type OrbisColumnDef } from "@orbis/svelte-grid";
import CurrencySelector from "$lib/shared/components/currency-selector.svelte";
import AccountTree from "$lib/features/finance/components/account-tree.svelte";
import {
  ArrowLeft,
  ArrowRight,
  DocumentChartBar,
  Refresh,
  Scale,
  Search,
  BuildingLibrary,
} from "@orbis/icons";
import { exportToCsv } from "$lib/shared/utils/export";

let { orgId }: { orgId: string } = $props();

let accounts = $state<any[]>([]);
let currencies = $state<any[]>([]);
let payments = $state<any[]>([]);
let isLoading = $state(true);
let error = $state<string | null>(null);
let activeView = $state<"all" | "single">("all");
let selectedAccountId = $state("");
let selectedCurrency = $state("all");
let selectedIds = $state<Set<string>>(new Set());
let searchQuery = $state("");
let period = $state("all");
let movementFilter = $state<"all" | "active">("all");
let balanceFilter = $state<"all" | "debit" | "credit">("all");
let asOfDate = $state("");

async function loadData() {
  isLoading = true;
  error = null;
  try {
    const [accountList, currencyList, paymentList] = await Promise.all([
      api.listAccounts(),
      api.listCurrencies().catch(() => []),
      api.listPayments(null, null).catch(() => []),
    ]);
    accounts = accountList || [];
    currencies = currencyList || [];
    payments = paymentList || [];

    const leaves = accounts.filter(
      (account) =>
        !accounts.some((candidate) => candidate.parent_id === account.id),
    );
    selectedIds = new Set(leaves.map((account) => account.id));
    if (!selectedAccountId && leaves[0]) selectedAccountId = leaves[0].id;
  } catch (cause) {
    error = cause instanceof Error ? cause.message : String(cause);
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (orgId) loadData();
});

function buildTree(items: any[]): any[] {
  const nodes = new Map(
    items.map((account) => [account.id, { ...account, children: [] }]),
  );
  const roots: any[] = [];
  for (const account of items) {
    const node = nodes.get(account.id);
    const parent = account.parent_id ? nodes.get(account.parent_id) : undefined;
    if (parent) parent.children.push(node);
    else roots.push(node);
  }
  return roots;
}

const treeCategories = $derived.by(() => {
  const definitions = [
    ["asset", "الأصول", "Assets"],
    ["liability", "الخصوم", "Liabilities"],
    ["equity", "حقوق الملكية", "Equity"],
    ["revenue", "الإيرادات", "Revenue"],
    ["expense", "المصروفات", "Expenses"],
  ];
  return definitions.map(([id, name_ar, name_en]) => ({
    id,
    name_ar,
    name_en,
    accounts: buildTree(
      accounts.filter(
        (account) =>
          String(account.type || account.item_type || "asset").toLowerCase() ===
          id,
      ),
    ),
  }));
});

const activeCurrencies = $derived(
  currencies
    .map((currency) => ({
      code: String(currency.code || "").toUpperCase(),
      name: currency.name_ar || currency.name || currency.code,
    }))
    .filter((currency) => currency.code),
);

const visibleCurrencies = $derived(
  selectedCurrency === "all"
    ? activeCurrencies
    : activeCurrencies.filter((currency) => currency.code === selectedCurrency),
);

const leafAccounts = $derived(
  accounts.filter(
    (account) =>
      !accounts.some((candidate) => candidate.parent_id === account.id),
  ),
);
const accountItems = $derived(
  leafAccounts.map((account) => ({
    id: account.id,
    value: account.id,
    label: `${account.code} - ${account.name_ar || account.name || "-"}`,
  })),
);
const selectedAccountItem = $derived(
  accountItems.find((item) => item.value === selectedAccountId),
);

function balanceFor(account: any, code: string) {
  return Number(
    account.balances?.[code] || account.balances?.[code.toUpperCase()] || 0,
  );
}

function accountRows() {
  const query = searchQuery.trim().toLowerCase();
  return accounts
    .filter(
      (account) =>
        !accounts.some((candidate) => candidate.parent_id === account.id),
    )
    .filter((account) => selectedIds.has(account.id))
    .filter(
      (account) =>
        !query ||
        String(account.name_ar || account.name || "")
          .toLowerCase()
          .includes(query) ||
        String(account.code || "").includes(query),
    )
    .filter(
      (account) => activeView === "all" || account.id === selectedAccountId,
    )
    .filter((account) => {
      const balances = visibleCurrencies.map((currency) =>
        balanceFor(account, currency.code),
      );
      const hasMovement = balances.some((balance) => balance !== 0);
      const isDebit = balances.some((balance) => balance > 0);
      const isCredit = balances.some((balance) => balance < 0);
      return (
        (movementFilter === "all" || hasMovement) &&
        (balanceFilter === "all" ||
          (balanceFilter === "debit" ? isDebit : isCredit))
      );
    })
    .map((account, index) => {
      const row: Record<string, unknown> = {
        __idx: index + 1,
        account_name: account.name_ar || account.name || "-",
        account_code: account.code || "-",
        account_type: account.type || account.item_type || "-",
      };
      for (const currency of visibleCurrencies) {
        const balance = balanceFor(account, currency.code);
        row[`${currency.code}_debit`] =
          balance > 0 ? formatCurrency(balance, currency.code) : "-";
        row[`${currency.code}_credit`] =
          balance < 0 ? formatCurrency(Math.abs(balance), currency.code) : "-";
      }
      return row;
    });
}

function statementRows() {
  const account = accounts.find((item) => item.id === selectedAccountId);
  if (!account) return [];
  const query = searchQuery.trim().toLowerCase();
  const currency =
    selectedCurrency === "all"
      ? String(
          account.currency || activeCurrencies[0]?.code || "YER",
        ).toUpperCase()
      : selectedCurrency;
  const rows = payments
    .filter(
      (payment) =>
        payment.cash_account_id === selectedAccountId ||
        payment.account_id === selectedAccountId ||
        payment.ledger_account_id === selectedAccountId,
    )
    .filter(
      (payment) =>
        String(payment.currency || currency).toUpperCase() === currency,
    )
    .map((payment) => ({
      date:
        String(payment.paid_at || payment.created_at || "").split("T")[0] ||
        "-",
      reference:
        payment.receipt_number ||
        payment.reference ||
        payment.id?.slice(0, 8) ||
        "-",
      description: payment.notes || payment.payer_name || "Payment",
      debit: Number(payment.amount || 0),
      credit: 0,
    }))
    .filter(
      (row) =>
        !query ||
        `${row.date} ${row.reference} ${row.description}`
          .toLowerCase()
          .includes(query),
    )
    .sort((first, second) => first.date.localeCompare(second.date));

  let running = Number(account.opening_balance || 0);
  return rows.map((row, index) => {
    running += row.debit - row.credit;
    return {
      __idx: index + 1,
      date: row.date,
      reference: row.reference,
      description: row.description,
      debit: row.debit ? formatCurrency(row.debit, currency) : "-",
      credit: row.credit ? formatCurrency(row.credit, currency) : "-",
      balance: formatCurrency(Math.abs(running), currency),
    };
  });
}

const source = $derived(
  activeView === "single" ? statementRows() : accountRows(),
);

const columns = $derived.by((): OrbisColumnDef[] => {
  if (activeView === "single") {
    return [
      {
        id: "__idx",
        prop: "__idx",
        header: "#",
        name: "#",
        width: 48,
        size: 48,
        readonly: true,
        align: "center",
      },
      {
        id: "date",
        prop: "date",
        header: "التاريخ",
        name: "Date",
        width: 115,
        size: 115,
        readonly: true,
      },
      {
        id: "reference",
        prop: "reference",
        header: "رقم المرجع",
        name: "Reference",
        width: 125,
        size: 125,
        readonly: true,
        cellClass: "font-mono font-bold",
      },
      {
        id: "description",
        prop: "description",
        header: "البيان والشرح",
        name: "Description",
        width: 260,
        size: 260,
        readonly: true,
      },
      {
        id: "debit",
        prop: "debit",
        header: "مدين (Debit)",
        name: "Debit",
        width: 130,
        size: 130,
        readonly: true,
        align: "end",
        cellClass: "font-mono text-emerald-600 font-semibold",
      },
      {
        id: "credit",
        prop: "credit",
        header: "دائن (Credit)",
        name: "Credit",
        width: 130,
        size: 130,
        readonly: true,
        align: "end",
        cellClass: "font-mono text-rose-600 font-semibold",
      },
      {
        id: "balance",
        prop: "balance",
        header: "الرصيد التراكمي",
        name: "Running balance",
        width: 145,
        size: 145,
        readonly: true,
        align: "end",
        cellClass: "font-mono font-bold",
      },
    ];
  }
  const currencyColumns = visibleCurrencies.flatMap((currency) => [
    {
      id: `${currency.code}_debit`,
      prop: `${currency.code}_debit`,
      header: `${currency.name} - مدين`,
      name: `${currency.name} - Debit`,
      width: 125,
      size: 125,
      readonly: true,
      align: "end",
      cellClass: "font-mono text-emerald-600 font-semibold",
    },
    {
      id: `${currency.code}_credit`,
      prop: `${currency.code}_credit`,
      header: `${currency.name} - دائن`,
      name: `${currency.name} - Credit`,
      width: 125,
      size: 125,
      readonly: true,
      align: "end",
      cellClass: "font-mono text-rose-600 font-semibold",
    },
  ]);
  return [
    {
      id: "__idx",
      prop: "__idx",
      header: "#",
      name: "#",
      width: 48,
      size: 48,
      readonly: true,
      align: "center",
    },
    {
      id: "account_name",
      prop: "account_name",
      header: m.common_ledger_col_name(),
      name: m.common_ledger_col_name(),
      width: 220,
      size: 220,
      readonly: true,
    },
    {
      id: "account_code",
      prop: "account_code",
      header: m.common_ledger_col_code(),
      name: m.common_ledger_col_code(),
      width: 105,
      size: 105,
      readonly: true,
      cellClass: "font-mono font-bold",
    },
    ...currencyColumns,
    {
      id: "account_type",
      prop: "account_type",
      header: "النوع",
      name: "Type",
      width: 110,
      size: 110,
      readonly: true,
    },
  ];
});

function selectAccount(account: any) {
  selectedAccountId = account.id;
  activeView = "single";
}

function handleExport() {
  const filename =
    activeView === "single"
      ? `Account_Statement_${selectedAccountItem?.label || selectedAccountId}`
      : `Chart_Of_Accounts_Balances_${orgId}`;

  const exportCols =
    activeView === "single"
      ? [
          { header: "#", accessor: (r: any) => r.__idx },
          { header: "التاريخ", accessor: (r: any) => r.date },
          { header: "رقم المرجع", accessor: (r: any) => r.reference },
          { header: "البيان", accessor: (r: any) => r.description },
          { header: "مدين", accessor: (r: any) => r.debit },
          { header: "دائن", accessor: (r: any) => r.credit },
          { header: "الرصيد", accessor: (r: any) => r.balance },
        ]
      : [
          { header: "#", accessor: (r: any) => r.__idx },
          { header: "اسم الحساب", accessor: (r: any) => r.account_name },
          { header: "رمز الحساب", accessor: (r: any) => r.account_code },
          { header: "نوع الحساب", accessor: (r: any) => r.account_type },
        ];

  exportToCsv(filename, source, exportCols);
}

function exitPage() {
  goto(`/${orgId}/finance`);
}

const rtl = $derived(getLocale() === "ar");
</script>

<div class="flex h-full min-h-0 w-full flex-col overflow-hidden text-foreground bg-background p-4 sm:p-6 font-sans">
  <div class="flex min-h-0 flex-1 flex-col gap-4 overflow-hidden lg:flex-row">
    <!-- Sidebar: Filter & Account Tree Card -->
    <Card class="flex w-full shrink-0 flex-col overflow-hidden lg:w-72">
      <CardHeader class="py-3 px-4">
        <div class="flex items-center gap-2">
          <div class="w-6 h-6 rounded-md bg-primary/10 flex items-center justify-center text-primary shrink-0">
            <BuildingLibrary class="w-3.5 h-3.5" />
          </div>
          <div class="flex flex-col">
            <CardTitle class="text-xs font-bold uppercase tracking-wider">
              {m.finance_ledger_chart_of_accounts()}
            </CardTitle>
            <CardDescription class="text-[10px]">
              تصفية الشجرة والفترات المحاسبية
            </CardDescription>
          </div>
        </div>
      </CardHeader>

      <CardContent class="p-3 flex-1 overflow-y-auto space-y-3">
        <AccountTree categories={treeCategories} bind:selectedIds={selectedIds} />

        <div class="flex flex-col gap-2.5 border-t border-border/70 pt-3">
          <div class="flex flex-col gap-1">
            <CurrencySelector
              title={m.common_ledger_currency()}
              description=""
              compact
              includeAll
              allLabel={m.common_ledger_all_currencies()}
              currentCurrency={selectedCurrency}
              onchange={(currency) => (selectedCurrency = currency)}
            />
          </div>

          <div class="grid grid-cols-2 gap-2">
            <label class="flex flex-col gap-1 text-[9px] font-bold text-muted-foreground">
              الفترة
              <select
                bind:value={period}
                class="h-8 rounded-md border border-border bg-card px-2 text-xs text-foreground focus:ring-1 focus:ring-ring outline-none"
              >
                <option value="all">كل الفترات</option>
                <option value="day">اليوم</option>
                <option value="month">هذا الشهر</option>
                <option value="year">هذا العام</option>
              </select>
            </label>

            <label class="flex flex-col gap-1 text-[9px] font-bold text-muted-foreground">
              حتى تاريخ
              <Input
                type="date"
                bind:value={asOfDate}
                class="h-8 px-2 text-xs"
              />
            </label>
          </div>

          <label class="flex items-center justify-between gap-2 text-[10px] text-muted-foreground cursor-pointer select-none">
            <span>إظهار الحسابات النشطة فقط</span>
            <input
              type="checkbox"
              checked={movementFilter === "active"}
              onchange={(event) =>
                (movementFilter = (event.currentTarget as HTMLInputElement).checked
                  ? "active"
                  : "all")}
              class="accent-primary h-3.5 w-3.5 rounded border-border"
            />
          </label>

          <select
            bind:value={balanceFilter}
            class="h-8 rounded-md border border-border bg-card px-2 text-xs text-foreground focus:ring-1 focus:ring-ring outline-none"
          >
            <option value="all">جميع الأرصدة</option>
            <option value="debit">الأرصدة المدينة فقط</option>
            <option value="credit">الأرصدة الدائنة فقط</option>
          </select>
        </div>
      </CardContent>

      <CardFooter class="border-t border-border/50 py-2 px-3">
        <Button
          variant="ghost"
          size="sm"
          class="w-full justify-start text-xs text-muted-foreground hover:text-foreground"
          onclick={exitPage}
        >
          {#if getLocale() === "ar"}
            <ArrowRight size={14} class="me-2" />
          {:else}
            <ArrowLeft size={14} class="me-2" />
          {/if}
          العودة للمركز المالي
        </Button>
      </CardFooter>
    </Card>

    <!-- Main Content: Statement Grid Card -->
    <Card class="flex min-h-0 min-w-0 flex-1 flex-col overflow-hidden">
      <CardHeader class="py-3 px-4">
        <div class="flex flex-wrap items-center gap-3">
          <div class="flex items-center gap-1 rounded-lg border border-border bg-muted/40 p-1">
            <button
              type="button"
              class="rounded-md px-3 py-1 text-xs font-semibold transition-colors cursor-pointer {activeView === 'all' ? 'bg-primary text-primary-foreground shadow-sm' : 'text-muted-foreground hover:text-foreground'}"
              onclick={() => (activeView = "all")}
            >
              جميع الحسابات
            </button>
            <button
              type="button"
              class="rounded-md px-3 py-1 text-xs font-semibold transition-colors cursor-pointer {activeView === 'single' ? 'bg-primary text-primary-foreground shadow-sm' : 'text-muted-foreground hover:text-foreground'}"
              onclick={() => (activeView = "single")}
            >
              كشف حساب تفصيلي
            </button>
          </div>

          {#if activeView === "single"}
            <ComboboxDropdown
              class="w-64"
              placeholder="اختر الحساب المحاسبي..."
              searchPlaceholder="بحث باسم أو رمز الحساب..."
              items={accountItems}
              selectedItem={selectedAccountItem}
              onSelect={(item) =>
                selectAccount(
                  accounts.find((account) => account.id === item.value),
                )}
            />
          {/if}
        </div>

        <CardAction>
          <div class="flex items-center gap-2">
            <div class="relative w-48 sm:w-60">
              <Search class="absolute start-2.5 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-muted-foreground pointer-events-none" />
              <Input
                type="search"
                placeholder="بحث في السجلات..."
                bind:value={searchQuery}
                class="h-8 ps-8 text-xs w-full bg-card"
              />
            </div>

            <Button
              variant="outline"
              size="sm"
              onclick={handleExport}
              disabled={source.length === 0}
              class="h-8 text-xs font-semibold gap-1.5"
            >
              <DocumentChartBar class="w-3.5 h-3.5" />
              تصدير CSV
            </Button>

            <Button
              variant="ghost"
              size="icon-sm"
              onclick={loadData}
              class="h-8 w-8 text-muted-foreground"
            >
              <Refresh size={14} class={isLoading ? "animate-spin" : ""} />
            </Button>
          </div>
        </CardAction>
      </CardHeader>

      <CardContent class="p-0 flex-1 min-h-0 flex flex-col overflow-hidden">
        {#if error}
          <div class="flex flex-1 flex-col items-center justify-center gap-3 text-center p-6">
            <p class="text-sm font-semibold text-destructive">{m.finance_error_loading()}</p>
            <p class="max-w-md text-xs text-muted-foreground">{error}</p>
            <Button variant="outline" size="sm" onclick={loadData}>{m.common_retry()}</Button>
          </div>
        {:else if isLoading}
          <div class="flex flex-col gap-2 p-4">
            {#each Array(8) as _}
              <Skeleton class="h-9 w-full" />
            {/each}
          </div>
        {:else}
          <div class="flex-1 min-h-0 w-full h-full">
            <OrbisGrid
              {columns}
              {source}
              search={searchQuery}
              clientQuery
              {rtl}
              readonly
              emptyTitle="لا توجد بيانات متاحة"
              emptyDescription="لم يتم العثور على حركات أو أرصدة مطابقة لمعايير البحث المحددة"
            />
          </div>
        {/if}
      </CardContent>
    </Card>
  </div>
</div>
