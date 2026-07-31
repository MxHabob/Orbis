<!--
@component CurrencySettings
A reusable currency selection card using `@orbis/ui-svelte/combobox-dropdown`, `commands.finance.listCurrencies()`, and dynamic ISO currency metadata.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { Label } from "@orbis/ui-svelte/label";
import { ComboboxDropdown } from "@orbis/ui-svelte/combobox-dropdown";
import { commands } from "$lib/core/ipc";
import { uniqueCurrencies } from "@orbis/location/currencies";

let {
  title = m.finance_ledger_currency(),
  description = m.settings_general_currency_desc(),
  currentCurrency = $bindable("SAR"),
  disabled = false,
  onchange,
}: {
  title?: string;
  description?: string;
  currentCurrency?: string;
  disabled?: boolean;
  onchange?: (currency: string) => void;
} = $props();

let dbCurrencies = $state<any[]>([]);

onMount(async () => {
  try {
    const list = await commands.finance.listCurrencies();
    if (Array.isArray(list)) {
      dbCurrencies = list;
    }
  } catch (e) {
    console.error("Failed to load currencies in CurrencySettings:", e);
  }
});

/**
 * Format dynamic ISO currency label using standard Intl.DisplayNames API.
 * No hardcoded strings required.
 */
function formatIsoCurrencyLabel(code: string): string {
  const upperCode = code.toUpperCase();
  try {
    const arNames = new Intl.DisplayNames(["ar"], { type: "currency" });
    const nameAr = arNames.of(upperCode);
    if (nameAr && nameAr !== upperCode) {
      return `${upperCode} - ${nameAr}`;
    }
  } catch {
    // fallback
  }
  return upperCode;
}

const items = $derived.by(() => {
  const map = new Map<string, { id: string; label: string; value: string }>();

  // 1. Add DB currencies first
  for (const c of dbCurrencies) {
    if (c.code) {
      const codeUpper = c.code.toUpperCase();
      const symbolStr = c.symbol ? ` (${c.symbol})` : "";
      const nameStr = c.name_ar || c.name || formatIsoCurrencyLabel(codeUpper);
      map.set(codeUpper, {
        id: codeUpper,
        label: `${codeUpper} - ${nameStr}${symbolStr}`,
        value: codeUpper,
      });
    }
  }

  // 2. Add dynamic ISO currencies from @orbis/location
  for (const code of uniqueCurrencies) {
    const upper = code.toUpperCase();
    if (!map.has(upper)) {
      map.set(upper, {
        id: upper,
        label: formatIsoCurrencyLabel(upper),
        value: upper,
      });
    }
  }

  // 3. Add current selected currency if not present
  if (currentCurrency && !map.has(currentCurrency.toUpperCase())) {
    const curUpper = currentCurrency.toUpperCase();
    map.set(curUpper, {
      id: curUpper,
      label: formatIsoCurrencyLabel(curUpper),
      value: curUpper,
    });
  }

  return Array.from(map.values());
});

const selectedItem = $derived(
  items.find((item) => item.value === currentCurrency?.toUpperCase()) || {
    id: currentCurrency?.toUpperCase() || "SAR",
    label: formatIsoCurrencyLabel(currentCurrency?.toUpperCase() || "SAR"),
    value: currentCurrency?.toUpperCase() || "SAR",
  },
);

function handleSelect(item: { id: string; label: string; value: string }) {
  if (item.value !== currentCurrency) {
    currentCurrency = item.value;
    onchange?.(item.value);
  }
}
</script>

<div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
  <div class="flex flex-col gap-1 text-start">
    <Label class="text-xs font-semibold text-foreground">
      {title}
    </Label>
    <span class="text-[11px] text-muted-foreground leading-relaxed">{description}</span>
  </div>
  <div class="w-full md:w-[480px] shrink-0 flex items-center justify-end">
    <ComboboxDropdown
      placeholder="Select currency..."
      searchPlaceholder="Search currency..."
      {selectedItem}
      {items}
      {disabled}
      onSelect={handleSelect}
      class="w-full"
    />
  </div>
</div>
