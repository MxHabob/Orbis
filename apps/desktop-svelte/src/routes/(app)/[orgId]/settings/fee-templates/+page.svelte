<!--
  @page FeeTemplatesManagementPage
  Lists all fee templates with OrbisGrid, allowing CRUD through FeeTemplateSheet.
-->
<script lang="ts">
import { onMount } from "svelte";
import { commands } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Plus, Receipt } from "lucide-svelte";
import { getLocale } from "$lib/paraglide/runtime";
import { formatCurrency } from "$lib/core/i18n";
import { toast } from "svelte-sonner";
import FeeTemplateSheet from "$lib/features/finance/components/fee-template-sheet.svelte";
import * as common from "$lib/paraglide/messages/common";

import { DataTable } from "$lib/tables";
import { feeTemplates as ftDef, withIndex } from "$lib/tables/definitions";

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
    templates = await commands.finance.listFeeTemplates(data.orgId);
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
  if (c === "tuition") return common.ft_category_tuition?.() || "Tuition";
  if (c === "transport") return common.ft_category_transport?.() || "Transport";
  if (c === "books") return common.ft_category_books?.() || "Books";
  if (c === "uniform") return common.ft_category_uniform?.() || "Uniform";
  if (c === "exam") return common.ft_category_exam?.() || "Exam";
  return common.ft_category_other?.() || "Other";
}

function getFrequencyLabel(freqStr: string) {
  const f = String(freqStr).toLowerCase();
  if (f === "once") return common.ft_frequency_once?.() || "Once";
  if (f === "monthly") return common.ft_frequency_monthly?.() || "Monthly";
  if (f === "termly") return common.ft_frequency_termly?.() || "Termly";
  if (f === "yearly") return common.ft_frequency_yearly?.() || "Yearly";
  return freqStr;
}

// ── Grid Source ─────────────────────────────────────────────────────────

const gridSource = $derived(
  withIndex(
    templates.map((tpl) => ({
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
    name: common.ft_col_name?.() || "Template",
    amount: common.ft_col_amount?.() || "Amount",
    frequency: common.ft_col_frequency?.() || "Frequency",
    status: common.ft_col_status?.() || "Status",
    classes: common.ft_col_category?.() || "Category",
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

<div class="h-full flex flex-col overflow-hidden bg-background p-6">
  <div class="flex items-center justify-between pb-4 flex-nowrap shrink-0">
    <div class="flex flex-col gap-0.5 text-start">
      <h1 class="text-lg font-bold tracking-tight text-foreground flex items-center gap-2">
        <Receipt class="text-primary" size={18} />
        <span>{common.ft_title?.() || "Fee Templates"}</span>
      </h1>
      <p class="text-xs text-muted-foreground">
        {common.ft_desc?.() || "Manage fee templates for student enrollment"}
      </p>
    </div>
    <Button size="sm" onclick={handleCreate} class="text-xs font-bold shrink-0">
      <Plus size={14} class="me-1" />
      {common.ft_add?.() || "Add Template"}
    </Button>
  </div>

  <div class="flex-1 min-h-0">
    <DataTable
      {columns}
      source={gridSource}
      loading={isLoading}
      emptyIcon={Receipt}
      emptyTitle={common.ft_empty?.() || "No fee templates"}
      emptyDescription={common.ft_empty_desc?.() || "Create fee templates to auto-apply on enrollment"}
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
