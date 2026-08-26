<!--
  @component AddCurrencyDialog
  Follows Orbis standard UI design tokens, dialog structures, and component patterns.
  Uses `@orbis/ui-svelte` Dialog, Input, Label, Select, Switch, Button and Paraglide i18n messages.
  Performance-optimized with static module-level Intl instance caching and O(1) preset lookups.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { commands } from "$lib/core/ipc";
import { toast } from "svelte-sonner";
import { uniqueCurrencies } from "@orbis/location/currencies";
import { ComboboxDropdown } from "@orbis/ui-svelte/combobox-dropdown";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import { Switch } from "@orbis/ui-svelte/switch";
import { Badge } from "@orbis/ui-svelte/badge";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@orbis/ui-svelte/dialog";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@orbis/ui-svelte/accordion";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import {
  Sparkles,
  Coins,
  ArrowRightLeft,
  Check,
  Globe,
  Loader2,
} from "@lucide/svelte";

interface Props {
  open: boolean;
  existingCurrencies?: any[];
  onsuccess?: () => void;
}

let {
  open = $bindable(false),
  existingCurrencies = [],
  onsuccess,
}: Props = $props();

// Module-level static cache for Intl instances (created once, zero overhead per render)
let cachedArDisplayNames: Intl.DisplayNames | null = null;
let cachedEnDisplayNames: Intl.DisplayNames | null = null;

function getArDisplayNames(): Intl.DisplayNames | null {
  if (cachedArDisplayNames === null) {
    try {
      cachedArDisplayNames = new Intl.DisplayNames(["ar"], { type: "currency" });
    } catch {
      cachedArDisplayNames = null;
    }
  }
  return cachedArDisplayNames;
}

function getEnDisplayNames(): Intl.DisplayNames | null {
  if (cachedEnDisplayNames === null) {
    try {
      cachedEnDisplayNames = new Intl.DisplayNames(["en"], { type: "currency" });
    } catch {
      cachedEnDisplayNames = null;
    }
  }
  return cachedEnDisplayNames;
}

// Pre-computed preset dropdown items (built once for instant opening)
let cachedPresetItems: { id: string; label: string; value: string }[] | null = null;
let cachedPresetMap: Map<string, { id: string; label: string; value: string }> | null = null;

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
let code = $state("");
let name = $state("");
let nameAr = $state("");
let symbol = $state("");
let symbolPosition = $state("after"); // "before" | "after"
let decimalPlaces = $state(2);
let exchangeRate = $state(1.0);
let isBase = $state(false);
let isSubmitting = $state(false);

const selectedPresetItem = $derived(
  code ? presetMap.get(code.toUpperCase()) || { id: code, label: code, value: code } : null,
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
function handlePresetSelect(item: { id: string; label: string; value: string }) {
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
  isSubmitting = false;
}

async function handleSubmit(e: Event) {
  e.preventDefault();
  if (isSubmitting) return;

  const cleanCode = code.trim().toUpperCase();
  if (!cleanCode || cleanCode.length < 2) {
    toast.error(m.settings_currencies_invalid_code());
    return;
  }

  if (existingCurrencies.some((c) => c.code?.toUpperCase() === cleanCode)) {
    toast.error(m.settings_currencies_already_exists());
    return;
  }

  if (exchangeRate <= 0) {
    toast.error(m.settings_currencies_invalid_rate());
    return;
  }

  isSubmitting = true;

  try {
    await commands.finance.createCurrency(
      cleanCode,
      name.trim() || cleanCode,
      nameAr.trim() || null,
      symbol.trim() || cleanCode,
      symbolPosition,
      decimalPlaces,
      exchangeRate,
      isBase,
    );

    toast.success(m.settings_currencies_added_success());
    open = false;
    resetForm();
    onsuccess?.();
  } catch (error: any) {
    console.error("Error creating currency:", error);
    const errorMsg = typeof error === "string" ? error : error?.message || "Unknown error";
    toast.error(m.settings_currencies_add_error({ error: errorMsg }));
  } finally {
    isSubmitting = false;
  }
}
</script>

<Dialog bind:open onOpenChange={(v) => !v && resetForm()}>
  <DialogContent class="max-w-md">
    <DialogHeader>
      <div class="flex items-center justify-between">
        <DialogTitle class="text-sm font-semibold flex items-center gap-2">
          <Coins size={16} class="text-primary" />
          <span>{m.settings_currencies_add_dialog_title()}</span>
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
        <AccordionItem value="currency" class="rounded-lg border border-border/50 px-3">
          <AccordionTrigger class="text-sm font-semibold hover:no-underline">{m.settings_nav_currencies()}</AccordionTrigger>
          <AccordionContent>
      <!-- Preset Quick Selection -->
      <div class="flex flex-col gap-1.5 p-3 rounded-lg border border-border/40 bg-muted/20">
        <div class="flex items-center gap-1.5 text-xs text-primary font-medium">
          <Sparkles size={13} />
          <span>{m.common_header_search()} ({m.settings_nav_currencies()})</span>
        </div>
        <ComboboxDropdown
          placeholder="اختر عملة للتعبيئة السريعة..."
          searchPlaceholder="ابحث باسم العملة أو الرمز (مثال: USD, EUR, YER)..."
          selectedItem={selectedPresetItem}
          items={presetItems}
          onSelect={handlePresetSelect}
          class="w-full bg-background border-border/60 text-xs h-8"
        />
      </div>

      <!-- Currency Code & Symbol -->
      <div class="grid grid-cols-2 gap-3">
        <div class="flex flex-col gap-1.5">
          <Label class="text-xs flex items-center justify-between">
            <span>{m.common_ledger_col_code()}</span>
            <span class="text-destructive text-[10px]">*</span>
          </Label>
          <Input
            bind:value={code}
            placeholder="USD"
            class="text-xs uppercase font-mono h-8"
            required
            disabled={isSubmitting}
          />
        </div>

        <div class="flex flex-col gap-1.5">
          <Label class="text-xs">{m.settings_currencies_col_symbol()}</Label>
          <Input
            bind:value={symbol}
            placeholder="$"
            class="text-xs font-mono h-8"
            disabled={isSubmitting}
          />
        </div>
      </div>

      <!-- Names Grid -->
      <div class="grid grid-cols-2 gap-3">
        <div class="flex flex-col gap-1.5">
          <Label class="text-xs">{m.common_cycle_form_name()}</Label>
          <Input
            bind:value={nameAr}
            placeholder="دولار أمريكي"
            class="text-xs h-8"
            disabled={isSubmitting}
          />
        </div>

        <div class="flex flex-col gap-1.5">
          <Label class="text-xs">الاسم بالإنجليزية</Label>
          <Input
            bind:value={name}
            placeholder="US Dollar"
            class="text-xs h-8"
            disabled={isSubmitting}
          />
        </div>
      </div>

          </AccordionContent>
        </AccordionItem>

        <AccordionItem value="formatting" class="rounded-lg border border-border/50 px-3">
          <AccordionTrigger class="text-sm font-semibold hover:no-underline">{m.settings_currencies_col_rate()}</AccordionTrigger>
          <AccordionContent>

      <!-- Position, Decimals & Rate -->
      <div class="grid grid-cols-3 gap-3">
        <div class="flex flex-col gap-1.5">
          <Label class="text-xs">موقع الرمز</Label>
          <Select type="single" bind:value={symbolPosition} disabled={isSubmitting}>
            <SelectTrigger class="h-8 w-full text-xs">
              {symbolPosition === "before" ? m.settings_currencies_symbol_position_before() : m.settings_currencies_symbol_position_after()}
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="after">{m.settings_currencies_symbol_position_after()}</SelectItem>
              <SelectItem value="before">{m.settings_currencies_symbol_position_before()}</SelectItem>
            </SelectContent>
          </Select>
        </div>

        <div class="flex flex-col gap-1.5">
          <Label class="text-xs">الخانات العشرية</Label>
          <Input
            type="number"
            min="0"
            max="4"
            bind:value={decimalPlaces}
            class="text-xs font-mono h-8"
            disabled={isSubmitting}
          />
        </div>

        <div class="flex flex-col gap-1.5">
          <Label class="text-xs flex items-center gap-1">
            <ArrowRightLeft size={12} class="text-muted-foreground" />
            <span>{m.settings_currencies_col_rate()}</span>
          </Label>
          <Input
            type="number"
            step="0.0001"
            min="0.000001"
            bind:value={exchangeRate}
            disabled={isBase || isSubmitting}
            class="text-xs font-mono h-8"
          />
        </div>
      </div>

      <!-- Base Currency Switch -->
      <div class="flex items-center justify-between p-3 rounded-lg border border-border/40 bg-muted/10">
        <div class="flex items-center gap-2">
          <Globe size={14} class="text-muted-foreground" />
          <div class="flex flex-col gap-0.5">
            <Label class="text-xs font-medium cursor-pointer">
              {m.settings_currencies_type_base()}
            </Label>
            <span class="text-[10px] text-muted-foreground">
              تعيين كعملة رئيسية للمنظمة والتقارير
            </span>
          </div>
        </div>

        <Switch
          checked={isBase}
          onCheckedChange={(val) => {
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
            <Loader2 size={12} class="animate-spin" />
          {:else}
            <Check size={12} />
          {/if}
          <span>{m.settings_currencies_add_btn()}</span>
        </Button>
      </DialogFooter>
    </form>
  </DialogContent>
</Dialog>
