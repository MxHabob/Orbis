<!--
@component FeeTemplatesManager
Lists all fee templates with OrbisGrid, allowing CRUD through FeeTemplateSheet.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api } from "$lib/api";
import { Button } from "@orbis/ui/button";
import { DocumentPage, Plus, Refresh } from "@orbis/icons";
import { getLocale } from "$lib/paraglide/runtime";
import { formatCurrency } from "$lib/core/i18n";
import { toast } from "@orbis/ui";
import FeeTemplateSheet from "$lib/features/finance/components/fee-template-sheet.svelte";
import { OrbisGrid, type OrbisColumnDef, type GridActionEvent } from "@orbis/svelte-grid";
import { FilterTrigger, type UniversalFilter } from "@orbis/ui/filter-builder";
import { feeTemplateFilterFields } from "$lib/core/filter-configs";

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
    templates = await api.listFeeTemplates();
  } catch (e) {
    toast.error((e as any)?.message || "Failed to load fee templates");
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (orgId) loadTemplates();
});

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

const gridSource = $derived(
  templates.map((tpl, i) => ({
    ...tpl,
    __idx: i + 1,
    name: tpl.name_ar || tpl.name || "—",
    amount: formatCurrency(Number(tpl.default_amount || 0), "YER"),
    frequency: getFrequencyLabel(tpl.frequency),
    status: (tpl.is_active === true || tpl.is_active === 1) ? "active" : "inactive",
    class_count: getCategoryLabel(tpl.category),
  })),
);

const columns: OrbisColumnDef[] = [
  { id: "__idx", prop: "__idx", header: "#", name: "#", width: 48, size: 48, readonly: true, pin: "left", align: "center" },
  { id: "name", prop: "name", header: m.common_ft_col_name?.() || "Template", name: "Template", width: 220, size: 220, readonly: true, cellClass: "font-medium" },
  { id: "class_count", prop: "class_count", header: m.common_ft_col_category?.() || "Category", name: "Category", width: 130, size: 130, readonly: true },
  { id: "amount", prop: "amount", header: m.common_ft_col_amount?.() || "Amount", name: "Amount", width: 120, size: 120, readonly: true, align: "end", cellClass: "font-mono font-bold" },
  { id: "frequency", prop: "frequency", header: m.common_ft_col_frequency?.() || "Frequency", name: "Frequency", width: 110, size: 110, readonly: true, align: "center" },
  { id: "status", prop: "status", header: m.common_word_status?.() || "Status", name: "Status", width: 100, size: 100, type: "badge", align: "center", readonly: true },
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
        <DocumentPage class="text-primary" size={18} />
        <span>{m.settings_nav_fee_templates ? m.settings_nav_fee_templates() : "Fee Templates"}</span>
      </h1>
      <p class="text-xs text-muted-foreground">
        {m.settings_nav_fee_templates_desc ? m.settings_nav_fee_templates_desc() : "Manage fee templates for student enrollment"}
      </p>
    </div>
    <div class="flex items-center gap-2">
      <FilterTrigger
        bind:filter={activeFilter}
        fields={feeTemplateFilterFields}
      />
      <Button variant="ghost" size="icon-sm" onclick={loadTemplates} class="text-muted-foreground">
        <Refresh size={14} class={isLoading ? "animate-spin" : ""} />
      </Button>
      <Button size="sm" onclick={handleCreate} class="text-xs font-bold shrink-0">
        <Plus size={14} class="me-1" />
        {m.settings_fee_templates_add ? m.settings_fee_templates_add() : "Add Template"}
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
      emptyTitle={m.common_ft_empty?.() || "No fee templates"}
      emptyDescription={m.common_ft_empty_desc?.() || "Create fee templates to auto-apply on enrollment"}
      rtl={getLocale() === "ar"}
      readonly
      onaction={handleGridAction}
    />
  </div>
</div>

<FeeTemplateSheet
  bind:open={isSheetOpen}
  {orgId}
  template={selectedTemplate}
  onSuccess={loadTemplates}
/>
