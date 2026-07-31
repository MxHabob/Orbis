<!--
  @component AddCurrencyDialog
  A modern, feature-rich modal for adding currencies to the organization.
  Uses `@orbis/location` and standard `Intl` APIs for smart preset auto-completion.
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
  DialogDescription,
  DialogFooter,
} from "@orbis/ui-svelte/dialog";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import { Sparkles, Coins, ArrowRightLeft, Info, Check } from "lucide-svelte";

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

// Smart preset dropdown items
function formatPresetLabel(curCode: string): string {
  const upper = curCode.toUpperCase();
  try {
    const arNames = new Intl.DisplayNames(["ar"], { type: "currency" });
    const nameArVal = arNames.of(upper);
    return nameArVal && nameArVal !== upper ? `${upper} - ${nameArVal}` : upper;
  } catch {
    return upper;
  }
}

const presetItems = uniqueCurrencies.map((c) => {
  const upper = c.toUpperCase();
  return {
    id: upper,
    label: formatPresetLabel(upper),
    value: upper,
  };
});

const selectedPresetItem = $derived(
  code ? { id: code, label: formatPresetLabel(code), value: code } : null,
);

/**
 * Smart auto-completion when selecting a currency preset
 */
function handlePresetSelect(item: { id: string; label: string; value: string }) {
  const curCode = item.value.toUpperCase();
  code = curCode;

  try {
    const arNames = new Intl.DisplayNames(["ar"], { type: "currency" });
    const enNames = new Intl.DisplayNames(["en"], { type: "currency" });
    nameAr = arNames.of(curCode) || curCode;
    name = enNames.of(curCode) || curCode;
  } catch {
    name = curCode;
    nameAr = curCode;
  }

  try {
    const formatter = new Intl.NumberFormat("ar", {
      style: "currency",
      currency: curCode,
    });
    const parts = formatter.formatToParts(100);
    const symPart = parts.find((p) => p.type === "currency");
    if (symPart?.value) {
      symbol = symPart.value;
    } else {
      symbol = curCode;
    }

    if (parts[0]?.type === "currency") {
      symbolPosition = "before";
    } else {
      symbolPosition = "after";
    }
  } catch {
    symbol = curCode;
  }

  // If this is the first currency in the system, suggest making it base
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
  <DialogContent class="sm:max-w-[560px] text-start border-border bg-card shadow-2xl p-0 overflow-hidden rounded-xl">
    <!-- Header with vibrant gradient accent -->
    <div class="bg-gradient-to-r from-primary/10 via-primary/5 to-transparent px-6 pt-6 pb-4 border-b border-border/60">
      <DialogHeader>
        <div class="flex items-center gap-2.5 mb-1">
          <div class="p-2 rounded-lg bg-primary/10 text-primary">
            <Coins class="w-5 h-5" />
          </div>
          <DialogTitle class="text-lg font-bold tracking-tight text-foreground">
            {m.settings_currencies_modal_add_title()}
          </DialogTitle>
        </div>
        <DialogDescription class="text-xs text-muted-foreground leading-relaxed ps-9">
          {m.settings_currencies_modal_add_desc()}
        </DialogDescription>
      </DialogHeader>
    </div>

    <form onsubmit={handleSubmit} class="p-6 space-y-5">
      <!-- Smart Location Selector -->
      <div class="p-3.5 rounded-lg border border-primary/20 bg-primary/5 space-y-2">
        <div class="flex items-center gap-1.5 text-xs font-semibold text-primary">
          <Sparkles class="w-3.5 h-3.5" />
          <span>{m.common_general_search()} ({m.settings_nav_currencies()})</span>
        </div>
        <ComboboxDropdown
          placeholder="اختر عملة للتعبيت السريعة..."
          searchPlaceholder="ابحث باسم العملة أو الرمز (مثال: USD, EUR, YER)..."
          selectedItem={selectedPresetItem}
          items={presetItems}
          onSelect={handlePresetSelect}
          class="w-full bg-background border-border/80 h-9"
        />
        <span class="text-[10px] text-muted-foreground block">
          اختيار العملة يقو بملء البيانات تلقائياً، ويمكنك تعديلها بحرية أدناه.
        </span>
      </div>

      <!-- Currency Code & Names -->
      <div class="grid grid-cols-1 sm:grid-cols-3 gap-3">
        <div class="space-y-1.5">
          <Label for="cur-code" class="text-xs font-semibold text-foreground flex items-center justify-between">
            <span>{m.settings_currencies_col_code()}</span>
            <span class="text-[10px] text-destructive">*</span>
          </Label>
          <Input
            id="cur-code"
            bind:value={code}
            placeholder="USD"
            class="uppercase font-mono text-sm h-9 bg-background"
            required
            disabled={isSubmitting}
          />
        </div>

        <div class="space-y-1.5">
          <Label for="cur-name-ar" class="text-xs font-semibold text-foreground">
            {m.settings_currencies_col_name()} (بالعربية)
          </Label>
          <Input
            id="cur-name-ar"
            bind:value={nameAr}
            placeholder="دولار أمريكي"
            class="text-sm h-9 bg-background"
            disabled={isSubmitting}
          />
        </div>

        <div class="space-y-1.5">
          <Label for="cur-name-en" class="text-xs font-semibold text-foreground">
            الاسم بالإنجليزية
          </Label>
          <Input
            id="cur-name-en"
            bind:value={name}
            placeholder="US Dollar"
            class="text-sm h-9 bg-background"
            disabled={isSubmitting}
          />
        </div>
      </div>

      <!-- Symbol, Position & Decimals -->
      <div class="grid grid-cols-1 sm:grid-cols-3 gap-3">
        <div class="space-y-1.5">
          <Label for="cur-symbol" class="text-xs font-semibold text-foreground">
            {m.settings_currencies_col_symbol()}
          </Label>
          <Input
            id="cur-symbol"
            bind:value={symbol}
            placeholder="$"
            class="text-sm h-9 bg-background font-mono"
            disabled={isSubmitting}
          />
        </div>

        <div class="space-y-1.5">
          <Label class="text-xs font-semibold text-foreground">
            موقع الرمز
          </Label>
          <Select type="single" bind:value={symbolPosition} disabled={isSubmitting}>
            <SelectTrigger class="h-9 w-full bg-background text-xs">
              {symbolPosition === "before" ? m.settings_currencies_symbol_position_before() : m.settings_currencies_symbol_position_after()}
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="after">{m.settings_currencies_symbol_position_after()}</SelectItem>
              <SelectItem value="before">{m.settings_currencies_symbol_position_before()}</SelectItem>
            </SelectContent>
          </Select>
        </div>

        <div class="space-y-1.5">
          <Label for="cur-decimals" class="text-xs font-semibold text-foreground">
            الخانة العشرية
          </Label>
          <Input
            id="cur-decimals"
            type="number"
            min="0"
            max="4"
            bind:value={decimalPlaces}
            class="text-sm h-9 bg-background font-mono"
            disabled={isSubmitting}
          />
        </div>
      </div>

      <!-- Exchange Rate & Base Currency Toggle -->
      <div class="p-3.5 rounded-lg border border-border/80 bg-muted/30 space-y-4">
        <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 items-center">
          <div class="space-y-1.5">
            <Label for="cur-rate" class="text-xs font-semibold text-foreground flex items-center gap-1.5">
              <ArrowRightLeft class="w-3.5 h-3.5 text-muted-foreground" />
              <span>{m.settings_currencies_col_rate()}</span>
            </Label>
            <Input
              id="cur-rate"
              type="number"
              step="0.0001"
              min="0.000001"
              bind:value={exchangeRate}
              disabled={isBase || isSubmitting}
              class="text-sm h-9 bg-background font-mono"
            />
            {#if isBase}
              <span class="text-[10px] text-muted-foreground block">
                العملة الأساسية تكون دائماً بسعر صرف 1.00
              </span>
            {/if}
          </div>

          <div class="flex items-center justify-between border-s border-border/60 ps-4 py-1">
            <div class="flex flex-col gap-0.5 text-start">
              <Label for="is-base-toggle" class="text-xs font-semibold text-foreground cursor-pointer">
                {m.settings_currencies_type_base()}
              </Label>
              <span class="text-[10px] text-muted-foreground">
                تعيين كعملة رئيسية للمنظمة
              </span>
            </div>
            <Switch
              id="is-base-toggle"
              checked={isBase}
              onCheckedChange={(val) => {
                isBase = val;
                if (val) exchangeRate = 1.0;
              }}
              disabled={isSubmitting}
            />
          </div>
        </div>
      </div>

      <DialogFooter class="pt-2 gap-2 border-t border-border/60">
        <Button
          type="button"
          variant="outline"
          size="sm"
          onclick={() => {
            open = false;
            resetForm();
          }}
          disabled={isSubmitting}
        >
          {m.common_action_cancel()}
        </Button>
        <Button type="submit" variant="lime" size="sm" disabled={isSubmitting} class="gap-1.5">
          <Check class="w-4 h-4" />
          <span>{m.settings_currencies_add_btn()}</span>
        </Button>
      </DialogFooter>
    </form>
  </DialogContent>
</Dialog>
