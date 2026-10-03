<!--
@component FeeTemplatesManager
Lists all fee templates with OrbisGrid, allowing CRUD through FeeTemplateSheet.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { api, queryKeys } from "$lib/api";
import { useQueryClient } from "@tanstack/svelte-query";
import { createFeeTemplatesQuery } from "$lib/features/finance/queries";
import { confirmStore } from "$lib/stores";
import { Button } from "@orbis/ui/button";
import { DocumentPage, Plus, Refresh } from "@orbis/icons";
import { getLocale } from "$lib/paraglide/runtime";
import { formatCurrency, formatDate } from "$lib/core/i18n";
import { toast } from "@orbis/ui";
import FeeTemplateSheet from "$lib/features/finance/components/fee-template-sheet.svelte";
import { OrbisGrid, type OrbisColumnDef, type GridActionEvent, type ActionItem } from "@orbis/svelte-grid";
import { FilterTrigger, type UniversalFilter } from "@orbis/ui/filter-builder";
import { feeTemplateFilterFields } from "$lib/core/filter-configs";

let { orgId }: { orgId: string } = $props();

const queryClient = useQueryClient();

// Queries
const feeTemplatesQuery = createFeeTemplatesQuery(() => orgId);
const templates = $derived(feeTemplatesQuery.data ?? []);
const isLoading = $derived(feeTemplatesQuery.isLoading);

// State
let isSheetOpen = $state(false);
let selectedTemplate = $state<any | null>(null);
let activeFilter = $state<UniversalFilter>({
  logic: "AND",
  conditions: [],
  groups: [],
});

function getCategoryLabel(catStr: string) {
  const c = String(catStr).toLowerCase();
  if (c === "tuition") return m.common_ft_category_tuition();
  if (c === "transport")
    return m.common_ft_category_transport();
  if (c === "books") return m.common_ft_category_books();
  if (c === "uniform") return m.common_ft_category_uniform();
  if (c === "exam") return m.common_ft_category_exam();
  return m.common_guardians_relationship_other();
}

function getFrequencyLabel(freqStr: string) {
  const f = String(freqStr).toLowerCase();
  if (f === "once") return m.common_ft_frequency_once();
  if (f === "monthly") return m.common_ft_frequency_monthly();
  if (f === "termly" || f === "per_term") return m.common_ft_frequency_termly();
  if (f === "yearly" || f === "annual") return m.common_ft_frequency_yearly();
  return freqStr;
}

function formatLevels(raw: any) {
  if (!raw) return m.ui_copy_all_levels_aa3ff22a();
  try {
    let levels: string[] = [];
    if (typeof raw === "string") {
      const parsed = JSON.parse(raw);
      levels = Array.isArray(parsed) ? parsed : (parsed.levels || (parsed.level ? [parsed.level] : []));
    } else if (Array.isArray(raw)) {
      levels = raw;
    } else if (typeof raw === "object") {
      levels = Array.isArray(raw.levels) ? raw.levels : (raw.level ? [raw.level] : []);
    }
    if (levels.length === 0) return m.ui_copy_all_levels_aa3ff22a();
    return levels.join(", ");
  } catch {
    return m.ui_copy_all_levels_aa3ff22a();
  }
}

const templateActions: ActionItem[] = [
  { id: "edit", label: m.common_action_edit(), icon: "Pencil" },
  { id: "delete", label: m.common_action_delete(), icon: "Trash2", variant: "destructive" },
];

const gridSource = $derived(
  templates.map((tpl, i) => ({
    ...tpl,
    __idx: i + 1,
    name: tpl.name_ar || tpl.name || "—",
    amount: formatCurrency(Number(tpl.default_amount || 0), tpl.currency || "YER"),
    frequency: getFrequencyLabel(tpl.frequency),
    applicable_levels_display: formatLevels(tpl.applicable_levels),
    status: (tpl.is_active === true || tpl.is_active === 1) ? "active" : "inactive",
    class_count: getCategoryLabel(tpl.category),
    created_at_display: tpl.created_at ? formatDate(tpl.created_at, "datetime") : "—",
    updated_at_display: tpl.updated_at ? formatDate(tpl.updated_at, "datetime") : "—",
  })),
);

const columns: OrbisColumnDef[] = [
  { id: "__idx", prop: "__idx", header: "#", name: "#", width: 48, size: 48, readonly: true, pin: "left", align: "center" },
  { id: "name", prop: "name", header: m.common_ft_col_name(), name: "Template", width: 220, size: 220, readonly: true, cellClass: "font-medium" },
  { id: "class_count", prop: "class_count", header: m.common_ft_col_category(), name: "Category", width: 120, size: 120, readonly: true },
  { id: "amount", prop: "amount", header: m.common_ft_col_amount(), name: "Amount", width: 120, size: 120, readonly: true, align: "end", cellClass: "font-mono font-bold" },
  { id: "frequency", prop: "frequency", header: m.common_ft_col_frequency(), name: "Frequency", width: 100, size: 100, readonly: true, align: "center" },
  { id: "applicable_levels_display", prop: "applicable_levels_display", header: m.ui_copy_target_levels_71287d45(), name: "Target Levels", width: 150, size: 150, readonly: true },
  { id: "status", prop: "status", header: m.common_word_status(), name: "Status", width: 90, size: 90, type: "badge", align: "center", readonly: true },
  { id: "created_at_display", prop: "created_at_display", header: m.ui_copy_created_at_01f6ea5f(), name: "Created At", width: 130, size: 130, readonly: true, cellClass: "font-mono text-xs text-muted-foreground" },
  { id: "updated_at_display", prop: "updated_at_display", header: m.ui_copy_updated_at_ba0d0a9f(), name: "Updated At", width: 130, size: 130, readonly: true, cellClass: "font-mono text-xs text-muted-foreground" },
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
    meta: { items: templateActions },
  },
];

function handleCreate() {
  selectedTemplate = null;
  isSheetOpen = true;
}

async function handleGridAction(event: GridActionEvent) {
  const row = event.row as any;
  if (event.actionId === "dblclick" || event.actionId === "edit") {
    selectedTemplate = row;
    isSheetOpen = true;
  } else if (event.actionId === "delete") {
    const templateName = row.name_ar || row.name || "";
    const confirmed = await confirmStore.prompt({
      title: `${m.common_action_delete()}؟`,
      description: `${m.common_action_delete()} "${templateName}"?`,
      confirmText: m.common_action_delete(),
      variant: "destructive",
    });
    if (confirmed) {
      try {
        await api.deleteFeeTemplate(row.id);
        toast.success(m.common_alert_deleted_successfully());
        await queryClient.invalidateQueries({
          queryKey: queryKeys.finance.feeTemplates(orgId),
        });
      } catch (err: any) {
        toast.error(err?.message || "Failed to delete fee template");
      }
    }
  }
}
</script>

<div class="h-full flex flex-col overflow-hidden p-6 gap-4">
  <div class="flex items-center justify-between pb-2 flex-nowrap shrink-0">
    <div class="flex flex-col gap-0.5 text-start">
      <h1 class="text-lg font-bold tracking-tight text-foreground flex items-center gap-2">
        <DocumentPage class="text-primary" size={18} />
        <span>{m.settings_nav_fee_templates()}</span>
      </h1>
      <p class="text-xs text-muted-foreground">
        {m.settings_nav_fee_templates_desc()}
      </p>
    </div>
    <div class="flex items-center gap-2">
      <FilterTrigger
        bind:filter={activeFilter}
        fields={feeTemplateFilterFields}
      />
      <Button 
        variant="ghost" 
        size="icon-sm" 
        onclick={() => feeTemplatesQuery.refetch()} 
        class="text-muted-foreground"
      >
        <Refresh size={14} class={isLoading ? "animate-spin" : ""} />
      </Button>
      <Button size="sm" onclick={handleCreate} class="text-xs font-bold shrink-0">
        <Plus size={14} class="me-1" />
        {m.settings_fee_templates_add()}
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
      emptyTitle={m.common_ft_empty()}
      emptyDescription={m.common_ft_empty_desc()}
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
  onSuccess={() => {
    queryClient.invalidateQueries({
      queryKey: queryKeys.finance.feeTemplates(orgId),
    });
  }}
/>
