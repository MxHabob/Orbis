<!--
  @page DiscountTemplatesPage
  Lists all discount/scholarship templates with OrbisGrid, CRUD via DiscountTemplateSheet.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api } from "$lib/api";
import { Button } from "@orbis/ui/button";
import { Plus, Percent } from "@lucide/svelte";
import { getLocale } from "$lib/paraglide/runtime";
import { formatCurrency } from "$lib/core/i18n";
import { toast } from "@orbis/ui";
import DiscountTemplateSheet from "$lib/features/finance/components/discount-template-sheet.svelte";

import { DataTable } from "$lib/tables";
import { discountTemplates as dtDef, withIndex } from "$lib/tables/definitions";
import type { UniversalFilter } from "@orbis/ui/filter-builder";
import { discountTemplateFilterFields } from "$lib/tables/definitions/filter-configs";
import { applyFilter } from "@orbis/ui/filter-builder";
import type { GridActionEvent } from "@orbis/svelte-grid";

let { data }: { data: { orgId: string } } = $props();

// ── State ──────────────────────────────────────────────────────────────
let templates = $state<any[]>([]);
let isLoading = $state(true);
let isSheetOpen = $state(false);
let selectedTemplate = $state<any | null>(null);
let activeFilter = $state<UniversalFilter>({
  logic: "AND",
  conditions: [],
  groups: [],
});

// ── Data Loading ────────────────────────────────────────────────────────

async function loadTemplates() {
  isLoading = true;
  try {
    templates = await api.listDiscountTemplates();
  } catch (e) {
    toast.error((e as any)?.message || "Failed to load discount templates");
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (data.orgId) loadTemplates();
});

function getFormattedValue(tpl: any): string {
  const val = Number(tpl.value || 0);
  if (String(tpl.calculation_method).toLowerCase() === "percentage")
    return `${val}%`;
  return formatCurrency(val, "YER");
}

function getTypeLabel(typeStr: string): string {
  const t = String(typeStr).toLowerCase();
  if (t === "manual") return "Manual";
  if (t === "scholarship") return "Scholarship";
  if (t === "system") return "System";
  return typeStr;
}

const filteredTemplates = $derived(applyFilter(templates, activeFilter));

// ── Grid Source ─────────────────────────────────────────────────────────

const gridSource = $derived(
  withIndex(
    filteredTemplates.map((tpl) => ({
      ...tpl,
      name: tpl.name_ar || tpl.name || "—",
      discount_type: getTypeLabel(tpl.discount_type || tpl.type || ""),
      value: getFormattedValue(tpl),
      status: tpl.is_active === 1 ? "active" : "inactive",
    })),
  ),
);

const columns = $derived(
  dtDef.columns({
    idx: "#",
    name: m.common_ft_col_name?.() || "Template",
    type: m.common_word_type?.() || "Type",
    value: m.common_dt_col_value?.() || "Value",
    status: m.common_word_status?.() || "Status",
  }),
);

function handleCreate() {
  selectedTemplate = null;
  isSheetOpen = true;
}

function handleGridAction(event: GridActionEvent) {
  if (event.actionId === "dblclick" || event.actionId === "edit") {
    selectedTemplate = event.row as any;
    isSheetOpen = true;
  }
}
</script>

<div class="h-full flex flex-col overflow-hidden p-6">
  <div class="flex items-center justify-between pb-4 flex-nowrap shrink-0">
    <div class="flex flex-col gap-0.5 text-start">
      <h1 class="text-lg font-bold tracking-tight text-foreground flex items-center gap-2">
        <Percent class="text-primary" size={18} />
        <span>{m.common_dt_title?.() || "Discount Templates"}</span>
      </h1>
      <p class="text-xs text-muted-foreground">
        {m.common_dt_desc?.() || "Manage discount and scholarship templates"}
      </p>
    </div>
    <Button size="sm" onclick={handleCreate} class="text-xs font-bold shrink-0">
      <Plus size={14} class="me-1" />
      {m.common_dt_add?.() || "Add Template"}
    </Button>
  </div>

  <div class="flex-1 min-h-0">
    <DataTable
      {columns}
      source={gridSource}
      loading={isLoading}
      filterable={true}
      filterFields={discountTemplateFilterFields}
      bind:filterValue={activeFilter}
      emptyIcon={Percent}
      emptyTitle={m.common_dt_empty?.() || "No discount templates"}
      emptyDescription={m.common_dt_empty_desc?.() || "Create standard discounts or scholarships"}
      rtl={getLocale() === "ar"}
      refreshable
      onRefresh={loadTemplates}
      onaction={handleGridAction}
    />
  </div>
</div>

<DiscountTemplateSheet
  bind:open={isSheetOpen}
  orgId={data.orgId}
  template={selectedTemplate}
  onSuccess={loadTemplates}
/>
