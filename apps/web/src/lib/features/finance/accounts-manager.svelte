<!--
  @component AccountsManager
  Professional ERP Chart of Accounts Master (دليل الحسابات).
  Features:
  - High-performance Split-View matching classic ERP systems (TreeView on the right, Master Detail & Form on the left)
  - Automatic Parent Account inheritance and Smart Code suggestion for sub-accounts
  - Direct account creation, editing, closing statement tagging, and live balance monitoring
  - Instant transition between Tree/Form Split View and Full Ledger Grid
-->
<script lang="ts">
import { effectiveContext } from "$lib/stores";
import CashierCashAccounts from "./components/cashier-cash-accounts.svelte";
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { useQueryClient } from "@tanstack/svelte-query";
import { api, extractErrorMessage, queryKeys, type AccountDto } from "$lib/api";
import { orgContext } from "$lib/stores/org-context.svelte";
import { Skeleton } from "@orbis/ui/skeleton";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Badge } from "@orbis/ui/badge";
import { Card, CardHeader, CardTitle, CardContent } from "@orbis/ui/card";
import { toast } from "@orbis/ui";
import { confirmStore } from "$lib/stores";
import {
  Grid,
  Plus,
  BuildingLibrary,
  BuildingOffice,
  Refresh,
  Download,
  Search,
  XMark,
  CheckCircle,
  DocumentChartBar,
  ArrowRightTag,
  Trash,
} from "@orbis/icons";
import { getLocale, localizeHref } from "$lib/paraglide/runtime";
import { formatCurrency, formatDate, isAr, l } from "$lib/core/i18n";
import {
  OrbisGrid,
  type GridActionEvent,
  type OrbisColumnDef,
} from "@orbis/svelte-grid";
import { financeRecordColumn } from "./record-presentation";
import AccountTree, {
  type AccountNode,
} from "$lib/features/finance/components/account-tree.svelte";
import CurrencySelector from "$lib/shared/components/currency-selector.svelte";
import { FilterTrigger, type UniversalFilter } from "@orbis/ui/filter-builder";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui/select";
import { accountFilterFields } from "$lib/core/filter-configs";
import {
  createAccountsQuery,
  createCurrenciesQuery,
} from "$lib/features/finance/queries";

let { orgId }: { orgId: string } = $props();

// ── State ──────────────────────────────────────────────────────────────
const queryClient = useQueryClient();
const accountsQuery = createAccountsQuery(() => orgId);
const currenciesQuery = createCurrenciesQuery(() => orgId);
const allAccounts = $derived(accountsQuery.data ?? []);
const currencies = $derived(currenciesQuery.data ?? []);
const baseCurrency = $derived(orgContext.baseCurrency || "");
const isLoading = $derived(
  accountsQuery.isLoading || currenciesQuery.isLoading,
);
const isRefreshing = $derived(
  accountsQuery.isFetching || currenciesQuery.isFetching,
);
let isSubmitting = $state(false);
let formError = $state("");
let viewMode = $state<"tree" | "grid">("tree");
let searchQuery = $state("");

// ── Master / Selected Account Form State ──────────────────────────────
let selectedAccountId = $state<string | null>(null);
const selectedAccount = $derived(
  allAccounts.find((account) => account.id === selectedAccountId) ??
    allAccounts[0] ??
    null,
);
let isAddingNew = $state(false);

// Form Fields
let formParentId = $state<string | null>(null);
let formCode = $state("");
let formNameAr = $state("");
let formNameEn = $state("");
let formAccountType = $state("asset"); // asset | liability | equity | revenue | expense
let formSubType = $state("general");
const accountCategories = [
  { id: "asset", label: m.finance_ui_account_type_asset_numbered },
  { id: "liability", label: m.finance_ui_account_type_liability_numbered },
  { id: "equity", label: m.finance_ui_account_type_equity_numbered },
  { id: "revenue", label: m.finance_ui_account_type_revenue_numbered },
  { id: "expense", label: m.finance_ui_account_type_expense_numbered },
];
const accountSubtypes = [
  { id: "general", label: m.finance_ui_account_type_general },
  { id: "cash", category: "asset", label: m.finance_ui_cash_treasury },
  { id: "bank", category: "asset", label: m.finance_ui_account_type_bank },
  {
    id: "gateway_clearing",
    category: "asset",
    label: m.finance_ui_account_type_gateway_clearing,
  },
  {
    id: "customer",
    category: "asset",
    label: m.finance_ui_account_type_customer_student,
  },
  {
    id: "supplier",
    category: "liability",
    label: m.finance_ui_account_type_supplier,
  },
  { id: "employee", label: m.finance_ui_account_type_employee },
  { id: "owner", category: "equity", label: m.finance_ui_account_type_owner },
  {
    id: "gateway_fee",
    category: "expense",
    label: m.finance_ui_account_type_gateway_fee,
  },
];
const availableSubtypes = $derived(
  accountSubtypes.filter(
    (item) => !item.category || item.category === formAccountType,
  ),
);

function setAccountType(value: string) {
  formAccountType = value;
  if (!availableSubtypes.some((item) => item.id === formSubType))
    formSubType = "general";
}
let formCurrency = $state(orgContext.baseCurrency || "");

let activeFilter = $state<UniversalFilter>({
  logic: "AND",
  conditions: [],
  groups: [],
  rules: [],
} as any);

const rtl = $derived(getLocale() === "ar");
const parentAccountIds = $derived(
  new Set(
    allAccounts.flatMap((account) =>
      account.parent_id ? [account.parent_id] : [],
    ),
  ),
);

async function loadData() {
  const [accountsResult, currenciesResult] = await Promise.all([
    accountsQuery.refetch(),
    currenciesQuery.refetch(),
  ]);
  if (accountsResult.isError || currenciesResult.isError) {
    console.error(
      "Error refreshing accounts:",
      accountsResult.error ?? currenciesResult.error,
    );
    toast.error(m.finance_ui_accounts_load_failed());
  }
}

$effect(() => {
  if (baseCurrency && !formCurrency) formCurrency = baseCurrency;
});

$effect(() => {
  if (
    allAccounts.length > 0 &&
    !isAddingNew &&
    (!selectedAccountId ||
      !allAccounts.some((account) => account.id === selectedAccountId))
  ) {
    handleSelectAccount(allAccounts[0]);
  }
});

// Build authentic hierarchical tree directly from backend accounts (Root accounts -> Child accounts)
const accountsByParent = $derived.by(() => {
  const index = new Map<string | null, AccountDto[]>();
  for (const account of allAccounts) {
    const parent = account.parent_id || null;
    const siblings = index.get(parent);
    if (siblings) siblings.push(account);
    else index.set(parent, [account]);
  }
  return index;
});

const accountTree = $derived.by(() => {
  const codeOrder = new Intl.Collator(undefined, { numeric: true });
  function buildSubTree(parentId: string | null): AccountNode[] {
    return [...(accountsByParent.get(parentId) ?? [])]
      .sort((a, b) => codeOrder.compare(a.code, b.code))
      .map((node) => ({
        ...node,
        name: l(node.name_ar, node.name) || "?",
        children: buildSubTree(node.id),
      }));
  }
  return buildSubTree(null);
});

// ── Smart Code Suggestion ──────────────────────────────────────────────
function suggestNextChildCode(parent: AccountNode | null): string {
  if (!parent)
    return String(allAccounts.filter((a) => !a.parent_id).length + 1);

  const existingChildren = allAccounts.filter((a) => a.parent_id === parent.id);
  if (existingChildren.length === 0) {
    // If parent is "1", first child is "11". If parent is "11", first child is "1101"
    return parent.code.length === 1 ? `${parent.code}1` : `${parent.code}01`;
  }

  // Find max numeric code among children
  const childNumbers = existingChildren
    .map((c) => Number(c.code))
    .filter((n) => !isNaN(n));

  if (childNumbers.length > 0) {
    const max = Math.max(...childNumbers);
    return String(max + 1);
  }

  return `${parent.code}${existingChildren.length + 1}`;
}

// ── Selection & Form Preparation ───────────────────────────────────────
function handleSelectAccount(account: AccountNode) {
  formError = "";
  selectedAccountId = account.id;
  isAddingNew = false;

  formParentId = account.parent_id ?? null;
  formCode = account.code || "";
  formNameAr = account.name_ar || (isAr() ? account.name : "") || "";
  formNameEn = account.name;
  formAccountType = account.account_type.toLowerCase();
  formSubType = account.sub_type || "general";
  formCurrency = account.currency || baseCurrency;
}

function startAddSubAccount(
  parentAccount: AccountNode | null = selectedAccount,
) {
  if (isSubmitting) return;
  formError = "";
  isAddingNew = true;
  formParentId = parentAccount ? parentAccount.id : null;
  formCode = suggestNextChildCode(parentAccount);
  formNameAr = "";
  formNameEn = "";
  formSubType = "general";
  formAccountType = parentAccount?.account_type.toLowerCase() ?? "asset";
  formCurrency = parentAccount?.currency || baseCurrency;
}

const parentAccountNode = $derived(
  allAccounts.find((a) => a.id === formParentId),
);
const selectedAccountHasChildren = $derived(
  !!selectedAccount && parentAccountIds.has(selectedAccount.id),
);
const isFinancePurposeAccount = $derived(
  !!selectedAccount &&
    (selectedAccount.is_finance_default ||
      selectedAccount.is_system_account ||
      selectedAccount.is_control_account),
);
const isAccountStructureLocked = $derived(
  !isAddingNew && (isFinancePurposeAccount || selectedAccountHasChildren),
);

// Determine Closing Statement based on Account Type
const closingStatementLabel = $derived.by(() => {
  const type = formAccountType.toLowerCase();
  if (type === "asset" || type === "liability" || type === "equity") {
    return m.finance_ui_balance_sheet();
  }
  return m.finance_ui_income_statement();
});

// ── Account Save / Create Action ───────────────────────────────────────
async function invalidateAccounts() {
  await queryClient.invalidateQueries({
    queryKey: queryKeys.finance.accounts(orgId),
  });
}

async function handleSaveAccount() {
  if (isSubmitting) return;
  formError = "";
  if (!formNameAr.trim() && !formNameEn.trim()) {
    formError = m.finance_ui_account_name_required();
    return;
  }
  if (!formCode.trim()) {
    formError = m.finance_ui_account_code_required();
    return;
  }

  isSubmitting = true;
  try {
    const payload = {
      code: formCode.trim(),
      name: formNameEn.trim() || formNameAr.trim(),
      name_ar: formNameAr.trim() || formNameEn.trim(),
      account_type: formAccountType,
      sub_type: formSubType !== "general" ? formSubType : undefined,
      currency: formCurrency,
      parent_id: formParentId || undefined,
    };

    let result: AccountDto | undefined;
    if (!isAddingNew && selectedAccount?.id) {
      result = await api.updateAccount(selectedAccount.id, payload);
      toast.success(m.finance_ui_account_update_success());
    } else {
      result = await api.createAccount(payload);
      toast.success(m.finance_ui_account_save_success());
    }
    await invalidateAccounts();

    if (result && result.id) {
      handleSelectAccount(result);
    }
  } catch (err) {
    formError = extractErrorMessage(err) || m.finance_ui_account_save_failed();
  } finally {
    isSubmitting = false;
  }
}

async function handleDeleteAccount() {
  const account = selectedAccount;
  if (!account?.id || isSubmitting) return;

  const confirmed = await confirmStore.prompt({
    title: m.finance_ui_delete_account(),
    description: m.finance_ui_delete_account_confirm({
      account: selectedAccount.name || selectedAccount.code,
    }),
    confirmText: m.finance_ui_delete_account(),
    cancelText: m.finance_ui_cancel(),
    variant: "destructive",
  });

  if (!confirmed) return;

  isSubmitting = true;
  try {
    await api.deleteAccount(account.id);
    toast.success(m.finance_ui_account_delete_success());
    await invalidateAccounts();
    selectedAccountId = null;
  } catch (err: any) {
    console.error("Failed to delete account:", err);
    toast.error(err?.message || m.finance_ui_account_delete_failed());
  } finally {
    isSubmitting = false;
  }
}

// ── Grid Data for Table View ───────────────────────────────────────────
const leafAccounts = $derived(
  allAccounts.filter((account) => !parentAccountIds.has(account.id)),
);

const treasurySource = $derived(
  leafAccounts.map((account, i) => {
    const bal = Number(
      (account.balances as Record<string, number | string> | null)?.[
        account.currency || baseCurrency
      ] || 0,
    );
    return {
      ...account,
      __idx: i + 1,
      name_display: l(account.name_ar, account.name) || "—",
      balance_formatted: formatCurrency(bal, account.currency || baseCurrency),
      currency_code: account.currency || baseCurrency,
      status: account.is_active ? "active" : "inactive",
      status_display: account.is_active
        ? m.finance_ui_active()
        : m.finance_ui_inactive(),
    };
  }),
);

const detailsColumn = $derived(financeRecordColumn());

function handleAccountRow(event: GridActionEvent) {
  if (
    isSubmitting ||
    (event.actionId !== "dblclick" && event.actionId !== "view")
  )
    return;
  const account = allAccounts.find((item) => item.id === event.row.id);
  if (!account) return;
  handleSelectAccount(account);
  viewMode = "tree";
}

const treasuryCols: OrbisColumnDef[] = [
  {
    id: "__idx",
    prop: "__idx",
    header: "#",
    name: "#",
    width: 48,
    size: 48,
    readonly: true,
    pin: "left",
    align: "center",
  },
  {
    id: "code",
    prop: "code",
    header: m.finance_ui_account_number(),
    name: m.finance_ui_account_number(),
    width: 110,
    size: 110,
    readonly: true,
    cellClass: "font-mono font-bold",
  },
  {
    id: "name_display",
    pin: "colPinStart",
    prop: "name_display",
    header: m.finance_ui_account_name(),
    name: m.finance_ui_account_name(),
    width: 220,
    size: 220,
    readonly: true,
    cellClass: "font-medium",
  },
  {
    id: "balance_formatted",
    prop: "balance_formatted",
    header: m.finance_ui_current_balance(),
    name: m.finance_ui_current_balance(),
    width: 140,
    size: 140,
    readonly: true,
    align: "end",
    cellClass: "font-mono font-bold text-primary",
  },
  {
    id: "currency_code",
    prop: "currency_code",
    header: m.finance_ui_currency(),
    name: m.finance_ui_currency(),
    width: 80,
    size: 80,
    readonly: true,
    align: "center",
  },
  {
    id: "status_display",
    prop: "status_display",
    header: m.finance_ui_status(),
    name: m.finance_ui_status(),
    width: 90,
    size: 90,
    type: "badge",
    align: "center",
    readonly: true,
  },
];

function exportAccountsCsv() {
  const rows = treasurySource.map((a) => [
    a.code,
    a.name_display,
    a.balance_formatted,
    a.currency_code,
    a.status_display,
  ]);
  const csv = [
    [
      m.finance_ui_account_code(),
      m.finance_ui_account_name(),
      m.finance_ui_balance(),
      m.finance_ui_currency(),
      m.finance_ui_status(),
    ],
    ...rows,
  ]
    .map((row) =>
      row.map((v) => `"${String(v ?? "").replaceAll('"', '""')}"`).join(","),
    )
    .join("\n");
  const url = URL.createObjectURL(
    new Blob(["\ufeff" + csv], { type: "text/csv;charset=utf-8" }),
  );
  const link = document.createElement("a");
  link.href = url;
  link.download = `chart-of-accounts-${orgId}.csv`;
  link.click();
  URL.revokeObjectURL(url);
}
</script>

<div class="flex h-full min-h-0 w-full flex-col overflow-hidden p-6 font-sans bg-background/50 text-start">
  <div class="flex min-h-0 w-full flex-1 flex-col gap-4">
    
    <!-- Top Action Toolbar & View Mode Switcher -->
    <header class="flex shrink-0 items-center justify-between gap-4">
      <div class="flex items-center gap-2">
        <div class="flex items-center gap-1 bg-muted/40 p-0.5 rounded-lg border border-border/40">
          <Button
            variant={viewMode === "tree" ? "default" : "ghost"}
            size="sm"
            class="h-7 text-xs font-semibold px-2.5 gap-1.5"
            onclick={() => viewMode = "tree"}
          >
            <BuildingOffice size={13} />
            <span>{m.finance_ui_accounts_tree()}</span>
          </Button>

          <Button
            variant={viewMode === "grid" ? "default" : "ghost"}
            size="sm"
            class="h-7 text-xs font-semibold px-2.5 gap-1.5"
            onclick={() => viewMode = "grid"}
          >
            <Grid size={13} />
            <span>{m.finance_ui_accounts_table()}</span>
          </Button>
        </div>

        <Button
          variant="secondary"
          size="sm"
          class="h-7 text-xs font-bold gap-1"
          onclick={() => startAddSubAccount(selectedAccount)}
        >
          <Plus size={13} />
          <span>{m.finance_ui_add_subaccount()}</span>
        </Button>
      </div>

      <div class="flex items-center gap-2">
        <Button variant="ghost" size="icon-sm" onclick={loadData} class="text-muted-foreground">
          {#if isRefreshing}<LoadingIndicator size={14} />{:else}<Refresh size={14} />{/if}
        </Button>

        <Button variant="ghost" size="icon-sm" onclick={exportAccountsCsv} class="text-muted-foreground">
          <Download size={14} />
        </Button>
      </div>
    </header>

    <!-- Content Workspace -->
    {#if isLoading}
      <div class="grid min-h-0 flex-1 grid-cols-1 gap-4 lg:grid-cols-12" aria-hidden="true">
        <div class="flex min-h-0 flex-col gap-3 rounded-xl border border-border/40 bg-card p-3 lg:col-span-5">
          <div class="flex items-center justify-between border-b border-border/40 pb-3">
            <Skeleton class="h-8 w-40 rounded-md" />
            <Skeleton class="h-8 w-20 rounded-md" />
          </div>
          <div class="flex flex-col gap-4 p-1">
            {#each { length: 10 } as _, index (index)}
              <div class="flex items-center gap-2">
                <Skeleton class="h-3.5 w-3.5 rounded-sm" />
                <Skeleton class="h-3.5 w-12 rounded" />
                <Skeleton class="h-3.5 flex-1 rounded" />
              </div>
            {/each}
          </div>
        </div>

        <div class="flex min-h-0 flex-col rounded-xl border border-border/50 bg-card lg:col-span-7">
          <div class="flex h-12 items-center justify-between border-b border-border/40 px-4">
            <Skeleton class="h-4 w-36" />
            <Skeleton class="h-5 w-16 rounded" />
          </div>
          <div class="grid flex-1 content-start grid-cols-1 gap-5 p-4 sm:grid-cols-2">
            <div class="flex flex-col gap-2 sm:col-span-2">
              <Skeleton class="h-3.5 w-24" />
              <Skeleton class="h-10 w-full rounded-md" />
            </div>
            {#each { length: 6 } as _, index (index)}
              <div class="flex flex-col gap-2">
                <Skeleton class="h-3.5 w-24" />
                <Skeleton class="h-9 w-full rounded-md" />
              </div>
            {/each}
          </div>
          <div class="flex justify-end gap-2 border-t border-border/40 bg-muted/20 p-3">
            <Skeleton class="h-8 w-20 rounded-md" />
            <Skeleton class="h-8 w-24 rounded-md" />
          </div>
        </div>
      </div>
    {:else if accountsQuery.isError && !accountsQuery.data}
      <div class="flex min-h-0 flex-1 items-center justify-center rounded-xl border border-border/50 bg-card">
        <div class="flex flex-col items-center gap-3 text-center">
          <p class="text-sm text-muted-foreground">{m.finance_ui_accounts_load_failed()}</p>
          <Button variant="outline" size="sm" onclick={loadData}>
            {m.common_retry()}
          </Button>
        </div>
      </div>
    {:else}
      {#if accountsQuery.isError || currenciesQuery.isError}
        <div class="flex shrink-0 items-center justify-between gap-3 rounded-lg border border-destructive/20 bg-destructive/5 px-3 py-2 text-xs text-destructive" role="alert">
          <span>{m.finance_ui_accounts_load_failed()}</span>
          <Button variant="ghost" size="sm" class="h-7" onclick={loadData}>
            {m.common_retry()}
          </Button>
        </div>
      {/if}
      <div class="flex min-h-0 flex-1 overflow-hidden">
      {#if viewMode === "tree"}
        <!-- ══════════════════════════════════════════════════════════════ -->
        <!-- Responsive Split-View Layout (Tree first, Form second)       -->
        <!-- RTL: Tree on Right, Form on Left                             -->
        <!-- LTR: Tree on Left, Form on Right                             -->
        <!-- ══════════════════════════════════════════════════════════════ -->
        <div class="grid grid-cols-1 lg:grid-cols-12 gap-4 w-full h-full min-h-0">
          
          <!-- Compact TreeView (5 cols) -->
          <div class="lg:col-span-5 flex flex-col h-full min-h-0 overflow-hidden rounded-xl border border-border/40 bg-card p-2 shadow-sm">
            <AccountTree
              accounts={accountTree}
              {orgId}
              onSelectAccount={(node) => { if (!isSubmitting) handleSelectAccount(node); }}
            />
          </div>

          <!-- Account Details & Editing / Creation Form (7 cols) -->
          <form class="lg:col-span-7 flex min-h-0 min-w-0 flex-col" aria-busy={isSubmitting} onsubmit={(event) => { event.preventDefault(); void handleSaveAccount(); }}>
            <Card class="border border-border/50 bg-card shadow-sm h-full flex flex-col justify-between">
              <CardHeader class="py-3 px-4 border-b border-border/40 shrink-0">
                <div class="flex items-center justify-between">
                  <div class="flex items-center gap-2">
                    <span class="h-2 w-2 rounded-full {isAddingNew ? 'bg-emerald-500 animate-pulse' : 'bg-primary'}"></span>
                    <CardTitle class="text-xs font-bold uppercase tracking-wider text-foreground">
                      {isAddingNew ? m.finance_ui_add_account_heading() : m.finance_ui_account_details_heading({ account: l(formNameAr, formNameEn) || formCode })}
                    </CardTitle>
                  </div>

                  {#if selectedAccount && !isAddingNew}
                    <Badge variant="outline" class="font-mono text-xs font-bold">
                      {selectedAccount.code}
                    </Badge>
                  {/if}
                </div>
              </CardHeader>

              <CardContent class="min-h-0 flex-1 overflow-y-auto p-4">
              <fieldset class="flex min-w-0 flex-col gap-4" disabled={isSubmitting}>
                {#if isFinancePurposeAccount && !isAddingNew}
                  <div class="rounded-md border border-amber-500/30 bg-amber-500/5 px-3 py-2 text-xs text-amber-800 dark:text-amber-200">
                    {m.finance_ui_fixed_finance_account_notice()}
                  </div>
                {/if}

                <h3 class="text-sm font-semibold">{m.finance_ui_account_identity()}</h3>
                <div class="flex flex-col gap-1">
                  <Label class="text-xs font-semibold text-foreground">{m.finance_ui_parent_account()}</Label>
                  <div class="p-2 rounded-md bg-muted/30 border border-border/50 flex items-center justify-between text-xs">
                    <div class="flex items-center gap-2">
                      <span class="font-mono font-bold text-muted-foreground">
                        {parentAccountNode ? parentAccountNode.code : "—"}
                      </span>
                      <span class="font-medium text-foreground">
                        {parentAccountNode ? (l(parentAccountNode.name_ar, parentAccountNode.name) || parentAccountNode.code) : m.finance_ui_root_account()}
                      </span>
                    </div>

                    <Button
                      variant="ghost"
                      size="sm"
                      class="h-6 text-[11px] px-2"
                      disabled={isAccountStructureLocked}
                      onclick={() => { formParentId = null; formCode = suggestNextChildCode(null); }}
                    >
                      {m.finance_ui_set_root_account()}
                    </Button>
                  </div>
                </div>

                <!-- Row 2: اسم الحساب (عربي + إنجليزي) -->
                <div class="grid min-w-0 grid-cols-1 gap-4 sm:grid-cols-2">
                  <div>
                    <Label class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_account_name_arabic()}</Label>
                    <Input
                      type="text"
                      bind:value={formNameAr}
                      placeholder={m.finance_ui_account_name_ar_placeholder()}
                      class="h-8 text-xs bg-background border-border/60"
                    />
                  </div>

                  <div>
                    <Label class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_account_name_english()}</Label>
                    <Input
                      type="text"
                      bind:value={formNameEn}
                      placeholder={m.finance_ui_account_name_en_placeholder()}
                      class="h-8 text-xs font-mono bg-background border-border/60"
                    />
                  </div>
                </div>

                <!-- Row 3: رقم الحساب + نوع المستوى (رئيسي / فرعي) -->
                <div class="grid min-w-0 grid-cols-1 gap-4 sm:grid-cols-2">
                  <div>
                    <Label class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_account_code()}</Label>
                    <Input
                      type="text"
                      bind:value={formCode}
                      placeholder="1101"
                      class="h-8 text-xs font-mono font-bold bg-background border-border/60"
                      disabled={isAccountStructureLocked}
                    />
                  </div>

                  <div>
                    <Label class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_account_level()}</Label>
                    <p class="flex h-8 items-center rounded-md border border-border/60 px-3 text-xs text-muted-foreground">
                      {formParentId ? m.finance_ui_account_level_sub() : m.finance_ui_account_level_primary()}
                    </p>
                  </div>
                </div>

                <h3 class="border-t border-border pt-3 text-sm font-semibold">{m.finance_ui_account_classification()}</h3>
                <div class="grid min-w-0 grid-cols-1 gap-4 sm:grid-cols-2 xl:grid-cols-3">
                  <div>
                    <Label class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_account_category()}</Label>
                    <Select
                      type="single"
                      value={formAccountType}
                      disabled={isAccountStructureLocked || Boolean(formParentId)}
                      onValueChange={(value) => {
                        if (value) setAccountType(value);
                      }}
                    >
                      <SelectTrigger class="h-8 w-full bg-background text-xs">
                        {accountCategories.find((category) => category.id === formAccountType)?.label() ?? formAccountType}
                      </SelectTrigger>
                      <SelectContent>
                        {#each accountCategories as category (category.id)}
                          <SelectItem value={category.id}>{category.label()}</SelectItem>
                        {/each}
                      </SelectContent>
                    </Select>
                  </div>

                  <div>
                    <Label class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_account_subtype()}</Label>
                    <Select
                      type="single"
                      value={formSubType}
                      disabled={isAccountStructureLocked}
                      onValueChange={(value) => {
                        if (value) formSubType = value;
                      }}
                    >
                      <SelectTrigger class="h-8 w-full bg-background text-xs">
                        {accountSubtypes.find((item) => item.id === formSubType)?.label() ?? formSubType}
                      </SelectTrigger>
                      <SelectContent>
                        {#each availableSubtypes as item (item.id)}
                          <SelectItem value={item.id}>{item.label()}</SelectItem>
                        {/each}
                      </SelectContent>
                    </Select>
                  </div>

                  <div>
                    <CurrencySelector
                      compact
                      title={m.finance_ui_account_currency()}
                      description=""
                      bind:currentCurrency={formCurrency}
                      currencyOptions={currencies}
                      disabled={isAccountStructureLocked || isLoading || currencies.length === 0}
                    />
                  </div>
                </div>

                <!-- Row 5: الحساب الختامي (Closing Statement) -->
                <div class="p-2.5 rounded-lg border border-border/50 bg-muted/10 flex items-center justify-between text-xs">
                  <span class="font-semibold text-muted-foreground">{m.finance_ui_closing_statement()}</span>
                  <span class="font-bold text-foreground">{closingStatementLabel}</span>
                </div>

                <!-- Current Balance Display (if existing account) -->
                {#if selectedAccount && !isAddingNew}
                  {@const curBal = Number((selectedAccount.balances as Record<string, number | string> | null)?.[formCurrency] || 0)}
                  <div class="p-3 rounded-lg border border-primary/20 bg-primary/5 flex items-center justify-between">
                    <span class="text-xs font-bold text-foreground">{m.finance_ui_current_account_balance()}</span>
                    <span class="text-sm font-mono font-extrabold text-primary">
                      {formatCurrency(curBal, formCurrency)}
                    </span>
                  </div>
                {/if}
              </fieldset>
              </CardContent>

              {#if formError}<p class="px-4 py-2 text-sm text-destructive" role="alert">{formError}</p>{/if}
              <!-- Bottom Action Toolbar -->
              <div class="py-3 px-4 border-t border-border/40 bg-muted/20 flex items-center justify-between gap-2 shrink-0">
                <div class="flex items-center gap-2">
                  {#if selectedAccount && !isAddingNew && orgId}
                    <a
                      href={localizeHref(`/${orgId}/finance/statement`)}
                      class="inline-flex items-center gap-1.5 h-8 px-3 text-xs font-semibold rounded-md border border-border bg-card hover:bg-muted text-foreground"
                    >
                      <DocumentChartBar size={13} />
                      <span>{m.finance_ui_account_statement()}</span>
                    </a>
                  {/if}
                </div>

                <div class="flex items-center gap-2">
                  {#if !isAddingNew && selectedAccount?.id}
                    <Button
                      variant="destructive"
                      size="sm"
                      class="h-8 text-xs font-semibold gap-1.5"
                      disabled={isSubmitting || isFinancePurposeAccount || selectedAccountHasChildren}
                      type="button"
                      onclick={handleDeleteAccount}
                    >
                      <Trash size={13} />
                      <span>{m.finance_ui_delete_account()}</span>
                    </Button>
                  {/if}

                  <Button
                    variant="ghost"
                    size="sm"
                    class="h-8 text-xs"
                    type="button"
                    disabled={isSubmitting}
                    onclick={() => {
                      if (selectedAccount) handleSelectAccount(selectedAccount);
                      else isAddingNew = false;
                    }}
                  >
                    {m.finance_ui_cancel()}
                  </Button>

                  <Button
                    variant="default"
                    size="sm"
                    class="h-8 text-xs font-bold px-4"
                    disabled={isSubmitting}
                    type="submit"
                  >
                    {isSubmitting ? m.finance_ui_saving() : (isAddingNew ? m.finance_ui_add_account() : m.finance_ui_save_changes())}
                  </Button>
                </div>
              </div>
            </Card>
          </form>
        </div>

      {:else}
        <!-- ══════════════════════════════════════════════════════════════ -->
        <!-- Full Ledger Grid View                                        -->
        <!-- ══════════════════════════════════════════════════════════════ -->
        <div class="flex flex-col h-full w-full gap-3">
          <div class="flex items-center justify-between gap-2 shrink-0">
            <div class="relative flex-1 max-w-xs">
              <Search
                size={14}
                class="absolute inset-s-2.5 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none"
              />
              <Input
                type="text"
                placeholder={m.finance_ui_account_search_placeholder()}
                bind:value={searchQuery}
                class="h-8 ps-8 text-xs bg-card border-border/60"
              />
            </div>
            <div class="flex items-center gap-1.5">
              <FilterTrigger
                bind:filter={activeFilter}
                fields={accountFilterFields}
              />
              <Button variant="ghost" size="icon-sm" onclick={loadData} class="text-muted-foreground">
                {#if isRefreshing}<LoadingIndicator size={14} />{:else}<Refresh size={14} />{/if}
              </Button>
            </div>
          </div>

          {#if effectiveContext.hasPermission("organization:write")}
            <CashierCashAccounts {orgId} accounts={accountsQuery.data ?? []} />
          {/if}
          <div class="flex-1 min-h-0">
            <OrbisGrid
              columns={[...treasuryCols, detailsColumn]}
              onaction={handleAccountRow}
              source={treasurySource}
              loading={isLoading}
              clientQuery
              toolbar={false}
              search={searchQuery}
              filter={activeFilter}
              {rtl}
              readonly
              emptyTitle={m.finance_no_ledgers()}
            />
          </div>
        </div>
      {/if}
      </div>
    {/if}
  </div>
</div>
