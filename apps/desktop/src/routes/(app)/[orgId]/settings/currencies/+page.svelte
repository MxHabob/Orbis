<!--
  @page CurrenciesSettingsPage
  Allows administrators to manage multi-currency exchange rates, define base currencies,
  add new currencies, edit currency configurations, and delete unused currencies.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { onMount } from "svelte";
import { api } from "$lib/api";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@orbis/ui";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import { Badge } from "@orbis/ui/badge";
import { Button } from "@orbis/ui/button";
import { FormField } from "@orbis/ui";
import { Input } from "@orbis/ui/input";
import { confirmStore } from "$lib/stores";
import { Switch } from "@orbis/ui/switch";
import { Skeleton } from "@orbis/ui/skeleton";
import { toast } from "@orbis/ui";
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
} from "@lucide/svelte";
import { fade, slide } from "svelte/transition";

import { l, isAr } from "$lib/core/i18n";
import AddCurrencyDialog from "$lib/features/settings/components/add-currency-dialog.svelte";

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
    currencies = await api.listCurrencies();
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
  showAddDialog = true;
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
    await api.finance.updateCurrency(editCode, editExchangeRate, editIsActive);
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

async function handleDeleteCurrency(currencyToDelete: any) {
  if (currencyToDelete.is_base === 1) {
    toast.error(m.settings_currencies_delete_base_error());
    return;
  }
  const confirmed = await confirmStore.prompt({
    title: m.settings_currencies_delete_dialog_title(),
    description:
      m.settings_currencies_delete_dialog_confirm({
        code: currencyToDelete.code,
        name: l(currencyToDelete.name_ar, currencyToDelete.name),
      }) +
      "\n" +
      m.settings_currencies_delete_dialog_warning(),
    confirmText: m.settings_currencies_btn_delete_confirm(),
    variant: "destructive",
  });
  if (!confirmed) return;
  try {
    await api.finance.deleteCurrency(currencyToDelete.code);
    toast.success(
      m.settings_currencies_deleted_success({ code: currencyToDelete.code }),
    );
    await loadCurrencies();
  } catch (e) {
    console.error("Error deleting currency:", e);
    const errorMsg = e instanceof Error ? e.message : String(e);
    toast.error(m.settings_currencies_delete_error({ error: errorMsg }));
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
    await api.finance.updateCurrency(code, editRateValue, null);
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

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 scrollbar-hide">
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
      </div>
    {:else}
      <div in:fade={{ duration: 150 }} class="overflow-hidden">
        <Table>
          <TableHeader class="bg-muted/40 select-none">
            <TableRow>
              <TableHead class="text-start font-bold text-xs">{m.common_ledger_col_code()}</TableHead>
              <TableHead class="text-start font-bold text-xs">{m.common_cycle_form_name()}</TableHead>
              <TableHead class="text-start font-bold text-xs">{m.common_user_form_name_en()}</TableHead>
              <TableHead class="text-start font-bold text-xs">{m.settings_currencies_col_symbol()}</TableHead>
              <TableHead class="text-start font-bold text-xs">{m.common_word_type()}</TableHead>
              <TableHead class="text-start font-bold text-xs">{m.common_word_status()}</TableHead>
              <TableHead class="text-end font-bold text-xs">{m.settings_currencies_col_rate()}</TableHead>
              <TableHead class="text-end font-bold text-xs w-35">{m.common_je_col_actions()}</TableHead>
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
                      {m.common_phase_active()}
                    </Badge>
                  {:else}
                    <Badge variant="secondary" class="text-[9px] font-semibold">
                      {m.common_status_disabled()}
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
                        class="h-8 w-25 text-end font-mono text-xs font-bold bg-background border border-primary/60 focus:border-primary"
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
                        onclick={() => handleDeleteCurrency(c)}
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
<!-- Add Currency Modal -->
<AddCurrencyDialog
  bind:open={showAddDialog}
  existingCurrencies={currencies}
  onsuccess={loadCurrencies}
/>

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
        <FormField id="edit_code" label={m.common_ledger_col_code()} required>
          {#snippet children()}
          <Input
            id="edit_code"
            type="text"
            value={editCode}
            disabled
            class="font-mono font-bold bg-muted/60 opacity-80"
          />
          {/snippet}
        </FormField>

        <!-- Symbol -->
        <FormField id="edit_symbol" label={m.settings_currencies_label_symbol()} required>
          {#snippet children()}
          <Input
            id="edit_symbol"
            type="text"
            bind:value={editSymbol}
            placeholder="$"
            required
            class="font-mono font-bold"
          />
          {/snippet}
        </FormField>

        <!-- Arabic Name -->
        <FormField id="edit_name_ar" label={m.common_cycle_form_name()}>
          {#snippet children()}
          <Input
            id="edit_name_ar"
            type="text"
            bind:value={editNameAr}
            placeholder={m.settings_currencies_name_ar_placeholder()}
          />
          {/snippet}
        </FormField>

        <!-- English Name -->
        <FormField id="edit_name" label={m.settings_currencies_label_name_en()} required>
          {#snippet children()}
          <Input
            id="edit_name"
            type="text"
            bind:value={editName}
            placeholder="e.g., US Dollar"
            required
          />
          {/snippet}
        </FormField>

        <!-- Symbol Position -->
        <FormField id="edit_position" label={m.settings_currencies_label_position()} required>
          {#snippet children()}
          <Select type="single" value={editSymbolPosition} onValueChange={(v: string | undefined) => (editSymbolPosition = v ?? "before")}>
            <SelectTrigger class="h-9 w-full text-xs">
              {editSymbolPosition === "before" ? m.settings_currencies_position_before() : m.settings_currencies_position_after()}
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="before">{m.settings_currencies_position_before()}</SelectItem>
              <SelectItem value="after">{m.settings_currencies_position_after()}</SelectItem>
            </SelectContent>
          </Select>
          {/snippet}
        </FormField>

        <!-- Decimal Places -->
        <FormField id="edit_decimals" label={m.settings_currencies_label_decimals()} required>
          {#snippet children()}
          <Input
            id="edit_decimals"
            type="number"
            bind:value={editDecimalPlaces}
            min={0}
            max={8}
            required
          />
          {/snippet}
        </FormField>

        <!-- Exchange Rate -->
        <FormField id="edit_rate" label={m.settings_currencies_label_rate_with_base()} required>
          {#snippet children()}
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
          {/snippet}
        </FormField>
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
            <span class="text-xs font-semibold text-foreground">{m.common_phase_active()}</span>
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


