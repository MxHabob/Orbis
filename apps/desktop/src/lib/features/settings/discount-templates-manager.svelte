<!--
@component DiscountTemplatesManager
Lists all discount/scholarship templates with OrbisGrid, CRUD via DiscountTemplateSheet.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api } from "$lib/api";
import { Button } from "@orbis/ui/button";
import { Plus, ReceiptPercent, Refresh } from "@orbis/icons";
import { getLocale } from "$lib/paraglide/runtime";
import { formatCurrency } from "$lib/core/i18n";
import { toast } from "@orbis/ui";
import DiscountTemplateSheet from "$lib/features/finance/components/discount-template-sheet.svelte";
import { OrbisGrid, type OrbisColumnDef, type GridActionEvent } from "@orbis/svelte-grid";
import { FilterTrigger, type UniversalFilter } from "@orbis/ui/filter-builder";
import { discountTemplateFilterFields } from "$lib/core/filter-configs";

let { orgId }: { orgId: string } = $props();

// State
let templates = $state<any[]>([]);
let isLoading = $state(true);
let isSheetOpen = $state(false);
let selectedTemplate = $state<any | null>(null);
let activeFilter = $state<UniversalFilter>({
  logic: "AND",
  conditions: [],
  groups: [],
});

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
  if (orgId) loadTemplates();
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

const gridSource = $derived(
  templates.map((tpl, i) => ({
    ...tpl,
    __idx: i + 1,
    name: tpl.name_ar || tpl.name || "—",
    discount_type: getTypeLabel(tpl.discount_type || tpl.type || ""),
    value: getFormattedValue(tpl),
    status: tpl.is_active === 1 ? "active" : "inactive",
  })),
);

const columns: OrbisColumnDef[] = [
  { id: "__idx", prop: "__idx", header: "#", name: "#", width: 48, size: 48, readonly: true, pin: "left", align: "center" },
  { id: "name", prop: "name", header: m.common_ft_col_name?.() || "Template", name: "Template", width: 220, size: 220, readonly: true, cellClass: "font-medium" },
  { id: "discount_type", prop: "discount_type", header: m.common_word_type?.() || "Type", name: "Type", width: 130, size: 130, readonly: true },
  { id: "value", prop: "value", header: m.common_dt_col_value?.() || "Value", name: "Value", width: 120, size: 120, readonly: true, align: "end", cellClass: "font-mono font-bold" },
  { id: "status", prop: "status", header: m.common_word_status?.() || "Status", name: "Status", width: 110, size: 110, type: "badge", align: "center", readonly: true },
];

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

<div class="h-full flex flex-col overflow-hidden p-6 gap-4">
  <div class="flex items-center justify-between pb-2 flex-nowrap shrink-0">
    <div class="flex flex-col gap-0.5 text-start">
      <h1 class="text-lg font-bold tracking-tight text-foreground flex items-center gap-2">
        <ReceiptPercent class="text-primary" size={18} />
        <span>{m.common_dt_title?.() || "Discount Templates"}</span>
      </h1>
      <p class="text-xs text-muted-foreground">
        {m.common_dt_desc?.() || "Manage discount and scholarship templates"}
      </p>
    </div>
    <div class="flex items-center gap-2">
      <FilterTrigger
        bind:filter={activeFilter}
        fields={discountTemplateFilterFields}
      />
      <Button variant="ghost" size="icon-sm" onclick={loadTemplates} class="text-muted-foreground">
        <Refresh size={14} class={isLoading ? "animate-spin" : ""} />
      </Button>
      <Button size="sm" onclick={handleCreate} class="text-xs font-bold shrink-0">
        <Plus size={14} class="me-1" />
        {m.common_dt_add?.() || "Add Template"}
      </Button>
    </div>
  </div>

  <div class="flex-1 min-h-0">
    <OrbisGrid
      {columns}
      source={gridSource}
      loading={isLoading}
      clientQuery
      filter={activeFilter}
      emptyTitle={m.common_dt_empty?.() || "No discount templates"}
      emptyDescription={m.common_dt_empty_desc?.() || "Create standard discounts or scholarships"}
      rtl={getLocale() === "ar"}
      readonly
      onaction={handleGridAction}
    />
  </div>
</div>

<DiscountTemplateSheet
  bind:open={isSheetOpen}
  {orgId}
  template={selectedTemplate}
  onSuccess={loadTemplates}
/>
