<!--
  @component CurrencySelector
  Reusable currency selection component using SearchableSelect and SettingsRow.
  Relying 100% on active currencies from the backend database via `api.listCurrencies()`.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { l } from "$lib/core/i18n";
import type { CurrencyDto } from "$lib/api/types";
import SearchableSelect from "$lib/shared/components/searchable-select.svelte";
import { api } from "$lib/api";
import SettingsRow from "$lib/shared/components/settings-row.svelte";
import { Label } from "@orbis/ui/label";

type CurrencyOption = string | (CurrencyDto & {
  name_ar?: string;
  exchange_rate?: number;
  is_base?: boolean;
});

let {
  title = m.finance_ledger_currency(),
  description = m.settings_general_currency_desc(),
  currentCurrency = $bindable("SAR"),
  currencyOptions = undefined,
  includeAll = false,
  allLabel = m.common_filter_all(),
  compact = false,
  disabled = false,
  last = false,
  onchange,
}: {
  title?: string;
  description?: string;
  currentCurrency?: string;
  currencyOptions?: CurrencyOption[];
  includeAll?: boolean;
  allLabel?: string;
  compact?: boolean;
  disabled?: boolean;
  last?: boolean;
  onchange?: (currency: string) => void;
} = $props();

let dbCurrencies = $state<CurrencyDto[]>([]);
let isLoading = $state(true);
const availableCurrencies = $derived(currencyOptions ?? dbCurrencies);

onMount(async () => {
  if (currencyOptions !== undefined) {
    isLoading = false;
    return;
  }

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

function getCurrencyValue(c: CurrencyOption | null | undefined): string {
  if (!c) return "";
  return (typeof c === "string" ? c : c.code).toUpperCase();
}

function getCurrencyLabel(c: CurrencyOption | null | undefined): string {
  if (!c) return "";
  if (typeof c === "string") return c;
  const codeUpper = c.code.toUpperCase();
  const nameStr = l(c.name_ar, c.name) || c.name;
  const symbolStr = c.symbol ? ` (${c.symbol})` : "";
  return nameStr ? `${codeUpper} - ${nameStr}${symbolStr}` : `${codeUpper}${symbolStr}`;
}

function getCurrencySubtitle(c: CurrencyOption | null | undefined): string | undefined {
  if (!c || typeof c === "string") return undefined;
  if (c?.exchange_rate && Number(c.exchange_rate) !== 1) {
    return m.shared_ui_exchange_rate({ rate: c.exchange_rate });
  }
  return undefined;
}

function getCurrencyBadge(c: CurrencyOption | null | undefined): string | undefined {
  if (!c || typeof c === "string") return undefined;
  if (c?.is_base) return m.shared_ui_base_currency();
  return undefined;
}

function handleChange(val: string | null) {
  const next = val || "SAR";
  currentCurrency = next;
  onchange?.(next);
}
</script>

{#if compact}
  <div class="flex flex-col gap-1.5">
    <Label class="text-xs font-semibold">
      {title}
    </Label>
    {#if description}
      <span class="text-[11px] text-muted-foreground leading-relaxed">{description}</span>
    {/if}
    <div class="w-full">
      <SearchableSelect
        bind:value={currentCurrency}
        items={availableCurrencies}
        placeholder={isLoading ? m.shared_ui_loading_currencies() : (m.finance_ledger_currency())}
        searchPlaceholder={m.shared_ui_search_currencies_placeholder()}
        emptyText={m.shared_ui_no_matching_currencies()}
        showAllOption={includeAll}
        {allLabel}
        disabled={disabled || isLoading}
        getItemValue={getCurrencyValue}
        getItemLabel={getCurrencyLabel}
        getItemSubtitle={getCurrencySubtitle}
        getItemBadge={getCurrencyBadge}
        onchange={handleChange}
        class="w-full"
      />
    </div>
  </div>
{:else}
  <SettingsRow {title} {description} {last}>
    {#snippet children()}
      <SearchableSelect
        bind:value={currentCurrency}
        items={availableCurrencies}
        placeholder={isLoading ? m.shared_ui_loading_currencies() : (m.finance_ledger_currency())}
        searchPlaceholder={m.shared_ui_search_currencies_placeholder()}
        emptyText={m.shared_ui_no_matching_currencies()}
        showAllOption={includeAll}
        {allLabel}
        disabled={disabled || isLoading}
        getItemValue={getCurrencyValue}
        getItemLabel={getCurrencyLabel}
        getItemSubtitle={getCurrencySubtitle}
        getItemBadge={getCurrencyBadge}
        onchange={handleChange}
        class="w-full"
      />
    {/snippet}
  </SettingsRow>
{/if}
