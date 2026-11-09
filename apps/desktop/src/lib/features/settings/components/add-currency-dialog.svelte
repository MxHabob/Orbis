<!--
  @component AddCurrencyDialog
  Follows Orbis standard UI design tokens, dialog structures, and component patterns.
  Uses `@orbis/ui` Dialog, Input, Label, Select, Switch, Button and Paraglide i18n messages.
  Performance-optimized with static module-level Intl instance caching and O(1) preset lookups.
-->
<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { api } from "$lib/api";
import { toast } from "@orbis/ui";
import { uniqueCurrencies } from "@orbis/location/currencies";
import SearchableSelect from "$lib/shared/components/searchable-select.svelte";
import { Button } from "@orbis/ui/button";
import { FormField } from "@orbis/ui";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Switch } from "@orbis/ui/switch";
import { Badge } from "@orbis/ui/badge";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@orbis/ui/dialog";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@orbis/ui/accordion";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import {
  AdjustmentsHorizontal,
  ArrowPath,
  Check,
  DocumentChartBar,
  GlobeAlt,
  Sparkles,
} from "@orbis/icons";
import { currencyInputSchema } from "@orbis/schemas";
import { FormController } from "$lib/shared/forms/form-controller.svelte";
import { BilingualNameFields } from "@orbis/ui/bilingual-name-fields";
import { suggestLocalTranslation } from "$lib/shared/translation/local-engine";

interface Props {
  open: boolean;
  currency?: any | null;
  existingCurrencies?: any[];
  onsuccess?: () => void;
}

let {
  open = $bindable(false),
  currency = null,
  existingCurrencies = [],
  onsuccess,
}: Props = $props();

const isEditing = $derived(currency !== null);

// Module-level static cache for Intl instances (created once, zero overhead per render)
let cachedArDisplayNames: Intl.DisplayNames | null = null;
let cachedEnDisplayNames: Intl.DisplayNames | null = null;

function getArDisplayNames(): Intl.DisplayNames | null {
  if (cachedArDisplayNames === null) {
    try {
      cachedArDisplayNames = new Intl.DisplayNames(["ar"], {
        type: "currency",
      });
    } catch {
      cachedArDisplayNames = null;
    }
  }
  return cachedArDisplayNames;
}

function getEnDisplayNames(): Intl.DisplayNames | null {
  if (cachedEnDisplayNames === null) {
    try {
      cachedEnDisplayNames = new Intl.DisplayNames(["en"], {
        type: "currency",
      });
    } catch {
      cachedEnDisplayNames = null;
    }
  }
  return cachedEnDisplayNames;
}

// Pre-computed preset dropdown items (built once for instant opening)
let cachedPresetItems: { id: string; label: string; value: string }[] | null =
  null;
let cachedPresetMap: Map<
  string,
  { id: string; label: string; value: string }
> | null = null;

function getPresetItemsData() {
  if (!cachedPresetItems) {
    const arNames = getArDisplayNames();
    const items: { id: string; label: string; value: string }[] = [];
    const map = new Map<string, { id: string; label: string; value: string }>();

    for (const c of uniqueCurrencies) {
      const upper = c.toUpperCase();
      const arVal = arNames?.of(upper);
      const label = arVal && arVal !== upper ? `${upper} - ${arVal}` : upper;
      const itemObj = { id: upper, label, value: upper };
      items.push(itemObj);
      map.set(upper, itemObj);
    }
    cachedPresetItems = items;
    cachedPresetMap = map;
  }
  return { items: cachedPresetItems, map: cachedPresetMap! };
}

const { items: presetItems, map: presetMap } = getPresetItemsData();

// Form State
let code = $state(currency?.code ?? "");
let name = $state(currency?.name ?? "");
let nameAr = $state(currency?.name_ar ?? "");
let symbol = $state(currency?.symbol ?? "");
let symbolPosition = $state(currency?.symbol_position ?? "after");
let decimalPlaces = $state(currency?.decimal_places ?? 2);
let exchangeRate = $state(Number(currency?.exchange_rate ?? 1));
let isBase = $state(currency?.is_base === true || currency?.is_base === 1);
const form = new FormController({
  initialValues: {
    code: "",
    name: "",
    name_ar: null as string | null,
    symbol: "",
    symbol_position: "after",
    decimal_places: 2,
    exchange_rate: 1,
    is_base: false,
  },
  schema: currencyInputSchema,
});
const isSubmitting = $derived(form.isSubmitting);

const selectedPresetItem = $derived(
  code
    ? presetMap.get(code.toUpperCase()) || {
        id: code,
        label: code,
        value: code,
      }
    : null,
);

// Formatted Live Preview Text
let livePreview = $derived.by(() => {
  const s = symbol.trim() || code.toUpperCase() || "$";
  const num = (100.0).toFixed(Math.max(0, Math.min(4, decimalPlaces)));
  return symbolPosition === "before" ? `${s} ${num}` : `${num} ${s}`;
});

/**
 * Optimized auto-completion handler using cached Intl instances
 */
function handlePresetSelect(item: {
  id: string;
  label: string;
  value: string;
}) {
  const curCode = item.value.toUpperCase();
  code = curCode;

  const arNames = getArDisplayNames();
  const enNames = getEnDisplayNames();

  nameAr = arNames?.of(curCode) || curCode;
  name = enNames?.of(curCode) || curCode;

  try {
    const formatter = new Intl.NumberFormat("ar", {
      style: "currency",
      currency: curCode,
    });
    const parts = formatter.formatToParts(100);
    const symPart = parts.find((p) => p.type === "currency");
    symbol = symPart?.value || curCode;
    symbolPosition = parts[0]?.type === "currency" ? "before" : "after";
  } catch {
    symbol = curCode;
    symbolPosition = "after";
  }

  if (existingCurrencies.length === 0) {
    isBase = true;
    exchangeRate = 1.0;
  }
}

function resetForm() {
  code = "";
  name = "";
  nameAr = "";
  symbol = "";
  symbolPosition = "after";
  decimalPlaces = 2;
  exchangeRate = 1.0;
  isBase = false;
}

async function handleSubmit(e: Event) {
  e.preventDefault();
  if (isSubmitting) return;

  const cleanCode = code.trim().toUpperCase();

  if (
    existingCurrencies.some(
      (c) =>
        c.code?.toUpperCase() === cleanCode &&
        c.code?.toUpperCase() !== currency?.code?.toUpperCase(),
    )
  ) {
    toast.error(m.settings_currencies_already_exists());
    return;
  }

  form.values = {
    code: cleanCode,
    name: name.trim() || cleanCode,
    name_ar: nameAr.trim() || null,
    symbol: symbol.trim() || cleanCode,
    symbol_position: symbolPosition,
    decimal_places: Number(decimalPlaces),
    exchange_rate: Number(exchangeRate),
    is_base: isBase,
  };
  const submitted = await form.submit(async (values) => {
    const request = {
      code: values.code,
      name: values.name,
      name_ar: values.name_ar ?? null,
      symbol: values.symbol,
      symbol_position: values.symbol_position,
      decimal_places: values.decimal_places,
      exchange_rate: values.exchange_rate,
      is_base: values.is_base,
    };
    if (currency) await api.updateCurrency(currency.code, request);
    else await api.createCurrency(request);

    toast.success(
      currency
        ? m.settings_currencies_updated_success({ code: values.code })
        : m.settings_currencies_added_success(),
    );
  });
  if (submitted) {
    open = false;
    resetForm();
    onsuccess?.();
  } else if (form.formError) toast.error(form.formError);
  else toast.error(m.settings_currencies_invalid_code());
}
</script>

<Dialog bind:open onOpenChange={(v: boolean) => !v && resetForm()}>
  <DialogContent class="max-w-md">
    <DialogHeader>
      <div class="flex items-center justify-between">
        <DialogTitle class="text-sm font-semibold flex items-center gap-2">
          <DocumentChartBar size={16} class="text-primary" />
          <span>{isEditing ? m.settings_currencies_edit_dialog_title() : m.settings_currencies_add_dialog_title()}</span>
        </DialogTitle>
        {#if code}
          <Badge variant="outline" class="font-mono text-[10px] bg-primary/5 text-primary border-primary/20">
            {livePreview}
          </Badge>
        {/if}
      </div>
    </DialogHeader>

    <form onsubmit={handleSubmit} class="flex flex-col gap-4 py-2">
      <Accordion type="multiple" value={["currency", "formatting"]} class="flex flex-col gap-3">
        <AccordionItem value="currency" class="rounded-lg px-3">
          <AccordionTrigger class="text-sm font-semibold hover:no-underline">{m.settings_nav_currencies()}</AccordionTrigger>
          <AccordionContent>
      <!-- Preset Quick Selection -->
      <div class="flex flex-col gap-1.5 py-3">
        <div class="flex items-center gap-1.5 text-xs text-primary font-medium">
          <Sparkles size={13} />
          <span>{m.common_header_search()} ({m.settings_nav_currencies()})</span>
        </div>
        <SearchableSelect
          value={code}
          items={presetItems}
          placeholder={m.settings_ui_currency_select_placeholder()}
          searchPlaceholder={m.settings_ui_currency_search_by_name_or_code()}
          getItemValue={(item) => item.value}
          getItemLabel={(item) => item.label}
          getItemSubtitle={(item) => item.value}
          onchange={(val) => {
            if (val) {
              const item = presetItems.find((p) => p.value === val);
              if (item) handlePresetSelect(item);
            }
          }}
          class="w-full bg-background border-border/60 text-xs h-8"
        />
      </div>

      <!-- Currency Code & Symbol -->
      <div class="grid grid-cols-2 gap-3">
        <FormField label={m.common_ledger_col_code()} error={form.errors.code?.[0]} required>
          {#snippet children()}
          <Input
            bind:value={code}
            placeholder="USD"
            class="text-xs uppercase font-mono h-8"
            required
            disabled={isSubmitting}
            readonly={isEditing}
          />
          {/snippet}
        </FormField>

        <FormField label={m.settings_currencies_col_symbol()} error={form.errors.symbol?.[0]} required>
          {#snippet children()}
          <Input
            bind:value={symbol}
            placeholder="$"
            class="text-xs font-mono h-8"
            disabled={isSubmitting}
          />
          {/snippet}
        </FormField>
      </div>

      <BilingualNameFields
        arId="currency-name-ar"
        enId="currency-name-en"
        arLabel={m.common_cycle_form_name()}
        enLabel={m.settings_ui_currency_english_name()}
        bind:arValue={nameAr}
        bind:enValue={name}
        arPlaceholder={m.settings_ui_currency_arabic_name_example()}
        enPlaceholder={m.settings_ui_currency_english_name_example()}
        arError={form.errors.name_ar?.[0]}
        enError={form.errors.name?.[0]}
        kind="label"
        requestSuggestion={suggestLocalTranslation}
        enRequired
        disabled={isSubmitting}
        loadingMessage={m.bilingual_entry_loading()}
        failureMessage={m.bilingual_entry_failure()}
      />

          </AccordionContent>
        </AccordionItem>

        <AccordionItem value="formatting" class="rounded-lg px-3">
          <AccordionTrigger class="text-sm font-semibold hover:no-underline">{m.settings_currencies_col_rate()}</AccordionTrigger>
          <AccordionContent>

      <!-- Position, Decimals & Rate -->
      <div class="grid grid-cols-3 gap-3">
        <FormField label={m.settings_ui_currency_symbol_position()} error={form.errors.symbol_position?.[0]} required>
          {#snippet children()}
          <Select type="single" bind:value={symbolPosition} disabled={isSubmitting}>
            <SelectTrigger class="h-8 w-full text-xs">
              {symbolPosition === "before" ? m.settings_currencies_symbol_position_before() : m.settings_currencies_symbol_position_after()}
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="after">{m.settings_currencies_symbol_position_after()}</SelectItem>
              <SelectItem value="before">{m.settings_currencies_symbol_position_before()}</SelectItem>
            </SelectContent>
          </Select>
          {/snippet}
        </FormField>

        <FormField label={m.settings_ui_currency_decimal_places()} error={form.errors.decimal_places?.[0]} required>
          {#snippet children()}
          <Input
            type="number"
            min="0"
            max="4"
            bind:value={decimalPlaces}
            class="text-xs font-mono h-8"
            disabled={isSubmitting}
          />
          {/snippet}
        </FormField>

        <FormField label={m.settings_currencies_col_rate()} error={form.errors.exchange_rate?.[0]} required>
          {#snippet children()}
          <Input
            type="number"
            step="0.0001"
            min="0.000001"
            bind:value={exchangeRate}
            disabled={isBase || isSubmitting}
            class="text-xs font-mono h-8"
          />
          {/snippet}
        </FormField>
      </div>

      <!-- Base Currency Switch -->
      <div class="flex items-center justify-between py-3">
        <div class="flex items-center gap-2">
          <GlobeAlt size={14} class="text-muted-foreground" />
          <div class="flex flex-col gap-0.5">
            <Label class="text-xs font-medium cursor-pointer">
              {m.settings_currencies_type_base()}
            </Label>
            <span class="text-[10px] text-muted-foreground">
              {m.settings_ui_currency_set_as_base_description()}
            </span>
          </div>
        </div>

        <Switch
          checked={isBase}
          onCheckedChange={(val: boolean) => {
            isBase = val;
            if (val) exchangeRate = 1.0;
          }}
          disabled={isSubmitting}
        />
      </div>
          </AccordionContent>
        </AccordionItem>
      </Accordion>

      <DialogFooter class="pt-2">
        <Button
          type="button"
          variant="outline"
          size="sm"
          class="text-xs cursor-pointer"
          onclick={() => {
            open = false;
            resetForm();
          }}
          disabled={isSubmitting}
        >
          {m.common_action_cancel()}
        </Button>
        <Button
          type="submit"
          size="sm"
          variant="lime"
          disabled={isSubmitting}
          class="text-xs cursor-pointer gap-1.5 font-bold"
        >
          {#if isSubmitting}
            <LoadingIndicator   size={12} />
          {:else}
            <Check size={12} />
          {/if}
          <span>{isEditing ? m.common_action_edit() : m.settings_currencies_add_btn()}</span>
        </Button>
      </DialogFooter>
    </form>
  </DialogContent>
</Dialog>
