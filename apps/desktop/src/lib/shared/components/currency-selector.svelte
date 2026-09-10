<!--
  @component CurrencySelector
  Reusable currency selection component using `SettingsRow`.
  Relying 100% on active currencies from the backend database via `api.listCurrencies()`.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { ComboboxDropdown } from "@orbis/ui/combobox-dropdown";
import { api } from "$lib/api";
import SettingsRow from "$lib/shared/components/settings-row.svelte";
import { Label } from "@orbis/ui/label";

let {
  title = m.finance_ledger_currency(),
  description = m.settings_general_currency_desc(),
  currentCurrency = $bindable("SAR"),
  includeAll = false,
  allLabel = "All currencies",
  compact = false,
  disabled = false,
  last = false,
  onchange,
}: {
  title?: string;
  description?: string;
  currentCurrency?: string;
  includeAll?: boolean;
  allLabel?: string;
  compact?: boolean;
  disabled?: boolean;
  last?: boolean;
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
      label: nameStr
        ? `${codeUpper} - ${nameStr}${symbolStr}`
        : `${codeUpper}${symbolStr}`,
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

{#if compact}
  <div class="flex flex-col gap-1">
    <Label class="text-[9px] font-bold text-muted-foreground">
      {title}
    </Label>
    {#if description}
      <span class="text-[11px] text-muted-foreground leading-relaxed">{description}</span>
    {/if}
    <div class="w-full">
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
{:else}
  <SettingsRow {title} {description} {last}>
    {#snippet children()}
      <ComboboxDropdown
        placeholder={isLoading ? "Loading currencies..." : "Select currency..."}
        searchPlaceholder="Search currency..."
        {selectedItem}
        {items}
        disabled={disabled || isLoading}
        onSelect={handleSelect}
        class="w-full"
      />
    {/snippet}
  </SettingsRow>
{/if}
