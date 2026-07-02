<!--
  @page CurrenciesSettingsPage
  Allows administrators to manage multi-currency exchange rates, define base currencies,
  add new currencies, edit currency configurations, and delete unused currencies.
-->
<script lang="ts">
import { page } from "$app/state";
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
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import { Badge } from "@orbis/ui-svelte/badge";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Switch } from "@orbis/ui-svelte/switch";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { toast } from "svelte-sonner";
import {
  Coins,
  RefreshCw,
  Edit2,
  Trash2,
  Plus,
  Check,
  X,
  Sparkles,
  Globe,
  Info,
} from "lucide-svelte";
import { fade, slide } from "svelte/transition";
import * as m from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";

let orgId = $derived((page.params.orgId as string) || "");

// State
let currencies = $state<any[]>([]);
let isLoading = $state(true);
let isSeeding = $state(false);
let isSubmitting = $state(false);

// Inline rate editing state
let editingRateCode = $state<string | null>(null);
let editRateValue = $state<number>(1.0);

// Dialogs state
let showAddDialog = $state(false);
let showEditDialog = $state(false);
let currencyToDelete = $state<any | null>(null);

// Add form state
let addCode = $state("");
let addName = $state("");
let addNameAr = $state("");
let addSymbol = $state("");
let addSymbolPosition = $state("after"); // "before" | "after"
let addDecimalPlaces = $state(2);
let addExchangeRate = $state(1.0);
let addIsBase = $state(false);
let addIsActive = $state(true);

// Edit form state
let editCode = $state("");
let editName = $state("");
let editNameAr = $state("");
let editSymbol = $state("");
let editSymbolPosition = $state("after");
let editDecimalPlaces = $state(2);
let editExchangeRate = $state(1.0);
let editIsBase = $state(false);
let editIsActive = $state(true);

async function loadCurrencies() {
  if (!orgId) return;
  isLoading = true;
  try {
    currencies = await commands.finance.getCurrencies(orgId);
  } catch (e) {
    console.error("Error loading currencies:", e);
    toast.error(
      getLocale() === "ar"
        ? "فشل في تحميل العملات"
        : "Failed to load currencies",
    );
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (orgId) {
    loadCurrencies();
  }
});

function openAddDialog() {
  addCode = "";
  addName = "";
  addNameAr = "";
  addSymbol = "";
  addSymbolPosition = "after";
  addDecimalPlaces = 2;
  addExchangeRate = 1.0;
  addIsBase = false;
  addIsActive = true;
  showAddDialog = true;
}

async function handleAddCurrency(e: Event) {
  e.preventDefault();
  if (isSubmitting) return;

  const code = addCode.trim().toUpperCase();
  if (!code || code.length < 2) {
    toast.error(
      getLocale() === "ar" ? "رمز العملة غير صالح" : "Invalid currency code",
    );
    return;
  }

  // Check if code already exists
  if (currencies.some((c) => c.code === code)) {
    toast.error(
      getLocale() === "ar"
        ? "هذه العملة مضافة بالفعل"
        : "This currency already exists",
    );
    return;
  }

  if (addExchangeRate <= 0) {
    toast.error(
      getLocale() === "ar"
        ? "سعر الصرف يجب أن يكون أكبر من الصفر"
        : "Exchange rate must be greater than zero",
    );
    return;
  }

  isSubmitting = true;
  try {
    const newCurrency = {
      code,
      org_id: orgId,
      name: addName.trim() || code,
      name_ar: addNameAr.trim() || null,
      symbol: addSymbol.trim() || code,
      symbol_position: addSymbolPosition,
      decimal_places: addDecimalPlaces,
      exchange_rate: addExchangeRate,
      is_base: addIsBase ? 1 : 0,
      is_active: addIsActive ? 1 : 0,
      updated_at: new Date().toISOString(),
    };

    await commands.finance.createCurrency(newCurrency);
    toast.success(
      getLocale() === "ar"
        ? "تم إضافة العملة بنجاح"
        : "Currency added successfully",
    );
    showAddDialog = false;
    await loadCurrencies();
  } catch (e) {
    console.error("Error creating currency:", e);
    const errorMsg = e instanceof Error ? e.message : String(e);
    toast.error(
      getLocale() === "ar"
        ? `خطأ أثناء الإضافة: ${errorMsg}`
        : `Error adding currency: ${errorMsg}`,
    );
  } finally {
    isSubmitting = false;
  }
}

function openEditDialog(c: any) {
  editCode = c.code;
  editName = c.name || "";
  editNameAr = c.name_ar || "";
  editSymbol = c.symbol || "";
  editSymbolPosition = c.symbol_position || "after";
  editDecimalPlaces = c.decimal_places ?? 2;
  editExchangeRate = c.exchange_rate ?? 1.0;
  editIsBase = c.is_base === 1;
  editIsActive = c.is_active === 1;
  showEditDialog = true;
}

async function handleEditCurrency(e: Event) {
  e.preventDefault();
  if (isSubmitting) return;

  if (editExchangeRate <= 0) {
    toast.error(
      getLocale() === "ar"
        ? "سعر الصرف يجب أن يكون أكبر من الصفر"
        : "Exchange rate must be greater than zero",
    );
    return;
  }

  isSubmitting = true;
  try {
    const updates = {
      name: editName.trim() || editCode,
      name_ar: editNameAr.trim() ? editNameAr.trim() : null,
      symbol: editSymbol.trim() || editCode,
      symbol_position: editSymbolPosition,
      decimal_places: editDecimalPlaces,
      exchange_rate: editExchangeRate,
      is_base: editIsBase ? 1 : 0,
      is_active: editIsActive ? 1 : 0,
    };

    await commands.finance.updateCurrency(orgId, editCode, updates);
    toast.success(
      getLocale() === "ar"
        ? "تم تحديث العملة بنجاح"
        : "Currency updated successfully",
    );
    showEditDialog = false;
    await loadCurrencies();
  } catch (e) {
    console.error("Error updating currency:", e);
    const errorMsg = e instanceof Error ? e.message : String(e);
    toast.error(
      getLocale() === "ar"
        ? `خطأ أثناء التعديل: ${errorMsg}`
        : `Error updating currency: ${errorMsg}`,
    );
  } finally {
    isSubmitting = false;
  }
}

async function confirmDeleteCurrency(c: any) {
  if (c.is_base === 1) {
    toast.error(
      getLocale() === "ar"
        ? "لا يمكن حذف العملة الأساسية للنظام"
        : "Cannot delete the base currency",
    );
    return;
  }
  currencyToDelete = c;
}

async function handleDeleteCurrency() {
  if (!currencyToDelete) return;
  try {
    await commands.finance.deleteCurrency(orgId, currencyToDelete.code);
    toast.success(
      getLocale() === "ar"
        ? `تم حذف العملة (${currencyToDelete.code}) بنجاح`
        : `Currency (${currencyToDelete.code}) deleted successfully`,
    );
    currencyToDelete = null;
    await loadCurrencies();
  } catch (e) {
    console.error("Error deleting currency:", e);
    const errorMsg = e instanceof Error ? e.message : String(e);
    toast.error(
      getLocale() === "ar"
        ? `فشل الحذف: ${errorMsg}`
        : `Failed to delete currency: ${errorMsg}`,
    );
  }
}

async function seedFinancials() {
  const confirmed = confirm(
    getLocale() === "ar"
      ? "هل أنت متأكد من رغبتك في تهيئة النظام المالي الافتراضي؟ سيتم إنشاء العملات (ريال، دولار، سعودي) وشجرة الحسابات المحاسبية الأساسية."
      : "Are you sure you want to seed the default financial system? This will generate currencies (YER, USD, SAR) and the basic Chart of Accounts.",
  );
  if (!confirmed) return;

  isSeeding = true;
  try {
    await commands.finance.seedDefaultFinancials(orgId);
    toast.success(
      getLocale() === "ar"
        ? "تم تهيئة النظام المالي وتنزيل شجرة الحسابات والعملات الافتراضية بنجاح!"
        : "Financial system, default accounts, and currencies seeded successfully!",
    );
    await loadCurrencies();
  } catch (e) {
    console.error("Error seeding financials:", e);
    const errorMsg = e instanceof Error ? e.message : String(e);
    toast.error(
      getLocale() === "ar"
        ? `خطأ أثناء التهيئة: ${errorMsg}`
        : `Error seeding financials: ${errorMsg}`,
    );
  } finally {
    isSeeding = false;
  }
}

function startEditingRate(c: any) {
  if (c.is_base === 1) return;
  editingRateCode = c.code;
  editRateValue = c.exchange_rate;
}

function cancelEditingRate() {
  editingRateCode = null;
}

async function saveRateValue(code: string) {
  if (editRateValue <= 0) {
    toast.error(
      getLocale() === "ar"
        ? "سعر الصرف يجب أن يكون أكبر من الصفر"
        : "Exchange rate must be greater than zero",
    );
    return;
  }
  try {
    await commands.finance.updateCurrency(orgId, code, {
      exchange_rate: editRateValue,
    });
    toast.success(
      getLocale() === "ar"
        ? "تم تحديث سعر الصرف بنجاح"
        : "Exchange rate updated successfully",
    );
    editingRateCode = null;
    await loadCurrencies();
  } catch (e) {
    console.error("Error updating exchange rate:", e);
    const errorMsg = e instanceof Error ? e.message : String(e);
    toast.error(
      getLocale() === "ar" ? `خطأ: ${errorMsg}` : `Error: ${errorMsg}`,
    );
  }
}

// Find base currency representation
let baseCurrencyCode = $derived(
  currencies.find((c) => c.is_base === 1)?.code || "YER",
);
</script>

<svelte:head>
  <title>{getLocale() === "ar" ? "العملات وأسعار الصرف" : "Currencies & Rates"} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto bg-background p-6 flex flex-col gap-6 scrollbar-hide">
  <!-- Header -->
  <div class="flex items-center justify-between gap-4 pb-5 select-none border-b border-border/40 font-sans">
    <div class="flex items-center gap-2">
      {#if !isLoading}
        <span class="text-[10px] px-2 py-0.5 rounded bg-muted font-mono text-muted-foreground font-bold">
          {getLocale() === "ar" ? `العملات (${currencies.length})` : `Currencies (${currencies.length})`}
        </span>
      {/if}
    </div>

    <!-- Actions -->
    <div class="flex items-center gap-2">
      <Button
        variant="outline"
        size="sm"
        onclick={seedFinancials}
        disabled={isSeeding}
        class="h-9 text-xs flex items-center gap-1.5 cursor-pointer border-primary/30 hover:bg-primary/5 text-primary font-semibold"
      >
        <Sparkles size={13} class={isSeeding ? "animate-spin" : ""} />
        <span>{isSeeding ? (getLocale() === "ar" ? "جاري التهيئة..." : "Seeding...") : (getLocale() === "ar" ? "تهيئة العملات الافتراضية" : "Seed Default Currencies")}</span>
      </Button>
      
      <Button
        variant="outline"
        size="sm"
        onclick={loadCurrencies}
        class="h-9 w-9 p-0 flex items-center justify-center cursor-pointer"
        title={getLocale() === "ar" ? "تحديث" : "Refresh"}
      >
        <RefreshCw size={13} />
      </Button>

      <Button
        variant="lime"
        size="sm"
        onclick={openAddDialog}
        class="h-9 text-xs flex items-center gap-1.5 font-bold cursor-pointer"
      >
        <Plus size={14} />
        <span>{getLocale() === "ar" ? "إضافة عملة" : "Add Currency"}</span>
      </Button>
    </div>
  </div>

  <!-- Info Alert Box -->
  <div class="border border-border/60 bg-muted/10 rounded-xl p-4 flex gap-3 text-start text-xs select-none hover:border-primary/25 transition-all duration-200">
    <Info class="text-primary shrink-0 animate-pulse" size={16} />
    <div class="flex flex-col gap-1 text-muted-foreground leading-relaxed">
      <span class="font-bold text-foreground text-xs">{getLocale() === "ar" ? "كيف يعمل نظام أسعار الصرف؟" : "How does the exchange rate system work?"}</span>
      <span>
        {getLocale() === "ar" 
          ? `يتم اعتماد العملة الأساسية (${baseCurrencyCode}) كعملة التقييم والتقارير في النظام. عند سداد الطلاب برسوم أجنبية (مثل الدولار أو السعودي)، يقوم النظام تلقائياً بتحويل المبلغ وحسابه بالعملة الأساسية بناءً على أسعار الصرف المدخلة هنا.`
          : `The base currency (${baseCurrencyCode}) is used for valuation and reports. When students pay with foreign currencies (e.g. USD or SAR), the system automatically converts and records the amount in the base currency based on the exchange rates defined here.`}
      </span>
    </div>
  </div>

  <!-- Content Section -->
  <div class="w-full">
    {#if isLoading}
      <div class="flex flex-col gap-3">
        {#each Array(3) as _}
          <Skeleton class="h-14 w-full rounded-lg" />
        {/each}
      </div>
    {:else if currencies.length === 0}
      <!-- Zero state -->
      <div class="border border-dashed border-border/80 rounded-2xl p-16 flex flex-col items-center justify-center text-center gap-3 select-none my-4">
        <div class="w-14 h-14 rounded-full bg-primary/10 flex items-center justify-center text-primary mb-2">
          <Globe size={28} />
        </div>
        <h3 class="text-sm font-bold text-foreground">{getLocale() === "ar" ? "لم يتم تهيئة أي عملات بعد" : "No currencies configured yet"}</h3>
        <p class="text-xs text-muted-foreground max-w-xs">
          {getLocale() === "ar" 
            ? "اضغط على زر التهيئة لتنزيل العملات وشجرة الحسابات الافتراضية للبدء فوراً." 
            : "Click 'Seed Default Currencies' to setup default currencies and Chart of Accounts."}
        </p>
        <Button variant="default" size="sm" onclick={seedFinancials} class="mt-2 cursor-pointer font-bold">
          {getLocale() === "ar" ? "تهيئة النظام الآن" : "Seed System Now"}
        </Button>
      </div>
    {:else}
      <div in:fade={{ duration: 150 }} class="overflow-hidden">
        <Table>
          <TableHeader class="bg-muted/40 select-none">
            <TableRow>
              <TableHead class="text-start font-bold text-xs">{getLocale() === "ar" ? "رمز العملة" : "Code"}</TableHead>
              <TableHead class="text-start font-bold text-xs">{getLocale() === "ar" ? "الاسم بالعربية" : "Arabic Name"}</TableHead>
              <TableHead class="text-start font-bold text-xs">{getLocale() === "ar" ? "الاسم بالإنجليزية" : "English Name"}</TableHead>
              <TableHead class="text-start font-bold text-xs">{getLocale() === "ar" ? "الرمز / العلامة" : "Symbol"}</TableHead>
              <TableHead class="text-start font-bold text-xs">{getLocale() === "ar" ? "النوع" : "Type"}</TableHead>
              <TableHead class="text-start font-bold text-xs">{getLocale() === "ar" ? "الحالة" : "Status"}</TableHead>
              <TableHead class="text-end font-bold text-xs">{getLocale() === "ar" ? "سعر الصرف" : "Exchange Rate"}</TableHead>
              <TableHead class="text-end font-bold text-xs w-[140px]">{getLocale() === "ar" ? "إجراءات" : "Actions"}</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {#each currencies as c (c.code)}
              <TableRow class="hover:bg-muted/10 transition-all duration-150">
                <TableCell class="text-start font-mono text-xs text-foreground font-bold">{c.code}</TableCell>
                <TableCell class="text-start text-xs font-semibold">{c.name_ar || "-"}</TableCell>
                <TableCell class="text-start text-xs text-muted-foreground">{c.name || "-"}</TableCell>
                <TableCell class="text-start font-mono text-xs">
                  <span class="bg-muted px-2 py-0.5 rounded border border-border/40 text-foreground font-bold">
                    {c.symbol}
                  </span>
                  <span class="text-[10px] text-muted-foreground ml-1">
                    ({c.symbol_position === 'before' ? (getLocale() === "ar" ? "قبل" : "before") : (getLocale() === "ar" ? "بعد" : "after")})
                  </span>
                </TableCell>
                <TableCell class="text-start text-xs">
                  {#if c.is_base === 1}
                    <Badge variant="lime" class="text-[9px] font-bold">
                      {getLocale() === "ar" ? "الأساسية" : "Base"}
                    </Badge>
                  {:else}
                    <Badge variant="secondary" class="text-[9px] font-semibold">
                      {getLocale() === "ar" ? "فرعية" : "Sub"}
                    </Badge>
                  {/if}
                </TableCell>
                <TableCell class="text-start text-xs">
                  {#if c.is_active === 1}
                    <Badge variant="lime" class="text-[9px] font-bold">
                      {getLocale() === "ar" ? "نشط" : "Active"}
                    </Badge>
                  {:else}
                    <Badge variant="secondary" class="text-[9px] font-semibold">
                      {getLocale() === "ar" ? "موقف" : "Inactive"}
                    </Badge>
                  {/if}
                </TableCell>
                <TableCell class="text-end text-xs font-bold font-mono">
                  {#if c.is_base === 1}
                    <span class="text-foreground select-none">1.0000</span>
                  {:else if editingRateCode === c.code}
                    <div class="flex items-center justify-end gap-1.5" transition:fade={{ duration: 100 }}>
                      <Input
                        type="number"
                        step="0.000001"
                        bind:value={editRateValue}
                        class="h-8 w-[100px] text-end font-mono text-xs font-bold bg-background border border-primary/60 focus:border-primary"
                      />
                      <span class="text-[10px] text-muted-foreground font-bold">{baseCurrencyCode}</span>
                      <Button
                        variant="default"
                        size="icon"
                        onclick={() => saveRateValue(c.code)}
                        class="h-7 w-7 bg-emerald-500 hover:bg-emerald-600 text-white cursor-pointer rounded-md shrink-0 flex items-center justify-center animate-in zoom-in-50"
                      >
                        <Check size={12} />
                      </Button>
                      <Button
                        variant="outline"
                        size="icon"
                        onclick={cancelEditingRate}
                        class="h-7 w-7 cursor-pointer rounded-md border-border/80 shrink-0 flex items-center justify-center animate-in zoom-in-50"
                      >
                        <X size={12} />
                      </Button>
                    </div>
                  {:else}
                    <button
                      onclick={() => startEditingRate(c)}
                      class="text-foreground hover:text-primary hover:underline cursor-pointer transition-colors duration-150 py-1 px-2 hover:bg-primary/5 rounded font-mono font-bold"
                      title={getLocale() === "ar" ? "اضغط لتعديل سعر الصرف" : "Click to edit exchange rate"}
                    >
                      {c.exchange_rate.toFixed(4)}
                    </button>
                    <span class="text-[10px] text-muted-foreground font-bold mr-1">{baseCurrencyCode}</span>
                  {/if}
                </TableCell>
                <TableCell class="text-end text-xs">
                  <div class="flex items-center justify-end gap-1.5">
                    <Button
                      variant="outline"
                      size="icon"
                      onclick={() => openEditDialog(c)}
                      class="h-7 w-7 cursor-pointer hover:text-primary hover:bg-primary/5"
                      title={getLocale() === "ar" ? "تعديل العملة" : "Edit Currency"}
                    >
                      <Edit2 size={12} />
                    </Button>
                    
                    {#if c.is_base !== 1}
                      <Button
                        variant="outline"
                        size="icon"
                        onclick={() => confirmDeleteCurrency(c)}
                        class="h-7 w-7 cursor-pointer text-destructive hover:bg-destructive/10 border-destructive/20"
                        title={getLocale() === "ar" ? "حذف العملة" : "Delete Currency"}
                      >
                        <Trash2 size={12} />
                      </Button>
                    {:else}
                      <span class="text-[9px] text-muted-foreground/60 italic px-1 select-none">
                        {getLocale() === "ar" ? "أساسية" : "Base"}
                      </span>
                    {/if}
                  </div>
                </TableCell>
              </TableRow>
            {/each}
          </TableBody>
        </Table>
      </div>
    {/if}
  </div>
</div>

<!-- Add Currency Modal -->
{#if showAddDialog}
  <div 
    in:fade={{ duration: 150 }} 
    class="fixed inset-0 z-50 bg-black/60 backdrop-blur-sm flex items-center justify-center p-4"
  >
    <form 
      onsubmit={handleAddCurrency}
      in:slide={{ duration: 200 }} 
      class="bg-card border border-border rounded-2xl w-full max-w-lg p-6 flex flex-col gap-5 text-start shadow-2xl overflow-y-auto max-h-[90vh]"
    >
      <div class="flex flex-col gap-1 border-b border-border/40 pb-3">
        <h2 class="text-sm font-bold text-foreground flex items-center gap-1.5">
          <Plus size={16} class="text-primary" />
          <span>{getLocale() === "ar" ? "إضافة عملة جديدة" : "Add New Currency"}</span>
        </h2>
        <p class="text-[10px] text-muted-foreground">
          {getLocale() === "ar" 
            ? "قم بإدخال بيانات العملة الجديدة وسعر صرفها مقابل العملة الأساسية." 
            : "Define code, names, symbol, position and initial exchange rate."}
        </p>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <!-- Currency Code -->
        <div class="flex flex-col gap-1.5">
          <label for="add_code" class="text-xs font-semibold text-muted-foreground">
            {getLocale() === "ar" ? "رمز العملة (مثال: USD)" : "Currency Code (e.g. USD)"} *
          </label>
          <Input
            id="add_code"
            type="text"
            bind:value={addCode}
            placeholder="USD"
            maxlength={5}
            required
            class="uppercase font-mono font-bold"
          />
        </div>

        <!-- Symbol -->
        <div class="flex flex-col gap-1.5">
          <label for="add_symbol" class="text-xs font-semibold text-muted-foreground">
            {getLocale() === "ar" ? "رمز / علامة العملة (مثال: $)" : "Symbol (e.g. $)"} *
          </label>
          <Input
            id="add_symbol"
            type="text"
            bind:value={addSymbol}
            placeholder="$"
            required
            class="font-mono font-bold"
          />
        </div>

        <!-- Arabic Name -->
        <div class="flex flex-col gap-1.5">
          <label for="add_name_ar" class="text-xs font-semibold text-muted-foreground">
            {getLocale() === "ar" ? "الاسم بالعربية" : "Arabic Name"}
          </label>
          <Input
            id="add_name_ar"
            type="text"
            bind:value={addNameAr}
            placeholder="مثال: دولار أمريكي"
          />
        </div>

        <!-- English Name -->
        <div class="flex flex-col gap-1.5">
          <label for="add_name" class="text-xs font-semibold text-muted-foreground">
            {getLocale() === "ar" ? "الاسم بالإنجليزية" : "English Name"} *
          </label>
          <Input
            id="add_name"
            type="text"
            bind:value={addName}
            placeholder="e.g., US Dollar"
            required
          />
        </div>

        <!-- Symbol Position -->
        <div class="flex flex-col gap-1.5">
          <label for="add_position" class="text-xs font-semibold text-muted-foreground">
            {getLocale() === "ar" ? "موضع الرمز" : "Symbol Position"}
          </label>
          <Select type="single" value={addSymbolPosition} onValueChange={(v) => (addSymbolPosition = v ?? "before")}>
            <SelectTrigger class="h-9 w-full text-xs">
              {addSymbolPosition === "before"
                ? (getLocale() === "ar" ? "قبل المبلغ ($10)" : "Before Amount ($10)")
                : (getLocale() === "ar" ? "بعد المبلغ (10﷼)" : "After Amount (10﷼)")}
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="before">{getLocale() === "ar" ? "قبل المبلغ ($10)" : "Before Amount ($10)"}</SelectItem>
              <SelectItem value="after">{getLocale() === "ar" ? "بعد المبلغ (10﷼)" : "After Amount (10﷼)"}</SelectItem>
            </SelectContent>
          </Select>
        </div>

        <!-- Decimal Places -->
        <div class="flex flex-col gap-1.5">
          <label for="add_decimals" class="text-xs font-semibold text-muted-foreground">
            {getLocale() === "ar" ? "الخانات العشرية" : "Decimal Places"}
          </label>
          <Input
            id="add_decimals"
            type="number"
            bind:value={addDecimalPlaces}
            min={0}
            max={8}
            required
          />
        </div>

        <!-- Exchange Rate -->
        <div class="flex flex-col gap-1.5 md:col-span-2">
          <label for="add_rate" class="text-xs font-semibold text-muted-foreground flex justify-between">
            <span>{getLocale() === "ar" ? "سعر الصرف مقابل العملة الأساسية" : "Exchange Rate"} *</span>
            <span class="text-xs font-bold text-primary">1 {addCode || 'CUR'} = {addExchangeRate} {baseCurrencyCode}</span>
          </label>
          <Input
            id="add_rate"
            type="number"
            step="0.000001"
            bind:value={addExchangeRate}
            min={0.000001}
            required
            class="font-mono font-bold"
          />
        </div>
      </div>

      <!-- Toggles -->
      <div class="flex flex-col gap-3 border-t border-border/40 pt-4 mt-2">
        <div class="flex items-center justify-between">
          <div class="flex flex-col gap-0.5">
            <span class="text-xs font-semibold text-foreground">{getLocale() === "ar" ? "عملة أساسية" : "Base Currency"}</span>
            <span class="text-[9px] text-muted-foreground">
              {getLocale() === "ar" ? "تحديد ما إذا كانت هذه هي العملة الأساسية للتقارير" : "Set this as the main base currency"}
            </span>
          </div>
          <Switch bind:checked={addIsBase} />
        </div>

        <div class="flex items-center justify-between border-t border-border/20 pt-3">
          <div class="flex flex-col gap-0.5">
            <span class="text-xs font-semibold text-foreground">{getLocale() === "ar" ? "حالة النشاط" : "Active"}</span>
            <span class="text-[9px] text-muted-foreground">
              {getLocale() === "ar" ? "تفعيل العملة لاستخدامها في المعاملات المالية" : "Enable currency usage in transactions"}
            </span>
          </div>
          <Switch bind:checked={addIsActive} />
        </div>
      </div>

      <div class="flex items-center justify-end gap-2 border-t border-border/40 pt-4 mt-2">
        <Button variant="ghost" class="text-xs h-8 cursor-pointer" onclick={() => showAddDialog = false} type="button">
          {getLocale() === "ar" ? "إلغاء" : "Cancel"}
        </Button>
        <Button variant="lime" class="text-xs h-8 font-bold cursor-pointer" type="submit" disabled={isSubmitting}>
          {isSubmitting ? (getLocale() === "ar" ? "جاري الإضافة..." : "Adding...") : (getLocale() === "ar" ? "حفظ وإضافة" : "Save & Add")}
        </Button>
      </div>
    </form>
  </div>
{/if}

<!-- Edit Currency Modal -->
{#if showEditDialog}
  <div 
    in:fade={{ duration: 150 }} 
    class="fixed inset-0 z-50 bg-black/60 backdrop-blur-sm flex items-center justify-center p-4"
  >
    <form 
      onsubmit={handleEditCurrency}
      in:slide={{ duration: 200 }} 
      class="bg-card border border-border rounded-2xl w-full max-w-lg p-6 flex flex-col gap-5 text-start shadow-2xl overflow-y-auto max-h-[90vh]"
    >
      <div class="flex flex-col gap-1 border-b border-border/40 pb-3">
        <h2 class="text-sm font-bold text-foreground flex items-center gap-1.5">
          <Edit2 size={16} class="text-primary" />
          <span>{getLocale() === "ar" ? `تعديل العملة (${editCode})` : `Edit Currency (${editCode})`}</span>
        </h2>
        <p class="text-[10px] text-muted-foreground">
          {getLocale() === "ar" 
            ? "قم بتعديل بيانات العملة المحددة وسعر صرفها اليومي." 
            : "Modify the configuration details for this currency."}
        </p>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <!-- Currency Code (Read Only) -->
        <div class="flex flex-col gap-1.5">
          <label for="edit_code" class="text-xs font-semibold text-muted-foreground">
            {getLocale() === "ar" ? "رمز العملة" : "Currency Code"}
          </label>
          <Input
            id="edit_code"
            type="text"
            value={editCode}
            disabled
            class="font-mono font-bold bg-muted/60 opacity-80"
          />
        </div>

        <!-- Symbol -->
        <div class="flex flex-col gap-1.5">
          <label for="edit_symbol" class="text-xs font-semibold text-muted-foreground">
            {getLocale() === "ar" ? "رمز / علامة العملة" : "Symbol"} *
          </label>
          <Input
            id="edit_symbol"
            type="text"
            bind:value={editSymbol}
            placeholder="$"
            required
            class="font-mono font-bold"
          />
        </div>

        <!-- Arabic Name -->
        <div class="flex flex-col gap-1.5">
          <label for="edit_name_ar" class="text-xs font-semibold text-muted-foreground">
            {getLocale() === "ar" ? "الاسم بالعربية" : "Arabic Name"}
          </label>
          <Input
            id="edit_name_ar"
            type="text"
            bind:value={editNameAr}
            placeholder="مثال: دولار أمريكي"
          />
        </div>

        <!-- English Name -->
        <div class="flex flex-col gap-1.5">
          <label for="edit_name" class="text-xs font-semibold text-muted-foreground">
            {getLocale() === "ar" ? "الاسم بالإنجليزية" : "English Name"} *
          </label>
          <Input
            id="edit_name"
            type="text"
            bind:value={editName}
            placeholder="e.g., US Dollar"
            required
          />
        </div>

        <!-- Symbol Position -->
        <div class="flex flex-col gap-1.5">
          <label for="edit_position" class="text-xs font-semibold text-muted-foreground">
            {getLocale() === "ar" ? "موضع الرمز" : "Symbol Position"}
          </label>
          <Select type="single" value={editSymbolPosition} onValueChange={(v) => (editSymbolPosition = v ?? "before")}>
            <SelectTrigger class="h-9 w-full text-xs">
              {editSymbolPosition === "before"
                ? (getLocale() === "ar" ? "قبل المبلغ ($10)" : "Before Amount ($10)")
                : (getLocale() === "ar" ? "بعد المبلغ (10﷼)" : "After Amount (10﷼)")}
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="before">{getLocale() === "ar" ? "قبل المبلغ ($10)" : "Before Amount ($10)"}</SelectItem>
              <SelectItem value="after">{getLocale() === "ar" ? "بعد المبلغ (10﷼)" : "After Amount (10﷼)"}</SelectItem>
            </SelectContent>
          </Select>
        </div>

        <!-- Decimal Places -->
        <div class="flex flex-col gap-1.5">
          <label for="edit_decimals" class="text-xs font-semibold text-muted-foreground">
            {getLocale() === "ar" ? "الخانات العشرية" : "Decimal Places"}
          </label>
          <Input
            id="edit_decimals"
            type="number"
            bind:value={editDecimalPlaces}
            min={0}
            max={8}
            required
          />
        </div>

        <!-- Exchange Rate -->
        <div class="flex flex-col gap-1.5 md:col-span-2">
          <label for="edit_rate" class="text-xs font-semibold text-muted-foreground flex justify-between">
            <span>{getLocale() === "ar" ? "سعر الصرف مقابل العملة الأساسية" : "Exchange Rate"} *</span>
            {#if !editIsBase}
              <span class="text-xs font-bold text-primary">1 {editCode} = {editExchangeRate} {baseCurrencyCode}</span>
            {/if}
          </label>
          <Input
            id="edit_rate"
            type="number"
            step="0.000001"
            bind:value={editExchangeRate}
            min={0.000001}
            disabled={editIsBase}
            required
            class="font-mono font-bold disabled:opacity-60"
          />
        </div>
      </div>

      <!-- Toggles -->
      <div class="flex flex-col gap-3 border-t border-border/40 pt-4 mt-2">
        <div class="flex items-center justify-between">
          <div class="flex flex-col gap-0.5">
            <span class="text-xs font-semibold text-foreground">{getLocale() === "ar" ? "عملة أساسية" : "Base Currency"}</span>
            <span class="text-[9px] text-muted-foreground">
              {getLocale() === "ar" ? "تحديد ما إذا كانت هذه هي العملة الأساسية للتقارير" : "Set this as the main base currency"}
            </span>
          </div>
          <Switch bind:checked={editIsBase} disabled={editIsBase && editCode === 'YER' /* prevent changing seed base YER directly */} />
        </div>

        <div class="flex items-center justify-between border-t border-border/20 pt-3">
          <div class="flex flex-col gap-0.5">
            <span class="text-xs font-semibold text-foreground">{getLocale() === "ar" ? "حالة النشاط" : "Active"}</span>
            <span class="text-[9px] text-muted-foreground">
              {getLocale() === "ar" ? "تفعيل العملة لاستخدامها في المعاملات المالية" : "Enable currency usage in transactions"}
            </span>
          </div>
          <Switch bind:checked={editIsActive} disabled={editIsBase /* Base currency must be active */} />
        </div>
      </div>

      <div class="flex items-center justify-end gap-2 border-t border-border/40 pt-4 mt-2">
        <Button variant="ghost" class="text-xs h-8 cursor-pointer" onclick={() => showEditDialog = false} type="button">
          {getLocale() === "ar" ? "إلغاء" : "Cancel"}
        </Button>
        <Button variant="lime" class="text-xs h-8 font-bold cursor-pointer" type="submit" disabled={isSubmitting}>
          {isSubmitting ? (getLocale() === "ar" ? "جاري التحديث..." : "Updating...") : (getLocale() === "ar" ? "حفظ التغييرات" : "Save Changes")}
        </Button>
      </div>
    </form>
  </div>
{/if}

<!-- Delete Confirmation Dialog -->
{#if currencyToDelete}
  <div 
    in:fade={{ duration: 150 }} 
    class="fixed inset-0 z-50 bg-black/60 backdrop-blur-sm flex items-center justify-center p-4"
  >
    <div 
      in:slide={{ duration: 200 }} 
      class="bg-card border border-border rounded-2xl w-full max-w-md p-6 flex flex-col gap-4 text-start shadow-2xl"
    >
      <div class="flex flex-col gap-1">
        <h3 class="text-sm font-bold text-destructive flex items-center gap-1.5">
          <Trash2 size={16} />
          <span>{getLocale() === "ar" ? "تأكيد حذف العملة" : "Confirm Currency Deletion"}</span>
        </h3>
        <p class="text-xs text-foreground mt-2 font-semibold">
          {getLocale() === "ar"
            ? `هل أنت متأكد من رغبتك في حذف عملة (${currencyToDelete.code} - ${currencyToDelete.name_ar || currencyToDelete.name}) نهائياً؟`
            : `Are you sure you want to permanently delete currency (${currencyToDelete.code} - ${currencyToDelete.name_ar || currencyToDelete.name})?`}
        </p>
        <p class="text-[10px] text-muted-foreground leading-relaxed mt-1">
          {getLocale() === "ar"
            ? "تنبيه: سيؤدي حذف العملة إلى عدم إمكانية استخدامها في أي عمليات أو رسوم جديدة. لن تتمكن من حذفها إذا كانت مرتبطة بمعاملات مسجلة بالفعل في النظام المالي."
            : "Warning: Deleting this currency will make it unavailable for new operations or fees. It cannot be deleted if there are existing transactions referencing it."}
        </p>
      </div>

      <div class="flex items-center justify-end gap-2 border-t border-border/40 pt-4 mt-2">
        <Button variant="ghost" class="text-xs h-8 cursor-pointer" onclick={() => currencyToDelete = null}>
          {getLocale() === "ar" ? "تراجع" : "Cancel"}
        </Button>
        <Button variant="destructive" class="text-xs h-8 font-bold cursor-pointer" onclick={handleDeleteCurrency}>
          {getLocale() === "ar" ? "تأكيد الحذف" : "Confirm Delete"}
        </Button>
      </div>
    </div>
  </div>
{/if}
