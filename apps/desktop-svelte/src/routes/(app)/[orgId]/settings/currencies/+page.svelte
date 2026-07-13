<!--
  @page CurrenciesSettingsPage
  Allows administrators to manage multi-currency exchange rates, define base currencies,
  add new currencies, edit currency configurations, and delete unused currencies.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { onMount } from "svelte";
import { commands } from "$lib/core/ipc";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@orbis/ui-svelte";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import { Badge } from "@orbis/ui-svelte/badge";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Switch } from "@orbis/ui-svelte/switch";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { toast } from "svelte-sonner";
import {
  Coins,
  RefreshCw,
  Edit2,
  Trash2,
  Plus,
  Check,
  X,
  Sparkles,
  Globe,
  Info,
} from "lucide-svelte";
import { fade, slide } from "svelte/transition";

import { l, isAr } from "$lib/core/i18n";

let orgId = $derived((page.params.orgId as string) || "");

// State
let currencies = $state<any[]>([]);
let isLoading = $state(true);
let isSeeding = $state(false);
let isSubmitting = $state(false);

// Inline rate editing state
let editingRateCode = $state<string | null>(null);
let editRateValue = $state<number>(1.0);

// Dialogs state
let showAddDialog = $state(false);
let showEditDialog = $state(false);
let currencyToDelete = $state<any | null>(null);

// Add form state
let addCode = $state("");
let addName = $state("");
let addNameAr = $state("");
let addSymbol = $state("");
let addSymbolPosition = $state("after"); // "before" | "after"
let addDecimalPlaces = $state(2);
let addExchangeRate = $state(1.0);
let addIsBase = $state(false);
let addIsActive = $state(true);

// Edit form state
let editCode = $state("");
let editName = $state("");
let editNameAr = $state("");
let editSymbol = $state("");
let editSymbolPosition = $state("after");
let editDecimalPlaces = $state(2);
let editExchangeRate = $state(1.0);
let editIsBase = $state(false);
let editIsActive = $state(true);

async function loadCurrencies() {
  if (!orgId) return;
  isLoading = true;
  try {
    currencies = await commands.finance.listCurrencies(orgId);
  } catch (e) {
    console.error("Error loading currencies:", e);
    toast.error(m.settings_currencies_load_error());
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (orgId) {
    loadCurrencies();
  }
});

function openAddDialog() {
  addCode = "";
  addName = "";
  addNameAr = "";
  addSymbol = "";
  addSymbolPosition = "after";
  addDecimalPlaces = 2;
  addExchangeRate = 1.0;
  addIsBase = false;
  addIsActive = true;
  showAddDialog = true;
}

async function handleAddCurrency(e: Event) {
  e.preventDefault();
  if (isSubmitting) return;

  const code = addCode.trim().toUpperCase();
  if (!code || code.length < 2) {
    toast.error(m.settings_currencies_invalid_code());
    return;
  }

  // Check if code already exists
  if (currencies.some((c) => c.code === code)) {
    toast.error(m.settings_currencies_already_exists());
    return;
  }

  if (addExchangeRate <= 0) {
    toast.error(m.settings_currencies_invalid_rate());
    return;
  }

  isSubmitting = true;
  try {
    const newCurrency = {
      code,
      org_id: orgId,
      name: addName.trim() || code,
      name_ar: addNameAr.trim() ? addNameAr.trim() : undefined,
      symbol: addSymbol.trim() || code,
      symbol_position: addSymbolPosition,
      decimal_places: addDecimalPlaces,
      exchange_rate: addExchangeRate,
      is_base: addIsBase ? 1 : 0,
      is_active: addIsActive ? 1 : 0,
      updated_at: new Date().toISOString(),
    };

    await commands.finance.createCurrency(orgId, newCurrency);
    toast.success(m.settings_currencies_added_success());
    showAddDialog = false;
    await loadCurrencies();
  } catch (e) {
    console.error("Error creating currency:", e);
    const errorMsg = e instanceof Error ? e.message : String(e);
    toast.error(m.settings_currencies_add_error({ error: errorMsg }));
  } finally {
    isSubmitting = false;
  }
}

function openEditDialog(c: any) {
  editCode = c.code;
  editName = c.name || "";
  editNameAr = c.name_ar || "";
  editSymbol = c.symbol || "";
  editSymbolPosition = c.symbol_position || "after";
  editDecimalPlaces = c.decimal_places ?? 2;
  editExchangeRate = c.exchange_rate ?? 1.0;
  editIsBase = c.is_base === 1;
  editIsActive = c.is_active === 1;
  showEditDialog = true;
}

async function handleEditCurrency(e: Event) {
  e.preventDefault();
  if (isSubmitting) return;

  if (editExchangeRate <= 0) {
    toast.error(m.settings_currencies_invalid_rate());
    return;
  }

  isSubmitting = true;
  try {
    const updates = {
      name: editName.trim() || editCode,
      name_ar: editNameAr.trim() ? editNameAr.trim() : null,
      symbol: editSymbol.trim() || editCode,
      symbol_position: editSymbolPosition,
      decimal_places: editDecimalPlaces,
      exchange_rate: editExchangeRate,
      is_active: editIsActive,
    };

    await commands.finance.updateCurrency(orgId, editCode, updates);
    toast.success(m.settings_currencies_updated_success());
    showEditDialog = false;
    await loadCurrencies();
  } catch (e) {
    console.error("Error updating currency:", e);
    const errorMsg = e instanceof Error ? e.message : String(e);
    toast.error(m.settings_currencies_update_error({ error: errorMsg }));
  } finally {
    isSubmitting = false;
  }
}

async function confirmDeleteCurrency(c: any) {
  if (c.is_base === 1) {
    toast.error(m.settings_currencies_delete_base_error());
    return;
  }
  currencyToDelete = c;
}

async function handleDeleteCurrency() {
  if (!currencyToDelete) return;
  try {
    await commands.finance.deleteCurrency(orgId, currencyToDelete.code);
    toast.success(
      m.settings_currencies_deleted_success({ code: currencyToDelete.code }),
    );
    currencyToDelete = null;
    await loadCurrencies();
  } catch (e) {
    console.error("Error deleting currency:", e);
    const errorMsg = e instanceof Error ? e.message : String(e);
    toast.error(m.settings_currencies_delete_error({ error: errorMsg }));
  }
}

async function seedFinancials() {
  const confirmed = confirm(m.settings_currencies_seed_confirm());
  if (!confirmed) return;

  isSeeding = true;
  try {
    await commands.finance.seedDefaultFinancials(orgId);
    toast.success(m.settings_currencies_seeded_success());
    await loadCurrencies();
  } catch (e) {
    console.error("Error seeding financials:", e);
    const errorMsg = e instanceof Error ? e.message : String(e);
    toast.error(m.settings_currencies_seed_error({ error: errorMsg }));
  } finally {
    isSeeding = false;
  }
}

function startEditingRate(c: any) {
  if (c.is_base === 1) return;
  editingRateCode = c.code;
  editRateValue = c.exchange_rate;
}

function cancelEditingRate() {
  editingRateCode = null;
}

async function saveRateValue(code: string) {
  if (editRateValue <= 0) {
    toast.error(m.settings_currencies_invalid_rate());
    return;
  }
  try {
    await commands.finance.updateCurrency(orgId, code, {
      exchange_rate: editRateValue,
    });
    toast.success(m.settings_currencies_rate_updated_success());
    editingRateCode = null;
    await loadCurrencies();
  } catch (e) {
    console.error("Error updating exchange rate:", e);
    const errorMsg = e instanceof Error ? e.message : String(e);
    toast.error(m.settings_currencies_rate_update_error({ error: errorMsg }));
  }
}

// Find base currency representation
let baseCurrencyCode = $derived(
  currencies.find((c) => c.is_base === 1)?.code || "YER",
);
</script>

<svelte:head>
  <title>{m.settings_nav_currencies()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto bg-background p-6 flex flex-col gap-6 scrollbar-hide">
  <!-- Header -->
  <div class="flex items-center justify-between gap-4 pb-5 select-none border-b border-border/40 font-sans">
    <div class="flex items-center gap-2">
      {#if !isLoading}
        <span class="text-[10px] px-2 py-0.5 rounded bg-muted font-mono text-muted-foreground font-bold">
          {m.settings_currencies_count_label({ count: currencies.length })}
        </span>
      {/if}
    </div>

    <!-- Actions -->
    <div class="flex items-center gap-2">
      <Button
        variant="outline"
        size="sm"
        onclick={seedFinancials}
        disabled={isSeeding}
        class="h-9 text-xs flex items-center gap-1.5 cursor-pointer border-primary/30 hover:bg-primary/5 text-primary font-semibold"
      >
        <Sparkles size={13} class={isSeeding ? "animate-spin" : ""} />
        <span>{isSeeding ? m.settings_currencies_seeding() : m.settings_currencies_seed_btn()}</span>
      </Button>
      
      <Button
        variant="outline"
        size="sm"
        onclick={loadCurrencies}
        class="h-9 w-9 p-0 flex items-center justify-center cursor-pointer"
        title={m.common_control_refresh()}
      >
        <RefreshCw size={13} />
      </Button>

      <Button
        variant="lime"
        size="sm"
        onclick={openAddDialog}
        class="h-9 text-xs flex items-center gap-1.5 font-bold cursor-pointer"
      >
        <Plus size={14} />
        <span>{m.settings_currencies_add_btn()}</span>
      </Button>
    </div>
  </div>

  <!-- Info Alert Box -->
  <div class="border border-border/60 bg-muted/10 rounded-xl p-4 flex gap-3 text-start text-xs select-none hover:border-primary/25 transition-all duration-200">
    <Info class="text-primary shrink-0 animate-pulse" size={16} />
    <div class="flex flex-col gap-1 text-muted-foreground leading-relaxed">
      <span class="font-bold text-foreground text-xs">{m.settings_currencies_how_it_works_title()}</span>
      <span>
        {m.settings_currencies_how_it_works_desc({ baseCurrencyCode })}
      </span>
    </div>
  </div>

  <!-- Content Section -->
  <div class="w-full">
    {#if isLoading}
      <div class="flex flex-col gap-3">
        {#each Array(3) as _}
          <Skeleton class="h-14 w-full rounded-lg" />
        {/each}
      </div>
    {:else if currencies.length === 0}
      <!-- Zero state -->
      <div class="border border-dashed border-border/80 rounded-2xl p-16 flex flex-col items-center justify-center text-center gap-3 select-none my-4">
        <div class="w-14 h-14 rounded-full bg-primary/10 flex items-center justify-center text-primary mb-2">
          <Globe size={28} />
        </div>
        <h3 class="text-sm font-bold text-foreground">{m.settings_currencies_no_currencies_title()}</h3>
        <p class="text-xs text-muted-foreground max-w-xs">
          {m.settings_currencies_seed_hint()}
        </p>
        <Button variant="default" size="sm" onclick={seedFinancials} class="mt-2 cursor-pointer font-bold">
          {m.settings_currencies_seed_now_btn()}
        </Button>
      </div>
    {:else}
      <div in:fade={{ duration: 150 }} class="overflow-hidden">
        <Table>
          <TableHeader class="bg-muted/40 select-none">
            <TableRow>
              <TableHead class="text-start font-bold text-xs">{m.settings_currencies_col_code()}</TableHead>
              <TableHead class="text-start font-bold text-xs">{m.common_cycle_form_name()}</TableHead>
              <TableHead class="text-start font-bold text-xs">{m.common_user_form_name_en()}</TableHead>
              <TableHead class="text-start font-bold text-xs">{m.settings_currencies_col_symbol()}</TableHead>
              <TableHead class="text-start font-bold text-xs">{m.common_word_type()}</TableHead>
              <TableHead class="text-start font-bold text-xs">{m.common_word_status()}</TableHead>
              <TableHead class="text-end font-bold text-xs">{m.settings_currencies_col_rate()}</TableHead>
              <TableHead class="text-end font-bold text-xs w-[140px]">{m.common_word_actions()}</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {#each currencies as c (c.code)}
              <TableRow class="hover:bg-muted/10 transition-all duration-150">
                <TableCell class="text-start font-mono text-xs text-foreground font-bold">{c.code}</TableCell>
                <TableCell class="text-start text-xs font-semibold">{c.name_ar || "-"}</TableCell>
                <TableCell class="text-start text-xs text-muted-foreground">{c.name || "-"}</TableCell>
                <TableCell class="text-start font-mono text-xs">
                  <span class="bg-muted px-2 py-0.5 rounded border border-border/40 text-foreground font-bold">
                    {c.symbol}
                  </span>
                  <span class="text-[10px] text-muted-foreground ml-1">
                    ({c.symbol_position === 'before' ? m.settings_currencies_symbol_position_before() : m.settings_currencies_symbol_position_after()})
                  </span>
                </TableCell>
                <TableCell class="text-start text-xs">
                  {#if c.is_base === 1}
                    <Badge variant="success" class="text-[9px] font-bold">
                      {m.settings_currencies_type_base()}
                    </Badge>
                  {:else}
                    <Badge variant="secondary" class="text-[9px] font-semibold">
                      {m.settings_currencies_type_sub()}
                    </Badge>
                  {/if}
                </TableCell>
                <TableCell class="text-start text-xs">
                  {#if c.is_active === 1}
                    <Badge variant="success" class="text-[9px] font-bold">
                      {m.common_status_active()}
                    </Badge>
                  {:else}
                    <Badge variant="secondary" class="text-[9px] font-semibold">
                      {m.settings_currencies_status_inactive()}
                    </Badge>
                  {/if}
                </TableCell>
                <TableCell class="text-end text-xs font-bold font-mono">
                  {#if c.is_base === 1}
                    <span class="text-foreground select-none">1.0000</span>
                  {:else if editingRateCode === c.code}
                    <div class="flex items-center justify-end gap-1.5" transition:fade={{ duration: 100 }}>
                      <Input
                        type="number"
                        step="0.000001"
                        bind:value={editRateValue}
                        class="h-8 w-[100px] text-end font-mono text-xs font-bold bg-background border border-primary/60 focus:border-primary"
                      />
                      <span class="text-[10px] text-muted-foreground font-bold">{baseCurrencyCode}</span>
                      <Button
                        variant="default"
                        size="icon"
                        onclick={() => saveRateValue(c.code)}
                        class="h-7 w-7 bg-emerald-500 hover:bg-emerald-600 text-white cursor-pointer rounded-md shrink-0 flex items-center justify-center animate-in zoom-in-50"
                      >
                        <Check size={12} />
                      </Button>
                      <Button
                        variant="outline"
                        size="icon"
                        onclick={cancelEditingRate}
                        class="h-7 w-7 cursor-pointer rounded-md border-border/80 shrink-0 flex items-center justify-center animate-in zoom-in-50"
                      >
                        <X size={12} />
                      </Button>
                    </div>
                  {:else}
                    <button
                      onclick={() => startEditingRate(c)}
                      class="text-foreground hover:text-primary hover:underline cursor-pointer transition-colors duration-150 py-1 px-2 hover:bg-primary/5 rounded font-mono font-bold"
                      title={m.settings_currencies_edit_rate_tooltip()}
                    >
                      {c.exchange_rate.toFixed(4)}
                    </button>
                    <span class="text-[10px] text-muted-foreground font-bold mr-1">{baseCurrencyCode}</span>
                  {/if}
                </TableCell>
                <TableCell class="text-end text-xs">
                  <div class="flex items-center justify-end gap-1.5">
                    <Button
                      variant="outline"
                      size="icon"
                      onclick={() => openEditDialog(c)}
                      class="h-7 w-7 cursor-pointer hover:text-primary hover:bg-primary/5"
                      title={m.settings_currencies_edit_title()}
                    >
                      <Edit2 size={12} />
                    </Button>
                    
                    {#if c.is_base !== 1}
                      <Button
                        variant="outline"
                        size="icon"
                        onclick={() => confirmDeleteCurrency(c)}
                        class="h-7 w-7 cursor-pointer text-destructive hover:bg-destructive/10 border-destructive/20"
                        title={m.settings_currencies_delete_title()}
                      >
                        <Trash2 size={12} />
                      </Button>
                    {:else}
                      <span class="text-[9px] text-muted-foreground/60 italic px-1 select-none">
                        {m.settings_currencies_type_base()}
                      </span>
                    {/if}
                  </div>
                </TableCell>
              </TableRow>
            {/each}
          </TableBody>
        </Table>
      </div>
    {/if}
  </div>
</div>

<!-- Add Currency Modal -->
{#if showAddDialog}
  <div 
    in:fade={{ duration: 150 }} 
    class="fixed inset-0 z-50 bg-black/60 backdrop-blur-sm flex items-center justify-center p-4"
  >
    <form 
      onsubmit={handleAddCurrency}
      in:slide={{ duration: 200 }} 
      class="bg-card border border-border rounded-2xl w-full max-w-lg p-6 flex flex-col gap-5 text-start shadow-2xl overflow-y-auto max-h-[90vh]"
    >
      <div class="flex flex-col gap-1 border-b border-border/40 pb-3">
        <h2 class="text-sm font-bold text-foreground flex items-center gap-1.5">
          <Plus size={16} class="text-primary" />
          <span>{m.settings_currencies_add_dialog_title()}</span>
        </h2>
        <p class="text-[10px] text-muted-foreground">
          {m.settings_currencies_add_dialog_desc()}
        </p>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <!-- Currency Code -->
        <div class="flex flex-col gap-1.5">
          <label for="add_code" class="text-xs font-semibold text-muted-foreground">
            {m.settings_currencies_label_code_example()}
          </label>
          <Input
            id="add_code"
            type="text"
            bind:value={addCode}
            placeholder="USD"
            maxlength={5}
            required
            class="uppercase font-mono font-bold"
          />
        </div>

        <!-- Symbol -->
        <div class="flex flex-col gap-1.5">
          <label for="add_symbol" class="text-xs font-semibold text-muted-foreground">
            {m.settings_currencies_label_symbol_example()}
          </label>
          <Input
            id="add_symbol"
            type="text"
            bind:value={addSymbol}
            placeholder="$"
            required
            class="font-mono font-bold"
          />
        </div>

        <!-- Arabic Name -->
        <div class="flex flex-col gap-1.5">
          <label for="add_name_ar" class="text-xs font-semibold text-muted-foreground">
            {m.common_cycle_form_name()}
          </label>
          <Input
            id="add_name_ar"
            type="text"
            bind:value={addNameAr}
            placeholder="مثال: دولار أمريكي"
          />
        </div>

        <!-- English Name -->
        <div class="flex flex-col gap-1.5">
          <label for="add_name" class="text-xs font-semibold text-muted-foreground">
            {m.settings_currencies_label_name_en()}
          </label>
          <Input
            id="add_name"
            type="text"
            bind:value={addName}
            placeholder="e.g., US Dollar"
            required
          />
        </div>

        <!-- Symbol Position -->
        <div class="flex flex-col gap-1.5">
          <label for="add_position" class="text-xs font-semibold text-muted-foreground">
            {m.settings_currencies_label_position()}
          </label>
          <Select type="single" value={addSymbolPosition} onValueChange={(v) => (addSymbolPosition = v ?? "before")}>
            <SelectTrigger class="h-9 w-full text-xs">
              {addSymbolPosition === "before" ? m.settings_currencies_position_before() : m.settings_currencies_position_after()}
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="before">{m.settings_currencies_position_before()}</SelectItem>
              <SelectItem value="after">{m.settings_currencies_position_after()}</SelectItem>
            </SelectContent>
          </Select>
        </div>

        <!-- Decimal Places -->
        <div class="flex flex-col gap-1.5">
          <label for="add_decimals" class="text-xs font-semibold text-muted-foreground">
            {m.settings_currencies_label_decimals()}
          </label>
          <Input
            id="add_decimals"
            type="number"
            bind:value={addDecimalPlaces}
            min={0}
            max={8}
            required
          />
        </div>

        <!-- Exchange Rate -->
        <div class="flex flex-col gap-1.5 md:col-span-2">
          <label for="add_rate" class="text-xs font-semibold text-muted-foreground flex justify-between">
            <span>{m.settings_currencies_label_rate_with_base()}</span>
            <span class="text-xs font-bold text-primary">1 {addCode || 'CUR'} = {addExchangeRate} {baseCurrencyCode}</span>
          </label>
          <Input
            id="add_rate"
            type="number"
            step="0.000001"
            bind:value={addExchangeRate}
            min={0.000001}
            required
            class="font-mono font-bold"
          />
        </div>
      </div>

      <!-- Toggles -->
      <div class="flex flex-col gap-3 border-t border-border/40 pt-4 mt-2">
        <div class="flex items-center justify-between">
          <div class="flex flex-col gap-0.5">
            <span class="text-xs font-semibold text-foreground">{m.settings_currencies_base_currency()}</span>
            <span class="text-[9px] text-muted-foreground">
              {m.settings_currencies_base_currency_desc()}
            </span>
          </div>
          <Switch bind:checked={addIsBase} />
        </div>

        <div class="flex items-center justify-between border-t border-border/20 pt-3">
          <div class="flex flex-col gap-0.5">
            <span class="text-xs font-semibold text-foreground">{m.settings_currencies_active()}</span>
            <span class="text-[9px] text-muted-foreground">
              {m.settings_currencies_active_desc()}
            </span>
          </div>
          <Switch bind:checked={addIsActive} />
        </div>
      </div>

      <div class="flex items-center justify-end gap-2 border-t border-border/40 pt-4 mt-2">
        <Button variant="ghost" class="text-xs h-8 cursor-pointer" onclick={() => showAddDialog = false} type="button">
          {m.common_action_cancel()}
        </Button>
        <Button variant="lime" class="text-xs h-8 font-bold cursor-pointer" type="submit" disabled={isSubmitting}>
          {isSubmitting ? m.settings_currencies_btn_adding() : m.settings_currencies_btn_add()}
        </Button>
      </div>
    </form>
  </div>
{/if}

<!-- Edit Currency Modal -->
{#if showEditDialog}
  <div 
    in:fade={{ duration: 150 }} 
    class="fixed inset-0 z-50 bg-black/60 backdrop-blur-sm flex items-center justify-center p-4"
  >
    <form 
      onsubmit={handleEditCurrency}
      in:slide={{ duration: 200 }} 
      class="bg-card border border-border rounded-2xl w-full max-w-lg p-6 flex flex-col gap-5 text-start shadow-2xl overflow-y-auto max-h-[90vh]"
    >
      <div class="flex flex-col gap-1 border-b border-border/40 pb-3">
        <h2 class="text-sm font-bold text-foreground flex items-center gap-1.5">
          <Edit2 size={16} class="text-primary" />
          <span>{m.settings_currencies_edit_dialog_title({ code: editCode })}</span>
        </h2>
        <p class="text-[10px] text-muted-foreground">
          {m.settings_currencies_edit_dialog_desc()}
        </p>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <!-- Currency Code (Read Only) -->
        <div class="flex flex-col gap-1.5">
          <label for="edit_code" class="text-xs font-semibold text-muted-foreground">
            {m.settings_currencies_col_code()}
          </label>
          <Input
            id="edit_code"
            type="text"
            value={editCode}
            disabled
            class="font-mono font-bold bg-muted/60 opacity-80"
          />
        </div>

        <!-- Symbol -->
        <div class="flex flex-col gap-1.5">
          <label for="edit_symbol" class="text-xs font-semibold text-muted-foreground">
            {m.settings_currencies_label_symbol()}
          </label>
          <Input
            id="edit_symbol"
            type="text"
            bind:value={editSymbol}
            placeholder="$"
            required
            class="font-mono font-bold"
          />
        </div>

        <!-- Arabic Name -->
        <div class="flex flex-col gap-1.5">
          <label for="edit_name_ar" class="text-xs font-semibold text-muted-foreground">
            {m.common_cycle_form_name()}
          </label>
          <Input
            id="edit_name_ar"
            type="text"
            bind:value={editNameAr}
            placeholder="مثال: دولار أمريكي"
          />
        </div>

        <!-- English Name -->
        <div class="flex flex-col gap-1.5">
          <label for="edit_name" class="text-xs font-semibold text-muted-foreground">
            {m.settings_currencies_label_name_en()}
          </label>
          <Input
            id="edit_name"
            type="text"
            bind:value={editName}
            placeholder="e.g., US Dollar"
            required
          />
        </div>

        <!-- Symbol Position -->
        <div class="flex flex-col gap-1.5">
          <label for="edit_position" class="text-xs font-semibold text-muted-foreground">
            {m.settings_currencies_label_position()}
          </label>
          <Select type="single" value={editSymbolPosition} onValueChange={(v) => (editSymbolPosition = v ?? "before")}>
            <SelectTrigger class="h-9 w-full text-xs">
              {editSymbolPosition === "before" ? m.settings_currencies_position_before() : m.settings_currencies_position_after()}
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="before">{m.settings_currencies_position_before()}</SelectItem>
              <SelectItem value="after">{m.settings_currencies_position_after()}</SelectItem>
            </SelectContent>
          </Select>
        </div>

        <!-- Decimal Places -->
        <div class="flex flex-col gap-1.5">
          <label for="edit_decimals" class="text-xs font-semibold text-muted-foreground">
            {m.settings_currencies_label_decimals()}
          </label>
          <Input
            id="edit_decimals"
            type="number"
            bind:value={editDecimalPlaces}
            min={0}
            max={8}
            required
          />
        </div>

        <!-- Exchange Rate -->
        <div class="flex flex-col gap-1.5 md:col-span-2">
          <label for="edit_rate" class="text-xs font-semibold text-muted-foreground flex justify-between">
            <span>{m.settings_currencies_label_rate_with_base()}</span>
            {#if !editIsBase}
              <span class="text-xs font-bold text-primary">1 {editCode} = {editExchangeRate} {baseCurrencyCode}</span>
            {/if}
          </label>
          <Input
            id="edit_rate"
            type="number"
            step="0.000001"
            bind:value={editExchangeRate}
            min={0.000001}
            disabled={editIsBase}
            required
            class="font-mono font-bold disabled:opacity-60"
          />
        </div>
      </div>

      <!-- Toggles -->
      <div class="flex flex-col gap-3 border-t border-border/40 pt-4 mt-2">
        <div class="flex items-center justify-between">
          <div class="flex flex-col gap-0.5">
            <span class="text-xs font-semibold text-foreground">{m.settings_currencies_base_currency()}</span>
            <span class="text-[9px] text-muted-foreground">
              {m.settings_currencies_base_currency_desc()}
            </span>
          </div>
          <Switch bind:checked={editIsBase} disabled={editIsBase && editCode === 'YER' /* prevent changing seed base YER directly */} />
        </div>

        <div class="flex items-center justify-between border-t border-border/20 pt-3">
          <div class="flex flex-col gap-0.5">
            <span class="text-xs font-semibold text-foreground">{m.settings_currencies_active()}</span>
            <span class="text-[9px] text-muted-foreground">
              {m.settings_currencies_active_desc()}
            </span>
          </div>
          <Switch bind:checked={editIsActive} disabled={editIsBase /* Base currency must be active */} />
        </div>
      </div>

      <div class="flex items-center justify-end gap-2 border-t border-border/40 pt-4 mt-2">
        <Button variant="ghost" class="text-xs h-8 cursor-pointer" onclick={() => showEditDialog = false} type="button">
          {m.common_action_cancel()}
        </Button>
        <Button variant="lime" class="text-xs h-8 font-bold cursor-pointer" type="submit" disabled={isSubmitting}>
          {isSubmitting ? m.settings_security_updating() : m.settings_action_save_changes()}
        </Button>
      </div>
    </form>
  </div>
{/if}

<!-- Delete Confirmation Dialog -->
{#if currencyToDelete}
  <div 
    in:fade={{ duration: 150 }} 
    class="fixed inset-0 z-50 bg-black/60 backdrop-blur-sm flex items-center justify-center p-4"
  >
    <div 
      in:slide={{ duration: 200 }} 
      class="bg-card border border-border rounded-2xl w-full max-w-md p-6 flex flex-col gap-4 text-start shadow-2xl"
    >
      <div class="flex flex-col gap-1">
        <h3 class="text-sm font-bold text-destructive flex items-center gap-1.5">
          <Trash2 size={16} />
          <span>{m.settings_currencies_delete_dialog_title()}</span>
        </h3>
        <p class="text-xs text-foreground mt-2 font-semibold">
          {m.settings_currencies_delete_dialog_confirm({ code: currencyToDelete.code, name: l(currencyToDelete.name_ar, currencyToDelete.name) })}
        </p>
        <p class="text-[10px] text-muted-foreground leading-relaxed mt-1">
          {m.settings_currencies_delete_dialog_warning()}
        </p>
      </div>

      <div class="flex items-center justify-end gap-2 border-t border-border/40 pt-4 mt-2">
        <Button variant="ghost" class="text-xs h-8 cursor-pointer" onclick={() => currencyToDelete = null}>
          {m.common_td_undo()}
        </Button>
        <Button variant="destructive" class="text-xs h-8 font-bold cursor-pointer" onclick={handleDeleteCurrency}>
          {m.settings_currencies_btn_delete_confirm()}
        </Button>
      </div>
    </div>
  </div>
{/if}
