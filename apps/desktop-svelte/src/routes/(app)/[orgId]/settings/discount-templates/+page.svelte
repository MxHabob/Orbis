<!--
  @page DiscountTemplatesPage
  Lists all discount and scholarship templates, allowing CRUD through DiscountTemplateSheet.
-->
<script lang="ts">
import { onMount } from "svelte";
import { commands } from "$lib/services/ipc";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@orbis/ui-svelte";
import { Button } from "@orbis/ui-svelte/button";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { Plus, Percent, Edit2, Trash2, ShieldAlert } from "lucide-svelte";
import { getLocale } from "$lib/paraglide/runtime";
import { formatCurrency } from "$lib/i18n-utils";
import { fade } from "svelte/transition";
import { toast } from "svelte-sonner";
import DiscountTemplateSheet from "$lib/components/sheets/discount-template-sheet.svelte";
import * as m from "$lib/paraglide/messages";

let { data }: { data: { orgId: string } } = $props();

// State
let templates = $state<any[]>([]);
let isLoading = $state(true);
let error = $state<string | null>(null);

// Sheets triggers
let isSheetOpen = $state(false);
let selectedTemplate = $state<any | null>(null);

async function loadTemplates() {
  isLoading = true;
  error = null;
  try {
    const list = await commands.finance.getDiscountTemplates(data.orgId);
    templates = list;
  } catch (e) {
    console.error("Error loading discount templates:", e);
    error = e instanceof Error ? e.message : String(e);
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (data.orgId) {
    loadTemplates();
  }
});

function handleCreate() {
  selectedTemplate = null;
  isSheetOpen = true;
}

function handleEdit(tpl: any) {
  selectedTemplate = tpl;
  isSheetOpen = true;
}

async function handleDelete(tpl: any) {
  const confirmMsg = m.dt_delete_confirm
    ? m.dt_delete_confirm()
    : "هل أنت متأكد من حذف قالب الخصم هذا؟";
  if (!confirm(confirmMsg)) return;

  try {
    await commands.finance.deleteDiscountTemplate(data.orgId, tpl.id);
    toast.success(
      m.dt_delete_success ? m.dt_delete_success() : "تم حذف قالب الخصم بنجاح",
    );
    loadTemplates();
  } catch (e) {
    const errorMsg = e instanceof Error ? e.message : String(e);
    toast.error(errorMsg);
  }
}

function getCalculationMethodLabel(method: string): string {
  const mLow = String(method).toLowerCase();
  if (mLow === "percentage")
    return m.dt_method_percentage ? m.dt_method_percentage() : "نسبة مئوية (%)";
  return m.dt_method_fixed ? m.dt_method_fixed() : "مبلغ ثابت";
}

function getFormattedValue(tpl: any): string {
  const val = Number(tpl.value || 0);
  if (String(tpl.calculation_method).toLowerCase() === "percentage") {
    return `${val}%`;
  }
  return formatCurrency(val, "YER");
}

function getTypeLabel(typeStr: string): string {
  const t = String(typeStr).toLowerCase();
  if (t === "manual")
    return m.attendance_status_unknown() === "غير محدد" ? "يدوي" : "Manual";
  if (t === "scholarship")
    return m.attendance_status_unknown() === "غير محدد"
      ? "منحة دراسية"
      : "Scholarship";
  if (t === "system")
    return m.attendance_status_unknown() === "غير محدد" ? "تلقائي" : "System";
  return typeStr;
}
</script>

<div class="h-full flex flex-col overflow-hidden bg-background p-6">
  <!-- Top header section -->
  <div class="flex items-center justify-between pb-6 select-none border-b border-border">
    <div class="flex flex-col gap-1 text-start">
      <h1 class="text-xl font-bold tracking-tight text-foreground flex items-center gap-2">
        <Percent class="text-primary" size={20} />
        <span>{m.dt_title ? m.dt_title() : "قوالب الخصومات"}</span>
      </h1>
      <p class="text-xs text-muted-foreground">
        {m.dt_desc ? m.dt_desc() : "إدارة قوالب الخصومات والمنح الدراسية وتخفيضات الرسوم"}
      </p>
    </div>

    <Button size="sm" onclick={handleCreate} class="flex items-center gap-1.5 text-xs font-bold cursor-pointer">
      <Plus size={14} />
      <span>{m.dt_add ? m.dt_add() : "إضافة قالب خصم"}</span>
    </Button>
  </div>

  <!-- Content panel -->
  <div class="flex-1 overflow-y-auto py-6">
    {#if isLoading}
      <div class="flex flex-col gap-3">
        {#each Array(5) as _}
          <Skeleton class="h-14 w-full rounded-lg" />
        {/each}
      </div>
    {:else if error}
      <div 
        in:fade={{ duration: 150 }}
        class="border border-destructive/20 bg-destructive/10 text-destructive rounded-xl p-4 flex items-center gap-3 text-sm text-start"
      >
        <ShieldAlert size={18} />
        <div>
          <span class="font-bold">Error:</span> {error}
        </div>
      </div>
    {:else if templates.length === 0}
      <div 
        in:fade={{ duration: 150 }}
        class="border border-dashed border-border/80 rounded-2xl p-12 flex flex-col items-center justify-center text-center gap-3 select-none"
      >
        <div class="w-12 h-12 rounded-full bg-primary/10 flex items-center justify-center text-primary mb-2">
          <Percent size={24} />
        </div>
        <h3 class="text-sm font-bold text-foreground">
          {m.dt_empty ? m.dt_empty() : "لا توجد قوالب خصومات"}
        </h3>
        <p class="text-xs text-muted-foreground max-w-sm">
          {m.dt_empty_desc ? m.dt_empty_desc() : "أنشئ قوالب الخصومات والمنح لتطبيقها على حسابات الطلاب ورسومهم المستحقة"}
        </p>
        <Button variant="outline" size="sm" onclick={handleCreate} class="mt-2 text-xs font-bold cursor-pointer">
          <Plus size={14} class="mr-1" />
          <span>{m.dt_add ? m.dt_add() : "إضافة قالب خصم"}</span>
        </Button>
      </div>
    {:else}
      <div in:fade={{ duration: 150 }} class="border border-border/60 rounded-xl overflow-hidden bg-card/30">
        <Table>
          <TableHeader class="bg-muted/40 select-none">
            <TableRow>
              <TableHead class="text-start font-bold text-xs">{m.dt_col_name ? m.dt_col_name() : "اسم القالب"}</TableHead>
              <TableHead class="text-start font-bold text-xs">Code</TableHead>
              <TableHead class="text-start font-bold text-xs">{m.dt_col_method ? m.dt_col_method() : "طريقة الحساب"}</TableHead>
              <TableHead class="text-start font-bold text-xs">{m.dt_col_value ? m.dt_col_value() : "القيمة"}</TableHead>
              <TableHead class="text-start font-bold text-xs">{m.attendance_status_unknown() === "غير محدد" ? "النوع" : "Type"}</TableHead>
              <TableHead class="text-start font-bold text-xs">{m.dt_col_status ? m.dt_col_status() : "الحالة"}</TableHead>
              <TableHead class="text-end font-bold text-xs w-[120px]">{m.dt_col_actions ? m.dt_col_actions() : "إجراءات"}</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {#each templates as tpl (tpl.id)}
              <TableRow class="hover:bg-muted/20 transition-all duration-150">
                <TableCell class="text-start font-medium text-xs">
                  <span class="font-bold text-foreground">{tpl.name}</span>
                </TableCell>
                <TableCell class="text-start text-xs font-mono text-muted-foreground">{tpl.code || '-'}</TableCell>
                <TableCell class="text-start text-xs text-muted-foreground">
                  {getCalculationMethodLabel(tpl.calculation_method)}
                </TableCell>
                <TableCell class="text-start text-xs font-bold text-foreground">
                  {getFormattedValue(tpl)}
                </TableCell>
                <TableCell class="text-start text-xs">
                  <span class="px-2 py-0.5 rounded-full text-[10px] font-semibold bg-primary/10 text-primary border border-primary/20">
                    {getTypeLabel(tpl.item_type)}
                  </span>
                </TableCell>
                <TableCell class="text-start text-xs">
                  {#if tpl.is_active === 1}
                    <span class="px-2 py-0.5 rounded-full text-[10px] font-semibold bg-emerald-500/10 text-emerald-500 border border-emerald-500/20">
                      {m.dt_status_active ? m.dt_status_active() : "نشط"}
                    </span>
                  {:else}
                    <span class="px-2 py-0.5 rounded-full text-[10px] font-semibold bg-muted/80 text-muted-foreground border border-border/40">
                      {m.dt_status_inactive ? m.dt_status_inactive() : "معطل"}
                    </span>
                  {/if}
                </TableCell>
                <TableCell class="text-end text-xs">
                  <div class="flex items-center justify-end gap-1">
                    <Button variant="ghost" size="icon" class="h-8 w-8 cursor-pointer text-muted-foreground hover:text-foreground" onclick={() => handleEdit(tpl)}>
                      <Edit2 size={13} />
                    </Button>
                    <Button variant="ghost" size="icon" class="h-8 w-8 cursor-pointer text-destructive hover:bg-destructive/10" onclick={() => handleDelete(tpl)}>
                      <Trash2 size={13} />
                    </Button>
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

<!-- Slide-over panel for Create/Edit -->
<DiscountTemplateSheet 
  bind:open={isSheetOpen} 
  orgId={data.orgId} 
  template={selectedTemplate}
  onSuccess={loadTemplates}
/>
