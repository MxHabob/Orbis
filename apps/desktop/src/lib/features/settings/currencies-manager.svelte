<!--
@component CurrenciesManager
Lists and manages all multi-currency exchange rates with OrbisGrid,
dialogs for creation and configuration.
Matches the design, layout, and UX of Rooms and Subjects managers.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api } from "$lib/api";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { toast } from "@orbis/ui";
import { confirmStore } from "$lib/stores";
import { l } from "$lib/core/i18n";
import { getLocale } from "$lib/paraglide/runtime";
import { Plus, Search, Refresh } from "@orbis/icons";
import { OrbisGrid, type OrbisColumnDef, type GridActionEvent, type ActionItem } from "@orbis/svelte-grid";
import { FilterTrigger, type UniversalFilter } from "@orbis/ui/filter-builder";
import { currencyFilterFields } from "$lib/core/filter-configs";
import AddCurrencyDialog from "./components/add-currency-dialog.svelte";

let { orgId }: { orgId: string } = $props();

// State
let currencies = $state<any[]>([]);
let isLoading = $state(true);
let searchQuery = $state("");
let showAddDialog = $state(false);
let activeFilter = $state<UniversalFilter>({
  logic: "AND",
  conditions: [],
  groups: [],
});

const currencyActions: ActionItem[] = [
  { id: "edit", label: "Edit", icon: "Pencil" },
  { id: "delete", label: "Delete", icon: "Trash2", variant: "destructive" },
];

async function loadCurrencies() {
  if (!orgId) return;
  isLoading = true;
  try {
    currencies = await api.listCurrencies();
  } catch (e) {
    toast.error(m.settings_currencies_load_error());
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (orgId) loadCurrencies();
});

const gridSource = $derived(
  currencies.map((c, i) => ({
    ...c,
    __idx: i + 1,
    name: l(c.name_ar, c.name) || c.name || c.name_ar || c.code,
    name_en: c.name || "—",
    symbol: c.symbol || c.code,
    is_base_display: c.is_base === true || c.is_base === 1 ? m.settings_currencies_type_base() : m.settings_currencies_type_sub(),
    exchange_rate: Number(c.exchange_rate ?? 1),
    decimal_places: c.decimal_places ?? 2,
    status: c.is_active === false || c.is_active === 0 ? "inactive" : "active",
  })),
);

const columns: OrbisColumnDef[] = [
  { id: "__idx", prop: "__idx", header: "#", name: "#", width: 48, size: 48, readonly: true, pin: "left", align: "center" },
  { id: "code", prop: "code", header: m.common_ledger_col_code(), name: m.common_ledger_col_code(), width: 90, size: 90, readonly: true, pin: "left", cellClass: "font-mono font-bold" },
  { id: "name", prop: "name", header: m.common_cycle_form_name(), name: m.common_cycle_form_name(), width: 190, size: 190, readonly: true, cellClass: "font-medium" },
  { id: "name_en", prop: "name_en", header: m.common_user_form_name_en(), name: m.common_user_form_name_en(), width: 160, size: 160, readonly: true },
  { id: "symbol", prop: "symbol", header: m.settings_currencies_col_symbol(), name: m.settings_currencies_col_symbol(), width: 90, size: 90, readonly: true, align: "center", cellClass: "font-mono font-semibold" },
  { id: "is_base_display", prop: "is_base_display", header: m.common_word_type(), name: m.common_word_type(), width: 110, size: 110, readonly: true, align: "center", type: "badge" },
  { id: "exchange_rate", prop: "exchange_rate", header: m.settings_currencies_col_rate(), name: m.settings_currencies_col_rate(), width: 130, size: 130, readonly: true, align: "end", type: "number" },
  { id: "decimal_places", prop: "decimal_places", header: "Decimals", name: "Decimals", width: 90, size: 90, readonly: true, align: "center", type: "number" },
  { id: "status", prop: "status", header: m.common_word_status(), name: m.common_word_status(), width: 100, size: 100, type: "badge", align: "center", readonly: true },
  {
    id: "actions",
    prop: "actions",
    header: m.common_je_col_actions(),
    name: "Actions",
    width: 90,
    size: 90,
    type: "action",
    align: "center",
    readonly: true,
    meta: { items: currencyActions },
  },
];

function openAdd() {
  showAddDialog = true;
}

async function handleGridAction(event: GridActionEvent) {
  const c = event.row;
  if (event.actionId === "dblclick" || event.actionId === "edit") {
    showAddDialog = true;
  } else if (event.actionId === "delete") {
    if (c.is_base === true || c.is_base === 1) {
      toast.error(m.settings_currencies_delete_base_error());
      return;
    }
    const confirmed = await confirmStore.prompt({
      title: m.settings_currencies_delete_dialog_title(),
      description: m.settings_currencies_delete_dialog_confirm({
        code: c.code,
        name: l(c.name_ar, c.name) || c.code,
      }),
      confirmText: m.settings_currencies_btn_delete_confirm(),
      variant: "destructive",
    });
    if (!confirmed) return;
    try {
      toast.info(m.settings_currencies_deleted_success({ code: c.code }));
      await loadCurrencies();
    } catch (e: any) {
      toast.error(e?.message || "Failed to delete currency");
    }
  }
}
</script>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-4 scrollbar-hide font-sans">
  <div class="flex items-center justify-between gap-4 flex-nowrap shrink-0">
    <div class="relative flex-1 max-w-xs">
      <Search
        size={14}
        class="absolute inset-s-2.5 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none"
      />
      <Input
        type="text"
        placeholder={m.settings_ui_currency_search_placeholder()}
        bind:value={searchQuery}
        class="h-8 ps-8 text-xs bg-card border-border/60"
      />
    </div>

    <div class="flex items-center gap-2">
      <FilterTrigger
        bind:filter={activeFilter}
        fields={currencyFilterFields}
      />
      <Button variant="ghost" size="icon-sm" onclick={loadCurrencies} class="text-muted-foreground">
        <Refresh size={14} class={isLoading ? "animate-spin" : ""} />
      </Button>
      <Button variant="lime" class="text-xs h-8 px-4 font-semibold shrink-0" onclick={openAdd}>
        <Plus size={14} class="me-1.5" />
        {m.settings_currencies_add_btn()}
      </Button>
    </div>
  </div>

  <div class="flex-1 min-h-0">
    <OrbisGrid
      {columns}
      source={gridSource}
      loading={isLoading}
      clientQuery
      search={searchQuery}
      filter={activeFilter}
      emptyTitle={m.settings_currencies_no_currencies_title()}
      emptyDescription={m.settings_currencies_seed_hint()}
      rtl={getLocale() === "ar"}
      readonly
      onaction={handleGridAction}
    />
  </div>
</div>

<AddCurrencyDialog
  bind:open={showAddDialog}
  existingCurrencies={currencies}
  onsuccess={loadCurrencies}
/>
