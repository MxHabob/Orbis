<script lang="ts">
import { onMount } from "svelte";
import { commands } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import { Select, SelectTrigger, SelectContent, SelectItem } from "@orbis/ui-svelte/select";
import { Tabs, TabsList, TabsTrigger, TabsContent } from "@orbis/ui-svelte/tabs";
import { ComboboxDropdown } from "@orbis/ui-svelte/combobox-dropdown";
import { toast } from "svelte-sonner";
import * as m from "$lib/paraglide/messages";

let {
  orgId,
  onSuccess,
  onCancel,
  defaultTab = "gl",
}: {
  orgId: string;
  onSuccess: () => void;
  onCancel: () => void;
  defaultTab?: "gl" | "treasury" | "entity";
} = $props();

// Global data state
let ledgers = $state<any[]>([]);
let currencies = $state<any[]>([]);
let isLoading = $state(true);
let activeTab = $state<"gl" | "treasury" | "entity">(defaultTab);
let isSubmitting = $state(false);

// GL Account States
let glParentId = $state("");
let glNameAr = $state("");
let glNameEn = $state("");
let glAccountType = $state("asset");
let glOpeningBalance = $state("0");

// Treasury States
let trNameAr = $state("");
let trNameEn = $state("");
let trType = $state<"cash" | "bank">("cash");
let trBankName = $state("");
let trAccountNumber = $state("");

// Entity States
let enType = $state<"vendor" | "partner" | "employee">("vendor");
let enNameAr = $state("");
let enNameEn = $state("");
let enOpeningBalance = $state("0");

// Load Chart of Accounts & Currencies
async function loadFormData() {
  try {
    isLoading = true;
    const [ledgerList, currencyList] = await Promise.all([
      commands.finance.getLedgerAccounts(orgId),
      commands.finance.getCurrencies(orgId),
    ]);
    ledgers = ledgerList;
    currencies = currencyList;
  } catch (e) {
    console.error("Error loading form data:", e);
    toast.error("حدث خطأ أثناء تحميل البيانات");
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (orgId) {
    loadFormData();
  }
});

// Organization Base Currency Helper
const baseCurrency = $derived.by(() => {
  return currencies.find((c) => c.is_base === 1)?.code || "YER";
});

// Derived Combobox Items (Only non-root control accounts or level > 1)
const parentItems = $derived.by(() => {
  return ledgers
    .filter((l) => l.depth > 0) // Exclude root Level 1 accounts
    .map((l) => ({
      id: l.id,
      label: `${l.code} - ${l.name_ar || l.name}`,
      value: l.id,
      code: l.code,
      type: l.item_type,
    }));
});

const selectedParentItem = $derived(
  parentItems.find((item) => item.value === glParentId)
);

function handleParentSelect(item: { id: string; label: string; value: string }) {
  glParentId = item.value;
  // Inherit account type from selected parent
  const parentAcc = ledgers.find(l => l.id === item.value);
  if (parentAcc) {
    glAccountType = parentAcc.item_type;
  }
}

// Reset form fields on tab change
function handleTabChange(tab: "gl" | "treasury" | "entity") {
  activeTab = tab;
}

async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();
  isSubmitting = true;

  try {
    if (activeTab === "gl") {
      if (!glParentId) {
        toast.error("يرجى اختيار الحساب الأب");
        isSubmitting = false;
        return;
      }
      if (!glNameAr && !glNameEn) {
        toast.error("يرجى إدخال اسم الحساب (بالعربية أو الإنجليزية)");
        isSubmitting = false;
        return;
      }

      const parent = ledgers.find((l) => l.id === glParentId);
      if (!parent) return;

      const request = {
        code: "", // Backend will auto-generate code sequentially under parent
        name: glNameEn || glNameAr,
        name_ar: glNameAr || glNameEn,
        item_type: glAccountType,
        currency: "", // Backend will inherit parent currency
        opening_balance: parseFloat(glOpeningBalance) || 0.0,
        parent_id: glParentId,
        sub_type: undefined,
      };

      await commands.finance.createLedgerAccount(orgId, request);
      toast.success(m.ledger_toast_success());
      
      // Reset
      glParentId = "";
      glNameAr = "";
      glNameEn = "";
      glOpeningBalance = "0";
      
    } else if (activeTab === "treasury") {
      if (!trNameAr && !trNameEn) {
        toast.error("يرجى إدخال اسم الخزينة / البنك");
        isSubmitting = false;
        return;
      }

      const request = {
        name: trNameEn || trNameAr,
        name_ar: trNameAr || trNameEn,
        item_type: trType,
        currency: baseCurrency, // Default treasury to base currency
        bank_name: trType === "bank" ? trBankName.trim() || null : null,
        account_number: trType === "bank" ? trAccountNumber.trim() || null : null,
        ledger_account_id: undefined,
      };

      await commands.finance.createTreasury(orgId, request);
      toast.success("تم إنشاء الصندوق/البنك بنجاح");
      
      // Reset
      trNameAr = "";
      trNameEn = "";
      trBankName = "";
      trAccountNumber = "";
      
    } else if (activeTab === "entity") {
      if (!enNameAr && !enNameEn) {
        toast.error("يرجى إدخال اسم الكيان المالي");
        isSubmitting = false;
        return;
      }

      const request = {
        code: "", // Backend will auto-generate code sequentially under appropriate parent
        name: enNameEn || enNameAr,
        name_ar: enNameAr || enNameEn,
        item_type: "", // Backend resolves from parent
        currency: "", // Backend resolves from parent
        opening_balance: parseFloat(enOpeningBalance) || 0.0,
        parent_id: undefined, // Backend resolves from sub_type
        sub_type: enType,
      };

      await commands.finance.createLedgerAccount(orgId, request);
      toast.success("تم إنشاء الحساب المساعد للكيان بنجاح");

      // Reset
      enNameAr = "";
      enNameEn = "";
      enOpeningBalance = "0";
    }

    onSuccess();
  } catch (e) {
    const errorMsg = e instanceof Error ? e.message : String(e);
    toast.error(m.ledger_toast_error_submit({ error: errorMsg }));
  } finally {
    isSubmitting = false;
    loadFormData(); // Refresh lists
  }
}
</script>

<div class="flex-1 flex flex-col h-full overflow-hidden">
  <Tabs defaultPreset={activeTab} class="w-full flex-1 flex flex-col overflow-hidden" onValueChange={(v) => handleTabChange(v as any)}>
    <TabsList class="grid w-full grid-cols-3 select-none shrink-0 mb-4 bg-muted/60 p-1 rounded-lg">
      <TabsTrigger value="gl" class="text-xs font-semibold py-1.5 cursor-pointer rounded-md transition-all">
        {m.ledger_tab_gl()}
      </TabsTrigger>
      <TabsTrigger value="treasury" class="text-xs font-semibold py-1.5 cursor-pointer rounded-md transition-all">
        {m.ledger_tab_treasury()}
      </TabsTrigger>
      <TabsTrigger value="entity" class="text-xs font-semibold py-1.5 cursor-pointer rounded-md transition-all">
        {m.ledger_tab_entity()}
      </TabsTrigger>
    </TabsList>

    <form onsubmit={handleSubmit} class="flex-1 overflow-y-auto px-1 flex flex-col gap-5 scrollbar-hide">
      <!-- GL Account Form Content -->
      <TabsContent value="gl" class="flex flex-col gap-4 mt-0 animate-in fade-in-50 duration-150">
        <!-- Parent Account Select -->
        <div class="flex flex-col gap-1.5 text-start">
          <Label class="text-xs font-bold text-foreground">
            {m.ledger_label_parent()}
          </Label>
          <ComboboxDropdown
            placeholder={m.ledger_placeholder_parent()}
            searchPlaceholder="ابحث عن الحساب الرئيسي..."
            selectedItem={selectedParentItem}
            items={parentItems}
            onSelect={handleParentSelect}
            class="w-full text-xs"
            disabled={isSubmitting || isLoading}
          />
        </div>

        <!-- Account Names -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div class="flex flex-col gap-1.5 text-start">
            <Label for="gl_name_ar" class="text-xs font-bold text-foreground">
              {m.ledger_label_name_ar()}
            </Label>
            <Input
              id="gl_name_ar"
              type="text"
              bind:value={glNameAr}
              placeholder="مثال: رسوم الأنشطة الترفيهية"
              required={!glNameEn}
              disabled={isSubmitting}
            />
          </div>
          <div class="flex flex-col gap-1.5 text-start">
            <Label for="gl_name_en" class="text-xs font-bold text-foreground">
              {m.ledger_label_name_en()}
            </Label>
            <Input
              id="gl_name_en"
              type="text"
              bind:value={glNameEn}
              placeholder="e.g., Recreational Activities Fees"
              required={!glNameAr}
              disabled={isSubmitting}
            />
          </div>
        </div>

        <!-- Inherited fields (Type only) -->
        {#if glParentId}
          <div class="bg-muted/20 p-3 rounded-lg border border-border/50 animate-in fade-in duration-150">
            <div class="flex flex-col gap-1.5 text-start">
              <span class="text-[10px] text-muted-foreground uppercase font-bold tracking-wider">
                {m.ledger_label_type()}
              </span>
              <span class="text-xs font-semibold text-foreground capitalize">
                {glAccountType}
              </span>
            </div>
          </div>
        {/if}

        <!-- Opening Balance -->
        <div class="flex flex-col gap-1.5 text-start">
          <Label for="gl_open_bal" class="text-xs font-bold text-foreground">
            {m.ledger_label_opening_balance()}
          </Label>
          <Input
            id="gl_open_bal"
            type="number"
            step="any"
            bind:value={glOpeningBalance}
            placeholder="0.00"
            disabled={isSubmitting}
          />
        </div>
      </TabsContent>

      <!-- Treasury Form Content -->
      <TabsContent value="treasury" class="flex flex-col gap-4 mt-0 animate-in fade-in-50 duration-150">
        <!-- Treasury Names -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div class="flex flex-col gap-1.5 text-start">
            <Label for="tr_name_ar" class="text-xs font-bold text-foreground">
              الاسم (عربي)
            </Label>
            <Input
              id="tr_name_ar"
              type="text"
              bind:value={trNameAr}
              placeholder="مثال: خزينة الإدارة العامة"
              required={!trNameEn}
              disabled={isSubmitting}
            />
          </div>
          <div class="flex flex-col gap-1.5 text-start">
            <Label for="tr_name_en" class="text-xs font-bold text-foreground">
              الاسم (إنجليزي)
            </Label>
            <Input
              id="tr_name_en"
              type="text"
              bind:value={trNameEn}
              placeholder="e.g., General Management Cashbox"
              required={!trNameAr}
              disabled={isSubmitting}
            />
          </div>
        </div>

        <!-- Treasury Type -->
        <div class="flex flex-col gap-1.5 text-start">
          <Label class="text-xs font-bold text-foreground">
            {m.ledger_label_treasury_type()}
          </Label>
          <Select bind:value={trType} type="single">
            <SelectTrigger class="w-full text-xs text-start flex items-center justify-between bg-background">
              <span class="truncate">
                {trType === "cash" ? m.ledger_type_cash() : m.ledger_type_bank()}
              </span>
            </SelectTrigger>
            <SelectContent class="text-start">
              <SelectItem value="cash" class="text-xs cursor-pointer">{m.ledger_type_cash()}</SelectItem>
              <SelectItem value="bank" class="text-xs cursor-pointer">{m.ledger_type_bank()}</SelectItem>
            </SelectContent>
          </Select>
        </div>

        <!-- Dynamic Bank inputs -->
        {#if trType === "bank"}
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4 animate-in fade-in duration-150">
            <div class="flex flex-col gap-1.5 text-start">
              <Label for="tr_bank_name" class="text-xs font-bold text-foreground">
                {m.ledger_label_bank_name()}
              </Label>
              <Input
                id="tr_bank_name"
                type="text"
                bind:value={trBankName}
                placeholder={m.ledger_placeholder_bank_name()}
                required
                disabled={isSubmitting}
              />
            </div>
            <div class="flex flex-col gap-1.5 text-start">
              <Label for="tr_acc_num" class="text-xs font-bold text-foreground">
                {m.ledger_label_account_number()}
              </Label>
              <Input
                id="tr_acc_num"
                type="text"
                bind:value={trAccountNumber}
                placeholder={m.ledger_placeholder_account_number()}
                required
                disabled={isSubmitting}
              />
            </div>
          </div>
        {/if}
      </TabsContent>

      <!-- Entity Form Content -->
      <TabsContent value="entity" class="flex flex-col gap-4 mt-0 animate-in fade-in-50 duration-150">
        <!-- Entity Type Selection -->
        <div class="flex flex-col gap-1.5 text-start">
          <Label class="text-xs font-bold text-foreground">
            {m.ledger_label_entity_type()}
          </Label>
          <Select bind:value={enType} type="single">
            <SelectTrigger class="w-full text-xs text-start flex items-center justify-between bg-background">
              <span class="truncate">
                {enType === "vendor" ? m.ledger_entity_vendor() : enType === "partner" ? m.ledger_entity_partner() : m.ledger_entity_employee()}
              </span>
            </SelectTrigger>
            <SelectContent class="text-start">
              <SelectItem value="vendor" class="text-xs cursor-pointer">{m.ledger_entity_vendor()}</SelectItem>
              <SelectItem value="partner" class="text-xs cursor-pointer">{m.ledger_entity_partner()}</SelectItem>
              <SelectItem value="employee" class="text-xs cursor-pointer">{m.ledger_entity_employee()}</SelectItem>
            </SelectContent>
          </Select>
        </div>

        <!-- Entity Names -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div class="flex flex-col gap-1.5 text-start">
            <Label for="en_name_ar" class="text-xs font-bold text-foreground">
              الاسم (عربي)
            </Label>
            <Input
              id="en_name_ar"
              type="text"
              bind:value={enNameAr}
              placeholder="مثال: شركة آفاق للمقاولات"
              required={!enNameEn}
              disabled={isSubmitting}
            />
          </div>
          <div class="flex flex-col gap-1.5 text-start">
            <Label for="en_name_en" class="text-xs font-bold text-foreground">
              الاسم (إنجليزي)
            </Label>
            <Input
              id="en_name_en"
              type="text"
              bind:value={enNameEn}
              placeholder="e.g., Afaq Contracting Company"
              required={!enNameAr}
              disabled={isSubmitting}
            />
          </div>
        </div>

        <!-- Opening Balance -->
        <div class="flex flex-col gap-1.5 text-start">
          <Label for="en_open_bal" class="text-xs font-bold text-foreground">
            {m.ledger_label_opening_balance()}
          </Label>
          <Input
            id="en_open_bal"
            type="number"
            step="any"
            bind:value={enOpeningBalance}
            placeholder="0.00"
            disabled={isSubmitting}
          />
        </div>
      </TabsContent>

      <!-- Form Actions (Footer) -->
      <div class="flex gap-3 justify-end mt-auto pt-6 border-t border-border/50 shrink-0">
        <Button
          type="button"
          variant="ghost"
          onclick={onCancel}
          disabled={isSubmitting}
          class="text-xs h-9"
        >
          {m.ledger_button_cancel()}
        </Button>
        <Button
          type="submit"
          variant="lime"
          class="font-bold text-xs h-9 px-5"
          disabled={isSubmitting || (activeTab === "gl" && !glParentId)}
        >
          {isSubmitting ? m.loading() : m.ledger_btn_add_account()}
        </Button>
      </div>
    </form>
  </Tabs>
</div>
