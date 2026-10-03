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
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api } from "$lib/api";
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
import { formatCurrency, formatDate, isAr } from "$lib/core/i18n";
import { fade, slide } from "svelte/transition";
import { OrbisGrid, type OrbisColumnDef } from "@orbis/svelte-grid";
import AccountTree, { type AccountNode } from "$lib/features/finance/components/account-tree.svelte";
import SearchableSelect from "$lib/shared/components/searchable-select.svelte";
import { FilterTrigger, type UniversalFilter } from "@orbis/ui/filter-builder";
import { accountFilterFields } from "$lib/core/filter-configs";

let { orgId }: { orgId: string } = $props();

// ── State ──────────────────────────────────────────────────────────────
let allAccounts = $state<AccountNode[]>([]);
let currencies = $state<any[]>([]);
let baseCurrency = $state("YER");
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
let formCurrency = $state("YER");

let activeFilter = $state<UniversalFilter>({
  logic: "AND",
  conditions: [],
  groups: [],
  rules: [],
} as any);

const rtl = $derived(getLocale() === "ar");

onMount(async () => {
  if (orgId) await loadData();
});

async function loadData() {
  isLoading = true;
  try {
    const [accs, currs] = await Promise.all([
      api.listAccounts().catch(() => []),
      api.listCurrencies().catch(() => []),
    ]);

    allAccounts = accs || [];
    currencies = currs || [];

    const base = currs.find((c: any) => c.is_base === 1 || c.is_base === true);
    if (base) baseCurrency = base.code;

    // Auto-select first root account if none selected
    if (!selectedAccount && allAccounts.length > 0) {
      handleSelectAccount(allAccounts[0]);
    }
  } catch (e) {
    console.error("Error loading accounts:", e);
    toast.error("فشل تحميل دليل الحسابات");
  } finally {
    isLoading = false;
  }
}

// Build authentic hierarchical tree directly from backend accounts (Root accounts -> Child accounts)
const accountTree = $derived.by(() => {
  function buildSubTree(parentId: string | null = null): AccountNode[] {
    return allAccounts
      .filter((a) => (parentId ? a.parent_id === parentId : !a.parent_id))
      .sort((a, b) => (a.code || "").localeCompare(b.code || "", undefined, { numeric: true }))
      .map((node) => ({
        ...node,
        name: node.name_ar || node.name || "—",
        children: buildSubTree(node.id),
      }));
  }
  return buildSubTree(null);
});

// ── Smart Code Suggestion ──────────────────────────────────────────────
function suggestNextChildCode(parent: AccountNode | null): string {
  if (!parent) return String(allAccounts.filter((a) => !a.parent_id).length + 1);

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
  formNameAr = account.name_ar || account.name || "";
  formNameEn = account.name_en || account.name || "";
  formAccountType = String(account.account_type || account.type || "asset").toLowerCase();
  formSubType = account.sub_type || "general";
  formCurrency = account.currency || baseCurrency;

  const hasChildren = allAccounts.some((a) => a.parent_id === account.id);
  formAccountLevel = hasChildren ? "primary" : "sub";
}

function startAddSubAccount(parentAccount: AccountNode | null = selectedAccount) {
  isAddingNew = true;
  formParentId = parentAccount ? parentAccount.id : null;
  formCode = suggestNextChildCode(parentAccount);
  formNameAr = "";
  formNameEn = "";
  formAccountLevel = "sub";

  if (parentAccount) {
    formAccountType = String(parentAccount.account_type || parentAccount.type || "asset").toLowerCase();
    formCurrency = parentAccount.currency || baseCurrency;
  }
}

const parentAccountNode = $derived(
  allAccounts.find((a) => a.id === formParentId)
);

// Determine Closing Statement based on Account Type
const closingStatementLabel = $derived.by(() => {
  const type = formAccountType.toLowerCase();
  if (type === "asset" || type === "liability" || type === "equity") {
    return "الميزانية العمومية (Balance Sheet)";
  }
  return "الأرباح والخسائر / قائمة الدخل (Income Statement)";
});

// ── Account Save / Create Action ───────────────────────────────────────
async function handleSaveAccount() {
  if (!formNameAr.trim() && !formNameEn.trim()) {
    toast.error("يرجى إدخال اسم الحساب");
    return;
  }
  if (!formCode.trim()) {
    toast.error("يرجى إدخال رقم / كود الحساب");
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
      toast.success("تم تحديث بيانات الحساب بنجاح");
    } else {
      result = await api.createAccount(payload);
      toast.success("تم حفظ الحساب في دليل الحسابات بنجاح");
    }
    await loadData();

    if (result && result.id) {
      handleSelectAccount(result as any);
    }
  } catch (err: any) {
    console.error("Failed to save account:", err);
    toast.error(err?.message || "فشل حفظ الحساب");
  } finally {
    isSubmitting = false;
  }
}

async function handleDeleteAccount() {
  if (!selectedAccount?.id) return;

  const confirmed = await confirmStore.prompt({
    title: "حذف الحساب",
    description: `هل أنت متأكد من رغبتك في حذف الحساب "${selectedAccount.name || selectedAccount.code}"؟ لن يمكن التراجع عن هذا الإجراء إذا كانت هناك قيود مسجلة.`,
    confirmText: "حذف الحساب",
    cancelText: "إلغاء",
    variant: "destructive",
  });

  if (!confirmed) return;

  isSubmitting = true;
  try {
    await api.deleteAccount(selectedAccount.id);
    toast.success("تم حذف الحساب بنجاح");
    selectedAccount = null;
    await loadData();
  } catch (err: any) {
    console.error("Failed to delete account:", err);
    toast.error(err?.message || "فشل حذف الحساب");
  } finally {
    isSubmitting = false;
  }
}

// ── Grid Data for Table View ───────────────────────────────────────────
const leafAccounts = $derived(
  allAccounts.filter((a) => !allAccounts.some((cand) => cand.parent_id === a.id))
);

const treasurySource = $derived(
  leafAccounts.map((account, i) => {
    const bal = Number(account.balances?.[account.currency || baseCurrency] || 0);
    return {
      ...account,
      __idx: i + 1,
      name_display: account.name_ar || account.name || "—",
      balance_formatted: formatCurrency(bal, account.currency || baseCurrency),
      currency_code: account.currency || baseCurrency,
      status: account.is_active === 1 ? "active" : "inactive",
    };
  })
);

const treasuryCols: OrbisColumnDef[] = [
  { id: "__idx", prop: "__idx", header: "#", name: "#", width: 48, size: 48, readonly: true, pin: "left", align: "center" },
  { id: "code", prop: "code", header: "رقم الحساب", name: "رقم الحساب", width: 110, size: 110, readonly: true, cellClass: "font-mono font-bold" },
  { id: "name_display", prop: "name_display", header: "اسم الحساب", name: "اسم الحساب", width: 220, size: 220, readonly: true, cellClass: "font-medium" },
  { id: "balance_formatted", prop: "balance_formatted", header: "الرصيد الحالي", name: "الرصيد الحالي", width: 140, size: 140, readonly: true, align: "end", cellClass: "font-mono font-bold text-primary" },
  { id: "currency_code", prop: "currency_code", header: "العملة", name: "العملة", width: 80, size: 80, readonly: true, align: "center" },
  { id: "status", prop: "status", header: "الحالة", name: "الحالة", width: 90, size: 90, type: "badge", align: "center", readonly: true },
];

function exportAccountsCsv() {
  const rows = treasurySource.map((a) => [
    a.code,
    a.name_display,
    a.balance_formatted,
    a.currency_code,
    a.status,
  ]);
  const csv = [
    ["Code", "Name", "Balance", "Currency", "Status"],
    ...rows,
  ]
    .map((row) => row.map((v) => `"${String(v ?? "").replaceAll('"', '""')}"`).join(","))
    .join("\n");
  const url = URL.createObjectURL(new Blob(["\ufeff" + csv], { type: "text/csv;charset=utf-8" }));
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
            <span>شجرة ودليل الحسابات</span>
          </Button>

          <Button
            variant={viewMode === "grid" ? "default" : "ghost"}
            size="sm"
            class="h-7 text-xs font-semibold px-2.5 gap-1.5"
            onclick={() => viewMode = "grid"}
          >
            <Grid size={13} />
            <span>جدول الحسابات الشامل</span>
          </Button>
        </div>

        <Button
          variant="secondary"
          size="sm"
          class="h-7 text-xs font-bold gap-1"
          onclick={() => startAddSubAccount(selectedAccount)}
        >
          <Plus size={13} />
          <span>إضافة حساب فرعي</span>
        </Button>
      </div>

      <div class="flex items-center gap-2">
        <Button variant="ghost" size="icon-sm" onclick={loadData} class="text-muted-foreground">
          <Refresh size={14} class={isLoading ? "animate-spin" : ""} />
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
        <!-- Classic Split-View Layout (Tree on Right + Form on Left)     -->
        <!-- ══════════════════════════════════════════════════════════════ -->
        <div class="grid grid-cols-1 lg:grid-cols-12 gap-4 w-full h-full min-h-0">
          
          <!-- Left: Account Details & Editing / Creation Form (7 cols) -->
          <div class="lg:col-span-7 flex flex-col h-full min-h-0 overflow-y-auto">
            <Card class="border border-border/50 bg-card shadow-sm h-full flex flex-col justify-between">
              <CardHeader class="py-3 px-4 border-b border-border/40 shrink-0">
                <div class="flex items-center justify-between">
                  <div class="flex items-center gap-2">
                    <span class="h-2 w-2 rounded-full {isAddingNew ? 'bg-emerald-500 animate-pulse' : 'bg-primary'}"></span>
                    <CardTitle class="text-xs font-bold uppercase tracking-wider text-foreground">
                      {isAddingNew ? "إضافة حساب جديد في الدليل" : `بيانات الحساب: ${formNameAr || formCode}`}
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
                <!-- Row 1: الحساب الأب (Parent Account) -->
                <div class="flex flex-col gap-1">
                  <Label class="text-xs font-semibold text-foreground">الحساب الأب (المستوى الأعلى)</Label>
                  <div class="p-2 rounded-md bg-muted/30 border border-border/50 flex items-center justify-between text-xs">
                    <div class="flex items-center gap-2">
                      <span class="font-mono font-bold text-muted-foreground">
                        {parentAccountNode ? parentAccountNode.code : "—"}
                      </span>
                      <span class="font-medium text-foreground">
                        {parentAccountNode ? (parentAccountNode.name_ar || parentAccountNode.name) : "حساب جذري رئيسي (بدون أب)"}
                      </span>
                    </div>

                    <Button
                      variant="ghost"
                      size="sm"
                      class="h-6 text-[11px] px-2"
                      onclick={() => { formParentId = null; formCode = suggestNextChildCode(null); }}
                    >
                      تعيين كحساب جذري
                    </Button>
                  </div>
                </div>

                <!-- Row 2: اسم الحساب (عربي + إنجليزي) -->
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <div>
                    <Label class="text-xs font-semibold text-foreground mb-1 block">اسم الحساب (بالعربية)</Label>
                    <Input
                      type="text"
                      bind:value={formNameAr}
                      placeholder="مثال: الصندوق الرئيسي / بنك التضامن"
                      class="h-8 text-xs bg-background border-border/60"
                    />
                  </div>

                  <div>
                    <Label class="text-xs font-semibold text-foreground mb-1 block">اسم الحساب (English)</Label>
                    <Input
                      type="text"
                      bind:value={formNameEn}
                      placeholder="e.g. Main Cash Vault"
                      class="h-8 text-xs font-mono bg-background border-border/60"
                    />
                  </div>
                </div>

                <!-- Row 3: رقم الحساب + نوع المستوى (رئيسي / فرعي) -->
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <div>
                    <Label class="text-xs font-semibold text-foreground mb-1 block">رقم / كود الحساب</Label>
                    <Input
                      type="text"
                      bind:value={formCode}
                      placeholder="1101"
                      class="h-8 text-xs font-mono font-bold bg-background border-border/60"
                    />
                  </div>

                  <div>
                    <Label class="text-xs font-semibold text-foreground mb-1 block">نوع الحساب</Label>
                    <select
                      bind:value={formAccountLevel}
                      class="w-full h-8 text-xs rounded-md bg-background border border-border/60 px-2 text-foreground focus:outline-none"
                    >
                      <option value="primary">حساب رئيسي (مجمع / لا يقبل القيود المباشرة)</option>
                      <option value="sub">حساب فرعي (تحليلي / يقبل حركات القيود والسندات)</option>
                    </select>
                  </div>
                </div>

                <!-- Row 4: التصنيف المالي + النوع الفرعي + العملة -->
                <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                  <div>
                    <Label class="text-xs font-semibold text-foreground mb-1 block">التصنيف المحاسبي</Label>
                    <select
                      bind:value={formAccountType}
                      class="w-full h-8 text-xs rounded-md bg-background border border-border/60 px-2 text-foreground focus:outline-none"
                    >
                      <option value="asset">1 - الأصول (Assets)</option>
                      <option value="liability">2 - الالتزامات (Liabilities)</option>
                      <option value="equity">3 - حقوق الملكية (Equity)</option>
                      <option value="revenue">4 - الإيرادات (Revenue)</option>
                      <option value="expense">5 - المصروفات (Expenses)</option>
                    </select>
                  </div>

                  <div>
                    <Label class="text-xs font-semibold text-foreground mb-1 block">النوع الفرعي</Label>
                    <select
                      bind:value={formSubType}
                      class="w-full h-8 text-xs rounded-md bg-background border border-border/60 px-2 text-foreground focus:outline-none"
                    >
                      <option value="general">عام / افتراضي</option>
                      <option value="cash">خزينة نقدية (صندوق)</option>
                      <option value="bank">حساب بنكي</option>
                      <option value="customer">عميل / طالب</option>
                      <option value="supplier">مورد</option>
                      <option value="employee">موظف</option>
                    </select>
                  </div>

                  <div>
                    <Label class="text-xs font-semibold text-foreground mb-1 block">عملة الحساب</Label>
                    <SearchableSelect
                      value={formCurrency}
                      placeholder="العملة..."
                      items={currencies.length > 0 ? currencies : [{ code: 'YER', name: 'ريال يمني' }]}
                      getItemValue={(c) => c.code}
                      getItemLabel={(c) => `${c.name || c.code} (${c.code})`}
                      onchange={(val) => { if (val) formCurrency = val; }}
                      class="w-full h-8 text-xs bg-background border-border/60"
                    />
                  </div>
                </div>

                <!-- Row 5: الحساب الختامي (Closing Statement) -->
                <div class="p-2.5 rounded-lg border border-border/50 bg-muted/10 flex items-center justify-between text-xs">
                  <span class="font-semibold text-muted-foreground">الحساب الختامي الموجه له:</span>
                  <span class="font-bold text-foreground">{closingStatementLabel}</span>
                </div>

                <!-- Current Balance Display (if existing account) -->
                {#if selectedAccount && !isAddingNew}
                  {@const curBal = Number(selectedAccount.balances?.[formCurrency] || 0)}
                  <div class="p-3 rounded-lg border border-primary/20 bg-primary/5 flex items-center justify-between">
                    <span class="text-xs font-bold text-foreground">الرصيد المالي الحالي للحساب:</span>
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
                      <span>كشف الحساب</span>
                    </a>
                  {/if}
                </div>

                <div class="flex items-center gap-2">
                  {#if !isAddingNew && selectedAccount?.id}
                    <Button
                      variant="destructive"
                      size="sm"
                      class="h-8 text-xs font-semibold gap-1.5"
                      disabled={isSubmitting}
                      onclick={handleDeleteAccount}
                    >
                      <Trash size={13} />
                      <span>حذف الحساب</span>
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
                    إلغاء
                  </Button>

                  <Button
                    variant="default"
                    size="sm"
                    class="h-8 text-xs font-bold px-4"
                    disabled={isSubmitting}
                    onclick={handleSaveAccount}
                  >
                    {isSubmitting ? "جاري الحفظ..." : (isAddingNew ? "إضافة الحساب" : "حفظ التعديلات")}
                  </Button>
                </div>
              </div>
            </Card>
          </div>

          <!-- Right: Compact TreeView (5 cols) -->
          <div class="lg:col-span-5 flex flex-col h-full min-h-0 overflow-hidden rounded-xl border border-border/40 bg-card p-2 shadow-sm">
            <AccountTree
              accounts={accountTree}
              {orgId}
              onSelectAccount={(node) => handleSelectAccount(node)}
            />
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
                placeholder="ابحث باسم الحساب أو رقمه..."
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
                <Refresh size={14} class={isLoading ? "animate-spin" : ""} />
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
