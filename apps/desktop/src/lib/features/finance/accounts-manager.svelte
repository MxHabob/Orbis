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
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api } from "$lib/api";
import type { CurrencyDto } from "$lib/api/types";
import { orgContext } from "$lib/stores/org-context.svelte";
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
import { fade, slide } from "svelte/transition";
import { OrbisGrid, type OrbisColumnDef } from "@orbis/svelte-grid";
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

let { orgId }: { orgId: string } = $props();

// ── State ──────────────────────────────────────────────────────────────
let allAccounts = $state<AccountNode[]>([]);
let currencies = $state<CurrencyDto[]>([]);
let baseCurrency = $state(orgContext.baseCurrency || "");
let isLoading = $state(true);
let isSubmitting = $state(false);
let viewMode = $state<"tree" | "grid">("tree");
let searchQuery = $state("");

// ── Master / Selected Account Form State ──────────────────────────────
let selectedAccount = $state<AccountNode | null>(null);
let isAddingNew = $state(false);

// Form Fields
let formParentId = $state<string | null>(null);
let formCode = $state("");
let formNameAr = $state("");
let formNameEn = $state("");
let formAccountType = $state("asset"); // asset | liability | equity | revenue | expense
let formSubType = $state<string>("general"); // general | cash | bank | supplier | customer | employee
let formAccountLevel = $state<"primary" | "sub">("sub"); // رئيسي أو فرعي
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

onMount(async () => {
  if (orgId) await loadData();
});

async function loadData() {
  isLoading = true;
  try {
    const [accs, currs] = await Promise.all([
      api.listAccounts(),
      api.listCurrencies(),
    ]);

    allAccounts = accs;
    currencies = currs;
    if (orgContext.baseCurrency) {
      baseCurrency = orgContext.baseCurrency;
      if (!formCurrency) formCurrency = orgContext.baseCurrency;
    }

    // Auto-select first root account if none selected
    if (!selectedAccount && allAccounts.length > 0) {
      handleSelectAccount(allAccounts[0]);
    }
  } catch (e) {
    console.error("Error loading accounts:", e);
    toast.error(m.finance_ui_accounts_load_failed());
  } finally {
    isLoading = false;
  }
}

// Build authentic hierarchical tree directly from backend accounts (Root accounts -> Child accounts)
const accountTree = $derived.by(() => {
  function buildSubTree(parentId: string | null = null): AccountNode[] {
    return allAccounts
      .filter((a) => (parentId ? a.parent_id === parentId : !a.parent_id))
      .sort((a, b) =>
        (a.code || "").localeCompare(b.code || "", undefined, {
          numeric: true,
        }),
      )
      .map((node) => ({
        ...node,
        name: l(node.name_ar, node.name_en || node.name) || "—",
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
  selectedAccount = account;
  isAddingNew = false;

  formParentId = (account as any).parent_id || null;
  formCode = account.code || "";
  formNameAr = account.name_ar || (isAr() ? account.name : "") || "";
  formNameEn = account.name_en || (!isAr() ? account.name : "") || "";
  formAccountType = String(
    account.account_type || account.type || "asset",
  ).toLowerCase();
  formSubType = account.sub_type || "general";
  formCurrency = account.currency || baseCurrency;

  const hasChildren = parentAccountIds.has(account.id);
  formAccountLevel = hasChildren ? "primary" : "sub";
}

function startAddSubAccount(
  parentAccount: AccountNode | null = selectedAccount,
) {
  isAddingNew = true;
  formParentId = parentAccount ? parentAccount.id : null;
  formCode = suggestNextChildCode(parentAccount);
  formNameAr = "";
  formNameEn = "";
  formAccountLevel = "sub";

  if (parentAccount) {
    formAccountType = String(
      parentAccount.account_type || parentAccount.type || "asset",
    ).toLowerCase();
    formCurrency = parentAccount.currency || baseCurrency;
  }
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
  isFinancePurposeAccount || selectedAccountHasChildren,
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
async function handleSaveAccount() {
  if (!formNameAr.trim() && !formNameEn.trim()) {
    toast.error(m.finance_ui_account_name_required());
    return;
  }
  if (!formCode.trim()) {
    toast.error(m.finance_ui_account_code_required());
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

    let result: any;
    if (!isAddingNew && selectedAccount?.id) {
      result = await api.updateAccount(selectedAccount.id, payload);
      toast.success(m.finance_ui_account_update_success());
    } else {
      result = await api.createAccount(payload);
      toast.success(m.finance_ui_account_save_success());
    }
    await loadData();

    if (result && result.id) {
      handleSelectAccount(result as any);
    }
  } catch (err: any) {
    console.error("Failed to save account:", err);
    toast.error(err?.message || m.finance_ui_account_save_failed());
  } finally {
    isSubmitting = false;
  }
}

async function handleDeleteAccount() {
  if (!selectedAccount?.id) return;

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
    await api.deleteAccount(selectedAccount.id);
    toast.success(m.finance_ui_account_delete_success());
    selectedAccount = null;
    await loadData();
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
      account.balances?.[account.currency || baseCurrency] || 0,
    );
    return {
      ...account,
      __idx: i + 1,
      name_display: l(account.name_ar, account.name_en || account.name) || "—",
      balance_formatted: formatCurrency(bal, account.currency || baseCurrency),
      currency_code: account.currency || baseCurrency,
      status:
        account.is_active === 1 || account.is_active === true
          ? "active"
          : "inactive",
      status_display:
        account.is_active === 1 || account.is_active === true
          ? m.finance_ui_active()
          : m.finance_ui_inactive(),
    };
  }),
);

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
  <div class="flex min-h-0 w-full flex-1 flex-col gap-4" in:fade={{ duration: 150 }}>
    
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
          {#if isLoading}<LoadingIndicator  size={14} />{:else}<Refresh size={14}   />{/if}
        </Button>

        <Button variant="ghost" size="icon-sm" onclick={exportAccountsCsv} class="text-muted-foreground">
          <Download size={14} />
        </Button>
      </div>
    </header>

    <!-- Content Workspace -->
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
              onSelectAccount={(node) => handleSelectAccount(node)}
            />
          </div>

          <!-- Account Details & Editing / Creation Form (7 cols) -->
          <div class="lg:col-span-7 flex flex-col h-full min-h-0 overflow-y-auto">
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

              <CardContent class="p-4 flex-1 flex flex-col gap-4 overflow-y-auto">
                {#if isFinancePurposeAccount}
                  <div class="rounded-md border border-amber-500/30 bg-amber-500/5 px-3 py-2 text-xs text-amber-800 dark:text-amber-200">
                    {m.finance_ui_fixed_finance_account_notice()}
                  </div>
                {/if}

                <!-- Row 1: الحساب الأب (Parent Account) -->
                <div class="flex flex-col gap-1">
                  <Label class="text-xs font-semibold text-foreground">{m.finance_ui_parent_account()}</Label>
                  <div class="p-2 rounded-md bg-muted/30 border border-border/50 flex items-center justify-between text-xs">
                    <div class="flex items-center gap-2">
                      <span class="font-mono font-bold text-muted-foreground">
                        {parentAccountNode ? parentAccountNode.code : "—"}
                      </span>
                      <span class="font-medium text-foreground">
                        {parentAccountNode ? (l(parentAccountNode.name_ar, parentAccountNode.name_en || parentAccountNode.name) || parentAccountNode.code) : m.finance_ui_root_account()}
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
                    <Select
                      type="single"
                      value={formAccountLevel}
                      disabled={isAccountStructureLocked}
                      onValueChange={(value) => {
                        if (value === "primary" || value === "sub") formAccountLevel = value;
                      }}
                    >
                      <SelectTrigger class="h-8 w-full bg-background text-xs">
                        {formAccountLevel === "primary"
                          ? m.finance_ui_account_level_primary()
                          : m.finance_ui_account_level_sub()}
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="primary">{m.finance_ui_account_level_primary()}</SelectItem>
                        <SelectItem value="sub">{m.finance_ui_account_level_sub()}</SelectItem>
                      </SelectContent>
                    </Select>
                  </div>
                </div>

                <!-- Row 4: التصنيف المالي + النوع الفرعي + العملة -->
                <div class="grid min-w-0 grid-cols-1 gap-4 sm:grid-cols-2 xl:grid-cols-3">
                  <div>
                    <Label class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_account_category()}</Label>
                    <Select
                      type="single"
                      value={formAccountType}
                      disabled={isAccountStructureLocked}
                      onValueChange={(value) => {
                        if (value === "asset" || value === "liability" || value === "equity" || value === "revenue" || value === "expense") formAccountType = value;
                      }}
                    >
                      <SelectTrigger class="h-8 w-full bg-background text-xs">
                        {formAccountType === "asset"
                          ? m.finance_ui_account_type_asset_numbered()
                          : formAccountType === "liability"
                            ? m.finance_ui_account_type_liability_numbered()
                            : formAccountType === "equity"
                              ? m.finance_ui_account_type_equity_numbered()
                              : formAccountType === "revenue"
                                ? m.finance_ui_account_type_revenue_numbered()
                                : m.finance_ui_account_type_expense_numbered()}
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="asset">{m.finance_ui_account_type_asset_numbered()}</SelectItem>
                        <SelectItem value="liability">{m.finance_ui_account_type_liability_numbered()}</SelectItem>
                        <SelectItem value="equity">{m.finance_ui_account_type_equity_numbered()}</SelectItem>
                        <SelectItem value="revenue">{m.finance_ui_account_type_revenue_numbered()}</SelectItem>
                        <SelectItem value="expense">{m.finance_ui_account_type_expense_numbered()}</SelectItem>
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
                        if (value === "general" || value === "cash" || value === "bank" || value === "customer" || value === "supplier" || value === "employee") formSubType = value;
                      }}
                    >
                      <SelectTrigger class="h-8 w-full bg-background text-xs">
                        {formSubType === "general"
                          ? m.finance_ui_account_type_general()
                          : formSubType === "cash"
                            ? m.finance_ui_cash_treasury()
                            : formSubType === "bank"
                              ? m.finance_ui_account_type_bank()
                              : formSubType === "customer"
                                ? m.finance_ui_account_type_customer_student()
                                : formSubType === "supplier"
                                  ? m.finance_ui_account_type_supplier()
                                  : m.finance_ui_account_type_employee()}
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="general">{m.finance_ui_account_type_general()}</SelectItem>
                        <SelectItem value="cash">{m.finance_ui_cash_treasury()}</SelectItem>
                        <SelectItem value="bank">{m.finance_ui_account_type_bank()}</SelectItem>
                        <SelectItem value="customer">{m.finance_ui_account_type_customer_student()}</SelectItem>
                        <SelectItem value="supplier">{m.finance_ui_account_type_supplier()}</SelectItem>
                        <SelectItem value="employee">{m.finance_ui_account_type_employee()}</SelectItem>
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
                  {@const curBal = Number(selectedAccount.balances?.[formCurrency] || 0)}
                  <div class="p-3 rounded-lg border border-primary/20 bg-primary/5 flex items-center justify-between">
                    <span class="text-xs font-bold text-foreground">{m.finance_ui_current_account_balance()}</span>
                    <span class="text-sm font-mono font-extrabold text-primary">
                      {formatCurrency(curBal, formCurrency)}
                    </span>
                  </div>
                {/if}
              </CardContent>

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
                    onclick={handleSaveAccount}
                  >
                    {isSubmitting ? m.finance_ui_saving() : (isAddingNew ? m.finance_ui_add_account() : m.finance_ui_save_changes())}
                  </Button>
                </div>
              </div>
            </Card>
          </div>
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
                {#if isLoading}<LoadingIndicator  size={14} />{:else}<Refresh size={14}   />{/if}
              </Button>
              <Button variant="ghost" size="icon-sm" onclick={exportAccountsCsv} class="text-muted-foreground">
                <Download size={14} />
              </Button>
            </div>
          </div>

          <div class="flex-1 min-h-0">
            <OrbisGrid
              columns={treasuryCols}
              source={treasurySource}
              loading={isLoading}
              clientQuery
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
  </div>
</div>
