<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { untrack } from "svelte";
import { api, queryKeys } from "$lib/api";
import { useQueryClient } from "@tanstack/svelte-query";
import { createAccountsQuery } from "../queries";
import { orgContext } from "$lib/stores/org-context.svelte";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import SearchableSelect from "$lib/shared/components/searchable-select.svelte";
import { toast } from "@orbis/ui";
import {
  cashAccountInputSchema,
  entityAccountInputSchema,
  generalLedgerAccountInputSchema,
} from "@orbis/schemas";
import { FormController } from "$lib/shared/forms/form-controller.svelte";
import { FormField } from "@orbis/ui";
import { ArrowPath, BookOpen, BuildingLibrary, Check, Info, Users } from "@orbis/icons";

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

const queryClient = useQueryClient();

// TanStack Query for Accounts Tree
const accountsQuery = createAccountsQuery(() => orgId);

const ledgers = $derived(accountsQuery.data ?? []);
const isLoading = $derived(accountsQuery.isLoading);

// Authoritative Base Currency from Organization Store
const baseCurrency = $derived(orgContext.baseCurrency || "SAR");

let activeTab = $state<"gl" | "cash_account" | "entity">(
  untrack(() => defaultTab),
);

// Form Controllers for each tab (Initialized during component initialization)
const glForm = new FormController({
  initialValues: {
    code: "",
    name: "",
    name_ar: "",
    item_type: "asset",
    currency: orgContext.baseCurrency || "SAR",
    parent_id: "",
    sub_type: undefined as string | undefined,
  },
  schema: generalLedgerAccountInputSchema,
});

const cashForm = new FormController({
  initialValues: {
    code: "",
    name: "",
    name_ar: "",
    item_type: "asset" as const,
    currency: orgContext.baseCurrency || "SAR",
    sub_type: "cash" as "cash" | "bank",
  },
  schema: cashAccountInputSchema,
});

const entityForm = new FormController({
  initialValues: {
    code: "",
    name: "",
    name_ar: "",
    item_type: "liability",
    currency: orgContext.baseCurrency || "SAR",
    sub_type: "vendor" as "vendor" | "partner" | "employee" | "student",
  },
  schema: entityAccountInputSchema,
});

const isSubmitting = $derived(
  glForm.isSubmitting || cashForm.isSubmitting || entityForm.isSubmitting
);

// GL Account States
let glParentId = $state("");
let glCode = $state("");
let glNameAr = $state("");
let glNameEn = $state("");
let glAccountType = $state("asset");
const glAccountTypeLabel = $derived(({
  asset: m.finance_ui_account_type_asset(),
  liability: m.finance_ui_account_type_liability(),
  equity: m.finance_ui_account_type_equity(),
  revenue: m.finance_ui_account_type_revenue(),
  expense: m.finance_ui_account_type_expense(),
} as Record<string, string>)[glAccountType.toLowerCase()] ?? glAccountType);

// Cash and bank account state
let trNameAr = $state("");
let trNameEn = $state("");
let trType = $state<"cash" | "bank">("cash");

// Entity States
let enType = $state<"vendor" | "partner" | "employee" | "student">("vendor");
let enNameAr = $state("");
let enNameEn = $state("");

// Derived Combobox Items (All accounts across the Chart of Accounts)
const parentItems = $derived.by(() => {
  return [...ledgers]
    .sort((a, b) => String(a.code || "").localeCompare(String(b.code || ""), undefined, { numeric: true }))
    .map((l) => ({
      id: l.id,
      label: `${l.code} - ${l.name}`,
      value: l.id,
      code: l.code,
      type: l.account_type || "asset",
      depth: l.parent_id ?? 1,
    }));
});

const selectedParentItem = $derived(
  parentItems.find((item) => item.value === glParentId),
);

// Suggest next sequential code when parent is selected
const suggestedCode = $derived.by(() => {
  if (!glParentId) return "";
  const parent = ledgers.find((l) => l.id === glParentId);
  if (!parent) return "";

  const children = ledgers.filter((l) => l.parent_id === glParentId);
  if (children.length === 0) {
    return `${parent.code}01`;
  }

  // Find max numeric child suffix
  const childCodes = children
    .map((c) => String(c.code))
    .filter((code) => code.startsWith(parent.code));

  if (childCodes.length > 0) {
    childCodes.sort((a, b) => a.localeCompare(b, undefined, { numeric: true }));
    const lastCode = childCodes[childCodes.length - 1];
    const suffix = lastCode.slice(parent.code.length);
    const num = parseInt(suffix, 10);
    if (!isNaN(num)) {
      const nextNum = num + 1;
      const padded = String(nextNum).padStart(suffix.length, "0");
      return `${parent.code}${padded}`;
    }
  }

  return `${parent.code}${children.length + 1}`;
});

function handleParentSelect(item: {
  id: string;
  label: string;
  value: string;
}) {
  glParentId = item.value;
  // Inherit account type from selected parent
  const parentAcc = ledgers.find((l) => l.id === item.value);
  if (parentAcc) {
    glAccountType =parentAcc.account_type || "asset";
  }
}

// Reset form fields on tab change
function handleTabChange(tab: "gl" | "cash_account" | "entity") {
  activeTab = tab;
}

async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();

  try {
    if (activeTab === "gl") {
      if (!glParentId) {
        toast.error(m.common_ledger_error_parent());
        return;
      }
      if (!glNameAr && !glNameEn) {
        toast.error(m.common_ledger_error_name());
        return;
      }

      const parent = ledgers.find((l) => l.id === glParentId);
      if (!parent) return;

      const finalCode = glCode.trim() || suggestedCode;

      glForm.setField("code", finalCode);
      glForm.setField("name", glNameEn || glNameAr);
      glForm.setField("name_ar", glNameAr || glNameEn);
      glForm.setField("item_type", glAccountType);
      glForm.setField("currency", parent.currency || baseCurrency);
      glForm.setField("parent_id", glParentId);

      const success = await glForm.submit(async (payload) => {
        await api.createAccount({
          code: payload.code,
          name: payload.name,
          name_ar: payload.name_ar,
          account_type: payload.item_type || glAccountType,
          currency: payload.currency,
          parent_id: payload.parent_id || undefined,
        });
      });

      if (!success) {
        toast.error(glForm.formError || m.common_ledger_error_name());
        return;
      }
      toast.success(m.common_ledger_toast_success());

      // Reset
      glForm.reset();
      glParentId = "";
      glCode = "";
      glNameAr = "";
      glNameEn = "";
    } else if (activeTab === "cash_account") {
      if (!trNameAr && !trNameEn) {
        toast.error(m.common_cash_account_name_required());
        return;
      }

      cashForm.setField("code", "");
      cashForm.setField("name", trNameEn || trNameAr);
      cashForm.setField("name_ar", trNameAr || trNameEn);
      cashForm.setField("item_type", "asset");
      cashForm.setField("currency", baseCurrency);
      cashForm.setField("sub_type", trType);

      const success = await cashForm.submit(async (payload) => {
        await api.createAccount({
          code: payload.code || `TR-${Date.now()}`,
          name: payload.name,
          name_ar: payload.name_ar,
          account_type: "asset",
          currency: payload.currency,
        });
      });

      if (!success) {
        toast.error(cashForm.formError || m.common_cash_account_name_required());
        return;
      }
      toast.success(m.common_cash_account_created());

      // Reset
      cashForm.reset();
      trNameAr = "";
      trNameEn = "";
    } else if (activeTab === "entity") {
      if (!enNameAr && !enNameEn) {
        toast.error(m.common_ledger_error_entity_name());
        return;
      }

      const itemType = enType === "vendor" ? "liability" : enType === "partner" ? "equity" : enType === "employee" ? "liability" : "asset";

      entityForm.setField("code", "");
      entityForm.setField("name", enNameEn || enNameAr);
      entityForm.setField("name_ar", enNameAr || enNameEn);
      entityForm.setField("item_type", itemType);
      entityForm.setField("currency", baseCurrency);
      entityForm.setField("sub_type", enType);

      const success = await entityForm.submit(async (payload) => {
        await api.createAccount({
          code: payload.code || `EN-${Date.now()}`,
          name: payload.name,
          name_ar: payload.name_ar,
          account_type: itemType,
          currency: payload.currency,
        });
      });

      if (!success) {
        toast.error(entityForm.formError || m.common_ledger_error_entity_name());
        return;
      }
      toast.success(m.common_ledger_entity_created());

      // Reset
      entityForm.reset();
      enNameAr = "";
      enNameEn = "";
    }

    // Invalidate TanStack query cache to instantly refresh parent lists & accounts table across the app
    await queryClient.invalidateQueries({ queryKey: queryKeys.finance.accounts(orgId) });
    onSuccess();
  } catch (e) {
    const errorMsg = e instanceof Error ? e.message : String(e);
    toast.error(m.common_ledger_toast_error_submit({ error: errorMsg }));
  }
}
</script>

<div class="flex h-full flex-col overflow-hidden">
  <form onsubmit={handleSubmit} class="flex min-h-full flex-col gap-5 px-1">
    <FormField id="account_type" label={m.common_ledger_label_type()} required>
      {#snippet children()}
      <Select type="single" value={activeTab} onValueChange={(value: string | undefined) => handleTabChange((value ?? "gl") as typeof activeTab)}>
        <SelectTrigger id="account_type" class="h-9 w-full text-xs text-start">
          <span class="flex items-center gap-2">
            {#if activeTab === "gl"}
              <BookOpen size={15} class="shrink-0 text-primary" />
              {m.common_ledger_tab_gl()}
            {:else if activeTab === "cash_account"}
              <BuildingLibrary size={15} class="shrink-0 text-primary" />
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
      {/snippet}
    </FormField>

    {#if activeTab === "gl"}
      <!-- GL Account Form Content -->
      <div class="flex flex-col gap-4 animate-in fade-in-50 duration-150">
        <!-- Parent Account Select -->
        <FormField label={m.common_ledger_label_parent()} required>
          {#snippet children()}
          <SearchableSelect
            value={glParentId}
            placeholder={m.common_ledger_placeholder_parent()}
            searchPlaceholder={m.common_ledger_search_parent()}
            items={ledgers}
            getItemValue={(l) => String(l.id)}
            getItemLabel={(l) => `${l.code} - ${l.name}`}
            getItemSubtitle={(l) => l.code}
            getItemBadge={(l) => l.account_type || "asset"}
            onchange={(val) => {
              const item = parentItems.find((p) => p.value === val);
              if (item) handleParentSelect(item);
            }}
            class="w-full text-xs"
          />
          {/snippet}
        </FormField>

        <!-- Account Code (Auto-suggested with custom override) -->
        <FormField id="gl_code" label={m.common_ledger_col_code()}>
          {#snippet children()}
          <div class="flex flex-col gap-1">
            <Input
              id="gl_code"
              type="text"
              bind:value={glCode}
              placeholder={suggestedCode || m.common_ledger_col_code()}
              disabled={isSubmitting || !glParentId}
              class="font-mono text-xs"
            />
            {#if suggestedCode}
              <span class="text-[10px] text-muted-foreground">
                {glCode.trim() ? m.finance_ui_account_code_manual() : m.finance_ui_account_code_suggested({ code: suggestedCode })}
              </span>
            {/if}
          </div>
          {/snippet}
        </FormField>

        <!-- Account Names -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <FormField id="gl_name_ar" label={m.common_ledger_label_name_ar()} required={!glNameEn}>
            {#snippet children()}
            <Input
              id="gl_name_ar"
              type="text"
              bind:value={glNameAr}
              placeholder={m.common_ledger_name_ar()}
              required={!glNameEn}
              disabled={isSubmitting}
            />
            {/snippet}
          </FormField>
          <FormField id="gl_name_en" label={m.common_ledger_label_name_en()} required={!glNameAr}>
            {#snippet children()}
            <Input
              id="gl_name_en"
              type="text"
              bind:value={glNameEn}
              placeholder={m.common_ledger_name_en()}
              required={!glNameAr}
              disabled={isSubmitting}
            />
            {/snippet}
          </FormField>
        </div>

        <!-- Inherited fields & Account Nature -->
        {#if glParentId}
          <div class="bg-muted/20 p-3 rounded-lg border border-border/50 animate-in fade-in duration-150 grid grid-cols-2 gap-3">
            <div class="flex flex-col gap-1 text-start">
              <span class="text-[10px] text-muted-foreground uppercase font-bold tracking-wider">
                {m.common_ledger_label_type()}
              </span>
              <span class="text-xs font-semibold text-foreground capitalize">
                {glAccountTypeLabel}
              </span>
            </div>
            <div class="flex flex-col gap-1 text-start">
              <span class="text-[10px] text-muted-foreground uppercase font-bold tracking-wider">
                {m.finance_ui_account_nature()}
              </span>
              <span class="text-xs font-semibold capitalize {['asset', 'expense'].includes(glAccountType.toLowerCase()) ? 'text-blue-500' : 'text-emerald-500'}">
                {['asset', 'expense'].includes(glAccountType.toLowerCase()) ? m.finance_ui_debit() : m.finance_ui_credit()}
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
          <FormField id="tr_name_ar" label={m.common_ledger_name_ar()} required={!trNameEn}>
            {#snippet children()}
            <Input
              id="tr_name_ar"
              type="text"
              bind:value={trNameAr}
              placeholder={m.common_ledger_name_ar()}
              required={!trNameEn}
              disabled={isSubmitting}
            />
            {/snippet}
          </FormField>
          <FormField id="tr_name_en" label={m.common_ledger_name_en()} required={!trNameAr}>
            {#snippet children()}
            <Input
              id="tr_name_en"
              type="text"
              bind:value={trNameEn}
              placeholder={m.common_ledger_name_en()}
              required={!trNameAr}
              disabled={isSubmitting}
            />
            {/snippet}
          </FormField>
        </div>

        <!-- Treasury Type -->
        <FormField label={m.common_ledger_label_cash_account_type()} required>
          {#snippet children()}
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
          {/snippet}
        </FormField>

        <div class="flex items-center gap-2 rounded-lg border border-border/50 bg-muted/20 px-3 py-2.5 text-[11px] text-muted-foreground">
          <Info size={14} class="shrink-0 text-primary" />
          <span>{m.common_ledger_label_type()}: <strong class="text-foreground">{m.finance_ui_account_type_asset()}</strong> · {m.common_ledger_col_code()}: <strong class="font-mono text-foreground">{baseCurrency}</strong></span>
        </div>

      </div>

    {:else}
      <!-- Entity Form Content -->
      <div class="flex flex-col gap-4 animate-in fade-in-50 duration-150">
        <!-- Entity Type Selection -->
        <FormField label={m.common_print_multi_up_entity_type()} required>
          {#snippet children()}
          <Select bind:value={enType} type="single">
            <SelectTrigger class="w-full text-xs text-start flex items-center justify-between bg-background">
              <span class="truncate">
                {enType === "vendor" ? m.common_ledger_entity_vendor() : enType === "partner" ? m.common_ledger_entity_partner() : enType === "employee" ? m.common_ledger_entity_employee() : m.common_ledger_entity_student()}
              </span>
            </SelectTrigger>
            <SelectContent class="text-start">
              <SelectItem value="vendor" class="text-xs cursor-pointer">{m.common_ledger_entity_vendor()}</SelectItem>
              <SelectItem value="partner" class="text-xs cursor-pointer">{m.common_ledger_entity_partner()}</SelectItem>
              <SelectItem value="employee" class="text-xs cursor-pointer">{m.common_ledger_entity_employee()}</SelectItem>
              <SelectItem value="student" class="text-xs cursor-pointer">{m.common_ledger_entity_student()}</SelectItem>
            </SelectContent>
          </Select>
          {/snippet}
        </FormField>

        <!-- Entity Names -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <FormField id="en_name_ar" label={m.common_ledger_name_ar()} required={!enNameEn}>
            {#snippet children()}
            <Input
              id="en_name_ar"
              type="text"
              bind:value={enNameAr}
              placeholder={m.common_ledger_entity_name_ar_placeholder()}
              required={!enNameEn}
              disabled={isSubmitting}
            />
            {/snippet}
          </FormField>
          <FormField id="en_name_en" label={m.common_ledger_name_en()} required={!enNameAr}>
            {#snippet children()}
            <Input
              id="en_name_en"
              type="text"
              bind:value={enNameEn}
              placeholder={m.common_ledger_entity_name_en_placeholder()}
              required={!enNameAr}
              disabled={isSubmitting}
            />
            {/snippet}
          </FormField>
        </div>

      </div>
    {/if}

      <!-- Form Actions (Footer) -->
      <div class="mt-auto flex shrink-0 flex-row justify-end gap-3 border-t border-border/50 px-0 pb-0 pt-4">
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
            <ArrowPath size={13} class="animate-spin" />
          {:else}
            <Check size={13} />
          {/if}
          <span>{isSubmitting ? m.common_loading() : m.common_ledger_btn_add_account()}</span>
        </Button>
      </div>
  </form>
</div>
