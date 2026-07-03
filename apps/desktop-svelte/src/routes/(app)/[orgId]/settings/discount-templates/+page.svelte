<!--
  @page DiscountTemplatesPage
  Lists all discount/scholarship templates with OrbisGrid, CRUD via DiscountTemplateSheet.
-->
<script lang="ts">
import { onMount } from "svelte";
import { commands } from "$lib/services/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Plus, Percent } from "lucide-svelte";
import { getLocale } from "$lib/paraglide/runtime";
import { formatCurrency } from "$lib/i18n-utils";
import { toast } from "svelte-sonner";
import DiscountTemplateSheet from "$lib/components/sheets/discount-template-sheet.svelte";
import * as m from "$lib/paraglide/messages";

import { DataTable } from "$lib/tables";
import { discountTemplates as dtDef, withIndex } from "$lib/tables/definitions";

let { data }: { data: { orgId: string } } = $props();

// ── State ──────────────────────────────────────────────────────────────
let templates = $state<any[]>([]);
let isLoading = $state(true);
let isSheetOpen = $state(false);
let selectedTemplate = $state<any | null>(null);

// ── Data Loading ────────────────────────────────────────────────────────

async function loadTemplates() {
  isLoading = true;
  try {
    templates = await commands.finance.getDiscountTemplates(data.orgId);
  } catch (e) {
    toast.error((e as any)?.message || "Failed to load discount templates");
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (data.orgId) loadTemplates();
});

// ── Helpers ─────────────────────────────────────────────────────────────

function getMethodLabel(method: string): string {
  const mLow = String(method).toLowerCase();
  if (mLow === "percentage")
    return m.dt_method_percentage?.() || "Percentage (%)";
  return m.dt_method_fixed?.() || "Fixed Amount";
}

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

// ── Grid Source ─────────────────────────────────────────────────────────

const gridSource = $derived(
  withIndex(
    templates.map((tpl) => ({
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
    name: m.dt_col_name?.() || "Template",
    type: m.dt_col_type?.() || "Type",
    value: m.dt_col_value?.() || "Value",
    status: m.dt_col_status?.() || "Status",
  }),
);

function handleCreate() {
  selectedTemplate = null;
  isSheetOpen = true;
}
</script>

<div class="h-full flex flex-col overflow-hidden bg-background p-6">
  <div class="flex items-center justify-between pb-4 flex-nowrap shrink-0">
    <div class="flex flex-col gap-0.5 text-start">
      <h1 class="text-lg font-bold tracking-tight text-foreground flex items-center gap-2">
        <Percent class="text-primary" size={18} />
        <span>{m.dt_title?.() || "Discount Templates"}</span>
      </h1>
      <p class="text-xs text-muted-foreground">
        {m.dt_desc?.() || "Manage discount and scholarship templates"}
      </p>
    </div>
    <Button size="sm" onclick={handleCreate} class="text-xs font-bold shrink-0">
      <Plus size={14} class="me-1" />
      {m.dt_add?.() || "Add Template"}
    </Button>
  </div>

  <div class="flex-1 min-h-0">
    <DataTable
      {columns}
      source={gridSource}
      loading={isLoading}
      emptyIcon={Percent}
      emptyTitle={m.dt_empty?.() || "No discount templates"}
      emptyDescription={m.dt_empty_desc?.() || "Create discount templates for student fees"}
      rtl={getLocale() === "ar"}
      refreshable
      onRefresh={loadTemplates}
    />
  </div>
</div>

<DiscountTemplateSheet
  bind:open={isSheetOpen}
  orgId={data.orgId}
  template={selectedTemplate}
  onSuccess={loadTemplates}
/>
