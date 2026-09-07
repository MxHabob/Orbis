<!--
  @component CurrencySelector
  Reusable currency selection component.
  Relying 100% on active currencies from the backend database via `api.listCurrencies()`.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { Label } from "@orbis/ui/label";
import { ComboboxDropdown } from "@orbis/ui/combobox-dropdown";
import { api, } from "$lib/api";

let {
  title = m.finance_ledger_currency(),
  description = m.settings_general_currency_desc(),
  currentCurrency = $bindable("SAR"),
  includeAll = false,
  allLabel = "All currencies",
  compact = false,
  disabled = false,
  onchange,
}: {
  title?: string;
  description?: string;
  currentCurrency?: string;
  includeAll?: boolean;
  allLabel?: string;
  compact?: boolean;
  disabled?: boolean;
  onchange?: (currency: string) => void;
} = $props();

let dbCurrencies = $state<any[]>([]);
let isLoading = $state(true);

onMount(async () => {
  try {
    const list = await api.listCurrencies();
    if (Array.isArray(list)) {
      dbCurrencies = list;
    }
  } catch (e) {
    console.warn("Failed to load currencies from backend:", e);
  } finally {
    isLoading = false;
  }
});

const items = $derived.by(() => {
  const currencyItems = dbCurrencies.map((c) => {
    const codeUpper = (c.code || "").toUpperCase();
    const symbolStr = c.symbol ? ` (${c.symbol})` : "";
    const nameStr = c.name_ar || c.name || "";
    return {
      id: codeUpper,
      label: nameStr ? `${codeUpper} - ${nameStr}${symbolStr}` : `${codeUpper}${symbolStr}`,
      value: codeUpper,
    };
  });

  return includeAll
    ? [{ id: "all", label: allLabel, value: "all" }, ...currencyItems]
    : currencyItems;
});

const selectedItem = $derived.by(() => {
  const currentUpper = (currentCurrency || "").toUpperCase();
  const match = items.find((item) => item.value === currentUpper);
  if (match) return match;

  return {
    id: currentUpper || "SAR",
    label: currentUpper || "SAR",
    value: currentUpper || "SAR",
  };
});

function handleSelect(item: { id: string; label: string; value: string }) {
  if (item.value !== currentCurrency) {
    currentCurrency = item.value;
    onchange?.(item.value);
  }
}
</script>

<div class={compact ? "flex flex-col gap-1" : "px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4 last:border-b-0"}>
  <div class="flex flex-col gap-1 text-start">
    <Label class={compact ? "text-[9px] font-bold text-muted-foreground" : "text-xs font-semibold text-foreground"}>
      {title}
    </Label>
    {#if description}
      <span class="text-[11px] text-muted-foreground leading-relaxed">{description}</span>
    {/if}
  </div>
  <div class={compact ? "w-full" : "w-full md:w-120 shrink-0 flex items-center justify-end"}>
    <ComboboxDropdown
      placeholder={isLoading ? "Loading currencies..." : "Select currency..."}
      searchPlaceholder="Search currency..."
      {selectedItem}
      {items}
      disabled={disabled || isLoading}
      onSelect={handleSelect}
      class="w-full"
    />
  </div>
</div>
