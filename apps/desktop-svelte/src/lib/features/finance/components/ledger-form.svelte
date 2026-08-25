<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { commands } from "$lib/core/ipc";
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
import { SheetFooter } from "@orbis/ui-svelte/sheet";
import { toast } from "svelte-sonner";
import { BookOpen, Landmark, Users, Info, Check, Loader2 } from "@lucide/svelte";

let {
  orgId,
  onSuccess,
  onCancel,
  defaultTab = "gl",
}: {
  orgId: string;
  onSuccess: () => void;
  onCancel: () => void;
  defaultTab?: "gl" | "cash_account" | "entity";
} = $props();

// Global data state
let ledgers = $state<any[]>([]);
let currencies = $state<any[]>([]);
let isLoading = $state(true);
let activeTab = $state<"gl" | "cash_account" | "entity">("gl");
let isSubmitting = $state(false);

// GL Account States
let glParentId = $state("");
let glNameAr = $state("");
let glNameEn = $state("");
let glAccountType = $state("asset");

// Cash and bank account state
let trNameAr = $state("");
let trNameEn = $state("");
let trType = $state<"cash" | "bank">("cash");

// Entity States
let enType = $state<"vendor" | "partner" | "employee">("vendor");
let enNameAr = $state("");
let enNameEn = $state("");

// Load Chart of Accounts & Currencies
async function loadFormData() {
  try {
    isLoading = true;
    const [ledgerList, currencyList] = await Promise.all([
      commands.finance.listAccounts(),
      commands.finance.listCurrencies(),
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
    activeTab = defaultTab;
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
  parentItems.find((item) => item.value === glParentId),
);

function handleParentSelect(item: {
  id: string;
  label: string;
  value: string;
}) {
  glParentId = item.value;
  // Inherit account type from selected parent
  const parentAcc = ledgers.find((l) => l.id === item.value);
  if (parentAcc) {
    glAccountType = parentAcc.item_type;
  }
}

// Reset form fields on tab change
function handleTabChange(tab: "gl" | "cash_account" | "entity") {
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
        parent_id: glParentId,
        sub_type: undefined,
      };

      await commands.finance.createAccount(request);
      toast.success(m.common_ledger_toast_success());

      // Reset
      glParentId = "";
      glNameAr = "";
      glNameEn = "";
    } else if (activeTab === "cash_account") {
      if (!trNameAr && !trNameEn) {
        toast.error(m.common_cash_account_name_required());
        isSubmitting = false;
        return;
      }

      const request = {
        name: trNameEn || trNameAr,
        name_ar: trNameAr || trNameEn,
        item_type: "asset",
        currency: baseCurrency,
        code: "",
        sub_type: trType,
        parent_id: undefined,
      };

      await commands.finance.createAccount(request);
      toast.success(m.common_cash_account_created());

      // Reset
      trNameAr = "";
      trNameEn = "";
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
        parent_id: undefined, // Backend resolves from sub_type
        sub_type: enType,
      };

      await commands.finance.createAccount(request);
      toast.success("تم إنشاء الحساب المساعد للكيان بنجاح");

      // Reset
      enNameAr = "";
      enNameEn = "";
    }

    onSuccess();
  } catch (e) {
    const errorMsg = e instanceof Error ? e.message : String(e);
    toast.error(m.common_ledger_toast_error_submit({ error: errorMsg }));
  } finally {
    isSubmitting = false;
    loadFormData(); // Refresh lists
  }
}
</script>

<div class="flex h-full flex-col overflow-hidden">
  <form onsubmit={handleSubmit} class="flex min-h-full flex-col gap-5 px-1">
    <div class="flex flex-col gap-1.5 text-start">
      <Label for="account_type" class="text-xs font-bold text-foreground">
        {m.common_ledger_label_type()}
      </Label>
      <Select type="single" value={activeTab} onValueChange={(value) => handleTabChange(value as typeof activeTab)}>
        <SelectTrigger id="account_type" class="h-9 w-full text-xs text-start">
          <span class="flex items-center gap-2">
            {#if activeTab === "gl"}
              <BookOpen size={15} class="shrink-0 text-primary" />
              {m.common_ledger_tab_gl()}
            {:else if activeTab === "cash_account"}
              <Landmark size={15} class="shrink-0 text-primary" />
              {m.common_ledger_tab_cash_account()}
            {:else}
              <Users size={15} class="shrink-0 text-primary" />
              {m.common_ledger_tab_entity()}
            {/if}
          </span>
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="gl" class="text-xs cursor-pointer">{m.common_ledger_tab_gl()}</SelectItem>
          <SelectItem value="cash_account" class="text-xs cursor-pointer">{m.common_ledger_tab_cash_account()}</SelectItem>
          <SelectItem value="entity" class="text-xs cursor-pointer">{m.common_ledger_tab_entity()}</SelectItem>
        </SelectContent>
      </Select>
    </div>

    {#if activeTab === "gl"}
      <!-- GL Account Form Content -->
      <div class="flex flex-col gap-4 animate-in fade-in-50 duration-150">
        <!-- Parent Account Select -->
        <div class="flex flex-col gap-1.5 text-start">
          <Label class="text-xs font-bold text-foreground">
            {m.common_ledger_label_parent()}
          </Label>
          <ComboboxDropdown
            placeholder={m.common_ledger_placeholder_parent()}
            searchPlaceholder="ابحث عن الحساب الرئيسي..."
            selectedItem={selectedParentItem}
            items={parentItems}
            onSelect={handleParentSelect}
            class="w-full text-xs"
          />
        </div>

        <!-- Account Names -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div class="flex flex-col gap-1.5 text-start">
            <Label for="gl_name_ar" class="text-xs font-bold text-foreground">
              {m.common_ledger_label_name_ar()}
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
              {m.common_ledger_label_name_en()}
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
                {m.common_ledger_label_type()}
              </span>
              <span class="text-xs font-semibold text-foreground capitalize">
                {glAccountType}
              </span>
            </div>
          </div>
        {/if}

      </div>

    {:else if activeTab === "cash_account"}
      <!-- Treasury Form Content -->
      <div class="flex flex-col gap-4 animate-in fade-in-50 duration-150">
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
              placeholder="مثال: الصندوق الرئيسي أو الحساب البنكي الرئيسي"
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
            {m.common_ledger_label_cash_account_type()}
          </Label>
          <Select bind:value={trType} type="single">
            <SelectTrigger class="w-full text-xs text-start flex items-center justify-between bg-background">
              <span class="truncate">
                {trType === "cash" ? m.common_ledger_type_cash() : m.common_cash_account_type_bank()}
              </span>
            </SelectTrigger>
            <SelectContent class="text-start">
              <SelectItem value="cash" class="text-xs cursor-pointer">{m.common_ledger_type_cash()}</SelectItem>
              <SelectItem value="bank" class="text-xs cursor-pointer">{m.common_cash_account_type_bank()}</SelectItem>
            </SelectContent>
          </Select>
        </div>

        <div class="flex items-center gap-2 rounded-lg border border-border/50 bg-muted/20 px-3 py-2.5 text-[11px] text-muted-foreground">
          <Info size={14} class="shrink-0 text-primary" />
          <span>{m.common_ledger_label_type()}: <strong class="text-foreground">asset</strong> · {m.common_ledger_col_code()}: <strong class="font-mono text-foreground">{baseCurrency}</strong></span>
        </div>

      </div>

    {:else}
      <!-- Entity Form Content -->
      <div class="flex flex-col gap-4 animate-in fade-in-50 duration-150">
        <!-- Entity Type Selection -->
        <div class="flex flex-col gap-1.5 text-start">
          <Label class="text-xs font-bold text-foreground">
            {m.common_print_multi_up_entity_type()}
          </Label>
          <Select bind:value={enType} type="single">
            <SelectTrigger class="w-full text-xs text-start flex items-center justify-between bg-background">
              <span class="truncate">
                {enType === "vendor" ? m.common_ledger_entity_vendor() : enType === "partner" ? m.common_ledger_entity_partner() : m.common_ledger_entity_employee()}
              </span>
            </SelectTrigger>
            <SelectContent class="text-start">
              <SelectItem value="vendor" class="text-xs cursor-pointer">{m.common_ledger_entity_vendor()}</SelectItem>
              <SelectItem value="partner" class="text-xs cursor-pointer">{m.common_ledger_entity_partner()}</SelectItem>
              <SelectItem value="employee" class="text-xs cursor-pointer">{m.common_ledger_entity_employee()}</SelectItem>
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

      </div>
    {/if}

      <!-- Form Actions (Footer) -->
      <SheetFooter class="mt-auto flex shrink-0 flex-row justify-end gap-3 border-t border-border/50 px-0 pb-0 pt-5">
        <Button
          type="button"
          onclick={onCancel}
          disabled={isSubmitting}
          variant="outline"
          class="text-xs h-8 cursor-pointer"
        >
          {m.common_action_cancel()}
        </Button>
        <Button
          type="submit"
          variant="lime"
          class="font-bold text-xs h-8 px-4 cursor-pointer gap-1.5"
          disabled={isSubmitting || (activeTab === "gl" && !glParentId)}
        >
          {#if isSubmitting}
            <Loader2 size={13} class="animate-spin" />
          {:else}
            <Check size={13} />
          {/if}
          <span>{isSubmitting ? m.common_loading() : m.common_ledger_btn_add_account()}</span>
        </Button>
      </SheetFooter>
  </form>
</div>
