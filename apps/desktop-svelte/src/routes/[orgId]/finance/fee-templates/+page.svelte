<!--
  @page FeeTemplatesManagementPage
  Lists all fee templates, allowing CRUD through FeeTemplateSheet.
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
import { Plus, Receipt, Edit2, Trash2, ShieldAlert } from "lucide-svelte";
import { getLocale } from "$lib/paraglide/runtime";
import { formatCurrency } from "$lib/i18n-utils";
import { fade } from "svelte/transition";
import { toast } from "svelte-sonner";
import FeeTemplateSheet from "$lib/components/sheets/fee-template-sheet.svelte";
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
    const list = await commands.finance.getFeeTemplates(data.orgId);
    templates = list;
  } catch (e) {
    console.error("Error loading fee templates:", e);
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
  const confirmMsg = m.ft_delete_confirm
    ? m.ft_delete_confirm()
    : "هل أنت متأكد من حذف هذا القالب؟";
  if (!confirm(confirmMsg)) return;

  try {
    await commands.finance.deleteFeeTemplate(data.orgId, tpl.id);
    toast.success(
      m.ft_delete_success ? m.ft_delete_success() : "تم حذف القالب بنجاح",
    );
    loadTemplates();
  } catch (e) {
    const errorMsg = e instanceof Error ? e.message : String(e);
    toast.error(errorMsg);
  }
}

function getCategoryLabel(catStr: string) {
  const c = String(catStr).toLowerCase();
  if (c === "tuition")
    return m.ft_category_tuition ? m.ft_category_tuition() : "رسوم دراسية";
  if (c === "transport")
    return m.ft_category_transport ? m.ft_category_transport() : "نقل";
  if (c === "books") return m.ft_category_books ? m.ft_category_books() : "كتب";
  if (c === "uniform")
    return m.ft_category_uniform ? m.ft_category_uniform() : "زي مدرسي";
  if (c === "exam")
    return m.ft_category_exam ? m.ft_category_exam() : "رسوم امتحانات";
  return m.ft_category_other ? m.ft_category_other() : "أخرى";
}

function getFrequencyLabel(freqStr: string) {
  const f = String(freqStr).toLowerCase();
  if (f === "once")
    return m.ft_frequency_once ? m.ft_frequency_once() : "مرة واحدة";
  if (f === "monthly")
    return m.ft_frequency_monthly ? m.ft_frequency_monthly() : "شهري";
  if (f === "termly")
    return m.ft_frequency_termly ? m.ft_frequency_termly() : "فصلي";
  if (f === "yearly")
    return m.ft_frequency_yearly ? m.ft_frequency_yearly() : "سنوي";
  return freqStr;
}

function getAmount(tpl: any): number {
  return Number(tpl.default_amount || 0);
}

function getLevelsLabel(
  applicableLevelsStr: string | null | undefined,
): string {
  if (!applicableLevelsStr) {
    return m.attendance_status_unknown() === "غير محدد"
      ? "جميع الصفوف"
      : "All Grades";
  }
  try {
    const rules = JSON.parse(applicableLevelsStr);
    const levels: string[] = rules.levels || (rules.level ? [rules.level] : []);
    if (levels.length === 0) {
      return m.attendance_status_unknown() === "غير محدد"
        ? "جميع الصفوف"
        : "All Grades";
    }

    const levelNames: Record<string, { en: string; ar: string }> = {
      kg1: { en: "KG 1", ar: "روضة أولى" },
      kg2: { en: "KG 2", ar: "روضة ثانية" },
      grade1: { en: "Grade 1", ar: "الصف الأول" },
      grade2: { en: "Grade 2", ar: "الصف الثاني" },
      grade3: { en: "Grade 3", ar: "الصف الثالث" },
      grade4: { en: "Grade 4", ar: "الصف الرابع" },
      grade5: { en: "Grade 5", ar: "الصف الخامس" },
      grade6: { en: "Grade 6", ar: "الصف السادس" },
      grade7: { en: "Grade 7", ar: "الصف السابع" },
      grade8: { en: "Grade 8", ar: "الصف الثامن" },
      grade9: { en: "Grade 9", ar: "الصف التاسع" },
    };

    const isArabic = m.attendance_status_unknown() === "غير محدد";
    return levels
      .map((lvl) => {
        const nameObj = levelNames[lvl] || { en: lvl, ar: lvl };
        return isArabic ? nameObj.ar : nameObj.en;
      })
      .join(" - ");
  } catch (e) {
    return m.attendance_status_unknown() === "غير محدد"
      ? "جميع الصفوف"
      : "All Grades";
  }
}
</script>

<div class="h-full flex flex-col overflow-hidden bg-background p-6">
  <!-- Top header section -->
  <div class="flex items-center justify-between pb-6 select-none border-b border-border">
    <div class="flex flex-col gap-1 text-start">
      <h1 class="text-xl font-bold tracking-tight text-foreground flex items-center gap-2">
        <Receipt class="text-primary" size={20} />
        <span>{m.ft_title ? m.ft_title() : "قوالب الرسوم"}</span>
      </h1>
      <p class="text-xs text-muted-foreground">
        {m.ft_desc ? m.ft_desc() : "إدارة قوالب الرسوم الدراسية والخدمات المرتبطة بالتسجيل"}
      </p>
    </div>

    <Button size="sm" onclick={handleCreate} class="flex items-center gap-1.5 text-xs font-bold cursor-pointer">
      <Plus size={14} />
      <span>{m.ft_add ? m.ft_add() : "إضافة قالب"}</span>
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
          <Receipt size={24} />
        </div>
        <h3 class="text-sm font-bold text-foreground">
          {m.ft_empty ? m.ft_empty() : "لا توجد قوالب رسوم"}
        </h3>
        <p class="text-xs text-muted-foreground max-w-sm">
          {m.ft_empty_desc ? m.ft_empty_desc() : "أنشئ قوالب الرسوم لتطبيقها تلقائياً عند تسجيل الطلاب"}
        </p>
        <Button variant="outline" size="sm" onclick={handleCreate} class="mt-2 text-xs font-bold cursor-pointer">
          <Plus size={14} class="mr-1" />
          <span>{m.ft_add ? m.ft_add() : "إضافة قالب"}</span>
        </Button>
      </div>
    {:else}
      <div in:fade={{ duration: 150 }} class="border border-border/60 rounded-xl overflow-hidden bg-card/30">
        <Table>
          <TableHeader class="bg-muted/40 select-none">
            <TableRow>
              <TableHead class="text-start font-bold text-xs">{m.ft_col_name ? m.ft_col_name() : "اسم القالب"}</TableHead>
              <TableHead class="text-start font-bold text-xs">Code</TableHead>
              <TableHead class="text-start font-bold text-xs">{m.ft_col_category ? m.ft_col_category() : "الفئة"}</TableHead>
              <TableHead class="text-start font-bold text-xs">{m.ft_col_amount ? m.ft_col_amount() : "المبلغ"}</TableHead>
              <TableHead class="text-start font-bold text-xs">{m.ft_col_frequency ? m.ft_col_frequency() : "التكرار"}</TableHead>
              <TableHead class="text-start font-bold text-xs">{m.ft_col_status ? m.ft_col_status() : "الحالة"}</TableHead>
              <TableHead class="text-end font-bold text-xs w-[120px]">{m.ft_col_actions ? m.ft_col_actions() : "إجراءات"}</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {#each templates as tpl (tpl.id)}
              <TableRow class="hover:bg-muted/20 transition-all duration-150">
                <TableCell class="text-start font-medium text-xs">
                  <div class="flex flex-col">
                    <span class="font-bold text-foreground">{tpl.name_ar || tpl.name}</span>
                    <span class="text-[10px] text-muted-foreground font-semibold">
                      {getLevelsLabel(tpl.applicable_levels)}
                    </span>
                    {#if tpl.name_ar && tpl.name}
                      <span class="text-[9px] text-muted-foreground font-mono opacity-80">{tpl.name}</span>
                    {/if}
                  </div>
                </TableCell>
                <TableCell class="text-start text-xs font-mono text-muted-foreground">{tpl.code || '-'}</TableCell>
                <TableCell class="text-start text-xs">
                  <span class="px-2 py-0.5 rounded-full text-[10px] font-semibold bg-primary/10 text-primary border border-primary/20">
                    {getCategoryLabel(tpl.category)}
                  </span>
                </TableCell>
                <TableCell class="text-start text-xs font-bold text-foreground">
                  {formatCurrency(getAmount(tpl), "YER")}
                </TableCell>
                <TableCell class="text-start text-xs text-muted-foreground">{getFrequencyLabel(tpl.frequency)}</TableCell>
                <TableCell class="text-start text-xs">
                  {#if tpl.is_active === 1}
                    <span class="px-2 py-0.5 rounded-full text-[10px] font-semibold bg-emerald-500/10 text-emerald-500 border border-emerald-500/20">
                      {m.ft_status_active ? m.ft_status_active() : "نشط"}
                    </span>
                  {:else}
                    <span class="px-2 py-0.5 rounded-full text-[10px] font-semibold bg-muted/80 text-muted-foreground border border-border/40">
                      {m.ft_status_inactive ? m.ft_status_inactive() : "معطل"}
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
<FeeTemplateSheet 
  bind:open={isSheetOpen} 
  orgId={data.orgId} 
  template={selectedTemplate}
  onSuccess={loadTemplates}
/>
