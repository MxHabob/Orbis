<!--
  @page FeeTemplatesManagementPage
  Lists all fee templates with OrbisGrid, allowing CRUD through FeeTemplateSheet.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api } from "$lib/api";
import { Button } from "@orbis/ui/button";
import { Plus, Receipt } from "@lucide/svelte";
import { getLocale } from "$lib/paraglide/runtime";
import { formatCurrency } from "$lib/core/i18n";
import { toast } from "@orbis/ui";
import FeeTemplateSheet from "$lib/features/finance/components/fee-template-sheet.svelte";

import { DataTable } from "$lib/tables";
import { feeTemplates as ftDef, withIndex } from "$lib/tables/definitions";
import type { UniversalFilter } from "@orbis/ui/filter-builder";
import { feeTemplateFilterFields } from "$lib/tables/definitions/filter-configs";
import { applyFilter } from "@orbis/ui/filter-builder";

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
    templates = await api.listFeeTemplates();
  } catch (e) {
    toast.error((e as any)?.message || "Failed to load fee templates");
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (data.orgId) loadTemplates();
});

// ── Helpers ─────────────────────────────────────────────────────────────

function getCategoryLabel(catStr: string) {
  const c = String(catStr).toLowerCase();
  if (c === "tuition") return m.common_ft_category_tuition?.() || "Tuition";
  if (c === "transport")
    return m.common_ft_category_transport?.() || "Transport";
  if (c === "books") return m.common_ft_category_books?.() || "Books";
  if (c === "uniform") return m.common_ft_category_uniform?.() || "Uniform";
  if (c === "exam") return m.common_ft_category_exam?.() || "Exam";
  return m.common_guardians_relationship_other?.() || "Other";
}

function getFrequencyLabel(freqStr: string) {
  const f = String(freqStr).toLowerCase();
  if (f === "once") return m.common_ft_frequency_once?.() || "Once";
  if (f === "monthly") return m.common_ft_frequency_monthly?.() || "Monthly";
  if (f === "termly") return m.common_ft_frequency_termly?.() || "Termly";
  if (f === "yearly") return m.common_ft_frequency_yearly?.() || "Yearly";
  return freqStr;
}

const filteredTemplates = $derived(applyFilter(templates, activeFilter));

// ── Grid Source ─────────────────────────────────────────────────────────

const gridSource = $derived(
  withIndex(
    filteredTemplates.map((tpl) => ({
      ...tpl,
      name: tpl.name_ar || tpl.name || "—",
      amount: formatCurrency(Number(tpl.default_amount || 0), "YER"),
      frequency: getFrequencyLabel(tpl.frequency),
      status: tpl.is_active === 1 ? "active" : "inactive",
      class_count: getCategoryLabel(tpl.category),
    })),
  ),
);

const columns = $derived(
  ftDef.columns({
    idx: "#",
    name: m.common_ft_col_name?.() || "Template",
    amount: m.common_ft_col_amount?.() || "Amount",
    frequency: m.common_ft_col_frequency?.() || "Frequency",
    status: m.common_word_status?.() || "Status",
    classes: m.common_ft_col_category?.() || "Category",
  }),
);

// ── Sheet Controls ──────────────────────────────────────────────────────

import type { GridActionEvent } from "@orbis/svelte-grid";

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
        <Receipt class="text-primary" size={18} />
        <span>{m.common_ft_title?.() || "Fee Templates"}</span>
      </h1>
      <p class="text-xs text-muted-foreground">
        {m.common_ft_desc?.() || "Manage fee templates for student enrollment"}
      </p>
    </div>
    <Button size="sm" onclick={handleCreate} class="text-xs font-bold shrink-0">
      <Plus size={14} class="me-1" />
      {m.common_ft_add?.() || "Add Template"}
    </Button>
  </div>

  <div class="flex-1 min-h-0">
    <DataTable
      {columns}
      source={gridSource}
      loading={isLoading}
      filterable={true}
      filterFields={feeTemplateFilterFields}
      bind:filterValue={activeFilter}
      emptyIcon={Receipt}
      emptyTitle={m.common_ft_empty?.() || "No fee templates"}
      emptyDescription={m.common_ft_empty_desc?.() || "Create fee templates to auto-apply on enrollment"}
      rtl={getLocale() === "ar"}
      refreshable
      onRefresh={loadTemplates}
      onaction={handleGridAction}
    />
  </div>
</div>

<FeeTemplateSheet
  bind:open={isSheetOpen}
  orgId={data.orgId}
  template={selectedTemplate}
  onSuccess={loadTemplates}
/>
